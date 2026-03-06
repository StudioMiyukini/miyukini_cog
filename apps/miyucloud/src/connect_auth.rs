//! Pont Miyukini Connect pour le portail web MiyuCloud.

use chrono::{Duration, Utc};
use miyukini_central::auth::{CentralAuthDb, CentralProfile};
use miyukini_connect::{
    AuthMethod, AuthSession, AuthVerifyRequest, ConnectError, ConnectService, IdentitySetup,
    OriginProbeResult, OriginProbeStatus, PermissionTier, RuntimeState,
};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PortalIdentity {
    pub subject_id: String,
    pub email: String,
    pub display_name: String,
    pub connect_session: AuthSession,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingQrChallengeSummary {
    pub id: String,
    pub browser_label: String,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone)]
pub struct PendingQrChallenge {
    pub id: String,
    pub browser_label: String,
    pub created_at: String,
    pub expires_at: String,
    pub expires_at_unix: u64,
    pub approved_identity: Option<PortalIdentity>,
    pub rejected: bool,
}

impl PendingQrChallenge {
    pub fn summary(&self) -> PendingQrChallengeSummary {
        PendingQrChallengeSummary {
            id: self.id.clone(),
            browser_label: self.browser_label.clone(),
            created_at: self.created_at.clone(),
            expires_at: self.expires_at.clone(),
        }
    }

    pub fn is_expired(&self) -> bool {
        now_unix() > self.expires_at_unix
    }
}

#[derive(Debug, Clone)]
pub struct WebPortalSession {
    pub token: String,
    pub subject_id: String,
    pub email: String,
    pub display_name: String,
    pub expires_at_unix: u64,
}

impl WebPortalSession {
    pub fn is_expired(&self) -> bool {
        now_unix() > self.expires_at_unix
    }
}

pub struct ConnectAuthManager {
    central_db_path: Option<PathBuf>,
    connect: Mutex<ConnectService>,
}

impl ConnectAuthManager {
    pub fn new(central_db_path: Option<PathBuf>) -> Self {
        let mut connect = ConnectService::new("2026.03.06");
        connect.origin_ping(OriginProbeResult {
            status: OriginProbeStatus::Ok,
            latency_ms: Some(0),
            capabilities: vec!["qr_login".to_string(), "password_login".to_string()],
        });
        Self {
            central_db_path,
            connect: Mutex::new(connect),
        }
    }

    pub fn authenticate_password(
        &self,
        email: &str,
        password: &str,
        owner_id: &str,
    ) -> Result<PortalIdentity, String> {
        let db = self.open_central_db()?;
        let profile = find_profile_by_email(&db, email)?
            .ok_or_else(|| "Compte Miyukini Connect introuvable.".to_string())?;
        if profile.id != owner_id {
            return Err("Ce compte n'est pas autorise sur ce cloud.".to_string());
        }

        let mut connect = self
            .connect
            .lock()
            .map_err(|e| format!("Lock Miyukini Connect: {e}"))?;
        connect.set_runtime_state(RuntimeState::OnlineFull);

        if let Some(stored_hash) = db
            .get_connect_password_hash(&profile.id)
            .map_err(|e| e.to_string())?
        {
            register_imported_identity(&mut connect, &profile.id, &stored_hash)?;
        } else {
            let Some(_) = db.sign_in(email, password).map_err(|e| e.to_string())? else {
                return Err("Identifiants invalides.".to_string());
            };
            let mut identity = IdentitySetup::new(profile.id.clone());
            identity
                .set_password(password)
                .map_err(|e| format!("Connect setup: {e}"))?;
            connect
                .register_identity(identity)
                .map_err(|e| format!("Connect register: {e}"))?;
            let connect_hash = connect
                .password_hash_for(&profile.id)
                .ok_or_else(|| "Hash Connect manquant après migration.".to_string())?
                .to_string();
            db.set_connect_password_hash(&profile.id, &connect_hash)
                .map_err(|e| e.to_string())?;
        }

        let session = connect
            .auth_verify(AuthVerifyRequest {
                subject_id: profile.id.clone(),
                password: password.to_string(),
                totp_code: None,
                requested_tier: PermissionTier::StandardWrite,
                runtime_state: RuntimeState::OnlineFull,
            })
            .map_err(map_connect_error)?;

        Ok(PortalIdentity {
            subject_id: profile.id.clone(),
            email: profile.email.clone(),
            display_name: profile_display_name(&profile),
            connect_session: session,
        })
    }

