//! @id     borderguard_arrete
//! @do     definir_sceller_verifier_arretes
//! @role   securite
//! @layer  core
//! @human  borderguard — arrêtés de frontière : format typé, versionné, signé (Sceau v1.1), refus par défaut.

//! # Arrêtés de frontière
//!
//! Un **arrêté** (modèle zones et couches v1.3, §10 « Hiérarchie des règles »)
//! fixe, pour **une** frontière, la liste exhaustive des flux autorisés.
//! BorderGuard le **définit** (INV-BG-1, INV-BG-6) ; la Douane l'applique.
//!
//! - **Refus par défaut** : le format ne sait exprimer que des autorisations ;
//!   ce qui n'est pas déclaré n'existe pas (§10).
//! - **Signé** : Sceau v1.1 (`miyusceau`), clé d'usage « arrêtés »
//!   ([`INDICE_USAGE_ARRETES`]), vérifiée contre des ancres épinglées de niveau
//!   `Souverain` (clé de son propre COG) ; aucun arrêté d'un autre COG.
//! - **Borné dans le temps** : fenêtre `pas_avant .. pas_apres` obligatoire,
//!   au plus [`DUREE_MAX_ARRETE_SECS`].
//! - **Versionné** : `format` (= [`FORMAT_ARRETE`]) et `numero` strictement
//!   croissant par frontière (anti-rejeu, contrôlé par l'instance Douane).
//! - **Dans la constitution** : [`Constitution::controler`] refuse tout flux
//!   hors des invariants de zones (INV-Z1 à INV-Z6) ou au-delà des plafonds.
//!
//! ## Enveloppe
//!
//! `miyusceau` n'a pas de type dédié : comme la passation de racine, l'arrêté
//! est un `TypeSceau::Certificat` (0x06) dont le premier champ `nature` vaut
//! [`NATURE_ARRETE`] (0x41, « A »). Un type distinct demanderait un amendement
//! du format Sceau (point ouvert).
//!
//! ## Charge TLV (tags croissants, tous obligatoires)
//!
//! | Tag  | Champ       | Valeur                                              |
//! |------|-------------|-----------------------------------------------------|
//! | 0x01 | `nature`    | 1 octet, `0x41`                                     |
//! | 0x02 | `format`    | `u64`, = 1                                          |
//! | 0x03 | `frontiere` | 1 octet ([`Frontiere::code`])                       |
//! | 0x04 | `numero`    | `u64` ≥ 1                                           |
//! | 0x05 | `emis_le`   | secondes Unix                                       |
//! | 0x06 | `pas_avant` | secondes Unix                                       |
//! | 0x07 | `pas_apres` | secondes Unix (> `pas_avant`)                       |
//! | 0x08 | `emetteur`  | texte 1..=128 (traçabilité, INV-BG-8)               |
//! | 0x09 | `motif`     | texte 1..=256 (justification, INV-BG-8)             |
//! | 0x0A | `voies`     | liste de [`Voie`]                                   |
//! | 0x0B | `flux`      | liste de [`Flux`] (au moins un)                     |
//!
//! Liste : `nombre (u16) || nombre × [ longueur (u32) || entrée TLV ]`, entrées
//! **strictement croissantes** (octets encodés) : ni doublon, ni désordre.
//!
//! Entrée `Voie` : `0x01 id`, `0x02 service`.
//! Entrée `Flux` : `0x01 voie`, `0x02 source` (1 o), `0x03 destination` (1 o),
//! `0x04 protocole` (1 o), `0x05 port` (2 o), `0x06 pairs` (suite de réseaux
//! encodés, strictement croissants), `0x07 debit_max_kio_s` (4 o),
//! `0x08 taille_max_paquet` (2 o).

use std::fmt;

use miyusceau::{
    AncresDeConfiance, CleSignature, EcrivainTlv, ErreurSceau, Kid, LecteurTlv, NiveauConfiance, Titre, TypeSceau,
};
use sha2::{Digest, Sha256};

use crate::zone::{Frontiere, Protocole, Reseau, Zone};

