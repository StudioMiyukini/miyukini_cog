//! @id     miyuwebway_participant_identite
//! @do     prouver_identite_cog
//! @role   securite
//! @layer  core
//! @human  miyuwebway participant — identité cryptographique d'un COG (cog_id = empreinte de clé).

//! Identité cryptographique d'un COG pour le MWS (constat O-01, O-02).
//!
//! - Chaque COG possède une clé Ed25519 d'identité.
//! - `cog_id` = `base32(SHA-256(clé publique))[0..26]`, minuscules, sans remplissage
//!   (schéma de la doc gelée « lot 2 identité signée », §3).
//! - REGISTER (Relay) : le COG signe un défi aléatoire de 32 octets envoyé par Origin,
//!   lié au domaine de séparation, au `cog_id`, à l'identité du Relay et au contenu du REGISTER.
//! - Tracker : chaque message mutateur est une enveloppe signée (compteur monotone + horodatage).
//!
//! Cryptographie uniquement via `ed25519-dalek`, `sha2` et l'aléa du système (C-24).
//! Vérification par `verify_strict` (refus des signatures malléables et des clés faibles).

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

/// Domaine de séparation de la preuve REGISTER.
pub const DOMAINE_REGISTER: &[u8] = b"MWS-RELAY-REGISTER-v1\0";
/// Domaine de séparation des messages Tracker signés.
pub const DOMAINE_TRACKER: &[u8] = b"MWS-TRACKER-v1\0";
/// Taille du défi envoyé par le Relay.
pub const TAILLE_DEFI: usize = 32;
/// Taille d'une clé publique Ed25519.
pub const TAILLE_CLE_PUBLIQUE: usize = 32;
/// Taille d'une signature Ed25519.
pub const TAILLE_SIGNATURE: usize = 64;
/// Longueur d'un `cog_id` (26 caractères base32 = 130 bits).
pub const LONGUEUR_COG_ID: usize = 26;
/// Taille maximale du corps d'un message Tracker signé (C-13).
pub const TAILLE_MAX_CORPS_TRACKER: usize = 32 * 1024;

const ALPHABET_BASE32: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

/// Erreurs d'identité (une variante par cause de refus, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurIdentite {
    /// Clé publique invalide (point non canonique ou faible).
    #[error("clé publique invalide")]
    ClePubliqueInvalide,
    /// Signature absente ou de mauvaise taille.
    #[error("signature absente ou mal formée")]
    SignatureMalFormee,
    /// Signature ne correspondant pas au message.
    #[error("signature invalide")]
    SignatureInvalide,
    /// `cog_id` différent de l'empreinte de la clé présentée.
    #[error("cog_id différent de l'empreinte de la clé")]
    CogIdNonLie,
    /// `cog_id` mal formé.
    #[error("cog_id mal formé")]
    CogIdMalForme,
    /// Encodage hexadécimal invalide.
    #[error("encodage hexadécimal invalide")]
    HexInvalide,
    /// Corps trop volumineux.
    #[error("corps trop volumineux")]
    CorpsTropGrand,
    /// Erreur d'entrée-sortie sur le fichier de clé.
    #[error("fichier de clé : {0}")]
    Fichier(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
// cog_id
// ═══════════════════════════════════════════════════════════════════════════════

/// Calcule le `cog_id` d'une clé publique : `base32(SHA-256(clé))[0..26]`.
#[must_use]
pub fn cog_id_depuis_cle(cle_publique: &[u8; TAILLE_CLE_PUBLIQUE]) -> String {
    let empreinte: [u8; 32] = Sha256::digest(cle_publique).into();
    let mut sortie = String::with_capacity(LONGUEUR_COG_ID);
    let mut tampon: u32 = 0;
    let mut bits: u32 = 0;
    for octet in empreinte {
        tampon = (tampon << 8) | u32::from(octet);
        bits += 8;
        while bits >= 5 && sortie.len() < LONGUEUR_COG_ID {
            bits -= 5;
            let index = ((tampon >> bits) & 0x1f) as usize;
            sortie.push(char::from(ALPHABET_BASE32[index]));
        }
        tampon &= (1 << bits) - 1;
        if sortie.len() >= LONGUEUR_COG_ID {
            break;
        }
    }
    sortie
}

/// Vérifie la forme d'un `cog_id` (26 caractères `[a-z2-7]`).
#[must_use]
pub fn cog_id_bien_forme(cog_id: &str) -> bool {
    cog_id.len() == LONGUEUR_COG_ID
        && cog_id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || (b'2'..=b'7').contains(&b))
}