    pub fn approve_qr_for_owner(&self, owner_id: &str) -> Result<PortalIdentity, String> {
        let db = self.open_central_db()?;
        let profile = db
            .get_profile(owner_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Profil COG introuvable.".to_string())?;
        let stored_hash = db
            .get_connect_password_hash(owner_id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| {
                "Ce profil doit d'abord se reconnecter dans Central pour activer Miyukini Connect."
                    .to_string()
            })?;

        let mut connect = self
            .connect
            .lock()
            .map_err(|e| format!("Lock Miyukini Connect: {e}"))?;
        connect.set_runtime_state(RuntimeState::OnlineFull);
        register_imported_identity(&mut connect, owner_id, &stored_hash)?;

        let session = connect
            .auth_issue_attested_session(
                owner_id,
                PermissionTier::SensitiveWrite,
                AuthMethod::QrSigned,
            )
            .map_err(map_connect_error)?;

        Ok(PortalIdentity {
            subject_id: profile.id.clone(),
            email: profile.email.clone(),
            display_name: profile_display_name(&profile),
            connect_session: session,
        })
    }

    fn open_central_db(&self) -> Result<CentralAuthDb, String> {
        let path = self
            .central_db_path
            .as_ref()
            .ok_or_else(|| "MIYUCLOUD_CENTRAL_DB_PATH absent.".to_string())?;
        CentralAuthDb::open(path).map_err(|e| e.to_string())
    }
}

fn find_profile_by_email(db: &CentralAuthDb, email: &str) -> Result<Option<CentralProfile>, String> {
    let profile_row = db
        .list_profiles()
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|profile| profile.email.eq_ignore_ascii_case(email.trim()));

    let Some(profile_row) = profile_row else {
        return Ok(None);
    };

    db.get_profile(&profile_row.id).map_err(|e| e.to_string())
}

fn register_imported_identity(
    connect: &mut ConnectService,
    subject_id: &str,
    password_hash: &str,
) -> Result<(), String> {
    let mut identity = IdentitySetup::new(subject_id.to_string());
    identity
        .set_password_hash_for_import(password_hash.to_string())
        .map_err(|e| e.to_string())?;
    connect
        .register_identity(identity)
        .map_err(|e| e.to_string())
}

fn profile_display_name(profile: &CentralProfile) -> String {
    profile
        .pseudonyme
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or(profile.email.as_str())
        .to_string()
}

fn map_connect_error(error: ConnectError) -> String {
    match error {
        ConnectError::InvalidCredentials => "Identifiants Miyukini Connect invalides.".to_string(),
        ConnectError::LockedOut {
            retry_after_seconds,
        } => format!("Trop de tentatives. Réessaie dans {retry_after_seconds}s."),
        other => other.to_string(),
    }
}

fn now_unix() -> u64 {
    Utc::now().timestamp().max(0) as u64
}

pub fn new_pending_qr_challenge(browser_label: String) -> PendingQrChallenge {
    let created_at = Utc::now();
    let expires_at = created_at + Duration::minutes(5);
    PendingQrChallenge {
        id: uuid::Uuid::new_v4().to_string(),
        browser_label,
        created_at: created_at.to_rfc3339(),
        expires_at: expires_at.to_rfc3339(),
        expires_at_unix: expires_at.timestamp().max(0) as u64,
        approved_identity: None,
        rejected: false,
    }
}
