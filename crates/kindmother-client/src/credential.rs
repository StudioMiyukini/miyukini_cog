//! Justificatif d'opérateur côté client (K-04).
//!
//! @id     kindmother_client_credential
//! @do     charger_justificatif_operateur_et_signer_defi
//! @role   securite
//! @layer  toolkit
//! @human  kindmother-client — clé Ed25519 de l'opérateur et réponse au défi du service.
//!
//! Le service KindMother approvisionne une clé Ed25519 par opérateur dans
//! `<KINDMOTHER_DATA_DIR>/credentials/<operator>.key` (droits `0600`). L'opérateur la
//! charge avec [`OperatorCredential::load`] et la présente à la connexion en signant
//! le défi du serveur. La clé privée ne transite jamais sur la connexion.

use std::path::{Path, PathBuf};

use ed25519_dalek::{Signer, SigningKey};
use zeroize::Zeroizing;

use crate::errors::ClientError;

/// Identifiant du protocole d'authentification attendu dans le défi.
pub const AUTH_PROTOCOL: &str = "kindmother-auth-v1";

/// Préfixe de séparation de domaine du message signé (identique au service).
const AUTH_DOMAIN: &[u8] = b"kindmother-auth-v1\0";

/// Taille du nonce de défi.
pub const NONCE_LEN: usize = 32;

/// Variable d'environnement : répertoire des justificatifs.
pub const ENV_CREDENTIALS_DIR: &str = "KINDMOTHER_CREDENTIALS_DIR";

/// Variable d'environnement : répertoire de données du service.
pub const ENV_DATA_DIR: &str = "KINDMOTHER_DATA_DIR";

/// Répertoire de données par défaut du service (relatif au répertoire courant).
const DEFAULT_DATA_DIR: &str = "./data/kindmother";

/// Construit le message signé : `domaine || operator_id || 0 || nonce` (format figé).
pub fn auth_message(operator_id: &str, nonce: &[u8; NONCE_LEN]) -> Vec<u8> {
    let mut message = Vec::with_capacity(AUTH_DOMAIN.len() + operator_id.len() + 1 + NONCE_LEN);
    message.extend_from_slice(AUTH_DOMAIN);
    message.extend_from_slice(operator_id.as_bytes());
    message.push(0);
    message.extend_from_slice(nonce);
    message
}

/// Justificatif d'un opérateur : identifiant et clé de signature Ed25519.
///
/// La clé est effacée de la mémoire à la destruction (`ZeroizeOnDrop`).
pub struct OperatorCredential {
    operator_id: String,
    signing_key: SigningKey,
}

impl std::fmt::Debug for OperatorCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperatorCredential")
            .field("operator_id", &self.operator_id)
            .field("signing_key", &"<redacted>")
            .finish()
    }
}

impl OperatorCredential {
    /// Construit un justificatif depuis une graine de 32 octets.
    pub fn from_seed(operator_id: &str, seed: &[u8; 32]) -> Result<Self, ClientError> {
        if !is_valid_operator_id(operator_id) {
            return Err(ClientError::InvalidRequest(format!(
                "Invalid operator id: {operator_id}"
            )));
        }
        Ok(Self {
            operator_id: operator_id.to_string(),
            signing_key: SigningKey::from_bytes(seed),
        })
    }

