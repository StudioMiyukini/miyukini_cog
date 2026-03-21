//! GDPR individual rights port — RGPD Art. 15-20.
//!
//! Provides data export, erasure ("right to be forgotten"), and rectification.

use crate::common::errors::DomainError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Structured export of all user data (RGPD Art. 15 + Art. 20 portability).
#[derive(Debug, Serialize)]
pub struct UserDataExport {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub files_count: u64,
    pub folders_count: u64,
    pub calendars_count: u64,
    pub contacts_count: u64,
    pub shares_count: u64,
    pub audit_events_count: u64,
    pub export_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Report of an erasure operation (RGPD Art. 17).
#[derive(Debug, Serialize)]
pub struct ErasureReport {
    pub user_id: Uuid,
    pub files_deleted: u64,
    pub folders_deleted: u64,
    pub shares_revoked: u64,
    pub audit_anonymized: u64,
    pub sessions_revoked: u64,
    pub account_deactivated: bool,
    pub erasure_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Updates for user PII rectification (RGPD Art. 16).
#[derive(Debug, Deserialize)]
pub struct UserRectification {
    pub email: Option<String>,
    pub username: Option<String>,
}

/// Port for GDPR rights management.
pub trait GdprRightsPort: Send + Sync + 'static {
    /// Export all user data (Art. 15 access + Art. 20 portability).
    fn export_user_data(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<UserDataExport, DomainError>> + Send;

    /// Erase all user data and anonymize audit trail (Art. 17).
    fn erase_user_data(
        &self,
        user_id: Uuid,
        reason: &str,
    ) -> impl std::future::Future<Output = Result<ErasureReport, DomainError>> + Send;

    /// Rectify user PII (Art. 16).
    fn rectify_user_data(
        &self,
        user_id: Uuid,
        updates: UserRectification,
    ) -> impl std::future::Future<Output = Result<(), DomainError>> + Send;
}
