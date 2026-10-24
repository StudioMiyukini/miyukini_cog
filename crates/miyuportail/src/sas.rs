//! @id     miyuportail_sas
//! @do     decoder_lot_dans_processus_isole
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — sas de décodage : protocole parent ⇄ sas, cœur exécuté sous seccomp, lanceur avec délai et mise à mort (EX-46).

//! # Sas de décodage
//!
//! Un processus `portail-sas` est lancé **par lot** de trames. Il se confine
//! ([`crate::bac_a_sable`]) **avant** de lire quoi que ce soit, décode chaque
//! trame avec le décodeur n° 1 ([`crate::msce::decoder_trame`]) et répond par
//! le **format interne** réencodé. Le parent impose un délai et tue le
//! processus en cas de dépassement ; toute fin anormale refuse le lot entier.
//!
//! Protocole (big-endian) :
//!
//! ```text
//! requête : "MPSR" | 0x01 | ls u8 | service | émetteur (32) | na u8 | na × clé (32)
//!           | lt u32 | traité | nt u8 (1..=64) | nt × [ len u32 | trame ]
//! réponse : "MPSA" | nt u8 | nt × [ statut u8 (0 = admis, sinon code de refus)
//!           | si admis : len u32 | format interne ]
//! ```

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::erreurs::{ErreurSas, RefusDecodage};
use crate::msce::{decoder_trame, encoder_interne, Ancres, Contexte, TAILLE_MAX_TRAME};
use crate::traite::{Curseur, IdService, Sens, Traite};

/// Nombre maximal de trames par lot.
pub const MAX_LOT: usize = 64;
/// Nombre maximal d'auteurs épinglés par session.
pub const MAX_AUTEURS: usize = 32;
/// Taille maximale d'une requête.
pub const MAX_REQUETE: usize = 8 * 1024 * 1024;
/// Taille maximale d'une réponse.
pub const MAX_REPONSE: usize = 8 * 1024 * 1024;
/// Délai par défaut d'un lot.
pub const DELAI_DEFAUT: Duration = Duration::from_secs(5);

const MAGIC_REQUETE: &[u8; 4] = b"MPSR";
const MAGIC_REPONSE: &[u8; 4] = b"MPSA";
const VERSION_PROTOCOLE: u8 = 0x01;

/// Requête adressée au sas (un lot d'une voie entrante).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequeteSas {
    /// Service de la voie.
    pub service: IdService,
    /// Ancres de la session.
    pub ancres: Ancres,
    /// Traité restreint au sens entrant.
    pub traite: Traite,
    /// Trames reçues (1 à [`MAX_LOT`]).
    pub trames: Vec<Vec<u8>>,
}

impl RequeteSas {
    /// Encode la requête ; refus si hors bornes.
    pub fn encoder(&self) -> Result<Vec<u8>, RefusDecodage> {
        if self.trames.is_empty()
            || self.trames.len() > MAX_LOT
            || self.ancres.auteurs.len() > MAX_AUTEURS
            || self.trames.iter().any(|t| t.len() > TAILLE_MAX_TRAME)
        {
            return Err(RefusDecodage::Requete);
        }
        let traite = self.traite.encoder();
        let mut s = Vec::new();
        s.extend_from_slice(MAGIC_REQUETE);
        s.push(VERSION_PROTOCOLE);
        s.push(self.service.as_str().len() as u8);
        s.extend_from_slice(self.service.as_str().as_bytes());
        s.extend_from_slice(&self.ancres.emetteur);
        s.push(self.ancres.auteurs.len() as u8);
        for a in &self.ancres.auteurs {
            s.extend_from_slice(a);
        }
        s.extend_from_slice(&(traite.len() as u32).to_be_bytes());
        s.extend_from_slice(&traite);
        s.push(self.trames.len() as u8);
        for t in &self.trames {
            s.extend_from_slice(&(t.len() as u32).to_be_bytes());
            s.extend_from_slice(t);
        }
        if s.len() > MAX_REQUETE {
            return Err(RefusDecodage::Requete);
        }
        Ok(s)
    }

