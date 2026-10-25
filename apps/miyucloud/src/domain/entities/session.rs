//! @id     miyucloud_domain_entities_session
//! @do     definir_session
//! @role   donnee
//! @layer  app
//! @human  miyucloud — module « domain entities session ».

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct Session {
    id: Uuid,
    user_id: Uuid,
    refresh_token: String,
    expires_at: DateTime<Utc>,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: DateTime<Utc>,
    revoked: bool,
}

impl Session {
    pub fn new(
        user_id: Uuid,
        refresh_token: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
        expires_in_days: i64,
    ) -> Result<Self, DomainError> {
        // Refus fail-closed : jamais de session sans jeton de rafraîchissement (C-10).
        if refresh_token.is_empty() {
            return Err(DomainError::internal_error(
                "Session",
                "Session refresh_token cannot be empty",
            ));
        }

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            refresh_token,
            expires_at: now + Duration::days(expires_in_days),
            ip_address,
            user_agent,
            created_at: now,
            revoked: false,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_raw(
        id: Uuid,
        user_id: Uuid,
        refresh_token: String,
        expires_at: DateTime<Utc>,
        ip_address: Option<String>,
        user_agent: Option<String>,
        created_at: DateTime<Utc>,
        revoked: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            refresh_token,
            expires_at,
            ip_address,
            user_agent,
            created_at,
            revoked,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }

    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn is_revoked(&self) -> bool {
        self.revoked
    }

    pub fn revoke(&mut self) {
        self.revoked = true;
    }
}
