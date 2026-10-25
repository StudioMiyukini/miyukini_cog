//! Service d'authentification MiyukiniAdmin (Authentication Contract).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §11

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use crate::models::{AdminAccount, AdminRole, AdminSession};

/// Échecs de connexion consécutifs avant verrouillage du compte.
pub const MAX_FAILED_ATTEMPTS: u32 = 5;
/// Durée de verrouillage (minutes) une fois le seuil atteint.
pub const LOCKOUT_MINUTES: i64 = 15;

/// Forme persistée d'un compte. `AdminAccount` masque le hash (`skip_serializing`) : sans ce
/// miroir, le registre écrit sur disque perdrait `password_hash` et `failed_attempts` ne
/// serait pas durable. `password_hash` a un défaut vide pour relire les anciens registres
/// (un hash vide ne vérifie jamais : connexion refusée).
#[derive(Serialize, Deserialize)]
struct StoredAccount {
    account_id: String,
    username: String,
    #[serde(default)]
    password_hash: String,
    role: AdminRole,
    #[serde(default)]
    mfa_enabled: bool,
    #[serde(default)]
    mfa_secret_encrypted: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(default)]
    last_login_at: Option<DateTime<Utc>>,
    #[serde(default)]
    locked_until: Option<DateTime<Utc>>,
    #[serde(default)]
    failed_attempts: u32,
}

impl From<StoredAccount> for AdminAccount {
    fn from(s: StoredAccount) -> Self {
        Self {
            account_id: s.account_id,
            username: s.username,
            password_hash: s.password_hash,
            role: s.role,
            mfa_enabled: s.mfa_enabled,
            mfa_secret_encrypted: s.mfa_secret_encrypted,
            created_at: s.created_at,
            updated_at: s.updated_at,
            last_login_at: s.last_login_at,
            locked_until: s.locked_until,
            failed_attempts: s.failed_attempts,
        }
    }
}

impl From<&AdminAccount> for StoredAccount {
    fn from(a: &AdminAccount) -> Self {
        Self {
            account_id: a.account_id.clone(),
            username: a.username.clone(),
            password_hash: a.password_hash.clone(),
            role: a.role,
            mfa_enabled: a.mfa_enabled,
            mfa_secret_encrypted: a.mfa_secret_encrypted.clone(),
            created_at: a.created_at,
            updated_at: a.updated_at,
            last_login_at: a.last_login_at,
            locked_until: a.locked_until,
            failed_attempts: a.failed_attempts,
        }
    }
}

/// @id miyukiniadmin_auth_service
/// @role infrastructure
/// @layer operator
/// @do authenticate_admin
//@human / @human Authentification : login, session, rate limit (stub).
/// @depends miyukiniadmin_models_admin_account, miyukiniadmin_models_session
#[derive(Clone)]
pub struct AuthService {
    /// Répertoire de données (admin_registry.json).
    data_dir: PathBuf,
    /// Sessions actives (session_id → AdminSession).
    sessions: Arc<RwLock<HashMap<String, AdminSession>>>,
    /// Paramètres : idle timeout (secs), absolute timeout (secs).
    session_idle_timeout_secs: u64,
    session_absolute_timeout_secs: u64,
    /// Sérialise les lectures-modifications-écritures du registre (compteur d'échecs).
    registry_lock: Arc<Mutex<()>>,
}

impl AuthService {
    /// Crée le service avec un répertoire de données.
    /// @id miyukiniadmin_auth_service_new
    /// @role constructor
    /// @layer operator
    /// @do create_auth_service
    //@human / @human Construit le service d'authentification.
    #[must_use]
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_idle_timeout_secs: 900,
            session_absolute_timeout_secs: 28800,
            registry_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Chemin du registre admin.
    fn admin_registry_path(&self) -> PathBuf {
        self.data_dir.join("admin_registry.json")
    }