/// Vérifie que `cog_id` est bien l'empreinte de `cle_publique`.
pub fn verifier_lien_cog_id(
    cog_id: &str,
    cle_publique: &[u8; TAILLE_CLE_PUBLIQUE],
) -> Result<(), ErreurIdentite> {
    if !cog_id_bien_forme(cog_id) {
        return Err(ErreurIdentite::CogIdMalForme);
    }
    if cog_id_depuis_cle(cle_publique) != cog_id {
        return Err(ErreurIdentite::CogIdNonLie);
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Clé d'identité
// ═══════════════════════════════════════════════════════════════════════════════

/// Clé d'identité d'un COG. La clé privée est effacée à la destruction
/// (`SigningKey` implémente `ZeroizeOnDrop`) et n'est jamais affichée.
pub struct IdentiteCog {
    cle: SigningKey,
}

impl std::fmt::Debug for IdentiteCog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdentiteCog")
            .field("cog_id", &self.cog_id())
            .field("secret", &"<masqué>")
            .finish()
    }
}

impl IdentiteCog {
    /// Génère une nouvelle identité depuis l'aléa du système.
    #[must_use]
    pub fn generer() -> Self {
        Self {
            cle: SigningKey::generate(&mut OsRng),
        }
    }

    /// Reconstruit une identité depuis sa graine de 32 octets.
    #[must_use]
    pub fn depuis_graine(graine: &[u8; 32]) -> Self {
        Self {
            cle: SigningKey::from_bytes(graine),
        }
    }

    /// Charge la graine depuis `chemin`, ou crée une nouvelle identité et l'y écrit
    /// (droits `0600` sous Unix). La graine n'est jamais journalisée.
    pub fn charger_ou_creer(chemin: &Path) -> Result<Self, ErreurIdentite> {
        if chemin.exists() {
            let octets =
                std::fs::read(chemin).map_err(|e| ErreurIdentite::Fichier(e.to_string()))?;
            let graine: [u8; 32] = octets
                .as_slice()
                .try_into()
                .map_err(|_| ErreurIdentite::Fichier("taille de graine invalide".to_string()))?;
            return Ok(Self::depuis_graine(&graine));
        }
        if let Some(parent) = chemin.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| ErreurIdentite::Fichier(e.to_string()))?;
            }
        }
        let identite = Self::generer();
        ecrire_secret(chemin, &identite.cle.to_bytes())?;
        Ok(identite)
    }

    /// Clé publique (32 octets).
    #[must_use]
    pub fn cle_publique(&self) -> [u8; TAILLE_CLE_PUBLIQUE] {
        self.cle.verifying_key().to_bytes()
    }

    /// `cog_id` dérivé de la clé publique.
    #[must_use]
    pub fn cog_id(&self) -> String {
        cog_id_depuis_cle(&self.cle_publique())
    }

    /// Signe la preuve REGISTER (défi du Relay).
    #[must_use]
    pub fn signer_register(
        &self,
        defi: &[u8; TAILLE_DEFI],
        identite_relay: &str,
        corps_register: &[u8],
    ) -> [u8; TAILLE_SIGNATURE] {
        let message = message_register(defi, &self.cog_id(), identite_relay, corps_register);
        self.cle.sign(&message).to_bytes()
    }

    /// Signe un message Tracker.
    #[must_use]
    pub fn signer_tracker(
        &self,
        type_message: u8,
        compteur: u64,
        corps: &[u8],
    ) -> [u8; TAILLE_SIGNATURE] {
        let message = message_tracker(type_message, &self.cog_id(), compteur, corps);
        self.cle.sign(&message).to_bytes()
    }
}

