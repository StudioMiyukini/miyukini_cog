//! Tools MiyuModerationForum — tool.moderation.report.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyumoderationforum_tool_report_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un signalement ; WriteIntent KindMother.
/// @do: report_create_under_governance
/// tool.moderation.report.create
pub fn create(
    ctx: &GovernedContext,
    target_type: &str,
    target_id: &str,
    reason: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let id = format!("rep:{}", UuidIdGenerator.generate());
    let mut guard = store::reports().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), (target_type.to_string(), target_id.to_string(), reason.to_string()));
    Ok(id)
}

/// @id: miyumoderationforum_tool_report_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les signalements.
/// @do: report_list_under_governance
/// tool.moderation.report.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<ReportItem>, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let guard = store::reports().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let items = guard.iter().map(|(id, (target_type, target_id, reason))| ReportItem {
        id: id.clone(),
        target_type: target_type.clone(),
        target_id: target_id.clone(),
        reason: reason.clone(),
    }).collect();
    Ok(items)
}

/// Élément signalement.
#[derive(Debug, Clone)]
pub struct ReportItem {
    pub id: String,
    pub target_type: String,
    pub target_id: String,
    pub reason: String,
}
