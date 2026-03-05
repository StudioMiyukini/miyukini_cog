//! Tools MiyuCMS — tool.content.comment.* (create, moderate, list).
//! Commentaires : create = WriteIntent KindMother ; moderate = décision StrongFather ; list = lecture.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyucms_tool_content_comment_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un commentaire ; données fournies ; WriteIntent KindMother.
/// @do: content_comment_create_under_governance
pub fn create(
    ctx: &GovernedContext,
    content_id: &str,
    payload: &str,
) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let id = format!("com:{}", UuidIdGenerator.generate());
    {
        let mut guard = store::comments()
            .lock()
            .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
        guard.insert(id.clone(), (content_id.to_string(), payload.to_string()));
    }
    {
        let mut guard = store::content_comments()
            .lock()
            .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
        guard
            .entry(content_id.to_string())
            .or_default()
            .push(id.clone());
    }
    Ok(id)
}

/// @id: miyucms_tool_content_comment_moderate
/// @role: mutator
/// @layer: tool
/// @human: Applique modération (approuver, rejeter) ; décision StrongFather ; WriteIntent KindMother.
/// @do: content_comment_moderate_under_governance
pub fn moderate(
    ctx: &GovernedContext,
    comment_id: &str,
    _action: &str,
) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let _guard = store::comments()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    if !store::comments()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?
        .contains_key(comment_id)
    {
        return Err(MiyucmsError::InvalidInput("comment not found".into()));
    }
    Ok(())
}

/// @id: miyucms_tool_content_comment_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les commentaires d'un contenu ; filtres fournis dans le flux.
/// @do: content_comment_list_under_governance
pub fn list(
    ctx: &GovernedContext,
    content_id: &str,
    _filters: Option<&str>,
) -> Result<Vec<String>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let guard = store::content_comments()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    let ids = guard.get(content_id).cloned().unwrap_or_default();
    Ok(ids)
}
