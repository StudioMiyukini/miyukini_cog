//! Tool MiyuPosAnalytics — tool.data.export.spreadsheet.

use crate::context::GovernedContext;
use crate::errors::MiyuposanalyticsError;

/// @id: miyuposanalytics_tool_export_spreadsheet
/// @role: mutator
/// @layer: tool
/// @human: Exporte des données en format tableur (données fournies).
/// @do: export_spreadsheet_under_governance
/// tool.data.export.spreadsheet
pub fn spreadsheet(
    ctx: &GovernedContext,
    _rows: &[Vec<String>],
    _format: &str,
) -> Result<Vec<u8>, MiyuposanalyticsError> {
    if !ctx.has_mandate() {
        return Err(MiyuposanalyticsError::NoMandate);
    }
    Err(MiyuposanalyticsError::Unimplemented)
}