/// Valeur du champ `nature` d'un certificat portant un arrêté (« A »).
pub const NATURE_ARRETE: u8 = 0x41;
/// Version du format d'arrêté.
pub const FORMAT_ARRETE: u64 = 1;
/// Indice d'usage SLIP-10 de la clé des arrêtés : `m/1296256839'/4'/g'`.
///
/// Le document gelé (§15) réserve « ≥ 4, attribué avec leur format » : ce
/// format l'attribue (4). À reporter dans `miyuracine::Usage` (point ouvert).
pub const INDICE_USAGE_ARRETES: u32 = 4;
/// Durée de validité maximale d'un arrêté (90 jours, rythme de rotation des clés d'usage).
pub const DUREE_MAX_ARRETE_SECS: u64 = 90 * 86_400;
/// Nombre maximal de voies par arrêté.
pub const MAX_VOIES: usize = 128;
/// Nombre maximal de flux par arrêté.
pub const MAX_FLUX: usize = 512;
/// Nombre maximal de pairs (liste blanche) par flux.
pub const MAX_PAIRS: usize = 64;
/// Longueur maximale d'un identifiant (voie, service).
pub const LONGUEUR_MAX_ID: usize = 32;
/// Taille minimale admise pour `taille_max_paquet` (entêtes IPv6 + TCP).
pub const TAILLE_MIN_PAQUET: u16 = 80;

const TAG_NATURE: u8 = 0x01;
const TAG_FORMAT: u8 = 0x02;
const TAG_FRONTIERE: u8 = 0x03;
const TAG_NUMERO: u8 = 0x04;
const TAG_EMIS_LE: u8 = 0x05;
const TAG_PAS_AVANT: u8 = 0x06;
const TAG_PAS_APRES: u8 = 0x07;
const TAG_EMETTEUR: u8 = 0x08;
const TAG_MOTIF: u8 = 0x09;
const TAG_VOIES: u8 = 0x0A;
const TAG_FLUX: u8 = 0x0B;

const TAG_V_ID: u8 = 0x01;
const TAG_V_SERVICE: u8 = 0x02;

const TAG_F_VOIE: u8 = 0x01;
const TAG_F_SOURCE: u8 = 0x02;
const TAG_F_DESTINATION: u8 = 0x03;
const TAG_F_PROTOCOLE: u8 = 0x04;
const TAG_F_PORT: u8 = 0x05;
const TAG_F_PAIRS: u8 = 0x06;
const TAG_F_DEBIT: u8 = 0x07;
const TAG_F_TAILLE: u8 = 0x08;

/// Voie : canal propre à un service sur une frontière (une voie par service, EX-44).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Voie {
    /// Identifiant de voie (`[a-z0-9-]`, 1..=32, sans tiret en bord).
    pub id: String,
    /// Service propriétaire (même alphabet).
    pub service: String,
}

/// Flux autorisé. Le sens est celui de l'**ouverture** de connexion :
/// `source` initie vers `destination` ; les réponses suivent l'état de connexion.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flux {
    /// Voie déclarée dans l'arrêté.
    pub voie: String,
    /// Zone qui ouvre la connexion.
    pub source: Zone,
    /// Zone qui reçoit la connexion.
    pub destination: Zone,
    /// Protocole de transport.
    pub protocole: Protocole,
    /// Port de destination (≠ 0).
    pub port: u16,
    /// Liste blanche des pairs extérieurs (extrémité extérieure du flux).
    /// Vide = toute la zone extérieure ; interdit pour International et Origin (INV-Z6).
    pub pairs: Vec<Reseau>,
    /// Débit maximal du flux, en Kio/s (≥ 1, ≤ plafond de la constitution).
    pub debit_max_kio_s: u32,
    /// Taille maximale d'un paquet IP (octets, ≥ [`TAILLE_MIN_PAQUET`]).
    pub taille_max_paquet: u16,
}

