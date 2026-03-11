//! Scheduler pour Alicia — taches one-shot et cron.
//!
//! Stocke les taches dans SQLite, verifie toutes les 30s si une tache est due.

use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;

/// Type de tache.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Once,
    Cron,
}

/// Tache planifiee.
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: String,
    pub task_type: TaskType,
    pub schedule: String,
    pub action: String,
    pub next_run: Option<String>,
    pub active: bool,
}

/// Scheduler avec persistance SQLite et background loop.
pub struct Scheduler {
    db_path: PathBuf,
    shutdown: Arc<RwLock<bool>>,
}

impl Scheduler {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            shutdown: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialise la table scheduled_tasks.
    pub fn init_db(&self) -> anyhow::Result<()> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS scheduled_tasks (
                 id TEXT PRIMARY KEY,
                 task_type TEXT NOT NULL,
                 schedule TEXT NOT NULL,
                 action TEXT NOT NULL,
                 created_at TEXT DEFAULT (datetime('now')),
                 last_run TEXT,
                 next_run TEXT,
                 active INTEGER DEFAULT 1
             );",
        )?;
        Ok(())
    }

    /// Planifie une tache.
    /// - type "once" : schedule = "5m", "1h", "30s", "1d"
    /// - type "cron" : schedule = "*/5 * * * *" (5 champs)
    pub fn add_task(
        &self,
        task_type: &str,
        schedule: &str,
        action: &str,
    ) -> anyhow::Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let tt = match task_type {
            "once" => TaskType::Once,
            "cron" => TaskType::Cron,
            _ => anyhow::bail!("Type invalide: '{task_type}'. Utilise 'once' ou 'cron'."),
        };

        let next_run = match tt {
            TaskType::Once => {
                let duration = parse_duration(schedule)?;
                let next = chrono::Utc::now() + duration;
                next.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            TaskType::Cron => {
                if !validate_cron_five_fields(schedule) {
                    anyhow::bail!("Expression cron invalide: '{schedule}'. Format: 'min hour dom month dow'");
                }
                // Le next_run sera calcule par le background loop
                "pending".to_string()
            }
        };

        let conn = rusqlite::Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO scheduled_tasks (id, task_type, schedule, action, next_run)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, task_type, schedule, action, next_run],
        )?;

        println!("  [Scheduler] Tache ajoutee: {id} ({task_type}, {schedule})");
        Ok(id)
    }

    /// Liste les taches actives.
    pub fn list_tasks(&self) -> anyhow::Result<Vec<ScheduledTask>> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare(
            "SELECT id, task_type, schedule, action, next_run, active
             FROM scheduled_tasks WHERE active = 1 ORDER BY next_run",
        )?;

        let tasks = stmt
            .query_map([], |row| {
                Ok(ScheduledTask {
                    id: row.get(0)?,
                    task_type: if row.get::<_, String>(1)? == "cron" {
                        TaskType::Cron
                    } else {
                        TaskType::Once
                    },
                    schedule: row.get(2)?,
                    action: row.get(3)?,
                    next_run: row.get(4)?,
                    active: row.get::<_, i32>(5)? != 0,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(tasks)
    }

    /// Annule une tache.
    pub fn cancel_task(&self, id: &str) -> anyhow::Result<bool> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        let affected = conn.execute(
            "UPDATE scheduled_tasks SET active = 0 WHERE id = ?1 AND active = 1",
            [id],
        )?;
        Ok(affected > 0)
    }

    /// Demarre la boucle de verification en arriere-plan.
    pub fn start_background(self: Arc<Self>) {
        let shutdown = Arc::clone(&self.shutdown);
        let db_path = self.db_path.clone();

        tokio::spawn(async move {
            println!("  [Scheduler] Background loop demarree (check toutes les 30s)");

            loop {
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;

                if *shutdown.read().await {
                    println!("  [Scheduler] Background loop arretee");
                    break;
                }

                // Verifier les taches dues
                if let Err(e) = check_due_tasks(&db_path).await {
                    println!("  [Scheduler] Erreur: {e}");
                }
            }
        });
    }

    /// Arrete la boucle de verification.
    pub async fn stop(&self) {
        let mut shutdown = self.shutdown.write().await;
        *shutdown = true;
    }
}

/// Verifie et execute les taches dues.
async fn check_due_tasks(db_path: &PathBuf) -> anyhow::Result<()> {
    let db = db_path.clone();
    tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
        let conn = rusqlite::Connection::open(&db)?;
        let now = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        // Taches one-shot dues
        let mut stmt = conn.prepare(
            "SELECT id, action, schedule FROM scheduled_tasks
             WHERE active = 1 AND task_type = 'once' AND next_run <= ?1",
        )?;
        let due_tasks: Vec<(String, String)> = stmt
            .query_map([&now], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .filter_map(|r| r.ok())
            .collect();

        for (id, action) in &due_tasks {
            println!("\n  [Scheduler] TACHE DUE: {action}");
            conn.execute(
                "UPDATE scheduled_tasks SET active = 0, last_run = ?1 WHERE id = ?2",
                rusqlite::params![now, id],
            )?;
        }

        // Taches cron : verifier si le cron match la minute actuelle
        let utc_now = chrono::Utc::now();
        let mut stmt = conn.prepare(
            "SELECT id, action, schedule FROM scheduled_tasks
             WHERE active = 1 AND task_type = 'cron'",
        )?;
        let cron_tasks: Vec<(String, String, String)> = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?
            .filter_map(|r| r.ok())
            .collect();

        for (id, action, cron_expr) in &cron_tasks {
            if cron_matches(cron_expr, &utc_now) {
                println!("\n  [Scheduler] CRON TACHE: {action}");
                conn.execute(
                    "UPDATE scheduled_tasks SET last_run = ?1 WHERE id = ?2",
                    rusqlite::params![now, id],
                )?;
            }
        }

        Ok(())
    })
    .await??;
    Ok(())
}

