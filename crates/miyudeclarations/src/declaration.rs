//! Tool MiyuDeclarations — tool.compta.declaration.list.
//! Liste l'historique des déclarations (filtres fournis).

use crate::context::GovernedContext;
use crate::errors::MiyudeclarationsError;

/// @id: miyudeclarations_tool_declaration_list
/// @role: mutator
/// @layer: tool
/// @human: Liste l'historique des déclarations (filtres fournis).
/// @do: declaration_list_under_governance
/// tool.compta.declaration.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &DeclarationFilters,
) -> Result<Vec<DeclarationItem>, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Err(MiyudeclarationsError::Unimplemented)
}

/// Filtres liste déclarations.
#[derive(Debug, Clone, Default)]
pub struct DeclarationFilters {
    pub kind: Option<String>,
    pub period: Option<String>,
}

/// Élément déclaration.
#[derive(Debug, Clone)]
pub struct DeclarationItem {
    pub id: String,
    pub kind: String,
    pub period: String,
    pub status: String,
}
