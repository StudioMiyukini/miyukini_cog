//! @id     miyucloud_infrastructure_services_session_token_service
//! @do     gerer_sessions_opaques
//! @role   securite
//! @layer  app
//! @human  miyucloud — sessions d'accès opaques côté serveur (remplace le JWT HS256 à secret partagé).

//! Magasin de sessions d'accès côté serveur (inventaire des secrets partagés #18).
//!
//! Avant : jeton d'accès JWT HS256 signé par `MIYUCLOUD_JWT_SECRET` (secret partagé
//! par tout ce qui devait vérifier ou émettre un jeton). Après :
//!
//! - le jeton d'accès est un **identifiant aléatoire** de 256 bits (`OsRng`),
//!   encodé en base64 URL ; il ne contient rien et ne se vérifie pas hors du serveur ;
//! - l'identité qu'il désigne est conservée en mémoire, indexée par le BLAKE3 du
//!   jeton (le jeton lui-même n'est jamais stocké) ;
//! - **révocable** : déconnexion (une session), changement de mot de passe,
//!   désactivation ou suppression du compte (toutes les sessions de l'utilisateur) ;
//! - **borné** : expiration (`MIYUCLOUD_ACCESS_TOKEN_EXPIRY_SECS`) et nombre maximal
//!   de sessions (plein = refus, fail-closed).
//!
//! Les sessions d'accès ne survivent pas à un redémarrage : le navigateur en
//! obtient une nouvelle grâce au jeton de rafraîchissement (table `auth.sessions`,
//! révocable). Un déploiement à plusieurs instances devra adosser ce magasin à un
//! stockage partagé (le chart Helm déploie une seule réplique).

use chrono::Utc;
use dashmap::DashMap;
use rand_core::{OsRng, RngCore};
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

use crate::application::ports::auth_ports::{TokenClaims, TokenServicePort};
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::user::User;

/// Préfixe des jetons d'accès (lisibilité des journaux, aucune valeur de sécurité).
pub const PREFIXE_JETON: &str = "mcs_";

/// Nombre maximal de sessions d'accès simultanées (plein = refus).
pub const SESSIONS_MAX_DEFAUT: usize = 200_000;

/// Taille d'un identifiant de session (octets aléatoires).
const TAILLE_JETON: usize = 32;

/// Encode en base64 URL sans remplissage.
fn b64(octets: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(octets)
}

/// Identifiant aléatoire (256 bits) préfixé.
fn jeton_aleatoire(prefixe: &str) -> String {
    let mut octets = [0u8; TAILLE_JETON];
    OsRng.fill_bytes(&mut octets);
    format!("{prefixe}{}", b64(&octets))
}

/// Clé d'index : BLAKE3 du jeton (le jeton n'est pas conservé en clair).
#[inline]
fn cle(jeton: &str) -> [u8; 32] {
    blake3::hash(jeton.as_bytes()).into()
}

/// Magasin de sessions d'accès opaques (implémente [`TokenServicePort`]).
pub struct SessionTokenService {
    sessions: DashMap<[u8; 32], TokenClaims>,
    access_token_expiry: i64,
    refresh_token_expiry: i64,
    capacite: usize,
    ouvertes: AtomicU64,
    revoquees: AtomicU64,
}

impl SessionTokenService {
    /// Magasin avec les durées de vie données (secondes) et la capacité par défaut.
    pub fn new(access_token_expiry_secs: i64, refresh_token_expiry_secs: i64) -> Self {
        Self::with_capacity(
            access_token_expiry_secs,
            refresh_token_expiry_secs,
            SESSIONS_MAX_DEFAUT,
        )
    }

    /// Magasin de `capacite` sessions au plus.
    pub fn with_capacity(
        access_token_expiry_secs: i64,
        refresh_token_expiry_secs: i64,
        capacite: usize,
    ) -> Self {
        Self {
            sessions: DashMap::new(),
            access_token_expiry: access_token_expiry_secs.max(1),
            refresh_token_expiry: refresh_token_expiry_secs.max(1),
            capacite: capacite.max(1),
            ouvertes: AtomicU64::new(0),
            revoquees: AtomicU64::new(0),
        }
    }

