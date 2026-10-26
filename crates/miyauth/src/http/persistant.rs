//! @id     miyauth_http_persistant
//! @do     persister_defis_et_nonces
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — défis de possession et nonces anti-rejeu persistants, partagés entre instances d'un service (fichiers atomiques, I5).

//! # Défis et nonces persistants (I5)
//!
//! Stockage choisi : **un fichier par élément** dans un dossier local, pas
//! KindMother. Raisons :
//!
//! - `miyauth` est une boîte à outils utilisée en **Franche** (façades
//!   jetables) : elle ne doit atteindre ni KindMother ni la zone COG (règle
//!   d'architecture vérifiée par `jayrdv-facade/tests/architecture.rs`) ;
//! - les primitives du système de fichiers donnent l'atomicité entre
//!   processus sans serveur : **création exclusive** (`O_CREAT|O_EXCL`) pour
//!   « premier vu » d'un nonce, **renommage** (atomique, un seul gagnant) pour
//!   consommer un défi ;
//! - durabilité : fichier puis dossier synchronisés (`fsync`) **avant**
//!   d'accepter, donc un rejeu après redémarrage (ou panne) est refusé.
//!
//! ## Formats (dossier `0700` ou `0750`, jamais ouvert aux « autres » ; fichiers `0600`)
//!
//! | élément | nom | contenu |
//! |---|---|---|
//! | nonce | `n-<hex(SHA-256("miyukini/miyauth/nonce/v1" ‖ 0 ‖ kid ‖ nonce))>` | `MNC1` ‖ expiration (u64 BE) |
//! | défi | `d-<hex(id, 16 o)>` | `MDF1` ‖ expiration (u64 BE) ‖ défi (32 o) ‖ audience |
//! | défi en consommation | `c-<hex(id)>-<hex(8 o aléatoires)>` | idem (renommé, puis lu et effacé) |
//!
//! Un nonce présent est **toujours** un rejeu, même expiré ou illisible
//! (fichier tronqué par une panne) : seule la purge le retire, quand son
//! horodatage ne peut plus être accepté. Purge au plus toutes les 10 s et à
//! saturation ; la capacité est comptée sur le dossier (toutes instances).

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use miyusceau::Kid;
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

use super::{DefiEmis, IdDefi, MagasinDefis, RefusHttp, CAPACITE_DEFIS_DEFAUT, DUREE_DEFI_SECS};
use crate::titres::audience_valide;

const MAGIC_NONCE: [u8; 4] = *b"MNC1";
const MAGIC_DEFI: [u8; 4] = *b"MDF1";
const DOMAINE_NONCE: &[u8] = b"miyukini/miyauth/nonce/v1";
/// Intervalle minimal entre deux purges (secondes).
const INTERVALLE_PURGE_SECS: u64 = 10;
/// Taille maximale lue d'un élément (en-tête + défi + audience).
const TAILLE_MAX_ELEMENT: u64 = 4 + 8 + 32 + 64;

fn hex(octets: &[u8]) -> String {
    use std::fmt::Write as _;
    octets.iter().fold(String::with_capacity(2 * octets.len()), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Prépare le dossier : créé `0700` s'il est absent ; refus s'il est un lien,
/// pas un répertoire, ou ouvert aux « autres ».
fn preparer_dossier(dossier: &Path) -> io::Result<()> {
    match fs::symlink_metadata(dossier) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::DirBuilder::new().recursive(true).mode(0o700).create(dossier)?;
        }
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    let m = fs::symlink_metadata(dossier)?;
    if !m.file_type().is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{} : pas un répertoire", dossier.display())));
    }
    if m.mode() & 0o007 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            format!("{} : ouvert aux autres (mode {:o})", dossier.display(), m.mode() & 0o7777),
        ));
    }
    Ok(())
}

fn creer_exclusif(chemin: &Path, contenu: &[u8]) -> io::Result<()> {
    // O_CREAT|O_EXCL : échoue si le nom existe, lien symbolique compris.
    let mut f = OpenOptions::new().write(true).create_new(true).mode(0o600).open(chemin)?;
    // En cas d'échec, un élément partiel reste (fail-closed pour un nonce) ; il sera purgé.
    f.write_all(contenu).and_then(|()| f.sync_all())
}

fn synchroniser_dossier(dossier: &Path) -> io::Result<()> {
    File::open(dossier)?.sync_all()
}

fn lire_borne(chemin: &Path) -> io::Result<Vec<u8>> {
    let f = OpenOptions::new().read(true).open(chemin)?;
    let mut o = Vec::new();
    f.take(TAILLE_MAX_ELEMENT + 1).read_to_end(&mut o)?;
    Ok(o)
}