    /// Charge un justificatif depuis un fichier `.key` (graine hex).
    ///
    /// Sous Unix, refuse un fichier lisible par le groupe ou les autres.
    pub fn from_file(operator_id: &str, path: impl AsRef<Path>) -> Result<Self, ClientError> {
        let path = path.as_ref();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(path)
                .map_err(|e| {
                    ClientError::Authentication(format!("Cannot read credential {path:?}: {e}"))
                })?
                .permissions()
                .mode();
            if mode & 0o077 != 0 {
                return Err(ClientError::Authentication(format!(
                    "Credential file {path:?} is accessible to group/others (expected 0600)"
                )));
            }
        }
        let text = Zeroizing::new(std::fs::read_to_string(path).map_err(|e| {
            ClientError::Authentication(format!("Cannot read credential {path:?}: {e}"))
        })?);
        let seed = Zeroizing::new(decode_hex_fixed::<32>(text.trim()).ok_or_else(|| {
            ClientError::Authentication(format!("Malformed credential file {path:?}"))
        })?);
        Self::from_seed(operator_id, &seed)
    }

    /// Charge le justificatif d'un opérateur depuis le répertoire par défaut.
    ///
    /// Répertoire : `$KINDMOTHER_CREDENTIALS_DIR`, sinon `$KINDMOTHER_DATA_DIR/credentials`,
    /// sinon `./data/kindmother/credentials` (défaut du service lancé par Central).
    pub fn load(operator_id: &str) -> Result<Self, ClientError> {
        if !is_valid_operator_id(operator_id) {
            return Err(ClientError::InvalidRequest(format!(
                "Invalid operator id: {operator_id}"
            )));
        }
        let path = Self::default_dir().join(format!("{operator_id}.key"));
        Self::from_file(operator_id, path)
    }

    /// Répertoire des justificatifs selon l'environnement.
    pub fn default_dir() -> PathBuf {
        if let Ok(dir) = std::env::var(ENV_CREDENTIALS_DIR) {
            return PathBuf::from(dir);
        }
        let data_dir = std::env::var(ENV_DATA_DIR).unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string());
        PathBuf::from(data_dir).join("credentials")
    }

    /// Identifiant de l'opérateur.
    pub fn operator_id(&self) -> &str {
        &self.operator_id
    }

    /// Signe le défi du serveur ; retourne la signature en hexadécimal.
    pub(crate) fn sign_challenge(&self, nonce: &[u8; NONCE_LEN]) -> String {
        let signature = self
            .signing_key
            .sign(&auth_message(&self.operator_id, nonce));
        encode_hex(&signature.to_bytes())
    }
}

/// Vérifie qu'un identifiant d'opérateur est sûr (même règle que le service).
fn is_valid_operator_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        && !id.starts_with('-')
        && !id.starts_with('_')
}

/// Encode des octets en hexadécimal minuscule.
pub(crate) fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(char::from(DIGITS[usize::from(b >> 4)]));
        out.push(char::from(DIGITS[usize::from(b & 0x0f)]));
    }
    out
}

/// Décode un hexadécimal de longueur fixe `N` octets.
pub(crate) fn decode_hex_fixed<const N: usize>(text: &str) -> Option<[u8; N]> {
    let bytes = text.as_bytes();
    if bytes.len() != N * 2 {
        return None;
    }
    let mut out = [0u8; N];
    // Longueur vérifiée ci-dessus : les indices 2i et 2i+1 existent.
    for (i, byte) in out.iter_mut().enumerate() {
        *byte = (hex_value(bytes[2 * i])? << 4) | hex_value(bytes[2 * i + 1])?;
    }
    Some(out)
}

fn hex_value(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signature, Verifier};

    /// Vecteur figé du message signé, identique à celui du service (C-61).
    #[test]
    fn auth_message_vecteur_fige() {
        let msg = auth_message("jaykoa", &[0xab; NONCE_LEN]);
        assert_eq!(
            encode_hex(&msg),
            "6b696e646d6f746865722d617574682d763100\
             6a61796b6f6100\
             abababababababababababababababababababababababababababababababab"
        );
    }

    #[test]
    fn signature_du_defi_verifiable() {
        let cred = OperatorCredential::from_seed("jaykoa", &[7u8; 32]).expect("cred");
        let nonce = [1u8; NONCE_LEN];
        let sig_hex = cred.sign_challenge(&nonce);
        let sig = Signature::from_bytes(&decode_hex_fixed::<64>(&sig_hex).expect("hex"));
        let vk = SigningKey::from_bytes(&[7u8; 32]).verifying_key();
        assert!(vk.verify(&auth_message("jaykoa", &nonce), &sig).is_ok());
        assert!(vk.verify(&auth_message("jaykonta", &nonce), &sig).is_err());
    }

    #[test]
    fn debug_ne_revele_pas_la_cle() {
        let cred = OperatorCredential::from_seed("jaykoa", &[7u8; 32]).expect("cred");
        let text = format!("{cred:?}");
        assert!(text.contains("redacted"));
        assert!(!text.contains("0707"));
    }

    #[test]
    fn identifiant_invalide_refuse() {
        assert!(OperatorCredential::from_seed("../x", &[0u8; 32]).is_err());
        assert!(OperatorCredential::from_seed("", &[0u8; 32]).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn fichier_trop_ouvert_refuse() {
        use std::os::unix::fs::PermissionsExt;
        let path = std::env::temp_dir().join(format!("kmc-cred-{}.key", uuid::Uuid::new_v4()));
        std::fs::write(&path, format!("{}\n", encode_hex(&[3u8; 32]))).expect("write");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).expect("perm");
        assert!(OperatorCredential::from_file("jaykoa", &path).is_err());
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("perm");
        assert!(OperatorCredential::from_file("jaykoa", &path).is_ok());
        let _ = std::fs::remove_file(&path);
    }
}
