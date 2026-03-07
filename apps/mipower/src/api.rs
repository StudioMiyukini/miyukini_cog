use std::{path::PathBuf, sync::Arc};
use axum::{
    Router,
    extract::{Query, State, Json},
    http::StatusCode,
    routing::{get, post},
    response::{IntoResponse, sse::{Event, KeepAlive, Sse}},
};
use serde::Deserialize;
use crate::{AppState, models::{SequenceMeta, ArtefactContent, PromptBuilderInput, SequencesIndex}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/sequences",          get(sequences_handler))
        .route("/artefact",           get(artefact_handler))
        .route("/artefacts/{slug}",   get(artefacts_handler))
        .route("/progress/{slug}",    get(progress_handler))
        .route("/prompt",             post(prompt_handler))
        .route("/init-sequence",      post(init_sequence_handler))
        .route("/settings",           post(settings_handler))
        .route("/health",             get(health_handler))
}

// ── /sse (SSE — Server-Sent Events) ──────────────────────

pub async fn sse_handler(
    State(state): State<Arc<AppState>>,
) -> Sse<impl futures::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.events.subscribe();
    let stream = futures::stream::unfold(rx, |mut rx| async move {
        loop {
            match rx.recv().await {
                Ok(slug) => return Some((Ok(Event::default().data(slug)), rx)),
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                Err(_) => return None,
            }
        }
    });
    Sse::new(stream).keep_alive(KeepAlive::default())
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
    if canonical_file.extension().is_none_or(|e| e != "md") {
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
            e.path().extension().is_some_and(|x| x == "md")
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

// ── /api/artefacts/:slug ──────────────────────────────────

async fn artefacts_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> axum::Json<serde_json::Value> {
    let mip_root = state.mip_root.lock().unwrap().clone();
    let mip_root_path = PathBuf::from(&mip_root);

    let Some(seq_dir) = find_seq_dir(&mip_root, &slug) else {
        return axum::Json(serde_json::json!({ "slug": slug, "files": [] }));
    };

    let mut files = Vec::new();
    walk_md(&seq_dir, &mip_root_path, &mut files);
    files.sort();

    axum::Json(serde_json::json!({
        "slug": slug,
        "path": seq_dir.strip_prefix(&mip_root_path)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|_| seq_dir.to_string_lossy().to_string()),
        "files": files
    }))
}

fn find_seq_dir(mip_root: &str, slug: &str) -> Option<PathBuf> {
    let base = find_sequences_dir(mip_root)?;
    std::fs::read_dir(&base).ok()?.find_map(|e| {
        let entry = e.ok()?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.ends_with(&format!("-{slug}")) || name == slug {
            Some(entry.path())
        } else {
            None
        }
    })
}

fn walk_md(current: &PathBuf, root: &PathBuf, files: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(current) else { return; };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if name != "ui" && name != "node_modules" && !name.starts_with('.') {
                walk_md(&path, root, files);
            }
        } else if path.extension().is_some_and(|e| e == "md") {
            if let Ok(rel) = path.strip_prefix(root) {
                files.push(rel.to_string_lossy().replace('\\', "/").to_string());
            }
        }
    }
}

// ── /api/prompt (POST) ────────────────────────────────────

const VALID_TASK_CLASSES:   &[&str] = &["T1", "T2", "T3", "T4", "T5"];
const VALID_DOMAINS:        &[&str] = &["back", "front", "fullstack", "infra", "ai-ml", "securite", "data", "autre"];
const VALID_AUTONOMY_MODES: &[&str] = &["FULL", "BIG_STEPS", "GUIDED"];
const VALID_AGENTS:         &[&str] = &["Maria", "Denis", "Lise", "Victor", "Hugo", "Fabrice", "George", "Jean", "Arianne", "Francois"];

