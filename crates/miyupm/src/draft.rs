//! Tools MiyuPM — tool.pm.draft.create, tool.pm.draft.update, tool.pm.draft.list.
//! Création, mise à jour et liste des brouillons ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyupmError;

/// @id miyupm_tool_pm_draft_create
/// @role mutator
/// @layer tool
/// @do pm_draft_create_under_governance
//@human / @human Crée un brouillon ; WriteIntent KindMother.
/// tool.pm.draft.create — ne décide pas ; WriteIntent.
pub fn create(
    ctx: &GovernedContext,
    _content: &str,
    _subject: Option<&str>,
) -> Result<String, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Ok(String::new())
}

/// @id miyupm_tool_pm_draft_update
/// @role mutator
/// @layer tool
/// @do pm_draft_update_under_governance
//@human / @human Met à jour un brouillon ; WriteIntent KindMother.
/// tool.pm.draft.update — ne décide pas ; WriteIntent.
pub fn update(ctx: &GovernedContext, _draft_id: &str, _payload: &str) -> Result<(), MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Ok(())
}

/// @id miyupm_tool_pm_draft_list
/// @role accessor
/// @layer tool
/// @do pm_draft_list_under_governance
//@human / @human Liste les brouillons.
/// tool.pm.draft.list — lecture.
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Ok(Vec::new())
}
