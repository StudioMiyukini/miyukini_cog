//! Tools MiyuPM — tool.pm.folder.list, tool.pm.folder.create, tool.pm.folder.update.
//! Liste, création et mise à jour de dossiers ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyupmError;

/// @id miyupm_tool_pm_folder_list
/// @role accessor
/// @layer tool
/// @do pm_folder_list_under_governance
//@human / @human Liste les dossiers.
/// tool.pm.folder.list — lecture.
pub fn list(ctx: &GovernedContext) -> Result<Vec<String>, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id miyupm_tool_pm_folder_create
/// @role mutator
/// @layer tool
/// @do pm_folder_create_under_governance
//@human / @human Crée un dossier ; WriteIntent KindMother.
/// tool.pm.folder.create — ne décide pas ; WriteIntent.
pub fn create(ctx: &GovernedContext, _name: &str) -> Result<String, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Ok(String::new())
}

/// @id miyupm_tool_pm_folder_update
/// @role mutator
/// @layer tool
/// @do pm_folder_update_under_governance
//@human / @human Met à jour un dossier ; WriteIntent KindMother.
/// tool.pm.folder.update — ne décide pas ; WriteIntent.
pub fn update(ctx: &GovernedContext, _folder_id: &str, _payload: &str) -> Result<(), MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Ok(())
}
