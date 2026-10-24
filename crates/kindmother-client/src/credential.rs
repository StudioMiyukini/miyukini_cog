//! Justificatif d'opérateur et pin du serveur côté client (K-04).
//!
//! @id     kindmother_client_credential
//! @do     charger_justificatif_operateur_et_signer_defi
//! @role   securite
//! @layer  toolkit
//! @human  kindmother-client — clé Ed25519 de l'opérateur, pin de la clé du serveur et authentification mutuelle.
//!
//! Le service KindMother approvisionne dans `<KINDMOTHER_DATA_DIR>/credentials/` :
//! - `<operator>.key` : graine Ed25519 de l'opérateur (droits `0600`) ;
//! - `server.pub` : clé publique du serveur, que le client **épingle**.
//!
//! [`OperatorCredential::load`] charge les deux ; sans pin, la connexion est refusée.
//! Pendant la poignée de main (`kindmother-auth-v2`), le client vérifie que le serveur
//! présente la clé épinglée et signe la transcription serveur (`verify_strict`), puis
//! signe la transcription client, liée aux deux nonces et à la clé du serveur : une
//! signature relayée par un faux serveur est inutilisable auprès du vrai. La clé privée
//! ne transite jamais sur la connexion.
//!
//! Transcriptions (format figé, identique au service) :
//! - serveur : `"kindmother-auth-v2\0" || "server\0" || client_nonce || server_nonce || server_pub`
//! - client : `"kindmother-auth-v2\0" || "client\0" || client_nonce || server_nonce || server_pub || operator_id`

use std::path::{Path, PathBuf};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use zeroize::Zeroizing;

use crate::errors::ClientError;

/// Identifiant du protocole d'authentification (seule version acceptée).
pub const AUTH_PROTOCOL: &str = "kindmother-auth-v2";

/// Préfixe de séparation de domaine (protocole et version, identique au service).
const AUTH_DOMAIN: &[u8] = b"kindmother-auth-v2\0";

/// Étiquette de rôle de la transcription signée par le serveur.
const ROLE_SERVER: &[u8] = b"server\0";

/// Étiquette de rôle de la transcription signée par le client.
const ROLE_CLIENT: &[u8] = b"client\0";

/// Taille des nonces.
pub const NONCE_LEN: usize = 32;

/// Taille d'une clé publique Ed25519.
pub const PUBLIC_KEY_LEN: usize = 32;

/// Nom du fichier de pin de la clé du serveur dans le répertoire des justificatifs.
pub const SERVER_PIN_FILE: &str = "server.pub";

/// Variable d'environnement : répertoire des justificatifs.
pub const ENV_CREDENTIALS_DIR: &str = "KINDMOTHER_CREDENTIALS_DIR";

/// Variable d'environnement : répertoire de données du service.
pub const ENV_DATA_DIR: &str = "KINDMOTHER_DATA_DIR";

/// Répertoire de données par défaut du service (relatif au répertoire courant).
const DEFAULT_DATA_DIR: &str = "./data/kindmother";

/// Transcription signée par le serveur (format figé).
pub fn server_transcript(
    client_nonce: &[u8; NONCE_LEN],
    server_nonce: &[u8; NONCE_LEN],
    server_pub: &[u8; PUBLIC_KEY_LEN],
) -> Vec<u8> {
    let mut t =
        Vec::with_capacity(AUTH_DOMAIN.len() + ROLE_SERVER.len() + 2 * NONCE_LEN + PUBLIC_KEY_LEN);
    t.extend_from_slice(AUTH_DOMAIN);
    t.extend_from_slice(ROLE_SERVER);
    t.extend_from_slice(client_nonce);
    t.extend_from_slice(server_nonce);
    t.extend_from_slice(server_pub);
    t
}

