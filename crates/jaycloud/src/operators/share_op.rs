//! `share_op` — gestion des liens publics signés.
//!
//! Un `ShareLink` pointe vers un fichier d'un snapshot, optionnellement
//! protégé par mot de passe et/ou expiration.
//!
//! ## Format du token
//! Token URL-safe base32 (lowercase, sans padding) de 16 bytes random
//! = 26 caractères, 128 bits d'entropie. Pas de checksum interne (les
//! liens sont court-vie et révoqués via la table `share_links`).
//!
//! ## Mot de passe
//! Hash Argon2id (paramètres standards). Stocké dans `password_hash`.
//! `None` si lien public sans mot de passe.

use std::collections::HashMap;
use std::sync::RwLock;

use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use base32::Alphabet;
use rand::RngCore;

const TOKEN_BYTES: usize = 16;
const BASE32_ALPHABET: Alphabet = Alphabet::Rfc4648Lower { padding: false };

/// Lien public de partage (côté serveur, avec password_hash).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShareLink {
    /// Token public (la partie qui apparaît dans l'URL).
    pub token: String,
    /// Snapshot référencé.
    pub snapshot_id: String,
    /// Chemin du fichier dans le snapshot (None = snapshot complet).
    pub resource_path: Option<String>,
    /// Owner (identité COG).
    pub owner_user_id: String,
    /// Hash Argon2id du mot de passe (None = lien public sans mot de passe).
    pub password_hash: Option<String>,
    /// Timestamp Unix de création.
    pub created_at: i64,
    /// Timestamp Unix d'expiration (None = pas d'expiration).
    pub expires_at: Option<i64>,
    /// Compteur de téléchargements.
    pub download_count: u64,
}

/// Ressource résolue après vérification d'un lien.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedResource {
    /// Snapshot.
    pub snapshot_id: String,
    /// Chemin du fichier (None = snapshot complet).
    pub resource_path: Option<String>,
}

/// Erreurs de l'Opérateur Share.
#[derive(Debug, thiserror::Error)]
pub enum ShareOpError {
    /// Token inconnu.
    #[error("lien introuvable")]
    NotFound,
    /// Lien expiré.
    #[error("lien expiré")]
    Expired,
    /// Mot de passe requis mais non fourni.
    #[error("mot de passe requis")]
    PasswordRequired,
    /// Mot de passe fourni invalide.
    #[error("mot de passe invalide")]
    InvalidPassword,
    /// Erreur Argon2 (hash ou vérif).
    #[error("argon2 : {0}")]
    Argon2(String),
}

/// Opérateur Share.
pub struct ShareOp {
    /// Index `token → ShareLink`.
    links: RwLock<HashMap<String, ShareLink>>,
}

impl Default for ShareOp {
    fn default() -> Self {
        Self::new()
    }
}

impl ShareOp {
    /// Construit un opérateur vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            links: RwLock::new(HashMap::new()),
        }
    }

    /// Crée un lien public.
    ///
    /// - `expires_at` : timestamp Unix d'expiration (None = pas d'expiration).
    /// - `password` : si fourni, hash Argon2id stocké.
    pub fn create(
        &self,
        owner_user_id: &str,
        snapshot_id: &str,
        resource_path: Option<&str>,
        expires_at: Option<i64>,
        password: Option<&str>,
        now_unix: i64,
    ) -> Result<ShareLink, ShareOpError> {
        let token = generate_token();
        let password_hash = match password {
            Some(p) if !p.is_empty() => Some(hash_password(p)?),
            _ => None,
        };
        let link = ShareLink {
            token: token.clone(),
            snapshot_id: snapshot_id.to_string(),
            resource_path: resource_path.map(ToString::to_string),
            owner_user_id: owner_user_id.to_string(),
            password_hash,
            created_at: now_unix,
            expires_at,
            download_count: 0,
        };
        self.links.write().unwrap().insert(token, link.clone());
        Ok(link)
    }

    /// Résout un token. Vérifie expiration et mot de passe.
    /// Incrémente `download_count` à chaque résolution réussie.
    pub fn resolve(
        &self,
        token: &str,
        password: Option<&str>,
        now_unix: i64,
    ) -> Result<SharedResource, ShareOpError> {
        let mut links = self.links.write().unwrap();
        let link = links.get_mut(token).ok_or(ShareOpError::NotFound)?;

        if let Some(exp) = link.expires_at {
            if now_unix >= exp {
                return Err(ShareOpError::Expired);
            }
        }

        if let Some(hash) = &link.password_hash {
            let provided = password.ok_or(ShareOpError::PasswordRequired)?;
            verify_password(provided, hash)?;
        }

        link.download_count += 1;
        Ok(SharedResource {
            snapshot_id: link.snapshot_id.clone(),
            resource_path: link.resource_path.clone(),
        })
    }

    /// Récupère les métadonnées d'un lien (sans incrémenter le compteur).
    pub fn get(&self, token: &str) -> Result<ShareLink, ShareOpError> {
        self.links
            .read()
            .unwrap()
            .get(token)
            .cloned()
            .ok_or(ShareOpError::NotFound)
    }

    /// Liste les liens créés par un owner.
    pub fn list_for_owner(&self, owner_user_id: &str) -> Vec<ShareLink> {
        let links = self.links.read().unwrap();
        let mut out: Vec<_> = links
            .values()
            .filter(|l| l.owner_user_id == owner_user_id)
            .cloned()
            .collect();
        out.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        out
    }

    /// Révoque (supprime) un lien.
    pub fn revoke(&self, token: &str) -> Result<(), ShareOpError> {
        self.links
            .write()
            .unwrap()
            .remove(token)
            .map(|_| ())
            .ok_or(ShareOpError::NotFound)
    }
}

