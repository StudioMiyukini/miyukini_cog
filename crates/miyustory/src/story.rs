//! Tools MiyuStory — tool.social.story.create, list, get, reaction.add.
//! Décision création/visibilité = StrongFather ; WriteIntent KindMother ; expiration = Ever Buddy.

use crate::context::GovernedContext;
use crate::errors::MiyustoryError;

/// @id: miyustory_tool_story_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une story ; WriteIntent KindMother.
/// @do: story_create_under_governance
/// tool.social.story.create
pub fn create(
    ctx: &GovernedContext,
    _content_type: &str,
    _payload: &[u8],
) -> Result<String, MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    Err(MiyustoryError::Unimplemented)
}

/// @id: miyustory_tool_story_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les stories ; filtres fournis.
/// @do: story_list_under_governance
/// tool.social.story.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &StoryFilters,
) -> Result<Vec<StoryItem>, MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    Err(MiyustoryError::Unimplemented)
}

/// @id: miyustory_tool_story_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère une story.
/// @do: story_get_under_governance
/// tool.social.story.get
pub fn get(ctx: &GovernedContext, _story_id: &str) -> Result<StoryDetail, MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    Err(MiyustoryError::Unimplemented)
}

/// @id: miyustory_tool_story_reaction_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une réaction à une story ; WriteIntent KindMother.
/// @do: story_reaction_add_under_governance
/// tool.social.story.reaction.add
pub fn reaction_add(
    ctx: &GovernedContext,
    _story_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyustoryError> {
    if !ctx.has_mandate() {
        return Err(MiyustoryError::NoMandate);
    }
    Err(MiyustoryError::Unimplemented)
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
