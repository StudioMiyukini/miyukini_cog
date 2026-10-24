//! @id     jaybureau_suite_jay_collab_auth
//! @do     authentifier_participant_collab
//! @role   securite
//! @layer  core
//! @human  jaybureau-suite — module « jay-collab auth » : jeton de connexion signé ed25519, audience serveur et annuaire de clés (R-008, R-021, R-022).

//! Authentification des participants d'une salle collaborative.
//!
//! ## Modèle
//!
//! - Chaque utilisateur possède une identité ed25519 (même mécanisme que
//!   `jaymessage::IdentityKey`, clés publiques en hex).
//! - Pour rejoindre une salle, le client envoie en **premier message**
//!   WebSocket un [`JoinRequest`] contenant un [`JoinToken`] signé sur
//!   (domaine v2, audience, doc_id, user_id, horodatage, nonce aléatoire).
//! - **Audience (R-022)** : le jeton est lié à l'identité du serveur visé
//!   (identifiant de serveur ou empreinte de sa clé publique). Un serveur
//!   refuse tout jeton émis pour une autre audience : un jeton intercepté ne
//!   peut pas être relayé vers un autre serveur.
//! - Le serveur vérifie le jeton via un [`IdentityVerifier`] (par défaut
//!   [`KeyDirectory`]) : audience, clé connue pour ce `user_id`,
//!   `verify_strict`, fraîcheur, **ACL du document**, puis seulement nonce
//!   jamais vu (anti-rejeu). Toute erreur = refus (C-12).
//! - **Anti-rejeu borné (R-021)** : seuls les principaux authentifiés **et**
//!   autorisés sur le document consomment de la place dans le cache ; chaque
//!   utilisateur est plafonné à [`MAX_NONCES_PER_USER`] nonces vivants, en plus
//!   du plafond global : un utilisateur ne peut pas saturer le cache des autres.
//! - L'identité retenue est **celle du jeton vérifié**, jamais un champ libre
//!   des messages suivants (C-20).

use crate::CollabError;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use jaybureau_core::{DocId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Mutex, RwLock};

/// Domaine de séparation des signatures de jeton.
///
/// v2 (R-022) : le message signé inclut l'audience. Les jetons v1 (sans
/// audience) sont refusés explicitement.
pub const JOIN_DOMAIN: &[u8] = b"miyukini/jay-collab/join/v2";

/// Durée de validité d'un jeton (secondes).
pub const MAX_TOKEN_AGE_SECS: i64 = 120;

/// Avance d'horloge tolérée (secondes).
pub const MAX_CLOCK_SKEW_SECS: i64 = 30;

/// Taille maximale d'un identifiant (user_id, doc_id, audience) dans un jeton (C-13).
pub const MAX_ID_LEN: usize = 256;

/// Taille du nonce de jeton (octets).
const TOKEN_NONCE_LEN: usize = 16;

/// Nombre maximal de nonces mémorisés par l'anti-rejeu, tous utilisateurs (C-13).
pub const MAX_SEEN_NONCES: usize = 100_000;

/// Nombre maximal de nonces vivants (dans la fenêtre de validité) par
/// utilisateur (R-021).
pub const MAX_NONCES_PER_USER: usize = 64;

/// Jeton de connexion à une salle, signé par l'identité de l'utilisateur.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinToken {
    /// Identité du serveur visé (R-022). Absente des jetons v1 : la valeur
    /// par défaut (vide) est refusée explicitement à la vérification.
    #[serde(default)]
    pub audience: String,
    pub user_id: UserId,
    pub doc_id: DocId,
    /// Horodatage Unix (secondes) d'émission.
    pub issued_at: i64,
    /// Nonce aléatoire (16 octets, hex).
    pub nonce: String,
    /// Signature ed25519 (64 octets, hex).
    pub signature: String,
}

/// Premier message envoyé par le client sur la WebSocket collab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JoinRequest {
    #[serde(rename = "join")]
    Join { token: JoinToken },
}

