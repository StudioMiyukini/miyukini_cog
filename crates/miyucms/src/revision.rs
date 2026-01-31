//! Tools MiyuCMS — tool.content.revision.* (list, restore, compare).
//! Révisions : list/compare = lecture ; restore = décision StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;

/// @id: miyucms_tool_content_revision_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les révisions d'un contenu ; lecture gouvernée.
/// @do: content_revision_list_under_governance
pub fn list(ctx: &GovernedContext, _content_id: &str) -> Result<Vec<String>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_revision_restore
/// @role: mutator
/// @layer: tool
/// @human: Restaure une révision ; décision StrongFather ; WriteIntent KindMother.
/// @do: content_revision_restore_under_governance
pub fn restore(ctx: &GovernedContext, _content_id: &str, _revision_id: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_revision_compare
/// @role: accessor
/// @layer: tool
/// @human: Compare deux révisions ; lecture seule.
/// @do: content_revision_compare_under_governance
pub fn compare(ctx: &GovernedContext, _revision_id_a: &str, _revision_id_b: &str) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}
