//! Shared contract types for Miyukini Connect APIs.
//!
//! @id: miyukini_connect_types
//! @do: define_connect_contracts
//! @role: api
//! @layer: contract

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeState {
    OnlineFull,
    OnlineDegraded,
    Isolated,
    Suspicious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionTier {
    Basic,
    StandardWrite,
    SensitiveRead,
    SensitiveWrite,
    CriticalAdmin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthMethod {
    Password,
    Totp,
    Passkey,
    QrSigned,
    HardwareKey,
    EmailOtp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthSession {
    pub session_id: Uuid,
    pub subject_id: String,
    pub aal: u8,
    pub permission_tier: PermissionTier,
    pub methods: Vec<AuthMethod>,
    pub auth_time_unix: u64,
    pub last_activity_unix: u64,
    pub idle_timeout_secs: u64,
    pub absolute_expires_unix: u64,
    pub step_up_until_unix: Option<u64>,
    pub runtime_state: RuntimeState,
    pub integrity_fingerprint: String,
    pub origin_capabilities_snapshot: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthVerifyRequest {
    pub subject_id: String,
    pub password: String,
    pub totp_code: Option<String>,
    pub requested_tier: PermissionTier,
    pub runtime_state: RuntimeState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OriginProbeStatus {
    Ok,
    Degraded,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OriginProbeResult {
    pub status: OriginProbeStatus,
    pub latency_ms: Option<u64>,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BootstrapResponse {
    pub runtime_state: RuntimeState,
    pub available_factors: Vec<AuthMethod>,
    pub policy_version: String,
    pub origin_probe: OriginProbeResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizeHint {
    pub current_aal: u8,
    pub required_aal: u8,
    pub step_up_required: bool,
    pub allowed: bool,
}
