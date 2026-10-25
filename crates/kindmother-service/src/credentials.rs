//! Justificatifs des opérateurs, identité du serveur et vérification du défi (K-04).
//!
//! @id     kindmother_service_credentials
//! @do     authentifier_operateurs_par_signature
//! @role   securite
//! @layer  core
//! @human  kindmother-service — justificatifs Ed25519 par opérateur, identité Ed25519 du serveur et authentification mutuelle.
//!
//! ## Principe (protocole `kindmother-auth-v2`)
//!
//! Authentification **mutuelle** par signatures Ed25519 :
//! - chaque opérateur possède une clé qui lui est propre ; le service ne garde en mémoire
//!   que la **clé publique** (aucun secret comparé côté serveur, C-22) ;
//! - le serveur possède sa propre clé d'identité ; le client **épingle** la clé publique
//!   du serveur et refuse tout serveur qui ne prouve pas la possession de la clé épinglée.
//!
//! Poignée de main :
//! 1. client → `ClientHello { protocol, client_nonce }` ;
//! 2. serveur → `Challenge { server_nonce, server_pub, server_sig }` où `server_sig`
//!    signe la transcription serveur ;
//! 3. le client vérifie `server_pub == clé épinglée` et `server_sig` (`verify_strict`),
//!    puis signe la transcription client ;
//! 4. serveur → `Authenticated` après vérification.
//!
//! Les deux transcriptions lient les deux nonces et la clé du serveur : une signature
//! d'opérateur obtenue par un relais pour une autre transcription (autre `client_nonce`,
//! autre clé de serveur) est refusée par le vrai serveur.
//!
//! ## Approvisionnement
//!
//! Au démarrage, le service garantit l'existence de :
//! - `<credentials_dir>/server.key` : graine privée du serveur (hex, `0600`) ;
//! - `<credentials_dir>/server.pub` : clé publique du serveur (hex), **épinglée** par les
//!   clients (chargée par `kindmother_client::OperatorCredential::load`) ;
//! - pour chaque opérateur, `<operator>.pub` (lue par le service) et `<operator>.key`
//!   (graine privée `0600`, créée une seule fois si aucune clé publique n'existe).
//!
//! L'administrateur peut déplacer un fichier `<operator>.key` hors du répertoire du
//! service : tant que le `.pub` existe, le service ne régénère rien. En revanche le
//! service refuse de démarrer si `server.pub` existe sans `server.key` (il ne change
//! jamais silencieusement d'identité épinglée).
//!
//! ## Transcriptions signées (format figé, C-61)
//!
//! - serveur : `"kindmother-auth-v2\0" || "server\0" || client_nonce(32) || server_nonce(32) || server_pub(32)`
//! - client : `"kindmother-auth-v2\0" || "client\0" || client_nonce(32) || server_nonce(32) || server_pub(32) || operator_id`
//!
//! Les étiquettes de rôle empêchent de réfléchir une signature d'un rôle vers l'autre.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand_core::{OsRng, RngCore};
use zeroize::Zeroizing;

use crate::errors::ServiceError;

/// Identifiant du protocole d'authentification (seule version acceptée).
pub const AUTH_PROTOCOL: &str = "kindmother-auth-v2";

/// Préfixe de séparation de domaine (protocole et version).
const AUTH_DOMAIN: &[u8] = b"kindmother-auth-v2\0";

/// Étiquette de rôle de la transcription signée par le serveur.
const ROLE_SERVER: &[u8] = b"server\0";

/// Étiquette de rôle de la transcription signée par le client.
const ROLE_CLIENT: &[u8] = b"client\0";

/// Taille des nonces (256 bits).
pub const NONCE_LEN: usize = 32;

/// Taille d'une clé publique Ed25519.
pub const PUBLIC_KEY_LEN: usize = 32;

/// Nom de base des fichiers de l'identité du serveur (`server.key`, `server.pub`).
pub const SERVER_KEY_NAME: &str = "server";

/// Longueur maximale d'un identifiant d'opérateur.
const MAX_OPERATOR_ID_LEN: usize = 64;

/// Transcription signée par le serveur.
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

/// Transcription signée par l'opérateur (client).
///
/// Tous les champs précédant `operator_id` sont de taille fixe : l'encodage est
/// sans ambiguïté.
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

/// Tire un nonce via `OsRng`.
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

/// Identité Ed25519 du serveur (clé privée effacée à la destruction).
pub struct ServerIdentity {
    signing_key: SigningKey,
    public: [u8; PUBLIC_KEY_LEN],
}

impl std::fmt::Debug for ServerIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerIdentity")
            .field("public", &encode_hex(&self.public))
            .field("signing_key", &"<redacted>")
            .finish()
    }
}