    /// Retire les sessions expirées ; rend le nombre retiré.
    pub fn purge_expired(&self) -> usize {
        let now = Utc::now().timestamp();
        let avant = self.sessions.len();
        self.sessions.retain(|_, c| c.exp > now);
        avant.saturating_sub(self.sessions.len())
    }

    /// Nombre de sessions en mémoire (expirées non purgées comprises).
    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    /// Vrai si aucune session.
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    /// Statistiques (sessions ouvertes, sessions révoquées) depuis le démarrage.
    pub fn stats(&self) -> (u64, u64) {
        (
            self.ouvertes.load(Ordering::Relaxed),
            self.revoquees.load(Ordering::Relaxed),
        )
    }

    fn refus(message: &str) -> DomainError {
        DomainError::new(ErrorKind::AccessDenied, "SessionToken", message.to_string())
    }
}

impl TokenServicePort for SessionTokenService {
    fn generate_access_token(&self, user: &User) -> Result<String, DomainError> {
        if self.sessions.len() >= self.capacite {
            self.purge_expired();
            if self.sessions.len() >= self.capacite {
                tracing::error!("Magasin de sessions plein ({}) : refus", self.capacite);
                return Err(DomainError::new(
                    ErrorKind::InternalError,
                    "SessionToken",
                    "Too many active sessions",
                ));
            }
        }
        let now = Utc::now().timestamp();
        let jeton = jeton_aleatoire(PREFIXE_JETON);
        let claims = TokenClaims {
            sub: user.id().to_string(),
            exp: now + self.access_token_expiry,
            iat: now,
            jti: Uuid::new_v4().to_string(),
            username: user.username().to_string(),
            email: user.email().to_string(),
            role: format!("{}", user.role()),
        };
        self.sessions.insert(cle(&jeton), claims);
        self.ouvertes.fetch_add(1, Ordering::Relaxed);
        Ok(jeton)
    }

    fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        if !token.starts_with(PREFIXE_JETON) || token.len() > 128 {
            return Err(Self::refus("Invalid token"));
        }
        let k = cle(token);
        let claims = self
            .sessions
            .get(&k)
            .map(|c| c.clone())
            .ok_or_else(|| Self::refus("Invalid token"))?;
        if claims.exp <= Utc::now().timestamp() {
            self.sessions.remove(&k);
            return Err(Self::refus("Token expired"));
        }
        Ok(claims)
    }

    fn generate_refresh_token(&self) -> String {
        jeton_aleatoire("mcr_")
    }

    fn refresh_token_expiry_secs(&self) -> i64 {
        self.refresh_token_expiry
    }

    fn refresh_token_expiry_days(&self) -> i64 {
        (self.refresh_token_expiry / (24 * 3600)).max(1)
    }

    fn access_token_expiry_secs(&self) -> i64 {
        self.access_token_expiry
    }

    fn revoke_token(&self, token: &str) -> bool {
        let retiree = self.sessions.remove(&cle(token)).is_some();
        if retiree {
            self.revoquees.fetch_add(1, Ordering::Relaxed);
        }
        retiree
    }

    fn revoke_user(&self, user_id: Uuid) -> usize {
        let sub = user_id.to_string();
        let avant = self.sessions.len();
        self.sessions.retain(|_, c| c.sub != sub);
        let n = avant.saturating_sub(self.sessions.len());
        self.revoquees.fetch_add(n as u64, Ordering::Relaxed);
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::user::{User, UserRole};

    fn utilisateur(id: &str, nom: &str) -> User {
        User::from_data(
            Uuid::parse_str(id).unwrap(),
            nom.to_string(),
            format!("{nom}@example.com"),
            "hashed_password".to_string(),
            UserRole::User,
            1024 * 1024 * 1024,
            0,
            chrono::Utc::now(),
            chrono::Utc::now(),
            None,
            true,
        )
    }

    fn alice() -> User {
        utilisateur("00000000-0000-0000-0000-000000000001", "alice")
    }

    #[test]
    fn session_opened_then_resolved() {
        let s = SessionTokenService::new(3600, 86400);
        let jeton = s.generate_access_token(&alice()).unwrap();
        assert!(jeton.starts_with(PREFIXE_JETON));
        // 32 octets aléatoires = 43 caractères base64 URL.
        assert_eq!(jeton.len(), PREFIXE_JETON.len() + 43);
        let c = s.validate_token(&jeton).unwrap();
        assert_eq!(c.sub, alice().id().to_string());
        assert_eq!(c.username, "alice");
    }

    #[test]
    fn tokens_are_unique_and_carry_nothing() {
        let s = SessionTokenService::new(3600, 86400);
        let a = s.generate_access_token(&alice()).unwrap();
        let b = s.generate_access_token(&alice()).unwrap();
        assert_ne!(a, b);
        assert!(!a.contains("alice"));
        assert_ne!(s.generate_refresh_token(), s.generate_refresh_token());
    }

    #[test]
    fn forged_or_foreign_tokens_are_refused() {
        let s = SessionTokenService::new(3600, 86400);
        let jeton = s.generate_access_token(&alice()).unwrap();
        // Identifiant forgé (bonne forme, jamais émis).
        let forge = jeton_aleatoire(PREFIXE_JETON);
        assert!(s.validate_token(&forge).is_err());
        // Un caractère modifié.
        let mut altere = jeton.clone();
        let dernier = altere.pop().unwrap();
        altere.push(if dernier == 'A' { 'B' } else { 'A' });
        assert!(s.validate_token(&altere).is_err());
        // Jeton d'un autre magasin (autre processus).
        let autre = SessionTokenService::new(3600, 86400);
        assert!(autre.validate_token(&jeton).is_err());
        // Ancien JWT HS256 : refusé.
        assert!(s.validate_token("eyJhbGciOiJIUzI1NiJ9.e30.x").is_err());
    }

    #[test]
    fn logout_revokes_the_session() {
        let s = SessionTokenService::new(3600, 86400);
        let jeton = s.generate_access_token(&alice()).unwrap();
        assert!(s.revoke_token(&jeton));
        assert!(s.validate_token(&jeton).is_err());
        assert!(!s.revoke_token(&jeton));
    }

    #[test]
    fn revoke_user_closes_all_their_sessions_only() {
        let s = SessionTokenService::new(3600, 86400);
        let bob = utilisateur("00000000-0000-0000-0000-000000000002", "bob");
        let a1 = s.generate_access_token(&alice()).unwrap();
        let a2 = s.generate_access_token(&alice()).unwrap();
        let b1 = s.generate_access_token(&bob).unwrap();
        assert_eq!(s.revoke_user(alice().id()), 2);
        assert!(s.validate_token(&a1).is_err());
        assert!(s.validate_token(&a2).is_err());
        assert!(s.validate_token(&b1).is_ok());
    }

    #[test]
    fn expired_session_is_refused_and_purged() {
        let s = SessionTokenService::new(3600, 86400);
        let jeton = s.generate_access_token(&alice()).unwrap();
        // Expiration forcée.
        s.sessions.get_mut(&cle(&jeton)).unwrap().exp = Utc::now().timestamp() - 1;
        assert!(s.validate_token(&jeton).is_err());
        assert!(s.is_empty());
    }

    #[test]
    fn full_store_refuses_new_sessions() {
        let s = SessionTokenService::with_capacity(3600, 86400, 2);
        s.generate_access_token(&alice()).unwrap();
        s.generate_access_token(&alice()).unwrap();
        assert!(s.generate_access_token(&alice()).is_err());
        assert_eq!(s.revoke_user(alice().id()), 2);
        assert!(s.generate_access_token(&alice()).is_ok());
    }
}
