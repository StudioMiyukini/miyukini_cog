//! Tools MiyuCMS — tool.content.* (create, update, publish, schedule).
//! Contenu : create/update/schedule = WriteIntent KindMother ; publish = décision StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;

/// @id: miyucms_tool_content_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un brouillon de contenu ; données fournies ; WriteIntent KindMother.
/// @do: content_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un contenu ; WriteIntent KindMother.
/// @do: content_update_under_governance
pub fn update(ctx: &GovernedContext, _content_id: &str, _payload: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_publish
/// @role: mutator
/// @layer: tool
/// @human: Marque un contenu comme publié ; décision StrongFather.
/// @do: content_publish_under_governance
pub fn publish(ctx: &GovernedContext, _content_id: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_schedule
/// @role: mutator
/// @layer: tool
/// @human: Planifie une publication ; date/heure fournie ; WriteIntent KindMother.
/// @do: content_schedule_under_governance
pub fn schedule(ctx: &GovernedContext, _content_id: &str, _at: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}
