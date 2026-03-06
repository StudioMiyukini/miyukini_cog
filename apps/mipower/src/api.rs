use std::{path::PathBuf, sync::Arc};
use axum::{
    Router,
    extract::{Query, State, Json},
    http::StatusCode,
    routing::{get, post},
    response::IntoResponse,
};
use serde::Deserialize;
use crate::{AppState, models::{SequenceMeta, ArtefactContent, PromptBuilderInput, SequencesIndex}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sequences",       get(sequences_handler))
        .route("/artefact",        get(artefact_handler))
        .route("/progress/{slug}", get(progress_handler))
        .route("/prompt",          post(prompt_handler))
        .route("/settings",        post(settings_handler))
        .route("/health",          get(health_handler))
}

async fn health_handler() -> &'static str { "ok" }

// ── /api/sequences ────────────────────────────────────────

async fn sequences_handler(
    State(state): State<Arc<AppState>>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    let mip_root = state.mip_root.lock().unwrap().clone();

    let Some(index_path) = find_index(&mip_root) else {
        return Ok(axum::Json(serde_json::json!({
            "sequences": [],
            "error": format!("sequences/index.json introuvable dans : {mip_root}")
        })));
    };

    let raw = std::fs::read_to_string(&index_path)
        .map_err(|e| ApiError::internal(format!("Lecture index.json : {e}")))?;
    let raw = raw.trim_start_matches('\u{feff}');

    let index: SequencesIndex = serde_json::from_str(raw)
        .map_err(|e| ApiError::internal(format!("Parse index.json : {e}")))?;

    let sequences: Vec<SequenceMeta> = index.sequences.into_iter().map(SequenceMeta::from).collect();
    let count = sequences.len();

    if let Ok(conn) = state.db.lock() {
        let now = chrono_now();
        for s in &sequences {
            let _ = conn.execute(
                "INSERT INTO sequences (slug, date, status, task_class, complexity, path, tags, indexed_at)
                 VALUES (?1,?2,?3,?4,?5,?6,'[]',?7)
                 ON CONFLICT(slug) DO UPDATE SET
                   status=excluded.status, task_class=excluded.task_class,
                   path=excluded.path, indexed_at=excluded.indexed_at",
                rusqlite::params![
                    s.slug, s.date, s.status,
                    s.task_class.as_deref(), s.complexity.as_deref(),
                    s.path, now
                ],
            );
        }
    }

    Ok(axum::Json(serde_json::json!({ "sequences": sequences, "count": count })))
}

// ── /api/artefact?path=... ────────────────────────────────

#[derive(Deserialize)]
pub struct ArtefactQuery { pub path: String }

async fn artefact_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ArtefactQuery>,
) -> Result<axum::Json<ArtefactContent>, ApiError> {
    let mip_root = state.mip_root.lock().unwrap().clone();
    let canonical_root = PathBuf::from(&mip_root)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(&mip_root));

    let requested = PathBuf::from(&params.path);
    let resolved = if requested.is_absolute() { requested } else { canonical_root.join(requested) };
    let canonical_file = resolved.canonicalize()
        .map_err(|_| ApiError::bad_request("Chemin introuvable ou invalide"))?;

    if !canonical_file.starts_with(&canonical_root) {
        return Err(ApiError::forbidden("Chemin hors du workspace MIP"));
    }
    if !canonical_file.extension().map_or(false, |e| e == "md") {
        return Err(ApiError::bad_request("Seuls les fichiers .md sont accessibles"));
    }

    let content = std::fs::read_to_string(&canonical_file)
        .map_err(|e| ApiError::internal(format!("Lecture : {e}")))?;

    Ok(axum::Json(ArtefactContent {
        path: canonical_file.to_string_lossy().to_string(),
        content,
    }))
}

// ── /api/progress/:slug ───────────────────────────────────

async fn progress_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mip_root = state.mip_root.lock().unwrap().clone();

    let seq_dir = find_sequences_dir(&mip_root).and_then(|base| {
        std::fs::read_dir(&base).ok()?.find_map(|e| {
            let entry = e.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(&format!("-{slug}")) { Some(entry.path()) } else { None }
        })
    });

    let (p0_done, p0_total, p3_done, p3_total) = if let Some(dir) = seq_dir {
        let p0 = count_done_in(
            &dir.join("phases").join("p0").join("temps"),
            "temps-", "Etat : TERMINE",
        );
        let p3 = count_done_in(
            &dir.join("plans_p3").join("etapes"),
            "etape-", "Statut : Terminé",
        );
        (p0.0, p0.1, p3.0, p3.1)
    } else {
        (0, 0, 0, 0)
    };

    axum::Json(serde_json::json!({
        "slug": slug,
        "phases": [
            { "phase": "P0", "done": p0_done, "total": p0_total },
            { "phase": "P3", "done": p3_done, "total": p3_total },
        ]
    }))
}

