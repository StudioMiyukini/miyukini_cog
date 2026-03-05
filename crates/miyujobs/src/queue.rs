//! Tools MiyuJobs — tool.jobs.queue.enqueue, tool.jobs.queue.process.
//! File d'attente ; autorisation = StrongFather.
//! Store par défaut en mémoire (persistance réelle déléguée à KindMother côté produit).

use crate::context::GovernedContext;
use crate::errors::MiyuJobsError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

/// Options d'enfilement (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct EnqueueOptions {
    pub priority: Option<u32>,
    pub delay_until_utc_ms: Option<i64>,
}

/// Statut d'une tâche en file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Processed,
}

/// Tâche enregistrée (format interne store).
#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,
    pub queue_id: String,
    pub payload: String,
    pub status: TaskStatus,
}

/// Store des files (en mémoire) ; une file = une VecDeque de tâches.
struct QueueStore {
    queues: HashMap<String, VecDeque<Task>>,
}

impl Default for QueueStore {
    fn default() -> Self {
        Self {
            queues: HashMap::new(),
        }
    }
}

static DEFAULT_STORE: std::sync::OnceLock<Mutex<QueueStore>> = std::sync::OnceLock::new();

fn default_store() -> &'static Mutex<QueueStore> {
    DEFAULT_STORE.get_or_init(|| Mutex::new(QueueStore::default()))
}

/// @id: miyujobs_tool_queue_enqueue
/// @role: mutator
/// @layer: tool
/// @human: Enfile une tâche (payload, queue, options fournis) ; autorisation = StrongFather.
/// @do: queue_enqueue_under_governance
/// tool.jobs.queue.enqueue
pub fn enqueue(
    ctx: &GovernedContext,
    queue_id: &str,
    payload: &str,
    _options: &EnqueueOptions,
) -> Result<String, MiyuJobsError> {
    if !ctx.has_mandate() {
        return Err(MiyuJobsError::NoMandate);
    }
    let q = queue_id.trim();
    if q.is_empty() {
        return Err(MiyuJobsError::InvalidInput("queue_id empty".into()));
    }
    let gen = UuidIdGenerator;
    let id = gen.generate().to_string();
    let task = Task {
        id: id.clone(),
        queue_id: q.to_string(),
        payload: payload.to_string(),
        status: TaskStatus::Pending,
    };
    let store = default_store();
    let mut guard = store
        .lock()
        .map_err(|_| MiyuJobsError::InvalidInput("store lock".into()))?;
    guard
        .queues
        .entry(q.to_string())
        .or_default()
        .push_back(task);
    Ok(format!("task:{q}:{id}"))
}

/// Résultat du traitement (ids des tâches traitées).
#[derive(Debug, Clone, Default)]
pub struct ProcessResult {
    pub processed: bool,
    pub task_id: Option<String>,
    /// Tous les ids des tâches traitées dans ce lot.
    pub task_ids: Vec<String>,
}

/// @id: miyujobs_tool_queue_process
/// @role: mutator
/// @layer: tool
/// @human: Traite jusqu'à batch_size tâches en attente ; handler à appeler côté flux sur les payloads.
/// @do: queue_process_under_governance
/// tool.jobs.queue.process
pub fn process(
    ctx: &GovernedContext,
    queue_id: &str,
    batch_size: u32,
) -> Result<ProcessResult, MiyuJobsError> {
    if !ctx.has_mandate() {
        return Err(MiyuJobsError::NoMandate);
    }
    let q = queue_id.trim();
    if q.is_empty() {
        return Err(MiyuJobsError::InvalidInput("queue_id empty".into()));
    }
    let store = default_store();
    let mut guard = store
        .lock()
        .map_err(|_| MiyuJobsError::InvalidInput("store lock".into()))?;
    let queue = guard.queues.get_mut(q);
    let (processed, task_ids) = if let Some(queue_ref) = queue {
        let n = (batch_size as usize).min(queue_ref.len());
        let ids: Vec<String> = (0..n)
            .filter_map(|_| queue_ref.pop_front())
            .map(|t| t.id)
            .collect();
        (!ids.is_empty(), ids)
    } else {
        (false, vec![])
    };
    drop(guard);
    Ok(ProcessResult {
        processed,
        task_id: task_ids.first().cloned(),
        task_ids,
    })
}

/// Récupère une tâche par id (pour exécution du handler côté flux) ; lecture seule.
pub fn get_task(ctx: &GovernedContext, task_id: &str) -> Result<Option<Task>, MiyuJobsError> {
    if !ctx.has_mandate() {
        return Err(MiyuJobsError::NoMandate);
    }
    let store = default_store();
    let guard = store
        .lock()
        .map_err(|_| MiyuJobsError::InvalidInput("store lock".into()))?;
    for queue in guard.queues.values() {
        for t in queue {
            if t.id == task_id {
                return Ok(Some(t.clone()));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_then_process() {
        let ctx = GovernedContext::new("m1".into(), 1);
        let id1 = enqueue(&ctx, "q1", "payload1", &EnqueueOptions::default()).unwrap();
        let id2 = enqueue(&ctx, "q1", "payload2", &EnqueueOptions::default()).unwrap();
        assert!(id1.starts_with("task:q1:"));
        assert!(id2.starts_with("task:q1:"));

        let res = process(&ctx, "q1", 2).unwrap();
        assert!(res.processed);
        assert_eq!(res.task_ids.len(), 2);

        let res2 = process(&ctx, "q1", 1).unwrap();
        assert!(!res2.processed);
        assert!(res2.task_ids.is_empty());
    }
}
