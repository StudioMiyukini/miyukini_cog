//! Audit event domain entity for persistent security logging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Categories of auditable actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    // Authentication
    Login,
    LoginFailed,
    Logout,
    PasswordChange,
    TokenRefresh,
    AccountLocked,

    // File operations
    FileUpload,
    FileDownload,
    FileDelete,
    FileMove,
    FileCopy,
    FileRename,

    // Folder operations
    FolderCreate,
    FolderDelete,
    FolderMove,

    // Sharing
    ShareCreate,
    ShareDelete,
    ShareAccess,

    // Trash
    TrashRestore,
    TrashPurge,

    // Admin
    AdminUserCreate,
    AdminUserDelete,
    AdminUserUpdate,
    AdminSettingsChange,

    // GDPR
    GdprExport,
    GdprErase,
    GdprRectify,
    ConsentGrant,
    ConsentRevoke,

    // System
    SystemStartup,
    SystemShutdown,
    MimeBlocked,
    RateLimitExceeded,
    BandwidthLimitExceeded,
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| format!("{:?}", self));
        write!(f, "{}", s)
    }
}

/// Type of resource involved in the audit event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    File,
    Folder,
    User,
    Share,
    Calendar,
    Contact,
    Session,
    System,
    Consent,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| format!("{:?}", self));
        write!(f, "{}", s)
    }
}

/// Outcome of the audited operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Success,
    Failure,
    Denied,
}

impl std::fmt::Display for AuditOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| format!("{:?}", self));
        write!(f, "{}", s)
    }
}

/// A single audit log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub action: AuditAction,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub outcome: AuditOutcome,
}

impl AuditEvent {
    /// Create a new audit event with the current timestamp.
    pub fn new(
        action: AuditAction,
        resource_type: ResourceType,
        outcome: AuditOutcome,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            user_id: None,
            username: None,
            action,
            resource_type,
            resource_id: None,
            ip_address: None,
            user_agent: None,
            details: None,
            outcome,
        }
    }

    pub fn with_user(mut self, user_id: Uuid, username: impl Into<String>) -> Self {
        self.user_id = Some(user_id);
        self.username = Some(username.into());
        self
    }

    pub fn with_resource(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Filter for querying audit events.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuditFilter {
    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub outcome: Option<String>,
}

/// Pagination parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct AuditPagination {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

impl Default for AuditPagination {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
        }
    }
}
