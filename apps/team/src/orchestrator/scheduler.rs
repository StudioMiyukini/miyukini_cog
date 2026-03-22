//! File de tâches — FIFO prioritisée pour les requêtes entrant dans TEAM.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Statut d'une tâche.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Done,
    Failed,
    Cancelled,
}

/// Priorité (plus petit = plus urgent).
pub type Priority = u8;
pub const PRIORITY_URGENT: Priority = 0;
pub const PRIORITY_HIGH: Priority = 1;
pub const PRIORITY_NORMAL: Priority = 5;
pub const PRIORITY_LOW: Priority = 10;

/// Définition d'une tâche soumise au scheduler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// ID unique.
    pub id: String,
    /// Priorité (0 = urgent, 5 = normal, 10 = low).
    pub priority: Priority,
    /// Type de tâche.
    pub kind: TaskKind,
    /// Payload (contenu spécifique au kind).
    pub payload: serde_json::Value,
    /// Tier hardware minimum requis.
    #[serde(default)]
    pub min_tier: Option<u8>,
    /// Pattern de modèle requis.
    #[serde(default)]
    pub model_pattern: Option<String>,
    /// Timeout en secondes.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    120
}

/// Type de tâche.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    /// Chat completion simple → POST /v1/agents/{id}/chat
    AgentChat,
    /// Étape MIPOWER → runner MIPOWER
    MiPowerStep,
    /// Requête directe vers un nœud spécifique.
    DirectRelay,
}

/// Entrée dans la file de tâches.
#[derive(Debug)]
struct TaskEntry {
    task: Task,
    status: TaskStatus,
    result: Option<serde_json::Value>,
    error: Option<String>,
    created_at: Instant,
    started_at: Option<Instant>,
    finished_at: Option<Instant>,
}

impl TaskEntry {
    fn new(task: Task) -> Self {
        Self {
            task,
            status: TaskStatus::Pending,
            result: None,
            error: None,
            created_at: Instant::now(),
            started_at: None,
            finished_at: None,
        }
    }
}

/// Résultat d'une tâche (retourné via l'API).
#[derive(Debug, Clone, Serialize)]
pub struct TaskResult {
    pub id: String,
    pub status: TaskStatus,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub priority: Priority,
    pub elapsed_ms: Option<u64>,
}

/// Scheduler FIFO prioritisé.
#[derive(Clone, Default)]
pub struct TaskScheduler {
    /// Clé = (priority, created_at_nanos) pour ordre FIFO dans chaque priorité.
    queue: Arc<Mutex<BTreeMap<(Priority, u128), String>>>,
    /// Map id → entry complète.
    tasks: Arc<Mutex<std::collections::HashMap<String, TaskEntry>>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Soumet une nouvelle tâche. Retourne son ID.
    pub fn submit(&self, mut task: Task) -> String {
        if task.id.is_empty() {
            task.id = Uuid::new_v4().to_string();
        }
        let id = task.id.clone();
        let entry = TaskEntry::new(task.clone());
        let key = (task.priority, entry.created_at.elapsed().subsec_nanos() as u128);

        self.queue.lock().unwrap_or_else(|e| e.into_inner()).insert(key, id.clone());
        self.tasks.lock().unwrap_or_else(|e| e.into_inner()).insert(id.clone(), entry);

        tracing::info!("Tâche soumise : {id} (priority={})", task.priority);
        id
    }

    /// Prend la prochaine tâche en attente (FIFO prioritisé).
    pub fn next_pending(&self) -> Option<Task> {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        let mut tasks = self.tasks.lock().unwrap_or_else(|e| e.into_inner());

        // Trouver la première tâche pending dans l'ordre de priorité
        let found = queue.iter().find_map(|(key, id)| {
            tasks.get(id).and_then(|entry| {
                if entry.status == TaskStatus::Pending {
                    Some((*key, id.clone()))
                } else {
                    None
                }
            })
        });

        if let Some((key, id)) = found {
            if let Some(entry) = tasks.get_mut(&id) {
                entry.status = TaskStatus::Running;
                entry.started_at = Some(Instant::now());
            }
            queue.remove(&key);
            tasks.get(&id).map(|e| e.task.clone())
        } else {
            None
        }
    }

    /// Marque une tâche comme terminée avec résultat.
    pub fn complete(&self, id: &str, result: serde_json::Value) {
        let mut tasks = self.tasks.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(entry) = tasks.get_mut(id) {
            entry.status = TaskStatus::Done;
            entry.result = Some(result);
            entry.finished_at = Some(Instant::now());
        }
    }

    /// Marque une tâche comme échouée.
    pub fn fail(&self, id: &str, error: impl Into<String>) {
        let mut tasks = self.tasks.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(entry) = tasks.get_mut(id) {
            entry.status = TaskStatus::Failed;
            entry.error = Some(error.into());
            entry.finished_at = Some(Instant::now());
        }
    }

    /// Retourne l'état d'une tâche.
    pub fn get_result(&self, id: &str) -> Option<TaskResult> {
        let tasks = self.tasks.lock().unwrap_or_else(|e| e.into_inner());
        tasks.get(id).map(|entry| {
            let elapsed_ms = entry
                .started_at
                .and_then(|s| entry.finished_at.map(|f| f.duration_since(s).as_millis() as u64));
            TaskResult {
                id: entry.task.id.clone(),
                status: entry.status,
                result: entry.result.clone(),
                error: entry.error.clone(),
                priority: entry.task.priority,
                elapsed_ms,
            }
        })
    }

    /// Nombre de tâches en attente.
    pub fn pending_count(&self) -> usize {
        self.tasks
            .lock()
            .unwrap()
            .values()
            .filter(|e| e.status == TaskStatus::Pending)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_task(priority: Priority) -> Task {
        Task {
            id: String::new(),
            priority,
            kind: TaskKind::AgentChat,
            payload: serde_json::json!({"message": "test"}),
            min_tier: None,
            model_pattern: None,
            timeout_secs: 30,
        }
    }

    #[test]
    fn test_submit_and_get_result() {
        let scheduler = TaskScheduler::new();
        let id = scheduler.submit(make_task(PRIORITY_NORMAL));
        let result = scheduler.get_result(&id).unwrap();
        assert_eq!(result.status, TaskStatus::Pending);
    }

    #[test]
    fn test_next_pending_fifo() {
        let scheduler = TaskScheduler::new();
        let id1 = scheduler.submit(make_task(PRIORITY_NORMAL));
        let id2 = scheduler.submit(make_task(PRIORITY_HIGH));

        // Urgent first
        let task = scheduler.next_pending();
        assert!(task.is_some());
        // La tâche high priority doit venir avant normal
        assert_eq!(task.unwrap().priority, PRIORITY_HIGH);
    }

    #[test]
    fn test_complete_task() {
        let scheduler = TaskScheduler::new();
        let id = scheduler.submit(make_task(PRIORITY_NORMAL));
        scheduler.next_pending(); // passer en Running
        scheduler.complete(&id, serde_json::json!({"text": "ok"}));
        let result = scheduler.get_result(&id).unwrap();
        assert_eq!(result.status, TaskStatus::Done);
    }

    #[test]
    fn test_fail_task() {
        let scheduler = TaskScheduler::new();
        let id = scheduler.submit(make_task(PRIORITY_NORMAL));
        scheduler.next_pending();
        scheduler.fail(&id, "Timeout");
        let result = scheduler.get_result(&id).unwrap();
        assert_eq!(result.status, TaskStatus::Failed);
    }
}