/// Arrêté de frontière (voir la documentation du module pour le format).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arrete {
    /// Frontière visée (une instance Douane).
    pub frontiere: Frontiere,
    /// Numéro d'arrêté, strictement croissant par frontière.
    pub numero: u64,
    /// Date d'émission.
    pub emis_le: u64,
    /// Début de validité.
    pub pas_avant: u64,
    /// Fin de validité (expiration).
    pub pas_apres: u64,
    /// Émetteur (Core ou cérémonie), pour la traçabilité.
    pub emetteur: String,
    /// Justification.
    pub motif: String,
    /// Voies déclarées.
    pub voies: Vec<Voie>,
    /// Flux autorisés (tout le reste est refusé).
    pub flux: Vec<Flux>,
}

impl Arrete {
    /// Voie déclarée par identifiant.
    pub fn voie(&self, id: &str) -> Option<&Voie> {
        self.voies.iter().find(|v| v.id == id)
    }
}

/// Motif de refus d'un arrêté. Tous signifient « refus » (fail-closed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurArrete {
    /// Erreur de sceau ou de format (signature, ancre, TLV, fenêtre de validité).
    Sceau(ErreurSceau),
    /// Signataire épinglé, mais pas de niveau `Souverain`.
    SignataireNonSouverain,
    /// Hors des invariants de zones (constitution réseau).
    HorsConstitution(String),
    /// Au-delà d'un plafond de la constitution.
    PlafondDepasse(String),
    /// Incohérence interne (voie non déclarée, doublon…).
    Incoherent(String),
}

impl fmt::Display for ErreurArrete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sceau(e) => write!(f, "sceau refusé : {e}"),
            Self::SignataireNonSouverain => f.write_str("signataire non souverain"),
            Self::HorsConstitution(m) => write!(f, "hors constitution : {m}"),
            Self::PlafondDepasse(m) => write!(f, "plafond dépassé : {m}"),
            Self::Incoherent(m) => write!(f, "arrêté incohérent : {m}"),
        }
    }
}

impl std::error::Error for ErreurArrete {}

impl From<ErreurSceau> for ErreurArrete {
    fn from(e: ErreurSceau) -> Self {
        Self::Sceau(e)
    }
}