impl ServerIdentity {
    /// Charge ou crée l'identité du serveur dans `dir` (`server.key` 0600, `server.pub`).
    pub fn provision(dir: &Path) -> Result<Self, ServiceError> {
        let key_path = dir.join(format!("{SERVER_KEY_NAME}.key"));
        let pub_path = dir.join(format!("{SERVER_KEY_NAME}.pub"));

        let signing_key = if key_path.exists() {
            set_mode(&key_path, 0o600)?;
            let text = Zeroizing::new(
                std::fs::read_to_string(&key_path)
                    .map_err(|e| ServiceError::Internal(format!("Cannot read server key: {e}")))?,
            );
            let seed = Zeroizing::new(decode_hex_fixed::<32>(text.trim()).ok_or_else(|| {
                ServiceError::Authentication("Malformed server key file".to_string())
            })?);
            SigningKey::from_bytes(&seed)
        } else if pub_path.exists() {
            // Fail-closed (C-12) : ne jamais changer silencieusement d'identité épinglée.
            return Err(ServiceError::Authentication(format!(
                "Server public key {pub_path:?} exists without its private key; \
                 restore server.key or remove server.pub (clients must re-pin)"
            )));
        } else {
            let signing_key = SigningKey::generate(&mut OsRng);
            let seed_hex = Zeroizing::new(format!("{}\n", encode_hex(signing_key.as_bytes())));
            write_new_private(&key_path, seed_hex.as_bytes())?;
            tracing::info!("Provisioned KindMother server identity");
            signing_key
        };

        let public = signing_key.verifying_key().to_bytes();

        if pub_path.exists() {
            let text = std::fs::read_to_string(&pub_path).map_err(|e| {
                ServiceError::Internal(format!("Cannot read server public key: {e}"))
            })?;
            if decode_hex_fixed::<PUBLIC_KEY_LEN>(text.trim()) != Some(public) {
                return Err(ServiceError::Authentication(
                    "server.pub does not match server.key".to_string(),
                ));
            }
        } else {
            std::fs::write(&pub_path, format!("{}\n", encode_hex(&public))).map_err(|e| {
                ServiceError::Internal(format!("Cannot write server public key: {e}"))
            })?;
        }
        // Le pin est public mais son intégrité compte : non modifiable par les autres.
        set_mode(&pub_path, 0o600)?;

        Ok(Self {
            signing_key,
            public,
        })
    }

    /// Clé publique du serveur (à épingler par les clients).
    pub fn public_key(&self) -> &[u8; PUBLIC_KEY_LEN] {
        &self.public
    }

    /// Signe la transcription serveur pour les nonces donnés.
    pub fn sign_challenge(
        &self,
        client_nonce: &[u8; NONCE_LEN],
        server_nonce: &[u8; NONCE_LEN],
    ) -> Signature {
        self.signing_key
            .sign(&server_transcript(client_nonce, server_nonce, &self.public))
    }
}

/// Magasin des clés publiques des opérateurs et identité du serveur.
pub struct CredentialStore {
    /// Répertoire des justificatifs
    dir: PathBuf,
    /// Clés publiques par opérateur
    keys: HashMap<String, VerifyingKey>,
    /// Identité du serveur
    server: ServerIdentity,
}