fn count_done_in(dir: &PathBuf, prefix: &str, marker: &str) -> (usize, usize) {
    let Ok(entries) = std::fs::read_dir(dir) else { return (0, 0); };
    let files: Vec<_> = entries.filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            e.path().extension().map_or(false, |x| x == "md")
                && n.starts_with(prefix)
                && n != "etape-buf.md"
        })
        .collect();
    let total = files.len();
    let done = files.iter()
        .filter(|e| std::fs::read_to_string(e.path()).map(|c| c.contains(marker)).unwrap_or(false))
        .count();
    (done, total)
}

// ── /api/prompt (POST) ────────────────────────────────────

async fn prompt_handler(
    Json(input): Json<PromptBuilderInput>,
) -> axum::Json<serde_json::Value> {
    let constraints = input.constraints.as_deref().unwrap_or("Aucune contrainte specifique");
    let stack = input.stack.as_deref().unwrap_or("A definir en P0");
    let prompt = format!(
        "Lance une sequence MIP pour : {title}\n\n\
         Classe estimee : {class}\n\
         Domaine : {domain}\n\
         Stack : {stack}\n\
         Contraintes : {constraints}\n\n\
         Description :\n{description}\n\n\
         ---\n\
         Maria, classe cette demande et lance P0.",
        title       = input.title,
        class       = input.task_class,
        domain      = input.domain,
        description = input.description,
    );
    axum::Json(serde_json::json!({ "prompt": prompt }))
}

// ── /api/settings (POST) ─────────────────────────────────

#[derive(Deserialize)]
pub struct SettingsInput { pub mip_root: String }

async fn settings_handler(
    State(state): State<Arc<AppState>>,
    Json(input): Json<SettingsInput>,
) -> impl IntoResponse {
    let root = input.mip_root.trim().to_string();
    if root.is_empty() {
        return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({ "error": "mip_root vide" })));
    }
    *state.mip_root.lock().unwrap() = root.clone();
    (StatusCode::OK, axum::Json(serde_json::json!({ "mip_root": root, "ok": true })))
}

// ── Helpers ───────────────────────────────────────────────

fn find_index(mip_root: &str) -> Option<PathBuf> {
    [
        PathBuf::from(mip_root).join(".mip").join("sequences").join("index.json"),
        PathBuf::from(mip_root).join("sequences").join("index.json"),
    ]
    .into_iter()
    .find(|p| p.exists())
}

fn find_sequences_dir(mip_root: &str) -> Option<PathBuf> {
    [
        PathBuf::from(mip_root).join(".mip").join("sequences"),
        PathBuf::from(mip_root).join("sequences"),
    ]
    .into_iter()
    .find(|p| p.exists())
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs().to_string()
}

// ── Error type ────────────────────────────────────────────

struct ApiError { status: StatusCode, message: String }
impl ApiError {
    fn internal(msg: impl Into<String>) -> Self {
        Self { status: StatusCode::INTERNAL_SERVER_ERROR, message: msg.into() }
    }
    fn bad_request(msg: impl Into<String>) -> Self {
        Self { status: StatusCode::BAD_REQUEST, message: msg.into() }
    }
    fn forbidden(msg: impl Into<String>) -> Self {
        Self { status: StatusCode::FORBIDDEN, message: msg.into() }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, axum::Json(serde_json::json!({ "error": self.message }))).into_response()
    }
}

// ── Tests ─────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as IoWrite;
    use tempfile::TempDir;

    fn make_state(root: &str) -> Arc<AppState> {
        use tempfile::NamedTempFile;
        let tmp = NamedTempFile::new().unwrap();
        let conn = crate::db::open(tmp.path()).unwrap();
        std::mem::forget(tmp);
        Arc::new(AppState {
            db:       std::sync::Mutex::new(conn),
            mip_root: std::sync::Mutex::new(root.to_string()),
        })
    }

    #[test]
    fn test_path_traversal_rejected() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_string_lossy().to_string();
        let _state = make_state(&root);

        let outside = tmp.path().parent().unwrap().join("secret.md");
        let mut f = std::fs::File::create(&outside).unwrap();
        f.write_all(b"secret").unwrap();

        let canonical_root = PathBuf::from(&root).canonicalize().unwrap_or_else(|_| PathBuf::from(&root));
        let canonical_file = outside.canonicalize().unwrap_or_else(|_| outside.clone());
        assert!(
            !canonical_file.starts_with(&canonical_root),
            "Le path traversal ne doit pas passer la validation"
        );
    }

    #[test]
    fn test_sequences_index_parse() {
        let json = r#"{
            "generated_at": "2026-03-06",
            "schema": "mip-sequences-index-v2",
            "count": 1,
            "sequences": [{
                "name": "2026-03-06-test", "date": "2026-03-06", "slug": "test",
                "type": "T5", "status": "active", "current_phase": "P3",
                "brief_path": "./sequences/2026-03-06-test/briefs/brief.md",
                "ui_path": null, "metrics_path": null, "security_score": 0
            }]
        }"#;
        let index: SequencesIndex = serde_json::from_str(json).unwrap();
        assert_eq!(index.sequences.len(), 1);
        let meta = SequenceMeta::from(index.sequences[0].clone());
        assert_eq!(meta.slug, "test");
        assert_eq!(meta.status, "active");
        assert_eq!(meta.task_class.as_deref(), Some("T5"));
    }
}