    /// Décodage strict (côté sas).
    pub fn decoder(octets: &[u8]) -> Result<Self, RefusDecodage> {
        if octets.len() > MAX_REQUETE {
            return Err(RefusDecodage::Requete);
        }
        let mut c = Curseur::new(octets);
        if c.prendre(4)? != MAGIC_REQUETE || c.u8()? != VERSION_PROTOCOLE {
            return Err(RefusDecodage::Requete);
        }
        let ls = usize::from(c.u8()?);
        let so = c.prendre(ls)?;
        let service = std::str::from_utf8(so)
            .ok()
            .and_then(|s| IdService::new(s).ok())
            .ok_or(RefusDecodage::Requete)?;
        let emetteur = c.b32()?;
        let na = usize::from(c.u8()?);
        if na > MAX_AUTEURS {
            return Err(RefusDecodage::Requete);
        }
        let mut auteurs = Vec::with_capacity(na);
        for _ in 0..na {
            auteurs.push(c.b32()?);
        }
        let lt = c.u32()? as usize;
        let traite = Traite::decoder(c.prendre(lt)?)?;
        let nt = usize::from(c.u8()?);
        if nt == 0 || nt > MAX_LOT {
            return Err(RefusDecodage::Requete);
        }
        let mut trames = Vec::with_capacity(nt);
        for _ in 0..nt {
            let n = c.u32()? as usize;
            if n > TAILLE_MAX_TRAME {
                return Err(RefusDecodage::Requete);
            }
            trames.push(c.prendre(n)?.to_vec());
        }
        c.fin()?;
        Ok(Self { service, ancres: Ancres { emetteur, auteurs }, traite, trames })
    }
}

/// Résultat brut d'une trame : format interne, ou code de refus.
pub type ResultatTrame = Result<Vec<u8>, RefusDecodage>;

/// Encode une réponse.
pub fn encoder_reponse(resultats: &[ResultatTrame]) -> Vec<u8> {
    let mut s = Vec::new();
    s.extend_from_slice(MAGIC_REPONSE);
    s.push(resultats.len() as u8);
    for r in resultats {
        match r {
            Ok(interne) => {
                s.push(0);
                s.extend_from_slice(&(interne.len() as u32).to_be_bytes());
                s.extend_from_slice(interne);
            }
            Err(refus) => s.push(refus.code()),
        }
    }
    s
}

/// Décode strictement une réponse ; le nombre de résultats doit être `attendus`.
pub fn decoder_reponse(octets: &[u8], attendus: usize) -> Result<Vec<ResultatTrame>, ErreurSas> {
    if octets.len() > MAX_REPONSE {
        return Err(ErreurSas::Reponse);
    }
    let mut c = Curseur::new(octets);
    let e = |_| ErreurSas::Reponse;
    if c.prendre(4).map_err(e)? != MAGIC_REPONSE {
        return Err(ErreurSas::Reponse);
    }
    let n = usize::from(c.u8().map_err(e)?);
    if n != attendus {
        return Err(ErreurSas::Reponse);
    }
    let mut sortie = Vec::with_capacity(n);
    for _ in 0..n {
        let statut = c.u8().map_err(e)?;
        if statut == 0 {
            let lg = c.u32().map_err(e)? as usize;
            if lg > TAILLE_MAX_TRAME {
                return Err(ErreurSas::Reponse);
            }
            sortie.push(Ok(c.prendre(lg).map_err(e)?.to_vec()));
        } else {
            sortie.push(Err(RefusDecodage::depuis_code(statut).ok_or(ErreurSas::Reponse)?));
        }
    }
    c.fin().map_err(e)?;
    Ok(sortie)
}

/// Cœur du sas (exécuté **sous seccomp** dans `portail-sas`) : décode une
/// requête et produit la réponse. Pur calcul : aucune entrée/sortie.
pub fn traiter_requete(octets: &[u8]) -> Result<Vec<u8>, RefusDecodage> {
    let requete = RequeteSas::decoder(octets)?;
    let ctx =
        Contexte { service: &requete.service, sens: Sens::Entrante, traite: &requete.traite, ancres: &requete.ancres };
    let resultats: Vec<ResultatTrame> =
        requete.trames.iter().map(|t| decoder_trame(t, &ctx).map(|d| encoder_interne(&d))).collect();
    Ok(encoder_reponse(&resultats))
}

