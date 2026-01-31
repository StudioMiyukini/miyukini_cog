//! Tool MiyuPM — tool.pm.export.
//! Exporte les messages ; exécution seule, pas d'écriture métier.

use crate::context::GovernedContext;
use crate::errors::MiyupmError;

/// @id: miyupm_tool_pm_export
/// @role: accessor
/// @layer: tool
/// @human: Exporte les messages ; exécution seule, pas d'écriture métier.
/// @do: pm_export_under_governance
/// tool.pm.export — ne décide pas ; critères fournis.
pub fn export(
    ctx: &GovernedContext,
    _filters: Option<&str>,
    _format: Option<&str>,
) -> Result<Vec<u8>, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Err(MiyupmError::Unimplemented)
}