async fn prompt_handler(
    Json(input): Json<PromptBuilderInput>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // Validations longueur
    if input.title.is_empty() || input.title.len() > 200 {
        return Err(ApiError::bad_request("title : 1-200 caracteres requis"));
    }
    if input.description.len() > 2000 {
        return Err(ApiError::bad_request("description : max 2000 caracteres"));
    }
    if input.constraints.as_deref().is_some_and(|c| c.len() > 500) {
        return Err(ApiError::bad_request("constraints : max 500 caracteres"));
    }
    if input.stack.as_deref().is_some_and(|s| s.len() > 200) {
        return Err(ApiError::bad_request("stack : max 200 caracteres"));
    }

    // Validations whitelist
    if !VALID_TASK_CLASSES.contains(&input.task_class.as_str()) {
        return Err(ApiError::bad_request("task_class invalide (T1..T5)"));
    }
    if !VALID_DOMAINS.contains(&input.domain.as_str()) {
        return Err(ApiError::bad_request("domain invalide"));
    }
    if let Some(mode) = &input.autonomy_mode {
        if !VALID_AUTONOMY_MODES.contains(&mode.as_str()) {
            return Err(ApiError::bad_request("autonomy_mode invalide (FULL|BIG_STEPS|GUIDED)"));
        }
    }
    if input.agents.len() > 10 || input.agents.iter().any(|a| !VALID_AGENTS.contains(&a.as_str())) {
        return Err(ApiError::bad_request("agents invalides (max 10, whitelist MIP uniquement)"));
    }
    if input.tags.len() > 10 || input.tags.iter().any(|t| t.len() > 50) {
        return Err(ApiError::bad_request("tags invalides (max 10, max 50c chacun)"));
    }

    // Construction du prompt
    let constraints = input.constraints.as_deref().unwrap_or("Aucune contrainte specifique");
    let stack       = input.stack.as_deref().unwrap_or("A definir en P0");

    let mut lines = vec![
        format!("Lance une sequence MIP pour : {}", input.title),
        String::new(),
        format!("Classe estimee : {}", input.task_class),
        format!("Domaine : {}", input.domain),
        format!("Stack : {stack}"),
        format!("Contraintes : {constraints}"),
    ];

    if let Some(mode) = &input.autonomy_mode {
        lines.push(format!("Mode autonomie : {mode}"));
    }
    if input.urgency        { lines.push("Urgence : Oui".into()); }
    if input.sensitive_data { lines.push("Donnees sensibles : Oui".into()); }
    if input.msw_toggle     { lines.push("Mode Sans Web : Oui".into()); }
    if !input.agents.is_empty() {
        lines.push(format!("Agents actifs : {}", input.agents.join(", ")));
    }
    if !input.tags.is_empty() {
        lines.push(format!("Tags : {}", input.tags.join(", ")));
    }

    lines.push(String::new());
    lines.push(format!("Description :\n{}", input.description));
    lines.push(String::new());
    lines.push("---".into());
    lines.push("Maria, classe cette demande et lance P0.".into());

    let prompt = lines.join("\n");
    Ok(axum::Json(serde_json::json!({ "prompt": prompt })))
}

// ── /api/init-sequence (POST) ────────────────────────────

#[derive(Deserialize)]
pub struct InitSequenceInput {
    pub slug:       String,
    pub complexity: String,
    pub date:       Option<String>,
}