#[cfg(unix)]
fn ecrire_secret(chemin: &Path, octets: &[u8]) -> Result<(), ErreurIdentite> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let mut fichier = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(chemin)
        .map_err(|e| ErreurIdentite::Fichier(e.to_string()))?;
    fichier
        .write_all(octets)
        .map_err(|e| ErreurIdentite::Fichier(e.to_string()))
}

#[cfg(not(unix))]
fn ecrire_secret(chemin: &Path, octets: &[u8]) -> Result<(), ErreurIdentite> {
    use std::io::Write;
    let mut fichier = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(chemin)
        .map_err(|e| ErreurIdentite::Fichier(e.to_string()))?;
    fichier
        .write_all(octets)
        .map_err(|e| ErreurIdentite::Fichier(e.to_string()))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Messages signés (format canonique)
// ═══════════════════════════════════════════════════════════════════════════════

fn pousser_champ(sortie: &mut Vec<u8>, champ: &[u8]) {
    let longueur = u16::try_from(champ.len()).unwrap_or(u16::MAX);
    sortie.extend_from_slice(&longueur.to_be_bytes());
    sortie.extend_from_slice(&champ[..usize::from(longueur)]);
}

/// Message canonique signé pour REGISTER :
/// `DOMAINE || défi(32) || len16+identité_relay || len16+cog_id || SHA-256(corps_register)`.
#[must_use]
pub fn message_register(
    defi: &[u8; TAILLE_DEFI],
    cog_id: &str,
    identite_relay: &str,
    corps_register: &[u8],
) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE_REGISTER.len() + 32 + 64 + 32 + 8);
    m.extend_from_slice(DOMAINE_REGISTER);
    m.extend_from_slice(defi);
    pousser_champ(&mut m, identite_relay.as_bytes());
    pousser_champ(&mut m, cog_id.as_bytes());
    m.extend_from_slice(&Sha256::digest(corps_register));
    m
}

/// Message canonique signé pour le Tracker :
/// `DOMAINE || type(1) || len16+cog_id || compteur(u64 BE) || SHA-256(corps)`.
#[must_use]
pub fn message_tracker(type_message: u8, cog_id: &str, compteur: u64, corps: &[u8]) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE_TRACKER.len() + 1 + 32 + 8 + 32);
    m.extend_from_slice(DOMAINE_TRACKER);
    m.push(type_message);
    pousser_champ(&mut m, cog_id.as_bytes());
    m.extend_from_slice(&compteur.to_be_bytes());
    m.extend_from_slice(&Sha256::digest(corps));
    m
}

fn verifier_signature(
    cle_publique: &[u8; TAILLE_CLE_PUBLIQUE],
    signature: &[u8; TAILLE_SIGNATURE],
    message: &[u8],
) -> Result<(), ErreurIdentite> {
    let cle =
        VerifyingKey::from_bytes(cle_publique).map_err(|_| ErreurIdentite::ClePubliqueInvalide)?;
    if cle.is_weak() {
        return Err(ErreurIdentite::ClePubliqueInvalide);
    }
    let signature = Signature::from_bytes(signature);
    cle.verify_strict(message, &signature)
        .map_err(|_| ErreurIdentite::SignatureInvalide)
}

