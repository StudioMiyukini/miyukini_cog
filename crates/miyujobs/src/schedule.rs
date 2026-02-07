//! Tools MiyuJobs — tool.jobs.schedule.at, tool.jobs.schedule.cron.
//! Planification ; autorisation = StrongFather (contexte fourni après ALLOW).

use crate::context::GovernedContext;
use crate::errors::MiyuJobsError;

/// @id: miyujobs_tool_schedule_at
/// @role: mutator
/// @layer: tool
/// @human: Planifie une exécution à une date/heure fournie ; autorisation = StrongFather.
/// @do: schedule_at_under_governance
/// tool.jobs.schedule.at
pub fn schedule_at(
    ctx: &GovernedContext,
    run_at_utc_ms: i64,
    job_id: &str,
    _payload: &str,
) -> Result<String, MiyuJobsError> {
    if !ctx.has_mandate() {
        return Err(MiyuJobsError::NoMandate);
    }
    let id = job_id.trim();
    if id.is_empty() {
        return Err(MiyuJobsError::InvalidInput("job_id empty".into()));
    }
    Ok(format!("at:{id}:{run_at_utc_ms}"))
}

/// @id: miyujobs_tool_schedule_cron
/// @role: mutator
/// @layer: tool
/// @human: Planifie une exécution selon expression cron fournie ; autorisation = StrongFather.
/// @do: schedule_cron_under_governance
/// tool.jobs.schedule.cron
pub fn schedule_cron(
    ctx: &GovernedContext,
    cron_expression: &str,
    job_id: &str,
    _payload: &str,
) -> Result<String, MiyuJobsError> {
    if !ctx.has_mandate() {
        return Err(MiyuJobsError::NoMandate);
    }
    let id = job_id.trim();
    if id.is_empty() {
        return Err(MiyuJobsError::InvalidInput("job_id empty".into()));
    }
    if !validate_cron_five_fields(cron_expression) {
        return Err(MiyuJobsError::InvalidInput("invalid cron expression".into()));
    }
    let safe = cron_expression.trim().replace(' ', "_");
    Ok(format!("cron:{id}:{safe}"))
}

fn validate_cron_five_fields(s: &str) -> bool {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }
    parts.iter().all(|p| !p.is_empty() && p.len() <= 4)
}