async fn init_sequence_handler(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InitSequenceInput>,
) -> Result<axum::Json<serde_json::Value>, ApiError> {
    // Validate slug: only [a-zA-Z0-9-], non-empty, no path separators
    if input.slug.is_empty()
        || !input.slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        || input.slug.contains("..") {
        return Err(ApiError::bad_request("Slug invalide (a-z0-9- uniquement)"));
    }

    const VALID_COMPLEXITIES: &[&str] = &["C1", "C2", "C3", "C4", "C5"];
    if !VALID_COMPLEXITIES.contains(&input.complexity.as_str()) {
        return Err(ApiError::bad_request("Complexité invalide (C1..C5 attendu)"));
    }

    let mip_root = state.mip_root.lock().unwrap().clone();
    let date = input.date.as_deref().unwrap_or("2026-03-07");

    let folder_name = format!("{date}-{}", input.slug);
    let seq_path    = PathBuf::from(&mip_root).join(".mip").join("sequences").join(&folder_name);

    std::fs::create_dir_all(&seq_path)
        .map_err(|e| ApiError::internal(format!("Création dossier : {e}")))?;

    // Locate the init script
    let script_path = PathBuf::from(&mip_root)
        .join(".mip").join("scripts").join("init-sequence-by-complexity.ps1");
    if !script_path.exists() {
        return Ok(axum::Json(serde_json::json!({
            "ok": true,
            "path": seq_path.to_string_lossy(),
            "message": format!("Dossier créé. Script PS1 introuvable à : {}", script_path.display())
        })));
    }

    let output = std::process::Command::new("powershell")
        .args([
            "-NonInteractive",
            "-ExecutionPolicy", "Bypass",
            "-File", &script_path.to_string_lossy(),
            "-SequencePath", &seq_path.to_string_lossy(),
            "-Complexity", &input.complexity,
        ])
        .output()
        .map_err(|e| ApiError::internal(format!("Lancement PowerShell : {e}")))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(axum::Json(serde_json::json!({
            "ok": true,
            "path": seq_path.to_string_lossy(),
            "message": if stdout.is_empty() { "Séquence initialisée.".to_string() } else { stdout }
        })))
    } else {
        Err(ApiError::internal(format!("Script échoué : {stderr}")))
    }
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
        let (events_tx, _) = tokio::sync::broadcast::channel::<String>(8);
        Arc::new(AppState {
            db:       std::sync::Mutex::new(conn),
            mip_root: std::sync::Mutex::new(root.to_string()),
            events:   events_tx,
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
    fn test_generate_prompt_non_empty() {
        use crate::models::PromptBuilderInput;
        let input = PromptBuilderInput {
            title:          "test-sequence".into(),
            task_class:     "T5".into(),
            domain:         "fullstack".into(),
            description:    "Une description de test.".into(),
            constraints:    None,
            stack:          None,
            autonomy_mode:  None,
            agents:         vec![],
            tags:           vec![],
            urgency:        false,
            sensitive_data: false,
            msw_toggle:     false,
        };
        let stack       = input.stack.as_deref().unwrap_or("A definir en P0");
        let constraints = input.constraints.as_deref().unwrap_or("Aucune contrainte specifique");
        let prompt = format!(
            "Lance une sequence MIP pour : {}\n\nClasse estimee : {}\nDomaine : {}\nStack : {stack}\nContraintes : {constraints}\n\nDescription :\n{}\n\n---\nMaria, classe cette demande et lance P0.",
            input.title, input.task_class, input.domain, input.description,
        );
        assert!(!prompt.is_empty());
        assert!(prompt.contains("test-sequence"));
        assert!(prompt.contains("T5"));
    }

    #[test]
    fn test_generate_prompt_with_agents() {
        use crate::models::PromptBuilderInput;
        let input = PromptBuilderInput {
            title:          "agents-test".into(),
            task_class:     "T4".into(),
            domain:         "back".into(),
            description:    "Test agents.".into(),
            constraints:    None,
            stack:          None,
            autonomy_mode:  None,
            agents:         vec!["Maria".into(), "Victor".into()],
            tags:           vec![],
            urgency:        false,
            sensitive_data: false,
            msw_toggle:     false,
        };
        assert!(VALID_AGENTS.contains(&"Maria"));
        assert!(VALID_AGENTS.contains(&"Victor"));
        let agents_line = format!("Agents actifs : {}", input.agents.join(", "));
        assert!(agents_line.contains("Maria"));
        assert!(agents_line.contains("Victor"));
        assert!(!input.agents.iter().any(|a| !VALID_AGENTS.contains(&a.as_str())));
    }

    #[test]
    fn test_generate_prompt_with_autonomy_mode() {
        use crate::models::PromptBuilderInput;
        let input = PromptBuilderInput {
            title:          "autonomy-test".into(),
            task_class:     "T3".into(),
            domain:         "fullstack".into(),
            description:    "Test autonomy mode.".into(),
            constraints:    None,
            stack:          None,
            autonomy_mode:  Some("FULL".into()),
            agents:         vec![],
            tags:           vec!["test".into()],
            urgency:        true,
            sensitive_data: true,
            msw_toggle:     false,
        };
        assert!(VALID_AUTONOMY_MODES.contains(&input.autonomy_mode.as_deref().unwrap()));
        assert!(input.urgency);
        assert!(input.sensitive_data);
        assert_eq!(input.tags, vec!["test"]);
    }

    #[test]
    fn test_init_sequence_slug_validation() {
        // Valid slugs
        assert!("mipower-v2".chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
        // Invalid: path separator
        let bad = "../../../etc/passwd";
        assert!(bad.contains("..") || bad.contains('/'));
        // Invalid: spaces
        let with_space = "my slug";
        assert!(!with_space.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
    }

    /// Smoke test RED — compile uniquement apres E01 (champs manquants dans PromptBuilderInput)
    #[test]
    fn test_smoke_prompt_builder_v2_structure() {
        use crate::models::PromptBuilderInput;
        let _input = PromptBuilderInput {
            title:          "smoke".into(),
            task_class:     "T4".into(),
            domain:         "fullstack".into(),
            description:    "smoke test v2".into(),
            constraints:    None,
            stack:          None,
            autonomy_mode:  Some("FULL".into()),
            agents:         vec!["Maria".into(), "Denis".into()],
            tags:           vec!["ui".into()],
            urgency:        true,
            sensitive_data: false,
            msw_toggle:     false,
        };
        assert_eq!(_input.task_class, "T4");
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