/// Mode d'essai du binaire (compilation de débogage uniquement).
#[cfg(debug_assertions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Essai {
    /// Tente d'ouvrir une socket après confinement (doit être tué par seccomp).
    Socket,
    /// Tente de créer un fichier après confinement (doit être tué par seccomp).
    Ecriture,
    /// Termine avec un code d'échec (plantage simulé).
    Plantage,
    /// Boucle sans fin (doit être tué par le délai du parent).
    Boucle,
    /// Altère la première réponse admise (doit être refusée par la deuxième validation).
    Divergence,
}

#[cfg(debug_assertions)]
impl Essai {
    /// Argument de ligne de commande.
    pub const fn argument(self) -> &'static str {
        match self {
            Self::Socket => "--essai=socket",
            Self::Ecriture => "--essai=ecriture",
            Self::Plantage => "--essai=plantage",
            Self::Boucle => "--essai=boucle",
            Self::Divergence => "--essai=divergence",
        }
    }

    fn depuis_argument(a: &str) -> Option<Self> {
        [Self::Socket, Self::Ecriture, Self::Plantage, Self::Boucle, Self::Divergence]
            .into_iter()
            .find(|e| e.argument() == a)
    }
}

/// Code de sortie : succès.
pub const SORTIE_OK: i32 = 0;
/// Code de sortie : requête refusée ou entrée illisible.
pub const SORTIE_REQUETE: i32 = 2;
/// Code de sortie : confinement impossible.
pub const SORTIE_BAC_A_SABLE: i32 = 3;
/// Code de sortie : arguments refusés.
pub const SORTIE_ARGUMENTS: i32 = 4;

/// Point d'entrée du binaire `portail-sas` : confinement, lecture bornée de
/// l'entrée standard, traitement, écriture de la réponse. Renvoie le code de sortie.
pub fn executer(arguments: &[String]) -> i32 {
    #[cfg(debug_assertions)]
    let essai = match arguments {
        [] => None,
        [a] => match Essai::depuis_argument(a) {
            Some(e) => Some(e),
            None => return SORTIE_ARGUMENTS,
        },
        _ => return SORTIE_ARGUMENTS,
    };
    #[cfg(not(debug_assertions))]
    if !arguments.is_empty() {
        return SORTIE_ARGUMENTS;
    }

    if crate::bac_a_sable::installer().is_err() {
        return SORTIE_BAC_A_SABLE;
    }

    #[cfg(debug_assertions)]
    match essai {
        Some(Essai::Socket) => {
            let _ = std::net::UdpSocket::bind("127.0.0.1:0");
            return SORTIE_REQUETE;
        }
        Some(Essai::Ecriture) => {
            let _ = std::fs::File::create("/tmp/portail-sas-essai-ecriture");
            return SORTIE_REQUETE;
        }
        Some(Essai::Plantage) => return 70,
        Some(Essai::Boucle) => loop {
            std::hint::spin_loop();
        },
        Some(Essai::Divergence) | None => {}
    }

    let Some(entree) = lire_entree_bornee() else {
        return SORTIE_REQUETE;
    };
    let Ok(reponse) = traiter_requete(&entree) else {
        return SORTIE_REQUETE;
    };
    #[cfg(debug_assertions)]
    let reponse = if essai == Some(Essai::Divergence) { alterer(&reponse) } else { reponse };

    let mut sortie = std::io::stdout().lock();
    if sortie.write_all(&reponse).and_then(|()| sortie.flush()).is_err() {
        return SORTIE_REQUETE;
    }
    SORTIE_OK
}

/// Altère la séquence de la première déclaration admise (essai de divergence).
#[cfg(debug_assertions)]
fn alterer(reponse: &[u8]) -> Vec<u8> {
    let Ok(mut resultats) = decoder_reponse(reponse, reponse.get(4).map_or(0, |n| usize::from(*n))) else {
        return reponse.to_vec();
    };
    if let Some(Ok(interne)) = resultats.iter_mut().find(|r| r.is_ok()) {
        if let Ok(mut d) = crate::msce::decoder_interne(interne) {
            d.sequence = d.sequence.wrapping_add(1);
            *interne = encoder_interne(&d);
        }
    }
    encoder_reponse(&resultats)
}

