//! @id     miyukini_cog_bridge_pairing
//! @do     calculer_code_sas_appairage
//! @role   security
//! @layer  core
//! @human  miyukini-cog-bridge — appairage par engagement puis révélation (SAS, R-026).

//! Appairage appareil ↔ COG : engagement puis révélation (« commit-then-reveal »),
//! sur le modèle de la comparaison numérique Bluetooth / du SAS de ZRTP (R-026).
//!
//! ## Protocole d'appairage v2
//!
//! 1. **Engagement** : l'appareil tire un aléa `Nd` de 256 bits (`OsRng`) et
//!    envoie sa clé d'identité `Kd` et l'engagement
//!    `C = SHA-256(COMMIT_DOMAIN || Kd || Nd)` — **jamais** `Nd` à ce stade.
//! 2. **Aléa du COG** : le COG répond avec sa clé d'identité `Kc` et son propre
//!    aléa `Nc` de 256 bits, après avoir enregistré `(Kd, C, Nc)`.
//! 3. **Révélation** : l'appareil (qui connaît maintenant `Kc` et `Nc`) révèle
//!    `Nd` ; le COG vérifie `C` en temps constant. Échec = appairage abandonné.
//! 4. **Code SAS** : les deux côtés calculent
//!    `SHA-256(SAS_DOMAIN || Kd || Kc || Nd || Nc)` réduit à [`SAS_DIGITS`]
//!    chiffres décimaux, affiché sur le téléphone et sur Central (bureau) ;
//!    l'administrateur n'approuve que si les deux codes sont identiques.
//! 5. **Double confirmation humaine** : le téléphone n'épingle `Kc` que si
//!    l'utilisateur a confirmé **sur le téléphone** que le code est identique
//!    à celui de Central **et** que le suivi renvoie `approved` ; il exécute
//!    ensuite un handshake E2E v2 avec `Kc` épinglée et n'enregistre la
//!    connexion qu'en cas de succès.
//!
//! ## Preuve de possession de `Kd` (signatures d'appairage)
//!
//! L'engagement et la révélation sont signés par la clé d'identité ed25519
//! de l'appareil (vérification `verify_strict` par le COG **avant** toute
//! autre opération, C-23) :
//!
//! - engagement : `PAIRING_COMMIT_SIG_DOMAIN || version || len16(device_id) ||
//!   device_id || Kd || C` ;
//! - révélation : `PAIRING_REVEAL_SIG_DOMAIN || version || len16(device_id) ||
//!   device_id || Kd || Nd || Nc`.
//!
//! Sans la clé secrète de `Kd`, un tiers qui connaît `device_id` ne peut ni
//! relancer la demande ni envoyer une révélation qui supprimerait l'entrée :
//! une signature absente ou invalide est refusée sans toucher à la demande.
//! Seule une révélation **valablement signée** avec un mauvais `Nd` (donc
//! émise par le détenteur de la clé) abandonne la demande.
//!
//! ## Pourquoi le téléphone ne se fie pas au statut `approved`
//!
//! Le suivi `/api/bridge/pair/status` n'est pas authentifié : un intermédiaire
//! peut répondre `approved`. Une signature du COG sur ce statut n'aiderait
//! pas : l'intermédiaire qui a substitué sa propre clé `Km` à `Kc` signerait
//! lui-même un `approved` valide sous `Km`, la clé que le téléphone croit être
//! celle du COG. Seule la comparaison des codes par l'humain détecte la
//! substitution (le code du téléphone dépend de `Km`, celui de Central de
//! `Kd'`/`Kc`) ; d'où la confirmation explicite exigée sur le téléphone avant
//! d'épingler, l'approbation desktop restant nécessaire de l'autre côté.
//!
//! ## Pourquoi un intermédiaire ne peut pas choisir ses clés
//!
//! L'appareil est engagé sur `Nd` avant de connaître `Nc`, et le COG a fixé
//! `Nc` avant de connaître `Nd`. Un intermédiaire qui substitue sa propre clé
//! d'un côté ou de l'autre doit s'engager (ou recevoir l'engagement) avant de
//! connaître l'aléa adverse : les deux codes qu'il provoque sont indépendants,
//! et ils ne coïncident qu'avec une probabilité de 10⁻⁸ par tentative (une
//! seule tentative par appairage : chaque échec est visible par l'humain).
//!
//! ## Réduction modulo 10⁸
//!
//! Le code est `u64(8 premiers octets, big-endian) mod 10⁸`. Le biais de la
//! réduction (2⁶⁴ n'est pas multiple de 10⁸) est inférieur à 10⁸ / 2⁶⁴ ≈ 5·10⁻¹²,
//! négligeable : aucun rejet n'est nécessaire.