impl CredentialStore {
    /// Approvisionne l'identité du serveur et les justificatifs des opérateurs donnés.
    pub fn provision(dir: impl AsRef<Path>, operator_ids: &[String]) -> Result<Self, ServiceError> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)
            .map_err(|e| ServiceError::Internal(format!("Cannot create credentials dir: {e}")))?;
        set_mode(&dir, 0o700)?;

        let mut keys = HashMap::new();
        for id in operator_ids {
            if !is_valid_operator_id(id) || id == SERVER_KEY_NAME {
                return Err(ServiceError::Validation(format!(
                    "Invalid operator id: {id}"
                )));
            }
            let key = Self::provision_one(&dir, id)?;
            keys.insert(id.clone(), key);
        }

        let server = ServerIdentity::provision(&dir)?;

        Ok(Self { dir, keys, server })
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

    /// Identité du serveur.
    pub fn server(&self) -> &ServerIdentity {
        &self.server
    }

    /// Clés publiques des opérateurs et du serveur : aucune ne peut servir de clé
    /// d'utilisateur (étiquette `publiable`, EX-47).
    pub(crate) fn service_public_keys(&self) -> Vec<[u8; PUBLIC_KEY_LEN]> {
        self.keys
            .values()
            .map(VerifyingKey::to_bytes)
            .chain(std::iter::once(self.server.public))
            .collect()
    }

    /// Vérifie la signature d'un opérateur sur la transcription client.
    ///
    /// Toute erreur (opérateur inconnu, signature mal formée ou invalide) produit le
    /// même refus, pour ne pas révéler quels opérateurs existent.
    pub fn verify(
        &self,
        operator_id: &str,
        client_nonce: &[u8; NONCE_LEN],
        server_nonce: &[u8; NONCE_LEN],
        signature_hex: &str,
    ) -> Result<(), ServiceError> {
        let refuse = || ServiceError::Authentication("authentication failed".to_string());

        if !is_valid_operator_id(operator_id) {
            return Err(refuse());
        }
        let key = self.keys.get(operator_id).ok_or_else(refuse)?;
        let sig_bytes = decode_hex_fixed::<64>(signature_hex).ok_or_else(refuse)?;
        let signature = Signature::from_bytes(&sig_bytes);
        let transcript = client_transcript(
            client_nonce,
            server_nonce,
            self.server.public_key(),
            operator_id,
        );
        key.verify_strict(&transcript, &signature)
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
    use ed25519_dalek::Verifier;

    fn temp_dir(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!("km-cred-{tag}-{}", uuid::Uuid::new_v4()))
    }

    const CN: [u8; NONCE_LEN] = [0x11; NONCE_LEN];
    const SN: [u8; NONCE_LEN] = [0x22; NONCE_LEN];
    const SP: [u8; PUBLIC_KEY_LEN] = [0x33; PUBLIC_KEY_LEN];

    /// Vecteur figé de la transcription serveur (C-61).
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

    /// Vecteur figé de la transcription client (C-61).
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

    /// Vecteur figé de signature (graine [7; 32]), identique côté client (C-61).
    #[test]
    fn signature_client_vecteur_fige() {
        let sk = SigningKey::from_bytes(&[7u8; 32]);
        let sig = sk.sign(&client_transcript(&CN, &SN, &SP, "jaykoa"));
        assert_eq!(encode_hex(&sig.to_bytes()), FROZEN_CLIENT_SIG);
    }

    /// Signature figée de la transcription client (graine [7; 32]).
    const FROZEN_CLIENT_SIG: &str = "d0d90d31ce6964ba62e34addaa8172b15c7e96edd10395e239474eecd398cb54\
                                     59a829acf874ddbcdaaa0114294980a191dfebed708c90cfe7ab68d967d01007";

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
            for f in ["op-a.key", "server.key", "server.pub"] {
                let mode = std::fs::metadata(dir.join(f))
                    .expect("fichier")
                    .permissions()
                    .mode();
                assert_eq!(mode & 0o777, 0o600, "{f}");
            }
        }

        // server.pub exporte la clé publique du serveur
        let pin = std::fs::read_to_string(dir.join("server.pub")).expect("pin");
        assert_eq!(
            decode_hex_fixed::<32>(pin.trim()),
            Some(*store.server().public_key())
        );

        let seed_hex = std::fs::read_to_string(dir.join("op-a.key")).expect("seed");
        let seed = decode_hex_fixed::<32>(seed_hex.trim()).expect("hex");
        let sk = SigningKey::from_bytes(&seed);
        let cn = new_nonce();
        let sn = new_nonce();
        let spub = *store.server().public_key();
        let sig = encode_hex(
            &sk.sign(&client_transcript(&cn, &sn, &spub, "op-a"))
                .to_bytes(),
        );

        assert!(store.verify("op-a", &cn, &sn, &sig).is_ok());
        // Mauvais opérateur pour la même signature
        assert!(store.verify("op-b", &cn, &sn, &sig).is_err());
        // Autre nonce serveur (rejeu) ou autre nonce client (relais)
        assert!(store.verify("op-a", &cn, &new_nonce(), &sig).is_err());
        assert!(store.verify("op-a", &new_nonce(), &sn, &sig).is_err());
        // Signature pour une autre clé de serveur (relais)
        let autre = encode_hex(
            &sk.sign(&client_transcript(&cn, &sn, &[9u8; 32], "op-a"))
                .to_bytes(),
        );
        assert!(store.verify("op-a", &cn, &sn, &autre).is_err());
        // Opérateur inconnu
        assert!(store.verify("inconnu", &cn, &sn, &sig).is_err());
        // Signature mal formée
        assert!(store.verify("op-a", &cn, &sn, "zz").is_err());

        // Signature du serveur vérifiable avec la clé exportée
        let ssig = store.server().sign_challenge(&cn, &sn);
        let vk = VerifyingKey::from_bytes(&spub).expect("vk");
        assert!(vk
            .verify(&server_transcript(&cn, &sn, &spub), &ssig)
            .is_ok());

        // Redémarrage : les clés sont relues, pas régénérées
        let store2 = CredentialStore::provision(&dir, &ids).expect("reprovision");
        assert!(store2.verify("op-a", &cn, &sn, &sig).is_ok());
        assert_eq!(store2.server().public_key(), &spub);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pin_sans_cle_serveur_refuse() {
        let dir = temp_dir("pin");
        let ids = vec!["op-a".to_string()];
        CredentialStore::provision(&dir, &ids).expect("provision");
        std::fs::remove_file(dir.join("server.key")).expect("rm");
        assert!(CredentialStore::provision(&dir, &ids).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn identifiant_invalide_refuse() {
        let dir = temp_dir("bad");
        assert!(CredentialStore::provision(&dir, &["../evil".to_string()]).is_err());
        // Nom réservé à l'identité du serveur
        assert!(CredentialStore::provision(&dir, &["server".to_string()]).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