/// Identifiant sûr : `[a-z0-9-]`, 1..=32, sans tiret en bord. Garantit qu'aucun
/// identifiant ne peut injecter de syntaxe dans une configuration générée.
pub fn identifiant_valide(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= LONGUEUR_MAX_ID
        && !id.starts_with('-')
        && !id.ends_with('-')
        && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn texte_valide(t: &str, max: usize) -> bool {
    !t.is_empty() && t.len() <= max && !t.chars().any(|c| c.is_control() || c == '"' || c == '\\')
}

// ---------------------------------------------------------------- encodage

fn encoder_liste(tag: u8, entrees: Vec<Vec<u8>>, ecrivain: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
    let mut entrees = entrees;
    entrees.sort();
    if entrees.windows(2).any(|w| w[0] == w[1]) {
        return Err(ErreurSceau::ValeurInvalide(tag));
    }
    let nombre = u16::try_from(entrees.len()).map_err(|_| ErreurSceau::ValeurInvalide(tag))?;
    let mut v = nombre.to_be_bytes().to_vec();
    for e in entrees {
        let l = u32::try_from(e.len()).map_err(|_| ErreurSceau::ValeurInvalide(tag))?;
        v.extend_from_slice(&l.to_be_bytes());
        v.extend_from_slice(&e);
    }
    ecrivain.octets(tag, &v)?;
    Ok(())
}

fn decoder_liste(tag: u8, valeur: &[u8], max: usize) -> Result<Vec<&[u8]>, ErreurSceau> {
    let err = ErreurSceau::ValeurInvalide(tag);
    let (n, mut reste) = valeur.split_at_checked(2).ok_or(err.clone())?;
    let nombre = usize::from(u16::from_be_bytes([n[0], n[1]]));
    if nombre > max {
        return Err(err);
    }
    let mut entrees: Vec<&[u8]> = Vec::with_capacity(nombre);
    for _ in 0..nombre {
        let (l, r) = reste.split_at_checked(4).ok_or(err.clone())?;
        let longueur = u32::from_be_bytes([l[0], l[1], l[2], l[3]]) as usize;
        let (e, r) = r.split_at_checked(longueur).ok_or(err.clone())?;
        if entrees.last().is_some_and(|p| *p >= e) {
            return Err(err);
        }
        entrees.push(e);
        reste = r;
    }
    if !reste.is_empty() {
        return Err(err);
    }
    Ok(entrees)
}

fn encoder_voie(v: &Voie) -> Result<Vec<u8>, ErreurSceau> {
    if !identifiant_valide(&v.id) {
        return Err(ErreurSceau::ValeurInvalide(TAG_V_ID));
    }
    if !identifiant_valide(&v.service) {
        return Err(ErreurSceau::ValeurInvalide(TAG_V_SERVICE));
    }
    let mut e = EcrivainTlv::new();
    e.texte(TAG_V_ID, &v.id)?.texte(TAG_V_SERVICE, &v.service)?;
    Ok(e.finir())
}

fn decoder_voie(octets: &[u8]) -> Result<Voie, ErreurSceau> {
    let mut l = LecteurTlv::new(octets);
    let id = l.exiger_texte(TAG_V_ID)?.to_owned();
    if !identifiant_valide(&id) {
        return Err(ErreurSceau::ValeurInvalide(TAG_V_ID));
    }
    let service = l.exiger_texte(TAG_V_SERVICE)?.to_owned();
    if !identifiant_valide(&service) {
        return Err(ErreurSceau::ValeurInvalide(TAG_V_SERVICE));
    }
    l.terminer()?;
    Ok(Voie { id, service })
}

fn controler_flux(f: &Flux) -> Result<(), ErreurSceau> {
    if !identifiant_valide(&f.voie) {
        return Err(ErreurSceau::ValeurInvalide(TAG_F_VOIE));
    }
    if f.port == 0 {
        return Err(ErreurSceau::ValeurInvalide(TAG_F_PORT));
    }
    if f.pairs.len() > MAX_PAIRS {
        return Err(ErreurSceau::ValeurInvalide(TAG_F_PAIRS));
    }
    if f.debit_max_kio_s == 0 {
        return Err(ErreurSceau::ValeurInvalide(TAG_F_DEBIT));
    }
    if f.taille_max_paquet < TAILLE_MIN_PAQUET {
        return Err(ErreurSceau::ValeurInvalide(TAG_F_TAILLE));
    }
    Ok(())
}

fn encoder_flux(f: &Flux) -> Result<Vec<u8>, ErreurSceau> {
    controler_flux(f)?;
    let mut pairs: Vec<Vec<u8>> = f.pairs.iter().map(Reseau::encoder).collect();
    pairs.sort();
    if pairs.windows(2).any(|w| w[0] == w[1]) {
        return Err(ErreurSceau::ValeurInvalide(TAG_F_PAIRS));
    }
    let mut e = EcrivainTlv::new();
    e.texte(TAG_F_VOIE, &f.voie)?
        .octets(TAG_F_SOURCE, &[f.source.code()])?
        .octets(TAG_F_DESTINATION, &[f.destination.code()])?
        .octets(TAG_F_PROTOCOLE, &[f.protocole.code()])?
        .octets(TAG_F_PORT, &f.port.to_be_bytes())?
        .octets(TAG_F_PAIRS, &pairs.concat())?
        .octets(TAG_F_DEBIT, &f.debit_max_kio_s.to_be_bytes())?
        .octets(TAG_F_TAILLE, &f.taille_max_paquet.to_be_bytes())?;
    Ok(e.finir())
}

fn un_octet(v: &[u8], tag: u8) -> Result<u8, ErreurSceau> {
    match v {
        [b] => Ok(*b),
        _ => Err(ErreurSceau::ValeurInvalide(tag)),
    }
}

fn decoder_flux(octets: &[u8]) -> Result<Flux, ErreurSceau> {
    let mut l = LecteurTlv::new(octets);
    let voie = l.exiger_texte(TAG_F_VOIE)?.to_owned();
    let source = Zone::depuis_code(un_octet(l.exiger(TAG_F_SOURCE)?, TAG_F_SOURCE)?)
        .ok_or(ErreurSceau::ValeurInvalide(TAG_F_SOURCE))?;
    let destination = Zone::depuis_code(un_octet(l.exiger(TAG_F_DESTINATION)?, TAG_F_DESTINATION)?)
        .ok_or(ErreurSceau::ValeurInvalide(TAG_F_DESTINATION))?;
    let protocole = Protocole::depuis_code(un_octet(l.exiger(TAG_F_PROTOCOLE)?, TAG_F_PROTOCOLE)?)
        .ok_or(ErreurSceau::ValeurInvalide(TAG_F_PROTOCOLE))?;
    let port: [u8; 2] = l.exiger(TAG_F_PORT)?.try_into().map_err(|_| ErreurSceau::ValeurInvalide(TAG_F_PORT))?;
    let mut brut = l.exiger(TAG_F_PAIRS)?;
    let mut pairs = Vec::new();
    let mut precedent: Option<Vec<u8>> = None;
    while !brut.is_empty() {
        let (r, reste) = Reseau::decoder(brut).ok_or(ErreurSceau::ValeurInvalide(TAG_F_PAIRS))?;
        let enc = r.encoder();
        if precedent.as_ref().is_some_and(|p| *p >= enc) || pairs.len() >= MAX_PAIRS {
            return Err(ErreurSceau::ValeurInvalide(TAG_F_PAIRS));
        }
        precedent = Some(enc);
        pairs.push(r);
        brut = reste;
    }
    let debit: [u8; 4] = l.exiger(TAG_F_DEBIT)?.try_into().map_err(|_| ErreurSceau::ValeurInvalide(TAG_F_DEBIT))?;
    let taille: [u8; 2] = l.exiger(TAG_F_TAILLE)?.try_into().map_err(|_| ErreurSceau::ValeurInvalide(TAG_F_TAILLE))?;
    l.terminer()?;
    let f = Flux {
        voie,
        source,
        destination,
        protocole,
        port: u16::from_be_bytes(port),
        pairs,
        debit_max_kio_s: u32::from_be_bytes(debit),
        taille_max_paquet: u16::from_be_bytes(taille),
    };
    controler_flux(&f)?;
    Ok(f)
}

fn controler_entete(a: &Arrete) -> Result<(), ErreurSceau> {
    if a.numero == 0 {
        return Err(ErreurSceau::ValeurInvalide(TAG_NUMERO));
    }
    if a.pas_apres <= a.pas_avant || a.pas_apres - a.pas_avant > DUREE_MAX_ARRETE_SECS {
        return Err(ErreurSceau::ValeurInvalide(TAG_PAS_APRES));
    }
    if a.emis_le > a.pas_apres {
        return Err(ErreurSceau::ValeurInvalide(TAG_EMIS_LE));
    }
    if !texte_valide(&a.emetteur, 128) {
        return Err(ErreurSceau::ValeurInvalide(TAG_EMETTEUR));
    }
    if !texte_valide(&a.motif, 256) {
        return Err(ErreurSceau::ValeurInvalide(TAG_MOTIF));
    }
    if a.voies.len() > MAX_VOIES {
        return Err(ErreurSceau::ValeurInvalide(TAG_VOIES));
    }
    if a.flux.is_empty() || a.flux.len() > MAX_FLUX {
        return Err(ErreurSceau::ValeurInvalide(TAG_FLUX));
    }
    // Voies : identifiants uniques ; chaque flux référence une voie déclarée.
    for (i, v) in a.voies.iter().enumerate() {
        if a.voies[..i].iter().any(|w| w.id == v.id) {
            return Err(ErreurSceau::ValeurInvalide(TAG_VOIES));
        }
    }
    if a.flux.iter().any(|f| a.voie(&f.voie).is_none()) {
        return Err(ErreurSceau::ValeurInvalide(TAG_FLUX));
    }
    Ok(())
}

impl Titre for Arrete {
    const TYPE: TypeSceau = TypeSceau::Certificat;

    fn encoder(&self, ecrivain: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        controler_entete(self)?;
        ecrivain
            .octets(TAG_NATURE, &[NATURE_ARRETE])?
            .u64(TAG_FORMAT, FORMAT_ARRETE)?
            .octets(TAG_FRONTIERE, &[self.frontiere.code()])?
            .u64(TAG_NUMERO, self.numero)?
            .u64(TAG_EMIS_LE, self.emis_le)?
            .u64(TAG_PAS_AVANT, self.pas_avant)?
            .u64(TAG_PAS_APRES, self.pas_apres)?
            .texte(TAG_EMETTEUR, &self.emetteur)?
            .texte(TAG_MOTIF, &self.motif)?;
        let voies = self.voies.iter().map(encoder_voie).collect::<Result<Vec<_>, _>>()?;
        encoder_liste(TAG_VOIES, voies, ecrivain)?;
        let flux = self.flux.iter().map(encoder_flux).collect::<Result<Vec<_>, _>>()?;
        encoder_liste(TAG_FLUX, flux, ecrivain)?;
        Ok(())
    }

    fn decoder(lecteur: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if lecteur.exiger(TAG_NATURE)? != [NATURE_ARRETE] {
            return Err(ErreurSceau::ValeurInvalide(TAG_NATURE));
        }
        if lecteur.exiger_u64(TAG_FORMAT)? != FORMAT_ARRETE {
            return Err(ErreurSceau::ValeurInvalide(TAG_FORMAT));
        }
        let frontiere = Frontiere::depuis_code(un_octet(lecteur.exiger(TAG_FRONTIERE)?, TAG_FRONTIERE)?)
            .ok_or(ErreurSceau::ValeurInvalide(TAG_FRONTIERE))?;
        let numero = lecteur.exiger_u64(TAG_NUMERO)?;
        let emis_le = lecteur.exiger_u64(TAG_EMIS_LE)?;
        let pas_avant = lecteur.exiger_u64(TAG_PAS_AVANT)?;
        let pas_apres = lecteur.exiger_u64(TAG_PAS_APRES)?;
        let emetteur = lecteur.exiger_texte(TAG_EMETTEUR)?.to_owned();
        let motif = lecteur.exiger_texte(TAG_MOTIF)?.to_owned();
        let voies = decoder_liste(TAG_VOIES, lecteur.exiger(TAG_VOIES)?, MAX_VOIES)?
            .into_iter()
            .map(decoder_voie)
            .collect::<Result<Vec<_>, _>>()?;
        let flux = decoder_liste(TAG_FLUX, lecteur.exiger(TAG_FLUX)?, MAX_FLUX)?
            .into_iter()
            .map(decoder_flux)
            .collect::<Result<Vec<_>, _>>()?;
        let a = Self { frontiere, numero, emis_le, pas_avant, pas_apres, emetteur, motif, voies, flux };
        controler_entete(&a)?;
        Ok(a)
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.pas_avant, self.pas_apres))
    }
}

