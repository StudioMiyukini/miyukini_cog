//! Policy engine for AAL and factor availability.
//!
//! @id: miyukini_connect_policy
//! @do: enforce_aal_and_factor_policy
//! @role: security
//! @layer: domain

use crate::types::{AuthMethod, PermissionTier, RuntimeState};

#[derive(Debug, Clone)]
pub struct PolicyEngine {
    version: String,
}

impl PolicyEngine {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn max_aal_for_method(method: AuthMethod) -> u8 {
        match method {
            AuthMethod::Password => 1,
            AuthMethod::Totp | AuthMethod::EmailOtp => 2,
            AuthMethod::Passkey | AuthMethod::QrSigned => 3,
            AuthMethod::HardwareKey => 4,
        }
    }

    pub fn required_aal_for_tier(
        tier: PermissionTier,
        runtime: RuntimeState,
        suspicious_mode: bool,
    ) -> u8 {
        let base = match tier {
            PermissionTier::Basic => 1,
            PermissionTier::StandardWrite => 2,
            PermissionTier::SensitiveRead | PermissionTier::SensitiveWrite => 3,
            PermissionTier::CriticalAdmin => 4,
        };

        if suspicious_mode || matches!(runtime, RuntimeState::Suspicious) {
            (base + 1).min(4)
        } else {
            base
        }
    }

    pub fn factor_allowed_in_state(method: AuthMethod, runtime: RuntimeState) -> bool {
        match runtime {
            RuntimeState::OnlineFull => true,
            RuntimeState::OnlineDegraded => method != AuthMethod::QrSigned,
            RuntimeState::Isolated => {
                !matches!(method, AuthMethod::EmailOtp | AuthMethod::QrSigned)
            }
            RuntimeState::Suspicious => matches!(method, AuthMethod::Passkey | AuthMethod::HardwareKey),
        }
    }
}