fn push_len_prefixed(m: &mut Vec<u8>, field: &[u8]) {
    // Longueurs préfixées : aucune ambiguïté de concaténation. Les champs sont
    // bornés à MAX_ID_LEN avant tout appel, la conversion ne tronque pas.
    let len = u32::try_from(field.len()).unwrap_or(u32::MAX);
    m.extend_from_slice(&len.to_be_bytes());
    m.extend_from_slice(field);
}

fn token_message(
    audience: &str,
    user_id: &str,
    doc_id: &str,
    issued_at: i64,
    nonce: &[u8],
) -> Vec<u8> {
    let mut m = Vec::with_capacity(
        JOIN_DOMAIN.len() + 12 + audience.len() + user_id.len() + doc_id.len() + 8 + nonce.len(),
    );
    m.extend_from_slice(JOIN_DOMAIN);
    push_len_prefixed(&mut m, audience.as_bytes());
    push_len_prefixed(&mut m, user_id.as_bytes());
    push_len_prefixed(&mut m, doc_id.as_bytes());
    m.extend_from_slice(&issued_at.to_be_bytes());
    m.extend_from_slice(nonce);
    m
}

fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= MAX_ID_LEN
}

/// Horodatage Unix courant (secondes).
pub fn unix_now() -> Result<i64, CollabError> {
    let d = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| CollabError::AuthInvalid("horloge système invalide".into()))?;
    i64::try_from(d.as_secs()).map_err(|_| CollabError::AuthInvalid("horloge hors bornes".into()))
}

impl JoinToken {
    /// Signe un jeton pour `user_id` sur `doc_id`, destiné au serveur
    /// `audience`, à l'instant `issued_at`.
    pub fn sign(
        signing_key: &SigningKey,
        audience: impl Into<String>,
        user_id: impl Into<UserId>,
        doc_id: impl Into<DocId>,
        issued_at: i64,
    ) -> Result<Self, CollabError> {
        use rand::RngCore;
        let audience = audience.into();
        let user_id = user_id.into();
        let doc_id = doc_id.into();
        if !valid_id(&audience) || !valid_id(&user_id) || doc_id.len() > MAX_ID_LEN {
            return Err(CollabError::AuthInvalid("identifiant vide ou trop long".into()));
        }
        let mut nonce = [0u8; TOKEN_NONCE_LEN];
        rand::rngs::OsRng.fill_bytes(&mut nonce);
        let sig: Signature =
            signing_key.sign(&token_message(&audience, &user_id, &doc_id, issued_at, &nonce));
        Ok(Self {
            audience,
            user_id,
            doc_id,
            issued_at,
            nonce: hex::encode(nonce),
            signature: hex::encode(sig.to_bytes()),
        })
    }
}

/// Vérifie un jeton de connexion et retourne l'identité authentifiée.
pub trait IdentityVerifier: Send + Sync {
    /// `expected_doc` : document de la salle visée (un jeton n'est valable
    /// que pour ce document).
    ///
    /// `authorized` : contrôle d'accès au document (ACL), appelé **après** la
    /// vérification de signature et **avant** l'enregistrement du nonce
    /// (R-021) ; `false` = refus [`CollabError::AccessDenied`] sans consommer
    /// de place dans l'anti-rejeu.
    fn verify(
        &self,
        token: &JoinToken,
        expected_doc: &str,
        authorized: &dyn Fn(&str) -> bool,
    ) -> Result<UserId, CollabError>;
}

/// Cache anti-rejeu borné : nonces par utilisateur → horodatage du jeton.
#[derive(Default)]
struct ReplayCache {
    by_user: HashMap<UserId, HashMap<[u8; TOKEN_NONCE_LEN], i64>>,
    total: usize,
}

impl ReplayCache {
    /// Purge les entrées sorties de la fenêtre de validité.
    fn purge(&mut self, horizon: i64) {
        let mut total = 0usize;
        self.by_user.retain(|_, nonces| {
            nonces.retain(|_, issued| *issued >= horizon);
            total = total.saturating_add(nonces.len());
            !nonces.is_empty()
        });
        self.total = total;
    }
}