fn expiration(o: &[u8], magic: [u8; 4]) -> Option<u64> {
    if o.len() < 12 || o[..4] != magic {
        return None;
    }
    let mut e = [0u8; 8];
    e.copy_from_slice(&o[4..12]);
    Some(u64::from_be_bytes(e))
}

/// Purge commune : retire les éléments de préfixe `prefixes` dont
/// l'expiration (ou, illisible, la date de modification + `marge`) est
/// passée ; rend le nombre d'éléments restants de préfixe `compte`.
fn purger(dossier: &Path, magic: [u8; 4], prefixes: &[&str], compte: &str, marge: u64, maintenant: u64) -> io::Result<usize> {
    let mut restants = 0;
    for e in fs::read_dir(dossier)? {
        let e = e?;
        let Ok(nom) = e.file_name().into_string() else {
            continue;
        };
        if !prefixes.iter().any(|p| nom.starts_with(p)) {
            continue;
        }
        let chemin = e.path();
        let fin = match lire_borne(&chemin) {
            Ok(o) => expiration(&o, magic),
            Err(_) => None,
        };
        let fin = match fin {
            Some(f) => f,
            None => match fs::symlink_metadata(&chemin) {
                Ok(m) => u64::try_from(m.mtime()).unwrap_or(0).saturating_add(marge),
                Err(_) => continue,
            },
        };
        if fin <= maintenant {
            let _ = fs::remove_file(&chemin);
        } else if nom.starts_with(compte) {
            restants += 1;
        }
    }
    Ok(restants)
}

#[derive(Debug, Default)]
struct Compteur {
    elements: usize,
    derniere_purge: Option<u64>,
}