/// Transcription signée par l'opérateur (format figé).
pub fn client_transcript(
    client_nonce: &[u8; NONCE_LEN],
    server_nonce: &[u8; NONCE_LEN],
    server_pub: &[u8; PUBLIC_KEY_LEN],
    operator_id: &str,
) -> Vec<u8> {
    let mut t = Vec::with_capacity(
        AUTH_DOMAIN.len() + ROLE_CLIENT.len() + 2 * NONCE_LEN + PUBLIC_KEY_LEN + operator_id.len(),
    );
    t.extend_from_slice(AUTH_DOMAIN);
    t.extend_from_slice(ROLE_CLIENT);
    t.extend_from_slice(client_nonce);
    t.extend_from_slice(server_nonce);
    t.extend_from_slice(server_pub);
    t.extend_from_slice(operator_id.as_bytes());
    t
}

/// Justificatif d'un opérateur : identifiant, clé de signature Ed25519 et pin du serveur.
///
/// La clé est effacée de la mémoire à la destruction (`ZeroizeOnDrop`).
pub struct OperatorCredential {
    operator_id: String,
    signing_key: SigningKey,
    server_pin: Option<VerifyingKey>,
}

impl std::fmt::Debug for OperatorCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OperatorCredential")
            .field("operator_id", &self.operator_id)
            .field("signing_key", &"<redacted>")
            .field(
                "server_pin",
                &self.server_pin.map(|k| encode_hex(k.as_bytes())),
            )
            .finish()
    }
}

impl OperatorCredential {
    /// Construit un justificatif depuis une graine de 32 octets (sans pin du serveur).
    pub fn from_seed(operator_id: &str, seed: &[u8; 32]) -> Result<Self, ClientError> {
        if !is_valid_operator_id(operator_id) {
            return Err(ClientError::InvalidRequest(format!(
                "Invalid operator id: {operator_id}"
            )));
        }
        Ok(Self {
            operator_id: operator_id.to_string(),
            signing_key: SigningKey::from_bytes(seed),
            server_pin: None,
        })
    }

    /// Charge un justificatif depuis un fichier `.key` (graine hex), sans pin du serveur.
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

    /// Épingle la clé publique du serveur (32 octets).
    pub fn with_server_pin(
        mut self,
        server_pub: &[u8; PUBLIC_KEY_LEN],
    ) -> Result<Self, ClientError> {
        let key = VerifyingKey::from_bytes(server_pub).map_err(|_| {
            ClientError::Authentication("Invalid server public key pin".to_string())
        })?;
        self.server_pin = Some(key);
        Ok(self)
    }

