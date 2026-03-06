//! Core service logic for Miyukini Connect.
//!
//! @id: miyukini_connect_service
//! @do: manage_auth_sessions_stepup_origin
//! @role: auth
//! @layer: domain

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use totp_rs::{Algorithm, TOTP};
use uuid::Uuid;

use crate::audit::AuditChain;
use crate::errors::ConnectError;
use crate::policy::PolicyEngine;
use crate::types::{
    AuthMethod, AuthSession, AuthVerifyRequest, AuthorizeHint, BootstrapResponse, OriginProbeResult,
    OriginProbeStatus, PermissionTier, RuntimeState,
};

#[derive(Debug, Clone)]
pub struct IdentitySetup {
    subject_id: String,
    password_hash: Option<String>,
    totp_secret: Option<Vec<u8>>,
    passkey_enrolled: bool,
    hardware_key_enrolled: bool,
}

impl IdentitySetup {
    pub fn new(subject_id: impl Into<String>) -> Self {
        Self {
            subject_id: subject_id.into(),
            password_hash: None,
            totp_secret: None,
            passkey_enrolled: false,
            hardware_key_enrolled: false,
        }
    }

    pub fn set_password(&mut self, plain: &str) -> Result<(), ConnectError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2
            .hash_password(plain.as_bytes(), &salt)
            .map_err(|error| ConnectError::Internal(error.to_string()))?
            .to_string();
        self.password_hash = Some(hash);
        Ok(())
    }

    pub fn set_password_hash_for_import(
        &mut self,
        password_hash: impl Into<String>,
    ) -> Result<(), ConnectError> {
        let hash = password_hash.into();
        if !hash.starts_with("$argon2id$") {
            return Err(ConnectError::Internal(
                "imported hash must be argon2id".to_string(),
            ));
        }
        self.password_hash = Some(hash);
        Ok(())
    }

    pub fn enable_totp(&mut self, secret: Vec<u8>) {
        self.totp_secret = Some(secret);
    }

    pub fn enable_passkey(&mut self) {
        self.passkey_enrolled = true;
    }

    pub fn enable_hardware_key(&mut self) {
        self.hardware_key_enrolled = true;
    }
}

#[derive(Debug, Clone)]
struct IdentityState {
    password_hash: String,
    totp_secret: Option<Vec<u8>>,
    enrolled: HashSet<AuthMethod>,
}

#[derive(Debug, Clone)]
pub struct ConnectService {
    policy: PolicyEngine,
    runtime_state: RuntimeState,
    suspicious_mode: bool,
    origin_probe: OriginProbeResult,
    origin_capabilities_cache: Vec<String>,
    identities: HashMap<String, IdentityState>,
    sessions: HashMap<Uuid, AuthSession>,
    failed_attempts: HashMap<String, u32>,
    lockouts: HashMap<String, u64>,
    audit_chain: AuditChain,
}

impl ConnectService {
    pub fn new(policy_version: impl Into<String>) -> Self {
        Self {
            policy: PolicyEngine::new(policy_version),
            runtime_state: RuntimeState::Isolated,
            suspicious_mode: false,
            origin_probe: OriginProbeResult {
                status: OriginProbeStatus::Unavailable,
                latency_ms: None,
                capabilities: Vec::new(),
            },
            origin_capabilities_cache: Vec::new(),
            identities: HashMap::new(),
            sessions: HashMap::new(),
            failed_attempts: HashMap::new(),
            lockouts: HashMap::new(),
            audit_chain: AuditChain::default(),
        }
    }

    pub fn register_identity(&mut self, identity: IdentitySetup) -> Result<(), ConnectError> {
        let password_hash = identity
            .password_hash
            .ok_or_else(|| ConnectError::Internal("password hash is required".to_string()))?;

        let mut enrolled = HashSet::from([AuthMethod::Password]);
        if identity.totp_secret.is_some() {
            enrolled.insert(AuthMethod::Totp);
        }
        if identity.passkey_enrolled {
            enrolled.insert(AuthMethod::Passkey);
        }
        if identity.hardware_key_enrolled {
            enrolled.insert(AuthMethod::HardwareKey);
        }

        self.identities.insert(
            identity.subject_id,
            IdentityState {
                password_hash,
                totp_secret: identity.totp_secret,
                enrolled,
            },
        );
        Ok(())
    }

