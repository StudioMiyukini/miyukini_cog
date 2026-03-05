//! Tools MiyuStory — tool.social.story.create, list, get, reaction.add.
//! Décision création/visibilité = StrongFather ; WriteIntent KindMother ; expiration = Ever Buddy.

use crate::context::GovernedContext;
use crate::errors::MiyustoryError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyustory_tool_story_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une story ; WriteIntent KindMother.
/// @do: story_create_under_governance
/// tool.social.story.create
pub fn create(
    ctx: &GovernedContext,
    content_type: &str,
    payload: &[u8],
) -> Result<String, MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    let id = format!("story:{}", UuidIdGenerator.generate());
    let author_id = ctx.mandate_id.clone();
    let expires_at = String::new();
    let mut guard = store::stories()
        .lock()
        .map_err(|_| MiyustoryError::InvalidInput("lock".into()))?;
    guard.insert(
        id.clone(),
        (
            author_id,
            content_type.to_string(),
            payload.to_vec(),
            expires_at,
        ),
    );
    Ok(id)
}

/// @id: miyustory_tool_story_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les stories ; filtres fournis.
/// @do: story_list_under_governance
/// tool.social.story.list
pub fn list(
    ctx: &GovernedContext,
    filters: &StoryFilters,
) -> Result<Vec<StoryItem>, MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    let guard = store::stories()
        .lock()
        .map_err(|_| MiyustoryError::InvalidInput("lock".into()))?;
    let mut items: Vec<StoryItem> = guard
        .iter()
        .filter(|(_, (author_id, _, _, _))| {
            filters.author_id.as_ref().map_or(true, |a| author_id == a)
        })
        .map(|(id, (author_id, content_type, _, expires_at))| StoryItem {
            id: id.clone(),
            author_id: author_id.clone(),
            content_type: content_type.clone(),
            expires_at: expires_at.clone(),
        })
        .collect();
    if let Some(limit) = filters.limit {
        items.truncate(limit as usize);
    }
    Ok(items)
}

/// @id: miyustory_tool_story_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère une story.
/// @do: story_get_under_governance
/// tool.social.story.get
pub fn get(ctx: &GovernedContext, story_id: &str) -> Result<StoryDetail, MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    let guard = store::stories()
        .lock()
        .map_err(|_| MiyustoryError::InvalidInput("lock".into()))?;
    let (author_id, content_type, payload, expires_at) = guard
        .get(story_id)
        .ok_or_else(|| MiyustoryError::InvalidInput("story not found".into()))?
        .clone();
    Ok(StoryDetail {
        id: story_id.to_string(),
        author_id,
        content_type,
        payload,
        expires_at,
    })
}

/// @id: miyustory_tool_story_reaction_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une réaction à une story ; WriteIntent KindMother.
/// @do: story_reaction_add_under_governance
/// tool.social.story.reaction.add
pub fn reaction_add(
    ctx: &GovernedContext,
    story_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    let guard = store::stories()
        .lock()
        .map_err(|_| MiyustoryError::InvalidInput("lock".into()))?;
    if !guard.contains_key(story_id) {
        return Err(MiyustoryError::InvalidInput("story not found".into()));
    }
    Ok(())
}

/// Filtres liste stories.
#[derive(Debug, Clone, Default)]
pub struct StoryFilters {
    pub author_id: Option<String>,
    pub limit: Option<u32>,
}

/// Élément story.
#[derive(Debug, Clone)]
pub struct StoryItem {
    pub id: String,
    pub author_id: String,
    pub content_type: String,
    pub expires_at: String,
}

/// Détail story.
#[derive(Debug, Clone, Default)]
pub struct StoryDetail {
    pub id: String,
    pub author_id: String,
    pub content_type: String,
    pub payload: Vec<u8>,
    pub expires_at: String,
}
