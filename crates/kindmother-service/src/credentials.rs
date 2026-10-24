//! Justificatifs des opérateurs et vérification du défi d'authentification (K-04).
//!
//! @id     kindmother_service_credentials
//! @do     authentifier_operateurs_par_signature
//! @role   securite
//! @layer  core
//! @human  kindmother-service — justificatifs Ed25519 par opérateur et vérification du défi.
//!
//! ## Principe
//!
//! Chaque opérateur possède une clé Ed25519 qui lui est propre. Le service ne garde en
//! mémoire que la **clé publique** : la vérification d'une signature ne compare aucun
//! secret (pas de comparaison à faire en temps constant côté serveur, C-22) et la fuite
//! du magasin de clés publiques ne permet pas d'usurper un opérateur.
//!
//! ## Approvisionnement
//!
//! Au démarrage, pour chaque opérateur enregistré, le service garantit l'existence de :
//! - `<credentials_dir>/<operator>.pub` : clé publique (hex), lue par le service ;
//! - `<credentials_dir>/<operator>.key` : graine privée (hex, 32 octets, droits `0600`),
//!   créée une seule fois si aucune clé publique n'existe ; c'est le justificatif que
//!   l'opérateur charge via `kindmother_client::OperatorCredential`.
//!
//! L'administrateur peut déplacer le fichier `.key` hors du répertoire du service :
//! tant que le `.pub` existe, le service ne régénère rien.
//!
//! ## Message signé (format figé, C-61)
//!
//! `"kindmother-auth-v1\0" || operator_id || "\0" || nonce(32 octets)`

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use rand_core::{OsRng, RngCore};
use zeroize::Zeroizing;

use crate::errors::ServiceError;

/// Identifiant du protocole d'authentification annoncé dans le défi.
pub const AUTH_PROTOCOL: &str = "kindmother-auth-v1";

/// Préfixe de séparation de domaine du message signé.
const AUTH_DOMAIN: &[u8] = b"kindmother-auth-v1\0";

/// Taille du nonce de défi (256 bits).
pub const NONCE_LEN: usize = 32;

/// Longueur maximale d'un identifiant d'opérateur.
const MAX_OPERATOR_ID_LEN: usize = 64;

/// Construit le message signé par l'opérateur.
pub fn auth_message(operator_id: &str, nonce: &[u8; NONCE_LEN]) -> Vec<u8> {
    let mut message = Vec::with_capacity(AUTH_DOMAIN.len() + operator_id.len() + 1 + NONCE_LEN);
    message.extend_from_slice(AUTH_DOMAIN);
    message.extend_from_slice(operator_id.as_bytes());
    message.push(0);
    message.extend_from_slice(nonce);
    message
}

/// Tire un nonce de défi via `OsRng`.
pub fn new_nonce() -> [u8; NONCE_LEN] {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Vérifie qu'un identifiant d'opérateur est sûr (nom de fichier, pas d'octet nul).
pub fn is_valid_operator_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= MAX_OPERATOR_ID_LEN
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        && !id.starts_with('-')
        && !id.starts_with('_')
}

/// Encode des octets en hexadécimal minuscule.
pub fn encode_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(char::from(DIGITS[usize::from(b >> 4)]));
        out.push(char::from(DIGITS[usize::from(b & 0x0f)]));
    }
    out
}

/// Décode un hexadécimal de longueur fixe `N` octets.
pub fn decode_hex_fixed<const N: usize>(text: &str) -> Option<[u8; N]> {
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

/// Magasin des clés publiques des opérateurs.
pub struct CredentialStore {
    /// Répertoire des justificatifs
    dir: PathBuf,
    /// Clés publiques par opérateur
    keys: HashMap<String, VerifyingKey>,
}

impl CredentialStore {
    /// Approvisionne les justificatifs des opérateurs donnés et charge leurs clés publiques.
    pub fn provision(dir: impl AsRef<Path>, operator_ids: &[String]) -> Result<Self, ServiceError> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir).map_err(|e| {
            ServiceError::Internal(format!("Cannot create credentials dir: {e}"))
        })?;
        set_mode(&dir, 0o700)?;

        let mut keys = HashMap::new();
        for id in operator_ids {
            if !is_valid_operator_id(id) {
                return Err(ServiceError::Validation(format!(
                    "Invalid operator id: {id}"
                )));
            }
            let key = Self::provision_one(&dir, id)?;
            keys.insert(id.clone(), key);
        }

        Ok(Self { dir, keys })
    }

    /// Garantit l'existence du justificatif d'un opérateur et retourne sa clé publique.
    fn provision_one(dir: &Path, id: &str) -> Result<VerifyingKey, ServiceError> {
        let pub_path = dir.join(format!("{id}.pub"));
        let key_path = dir.join(format!("{id}.key"));

        if pub_path.exists() {
            let text = std::fs::read_to_string(&pub_path)
                .map_err(|e| ServiceError::Internal(format!("Cannot read public key: {e}")))?;
            let bytes = decode_hex_fixed::<32>(text.trim()).ok_or_else(|| {
                ServiceError::Authentication(format!("Malformed public key for '{id}'"))
            })?;
            return VerifyingKey::from_bytes(&bytes).map_err(|_| {
                ServiceError::Authentication(format!("Invalid public key for '{id}'"))
            });
        }

        let signing_key = if key_path.exists() {
            // Graine existante sans clé publique : on la dérive.
            set_mode(&key_path, 0o600)?;
            let text = Zeroizing::new(
                std::fs::read_to_string(&key_path)
                    .map_err(|e| ServiceError::Internal(format!("Cannot read key: {e}")))?,
            );
            let seed = Zeroizing::new(decode_hex_fixed::<32>(text.trim()).ok_or_else(|| {
                ServiceError::Authentication(format!("Malformed key file for '{id}'"))
            })?);
            SigningKey::from_bytes(&seed)
        } else {
            let signing_key = SigningKey::generate(&mut OsRng);
            let seed_hex = Zeroizing::new(format!("{}\n", encode_hex(signing_key.as_bytes())));
            write_new_private(&key_path, seed_hex.as_bytes())?;
            tracing::info!("Provisioned credential for operator '{id}'");
            signing_key
        };

        let verifying_key = signing_key.verifying_key();
        std::fs::write(
            &pub_path,
            format!("{}\n", encode_hex(verifying_key.as_bytes())),
        )
        .map_err(|e| ServiceError::Internal(format!("Cannot write public key: {e}")))?;
        set_mode(&pub_path, 0o600)?;

        Ok(verifying_key)
    }

    /// Répertoire des justificatifs.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Vérifie la réponse d'un opérateur au défi `nonce`.
    ///
    /// Toute erreur (opérateur inconnu, signature mal formée ou invalide) produit le
    /// même refus, pour ne pas révéler quels opérateurs existent.
    pub fn verify(
        &self,
        operator_id: &str,
        nonce: &[u8; NONCE_LEN],
        signature_hex: &str,
    ) -> Result<(), ServiceError> {
        let refuse = || ServiceError::Authentication("authentication failed".to_string());

        if !is_valid_operator_id(operator_id) {
            return Err(refuse());
        }
        let key = self.keys.get(operator_id).ok_or_else(refuse)?;
        let sig_bytes = decode_hex_fixed::<64>(signature_hex).ok_or_else(refuse)?;
        let signature = Signature::from_bytes(&sig_bytes);
        key.verify_strict(&auth_message(operator_id, nonce), &signature)
            .map_err(|_| refuse())
    }
}