    pub fn password_hash_for(&self, subject_id: &str) -> Option<&str> {
        self.identities
            .get(subject_id)
            .map(|identity| identity.password_hash.as_str())
    }

    pub fn set_runtime_state(&mut self, runtime_state: RuntimeState) {
        self.runtime_state = runtime_state;
    }

    pub fn current_runtime_state(&self) -> RuntimeState {
        self.runtime_state
    }

    pub fn bootstrap(&self) -> BootstrapResponse {
        BootstrapResponse {
            runtime_state: self.runtime_state,
            available_factors: self.available_factors(),
            policy_version: self.policy.version().to_string(),
            origin_probe: self.origin_probe.clone(),
        }
    }

    pub fn origin_ping(&mut self, probe: OriginProbeResult) {
        self.origin_capabilities_cache.clone_from(&probe.capabilities);
        self.runtime_state = match probe.status {
            OriginProbeStatus::Ok => RuntimeState::OnlineFull,
            OriginProbeStatus::Degraded => RuntimeState::OnlineDegraded,
            OriginProbeStatus::Unavailable => RuntimeState::Isolated,
        };
        self.origin_probe = probe;
    }

    pub fn auth_verify(&mut self, request: AuthVerifyRequest) -> Result<AuthSession, ConnectError> {
        self.runtime_state = request.runtime_state;
        self.enforce_not_locked(&request.subject_id)?;

        let identity = self
            .identities
            .get(&request.subject_id)
            .ok_or(ConnectError::IdentityNotFound)?;

        if verify_password(&identity.password_hash, &request.password).is_err() {
            self.record_failed_attempt(&request.subject_id)?;
            return Err(ConnectError::InvalidCredentials);
        }
        let mut methods = vec![AuthMethod::Password];

        if let Some(secret) = &identity.totp_secret {
            let code = request.totp_code.ok_or(ConnectError::MissingTotpCode)?;
            if !verify_totp(secret, &code)? {
                self.record_failed_attempt(&request.subject_id)?;
                return Err(ConnectError::InvalidCredentials);
            }
            methods.push(AuthMethod::Totp);
        }
        self.clear_failed_attempts(&request.subject_id);

        let current_aal = methods
            .iter()
            .map(|method| PolicyEngine::max_aal_for_method(*method))
            .max()
            .unwrap_or(1);
        let required_aal = PolicyEngine::required_aal_for_tier(
            request.requested_tier,
            self.runtime_state,
            self.suspicious_mode,
        );

        if current_aal < required_aal {
            return Err(ConnectError::StepUpRequired {
                current_aal,
                required_aal,
            });
        }

        let session = self.create_session(
            &request.subject_id,
            request.requested_tier,
            methods,
            None,
            self.runtime_state,
        )?;
        self.sessions.insert(session.session_id, session.clone());
        self.audit_chain.append(
            "auth_verify_success",
            Some(&request.subject_id),
            Some(&session.session_id.to_string()),
            "login success",
        )?;
        Ok(session)
    }

    pub fn auth_step_up(
        &mut self,
        session_id: &Uuid,
        method: AuthMethod,
    ) -> Result<AuthSession, ConnectError> {
        if !PolicyEngine::factor_allowed_in_state(method, self.runtime_state) {
            return Err(ConnectError::FactorNotAllowed(method));
        }

        let existing = self
            .sessions
            .remove(session_id)
            .ok_or(ConnectError::SessionNotFound)?;
        let identity = self
            .identities
            .get(&existing.subject_id)
            .ok_or(ConnectError::IdentityNotFound)?;
        if !identity.enrolled.contains(&method) {
            return Err(ConnectError::FactorNotEnrolled(method));
        }

        let mut methods = existing.methods.clone();
        if !methods.contains(&method) {
            methods.push(method);
        }
        let aal = methods
            .iter()
            .map(|current_method| PolicyEngine::max_aal_for_method(*current_method))
            .max()
            .unwrap_or(1);
        let step_up_until_unix = Some(unix_ts_now()? + 10 * 60);
        let rotated = self.create_session(
            &existing.subject_id,
            existing.permission_tier,
            methods,
            step_up_until_unix,
            self.runtime_state,
        )?;

        self.audit_chain.append(
            "auth_step_up_success",
            Some(&existing.subject_id),
            Some(&rotated.session_id.to_string()),
            "session id rotated",
        )?;
        self.sessions.insert(rotated.session_id, rotated.clone());
        Ok(rotated)
    }