impl Compteur {
    fn a_purger(&self, capacite: usize, maintenant: u64) -> bool {
        self.elements >= capacite
            || self.derniere_purge.is_none_or(|d| maintenant >= d.saturating_add(INTERVALLE_PURGE_SECS) || maintenant < d)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Nonces
// ═══════════════════════════════════════════════════════════════════════════════

/// Nonces vus, un fichier par nonce (voir l'en-tête du module).
#[derive(Debug)]
pub struct NoncesFichier {
    dossier: PathBuf,
    compteur: Mutex<Compteur>,
}

impl NoncesFichier {
    /// Ouvre (crée) le dossier des nonces.
    pub fn ouvrir(dossier: &Path) -> io::Result<Self> {
        preparer_dossier(dossier)?;
        Ok(Self { dossier: dossier.to_path_buf(), compteur: Mutex::new(Compteur::default()) })
    }

    fn chemin(&self, kid: &Kid, nonce: &[u8]) -> PathBuf {
        let mut h = Sha256::new();
        h.update(DOMAINE_NONCE);
        h.update([0u8]);
        h.update(kid);
        h.update(nonce);
        self.dossier.join(format!("n-{}", hex(&h.finalize())))
    }

    /// Vrai si le nonce est connu (ou si le stockage ne répond pas : fail-closed).
    pub fn deja_vu(&self, kid: &Kid, nonce: &[u8]) -> bool {
        match fs::symlink_metadata(self.chemin(kid, nonce)) {
            Ok(_) => true,
            Err(e) => e.kind() != io::ErrorKind::NotFound,
        }
    }

    /// Enregistre le nonce (création exclusive, synchronisée).
    pub fn consommer(
        &self,
        kid: &Kid,
        nonce: &[u8],
        expire: u64,
        capacite: usize,
        maintenant: u64,
    ) -> Result<(), RefusHttp> {
        let mut c = self.compteur.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if c.a_purger(capacite, maintenant) {
            let marge = expire.saturating_sub(maintenant);
            c.elements = purger(&self.dossier, MAGIC_NONCE, &["n-"], "n-", marge, maintenant)
                .map_err(|_| RefusHttp::NoncesSatures)?;
            c.derniere_purge = Some(maintenant);
        }
        if c.elements >= capacite {
            return Err(RefusHttp::NoncesSatures);
        }
        let chemin = self.chemin(kid, nonce);
        let mut contenu = MAGIC_NONCE.to_vec();
        contenu.extend_from_slice(&expire.to_be_bytes());
        match creer_exclusif(&chemin, &contenu) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => return Err(RefusHttp::Rejeu),
            Err(_) => return Err(RefusHttp::NoncesSatures),
        }
        synchroniser_dossier(&self.dossier).map_err(|_| RefusHttp::NoncesSatures)?;
        c.elements += 1;
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Défis
// ═══════════════════════════════════════════════════════════════════════════════

/// Magasin de défis persistant et partagé (voir l'en-tête du module) :
/// un défi émis par une instance est consommable **une seule fois** par
/// n'importe quelle instance partageant le dossier, y compris après redémarrage.
#[derive(Debug)]
pub struct MagasinDefisFichier {
    dossier: PathBuf,
    capacite: usize,
    duree: u64,
    compteur: Mutex<Compteur>,
}

impl MagasinDefisFichier {
    /// Magasin de `capacite` défis au plus, chacun valable `duree` secondes (≥ 1).
    pub fn ouvrir(dossier: &Path, capacite: usize, duree: u64) -> io::Result<Self> {
        preparer_dossier(dossier)?;
        Ok(Self { dossier: dossier.to_path_buf(), capacite, duree: duree.max(1), compteur: Mutex::new(Compteur::default()) })
    }

    /// Magasin aux réglages par défaut (10 000 défis, 60 s).
    pub fn ouvrir_defaut(dossier: &Path) -> io::Result<Self> {
        Self::ouvrir(dossier, CAPACITE_DEFIS_DEFAUT, DUREE_DEFI_SECS)
    }

    fn chemin(&self, id: &IdDefi) -> PathBuf {
        self.dossier.join(format!("d-{}", hex(id)))
    }
}

impl MagasinDefis for MagasinDefisFichier {
    fn emettre(&self, audience: &str, maintenant: u64) -> Result<DefiEmis, RefusHttp> {
        if !audience_valide(audience) {
            return Err(RefusHttp::AudienceInvalide);
        }
        let mut c = self.compteur.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if c.a_purger(self.capacite, maintenant) {
            c.elements = purger(&self.dossier, MAGIC_DEFI, &["d-", "c-"], "d-", self.duree, maintenant)
                .map_err(|_| RefusHttp::MagasinPlein)?;
            c.derniere_purge = Some(maintenant);
        }
        if c.elements >= self.capacite {
            return Err(RefusHttp::MagasinPlein);
        }
        let mut id = [0u8; 16];
        let mut defi = [0u8; 32];
        OsRng.fill_bytes(&mut id);
        OsRng.fill_bytes(&mut defi);
        let expire_a = maintenant.saturating_add(self.duree);
        let mut contenu = MAGIC_DEFI.to_vec();
        contenu.extend_from_slice(&expire_a.to_be_bytes());
        contenu.extend_from_slice(&defi);
        contenu.extend_from_slice(audience.as_bytes());
        creer_exclusif(&self.chemin(&id), &contenu).map_err(|_| RefusHttp::MagasinPlein)?;
        synchroniser_dossier(&self.dossier).map_err(|_| RefusHttp::MagasinPlein)?;
        c.elements += 1;
        Ok(DefiEmis { id, defi, audience: audience.to_string(), expire_a })
    }

    fn consommer(&self, id: &IdDefi, audience: &str, maintenant: u64) -> Result<[u8; 32], RefusHttp> {
        // Renommage atomique : une seule instance gagne, le défi n'existe plus
        // sous son nom, même si la suite échoue (usage unique, même en cas d'échec).
        let mut suffixe = [0u8; 8];
        OsRng.fill_bytes(&mut suffixe);
        let reclame = self.dossier.join(format!("c-{}-{}", hex(id), hex(&suffixe)));
        fs::rename(self.chemin(id), &reclame).map_err(|_| RefusHttp::DefiInvalide)?;
        let lu = lire_borne(&reclame);
        let _ = fs::remove_file(&reclame);
        let _ = synchroniser_dossier(&self.dossier);
        let o = lu.map_err(|_| RefusHttp::DefiInvalide)?;
        let expire = expiration(&o, MAGIC_DEFI).ok_or(RefusHttp::DefiInvalide)?;
        if o.len() < 44 || expire <= maintenant || &o[44..] != audience.as_bytes() {
            return Err(RefusHttp::DefiInvalide);
        }
        let mut defi = [0u8; 32];
        defi.copy_from_slice(&o[12..44]);
        Ok(defi)
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::*;
    use crate::http::RegistreNonces;

    fn dossier(nom: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!("miyauth-persistant-{nom}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        base.join("etat")
    }

    #[test]
    fn nonce_rejoue_apres_redemarrage_et_entre_instances() {
        let d = dossier("nonces");
        let a = RegistreNonces::persistant(&d, 100, 300).unwrap();
        let b = RegistreNonces::persistant(&d, 100, 300).unwrap();
        assert!(a.consommer([1; 32], b"nonce-0123456789", 1000).is_ok());
        assert_eq!(b.consommer([1; 32], b"nonce-0123456789", 1001), Err(RefusHttp::Rejeu), "autre instance");
        assert!(b.deja_vu(&[1; 32], b"nonce-0123456789"));
        drop((a, b));
        let c = RegistreNonces::persistant(&d, 100, 300).unwrap();
        assert_eq!(c.consommer([1; 32], b"nonce-0123456789", 1002), Err(RefusHttp::Rejeu), "après redémarrage");
        // Autre signataire, même nonce : distinct.
        assert!(c.consommer([2; 32], b"nonce-0123456789", 1002).is_ok());
        // Fichier tronqué (panne pendant l'écriture) : toujours un rejeu.
        let tronque = c.stockage_chemin([3; 32], b"tronque-01234567");
        fs::write(&tronque, b"MN").unwrap();
        assert_eq!(c.consommer([3; 32], b"tronque-01234567", 1002), Err(RefusHttp::Rejeu));
        let _ = fs::remove_dir_all(d.parent().unwrap());
    }

    #[test]
    fn nonces_purges_et_capacite() {
        let d = dossier("capacite");
        let r = RegistreNonces::persistant(&d, 2, 10).unwrap();
        r.consommer([1; 32], b"aaaaaaaaaaaaaaaa", 100).unwrap();
        r.consommer([1; 32], b"bbbbbbbbbbbbbbbb", 100).unwrap();
        assert_eq!(r.consommer([1; 32], b"cccccccccccccccc", 100), Err(RefusHttp::NoncesSatures));
        // Après 2 × fenêtre + 1 : purgés, place libérée.
        r.consommer([1; 32], b"cccccccccccccccc", 122).unwrap();
        // Saturation vue par une autre instance (capacité comptée sur le dossier).
        let r2 = RegistreNonces::persistant(&d, 1, 10).unwrap();
        assert_eq!(r2.consommer([1; 32], b"dddddddddddddddd", 122), Err(RefusHttp::NoncesSatures));
        let _ = fs::remove_dir_all(d.parent().unwrap());
    }

    #[test]
    fn dossier_ouvert_ou_lien_refuse() {
        use std::os::unix::fs::PermissionsExt;
        let d = dossier("droits");
        fs::create_dir_all(&d).unwrap();
        fs::set_permissions(&d, fs::Permissions::from_mode(0o757)).unwrap();
        assert!(RegistreNonces::persistant(&d, 10, 300).is_err());
        assert!(MagasinDefisFichier::ouvrir_defaut(&d).is_err());
        fs::set_permissions(&d, fs::Permissions::from_mode(0o700)).unwrap();
        let lien = d.parent().unwrap().join("lien");
        std::os::unix::fs::symlink(&d, &lien).unwrap();
        assert!(RegistreNonces::persistant(&lien, 10, 300).is_err());
        assert!(RegistreNonces::persistant(&d, 10, 300).is_ok());
        let _ = fs::remove_dir_all(d.parent().unwrap());
    }

    #[test]
    fn defi_partage_usage_unique_et_persistant() {
        let d = dossier("defis");
        let a = MagasinDefisFichier::ouvrir(&d, 3, 60).unwrap();
        let b = MagasinDefisFichier::ouvrir(&d, 3, 60).unwrap();
        let e = a.emettre("forum", 100).unwrap();
        // Consommé par l'autre instance, une seule fois.
        assert_eq!(b.consommer(&e.id, "forum", 110).unwrap(), e.defi);
        assert_eq!(a.consommer(&e.id, "forum", 110), Err(RefusHttp::DefiInvalide));
        // Survit au redémarrage, consommé même en cas d'échec (autre audience).
        let e = a.emettre("forum", 100).unwrap();
        drop((a, b));
        let c = MagasinDefisFichier::ouvrir(&d, 3, 60).unwrap();
        assert_eq!(c.consommer(&e.id, "autre", 101), Err(RefusHttp::DefiInvalide));
        assert_eq!(c.consommer(&e.id, "forum", 101), Err(RefusHttp::DefiInvalide));
        // Expiré.
        let e = c.emettre("forum", 100).unwrap();
        assert_eq!(c.consommer(&e.id, "forum", 160), Err(RefusHttp::DefiInvalide));
        // Capacité, purge des expirés.
        for _ in 0..3 {
            c.emettre("forum", 200).unwrap();
        }
        assert_eq!(c.emettre("forum", 200), Err(RefusHttp::MagasinPlein));
        assert!(c.emettre("forum", 261).is_ok());
        assert_eq!(c.emettre("Forum!", 300), Err(RefusHttp::AudienceInvalide));
        let _ = fs::remove_dir_all(d.parent().unwrap());
    }

    impl RegistreNonces {
        fn stockage_chemin(&self, kid: Kid, nonce: &[u8]) -> PathBuf {
            match &self.stockage {
                super::super::StockageNonces::Fichier(f) => f.chemin(&kid, nonce),
                super::super::StockageNonces::Memoire(_) => PathBuf::new(),
            }
        }
    }
}