// ------------------------------------------------------------ constitution

/// Constitution réseau : invariants de zones (codés en dur, doc §6) et plafonds.
///
/// Le document la place sous la racine COG (hors ligne). Sa forme signée n'est
/// pas encore spécifiée : ici, invariants figés + plafonds de référence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constitution {
    /// Débit maximal par flux (Kio/s).
    pub plafond_debit_kio_s: u32,
    /// Nombre maximal de flux par arrêté.
    pub plafond_flux: usize,
}

impl Constitution {
    /// Plafonds de référence (100 Mio/s par flux, [`MAX_FLUX`] flux).
    pub const fn reference() -> Self {
        Self { plafond_debit_kio_s: 102_400, plafond_flux: MAX_FLUX }
    }

    /// Contrôle un arrêté (déjà bien formé) contre la constitution.
    ///
    /// - couple `(source, destination)` admis par la frontière (INV-Z1, Z2, Z3, Z7) ;
    /// - liste blanche obligatoire pour International et Origin (INV-Z6) ;
    /// - pairs seulement pour une extrémité extérieure, d'une seule famille cohérente ;
    /// - plafonds de débit et de nombre de flux.
    pub fn controler(&self, a: &Arrete) -> Result<(), ErreurArrete> {
        if a.flux.len() > self.plafond_flux {
            return Err(ErreurArrete::PlafondDepasse(format!("{} flux", a.flux.len())));
        }
        for f in &a.flux {
            let lieu = format!("voie {} ({} → {})", f.voie, f.source, f.destination);
            if !a.frontiere.couple_admis(f.source, f.destination) {
                return Err(ErreurArrete::HorsConstitution(format!("{lieu} sur la frontière {}", a.frontiere)));
            }
            let exterieure = [f.source, f.destination].into_iter().find(|z| z.est_exterieure());
            match exterieure {
                Some(Zone::International | Zone::Origin) if f.pairs.is_empty() => {
                    return Err(ErreurArrete::HorsConstitution(format!("{lieu} : liste blanche obligatoire")));
                }
                None if !f.pairs.is_empty() => {
                    return Err(ErreurArrete::HorsConstitution(format!("{lieu} : pairs sans extrémité extérieure")));
                }
                _ => {}
            }
            if f.debit_max_kio_s > self.plafond_debit_kio_s {
                return Err(ErreurArrete::PlafondDepasse(format!("{lieu} : {} Kio/s", f.debit_max_kio_s)));
            }
        }
        Ok(())
    }
}