/// Crée un fichier privé (échoue s'il existe déjà) avec les droits `0600`.
fn write_new_private(path: &Path, content: &[u8]) -> Result<(), ServiceError> {
    use std::io::Write;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| ServiceError::Internal(format!("Cannot create key file: {e}")))?;
    file.write_all(content)
        .and_then(|()| file.sync_all())
        .map_err(|e| ServiceError::Internal(format!("Cannot write key file: {e}")))
}

/// Applique des droits restrictifs (Unix uniquement).
fn set_mode(path: &Path, mode: u32) -> Result<(), ServiceError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
            .map_err(|e| ServiceError::Internal(format!("Cannot set permissions: {e}")))?;
    }
    #[cfg(not(unix))]
    {
        let _ = (path, mode);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signer;

    fn temp_dir(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("km-cred-{tag}-{}", uuid::Uuid::new_v4()))
    }

    /// Vecteur figé du message signé (C-61).
    #[test]
    fn auth_message_vecteur_fige() {
        let nonce = [0xabu8; NONCE_LEN];
        let msg = auth_message("jaykoa", &nonce);
        assert_eq!(
            encode_hex(&msg),
            "6b696e646d6f746865722d617574682d763100\
             6a61796b6f6100\
             abababababababababababababababababababababababababababababababab"
        );
    }

    #[test]
    fn hex_aller_retour() {
        let bytes = [0u8, 1, 0x7f, 0x80, 0xff];
        let text = encode_hex(&bytes);
        assert_eq!(text, "00017f80ff");
        assert_eq!(decode_hex_fixed::<5>(&text), Some(bytes));
        assert_eq!(decode_hex_fixed::<5>("00017f80f"), None);
        assert_eq!(decode_hex_fixed::<5>("00017f80fg"), None);
    }

    #[test]
    fn approvisionnement_puis_verification() {
        let dir = temp_dir("prov");
        let ids = vec!["op-a".to_string(), "op-b".to_string()];
        let store = CredentialStore::provision(&dir, &ids).expect("provision");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(dir.join("op-a.key"))
                .expect("key")
                .permissions()
                .mode();
            assert_eq!(mode & 0o777, 0o600);
        }

        let seed_hex = std::fs::read_to_string(dir.join("op-a.key")).expect("seed");
        let seed = decode_hex_fixed::<32>(seed_hex.trim()).expect("hex");
        let sk = SigningKey::from_bytes(&seed);
        let nonce = new_nonce();
        let sig = encode_hex(&sk.sign(&auth_message("op-a", &nonce)).to_bytes());

        assert!(store.verify("op-a", &nonce, &sig).is_ok());
        // Mauvais opérateur pour la même signature
        assert!(store.verify("op-b", &nonce, &sig).is_err());
        // Autre nonce (rejeu)
        assert!(store.verify("op-a", &new_nonce(), &sig).is_err());
        // Opérateur inconnu
        assert!(store.verify("inconnu", &nonce, &sig).is_err());
        // Signature mal formée
        assert!(store.verify("op-a", &nonce, "zz").is_err());

        // Redémarrage : les clés sont relues, pas régénérées
        let store2 = CredentialStore::provision(&dir, &ids).expect("reprovision");
        assert!(store2.verify("op-a", &nonce, &sig).is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn identifiant_invalide_refuse() {
        let dir = temp_dir("bad");
        let ids = vec!["../evil".to_string()];
        assert!(CredentialStore::provision(&dir, &ids).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
