//! Tools MiyuCMS — tool.content.comment.* (create, moderate, list).
//! Commentaires : create = WriteIntent KindMother ; moderate = décision StrongFather ; list = lecture.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;

/// @id: miyucms_tool_content_comment_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un commentaire ; données fournies ; WriteIntent KindMother.
/// @do: content_comment_create_under_governance
pub fn create(ctx: &GovernedContext, _content_id: &str, _payload: &str) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_comment_moderate
/// @role: mutator
/// @layer: tool
/// @human: Applique modération (approuver, rejeter) ; décision StrongFather ; WriteIntent KindMother.
/// @do: content_comment_moderate_under_governance
pub fn moderate(ctx: &GovernedContext, _comment_id: &str, _action: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_content_comment_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les commentaires d'un contenu ; filtres fournis dans le flux.
/// @do: content_comment_list_under_governance
pub fn list(ctx: &GovernedContext, _content_id: &str, _filters: Option<&str>) -> Result<Vec<String>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}