/// Lit l'entrée standard par appels `read(0, …)` uniquement, au plus [`MAX_REQUETE`] octets.
fn lire_entree_bornee() -> Option<Vec<u8>> {
    let mut entree = std::io::stdin().lock();
    let mut tampon = vec![0u8; 64 * 1024];
    let mut lu = Vec::new();
    loop {
        match entree.read(&mut tampon) {
            Ok(0) => return Some(lu),
            Ok(n) => {
                if lu.len() + n > MAX_REQUETE {
                    return None;
                }
                lu.extend_from_slice(&tampon[..n]);
            }
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return None,
        }
    }
}

/// Décodeur isolé : décode un lot de trames hors de l'espace mémoire de la zone COG.
pub trait DecodeurIsole: Send + Sync {
    /// Décode un lot ; une erreur refuse le lot entier.
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas>;
}

/// Sas par processus `portail-sas` confiné (seule implémentation admise en production).
#[derive(Debug, Clone)]
pub struct SasProcessus {
    binaire: PathBuf,
    delai: Duration,
    #[cfg(debug_assertions)]
    essai: Option<Essai>,
}

impl SasProcessus {
    /// Sas lançant `binaire` avec le délai par défaut.
    pub fn new(binaire: impl Into<PathBuf>) -> Self {
        Self {
            binaire: binaire.into(),
            delai: DELAI_DEFAUT,
            #[cfg(debug_assertions)]
            essai: None,
        }
    }

    /// Fixe le délai d'un lot.
    pub fn avec_delai(mut self, delai: Duration) -> Self {
        self.delai = delai;
        self
    }

    /// Active un mode d'essai (compilation de débogage uniquement).
    #[cfg(debug_assertions)]
    pub fn avec_essai(mut self, essai: Essai) -> Self {
        self.essai = Some(essai);
        self
    }

    #[cfg(target_os = "linux")]
    fn lancer(&self, entree: Vec<u8>, attendus: usize) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let mut commande = Command::new(&self.binaire);
        commande.env_clear().current_dir("/").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
        #[cfg(debug_assertions)]
        if let Some(essai) = self.essai {
            commande.arg(essai.argument());
        }
        let mut enfant = commande.spawn().map_err(|_| ErreurSas::Lancement)?;
        let (Some(mut stdin), Some(stdout)) = (enfant.stdin.take(), enfant.stdout.take()) else {
            let _ = enfant.kill();
            let _ = enfant.wait();
            return Err(ErreurSas::Lancement);
        };
        let ecrivain = std::thread::spawn(move || stdin.write_all(&entree).is_ok());
        let lecteur = std::thread::spawn(move || {
            let mut tampon = Vec::new();
            stdout.take(MAX_REPONSE as u64 + 1).read_to_end(&mut tampon).map(|_| tampon).ok()
        });

        let debut = Instant::now();
        let statut = loop {
            match enfant.try_wait() {
                Ok(Some(statut)) => break statut,
                Ok(None) if debut.elapsed() >= self.delai => {
                    let _ = enfant.kill();
                    let _ = enfant.wait();
                    let _ = ecrivain.join();
                    let _ = lecteur.join();
                    return Err(ErreurSas::Delai);
                }
                Ok(None) => std::thread::sleep(Duration::from_millis(2)),
                Err(_) => {
                    let _ = enfant.kill();
                    let _ = enfant.wait();
                    let _ = ecrivain.join();
                    let _ = lecteur.join();
                    return Err(ErreurSas::FinAnormale);
                }
            }
        };
        let ecrit = ecrivain.join().unwrap_or(false);
        let reponse = lecteur.join().ok().flatten();
        if !statut.success() || !ecrit {
            return Err(ErreurSas::FinAnormale);
        }
        let reponse = reponse.ok_or(ErreurSas::Reponse)?;
        decoder_reponse(&reponse, attendus)
    }
}

impl DecodeurIsole for SasProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let entree = requete.encoder().map_err(|_| ErreurSas::Reponse)?;
        #[cfg(target_os = "linux")]
        {
            self.lancer(entree, requete.trames.len())
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = entree;
            Err(ErreurSas::PlateformeNonSupportee)
        }
    }
}