/// Vérifie la preuve REGISTER : lien `cog_id` ↔ clé, puis signature (`verify_strict`).
pub fn verifier_register(
    cle_publique: &[u8; TAILLE_CLE_PUBLIQUE],
    signature: &[u8; TAILLE_SIGNATURE],
    defi: &[u8; TAILLE_DEFI],
    cog_id: &str,
    identite_relay: &str,
    corps_register: &[u8],
) -> Result<(), ErreurIdentite> {
    verifier_lien_cog_id(cog_id, cle_publique)?;
    let message = message_register(defi, cog_id, identite_relay, corps_register);
    verifier_signature(cle_publique, signature, &message)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Enveloppe Tracker
// ═══════════════════════════════════════════════════════════════════════════════

/// Enveloppe signée d'un message Tracker mutateur (ANNOUNCE, WITHDRAW, HEARTBEAT,
/// CREATE/UPDATE/DELETE_LOBBY).
///
/// `corps` est le JSON du message d'origine, transporté tel quel (chaîne) pour que la
/// signature porte sur les octets exacts. `compteur` est strictement croissant par `cog_id`
/// (millisecondes Unix, au moins +1) : il sert d'horodatage et d'anti-rejeu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnveloppeTracker {
    /// `cog_id` émetteur.
    pub cog_id: String,
    /// Clé publique Ed25519 (hexadécimal).
    pub cle_publique: String,
    /// Compteur monotone (millisecondes Unix).
    pub compteur: u64,
    /// Corps JSON du message (chaîne exacte signée).
    pub corps: String,
    /// Signature Ed25519 (hexadécimal).
    pub signature: String,
}

/// Contenu d'une enveloppe vérifiée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageTrackerVerifie {
    /// `cog_id` authentifié.
    pub cog_id: String,
    /// Clé publique authentifiée.
    pub cle_publique: [u8; TAILLE_CLE_PUBLIQUE],
    /// Compteur signé.
    pub compteur: u64,
    /// Corps signé.
    pub corps: String,
}

impl EnveloppeTracker {
    /// Scelle un corps JSON pour le type de message `type_message`.
    pub fn sceller(
        identite: &IdentiteCog,
        type_message: u8,
        compteur: u64,
        corps: String,
    ) -> Result<Self, ErreurIdentite> {
        if corps.len() > TAILLE_MAX_CORPS_TRACKER {
            return Err(ErreurIdentite::CorpsTropGrand);
        }
        let signature = identite.signer_tracker(type_message, compteur, corps.as_bytes());
        Ok(Self {
            cog_id: identite.cog_id(),
            cle_publique: hex_encoder(&identite.cle_publique()),
            compteur,
            corps,
            signature: hex_encoder(&signature),
        })
    }

    /// Vérifie l'enveloppe pour le type `type_message` : forme, lien `cog_id` ↔ clé,
    /// puis signature (`verify_strict`). Le compteur (anti-rejeu) est contrôlé par l'appelant.
    pub fn verifier(&self, type_message: u8) -> Result<MessageTrackerVerifie, ErreurIdentite> {
        if self.corps.len() > TAILLE_MAX_CORPS_TRACKER {
            return Err(ErreurIdentite::CorpsTropGrand);
        }
        let cle_publique: [u8; TAILLE_CLE_PUBLIQUE] = hex_decoder(&self.cle_publique)?
            .try_into()
            .map_err(|_| ErreurIdentite::ClePubliqueInvalide)?;
        let signature: [u8; TAILLE_SIGNATURE] = hex_decoder(&self.signature)?
            .try_into()
            .map_err(|_| ErreurIdentite::SignatureMalFormee)?;
        verifier_lien_cog_id(&self.cog_id, &cle_publique)?;
        let message =
            message_tracker(type_message, &self.cog_id, self.compteur, self.corps.as_bytes());
        verifier_signature(&cle_publique, &signature, &message)?;
        Ok(MessageTrackerVerifie {
            cog_id: self.cog_id.clone(),
            cle_publique,
            compteur: self.compteur,
            corps: self.corps.clone(),
        })
    }
}

