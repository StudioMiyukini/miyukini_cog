//! Tools MiyuJobs — tool.jobs.queue.enqueue, tool.jobs.queue.process.
//! File d'attente ; autorisation = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuJobsError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// Options d'enfilement (fournies dans le flux).
#[derive(Debug, Clone, Default)]
pub struct EnqueueOptions {
    pub priority: Option<u32>,
    pub delay_until_utc_ms: Option<i64>,
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
    _payload: &str,
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
    let id = gen.generate();
    Ok(format!("task:{}:{}", q, id))
}

/// Résultat du traitement d'une tâche (handler fourni dans le flux).
#[derive(Debug, Clone, Default)]
pub struct ProcessResult {
    pub processed: bool,
    pub task_id: Option<String>,
}

/// @id: miyujobs_tool_queue_process
/// @role: mutator
/// @layer: tool
/// @human: Traite une tâche (ou un lot) depuis une queue ; handler fourni dans le flux.
/// @do: queue_process_under_governance
/// tool.jobs.queue.process
pub fn process(
    ctx: &GovernedContext,
    _queue_id: &str,
    _batch_size: u32,
) -> Result<ProcessResult, MiyuJobsError> {
    if !ctx.has_mandate() {
        return Err(MiyuJobsError::NoMandate);
    }
    Ok(ProcessResult {
        processed: false,
        task_id: None,
    })
}