    pub fn auth_issue_attested_session(
        &mut self,
        subject_id: &str,
        requested_tier: PermissionTier,
        method: AuthMethod,
    ) -> Result<AuthSession, ConnectError> {
        if !PolicyEngine::factor_allowed_in_state(method, self.runtime_state) {
            return Err(ConnectError::FactorNotAllowed(method));
        }
        if !self.identities.contains_key(subject_id) {
            return Err(ConnectError::IdentityNotFound);
        }

        let hint = self.session_authorize_hint_for_aal(
            PolicyEngine::max_aal_for_method(method),
            requested_tier,
        )?;
        if hint.step_up_required {
            return Err(ConnectError::StepUpRequired {
                current_aal: hint.current_aal,
                required_aal: hint.required_aal,
            });
        }

        let session = self.create_session(
            subject_id,
            requested_tier,
            vec![method],
            Some(unix_ts_now()? + 10 * 60),
            self.runtime_state,
        )?;
        self.audit_chain.append(
            "auth_attested_issue_success",
            Some(subject_id),
            Some(&session.session_id.to_string()),
            "attested factor login success",
        )?;
        self.sessions.insert(session.session_id, session.clone());
        Ok(session)
    }

    pub fn auth_logout(&mut self, session_id: &Uuid) {
        self.sessions.remove(session_id);
    }

    pub fn session_current(&self, session_id: &Uuid) -> Option<&AuthSession> {
        self.sessions
            .get(session_id)
            .filter(|session| !self.is_session_expired(session))
    }

    pub fn session_introspect(&self, session_id: &Uuid) -> Result<Option<AuthSession>, ConnectError> {
        Ok(self
            .sessions
            .get(session_id)
            .filter(|session| !self.is_session_expired(session))
            .cloned())
    }

