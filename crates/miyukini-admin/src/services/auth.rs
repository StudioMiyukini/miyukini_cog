//! Service d'authentification MiyukiniAdmin (Authentication Contract).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §11

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{AdminAccount, AdminRole, AdminSession};

/// @id: miyukiniadmin_auth_service
/// @role: infrastructure
/// @layer: operator
/// @human: Authentification : login, session, rate limit (stub).
/// @do: authenticate_admin
/// @depends: miyukiniadmin_models_admin_account, miyukiniadmin_models_session
#[derive(Clone)]
pub struct AuthService {
    /// Répertoire de données (admin_registry.json).
    data_dir: PathBuf,
    /// Sessions actives (session_id → AdminSession).
    sessions: Arc<RwLock<HashMap<String, AdminSession>>>,
    /// Paramètres : idle timeout (secs), absolute timeout (secs).
    session_idle_timeout_secs: u64,
    session_absolute_timeout_secs: u64,
}

impl AuthService {
    /// Crée le service avec un répertoire de données.
    /// @id: miyukiniadmin_auth_service_new
    /// @role: constructor
    /// @layer: operator
    /// @human: Construit le service d'authentification.
    /// @do: create_auth_service
    #[must_use] 
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            session_idle_timeout_secs: 900,
            session_absolute_timeout_secs: 28800,
        }
    }

    /// Chemin du registre admin.
    fn admin_registry_path(&self) -> PathBuf {
        self.data_dir.join("admin_registry.json")
    }

    /// Hash du mot de passe (Argon2id).
    /// @id: miyukiniadmin_auth_hash_password
    fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
    }

    /// Vérifie le mot de passe contre un hash.
    /// @id: miyukiniadmin_auth_verify_password
    fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
        let parsed = PasswordHash::new(hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed)
    }

    /// Charge le registre admin depuis le fichier (ou vide).
    /// @id: miyukiniadmin_auth_load_registry
    pub async fn load_registry(&self) -> Result<Vec<AdminAccount>, std::io::Error> {
        let path = self.admin_registry_path();
        if !fs::metadata(&path).await.map(|m| m.is_file()).unwrap_or(false) {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(&path).await?;
        let accounts: Vec<AdminAccount> = serde_json::from_str(&content).unwrap_or_default();
        Ok(accounts)
    }

    /// Enregistre le registre admin.
    /// @id: miyukiniadmin_auth_save_registry
    pub async fn save_registry(&self, accounts: &[AdminAccount]) -> Result<(), std::io::Error> {
        let path = self.admin_registry_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(accounts).unwrap_or_default();
        fs::write(&path, content).await
    }

    /// Login : vérifie username/password, crée une session (Authentication Contract §11.2).
    /// Réponse générique en cas d'échec : « Identifiants invalides ».
    /// @id: miyukiniadmin_auth_login
    /// @role: mutator
    /// @layer: operator
    /// @human: Authentifie un admin et crée une session.
    /// @do: login_admin
    pub async fn login(
        &self,
        username: &str,
        password: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<AdminSession, LoginError> {
        let accounts = self.load_registry().await.map_err(|_| LoginError::InvalidCredentials)?;
        let account = accounts
            .iter()
            .find(|a| a.username.eq_ignore_ascii_case(username))
            .ok_or(LoginError::InvalidCredentials)?;

        if let Some(locked) = account.locked_until {
            if Utc::now() < locked {
                return Err(LoginError::AccountLocked);
            }
        }

        Self::verify_password(password, &account.password_hash).map_err(|_| LoginError::InvalidCredentials)?;

        let now = Utc::now();
        let expires_at = now
            + Duration::seconds(self.session_idle_timeout_secs as i64)
                .min(Duration::seconds(self.session_absolute_timeout_secs as i64));
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
    /// @id: miyukiniadmin_auth_validate_session
    /// @role: accessor
    /// @layer: operator
    /// @human: Vérifie qu'une session est valide et non expirée.
    /// @do: validate_session
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
    /// @id: miyukiniadmin_auth_logout
    pub async fn logout(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    /// Crée un compte admin (pour parcours setup ou par un admin avec capacité).
    /// Hash Argon2id, rôle par défaut Admin.
    /// @id: miyukiniadmin_auth_create_account
    pub async fn create_account(
        &self,
        username: &str,
        password: &str,
        role: AdminRole,
    ) -> Result<AdminAccount, CreateAccountError> {
        if username.is_empty() || password.len() < 12 {
            return Err(CreateAccountError::PolicyViolation);
        }
        let mut accounts = self.load_registry().await.map_err(|_| CreateAccountError::Io)?;
        if accounts.iter().any(|a| a.username.eq_ignore_ascii_case(username)) {
            return Err(CreateAccountError::UsernameExists);
        }
        let password_hash = Self::hash_password(password).map_err(|_| CreateAccountError::HashError)?;
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
        self.save_registry(&accounts).await.map_err(|_| CreateAccountError::Io)?;
        Ok(account)
    }
}

/// Erreur de login (réponse générique pour ne pas fuiter d'information).
/// @id: miyukiniadmin_auth_login_error
#[derive(Debug)]
pub enum LoginError {
    /// Identifiants invalides ou compte inexistant.
    InvalidCredentials,
    /// Compte verrouillé (rate limit).
    AccountLocked,
}

/// Erreur de création de compte.
/// @id: miyukiniadmin_auth_create_account_error
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