/// Annuaire de clés publiques ed25519 (user_id → clé) avec anti-rejeu.
///
/// Seules les identités enregistrées par le serveur (source de confiance :
/// annuaire Miyukini Connect, fédération) peuvent se connecter, et seulement
/// avec un jeton émis pour l'audience de ce serveur.
pub struct KeyDirectory {
    /// Identité de ce serveur (R-022) : identifiant ou empreinte de clé.
    audience: String,
    keys: RwLock<HashMap<UserId, VerifyingKey>>,
    seen: Mutex<ReplayCache>,
    max_per_user: usize,
    max_total: usize,
}

impl KeyDirectory {
    /// Crée un annuaire pour le serveur identifié par `audience` (identifiant
    /// de serveur ou empreinte de sa clé publique). Audience vide ou trop
    /// longue = refus.
    pub fn new(audience: impl Into<String>) -> Result<Self, CollabError> {
        let audience = audience.into();
        if !valid_id(&audience) {
            return Err(CollabError::AuthInvalid("audience vide ou trop longue".into()));
        }
        Ok(Self {
            audience,
            keys: RwLock::new(HashMap::new()),
            seen: Mutex::new(ReplayCache::default()),
            max_per_user: MAX_NONCES_PER_USER,
            max_total: MAX_SEEN_NONCES,
        })
    }

    /// Ajuste les plafonds de l'anti-rejeu (bornés par les valeurs par défaut,
    /// jamais au-delà ; 0 = tout refuser).
    pub fn with_replay_limits(mut self, per_user: usize, total: usize) -> Self {
        self.max_per_user = per_user.min(MAX_NONCES_PER_USER);
        self.max_total = total.min(MAX_SEEN_NONCES);
        self
    }

    /// Audience (identité serveur) acceptée par cet annuaire.
    pub fn audience(&self) -> &str {
        &self.audience
    }

    /// Enregistre (ou remplace) la clé publique d'un utilisateur.
    pub fn register(&self, user_id: UserId, key: VerifyingKey) -> Result<(), CollabError> {
        if !valid_id(&user_id) {
            return Err(CollabError::AuthInvalid("identifiant vide ou trop long".into()));
        }
        let mut keys = self
            .keys
            .write()
            .map_err(|_| CollabError::AuthInvalid("annuaire verrouillé".into()))?;
        keys.insert(user_id, key);
        Ok(())
    }

    /// Enregistre une clé publique fournie en hex (format `PublicIdentity`).
    pub fn register_hex(&self, user_id: UserId, public_key_hex: &str) -> Result<(), CollabError> {
        let bytes: [u8; 32] = hex::decode(public_key_hex)
            .ok()
            .and_then(|b| b.as_slice().try_into().ok())
            .ok_or_else(|| CollabError::AuthInvalid("clé publique: 32 octets hex attendus".into()))?;
        let key = VerifyingKey::from_bytes(&bytes)
            .map_err(|_| CollabError::AuthInvalid("clé publique invalide".into()))?;
        self.register(user_id, key)
    }

    /// Retire un utilisateur de l'annuaire.
    pub fn revoke(&self, user_id: &str) {
        if let Ok(mut keys) = self.keys.write() {
            keys.remove(user_id);
        }
    }

