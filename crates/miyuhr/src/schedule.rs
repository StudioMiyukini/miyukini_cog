//! Tool MiyuHR — tool.hr.schedule.get.
//! Retourne le planning (shifts) pour un employé/période.

use crate::context::GovernedContext;
use crate::errors::MiyuhrError;

/// @id: miyuhr_tool_schedule_get
/// @role: mutator
/// @layer: tool
/// @human: Retourne le planning (shifts) pour un employé/période.
/// @do: schedule_get_under_governance
/// tool.hr.schedule.get
pub fn get(
    ctx: &GovernedContext,
    _employee_id: &str,
    _period_start: &str,
    _period_end: &str,
) -> Result<ScheduleResult, MiyuhrError> {
    if !ctx.has_mandate() {
        return Err(MiyuhrError::NoMandate);
    }
    Ok(ScheduleResult { shifts: Vec::new() })
}

/// Résultat planning.
#[derive(Debug, Clone, Default)]
pub struct ScheduleResult {
    pub shifts: Vec<ShiftItem>,
}

/// Élément shift.
#[derive(Debug, Clone)]
pub struct ShiftItem {
    pub id: String,
    pub start: String,
    pub end: String,
}
