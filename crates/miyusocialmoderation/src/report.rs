//! Tools MiyuSocialModeration — tool.moderation.report.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmoderationError;

/// @id: miyusocialmoderation_tool_report_create
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
) -> Result<String, MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    Err(MiyusocialmoderationError::Unimplemented)
}

/// @id: miyusocialmoderation_tool_report_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les signalements.
/// @do: report_list_under_governance
/// tool.moderation.report.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<ReportItem>, MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    Err(MiyusocialmoderationError::Unimplemented)
}

/// Élément signalement.
#[derive(Debug, Clone)]
pub struct ReportItem {
    pub id: String,
    pub target_type: String,
    pub target_id: String,
}