    /// Vérification avec une horloge explicite (tests).
    ///
    /// Ordre : bornes → audience → document → fraîcheur → identité →
    /// signature (`verify_strict`) → ACL → anti-rejeu. Seul un principal
    /// authentifié **et** autorisé consomme une entrée du cache (R-021).
    pub fn verify_at(
        &self,
        token: &JoinToken,
        expected_doc: &str,
        now: i64,
        authorized: &dyn Fn(&str) -> bool,
    ) -> Result<UserId, CollabError> {
        if token.audience.is_empty() {
            return Err(CollabError::AuthInvalid(
                "jeton sans audience (format v1 refusé)".into(),
            ));
        }
        if !valid_id(&token.user_id)
            || token.doc_id.len() > MAX_ID_LEN
            || token.audience.len() > MAX_ID_LEN
        {
            return Err(CollabError::AuthInvalid("identifiant vide ou trop long".into()));
        }
        if token.audience != self.audience {
            return Err(CollabError::WrongAudience);
        }
        if token.doc_id != expected_doc {
            return Err(CollabError::AuthInvalid("jeton émis pour un autre document".into()));
        }
        if token.issued_at > now.saturating_add(MAX_CLOCK_SKEW_SECS)
            || now.saturating_sub(token.issued_at) > MAX_TOKEN_AGE_SECS
        {
            return Err(CollabError::TokenExpired);
        }
        let key = {
            let keys = self
                .keys
                .read()
                .map_err(|_| CollabError::AuthInvalid("annuaire verrouillé".into()))?;
            *keys.get(&token.user_id).ok_or(CollabError::UnknownIdentity)?
        };
        // Hex canonique minuscule exigé : un même nonce ne doit avoir qu'une
        // seule représentation (sinon rejeu par changement de casse).
        let nonce: [u8; TOKEN_NONCE_LEN] = decode_canonical_hex(&token.nonce)
            .ok_or_else(|| CollabError::AuthInvalid("nonce invalide".into()))?;
        let sig: [u8; 64] = decode_canonical_hex(&token.signature)
            .ok_or_else(|| CollabError::AuthInvalid("signature invalide".into()))?;
        let msg = token_message(
            &token.audience,
            &token.user_id,
            &token.doc_id,
            token.issued_at,
            &nonce,
        );
        key.verify_strict(&msg, &Signature::from_bytes(&sig))
            .map_err(|_| CollabError::AuthInvalid("signature invalide".into()))?;

        // ACL avant l'anti-rejeu (R-021) : un principal sans droit sur le
        // document ne consomme aucune entrée du cache.
        if !authorized(&token.user_id) {
            return Err(CollabError::AccessDenied);
        }

        let mut seen = self
            .seen
            .lock()
            .map_err(|_| CollabError::AuthInvalid("anti-rejeu verrouillé".into()))?;
        let horizon = now.saturating_sub(MAX_TOKEN_AGE_SECS + MAX_CLOCK_SKEW_SECS);
        seen.purge(horizon);
        let user_count = match seen.by_user.get(&token.user_id) {
            Some(nonces) if nonces.contains_key(&nonce) => {
                return Err(CollabError::TokenReplayed);
            }
            Some(nonces) => nonces.len(),
            None => 0,
        };
        // Saturation : on refuse plutôt que d'oublier des nonces (C-12).
        if user_count >= self.max_per_user {
            return Err(CollabError::TooManyTokens);
        }
        if seen.total >= self.max_total {
            return Err(CollabError::ReplayCacheSaturated);
        }
        seen.by_user
            .entry(token.user_id.clone())
            .or_default()
            .insert(nonce, token.issued_at);
        seen.total = seen.total.saturating_add(1);
        Ok(token.user_id.clone())
    }

    /// Nombre d'entrées du cache anti-rejeu (tests, supervision).
    pub fn replay_cache_len(&self) -> usize {
        self.seen.lock().map_or(0, |s| s.total)
    }
}

/// Décode une chaîne hex de longueur exacte `N` octets, uniquement si elle est
/// en forme canonique (minuscules) ; sinon `None`.
fn decode_canonical_hex<const N: usize>(s: &str) -> Option<[u8; N]> {
    if s.len() != N * 2 {
        return None;
    }
    let bytes: [u8; N] = hex::decode(s).ok()?.as_slice().try_into().ok()?;
    (hex::encode(bytes) == s).then_some(bytes)
}

impl IdentityVerifier for KeyDirectory {
    fn verify(
        &self,
        token: &JoinToken,
        expected_doc: &str,
        authorized: &dyn Fn(&str) -> bool,
    ) -> Result<UserId, CollabError> {
        self.verify_at(token, expected_doc, unix_now()?, authorized)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Audience du serveur de test.
    pub(crate) const AUD: &str = "server-a";

    pub(crate) fn new_key() -> SigningKey {
        use rand::RngCore;
        let mut seed = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut seed);
        SigningKey::from_bytes(&seed)
    }

