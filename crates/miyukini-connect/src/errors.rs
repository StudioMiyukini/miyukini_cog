//! Error contracts for Miyukini Connect.
//!
//! @id: miyukini_connect_errors
//! @do: normalize_connect_errors
//! @role: api
//! @layer: contract

use crate::types::AuthMethod;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ConnectError {
    #[error("identity not found")]
    IdentityNotFound,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("missing totp code")]
    MissingTotpCode,
    #[error("session not found")]
    SessionNotFound,
    #[error("step-up required: current_aal={current_aal}, required_aal={required_aal}")]
    StepUpRequired { current_aal: u8, required_aal: u8 },
    #[error("subject is temporarily locked out; retry_after_seconds={retry_after_seconds}")]
    LockedOut { retry_after_seconds: u64 },
    #[error("factor {0:?} is not allowed in current runtime state")]
    FactorNotAllowed(AuthMethod),
    #[error("factor {0:?} is not enrolled for identity")]
    FactorNotEnrolled(AuthMethod),
    #[error("session expired")]
    SessionExpired,
    #[error("audit chain is empty")]
    EmptyAuditChain,
    #[error("internal error: {0}")]
    Internal(String),
}