// -------------------------------------------------------- émission / preuve

/// Arrêté dont la signature, l'ancre, la fenêtre et la constitution ont été
/// vérifiées. Seul [`verifier_arrete`] en construit : le détenir prouve le contrôle.
#[derive(Debug, Clone)]
pub struct ArreteVerifie {
    arrete: Arrete,
    signataire: Kid,
    empreinte: [u8; 32],
}

impl ArreteVerifie {
    /// Arrêté vérifié.
    pub fn arrete(&self) -> &Arrete {
        &self.arrete
    }

    /// `kid` du signataire (clé d'usage « arrêtés »).
    pub fn signataire(&self) -> &Kid {
        &self.signataire
    }

    /// SHA-256 du sceau complet (traçabilité, en-tête des règles générées).
    pub fn empreinte(&self) -> &[u8; 32] {
        &self.empreinte
    }

    /// Empreinte en hexadécimal.
    pub fn empreinte_hex(&self) -> String {
        use std::fmt::Write as _;
        self.empreinte.iter().fold(String::with_capacity(64), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
    }

    /// Vrai si l'arrêté est dans sa fenêtre de validité à `maintenant_secs`
    /// (sans tolérance : l'expiration en cours d'exploitation est stricte).
    pub fn en_vigueur(&self, maintenant_secs: u64) -> bool {
        self.arrete.pas_avant <= maintenant_secs && maintenant_secs < self.arrete.pas_apres
    }
}

/// Scelle un arrêté avec la clé d'usage « arrêtés » (contrôle la constitution avant).
pub fn sceller_arrete(arrete: &Arrete, cle: &CleSignature, constitution: &Constitution) -> Result<Vec<u8>, ErreurArrete> {
    constitution.controler(arrete)?;
    Ok(miyusceau::sceller(arrete, cle)?)
}

/// Vérifie un arrêté scellé (fail-closed) : format strict, signature contre
/// des ancres **épinglées** de niveau `Souverain`, fenêtre de validité,
/// cohérence, constitution.
pub fn verifier_arrete(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    constitution: &Constitution,
    maintenant_secs: u64,
) -> Result<ArreteVerifie, ErreurArrete> {
    let verifie = miyusceau::verifier::<Arrete>(octets, ancres, maintenant_secs)?;
    if verifie.niveau() != NiveauConfiance::Souverain {
        return Err(ErreurArrete::SignataireNonSouverain);
    }
    let signataire = *verifie.signataire();
    let arrete = verifie.into_titre();
    constitution.controler(&arrete)?;
    Ok(ArreteVerifie { arrete, signataire, empreinte: Sha256::digest(octets).into() })
}