    pub(crate) fn new_dir() -> KeyDirectory {
        KeyDirectory::new(AUD).unwrap()
    }

    fn allow(_: &str) -> bool {
        true
    }

    fn deny(_: &str) -> bool {
        false
    }

    fn sign(k: &SigningKey, user: &str, doc: &str, at: i64) -> JoinToken {
        JoinToken::sign(k, AUD, user, doc, at).unwrap()
    }

    fn verify(dir: &KeyDirectory, t: &JoinToken, doc: &str, now: i64) -> Result<UserId, CollabError> {
        dir.verify_at(t, doc, now, &allow)
    }

    #[test]
    fn valid_token_accepted_once() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = sign(&k, "alice", "doc-1", now);
        assert_eq!(verify(&dir, &t, "doc-1", now).unwrap(), "alice");
        // Rejeu refusé
        assert!(matches!(verify(&dir, &t, "doc-1", now), Err(CollabError::TokenReplayed)));
    }

    #[test]
    fn empty_audience_directory_refused() {
        assert!(KeyDirectory::new("").is_err());
        assert!(KeyDirectory::new("x".repeat(MAX_ID_LEN + 1)).is_err());
        assert!(JoinToken::sign(&new_key(), "", "alice", "doc-1", 0).is_err());
    }

    #[test]
    fn replay_with_uppercased_nonce_refused() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = sign(&k, "alice", "doc-1", now);
        // Variante non canonique refusée même avant le premier usage.
        let mut upper = t.clone();
        upper.nonce = upper.nonce.to_uppercase();
        if upper.nonce != t.nonce {
            assert!(matches!(
                verify(&dir, &upper, "doc-1", now),
                Err(CollabError::AuthInvalid(_))
            ));
        }
        assert!(verify(&dir, &t, "doc-1", now).is_ok());
        // Rejeu par changement de casse (nonce et signature) refusé.
        let mut replay = t.clone();
        replay.nonce = replay.nonce.to_uppercase();
        replay.signature = replay.signature.to_uppercase();
        assert!(verify(&dir, &replay, "doc-1", now).is_err());
        // Forcer un nonce avec lettres pour que le test soit significatif.
        let mut t2 = sign(&k, "alice", "doc-1", now);
        while t2.nonce == t2.nonce.to_uppercase() {
            t2 = sign(&k, "alice", "doc-1", now);
        }
        assert!(verify(&dir, &t2, "doc-1", now).is_ok());
        let mut r2 = t2.clone();
        r2.nonce = r2.nonce.to_uppercase();
        assert!(verify(&dir, &r2, "doc-1", now).is_err());
        assert!(matches!(verify(&dir, &t2, "doc-1", now), Err(CollabError::TokenReplayed)));
    }

    #[test]
    fn replay_cache_expires_entries() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = sign(&k, "alice", "doc-1", now);
        assert!(verify(&dir, &t, "doc-1", now).is_ok());
        assert_eq!(dir.replay_cache_len(), 1);
        // Après la fenêtre de validité, l'entrée est purgée au prochain appel.
        let later = now + MAX_TOKEN_AGE_SECS + MAX_CLOCK_SKEW_SECS + 1;
        let t2 = sign(&k, "alice", "doc-1", later);
        assert!(verify(&dir, &t2, "doc-1", later).is_ok());
        assert_eq!(dir.replay_cache_len(), 1);
        // Et le vieux jeton reste refusé (expiré).
        assert!(matches!(verify(&dir, &t, "doc-1", later), Err(CollabError::TokenExpired)));
    }

    #[test]
    fn unknown_identity_refused() {
        let dir = new_dir();
        let k = new_key();
        let now = unix_now().unwrap();
        let t = sign(&k, "mallory", "doc-1", now);
        assert!(matches!(verify(&dir, &t, "doc-1", now), Err(CollabError::UnknownIdentity)));
        assert_eq!(dir.replay_cache_len(), 0);
    }

    #[test]
    fn forged_signature_refused() {
        let dir = new_dir();
        let alice = new_key();
        let mallory = new_key();
        dir.register("alice".into(), alice.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        // Mallory signe un jeton au nom d'alice avec sa propre clé.
        let t = sign(&mallory, "alice", "doc-1", now);
        assert!(verify(&dir, &t, "doc-1", now).is_err());
        assert_eq!(dir.replay_cache_len(), 0);
    }

    #[test]
    fn tampered_token_refused() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        dir.register("bob".into(), new_key().verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let mut t = sign(&k, "alice", "doc-1", now);
        t.user_id = "bob".into();
        assert!(verify(&dir, &t, "doc-1", now).is_err());
    }

    #[test]
    fn token_for_other_doc_refused() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = sign(&k, "alice", "doc-1", now);
        assert!(verify(&dir, &t, "doc-2", now).is_err());
    }

    #[test]
    fn expired_token_refused() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let t = sign(&k, "alice", "doc-1", now - MAX_TOKEN_AGE_SECS - 1);
        assert!(matches!(verify(&dir, &t, "doc-1", now), Err(CollabError::TokenExpired)));
        let t = sign(&k, "alice", "doc-1", now + MAX_CLOCK_SKEW_SECS + 60);
        assert!(matches!(verify(&dir, &t, "doc-1", now), Err(CollabError::TokenExpired)));
    }

    #[test]
    fn empty_signature_refused() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let mut t = sign(&k, "alice", "doc-1", now);
        t.signature.clear();
        assert!(verify(&dir, &t, "doc-1", now).is_err());
    }

    // --- R-022 : audience -------------------------------------------------

    #[test]
    fn token_for_other_audience_refused() {
        let server_a = new_dir();
        let k = new_key();
        server_a.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        // Jeton émis pour le serveur B, relayé vers le serveur A.
        let t = JoinToken::sign(&k, "server-b", "alice", "doc-1", now).unwrap();
        assert!(matches!(verify(&server_a, &t, "doc-1", now), Err(CollabError::WrongAudience)));
        // Réécrire l'audience casse la signature.
        let mut relabeled = t.clone();
        relabeled.audience = AUD.into();
        assert!(matches!(
            verify(&server_a, &relabeled, "doc-1", now),
            Err(CollabError::AuthInvalid(_))
        ));
        assert_eq!(server_a.replay_cache_len(), 0);
        // Le même jeton reste valable pour le serveur B.
        let server_b = KeyDirectory::new("server-b").unwrap();
        server_b.register("alice".into(), k.verifying_key()).unwrap();
        assert!(verify(&server_b, &t, "doc-1", now).is_ok());
    }

    /// Message signé au format v1 (domaine v1, sans audience).
    fn v1_message(user_id: &str, doc_id: &str, issued_at: i64, nonce: &[u8]) -> Vec<u8> {
        let mut m = b"miyukini/jay-collab/join/v1".to_vec();
        m.extend_from_slice(&(user_id.len() as u32).to_be_bytes());
        m.extend_from_slice(user_id.as_bytes());
        m.extend_from_slice(&(doc_id.len() as u32).to_be_bytes());
        m.extend_from_slice(doc_id.as_bytes());
        m.extend_from_slice(&issued_at.to_be_bytes());
        m.extend_from_slice(nonce);
        m
    }

    #[test]
    fn old_domain_token_refused() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let nonce = [7u8; TOKEN_NONCE_LEN];
        let sig = k.sign(&v1_message("alice", "doc-1", now, &nonce));
        // Jeton v1 tel qu'émis par un ancien client (sans champ audience).
        let json = serde_json::json!({
            "user_id": "alice",
            "doc_id": "doc-1",
            "issued_at": now,
            "nonce": hex::encode(nonce),
            "signature": hex::encode(sig.to_bytes()),
        });
        let mut t: JoinToken = serde_json::from_value(json).unwrap();
        assert_eq!(t.audience, "");
        assert!(matches!(verify(&dir, &t, "doc-1", now), Err(CollabError::AuthInvalid(_))));
        // Même en ajoutant l'audience attendue, la signature v1 ne vaut rien.
        t.audience = AUD.into();
        assert!(matches!(verify(&dir, &t, "doc-1", now), Err(CollabError::AuthInvalid(_))));
        assert_eq!(dir.replay_cache_len(), 0);
    }

    // --- R-021 : anti-rejeu borné -----------------------------------------

    #[test]
    fn unauthorized_principal_does_not_consume_cache() {
        let dir = new_dir();
        let k = new_key();
        dir.register("mallory".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        for _ in 0..(MAX_NONCES_PER_USER * 2) {
            let t = sign(&k, "mallory", "doc-1", now);
            assert!(matches!(
                dir.verify_at(&t, "doc-1", now, &deny),
                Err(CollabError::AccessDenied)
            ));
        }
        assert_eq!(dir.replay_cache_len(), 0);
    }

    #[test]
    fn per_user_cap_enforced() {
        let dir = new_dir();
        let k = new_key();
        dir.register("alice".into(), k.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        for _ in 0..MAX_NONCES_PER_USER {
            assert!(verify(&dir, &sign(&k, "alice", "doc-1", now), "doc-1", now).is_ok());
        }
        assert!(matches!(
            verify(&dir, &sign(&k, "alice", "doc-1", now), "doc-1", now),
            Err(CollabError::TooManyTokens)
        ));
        assert_eq!(dir.replay_cache_len(), MAX_NONCES_PER_USER);
        // Une fois la fenêtre écoulée, l'utilisateur retrouve son quota.
        let later = now + MAX_TOKEN_AGE_SECS + MAX_CLOCK_SKEW_SECS + 1;
        assert!(verify(&dir, &sign(&k, "alice", "doc-1", later), "doc-1", later).is_ok());
        assert_eq!(dir.replay_cache_len(), 1);
    }

    #[test]
    fn flooding_user_cannot_block_others() {
        // Plafond global réduit pour rendre le scénario observable.
        let dir = new_dir().with_replay_limits(MAX_NONCES_PER_USER, MAX_NONCES_PER_USER * 2);
        let mallory = new_key();
        let bob = new_key();
        dir.register("mallory".into(), mallory.verifying_key()).unwrap();
        dir.register("bob".into(), bob.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        let mut refused = 0;
        for _ in 0..(MAX_NONCES_PER_USER * 4) {
            if verify(&dir, &sign(&mallory, "mallory", "doc-1", now), "doc-1", now).is_err() {
                refused += 1;
            }
        }
        assert_eq!(refused, MAX_NONCES_PER_USER * 3);
        assert_eq!(dir.replay_cache_len(), MAX_NONCES_PER_USER);
        // Bob se connecte toujours.
        assert_eq!(
            verify(&dir, &sign(&bob, "bob", "doc-1", now), "doc-1", now).unwrap(),
            "bob"
        );
    }

    #[test]
    fn global_cap_still_enforced() {
        let dir = new_dir().with_replay_limits(2, 3);
        let (a, b) = (new_key(), new_key());
        dir.register("alice".into(), a.verifying_key()).unwrap();
        dir.register("bob".into(), b.verifying_key()).unwrap();
        let now = unix_now().unwrap();
        assert!(verify(&dir, &sign(&a, "alice", "doc-1", now), "doc-1", now).is_ok());
        assert!(verify(&dir, &sign(&a, "alice", "doc-1", now), "doc-1", now).is_ok());
        assert!(verify(&dir, &sign(&b, "bob", "doc-1", now), "doc-1", now).is_ok());
        assert!(matches!(
            verify(&dir, &sign(&b, "bob", "doc-1", now), "doc-1", now),
            Err(CollabError::ReplayCacheSaturated)
        ));
        // Les limites ne peuvent pas dépasser les plafonds par défaut.
        let wide = new_dir().with_replay_limits(usize::MAX, usize::MAX);
        assert_eq!(wide.max_per_user, MAX_NONCES_PER_USER);
        assert_eq!(wide.max_total, MAX_SEEN_NONCES);
    }
}
