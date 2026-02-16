//! Tools MiyuPolls — tool.poll.create, vote, list, result.
//! Store en mémoire (KindMother côté produit pour persistance).

use crate::context::GovernedContext;
use crate::errors::MiyupollsError;
use std::collections::HashMap;
use std::sync::Mutex;

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

struct PollRecord {
    item: PollItem,
    votes: HashMap<String, u64>, // option_id -> count
}

static POLLS: std::sync::OnceLock<Mutex<HashMap<String, PollRecord>>> = std::sync::OnceLock::new();

fn polls() -> &'static Mutex<HashMap<String, PollRecord>> {
    POLLS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyupolls_tool_poll_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un sondage ; WriteIntent KindMother.
/// @do: poll_create_under_governance
/// tool.poll.create
pub fn create(
    ctx: &GovernedContext,
    title: &str,
    options: &[String],
    expires_at: Option<&str>,
) -> Result<String, MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let id = format!("poll:{}", NEXT.fetch_add(1, Ordering::Relaxed));
    let item = PollItem {
        id: id.clone(),
        title: title.to_string(),
        options: options.to_vec(),
        expires_at: expires_at.map(String::from),
    };
    let record = PollRecord {
        item: item.clone(),
        votes: HashMap::new(),
    };
    let mut guard = polls().lock().map_err(|_| MiyupollsError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), record);
    Ok(id)
}

/// @id: miyupolls_tool_poll_vote
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un vote ; WriteIntent KindMother.
/// @do: poll_vote_under_governance
/// tool.poll.vote
pub fn vote(
    ctx: &GovernedContext,
    poll_id: &str,
    option_id: &str,
) -> Result<(), MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    let mut guard = polls().lock().map_err(|_| MiyupollsError::InvalidInput("lock".into()))?;
    let rec = guard.get_mut(poll_id).ok_or_else(|| MiyupollsError::InvalidInput("poll not found".into()))?;
    if !rec.item.options.iter().any(|o| o == option_id) {
        return Err(MiyupollsError::InvalidInput("option not found".into()));
    }
    *rec.votes.entry(option_id.to_string()).or_insert(0) += 1;
    Ok(())
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
    let guard = polls().lock().map_err(|_| MiyupollsError::InvalidInput("lock".into()))?;
    Ok(guard.values().map(|r| r.item.clone()).collect())
}

/// @id: miyupolls_tool_poll_result
/// @role: mutator
/// @layer: tool
/// @human: Récupère les résultats (agrégés) ; pas d'écriture.
/// @do: poll_result_under_governance
/// tool.poll.result
pub fn result(ctx: &GovernedContext, poll_id: &str) -> Result<PollResult, MiyupollsError> {
    if !ctx.has_mandate() {
        return Err(MiyupollsError::NoMandate);
    }
    let guard = polls().lock().map_err(|_| MiyupollsError::InvalidInput("lock".into()))?;
    let rec = guard.get(poll_id).ok_or_else(|| MiyupollsError::InvalidInput("poll not found".into()))?;
    let option_counts: Vec<(String, u64)> = rec
        .item
        .options
        .iter()
        .map(|opt| (opt.clone(), rec.votes.get(opt).copied().unwrap_or(0)))
        .collect();
    Ok(PollResult { option_counts })
}
