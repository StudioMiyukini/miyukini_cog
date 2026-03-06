//! Append-only hash chain for authentication events.
//!
//! @id: miyukini_connect_audit
//! @do: track_auth_events_with_hash_chain
//! @role: security
//! @layer: infrastructure

use sha2::{Digest, Sha256};

use crate::errors::ConnectError;

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub index: u64,
    pub unix_ts: u64,
    pub event_type: String,
    pub subject_id: Option<String>,
    pub session_id: Option<String>,
    pub payload: String,
    pub prev_hash: String,
    pub event_hash: String,
}

#[derive(Debug, Clone, Default)]
pub struct AuditChain {
    events: Vec<AuditEvent>,
}

impl AuditChain {
    pub fn append(
        &mut self,
        event_type: &str,
        subject_id: Option<&str>,
        session_id: Option<&str>,
        payload: &str,
    ) -> Result<(), ConnectError> {
        let index = self.events.len() as u64;
        let unix_ts = crate::service::unix_ts_now()?;
        let prev_hash = self.events.last().map_or_else(
            || "GENESIS".to_string(),
            |event| event.event_hash.clone(),
        );

        let event_hash = compute_event_hash(
            index,
            unix_ts,
            event_type,
            subject_id,
            session_id,
            payload,
            &prev_hash,
        );
        self.events.push(AuditEvent {
            index,
            unix_ts,
            event_type: event_type.to_string(),
            subject_id: subject_id.map(ToString::to_string),
            session_id: session_id.map(ToString::to_string),
            payload: payload.to_string(),
            prev_hash,
            event_hash,
        });
        Ok(())
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn events_mut_for_test(&mut self) -> &mut [AuditEvent] {
        &mut self.events
    }

    pub fn verify_integrity(&self) -> bool {
        let mut last_hash = "GENESIS".to_string();
        for event in &self.events {
            if event.prev_hash != last_hash {
                return false;
            }
            let computed = compute_event_hash(
                event.index,
                event.unix_ts,
                &event.event_type,
                event.subject_id.as_deref(),
                event.session_id.as_deref(),
                &event.payload,
                &event.prev_hash,
            );
            if computed != event.event_hash {
                return false;
            }
            last_hash.clone_from(&event.event_hash);
        }
        true
    }
}

fn compute_event_hash(
    index: u64,
    unix_ts: u64,
    event_type: &str,
    subject_id: Option<&str>,
    session_id: Option<&str>,
    payload: &str,
    prev_hash: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string().as_bytes());
    hasher.update(unix_ts.to_string().as_bytes());
    hasher.update(event_type.as_bytes());
    hasher.update(subject_id.unwrap_or_default().as_bytes());
    hasher.update(session_id.unwrap_or_default().as_bytes());
    hasher.update(payload.as_bytes());
    hasher.update(prev_hash.as_bytes());
    let digest = hasher.finalize();
    format!("{digest:x}")
}
