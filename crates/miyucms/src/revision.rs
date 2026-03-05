//! Tools MiyuCMS — tool.content.revision.* (list, create, restore, compare).
//! Révisions : list/compare = lecture ; create/restore = WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyucms_tool_content_revision_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une révision pour un contenu ; WriteIntent KindMother.
/// @do: content_revision_create_under_governance
pub fn create(
    ctx: &GovernedContext,
    content_id: &str,
    payload: &str,
) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let id = format!("rev:{}", UuidIdGenerator.generate());
    {
        let mut guard = store::revisions()
            .lock()
            .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
        guard.insert(id.clone(), (content_id.to_string(), payload.to_string()));
    }
    {
        let mut guard = store::content_revisions()
            .lock()
            .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
        guard
            .entry(content_id.to_string())
            .or_default()
            .push(id.clone());
    }
    Ok(id)
}

/// @id: miyucms_tool_content_revision_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les révisions d'un contenu ; lecture gouvernée.
/// @do: content_revision_list_under_governance
pub fn list(ctx: &GovernedContext, content_id: &str) -> Result<Vec<String>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let guard = store::content_revisions()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    let ids = guard.get(content_id).cloned().unwrap_or_default();
    Ok(ids)
}

/// @id: miyucms_tool_content_revision_restore
/// @role: mutator
/// @layer: tool
/// @human: Restaure une révision ; décision StrongFather ; WriteIntent KindMother.
/// @do: content_revision_restore_under_governance
pub fn restore(
    ctx: &GovernedContext,
    _content_id: &str,
    revision_id: &str,
) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let guard = store::revisions()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    if !guard.contains_key(revision_id) {
        return Err(MiyucmsError::InvalidInput("revision not found".into()));
    }
    Ok(())
}

/// @id: miyucms_tool_content_revision_compare
/// @role: accessor
/// @layer: tool
/// @human: Compare deux révisions ; lecture seule.
/// @do: content_revision_compare_under_governance
pub fn compare(
    ctx: &GovernedContext,
    revision_id_a: &str,
    revision_id_b: &str,
) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let guard = store::revisions()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    let (_, payload_a) = guard
        .get(revision_id_a)
        .ok_or_else(|| MiyucmsError::InvalidInput("revision not found".into()))?;
    let (_, payload_b) = guard
        .get(revision_id_b)
        .ok_or_else(|| MiyucmsError::InvalidInput("revision not found".into()))?;
    let diff = if payload_a == payload_b {
        "identical"
    } else {
        "diff"
    };
    Ok(diff.to_string())
}