    /// Hash du mot de passe (Argon2id).
    /// @id miyukiniadmin_auth_hash_password
    /// @do hash_password
    fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
    }

    /// Vérifie le mot de passe contre un hash.
    /// @id miyukiniadmin_auth_verify_password
    /// @do verify_password
    fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
        let parsed = PasswordHash::new(hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed)
    }

    /// Charge le registre admin depuis le fichier (ou vide).
    /// @id miyukiniadmin_auth_load_registry
    /// @do load_registry
    pub async fn load_registry(&self) -> Result<Vec<AdminAccount>, std::io::Error> {
        let path = self.admin_registry_path();
        if !fs::metadata(&path)
            .await
            .is_ok_and(|m| m.is_file())
        {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path).await?;
        // Fail-closed : un registre illisible est une erreur (jamais « vide », ce qui
        // permettrait de l'écraser ou de rejouer le setup).
        let stored: Vec<StoredAccount> = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(stored.into_iter().map(AdminAccount::from).collect())
    }

    /// Enregistre le registre admin.
    /// @id miyukiniadmin_auth_save_registry
    /// @do save_registry
    pub async fn save_registry(&self, accounts: &[AdminAccount]) -> Result<(), std::io::Error> {
        let path = self.admin_registry_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let stored: Vec<StoredAccount> = accounts.iter().map(StoredAccount::from).collect();
        let content = serde_json::to_string_pretty(&stored)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        fs::write(&path, content).await
    }

    /// Login : vérifie username/password, crée une session (Authentication Contract §11.2).
    /// Réponse générique en cas d'échec : « Identifiants invalides ».
    /// @id miyukiniadmin_auth_login
    /// @role mutator
    /// @layer operator
    /// @do login_admin
    //@human / @human Authentifie un admin et crée une session.
    pub async fn login(
        &self,
        username: &str,
        password: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<AdminSession, LoginError> {
        let _guard = self.registry_lock.lock().await;
        let mut accounts = self
            .load_registry()
            .await
            .map_err(|_| LoginError::InvalidCredentials)?;
        let idx = accounts
            .iter()
            .position(|a| a.username.eq_ignore_ascii_case(username))
            .ok_or(LoginError::InvalidCredentials)?;

        if let Some(locked) = accounts[idx].locked_until {
            if Utc::now() < locked {
                return Err(LoginError::AccountLocked);
            }
        }

        if Self::verify_password(password, &accounts[idx].password_hash).is_err() {
            // Compteur d'échecs persistant ; verrouillage au seuil (audit H6).
            let acc = &mut accounts[idx];
            acc.failed_attempts = acc.failed_attempts.saturating_add(1);
            acc.updated_at = Utc::now();
            if acc.failed_attempts >= MAX_FAILED_ATTEMPTS {
                acc.locked_until = Some(Utc::now() + Duration::minutes(LOCKOUT_MINUTES));
                acc.failed_attempts = 0;
            }
            // Si l'écriture échoue, on refuse quand même la connexion.
            let _ = self.save_registry(&accounts).await;
            return Err(LoginError::InvalidCredentials);
        }

        // Succès : remise à zéro du compteur et du verrou expiré.
        {
            let acc = &mut accounts[idx];
            acc.failed_attempts = 0;
            acc.locked_until = None;
            acc.last_login_at = Some(Utc::now());
        }
        let _ = self.save_registry(&accounts).await;
        let account = &accounts[idx];

        let now = Utc::now();
        // Durées configurées hors bornes : session expirée d'emblée (échec fermé, jamais de panique).
        let duree = |secs: u64| i64::try_from(secs).ok().and_then(Duration::try_seconds);
        let expires_at = duree(self.session_idle_timeout_secs)
            .zip(duree(self.session_absolute_timeout_secs))
            .and_then(|(idle, absolue)| now.checked_add_signed(idle.min(absolue)))
            .unwrap_or(now);
        let session_id = Uuid::new_v4().to_string();
        let session = AdminSession {
            session_id: session_id.clone(),
            account_id: account.account_id.clone(),
            role: account.role,
            expires_at,
            created_at: now,
            ip: ip.map(String::from),
            user_agent: user_agent.map(String::from),
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session.clone());

        Ok(session)
    }

    /// Valide une session par token (session_id) ; retourne la session si valide.
    /// @id miyukiniadmin_auth_validate_session
    /// @role accessor
    /// @layer operator
    /// @do validate_session
    //@human / @human Vérifie qu'une session est valide et non expirée.
    pub async fn validate_session(&self, session_id: &str) -> Option<AdminSession> {
        let sessions = self.sessions.write().await;
        let session = sessions.get(session_id).cloned();
        drop(sessions);
        let session = session?;
        if session.is_valid(Utc::now()) {
            Some(session)
        } else {
            let mut sessions = self.sessions.write().await;
            sessions.remove(session_id);
            None
        }
    }

    /// Déconnexion : supprime la session.
    /// @id miyukiniadmin_auth_logout
    /// @do logout
    pub async fn logout(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    /// Crée un compte admin (pour parcours setup ou par un admin avec capacité).
    /// Hash Argon2id, rôle par défaut Admin.
    /// @id miyukiniadmin_auth_create_account
    /// @do create_account
    pub async fn create_account(
        &self,
        username: &str,
        password: &str,
        role: AdminRole,
    ) -> Result<AdminAccount, CreateAccountError> {
        if username.is_empty() || password.len() < 12 {
            return Err(CreateAccountError::PolicyViolation);
        }
        let _guard = self.registry_lock.lock().await;
        let mut accounts = self
            .load_registry()
            .await
            .map_err(|_| CreateAccountError::Io)?;
        if accounts
            .iter()
            .any(|a| a.username.eq_ignore_ascii_case(username))
        {
            return Err(CreateAccountError::UsernameExists);
        }
        let password_hash =
            Self::hash_password(password).map_err(|_| CreateAccountError::HashError)?;
        let now = Utc::now();
        let account = AdminAccount {
            account_id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            password_hash,
            role,
            mfa_enabled: false,
            mfa_secret_encrypted: None,
            created_at: now,
            updated_at: now,
            last_login_at: None,
            locked_until: None,
            failed_attempts: 0,
        };
        accounts.push(account.clone());
        self.save_registry(&accounts)
            .await
            .map_err(|_| CreateAccountError::Io)?;
        Ok(account)
    }
}

/// Erreur de login (réponse générique pour ne pas fuiter d'information).
/// @id miyukiniadmin_auth_login_error
/// @do definir_login_error
#[derive(Debug)]
pub enum LoginError {
    /// Identifiants invalides ou compte inexistant.
    InvalidCredentials,
    /// Compte verrouillé (rate limit).
    AccountLocked,
}

/// Erreur de création de compte.
/// @id miyukiniadmin_auth_create_account_error
/// @do definir_create_account_error
#[derive(Debug)]
pub enum CreateAccountError {
    /// Politique violée (ex. mot de passe trop court).
    PolicyViolation,
    /// Nom d'utilisateur déjà existant.
    UsernameExists,
    /// Erreur de hash (Argon2).
    HashError,
    /// Erreur I/O (fichier registre).
    Io,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        let d = std::env::temp_dir().join(format!("miyadmin-auth-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    async fn svc_with_admin() -> AuthService {
        let svc = AuthService::new(tmp());
        svc.create_account("root", "un-mot-de-passe-long", AdminRole::Admin)
            .await
            .unwrap();
        svc
    }

    #[tokio::test]
    async fn good_login_works_after_reload_from_disk() {
        let svc = svc_with_admin().await;
        // Le hash doit survivre à la persistance (régression : skip_serializing).
        assert!(svc.login("root", "un-mot-de-passe-long", None, None).await.is_ok());
    }

    #[tokio::test]
    async fn account_locks_after_max_failures_even_with_good_password() {
        let svc = svc_with_admin().await;
        for _ in 0..MAX_FAILED_ATTEMPTS {
            assert!(matches!(
                svc.login("root", "mauvais", None, None).await,
                Err(LoginError::InvalidCredentials)
            ));
        }
        assert!(matches!(
            svc.login("root", "un-mot-de-passe-long", None, None).await,
            Err(LoginError::AccountLocked)
        ));
        let acc = &svc.load_registry().await.unwrap()[0];
        assert!(acc.locked_until.is_some());
    }

    #[tokio::test]
    async fn expired_lock_allows_login_and_resets_counter() {
        let svc = svc_with_admin().await;
        let mut accs = svc.load_registry().await.unwrap();
        accs[0].locked_until = Some(Utc::now() - Duration::minutes(1));
        accs[0].failed_attempts = 3;
        svc.save_registry(&accs).await.unwrap();
        assert!(svc.login("root", "un-mot-de-passe-long", None, None).await.is_ok());
        let acc = &svc.load_registry().await.unwrap()[0];
        assert_eq!(acc.failed_attempts, 0);
        assert!(acc.locked_until.is_none());
    }

    #[tokio::test]
    async fn failures_below_threshold_do_not_lock_and_success_resets() {
        let svc = svc_with_admin().await;
        for _ in 0..(MAX_FAILED_ATTEMPTS - 1) {
            let _ = svc.login("root", "x", None, None).await;
        }
        assert!(svc.login("root", "un-mot-de-passe-long", None, None).await.is_ok());
        assert_eq!(svc.load_registry().await.unwrap()[0].failed_attempts, 0);
    }

    #[tokio::test]
    async fn corrupt_registry_is_an_error_not_empty() {
        let d = tmp();
        std::fs::write(d.join("admin_registry.json"), "{pas du json").unwrap();
        let svc = AuthService::new(d);
        assert!(svc.load_registry().await.is_err());
        assert!(svc.create_account("a", "un-mot-de-passe-long", AdminRole::Admin).await.is_err());
    }
}