use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroizing;

use ed25519_dalek::{Signature, VerifyingKey};

use crate::e2e::{parse_identity_key_hex, sha256, E2eError, E2eIdentity};

/// Version du protocole d'appairage (engagement + SAS). Les demandes sans
/// version (appairage v1 : code à 6 chiffres sans engagement) sont refusées.
pub const PAIRING_PROTOCOL_VERSION: u8 = 2;

/// Taille des aléas d'appairage `Nd` / `Nc` (256 bits).
pub const PAIRING_NONCE_SIZE: usize = 32;

/// Nombre de chiffres décimaux du code de confirmation (≥ 26 bits d'entropie
/// visible, calculé sur 64 bits de condensat).
pub const SAS_DIGITS: usize = 8;

/// 10^[`SAS_DIGITS`].
const SAS_MODULUS: u64 = 100_000_000;

/// Domaine de l'engagement de l'appareil.
pub const PAIRING_COMMIT_DOMAIN: &[u8] = b"miyukini/cog-bridge/pairing-commit/v2";

/// Domaine du code SAS.
pub const PAIRING_SAS_DOMAIN: &[u8] = b"miyukini/cog-bridge/pairing-sas/v2";

/// Domaine de la signature de l'engagement par la clé de l'appareil.
pub const PAIRING_COMMIT_SIG_DOMAIN: &[u8] = b"miyukini/cog-bridge/pairing-commit-sig/v2";

/// Domaine de la signature de la révélation par la clé de l'appareil.
pub const PAIRING_REVEAL_SIG_DOMAIN: &[u8] = b"miyukini/cog-bridge/pairing-reveal-sig/v2";

/// Taille d'une signature d'appairage ed25519.
pub const PAIRING_SIGNATURE_SIZE: usize = 64;

/// Domaine de l'empreinte d'une clé d'identité (affichage humain).
pub const FINGERPRINT_DOMAIN: &[u8] = b"miyukini/cog-bridge/identity-fingerprint/v1";

// ═══════════════════════════════════════════════════════════════════════════
// Aléas
// ═══════════════════════════════════════════════════════════════════════════

/// Aléa d'appairage de 256 bits, effacé à la destruction (C-25).
///
/// `Nd` reste secret jusqu'à la révélation ; `Nc` est public dès l'étape 2.
#[derive(Clone)]
pub struct PairingNonce(Zeroizing<[u8; PAIRING_NONCE_SIZE]>);

impl std::fmt::Debug for PairingNonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PairingNonce(<redacted>)")
    }
}

impl PairingNonce {
    /// Tire un nouvel aléa (`OsRng`, C-24).
    pub fn generate() -> Self {
        let mut bytes = Zeroizing::new([0u8; PAIRING_NONCE_SIZE]);
        OsRng.fill_bytes(bytes.as_mut());
        Self(bytes)
    }

    /// Reconstruit un aléa reçu (ou figé pour un vecteur de test).
    pub fn from_bytes(bytes: [u8; PAIRING_NONCE_SIZE]) -> Self {
        Self(Zeroizing::new(bytes))
    }

    /// Décode un aléa hex (64 caractères exactement, C-13).
    pub fn from_hex(s: &str) -> Result<Self, E2eError> {
        if s.len() != 2 * PAIRING_NONCE_SIZE {
            return Err(E2eError::Malformed("pairing_nonce"));
        }
        let mut bytes = Zeroizing::new([0u8; PAIRING_NONCE_SIZE]);
        hex::decode_to_slice(s, bytes.as_mut()).map_err(|_| E2eError::Malformed("pairing_nonce"))?;
        Ok(Self(bytes))
    }

    /// Octets de l'aléa.
    pub fn as_bytes(&self) -> &[u8; PAIRING_NONCE_SIZE] {
        &self.0
    }