    pub fn session_authorize_hint(
        &self,
        session_id: &Uuid,
        tier: PermissionTier,
    ) -> Result<AuthorizeHint, ConnectError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or(ConnectError::SessionNotFound)?;
        if self.is_session_expired(session) {
            return Err(ConnectError::SessionExpired);
        }
        self.session_authorize_hint_for_aal(session.aal, tier)
    }

    pub fn session_authorize_hint_for_aal(
        &self,
        current_aal: u8,
        tier: PermissionTier,
    ) -> Result<AuthorizeHint, ConnectError> {
        let required_aal =
            PolicyEngine::required_aal_for_tier(tier, self.runtime_state, self.suspicious_mode);
        Ok(AuthorizeHint {
            current_aal,
            required_aal,
            step_up_required: current_aal < required_aal,
            allowed: current_aal >= required_aal,
        })
    }

    pub fn enrollment_allowed(&self, method: AuthMethod) -> bool {
        PolicyEngine::factor_allowed_in_state(method, self.runtime_state)
    }

    pub fn report_anomaly(&mut self) {
        self.suspicious_mode = true;
        self.runtime_state = RuntimeState::Suspicious;
        self.sessions
            .retain(|_, session| session.permission_tier == PermissionTier::Basic);
    }

    pub fn append_audit(
        &mut self,
        event_type: &str,
        subject_id: Option<&str>,
        session_id: Option<&str>,
        payload: &str,
    ) -> Result<(), ConnectError> {
        self.audit_chain
            .append(event_type, subject_id, session_id, payload)
    }

    pub fn verify_audit_chain(&self) -> bool {
        self.audit_chain.verify_integrity()
    }

    pub fn force_audit_tamper_for_test(&mut self, bad_hash: String) {
        if let Some(last) = self.audit_chain.events_mut_for_test().last_mut() {
            last.event_hash = bad_hash;
        }
    }

    pub fn force_session_expire_for_test(&mut self, session_id: &Uuid) {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.absolute_expires_unix = 0;
        }
    }

    fn available_factors(&self) -> Vec<AuthMethod> {
        [
            AuthMethod::Password,
            AuthMethod::Totp,
            AuthMethod::Passkey,
            AuthMethod::QrSigned,
            AuthMethod::HardwareKey,
            AuthMethod::EmailOtp,
        ]
        .into_iter()
        .filter(|method| PolicyEngine::factor_allowed_in_state(*method, self.runtime_state))
        .collect()
    }

    fn create_session(
        &self,
        subject_id: &str,
        permission_tier: PermissionTier,
        methods: Vec<AuthMethod>,
        step_up_until_unix: Option<u64>,
        runtime_state: RuntimeState,
    ) -> Result<AuthSession, ConnectError> {
        let session_id = Uuid::new_v4();
        let auth_time_unix = unix_ts_now()?;
        let idle_timeout_secs = 15 * 60;
        let absolute_expires_unix = auth_time_unix + (8 * 60 * 60);
        let aal = methods
            .iter()
            .map(|method| PolicyEngine::max_aal_for_method(*method))
            .max()
            .unwrap_or(1);
        let integrity_fingerprint =
            compute_integrity_fingerprint(session_id, subject_id, aal, runtime_state, auth_time_unix);
        Ok(AuthSession {
            session_id,
            subject_id: subject_id.to_string(),
            aal,
            permission_tier,
            methods,
            auth_time_unix,
            last_activity_unix: auth_time_unix,
            idle_timeout_secs,
            absolute_expires_unix,
            step_up_until_unix,
            runtime_state,
            integrity_fingerprint,
            origin_capabilities_snapshot: self.origin_capabilities_cache.clone(),
        })
    }

    fn enforce_not_locked(&self, subject_id: &str) -> Result<(), ConnectError> {
        if let Some(until) = self.lockouts.get(subject_id) {
            let now = unix_ts_now()?;
            if *until > now {
                return Err(ConnectError::LockedOut {
                    retry_after_seconds: *until - now,
                });
            }
        }
        Ok(())
    }

    fn record_failed_attempt(&mut self, subject_id: &str) -> Result<(), ConnectError> {
        const MAX_ATTEMPTS: u32 = 5;
        const LOCKOUT_SECS: u64 = 300;
        let next = self
            .failed_attempts
            .get(subject_id)
            .copied()
            .unwrap_or(0)
            .saturating_add(1);
        self.failed_attempts.insert(subject_id.to_string(), next);
        if next >= MAX_ATTEMPTS {
            self.lockouts
                .insert(subject_id.to_string(), unix_ts_now()? + LOCKOUT_SECS);
            self.failed_attempts.remove(subject_id);
        }
        Ok(())
    }

    fn clear_failed_attempts(&mut self, subject_id: &str) {
        self.failed_attempts.remove(subject_id);
        self.lockouts.remove(subject_id);
    }

    fn is_session_expired(&self, session: &AuthSession) -> bool {
        match unix_ts_now() {
            Ok(now) => {
                if now > session.absolute_expires_unix {
                    return true;
                }
                now.saturating_sub(session.last_activity_unix) > session.idle_timeout_secs
            }
            Err(_) => true,
        }
    }
}

pub fn unix_ts_now() -> Result<u64, ConnectError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| ConnectError::Internal(error.to_string()))
}

fn verify_password(hash: &str, password: &str) -> Result<(), ConnectError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|error| ConnectError::Internal(error.to_string()))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ConnectError::InvalidCredentials)
}

fn verify_totp(secret: &[u8], code: &str) -> Result<bool, ConnectError> {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_vec(),
        Some("Miyukini".to_string()),
        "connect".to_string(),
    )
    .map_err(|error| ConnectError::Internal(error.to_string()))?;
    totp.check_current(code)
        .map_err(|error| ConnectError::Internal(error.to_string()))
}

fn compute_integrity_fingerprint(
    session_id: Uuid,
    subject_id: &str,
    aal: u8,
    runtime_state: RuntimeState,
    auth_time_unix: u64,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(session_id.as_bytes());
    hasher.update(subject_id.as_bytes());
    hasher.update(aal.to_string().as_bytes());
    hasher.update(format!("{runtime_state:?}").as_bytes());
    hasher.update(auth_time_unix.to_string().as_bytes());
    let digest = hasher.finalize();
    format!("sha256:{digest:x}")
}
