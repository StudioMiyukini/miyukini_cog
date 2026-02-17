//! Tool MiyuDeclarations — tool.compta.declaration.deadline.list.
//! Liste les échéances fiscales et sociales (données fournies).

use crate::context::GovernedContext;
use crate::errors::MiyudeclarationsError;

/// @id: miyudeclarations_tool_deadline_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les échéances fiscales et sociales (données fournies).
/// @do: deadline_list_under_governance
/// tool.compta.declaration.deadline.list
pub fn list(
    ctx: &GovernedContext,
    _from_date: &str,
    _to_date: &str,
) -> Result<Vec<DeadlineItem>, MiyudeclarationsError> {
    if !ctx.has_mandate() {
        return Err(MiyudeclarationsError::NoMandate);
    }
    Ok(Vec::new())
}

/// Élément échéance.
#[derive(Debug, Clone)]
pub struct DeadlineItem {
    pub id: String,
    pub kind: String,
    pub due_date: String,
}
