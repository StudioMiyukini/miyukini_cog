//! Tools MiyuModerationForum — tool.moderation.report.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_report_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un signalement ; WriteIntent KindMother.
/// @do: report_create_under_governance
/// tool.moderation.report.create
pub fn create(
    ctx: &GovernedContext,
    _target_type: &str,
    _target_id: &str,
    _reason: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
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
    Err(MiyumoderationforumError::Unimplemented)
}

/// Élément signalement.
#[derive(Debug, Clone)]
pub struct ReportItem {
    pub id: String,
    pub target_type: String,
    pub target_id: String,
    pub reason: String,
}
