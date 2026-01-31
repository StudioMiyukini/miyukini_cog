//! Tools MiyuSocialFeed — tool.social.reaction.add, remove, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;

/// @id: miyusocialfeed_tool_reaction_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une réaction ; WriteIntent KindMother.
/// @do: reaction_add_under_governance
/// tool.social.reaction.add
pub fn add(
    ctx: &GovernedContext,
    _post_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_reaction_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime une réaction ; WriteIntent KindMother.
/// @do: reaction_remove_under_governance
/// tool.social.reaction.remove
pub fn remove(
    ctx: &GovernedContext,
    _post_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_reaction_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les réactions.
/// @do: reaction_list_under_governance
/// tool.social.reaction.list
pub fn list(ctx: &GovernedContext, _post_id: &str) -> Result<Vec<ReactionItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// Élément réaction.
#[derive(Debug, Clone)]
pub struct ReactionItem {
    pub user_id: String,
    pub reaction_type: String,
}