    /// Encodage hex (pour l'envoi sur le réseau ; effacé à la destruction).
    pub fn to_hex(&self) -> Zeroizing<String> {
        Zeroizing::new(hex::encode(self.0.as_ref()))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Engagement, code SAS, empreinte
// ═══════════════════════════════════════════════════════════════════════════

/// Engagement de l'appareil : `SHA-256(COMMIT_DOMAIN || Kd || Nd)`.
pub fn pairing_commitment(device_identity: &[u8; 32], device_nonce: &PairingNonce) -> [u8; 32] {
    let mut input = Zeroizing::new(Vec::with_capacity(PAIRING_COMMIT_DOMAIN.len() + 64));
    input.extend_from_slice(PAIRING_COMMIT_DOMAIN);
    input.extend_from_slice(device_identity);
    input.extend_from_slice(device_nonce.as_bytes());
    sha256(&input)
}

/// Décode un engagement hex (32 octets exactement).
pub fn parse_commitment_hex(s: &str) -> Result<[u8; 32], E2eError> {
    if s.len() != 64 {
        return Err(E2eError::Malformed("commitment"));
    }
    let mut out = [0u8; 32];
    hex::decode_to_slice(s, &mut out).map_err(|_| E2eError::Malformed("commitment"))?;
    Ok(out)
}

/// Vérifie en temps constant que `Nd` révélé correspond à l'engagement `C`.
pub fn verify_pairing_commitment(
    commitment: &[u8; 32],
    device_identity: &[u8; 32],
    device_nonce: &PairingNonce,
) -> bool {
    ct_eq_32(commitment, &pairing_commitment(device_identity, device_nonce))
}

/// Code de confirmation SAS à [`SAS_DIGITS`] chiffres :
/// `SHA-256(SAS_DOMAIN || Kd || Kc || Nd || Nc)`, 8 premiers octets en u64
/// big-endian, modulo 10⁸ (biais < 5·10⁻¹², voir la doc du module).
pub fn pairing_sas_code(
    device_identity: &[u8; 32],
    cog_identity: &[u8; 32],
    device_nonce: &PairingNonce,
    cog_nonce: &PairingNonce,
) -> String {
    let mut input = Zeroizing::new(Vec::with_capacity(PAIRING_SAS_DOMAIN.len() + 128));
    input.extend_from_slice(PAIRING_SAS_DOMAIN);
    input.extend_from_slice(device_identity);
    input.extend_from_slice(cog_identity);
    input.extend_from_slice(device_nonce.as_bytes());
    input.extend_from_slice(cog_nonce.as_bytes());
    let digest = sha256(&input);
    let mut head = [0u8; 8];
    head.copy_from_slice(&digest[..8]);
    let n = u64::from_be_bytes(head) % SAS_MODULUS;
    format!("{n:08}")
}

/// Normalise un code saisi ou affiché (espaces retirés). `None` si le code
/// n'a pas exactement [`SAS_DIGITS`] chiffres ASCII.
pub fn normalize_sas_code(code: &str) -> Option<String> {
    let digits: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    (digits.len() == SAS_DIGITS && digits.bytes().all(|b| b.is_ascii_digit())).then_some(digits)
}

/// Compare deux codes SAS en temps constant (après normalisation). Tout code
/// mal formé est refusé (fail-closed).
pub fn sas_codes_equal(a: &str, b: &str) -> bool {
    let (Some(a), Some(b)) = (normalize_sas_code(a), normalize_sas_code(b)) else {
        return false;
    };
    // Longueurs identiques (SAS_DIGITS) garanties par la normalisation.
    let mut diff = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        diff |= x ^ y;
    }
    std::hint::black_box(diff) == 0
}

/// Présentation lisible d'un code SAS : `1234 5678`.
pub fn format_sas_code(code: &str) -> String {
    match normalize_sas_code(code) {
        Some(d) => format!("{} {}", &d[..4], &d[4..]),
        None => code.to_string(),
    }
}

/// Empreinte lisible d'une clé d'identité : 128 premiers bits de
/// `SHA-256(FINGERPRINT_DOMAIN || clé)` en hex, groupés par 4.
pub fn identity_fingerprint(identity: &[u8; 32]) -> String {
    let mut input = Vec::with_capacity(FINGERPRINT_DOMAIN.len() + 32);
    input.extend_from_slice(FINGERPRINT_DOMAIN);
    input.extend_from_slice(identity);
    let digest = sha256(&input);
    hex::encode(&digest[..16])
        .as_bytes()
        .chunks(4)
        .map(|c| String::from_utf8_lossy(c).into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Empreinte d'une clé d'identité fournie en hex ; `None` si la clé est invalide.
pub fn identity_fingerprint_hex(identity_hex: &str) -> Option<String> {
    parse_identity_key_hex(identity_hex)
        .ok()
        .map(|k| identity_fingerprint(&k))
}

// ═══════════════════════════════════════════════════════════════════════════
// Signatures d'appairage (preuve de possession de Kd)
// ═══════════════════════════════════════════════════════════════════════════

/// Ajoute `version || len16(device_id) || device_id` (longueur big-endian :
/// aucune ambiguïté de concaténation). Refus si `device_id` dépasse 65 535 octets.
fn push_header(buf: &mut Vec<u8>, device_id: &str) -> Result<(), E2eError> {
    let len = u16::try_from(device_id.len()).map_err(|_| E2eError::TooLarge("device_id"))?;
    buf.push(PAIRING_PROTOCOL_VERSION);
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(device_id.as_bytes());
    Ok(())
}

/// Message signé à l'engagement :
/// `COMMIT_SIG_DOMAIN || version || len16(device_id) || device_id || Kd || C`.
pub fn pairing_commit_sig_message(
    device_id: &str,
    device_identity: &[u8; 32],
    commitment: &[u8; 32],
) -> Result<Vec<u8>, E2eError> {
    let mut m = Vec::with_capacity(PAIRING_COMMIT_SIG_DOMAIN.len() + 3 + device_id.len() + 64);
    m.extend_from_slice(PAIRING_COMMIT_SIG_DOMAIN);
    push_header(&mut m, device_id)?;
    m.extend_from_slice(device_identity);
    m.extend_from_slice(commitment);
    Ok(m)
}

/// Message signé à la révélation :
/// `REVEAL_SIG_DOMAIN || version || len16(device_id) || device_id || Kd || Nd || Nc`.
pub fn pairing_reveal_sig_message(
    device_id: &str,
    device_identity: &[u8; 32],
    device_nonce: &PairingNonce,
    cog_nonce: &PairingNonce,
) -> Result<Zeroizing<Vec<u8>>, E2eError> {
    let mut m = Zeroizing::new(Vec::with_capacity(
        PAIRING_REVEAL_SIG_DOMAIN.len() + 3 + device_id.len() + 96,
    ));
    m.extend_from_slice(PAIRING_REVEAL_SIG_DOMAIN);
    push_header(&mut m, device_id)?;
    m.extend_from_slice(device_identity);
    m.extend_from_slice(device_nonce.as_bytes());
    m.extend_from_slice(cog_nonce.as_bytes());
    Ok(m)
}

/// Signe l'engagement `commitment` avec l'identité de l'appareil (`Kd`).
pub fn sign_pairing_commit(
    identity: &E2eIdentity,
    device_id: &str,
    commitment: &[u8; 32],
) -> Result<[u8; PAIRING_SIGNATURE_SIZE], E2eError> {
    let msg = pairing_commit_sig_message(device_id, &identity.public_key_bytes(), commitment)?;
    Ok(identity.sign(&msg))
}

/// Signe la révélation de `Nd` (liée à `Nc`) avec l'identité de l'appareil.
pub fn sign_pairing_reveal(
    identity: &E2eIdentity,
    device_id: &str,
    device_nonce: &PairingNonce,
    cog_nonce: &PairingNonce,
) -> Result<[u8; PAIRING_SIGNATURE_SIZE], E2eError> {
    let msg = pairing_reveal_sig_message(
        device_id,
        &identity.public_key_bytes(),
        device_nonce,
        cog_nonce,
    )?;
    Ok(identity.sign(&msg))
}

/// Vérifie (`verify_strict`) la signature de l'engagement par `Kd`.
pub fn verify_pairing_commit_signature(
    device_identity: &[u8; 32],
    device_id: &str,
    commitment: &[u8; 32],
    signature: &[u8; PAIRING_SIGNATURE_SIZE],
) -> Result<(), E2eError> {
    let msg = pairing_commit_sig_message(device_id, device_identity, commitment)?;
    verify_strict(device_identity, &msg, signature)
}

/// Vérifie (`verify_strict`) la signature de la révélation par `Kd`.
pub fn verify_pairing_reveal_signature(
    device_identity: &[u8; 32],
    device_id: &str,
    device_nonce: &PairingNonce,
    cog_nonce: &PairingNonce,
    signature: &[u8; PAIRING_SIGNATURE_SIZE],
) -> Result<(), E2eError> {
    let msg = pairing_reveal_sig_message(device_id, device_identity, device_nonce, cog_nonce)?;
    verify_strict(device_identity, &msg, signature)
}

/// Décode une signature d'appairage hex (128 caractères exactement, C-13).
pub fn parse_pairing_signature_hex(s: &str) -> Result<[u8; PAIRING_SIGNATURE_SIZE], E2eError> {
    if s.len() != 2 * PAIRING_SIGNATURE_SIZE {
        return Err(E2eError::Malformed("pairing_signature"));
    }
    let mut out = [0u8; PAIRING_SIGNATURE_SIZE];
    hex::decode_to_slice(s, &mut out).map_err(|_| E2eError::Malformed("pairing_signature"))?;
    Ok(out)
}

/// Vérification stricte : clé valide et non faible, signature canonique.
fn verify_strict(
    key: &[u8; 32],
    msg: &[u8],
    signature: &[u8; PAIRING_SIGNATURE_SIZE],
) -> Result<(), E2eError> {
    let vk = VerifyingKey::from_bytes(key).map_err(|_| E2eError::InvalidIdentityKey)?;
    if vk.is_weak() {
        return Err(E2eError::InvalidIdentityKey);
    }
    vk.verify_strict(msg, &Signature::from_bytes(signature))
        .map_err(|_| E2eError::InvalidSignature)
}

/// Égalité en temps constant sur 32 octets.
fn ct_eq_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    std::hint::black_box(diff) == 0
}

// ═══════════════════════════════════════════════════════════════════════════
// Côté appareil : machine à états typée
// ═══════════════════════════════════════════════════════════════════════════

/// Appairage côté appareil, étape 1 : engagement émis, `Nd` encore secret.
///
/// L'aléa `Nd` n'est accessible qu'après réception de l'aléa du COG (voir
/// [`DevicePairing::receive_cog_nonce`]) : l'ordre du protocole est garanti
/// par les types.
pub struct DevicePairing {
    device_identity: [u8; 32],
    device_nonce: PairingNonce,
    commitment: [u8; 32],
}

impl std::fmt::Debug for DevicePairing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DevicePairing")
            .field("commitment", &hex::encode(self.commitment))
            .field("device_nonce", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl DevicePairing {
    /// Démarre un appairage : tire `Nd` et calcule l'engagement.
    pub fn start(device_identity: [u8; 32]) -> Self {
        Self::with_nonce(device_identity, PairingNonce::generate())
    }

    /// Variante à aléa fourni (tests, vecteurs figés).
    pub fn with_nonce(device_identity: [u8; 32], device_nonce: PairingNonce) -> Self {
        let commitment = pairing_commitment(&device_identity, &device_nonce);
        Self {
            device_identity,
            device_nonce,
            commitment,
        }
    }

    /// Engagement `C` à envoyer (hex).
    pub fn commitment_hex(&self) -> String {
        hex::encode(self.commitment)
    }

    /// Signature de l'engagement par l'identité de l'appareil (hex), à joindre
    /// à la requête d'engagement. Refus si `identity` n'est pas `Kd`.
    pub fn commit_signature_hex(
        &self,
        identity: &E2eIdentity,
        device_id: &str,
    ) -> Result<String, E2eError> {
        if identity.public_key_bytes() != self.device_identity {
            return Err(E2eError::IdentityMismatch);
        }
        Ok(hex::encode(sign_pairing_commit(identity, device_id, &self.commitment)?))
    }

    /// Étape 3 : reçoit la clé et l'aléa du COG, calcule le code SAS et
    /// autorise la révélation de `Nd`. Refus si la clé du COG est invalide
    /// ou égale à celle de l'appareil.
    pub fn receive_cog_nonce(
        self,
        cog_identity: [u8; 32],
        cog_nonce: PairingNonce,
    ) -> Result<RevealedDevicePairing, E2eError> {
        if cog_identity == self.device_identity {
            return Err(E2eError::SelfHandshake);
        }
        let code = pairing_sas_code(
            &self.device_identity,
            &cog_identity,
            &self.device_nonce,
            &cog_nonce,
        );
        Ok(RevealedDevicePairing {
            device_identity: self.device_identity,
            cog_identity,
            device_nonce: self.device_nonce,
            cog_nonce,
            code,
        })
    }
}

/// Appairage côté appareil, étape 3 : aléa du COG reçu, `Nd` révélable.
pub struct RevealedDevicePairing {
    device_identity: [u8; 32],
    cog_identity: [u8; 32],
    device_nonce: PairingNonce,
    cog_nonce: PairingNonce,
    code: String,
}

impl std::fmt::Debug for RevealedDevicePairing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RevealedDevicePairing")
            .field("cog_identity", &hex::encode(self.cog_identity))
            .field("device_nonce", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl RevealedDevicePairing {
    /// `Nd` à révéler au COG (hex).
    pub fn device_nonce_hex(&self) -> Zeroizing<String> {
        self.device_nonce.to_hex()
    }

    /// Signature de la révélation (`Nd` lié à `Nc`) par l'identité de
    /// l'appareil (hex). Refus si `identity` n'est pas `Kd`.
    pub fn reveal_signature_hex(
        &self,
        identity: &E2eIdentity,
        device_id: &str,
    ) -> Result<String, E2eError> {
        if identity.public_key_bytes() != self.device_identity {
            return Err(E2eError::IdentityMismatch);
        }
        Ok(hex::encode(sign_pairing_reveal(
            identity,
            device_id,
            &self.device_nonce,
            &self.cog_nonce,
        )?))
    }

    /// Code SAS à afficher (8 chiffres).
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Clé d'identité du COG à épingler après approbation.
    pub fn cog_identity(&self) -> [u8; 32] {
        self.cog_identity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::e2e::E2eIdentity;

    fn key(seed: u8) -> [u8; 32] {
        E2eIdentity::from_secret_bytes(&[seed; 32]).public_key_bytes()
    }

    #[test]
    fn honest_pairing_both_sides_same_code() {
        let kd = key(1);
        let kc = key(2);
        // 1. Appareil : engagement.
        let device = DevicePairing::start(kd);
        let commitment = parse_commitment_hex(&device.commitment_hex()).unwrap();
        // 2. COG : aléa.
        let nc = PairingNonce::generate();
        // 3. Appareil : reçoit Nc, révèle Nd.
        let revealed = device.receive_cog_nonce(kc, nc.clone()).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        // COG : vérifie l'engagement puis calcule le code.
        assert!(verify_pairing_commitment(&commitment, &kd, &nd));
        let cog_code = pairing_sas_code(&kd, &kc, &nd, &nc);
        assert_eq!(cog_code, revealed.code());
        assert_eq!(cog_code.len(), SAS_DIGITS);
        assert!(cog_code.bytes().all(|b| b.is_ascii_digit()));
        assert!(sas_codes_equal(&cog_code, &format_sas_code(revealed.code())));
    }

    #[test]
    fn wrong_nonce_fails_commitment() {
        let kd = key(1);
        let nd = PairingNonce::generate();
        let c = pairing_commitment(&kd, &nd);
        assert!(!verify_pairing_commitment(&c, &kd, &PairingNonce::generate()));
        // Même Nd mais autre clé : refus aussi (l'engagement lie la clé).
        assert!(!verify_pairing_commitment(&c, &key(3), &nd));
    }

    #[test]
    fn mitm_cannot_choose_cog_key_to_match_code() {
        // C (donc Kd, Nd) et Nc fixés : remplacer la clé du COG change le code.
        let kd = key(1);
        let nd = PairingNonce::from_bytes([0x11; 32]);
        let nc = PairingNonce::from_bytes([0x22; 32]);
        let honest = pairing_sas_code(&kd, &key(2), &nd, &nc);
        let mut collisions = 0;
        for _ in 0..2000 {
            let attacker = E2eIdentity::generate().public_key_bytes();
            if pairing_sas_code(&kd, &attacker, &nd, &nc) == honest {
                collisions += 1;
            }
        }
        // Probabilité attendue ≈ 2000 / 10⁸.
        assert_eq!(collisions, 0);
        // Changer l'un quelconque des aléas change aussi le code.
        assert_ne!(honest, pairing_sas_code(&kd, &key(2), &nd, &PairingNonce::from_bytes([0x23; 32])));
        assert_ne!(honest, pairing_sas_code(&kd, &key(2), &PairingNonce::from_bytes([0x12; 32]), &nc));
    }

    #[test]
    fn device_refuses_own_key_as_cog_key() {
        let kd = key(1);
        let device = DevicePairing::start(kd);
        assert_eq!(
            device.receive_cog_nonce(kd, PairingNonce::generate()).unwrap_err(),
            E2eError::SelfHandshake
        );
    }

    #[test]
    fn hex_parsing_bounded() {
        assert!(PairingNonce::from_hex(&"ab".repeat(32)).is_ok());
        assert!(PairingNonce::from_hex(&"ab".repeat(31)).is_err());
        assert!(PairingNonce::from_hex(&"zz".repeat(32)).is_err());
        assert!(parse_commitment_hex(&"00".repeat(33)).is_err());
        assert!(parse_commitment_hex("").is_err());
        assert!(!format!("{:?}", PairingNonce::generate()).contains(|c: char| c.is_ascii_digit()));
    }

    #[test]
    fn sas_code_comparison_fail_closed() {
        assert!(sas_codes_equal("12345678", "1234 5678"));
        assert!(!sas_codes_equal("12345678", "12345679"));
        assert!(!sas_codes_equal("1234567", "1234567"));
        assert!(!sas_codes_equal("abcdefgh", "abcdefgh"));
        assert!(!sas_codes_equal("", ""));
        assert_eq!(format_sas_code("12345678"), "1234 5678");
    }

    #[test]
    fn fingerprint_format() {
        let fp = identity_fingerprint(&key(1));
        assert_eq!(fp.len(), 32 + 7);
        assert_eq!(fp.split(' ').count(), 8);
        assert_ne!(fp, identity_fingerprint(&key(2)));
        assert_eq!(identity_fingerprint_hex(&hex::encode(key(1))), Some(fp));
        assert_eq!(identity_fingerprint_hex("zz"), None);
    }

    // ── Vecteur de référence figé (C-61) ─────────────────────────────────
    //
    // Entrées purement octets (indépendantes d'ed25519) : Kd = 0x01×32,
    // Kc = 0x02×32, Nd = 0x03×32, Nc = 0x04×32. Valeurs recalculées
    // indépendamment (Python hashlib).

    #[test]
    fn frozen_sas_vector() {
        let kd = [0x01; 32];
        let kc = [0x02; 32];
        let nd = PairingNonce::from_bytes([0x03; 32]);
        let nc = PairingNonce::from_bytes([0x04; 32]);
        assert_eq!(hex::encode(pairing_commitment(&kd, &nd)), FROZEN_COMMITMENT);
        assert_eq!(pairing_sas_code(&kd, &kc, &nd, &nc), FROZEN_SAS);
        assert_eq!(identity_fingerprint(&kd), FROZEN_FINGERPRINT);
    }

    #[test]
    fn honest_signatures_verify() {
        let id = E2eIdentity::generate();
        let kd = id.public_key_bytes();
        let device = DevicePairing::start(kd);
        let c = parse_commitment_hex(&device.commitment_hex()).unwrap();
        let sig = parse_pairing_signature_hex(&device.commit_signature_hex(&id, "dev-1").unwrap()).unwrap();
        assert_eq!(verify_pairing_commit_signature(&kd, "dev-1", &c, &sig), Ok(()));
        let nc = PairingNonce::generate();
        let revealed = device.receive_cog_nonce(key(2), nc.clone()).unwrap();
        let nd = PairingNonce::from_hex(&revealed.device_nonce_hex()).unwrap();
        let rsig =
            parse_pairing_signature_hex(&revealed.reveal_signature_hex(&id, "dev-1").unwrap()).unwrap();
        assert_eq!(verify_pairing_reveal_signature(&kd, "dev-1", &nd, &nc, &rsig), Ok(()));
    }

    #[test]
    fn signatures_bound_to_every_field() {
        let id = E2eIdentity::generate();
        let kd = id.public_key_bytes();
        let other = E2eIdentity::generate();
        let c = [0x55; 32];
        let sig = sign_pairing_commit(&id, "dev-1", &c).unwrap();
        // Autre device_id, autre engagement, autre clé, signature altérée.
        assert_eq!(verify_pairing_commit_signature(&kd, "dev-2", &c, &sig), Err(E2eError::InvalidSignature));
        assert_eq!(verify_pairing_commit_signature(&kd, "dev-1", &[0x56; 32], &sig), Err(E2eError::InvalidSignature));
        assert_eq!(
            verify_pairing_commit_signature(&other.public_key_bytes(), "dev-1", &c, &sig),
            Err(E2eError::InvalidSignature)
        );
        let mut bad = sig;
        bad[10] ^= 1;
        assert_eq!(verify_pairing_commit_signature(&kd, "dev-1", &c, &bad), Err(E2eError::InvalidSignature));
        // Signature d'engagement rejouée comme signature de révélation : domaines distincts.
        let nd = PairingNonce::from_bytes([1; 32]);
        let nc = PairingNonce::from_bytes([2; 32]);
        assert_eq!(verify_pairing_reveal_signature(&kd, "dev-1", &nd, &nc, &sig), Err(E2eError::InvalidSignature));
        let rsig = sign_pairing_reveal(&id, "dev-1", &nd, &nc).unwrap();
        // Révélation liée à Nc : un autre aléa du COG invalide la signature.
        assert_eq!(
            verify_pairing_reveal_signature(&kd, "dev-1", &nd, &PairingNonce::from_bytes([3; 32]), &rsig),
            Err(E2eError::InvalidSignature)
        );
        // Mauvaise identité locale : l'appareil refuse de signer.
        let device = DevicePairing::start(kd);
        assert_eq!(device.commit_signature_hex(&other, "dev-1").unwrap_err(), E2eError::IdentityMismatch);
        // device_id hors bornes.
        assert_eq!(
            sign_pairing_commit(&id, &"d".repeat(70_000), &c).unwrap_err(),
            E2eError::TooLarge("device_id")
        );
        // Décodage borné.
        assert!(parse_pairing_signature_hex(&"ab".repeat(63)).is_err());
        assert!(parse_pairing_signature_hex(&"zz".repeat(64)).is_err());
        assert!(parse_pairing_signature_hex("").is_err());
    }

    #[test]
    fn frozen_signature_vectors() {
        let kd = [0x01; 32];
        let nd = PairingNonce::from_bytes([0x03; 32]);
        let nc = PairingNonce::from_bytes([0x04; 32]);
        let c = pairing_commitment(&kd, &nd);
        let m = pairing_commit_sig_message("device-0001", &kd, &c).unwrap();
        assert_eq!(hex::encode(sha256(&m)), FROZEN_COMMIT_SIG_MSG_SHA256);
        let m = pairing_reveal_sig_message("device-0001", &kd, &nd, &nc).unwrap();
        assert_eq!(hex::encode(sha256(&m)), FROZEN_REVEAL_SIG_MSG_SHA256);
        // ed25519 est déterministe : signatures figées pour la graine 0x07×32.
        let id = E2eIdentity::from_secret_bytes(&[0x07; 32]);
        let pk = id.public_key_bytes();
        assert_eq!(hex::encode(pk), FROZEN_SIG_PUBLIC_KEY);
        let c = pairing_commitment(&pk, &nd);
        let sig = sign_pairing_commit(&id, "device-0001", &c).unwrap();
        assert_eq!(hex::encode(sig), FROZEN_COMMIT_SIG);
        let rsig = sign_pairing_reveal(&id, "device-0001", &nd, &nc).unwrap();
        assert_eq!(hex::encode(rsig), FROZEN_REVEAL_SIG);
        assert!(verify_pairing_commit_signature(&pk, "device-0001", &c, &sig).is_ok());
        assert!(verify_pairing_reveal_signature(&pk, "device-0001", &nd, &nc, &rsig).is_ok());
    }

    // Vecteurs de signature recalculés indépendamment (Python hashlib +
    // cryptography Ed25519), device_id = "device-0001".
    const FROZEN_COMMIT_SIG_MSG_SHA256: &str =
        "ba7ed96982a388b99c5eda1f8c8bbe4170a5b3dea1ebe934e0499d7c7a6f4fd6";
    const FROZEN_REVEAL_SIG_MSG_SHA256: &str =
        "12e9c792440abdfa685bb801559d72ae62aa850c6307f34761511b3f161f0f9f";
    const FROZEN_SIG_PUBLIC_KEY: &str =
        "ea4a6c63e29c520abef5507b132ec5f9954776aebebe7b92421eea691446d22c";
    const FROZEN_COMMIT_SIG: &str = "c9143e6017612ba85e80e6f0fd95a55932fbddf55fbf545319fad9396daf83eb1a387445e8e46d76f80b38db47c3ff0db352e4cd5b70c0fd3ede2b9527fa100d";
    const FROZEN_REVEAL_SIG: &str = "b7e95537df5218279db5035931cf8537fd2b51036d08f8c4a5967b7b68386d56b6eb394d53277fc6e50422b67a0b85e7a99343f77e3a3f1a422d0a66b08f1608";

    const FROZEN_COMMITMENT: &str =
        "0c3b482e7dce81c68573cb8622bde867c12b8f86cf8d5bfdee37ca4a51e635d7";
    const FROZEN_SAS: &str = "03393231";
    const FROZEN_FINGERPRINT: &str = "411d 9449 70f2 ac19 f06e 7934 802c 437e";
}