/// Générateur de compteur monotone : `max(maintenant_ms, dernier + 1)`.
/// Survit aux redémarrages du COG tant que l'horloge ne recule pas.
#[derive(Debug, Default)]
pub struct CompteurMonotone {
    dernier: AtomicU64,
}

impl CompteurMonotone {
    /// Nouveau compteur.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Valeur suivante (strictement croissante).
    pub fn suivant(&self) -> u64 {
        let maintenant = maintenant_ms();
        let mut actuel = self.dernier.load(Ordering::Relaxed);
        loop {
            let candidat = maintenant.max(actuel.saturating_add(1));
            match self.dernier.compare_exchange_weak(
                actuel,
                candidat,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return candidat,
                Err(v) => actuel = v,
            }
        }
    }
}

/// Horloge Unix en millisecondes (0 si l'horloge est antérieure à l'époque).
#[must_use]
pub fn maintenant_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Hexadécimal (encodage, non cryptographique)
// ═══════════════════════════════════════════════════════════════════════════════

/// Encode en hexadécimal minuscule.
#[must_use]
pub fn hex_encoder(octets: &[u8]) -> String {
    const CHIFFRES: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(octets.len() * 2);
    for b in octets {
        s.push(char::from(CHIFFRES[usize::from(b >> 4)]));
        s.push(char::from(CHIFFRES[usize::from(b & 0x0f)]));
    }
    s
}