/// Parse une duree relative : "5m", "1h", "30s", "2d".
fn parse_duration(s: &str) -> anyhow::Result<chrono::Duration> {
    let s = s.trim();
    if s.len() < 2 {
        anyhow::bail!("Format invalide: '{s}'. Ex: 5m, 1h, 30s, 2d");
    }
    let (num_str, unit) = s.split_at(s.len() - 1);
    let num: i64 = num_str
        .parse()
        .map_err(|_| anyhow::anyhow!("Nombre invalide: '{num_str}'"))?;

    match unit {
        "s" => Ok(chrono::Duration::seconds(num)),
        "m" => Ok(chrono::Duration::minutes(num)),
        "h" => Ok(chrono::Duration::hours(num)),
        "d" => Ok(chrono::Duration::days(num)),
        _ => anyhow::bail!("Unite invalide: '{unit}'. Utilise s, m, h, d"),
    }
}

/// Valide une expression cron a 5 champs.
fn validate_cron_five_fields(s: &str) -> bool {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }
    parts.iter().all(|p| !p.is_empty() && p.len() <= 4)
}

/// Verifie si une expression cron (5 champs) match une date/heure.
/// Support MVP : `*` et valeurs litterales uniquement.
fn cron_matches(cron: &str, dt: &chrono::DateTime<chrono::Utc>) -> bool {
    use chrono::Datelike;
    use chrono::Timelike;

    let parts: Vec<&str> = cron.split_whitespace().collect();
    if parts.len() != 5 {
        return false;
    }

    let checks = [
        (parts[0], dt.minute() as i32),
        (parts[1], dt.hour() as i32),
        (parts[2], dt.day() as i32),
        (parts[3], dt.month() as i32),
        (parts[4], dt.weekday().num_days_from_sunday() as i32),
    ];

    for (field, value) in &checks {
        if *field == "*" {
            continue;
        }
        // Support */N
        if let Some(step) = field.strip_prefix("*/") {
            if let Ok(n) = step.parse::<i32>() {
                if n > 0 && value % n != 0 {
                    return false;
                }
                continue;
            }
        }
        // Valeur litterale
        if let Ok(expected) = field.parse::<i32>() {
            if *value != expected {
                return false;
            }
        }
    }

    true
}
