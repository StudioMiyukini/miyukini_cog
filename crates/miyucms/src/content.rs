//! Tools MiyuCMS — tool.content.* (create, update, publish, schedule).
//! Store en mémoire : content_id -> (payload, status: draft|scheduled|published).

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

struct ContentRecord {
    payload: String,
    status: String,
}

static CONTENTS: std::sync::OnceLock<Mutex<HashMap<String, ContentRecord>>> =
    std::sync::OnceLock::new();

fn contents() -> &'static Mutex<HashMap<String, ContentRecord>> {
    CONTENTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id miyucms_tool_content_create
/// @role mutator
/// @layer tool
/// @do content_create_under_governance
//@human / @human Crée un brouillon de contenu ; données fournies ; WriteIntent KindMother.
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let id = format!("cms:{}", UuidIdGenerator.generate());
    let mut guard = contents()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    guard.insert(
        id.clone(),
        ContentRecord {
            payload: payload.to_string(),
            status: "draft".to_string(),
        },
    );
    Ok(id)
}

/// @id miyucms_tool_content_update
/// @role mutator
/// @layer tool
/// @do content_update_under_governance
//@human / @human Met à jour un contenu ; WriteIntent KindMother.
pub fn update(ctx: &GovernedContext, content_id: &str, payload: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let mut guard = contents()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    if let Some(rec) = guard.get_mut(content_id) {
        rec.payload = payload.to_string();
    }
    Ok(())
}

/// @id miyucms_tool_content_publish
/// @role mutator
/// @layer tool
/// @do content_publish_under_governance
//@human / @human Marque un contenu comme publié ; décision StrongFather.
pub fn publish(ctx: &GovernedContext, content_id: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let mut guard = contents()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    if let Some(rec) = guard.get_mut(content_id) {
        rec.status = "published".to_string();
    }
    Ok(())
}

/// @id miyucms_tool_content_schedule
/// @role mutator
/// @layer tool
/// @do content_schedule_under_governance
//@human / @human Planifie une publication ; date/heure fournie ; WriteIntent KindMother.
pub fn schedule(ctx: &GovernedContext, content_id: &str, _at: &str) -> Result<(), MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let mut guard = contents()
        .lock()
        .map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    if let Some(rec) = guard.get_mut(content_id) {
        rec.status = "scheduled".to_string();
    }
    Ok(())
}