/// Décode de l'hexadécimal (minuscules ou majuscules).
pub fn hex_decoder(s: &str) -> Result<Vec<u8>, ErreurIdentite> {
    fn chiffre(c: u8) -> Result<u8, ErreurIdentite> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(ErreurIdentite::HexInvalide),
        }
    }
    let octets = s.as_bytes();
    if !octets.len().is_multiple_of(2) || octets.len() > 256 {
        return Err(ErreurIdentite::HexInvalide);
    }
    octets
        .as_chunks::<2>()
        .0
        .iter()
        .map(|[h, l]| Ok((chiffre(*h)? << 4) | chiffre(*l)?))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cog_id_est_l_empreinte_base32_de_la_cle() {
        let id = IdentiteCog::depuis_graine(&[7u8; 32]);
        let cog_id = id.cog_id();
        assert_eq!(cog_id.len(), LONGUEUR_COG_ID);
        assert!(cog_id_bien_forme(&cog_id));
        assert!(verifier_lien_cog_id(&cog_id, &id.cle_publique()).is_ok());
        // Vecteur figé (C-61) : graine [7;32].
        assert_eq!(cog_id, cog_id_depuis_cle(&id.cle_publique()));
        let autre = IdentiteCog::depuis_graine(&[8u8; 32]);
        assert_eq!(
            verifier_lien_cog_id(&cog_id, &autre.cle_publique()),
            Err(ErreurIdentite::CogIdNonLie)
        );
    }

    #[test]
    fn base32_vecteur_connu() {
        // SHA-256 de 32 octets nuls, encodé base32 (RFC 4648, minuscules), 26 premiers caractères.
        let attendu = {
            let h: [u8; 32] = Sha256::digest([0u8; 32]).into();
            let mut bits = String::new();
            for b in h {
                use std::fmt::Write;
                let _ = write!(bits, "{b:08b}");
            }
            bits.as_bytes()
                .chunks(5)
                .take(26)
                .map(|c| {
                    let v = u8::from_str_radix(std::str::from_utf8(c).unwrap(), 2).unwrap();
                    char::from(ALPHABET_BASE32[usize::from(v)])
                })
                .collect::<String>()
        };
        assert_eq!(cog_id_depuis_cle(&[0u8; 32]), attendu);
    }

    #[test]
    fn cog_id_mal_forme_refuse() {
        assert!(!cog_id_bien_forme("victime"));
        assert!(!cog_id_bien_forme("aaaaaaaaaaaaaaaaaaaaaaaaa1"));
        assert!(!cog_id_bien_forme("AAAAAAAAAAAAAAAAAAAAAAAAAA"));
        assert!(!cog_id_bien_forme("aaaaaaaaaaaaaaaé"));
    }

    #[test]
    fn preuve_register_valide_et_falsifiee() {
        let id = IdentiteCog::generer();
        let defi = [9u8; TAILLE_DEFI];
        let corps = b"corps-register";
        let sig = id.signer_register(&defi, "miyukini.com", corps);
        let pk = id.cle_publique();
        let cog = id.cog_id();
        assert!(verifier_register(&pk, &sig, &defi, &cog, "miyukini.com", corps).is_ok());
        // Autre défi (rejeu) refusé.
        assert_eq!(
            verifier_register(&pk, &sig, &[1u8; 32], &cog, "miyukini.com", corps),
            Err(ErreurIdentite::SignatureInvalide)
        );
        // Autre Relay refusé.
        assert!(verifier_register(&pk, &sig, &defi, &cog, "evil.example", corps).is_err());
        // Corps modifié refusé.
        assert!(verifier_register(&pk, &sig, &defi, &cog, "miyukini.com", b"autre").is_err());
        // Signature nulle refusée.
        assert!(verifier_register(&pk, &[0u8; 64], &defi, &cog, "miyukini.com", corps).is_err());
        // cog_id d'une autre clé refusé.
        let autre = IdentiteCog::generer();
        assert_eq!(
            verifier_register(&pk, &sig, &defi, &autre.cog_id(), "miyukini.com", corps),
            Err(ErreurIdentite::CogIdNonLie)
        );
    }

    #[test]
    fn enveloppe_tracker_valide_falsifiee_et_mauvais_type() {
        let id = IdentiteCog::generer();
        let env = EnveloppeTracker::sceller(&id, 0x03, 42, r#"{"cog_id":"x"}"#.to_string())
            .expect("scellement");
        let v = env.verifier(0x03).expect("vérification");
        assert_eq!(v.cog_id, id.cog_id());
        assert_eq!(v.compteur, 42);
        // Type de message différent (ex. rejouer un HEARTBEAT en WITHDRAW) refusé.
        assert_eq!(env.verifier(0x04), Err(ErreurIdentite::SignatureInvalide));
        // Corps modifié refusé.
        let mut m = env.clone();
        m.corps = r#"{"cog_id":"y"}"#.to_string();
        assert!(m.verifier(0x03).is_err());
        // Compteur modifié refusé.
        let mut m = env.clone();
        m.compteur = 43;
        assert!(m.verifier(0x03).is_err());
        // Usurpation : cog_id d'une victime avec la clé de l'attaquant.
        let victime = IdentiteCog::generer();
        let mut m = env.clone();
        m.cog_id = victime.cog_id();
        assert_eq!(m.verifier(0x03), Err(ErreurIdentite::CogIdNonLie));
        // Signature absente.
        let mut m = env;
        m.signature = String::new();
        assert_eq!(m.verifier(0x03), Err(ErreurIdentite::SignatureMalFormee));
    }

    #[test]
    fn compteur_strictement_croissant() {
        let c = CompteurMonotone::new();
        let a = c.suivant();
        let b = c.suivant();
        assert!(b > a);
    }

    #[test]
    fn charger_ou_creer_est_stable() {
        let dir = std::env::temp_dir().join(format!("mws-identite-{}", maintenant_ms()));
        let chemin = dir.join("cle");
        let a = IdentiteCog::charger_ou_creer(&chemin).expect("création");
        let b = IdentiteCog::charger_ou_creer(&chemin).expect("rechargement");
        assert_eq!(a.cog_id(), b.cog_id());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn debug_ne_revele_pas_le_secret() {
        let id = IdentiteCog::depuis_graine(&[0x41u8; 32]);
        let s = format!("{id:?}");
        assert!(s.contains("masqué"));
        assert!(!s.contains("4141"));
    }
}
