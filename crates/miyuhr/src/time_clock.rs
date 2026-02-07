//! Tools MiyuHR — tool.hr.time_clock.in, tool.hr.time_clock.out.
//! Enregistre entrée/sortie ; heures = KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyuhrError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

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
    let gen = UuidIdGenerator;
    let id = gen.generate();
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
    let gen = UuidIdGenerator;
    let id = gen.generate();
    Ok(format!("clock:out:{eid}:{id}"))
}