fn generate_token() -> String {
    let mut bytes = [0u8; TOKEN_BYTES];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    base32::encode(BASE32_ALPHABET, &bytes)
}

fn hash_password(password: &str) -> Result<String, ShareOpError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ShareOpError::Argon2(e.to_string()))?
        .to_string())
}

fn verify_password(provided: &str, hash: &str) -> Result<(), ShareOpError> {
    let parsed = PasswordHash::new(hash).map_err(|e| ShareOpError::Argon2(e.to_string()))?;
    Argon2::default()
        .verify_password(provided.as_bytes(), &parsed)
        .map_err(|_| ShareOpError::InvalidPassword)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_resolve_public() {
        let op = ShareOp::new();
        let link = op
            .create("u_alice", "snap_1", Some("docs/file.txt"), None, None, 1000)
            .unwrap();
        let res = op.resolve(&link.token, None, 2000).unwrap();
        assert_eq!(res.snapshot_id, "snap_1");
        assert_eq!(res.resource_path, Some("docs/file.txt".into()));
    }

    #[test]
    fn token_is_url_safe_base32_26_chars() {
        let op = ShareOp::new();
        let link = op.create("u", "s", None, None, None, 1000).unwrap();
        // 16 bytes → base32 sans padding = 26 chars
        assert_eq!(link.token.len(), 26);
        assert!(link
            .token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() && (c.is_ascii_lowercase() || c.is_ascii_digit())));
    }

    #[test]
    fn two_links_have_different_tokens() {
        let op = ShareOp::new();
        let l1 = op.create("u", "s", None, None, None, 1000).unwrap();
        let l2 = op.create("u", "s", None, None, None, 1000).unwrap();
        assert_ne!(l1.token, l2.token);
    }

    #[test]
    fn resolve_increments_download_count() {
        let op = ShareOp::new();
        let link = op.create("u", "s", None, None, None, 1000).unwrap();
        op.resolve(&link.token, None, 2000).unwrap();
        op.resolve(&link.token, None, 2001).unwrap();
        op.resolve(&link.token, None, 2002).unwrap();
        assert_eq!(op.get(&link.token).unwrap().download_count, 3);
    }

    #[test]
    fn unknown_token_fails() {
        let op = ShareOp::new();
        let r = op.resolve("nope", None, 1000);
        assert!(matches!(r, Err(ShareOpError::NotFound)));
    }

    #[test]
    fn expired_link_fails() {
        let op = ShareOp::new();
        let link = op
            .create("u", "s", None, Some(2000), None, 1000)
            .unwrap();
        // Avant expiration : OK.
        op.resolve(&link.token, None, 1999).unwrap();
        // Après expiration : refus.
        let r = op.resolve(&link.token, None, 2000);
        assert!(matches!(r, Err(ShareOpError::Expired)));
    }

    #[test]
    fn password_required_when_set() {
        let op = ShareOp::new();
        let link = op
            .create("u", "s", None, None, Some("secret123"), 1000)
            .unwrap();
        let r = op.resolve(&link.token, None, 2000);
        assert!(matches!(r, Err(ShareOpError::PasswordRequired)));
    }

    #[test]
    fn correct_password_succeeds() {
        let op = ShareOp::new();
        let link = op
            .create("u", "s", None, None, Some("secret123"), 1000)
            .unwrap();
        let r = op.resolve(&link.token, Some("secret123"), 2000).unwrap();
        assert_eq!(r.snapshot_id, "s");
    }

    #[test]
    fn wrong_password_fails() {
        let op = ShareOp::new();
        let link = op
            .create("u", "s", None, None, Some("secret123"), 1000)
            .unwrap();
        let r = op.resolve(&link.token, Some("wrong"), 2000);
        assert!(matches!(r, Err(ShareOpError::InvalidPassword)));
    }

    #[test]
    fn empty_password_treated_as_none() {
        let op = ShareOp::new();
        let link = op.create("u", "s", None, None, Some(""), 1000).unwrap();
        assert!(link.password_hash.is_none());
        op.resolve(&link.token, None, 2000).unwrap();
    }

    #[test]
    fn revoke_removes_link() {
        let op = ShareOp::new();
        let link = op.create("u", "s", None, None, None, 1000).unwrap();
        op.revoke(&link.token).unwrap();
        let r = op.resolve(&link.token, None, 2000);
        assert!(matches!(r, Err(ShareOpError::NotFound)));
    }

    #[test]
    fn revoke_unknown_fails() {
        let op = ShareOp::new();
        let r = op.revoke("ghost");
        assert!(matches!(r, Err(ShareOpError::NotFound)));
    }

    #[test]
    fn list_for_owner_filters() {
        let op = ShareOp::new();
        op.create("alice", "s1", None, None, None, 1000).unwrap();
        op.create("alice", "s2", None, None, None, 1001).unwrap();
        op.create("bob", "s3", None, None, None, 1002).unwrap();
        assert_eq!(op.list_for_owner("alice").len(), 2);
        assert_eq!(op.list_for_owner("bob").len(), 1);
    }

    #[test]
    fn list_for_owner_sorted_by_creation() {
        let op = ShareOp::new();
        op.create("u", "s", None, None, None, 3000).unwrap();
        op.create("u", "s", None, None, None, 1000).unwrap();
        op.create("u", "s", None, None, None, 2000).unwrap();
        let list = op.list_for_owner("u");
        let times: Vec<_> = list.iter().map(|l| l.created_at).collect();
        assert_eq!(times, vec![1000, 2000, 3000]);
    }
}
