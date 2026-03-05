//! Tools MiyuSocialFeed — tool.social.reaction.add, remove, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;
use crate::store;

/// @id: miyusocialfeed_tool_reaction_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une réaction ; WriteIntent KindMother.
/// @do: reaction_add_under_governance
/// tool.social.reaction.add
pub fn add(
    ctx: &GovernedContext,
    post_id: &str,
    reaction_type: &str,
) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let user_id = ctx.mandate_id.clone();
    let mut guard = store::reactions()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    guard
        .entry(post_id.to_string())
        .or_default()
        .push((user_id, reaction_type.to_string()));
    Ok(())
}

/// @id: miyusocialfeed_tool_reaction_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime une réaction ; WriteIntent KindMother.
/// @do: reaction_remove_under_governance
/// tool.social.reaction.remove
pub fn remove(
    ctx: &GovernedContext,
    post_id: &str,
    reaction_type: &str,
) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let user_id = ctx.mandate_id.clone();
    let mut guard = store::reactions()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    if let Some(v) = guard.get_mut(post_id) {
        v.retain(|(u, r)| !(u == &user_id && r == reaction_type));
    }
    Ok(())
}

/// @id: miyusocialfeed_tool_reaction_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les réactions.
/// @do: reaction_list_under_governance
/// tool.social.reaction.list
pub fn list(
    ctx: &GovernedContext,
    post_id: &str,
) -> Result<Vec<ReactionItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let guard = store::reactions()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let items = guard
        .get(post_id)
        .map(|v| {
            v.iter()
                .map(|(user_id, reaction_type)| ReactionItem {
                    user_id: user_id.clone(),
                    reaction_type: reaction_type.clone(),
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(items)
}

/// Élément réaction.
#[derive(Debug, Clone)]
pub struct ReactionItem {
    pub user_id: String,
    pub reaction_type: String,
}
