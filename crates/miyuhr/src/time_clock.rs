//! Tools MiyuHR — tool.hr.time_clock.in, tool.hr.time_clock.out.
//! Store en mémoire des pointages (persistance réelle = KindMother côté produit).

use crate::context::GovernedContext;
use crate::errors::MiyuhrError;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};
use std::sync::Mutex;

/// Un pointage (entrée ou sortie).
#[derive(Debug, Clone)]
pub struct ClockEvent {
    pub id: String,
    pub employee_id: String,
    pub kind: ClockEventKind,
    pub timestamp_utc: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockEventKind {
    In,
    Out,
}

static CLOCK_EVENTS: std::sync::OnceLock<Mutex<Vec<ClockEvent>>> = std::sync::OnceLock::new();

fn events() -> &'static Mutex<Vec<ClockEvent>> {
    CLOCK_EVENTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn now_utc() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", t.as_secs())
}

/// @id: miyuhr_tool_time_clock_in
/// @role: mutator
/// @layer: tool
/// @human: Enregistre une entrée (début de shift) ; heures = KindMother.
/// @do: time_clock_in_under_governance
/// tool.hr.time_clock.in
pub fn clock_in(ctx: &GovernedContext, employee_id: &str) -> Result<String, MiyuhrError> {
    if !ctx.has_mandate() {
        return Err(MiyuhrError::NoMandate);
    }
    let eid = employee_id.trim();
    if eid.is_empty() {
        return Err(MiyuhrError::InvalidInput("employee_id empty".into()));
    }
    let id = format!("{}", UuidIdGenerator.generate());
    let event = ClockEvent {
        id: id.clone(),
        employee_id: eid.to_string(),
        kind: ClockEventKind::In,
        timestamp_utc: now_utc(),
    };
    let mut guard = events()
        .lock()
        .map_err(|_| MiyuhrError::InvalidInput("lock".into()))?;
    guard.push(event);
    Ok(format!("clock:in:{eid}:{id}"))
}

/// @id: miyuhr_tool_time_clock_out
/// @role: mutator
/// @layer: tool
/// @human: Enregistre une sortie (fin de shift).
/// @do: time_clock_out_under_governance
/// tool.hr.time_clock.out
pub fn clock_out(ctx: &GovernedContext, employee_id: &str) -> Result<String, MiyuhrError> {
    if !ctx.has_mandate() {
        return Err(MiyuhrError::NoMandate);
    }
    let eid = employee_id.trim();
    if eid.is_empty() {
        return Err(MiyuhrError::InvalidInput("employee_id empty".into()));
    }
    let id = format!("{}", UuidIdGenerator.generate());
    let event = ClockEvent {
        id: id.clone(),
        employee_id: eid.to_string(),
        kind: ClockEventKind::Out,
        timestamp_utc: now_utc(),
    };
    let mut guard = events()
        .lock()
        .map_err(|_| MiyuhrError::InvalidInput("lock".into()))?;
    guard.push(event);
    Ok(format!("clock:out:{eid}:{id}"))
}

/// Liste les pointages d'un employé (ordre chronologique).
pub fn list_clock_events(
    ctx: &GovernedContext,
    employee_id: &str,
) -> Result<Vec<ClockEvent>, MiyuhrError> {
    if !ctx.has_mandate() {
        return Err(MiyuhrError::NoMandate);
    }
    let guard = events()
        .lock()
        .map_err(|_| MiyuhrError::InvalidInput("lock".into()))?;
    Ok(guard
        .iter()
        .filter(|e| e.employee_id == employee_id)
        .cloned()
        .collect())
}
