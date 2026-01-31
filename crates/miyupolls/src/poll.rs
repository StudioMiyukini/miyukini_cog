//! Tools MiyuPolls — tool.poll.create, vote, list, result.
//! Décision création/vote = StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyupollsError;

/// @id: miyupolls_tool_poll_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un sondage ; WriteIntent KindMother.
/// @do: poll_create_under_governance
/// tool.poll.create
pub fn create(
    ctx: &GovernedContext,
    _title: &str,
    _options: &[String],
    _expires_at: Option<&str>,
) -> Result<String, MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    Err(MiyupollsError::Unimplemented)
}

/// @id: miyupolls_tool_poll_vote
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un vote ; WriteIntent KindMother.
/// @do: poll_vote_under_governance
/// tool.poll.vote
pub fn vote(
    ctx: &GovernedContext,
    _poll_id: &str,
    _option_id: &str,
) -> Result<(), MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    Err(MiyupollsError::Unimplemented)
}

/// @id: miyupolls_tool_poll_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les sondages.
/// @do: poll_list_under_governance
/// tool.poll.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<PollItem>, MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    Err(MiyupollsError::Unimplemented)
}

/// @id: miyupolls_tool_poll_result
/// @role: mutator
/// @layer: tool
/// @human: Récupère les résultats (agrégés) ; pas d'écriture.
/// @do: poll_result_under_governance
/// tool.poll.result
pub fn result(ctx: &GovernedContext, _poll_id: &str) -> Result<PollResult, MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    Err(MiyupollsError::Unimplemented)
}

/// Élément sondage.
#[derive(Debug, Clone)]
pub struct PollItem {
    pub id: String,
    pub title: String,
    pub options: Vec<String>,
    pub expires_at: Option<String>,
}

/// Résultats agrégés.
#[derive(Debug, Clone, Default)]
pub struct PollResult {
    pub option_counts: Vec<(String, u64)>,
}
