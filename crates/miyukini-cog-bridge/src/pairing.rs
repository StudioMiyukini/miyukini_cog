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

use crate::e2e::{parse_identity_key_hex, sha256, E2eError};

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
            cog_identity,
            device_nonce: self.device_nonce,
            code,
        })
    }
}

/// Appairage côté appareil, étape 3 : aléa du COG reçu, `Nd` révélable.
pub struct RevealedDevicePairing {
    cog_identity: [u8; 32],
    device_nonce: PairingNonce,
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

    const FROZEN_COMMITMENT: &str =
        "0c3b482e7dce81c68573cb8622bde867c12b8f86cf8d5bfdee37ca4a51e635d7";
    const FROZEN_SAS: &str = "03393231";
    const FROZEN_FINGERPRINT: &str = "411d 9449 70f2 ac19 f06e 7934 802c 437e";
}