    /// Épingle la clé du serveur depuis un fichier `server.pub` (hex).
    ///
    /// Sous Unix, refuse un fichier modifiable par le groupe ou les autres.
    pub fn with_server_pin_file(self, path: impl AsRef<Path>) -> Result<Self, ClientError> {
        let path = path.as_ref();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(path)
                .map_err(|e| {
                    ClientError::Authentication(format!("Cannot read server pin {path:?}: {e}"))
                })?
                .permissions()
                .mode();
            if mode & 0o022 != 0 {
                return Err(ClientError::Authentication(format!(
                    "Server pin {path:?} is writable by group/others"
                )));
            }
        }
        let text = std::fs::read_to_string(path).map_err(|e| {
            ClientError::Authentication(format!("Cannot read server pin {path:?}: {e}"))
        })?;
        let bytes = decode_hex_fixed::<PUBLIC_KEY_LEN>(text.trim())
            .ok_or_else(|| ClientError::Authentication(format!("Malformed server pin {path:?}")))?;
        self.with_server_pin(&bytes)
    }

    /// Charge le justificatif d'un opérateur et le pin du serveur depuis le répertoire
    /// par défaut ; refuse si l'un des deux manque.
    ///
    /// Répertoire : `$KINDMOTHER_CREDENTIALS_DIR`, sinon `$KINDMOTHER_DATA_DIR/credentials`,
    /// sinon `./data/kindmother/credentials` (défaut du service lancé par Central).
    pub fn load(operator_id: &str) -> Result<Self, ClientError> {
        Self::load_from_dir(operator_id, Self::default_dir())
    }

    /// Charge `<dir>/<operator>.key` et le pin `<dir>/server.pub` ; refuse si l'un manque.
    pub fn load_from_dir(operator_id: &str, dir: impl AsRef<Path>) -> Result<Self, ClientError> {
        if !is_valid_operator_id(operator_id) {
            return Err(ClientError::InvalidRequest(format!(
                "Invalid operator id: {operator_id}"
            )));
        }
        let dir = dir.as_ref();
        Self::from_file(operator_id, dir.join(format!("{operator_id}.key")))?
            .with_server_pin_file(dir.join(SERVER_PIN_FILE))
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

    /// Clé du serveur épinglée, si présente.
    pub fn server_pin(&self) -> Option<[u8; PUBLIC_KEY_LEN]> {
        self.server_pin.map(|k| k.to_bytes())
    }

    /// Vérifie la preuve d'identité du serveur : `server_pub` doit égaler la clé
    /// épinglée et `server_sig` signer la transcription serveur (`verify_strict`).
    pub(crate) fn verify_server(
        &self,
        client_nonce: &[u8; NONCE_LEN],
        server_nonce: &[u8; NONCE_LEN],
        server_pub: &[u8; PUBLIC_KEY_LEN],
        server_sig_hex: &str,
    ) -> Result<(), ClientError> {
        let pin = self.server_pin.ok_or_else(|| {
            ClientError::Authentication("No pinned server key (server.pub missing)".to_string())
        })?;
        if pin.as_bytes() != server_pub {
            return Err(ClientError::Authentication(
                "Server key does not match the pinned key".to_string(),
            ));
        }
        let sig_bytes = decode_hex_fixed::<64>(server_sig_hex)
            .ok_or_else(|| ClientError::Authentication("Malformed server signature".to_string()))?;
        pin.verify_strict(
            &server_transcript(client_nonce, server_nonce, server_pub),
            &Signature::from_bytes(&sig_bytes),
        )
        .map_err(|_| ClientError::Authentication("Invalid server signature".to_string()))
    }

    /// Signe la transcription client ; retourne la signature en hexadécimal.
    pub(crate) fn sign_transcript(
        &self,
        client_nonce: &[u8; NONCE_LEN],
        server_nonce: &[u8; NONCE_LEN],
        server_pub: &[u8; PUBLIC_KEY_LEN],
    ) -> String {
        let signature = self.signing_key.sign(&client_transcript(
            client_nonce,
            server_nonce,
            server_pub,
            &self.operator_id,
        ));
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
    use ed25519_dalek::Verifier;

    const CN: [u8; NONCE_LEN] = [0x11; NONCE_LEN];
    const SN: [u8; NONCE_LEN] = [0x22; NONCE_LEN];
    const SP: [u8; PUBLIC_KEY_LEN] = [0x33; PUBLIC_KEY_LEN];

    /// Vecteur figé de la transcription serveur, identique au service (C-61).
    #[test]
    fn transcription_serveur_vecteur_fige() {
        assert_eq!(
            encode_hex(&server_transcript(&CN, &SN, &SP)),
            "6b696e646d6f746865722d617574682d763200\
             73657276657200\
             1111111111111111111111111111111111111111111111111111111111111111\
             2222222222222222222222222222222222222222222222222222222222222222\
             3333333333333333333333333333333333333333333333333333333333333333"
        );
    }

    /// Vecteur figé de la transcription client, identique au service (C-61).
    #[test]
    fn transcription_client_vecteur_fige() {
        assert_eq!(
            encode_hex(&client_transcript(&CN, &SN, &SP, "jaykoa")),
            "6b696e646d6f746865722d617574682d763200\
             636c69656e7400\
             1111111111111111111111111111111111111111111111111111111111111111\
             2222222222222222222222222222222222222222222222222222222222222222\
             3333333333333333333333333333333333333333333333333333333333333333\
             6a61796b6f61"
        );
    }

    /// Vecteur figé de signature (graine [7; 32]), identique au service (C-61).
    #[test]
    fn signature_client_vecteur_fige() {
        let cred = OperatorCredential::from_seed("jaykoa", &[7u8; 32]).expect("cred");
        assert_eq!(cred.sign_transcript(&CN, &SN, &SP), FROZEN_CLIENT_SIG);
    }

    /// Signature figée de la transcription client (graine [7; 32]).
    const FROZEN_CLIENT_SIG: &str = "d0d90d31ce6964ba62e34addaa8172b15c7e96edd10395e239474eecd398cb54\
                                     59a829acf874ddbcdaaa0114294980a191dfebed708c90cfe7ab68d967d01007";

    #[test]
    fn signature_transcription_verifiable() {
        let cred = OperatorCredential::from_seed("jaykoa", &[7u8; 32]).expect("cred");
        let sig_hex = cred.sign_transcript(&CN, &SN, &SP);
        let sig = Signature::from_bytes(&decode_hex_fixed::<64>(&sig_hex).expect("hex"));
        let vk = SigningKey::from_bytes(&[7u8; 32]).verifying_key();
        assert!(vk
            .verify(&client_transcript(&CN, &SN, &SP, "jaykoa"), &sig)
            .is_ok());
        assert!(vk
            .verify(&client_transcript(&CN, &SN, &SP, "jaykonta"), &sig)
            .is_err());
        assert!(vk
            .verify(&client_transcript(&SN, &SN, &SP, "jaykoa"), &sig)
            .is_err());
    }

    #[test]
    fn verification_du_serveur() {
        let server = SigningKey::from_bytes(&[5u8; 32]);
        let spub = server.verifying_key().to_bytes();
        let sig = encode_hex(&server.sign(&server_transcript(&CN, &SN, &spub)).to_bytes());

        // Sans pin : refus
        let sans_pin = OperatorCredential::from_seed("jaykoa", &[7u8; 32]).expect("cred");
        assert!(sans_pin.verify_server(&CN, &SN, &spub, &sig).is_err());

        let cred = OperatorCredential::from_seed("jaykoa", &[7u8; 32])
            .expect("cred")
            .with_server_pin(&spub)
            .expect("pin");
        assert!(cred.verify_server(&CN, &SN, &spub, &sig).is_ok());
        // Autre nonce client (relais) : refus
        assert!(cred.verify_server(&SN, &SN, &spub, &sig).is_err());
        // Autre clé de serveur présentée : refus
        let autre = SigningKey::from_bytes(&[6u8; 32]);
        let apub = autre.verifying_key().to_bytes();
        let asig = encode_hex(&autre.sign(&server_transcript(&CN, &SN, &apub)).to_bytes());
        assert!(cred.verify_server(&CN, &SN, &apub, &asig).is_err());
        // Signature mal formée
        assert!(cred.verify_server(&CN, &SN, &spub, "zz").is_err());
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

    #[cfg(unix)]
    #[test]
    fn chargement_sans_pin_refuse() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("kmc-dir-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("dir");
        let key = dir.join("jaykoa.key");
        std::fs::write(&key, format!("{}\n", encode_hex(&[3u8; 32]))).expect("write");
        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o600)).expect("perm");
        // Pin absent : refus
        assert!(OperatorCredential::load_from_dir("jaykoa", &dir).is_err());
        // Pin présent mais modifiable par d'autres : refus
        let pin = dir.join(SERVER_PIN_FILE);
        let spub = SigningKey::from_bytes(&[5u8; 32])
            .verifying_key()
            .to_bytes();
        std::fs::write(&pin, format!("{}\n", encode_hex(&spub))).expect("write");
        std::fs::set_permissions(&pin, std::fs::Permissions::from_mode(0o666)).expect("perm");
        assert!(OperatorCredential::load_from_dir("jaykoa", &dir).is_err());
        std::fs::set_permissions(&pin, std::fs::Permissions::from_mode(0o600)).expect("perm");
        let cred = OperatorCredential::load_from_dir("jaykoa", &dir).expect("load");
        assert_eq!(cred.server_pin(), Some(spub));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
