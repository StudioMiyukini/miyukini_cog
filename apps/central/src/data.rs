//! Couche données : connexion auth Central.
//!
//! Les services sont désormais des binaires indépendants qui gèrent leurs propres bases de données.
//! Central ne conserve que la base auth pour la gestion des profils et sessions.

use std::path::Path;
use std::sync::{Arc, Mutex};

use dioxus::prelude::*;
use miyukini_central::auth::{CentralAuthDb, CentralProfile};
use miyukini_connect::{
    AuthVerifyRequest, ConnectError, ConnectService, IdentitySetup, PermissionTier, RuntimeState,
};

/// Connexions DB de Central (standalone — uniquement auth).
pub struct ServiceConnections {
    /// Base auth Central (profils, COG vierge, session).
    pub auth_db: Arc<CentralAuthDb>,
    /// Service local Miyukini Connect (etat runtime + policies d'auth).
    pub connect: Arc<Mutex<ConnectService>>,
}

impl ServiceConnections {
    /// Ouvre la base auth. `base_path` = répertoire des données.
    pub fn open(base_path: &Path) -> Result<Self, String> {
        let auth_db = CentralAuthDb::open(base_path.join("central.db"))
            .map_err(|e| format!("Central auth DB: {e}"))?;
        let connect = ConnectService::new("2026.03.05");

        Ok(Self {
            auth_db: Arc::new(auth_db),
            connect: Arc::new(Mutex::new(connect)),
        })
    }

    /// Connexion via Miyukini Connect.
    /// Si le profil n'a pas encore de credential Connect, migration one-shot depuis le Legacy.
    pub fn connect_sign_in(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<CentralProfile>, String> {
        let email = email.trim();
        if email.is_empty() {
            return Ok(None);
        }

        let profile = match self.find_profile_by_email(email)? {
            Some(profile) => profile,
            None => return Ok(None),
        };

        if let Some(stored_hash) = self
            .auth_db
            .get_connect_password_hash(&profile.id)
            .map_err(|e| e.to_string())?
        {
            let mut connect = self
                .connect
                .lock()
                .map_err(|e| format!("Miyukini Connect lock: {e}"))?;
            let mut identity = IdentitySetup::new(profile.id.clone());
            identity
                .set_password_hash_for_import(stored_hash)
                .map_err(|e| format!("Miyukini Connect import: {e}"))?;
            connect
                .register_identity(identity)
                .map_err(|e| format!("Miyukini Connect register: {e}"))?;

            let runtime_state = connect.current_runtime_state();
            let request = AuthVerifyRequest {
                subject_id: profile.id.clone(),
                password: password.to_string(),
                totp_code: None,
                requested_tier: PermissionTier::Basic,
                runtime_state,
            };
            return match connect.auth_verify(request) {
                Ok(_) => Ok(Some(profile)),
                Err(ConnectError::InvalidCredentials) => Ok(None),
                Err(e) => Err(format!("Miyukini Connect auth: {e}")),
            };
        }

        // Migration Legacy -> Connect : valider une fois avec l'ancien hash, puis persister Argon2id.
        let legacy_profile = match self
            .auth_db
            .sign_in(email, password)
            .map_err(|e| e.to_string())?
        {
            Some(profile) => profile,
            None => return Ok(None),
        };

        let connect_hash = {
            let mut connect = self
                .connect
                .lock()
                .map_err(|e| format!("Miyukini Connect lock: {e}"))?;
            let mut identity = IdentitySetup::new(legacy_profile.id.clone());
            identity
                .set_password(password)
                .map_err(|e| format!("Miyukini Connect setup: {e}"))?;
            connect
                .register_identity(identity)
                .map_err(|e| format!("Miyukini Connect register: {e}"))?;

            let runtime_state = connect.current_runtime_state();
            let request = AuthVerifyRequest {
                subject_id: legacy_profile.id.clone(),
                password: password.to_string(),
                totp_code: None,
                requested_tier: PermissionTier::Basic,
                runtime_state,
            };
            match connect.auth_verify(request) {
                Ok(_) => {}
                Err(ConnectError::InvalidCredentials) => return Ok(None),
                Err(e) => return Err(format!("Miyukini Connect auth: {e}")),
            }

            connect
                .password_hash_for(&legacy_profile.id)
                .ok_or_else(|| "Miyukini Connect hash manquant apres migration".to_string())?
                .to_string()
        };

        self.auth_db
            .set_connect_password_hash(&legacy_profile.id, &connect_hash)
            .map_err(|e| format!("Persist Connect credential: {e}"))?;

        Ok(Some(legacy_profile))
    }

    /// Inscription d'un profil Central avec credential Miyukini Connect.
    pub fn connect_sign_up(
        &self,
        email: &str,
        password: &str,
        pseudonyme: Option<&str>,
    ) -> Result<CentralProfile, String> {
        let profile = self
            .auth_db
            .sign_up(email, password, pseudonyme)
            .map_err(|e| e.to_string())?;

        let connect_hash = {
            let mut connect = self
                .connect
                .lock()
                .map_err(|e| format!("Miyukini Connect lock: {e}"))?;
            let mut identity = IdentitySetup::new(profile.id.clone());
            identity
                .set_password(password)
                .map_err(|e| format!("Miyukini Connect setup: {e}"))?;
            connect
                .register_identity(identity)
                .map_err(|e| format!("Miyukini Connect register: {e}"))?;

            let runtime_state = connect.current_runtime_state();
            let request = AuthVerifyRequest {
                subject_id: profile.id.clone(),
                password: password.to_string(),
                totp_code: None,
                requested_tier: PermissionTier::Basic,
                runtime_state,
            };
            connect
                .auth_verify(request)
                .map_err(|e| format!("Miyukini Connect auth: {e}"))?;

            connect
                .password_hash_for(&profile.id)
                .ok_or_else(|| "Miyukini Connect hash manquant apres inscription".to_string())?
                .to_string()
        };

        self.auth_db
            .set_connect_password_hash(&profile.id, &connect_hash)
            .map_err(|e| format!("Persist Connect credential: {e}"))?;

        Ok(profile)
    }

    fn find_profile_by_email(&self, email: &str) -> Result<Option<CentralProfile>, String> {
        let profile_row = self
            .auth_db
            .list_profiles()
            .map_err(|e| e.to_string())?
            .into_iter()
            .find(|profile| profile.email.eq_ignore_ascii_case(email));

        let Some(profile_row) = profile_row else {
            return Ok(None);
        };

        self.auth_db
            .get_profile(&profile_row.id)
            .map_err(|e| e.to_string())
    }
}

/// Etat runtime courant de Miyukini Connect.
pub fn connect_runtime_state(connections: &ServiceConnections) -> RuntimeState {
    match connections.connect.lock() {
        Ok(connect) => connect.bootstrap().runtime_state,
        Err(_) => RuntimeState::Suspicious,
    }
}

/// Pseudo ou email affiché pour le profil connecté.
pub fn profile_display_name(profile: &CentralProfile) -> String {
    profile
        .pseudonyme
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(profile.email.as_str())
        .to_string()
}

/// Raccourci Dioxus pour accéder aux connexions DB depuis n'importe quel composant.
pub fn use_service_connections() -> Signal<Arc<ServiceConnections>> {
    use_context::<crate::state::AppContext>().connections
}
