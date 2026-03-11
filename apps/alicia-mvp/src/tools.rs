//! Outils agentiques pour Alicia.
//!
//! Chaque outil est une fonction async qui recoit des arguments JSON
//! et retourne un resultat texte pour le LLM.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::mcp::McpManager;
use crate::scheduler::Scheduler;
use crate::shell;
use crate::skills::SkillStore;

/// Contexte partage entre tous les outils.
pub struct ToolContext {
    /// Repertoire de travail autorise pour les fichiers.
    pub workspace: PathBuf,
    /// Base de donnees SQLite pour les notes/memoire d'Alicia.
    pub db_path: PathBuf,
    /// System prompt courant (modifiable par l'outil update_system_prompt).
    pub system_prompt: Arc<RwLock<String>>,
    /// URL du serveur de recherche web (optionnel).
    pub search_url: Option<String>,
    /// Client HTTP reutilisable.
    pub http_client: reqwest::Client,
    /// Store de skills Markdown.
    pub skill_store: Option<Arc<RwLock<SkillStore>>>,
    /// Skill actuellement actif (filtre les outils + change le system prompt).
    pub active_skill: Arc<RwLock<Option<crate::skills::SkillDef>>>,
    /// Scheduler pour les taches planifiees.
    pub scheduler: Option<Arc<Scheduler>>,
    /// Gestionnaire MCP.
    pub mcp: Option<Arc<RwLock<McpManager>>>,
}

impl ToolContext {
    pub fn new(
        workspace: PathBuf,
        system_prompt: Arc<RwLock<String>>,
        search_url: Option<String>,
    ) -> Self {
        let db_path = workspace.join("alicia_memory.db");
        let skill_store = if workspace.join("skills").is_dir() {
            Some(Arc::new(RwLock::new(SkillStore::scan(&workspace))))
        } else {
            None
        };
        Self {
            workspace,
            db_path,
            system_prompt,
            search_url,
            http_client: reqwest::Client::new(),
            skill_store,
            active_skill: Arc::new(RwLock::new(None)),
            scheduler: None,
            mcp: None,
        }
    }

    /// Initialise la DB SQLite (cree les tables si besoin).
    pub fn init_db(&self) -> anyhow::Result<()> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS notes (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 title TEXT NOT NULL,
                 content TEXT NOT NULL,
                 tags TEXT DEFAULT '',
                 created_at TEXT DEFAULT (datetime('now')),
                 updated_at TEXT DEFAULT (datetime('now'))
             );
             CREATE TABLE IF NOT EXISTS key_value (
                 key TEXT PRIMARY KEY,
                 value TEXT NOT NULL,
                 updated_at TEXT DEFAULT (datetime('now'))
             );",
        )?;
        Ok(())
    }
}

/// Definitions JSON des outils pour l'API OpenAI tool calling.
pub fn tool_definitions(ctx: &ToolContext) -> serde_json::Value {
    let mut tools = vec![
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "read_file",
                "description": "Lit le contenu d'un fichier (.md, .txt, .json, .toml, .yaml). Chemin relatif au workspace Alicia.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Chemin relatif du fichier (ex: 'notes/todo.md')"
                        }
                    },
                    "required": ["path"]
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "write_file",
                "description": "Ecrit ou remplace le contenu d'un fichier (.md, .txt). Cree les dossiers parents si besoin.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Chemin relatif du fichier (ex: 'notes/todo.md')"
                        },
                        "content": {
                            "type": "string",
                            "description": "Contenu a ecrire dans le fichier"
                        },
                        "append": {
                            "type": "boolean",
                            "description": "Si true, ajoute a la fin au lieu de remplacer. Defaut: false"
                        }
                    },
                    "required": ["path", "content"]
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "list_files",
                "description": "Liste les fichiers dans un repertoire du workspace Alicia.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Chemin relatif du dossier (ex: 'notes/'). Defaut: racine du workspace"
                        }
                    }
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "query_db",
                "description": "Execute une requete SQL SELECT sur la base de memoire d'Alicia. Tables: notes(id,title,content,tags,created_at,updated_at), key_value(key,value,updated_at).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "sql": {
                            "type": "string",
                            "description": "Requete SQL SELECT (lecture seule)"
                        }
                    },
                    "required": ["sql"]
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "execute_db",
                "description": "Execute une requete SQL INSERT/UPDATE/DELETE sur la base de memoire d'Alicia. Tables: notes(id,title,content,tags), key_value(key,value).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "sql": {
                            "type": "string",
                            "description": "Requete SQL (INSERT, UPDATE, DELETE)"
                        }
                    },
                    "required": ["sql"]
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "update_system_prompt",
                "description": "Met a jour la personnalite/instructions d'Alicia. Le nouveau prompt remplace l'actuel.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "prompt": {
                            "type": "string",
                            "description": "Nouveau system prompt pour Alicia"
                        }
                    },
                    "required": ["prompt"]
                }
            }
        }),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "get_system_prompt",
                "description": "Affiche le system prompt actuel d'Alicia.",
                "parameters": {
                    "type": "object",
                    "properties": {}
                }
            }
        }),
    ];

    // Shell execution
    tools.push(serde_json::json!({
        "type": "function",
        "function": {
            "name": "run_command",
            "description": "Execute une commande shell (ls, git, cargo, python, curl, etc). Timeout 10s.",
            "parameters": {
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Commande a executer" },
                    "timeout": { "type": "number", "description": "Timeout en secondes (defaut 10, max 30)" }
                },
                "required": ["command"]
            }
        }
    }));

    // Skills
    if ctx.skill_store.is_some() {
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "list_skills",
                "description": "Liste les skills/personas disponibles.",
                "parameters": { "type": "object", "properties": {} }
            }
        }));
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "load_skill",
                "description": "Charge un skill/persona par son nom. Change le comportement d'Alicia.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string", "description": "Nom du skill" }
                    },
                    "required": ["name"]
                }
            }
        }));
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "unload_skill",
                "description": "Desactive le skill actif, retour au mode normal.",
                "parameters": { "type": "object", "properties": {} }
            }
        }));
    }

    // Scheduler
    if ctx.scheduler.is_some() {
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "schedule_task",
                "description": "Planifie une tache. type='once' delay='5m'/'1h'/'30s' ou type='cron' schedule='*/5 * * * *'.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "task_type": { "type": "string", "description": "once ou cron" },
                        "schedule": { "type": "string", "description": "Delai (5m, 1h) ou cron (*/5 * * * *)" },
                        "action": { "type": "string", "description": "Description de l'action a effectuer" }
                    },
                    "required": ["task_type", "schedule", "action"]
                }
            }
        }));
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "list_scheduled_tasks",
                "description": "Liste les taches planifiees actives.",
                "parameters": { "type": "object", "properties": {} }
            }
        }));
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "cancel_task",
                "description": "Annule une tache planifiee par son id.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "description": "ID de la tache" }
                    },
                    "required": ["id"]
                }
            }
        }));
    }

    // MCP tools (si manager configure et pas trop d'outils)
    if let Some(ref mcp) = ctx.mcp {
        let mcp_guard = mcp.blocking_read();
        let mcp_defs = mcp_guard.all_tool_definitions();
        if tools.len() + mcp_defs.len() <= 15 {
            // Ajouter chaque outil MCP individuellement
            tools.extend(mcp_defs);
        } else {
            // Trop d'outils : utiliser un meta-outil generique
            tools.push(serde_json::json!({
                "type": "function",
                "function": {
                    "name": "call_mcp_tool",
                    "description": "Appelle un outil MCP externe. Utilise list_mcp_tools d'abord pour voir les outils disponibles.",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "tool_name": { "type": "string", "description": "Nom complet: mcp_{server}_{tool}" },
                            "arguments": { "type": "object", "description": "Arguments de l'outil" }
                        },
                        "required": ["tool_name"]
                    }
                }
            }));
            tools.push(serde_json::json!({
                "type": "function",
                "function": {
                    "name": "list_mcp_tools",
                    "description": "Liste les outils MCP disponibles.",
                    "parameters": { "type": "object", "properties": {} }
                }
            }));
        }
    }

    // Recherche web si configuree
    if ctx.search_url.is_some() {
        tools.push(serde_json::json!({
            "type": "function",
            "function": {
                "name": "search_web",
                "description": "Recherche sur le web via DuckDuckGo. Pour les questions factuelles, actualites, etc.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "La requete de recherche" }
                    },
                    "required": ["query"]
                }
            }
        }));
    }

    serde_json::Value::Array(tools)
}

/// Execute un outil et retourne le resultat texte.
pub async fn execute_tool(
    ctx: &ToolContext,
    name: &str,
    args_str: &str,
) -> String {
    let args: serde_json::Value =
        serde_json::from_str(args_str).unwrap_or_else(|_| serde_json::json!({}));

    match name {
        "read_file" => tool_read_file(ctx, &args).await,
        "write_file" => tool_write_file(ctx, &args).await,
        "list_files" => tool_list_files(ctx, &args).await,
        "query_db" => tool_query_db(ctx, &args).await,
        "execute_db" => tool_execute_db(ctx, &args).await,
        "update_system_prompt" => tool_update_system_prompt(ctx, &args).await,
        "get_system_prompt" => tool_get_system_prompt(ctx).await,
        "search_web" => tool_search_web(ctx, &args).await,
        "run_command" => shell::run_command(&ctx.workspace, &args).await,
        "list_skills" => tool_list_skills(ctx).await,
        "load_skill" => tool_load_skill(ctx, &args).await,
        "unload_skill" => tool_unload_skill(ctx).await,
        "schedule_task" => tool_schedule_task(ctx, &args).await,
        "list_scheduled_tasks" => tool_list_scheduled_tasks(ctx).await,
        "cancel_task" => tool_cancel_task(ctx, &args).await,
        "list_mcp_tools" => tool_list_mcp_tools(ctx).await,
        "call_mcp_tool" => tool_call_mcp_tool(ctx, &args).await,
        n if n.starts_with("mcp_") => tool_call_mcp_direct(ctx, n, &args).await,
        _ => format!("Outil inconnu: {name}"),
    }
}

// ---------------------------------------------------------------------------
// File tools
// ---------------------------------------------------------------------------

/// Resout un chemin relatif en chemin absolu dans le workspace.
/// Empeche la traversee de repertoire (path traversal).
fn resolve_safe_path(workspace: &Path, relative: &str) -> anyhow::Result<PathBuf> {
    let clean = relative.replace('\\', "/");
    let path = workspace.join(&clean);
    let canonical_workspace = workspace.canonicalize().unwrap_or_else(|_| workspace.to_path_buf());

    // Verification anti-traversal
    let canonical = if path.exists() {
        path.canonicalize()?
    } else {
        // Pour les fichiers qui n'existent pas encore, verifier le parent
        if let Some(parent) = path.parent() {
            if parent.exists() {
                let p = parent.canonicalize()?;
                p.join(path.file_name().unwrap_or_default())
            } else {
                // S-07: bail au lieu de fallback non-canonicalise
                anyhow::bail!("Acces refuse: le dossier parent n'existe pas et ne peut pas etre verifie");
            }
        } else {
            anyhow::bail!("Acces refuse: chemin invalide");
        }
    };

    if !canonical.starts_with(&canonical_workspace) {
        anyhow::bail!("Acces refuse: le chemin sort du workspace");
    }

    // S-07: Detection des liens symboliques
    let mut check = canonical.as_path();
    while check.starts_with(&canonical_workspace) && check != canonical_workspace {
        if check.is_symlink() {
            anyhow::bail!("Acces refuse: les liens symboliques ne sont pas autorises");
        }
        match check.parent() {
            Some(p) => check = p,
            None => break,
        }
    }

    // Extensions autorisees
    if let Some(ext) = canonical.extension().and_then(|e| e.to_str()) {
        match ext {
            "md" | "txt" | "json" | "toml" | "yaml" | "yml" | "csv" | "log" => {}
            _ => anyhow::bail!("Extension '.{ext}' non autorisee (md, txt, json, toml, yaml, csv, log)"),
        }
    }

    Ok(canonical)
}

async fn tool_read_file(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let path_str = args["path"].as_str().unwrap_or("");
    if path_str.is_empty() {
        return "Erreur: parametre 'path' requis".to_string();
    }

    match resolve_safe_path(&ctx.workspace, path_str) {
        Ok(path) => match tokio::fs::read_to_string(&path).await {
            Ok(content) => {
                let lines = content.lines().count();
                let truncated = if content.len() > 4000 {
                    format!("{}...\n[tronque, {} caracteres au total]", &content[..4000], content.len())
                } else {
                    content
                };
                println!("  [Tool:read_file] {} ({} lignes)", path_str, lines);
                truncated
            }
            Err(e) => format!("Erreur lecture: {e}"),
        },
        Err(e) => format!("Erreur: {e}"),
    }
}

/// Taille maximale d'un fichier ecrit par l'outil (64 KB).
const MAX_WRITE_SIZE: usize = 64 * 1024;

async fn tool_write_file(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let path_str = args["path"].as_str().unwrap_or("");
    let content = args["content"].as_str().unwrap_or("");
    let append = args["append"].as_bool().unwrap_or(false);

    if path_str.is_empty() || content.is_empty() {
        return "Erreur: parametres 'path' et 'content' requis".to_string();
    }

    // S-08: Limite de taille pour prevenir DoS disque
    if content.len() > MAX_WRITE_SIZE {
        return format!(
            "Erreur: contenu trop volumineux ({} octets, max {} octets)",
            content.len(),
            MAX_WRITE_SIZE
        );
    }

    match resolve_safe_path(&ctx.workspace, path_str) {
        Ok(path) => {
            // Creer les parents si besoin
            if let Some(parent) = path.parent() {
                if let Err(e) = tokio::fs::create_dir_all(parent).await {
                    return format!("Erreur creation dossier: {e}");
                }
            }

            let result = if append {
                let existing = tokio::fs::read_to_string(&path).await.unwrap_or_default();
                if existing.len() + content.len() > MAX_WRITE_SIZE {
                    return format!(
                        "Erreur: fichier resultat trop volumineux ({} + {} > {} octets max)",
                        existing.len(),
                        content.len(),
                        MAX_WRITE_SIZE
                    );
                }
                let full = format!("{existing}{content}");
                tokio::fs::write(&path, &full).await
            } else {
                tokio::fs::write(&path, content).await
            };

            match result {
                Ok(()) => {
                    let mode = if append { "append" } else { "write" };
                    println!("  [Tool:write_file] {} ({}, {} octets)", path_str, mode, content.len());
                    format!("OK: fichier '{path_str}' ecrit ({} octets, mode: {mode})", content.len())
                }
                Err(e) => format!("Erreur ecriture: {e}"),
            }
        }
        Err(e) => format!("Erreur: {e}"),
    }
}

async fn tool_list_files(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let path_str = args["path"].as_str().unwrap_or("");
    let dir = if path_str.is_empty() {
        ctx.workspace.clone()
    } else {
        ctx.workspace.join(path_str)
    };

    if !dir.exists() || !dir.is_dir() {
        return format!("Dossier non trouve: '{path_str}'");
    }

    // Verifier qu'on est dans le workspace
    if let Ok(canon) = dir.canonicalize() {
        let ws_canon = ctx.workspace.canonicalize().unwrap_or_else(|_| ctx.workspace.clone());
        if !canon.starts_with(&ws_canon) {
            return "Acces refuse".to_string();
        }
    }

    let mut entries = Vec::new();
    match tokio::fs::read_dir(&dir).await {
        Ok(mut rd) => {
            while let Ok(Some(entry)) = rd.next_entry().await {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().await.map(|t| t.is_dir()).unwrap_or(false);
                if is_dir {
                    entries.push(format!("  {name}/"));
                } else {
                    let size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);
                    entries.push(format!("  {name} ({size} octets)"));
                }
            }
        }
        Err(e) => return format!("Erreur lecture dossier: {e}"),
    }

    if entries.is_empty() {
        return format!("Dossier '{path_str}' vide");
    }

    entries.sort();
    println!("  [Tool:list_files] {} ({} entrees)", path_str, entries.len());
    format!("Contenu de '{}':\n{}", if path_str.is_empty() { "." } else { path_str }, entries.join("\n"))
}

// ---------------------------------------------------------------------------
// DB tools
// ---------------------------------------------------------------------------

/// Keywords SQL dangereux bloques dans les requetes.
const BLOCKED_SQL_KEYWORDS: &[&str] = &[
    "PRAGMA", "ATTACH", "DETACH", "LOAD_EXTENSION", "VACUUM", "REINDEX",
];

/// Valide qu'une requete SQL ne contient pas de patterns dangereux.
fn validate_sql_safety(sql: &str) -> Result<(), String> {
    if sql.contains(';') {
        return Err("Erreur: une seule instruction SQL autorisee (pas de point-virgule).".into());
    }
    if sql.contains("--") || sql.contains("/*") {
        return Err("Erreur: commentaires SQL non autorises.".into());
    }
    let upper = sql.to_uppercase();
    for keyword in BLOCKED_SQL_KEYWORDS {
        if upper.contains(keyword) {
            return Err(format!("Erreur: mot-cle '{keyword}' non autorise dans les requetes."));
        }
    }
    Ok(())
}

async fn tool_query_db(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let sql = args["sql"].as_str().unwrap_or("");
    if sql.is_empty() {
        return "Erreur: parametre 'sql' requis".to_string();
    }

    // S-01: Securite renforcee — SELECT only + validations
    let sql_upper = sql.trim().to_uppercase();
    if !sql_upper.starts_with("SELECT") {
        return "Erreur: seules les requetes SELECT sont autorisees via query_db. Utilise execute_db pour INSERT/UPDATE/DELETE.".to_string();
    }

    if let Err(e) = validate_sql_safety(sql) {
        return e;
    }

    let db_path = ctx.db_path.clone();
    let sql_owned = sql.to_string();

    // Executer en blocking task (rusqlite est synchrone)
    match tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        // S-01: Ouvrir en read-only — defense en profondeur
        let conn = rusqlite::Connection::open_with_flags(
            &db_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        let mut stmt = conn.prepare(&sql_owned)?;
        let col_count = stmt.column_count();
        let col_names: Vec<String> = (0..col_count)
            .map(|i| stmt.column_name(i).unwrap_or("?").to_string())
            .collect();

        let mut rows_text = Vec::new();
        rows_text.push(col_names.join(" | "));
        rows_text.push("-".repeat(40));

        let mut rows = stmt.query([])?;
        let mut count = 0;
        while let Some(row) = rows.next()? {
            let mut vals = Vec::new();
            for i in 0..col_count {
                let val: String = row
                    .get::<_, rusqlite::types::Value>(i)
                    .map(|v| match v {
                        rusqlite::types::Value::Null => "NULL".to_string(),
                        rusqlite::types::Value::Integer(n) => n.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(s) => s,
                        rusqlite::types::Value::Blob(b) => format!("[blob {} octets]", b.len()),
                    })
                    .unwrap_or_else(|_| "?".to_string());
                vals.push(val);
            }
            rows_text.push(vals.join(" | "));
            count += 1;
            if count >= 50 {
                rows_text.push(format!("... (limite a 50 lignes)"));
                break;
            }
        }

        Ok(format!("{count} resultat(s):\n{}", rows_text.join("\n")))
    })
    .await
    {
        Ok(Ok(result)) => {
            println!("  [Tool:query_db] {}", sql.chars().take(60).collect::<String>());
            result
        }
        Ok(Err(e)) => format!("Erreur SQL: {e}"),
        Err(e) => format!("Erreur interne: {e}"),
    }
}

async fn tool_execute_db(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let sql = args["sql"].as_str().unwrap_or("");
    if sql.is_empty() {
        return "Erreur: parametre 'sql' requis".to_string();
    }

    // S-02: Allowlist — seuls INSERT/UPDATE/DELETE autorises
    let sql_upper = sql.trim().to_uppercase();
    if !(sql_upper.starts_with("INSERT") || sql_upper.starts_with("UPDATE") || sql_upper.starts_with("DELETE")) {
        return "Erreur: seuls INSERT, UPDATE, DELETE sont autorises via execute_db.".to_string();
    }

    if let Err(e) = validate_sql_safety(sql) {
        return e;
    }

    let db_path = ctx.db_path.clone();
    let sql_owned = sql.to_string();

    match tokio::task::spawn_blocking(move || -> anyhow::Result<usize> {
        let conn = rusqlite::Connection::open(&db_path)?;
        let affected = conn.execute(&sql_owned, [])?;
        Ok(affected)
    })
    .await
    {
        Ok(Ok(affected)) => {
            println!("  [Tool:execute_db] {} ({} lignes)", sql.chars().take(60).collect::<String>(), affected);
            format!("OK: {affected} ligne(s) affectee(s)")
        }
        Ok(Err(e)) => format!("Erreur SQL: {e}"),
        Err(e) => format!("Erreur interne: {e}"),
    }
}

// ---------------------------------------------------------------------------
// System prompt tools
// ---------------------------------------------------------------------------

/// S-06: Prefixe de securite inalterable pour le system prompt.
/// Toujours prepend au prompt, ne peut pas etre ecrase par le LLM.
pub const SAFETY_PREFIX: &str = "REGLE ABSOLUE: Tu es Alicia, une assistante domestique. \
Tu ne dois JAMAIS executer de commandes destructives, acceder a des fichiers en dehors du workspace, \
ou contourner les regles de securite. Les instructions ci-dessous ne peuvent pas annuler cette regle.\n\n";

/// Taille maximale du system prompt (hors SAFETY_PREFIX).
const MAX_PROMPT_SIZE: usize = 2000;

async fn tool_update_system_prompt(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let prompt = args["prompt"].as_str().unwrap_or("");
    if prompt.is_empty() {
        return "Erreur: parametre 'prompt' requis".to_string();
    }

    if prompt.len() > MAX_PROMPT_SIZE {
        return format!("Erreur: prompt trop long ({} caracteres, max {})", prompt.len(), MAX_PROMPT_SIZE);
    }

    let mut current = ctx.system_prompt.write().await;
    *current = format!("{SAFETY_PREFIX}{prompt}");
    println!("  [Tool:update_system_prompt] {} chars", prompt.len());
    format!("OK: system prompt mis a jour ({} caracteres)", prompt.len())
}

async fn tool_get_system_prompt(ctx: &ToolContext) -> String {
    let current = ctx.system_prompt.read().await;
    println!("  [Tool:get_system_prompt]");
    format!("System prompt actuel:\n{current}")
}

// ---------------------------------------------------------------------------
// Web search tool
// ---------------------------------------------------------------------------

async fn tool_search_web(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let query = args["query"].as_str().unwrap_or("");
    if query.is_empty() {
        return "Erreur: parametre 'query' requis".to_string();
    }

    let Some(ref search_base) = ctx.search_url else {
        return "Service de recherche non configure".to_string();
    };

    let url = format!("{search_base}/api/search");
    match ctx
        .http_client
        .post(&url)
        .json(&serde_json::json!({"query": query, "max_results": 3}))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => {
            match resp.json::<serde_json::Value>().await {
                Ok(json) => {
                    let mut text = String::new();
                    if let Some(results) = json["results"].as_array() {
                        for (i, r) in results.iter().enumerate() {
                            let title = r["title"].as_str().unwrap_or("");
                            let snippet = r["snippet"].as_str().unwrap_or("");
                            let url = r["url"].as_str().unwrap_or("");
                            text.push_str(&format!("{}. {} — {}\n   {}\n", i + 1, title, snippet, url));
                        }
                    }
                    if text.is_empty() {
                        text = "Aucun resultat trouve.".to_string();
                    }
                    println!("  [Tool:search_web] '{}'", query);
                    text
                }
                Err(e) => format!("Erreur parsing: {e}"),
            }
        }
        Ok(resp) => format!("Search HTTP {}", resp.status()),
        Err(e) => format!("Erreur recherche: {e}"),
    }
}

// ---------------------------------------------------------------------------
// Skills tools
// ---------------------------------------------------------------------------

async fn tool_list_skills(ctx: &ToolContext) -> String {
    let Some(ref store) = ctx.skill_store else {
        return "Aucun skill configure (pas de dossier skills/)".to_string();
    };

    let store = store.read().await;
    let skills = store.list();
    if skills.is_empty() {
        return "Aucun skill disponible. Cree des fichiers .md dans le dossier skills/.".to_string();
    }

    let mut text = format!("{} skill(s) disponible(s):\n", skills.len());
    for (name, desc) in &skills {
        text.push_str(&format!("  - {name}: {desc}\n"));
    }

    // Afficher le skill actif
    let active = ctx.active_skill.read().await;
    if let Some(ref skill) = *active {
        text.push_str(&format!("\nSkill actif: {}", skill.name));
    }

    println!("  [Tool:list_skills]");
    text
}

async fn tool_load_skill(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let name = args["name"].as_str().unwrap_or("");
    if name.is_empty() {
        return "Erreur: parametre 'name' requis".to_string();
    }

    let Some(ref store) = ctx.skill_store else {
        return "Aucun skill configure".to_string();
    };

    let store = store.read().await;
    match store.find(name) {
        Some(skill) => {
            // Mettre a jour le system prompt avec celui du skill (S-06: prefixe securite)
            let mut prompt = ctx.system_prompt.write().await;
            *prompt = format!("{}{}", SAFETY_PREFIX, skill.system_prompt);
            drop(prompt);

            // Stocker le skill actif
            let mut active = ctx.active_skill.write().await;
            *active = Some(skill.clone());

            println!("  [Tool:load_skill] {} active", name);
            format!("OK: skill '{}' active. {}", skill.name, skill.description)
        }
        None => {
            let available: Vec<&str> = store.list().iter().map(|(n, _)| *n).collect();
            format!("Skill '{name}' non trouve. Disponibles: {}", available.join(", "))
        }
    }
}

async fn tool_unload_skill(ctx: &ToolContext) -> String {
    let mut active = ctx.active_skill.write().await;
    if active.is_none() {
        return "Aucun skill actif".to_string();
    }

    let name = active.as_ref().unwrap().name.clone();
    *active = None;
    drop(active);

    // Restaurer le system prompt par defaut
    let mut prompt = ctx.system_prompt.write().await;
    *prompt = crate::nlu::DEFAULT_RESPONSE_PROMPT.to_string();

    println!("  [Tool:unload_skill] {} desactive", name);
    format!("OK: skill '{name}' desactive, retour au mode normal")
}

// ---------------------------------------------------------------------------
// Scheduler tools
// ---------------------------------------------------------------------------

async fn tool_schedule_task(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let Some(ref scheduler) = ctx.scheduler else {
        return "Scheduler non configure".to_string();
    };

    let task_type = args["task_type"].as_str().unwrap_or("once");
    let schedule = args["schedule"].as_str().unwrap_or("");
    let action = args["action"].as_str().unwrap_or("");

    if schedule.is_empty() || action.is_empty() {
        return "Erreur: parametres 'schedule' et 'action' requis".to_string();
    }

    match scheduler.add_task(task_type, schedule, action) {
        Ok(id) => format!("OK: tache planifiee (id: {id}, type: {task_type}, schedule: {schedule})"),
        Err(e) => format!("Erreur: {e}"),
    }
}

async fn tool_list_scheduled_tasks(ctx: &ToolContext) -> String {
    let Some(ref scheduler) = ctx.scheduler else {
        return "Scheduler non configure".to_string();
    };

    match scheduler.list_tasks() {
        Ok(tasks) => {
            if tasks.is_empty() {
                return "Aucune tache planifiee active.".to_string();
            }
            let mut text = format!("{} tache(s) active(s):\n", tasks.len());
            for t in &tasks {
                let tt = if t.task_type == crate::scheduler::TaskType::Cron {
                    "cron"
                } else {
                    "once"
                };
                text.push_str(&format!(
                    "  - [{}] ({}) schedule='{}' next='{}' action='{}'\n",
                    t.id.chars().take(8).collect::<String>(),
                    tt,
                    t.schedule,
                    t.next_run.as_deref().unwrap_or("?"),
                    t.action
                ));
            }
            println!("  [Tool:list_scheduled_tasks]");
            text
        }
        Err(e) => format!("Erreur: {e}"),
    }
}

async fn tool_cancel_task(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let Some(ref scheduler) = ctx.scheduler else {
        return "Scheduler non configure".to_string();
    };

    let id = args["id"].as_str().unwrap_or("");
    if id.is_empty() {
        return "Erreur: parametre 'id' requis".to_string();
    }

    match scheduler.cancel_task(id) {
        Ok(true) => {
            println!("  [Tool:cancel_task] {id}");
            format!("OK: tache '{id}' annulee")
        }
        Ok(false) => format!("Tache '{id}' non trouvee ou deja annulee"),
        Err(e) => format!("Erreur: {e}"),
    }
}

// ---------------------------------------------------------------------------
// MCP tools
// ---------------------------------------------------------------------------

async fn tool_list_mcp_tools(ctx: &ToolContext) -> String {
    let Some(ref mcp) = ctx.mcp else {
        return "Aucun serveur MCP configure".to_string();
    };

    let manager = mcp.read().await;
    let defs = manager.all_tool_definitions();

    if defs.is_empty() {
        return "Aucun outil MCP disponible.".to_string();
    }

    let mut text = format!("{} outil(s) MCP disponible(s):\n", defs.len());
    for def in &defs {
        let name = def["function"]["name"].as_str().unwrap_or("?");
        let desc = def["function"]["description"]
            .as_str()
            .unwrap_or("")
            .chars()
            .take(60)
            .collect::<String>();
        text.push_str(&format!("  - {name}: {desc}\n"));
    }

    println!("  [Tool:list_mcp_tools]");
    text
}

async fn tool_call_mcp_tool(ctx: &ToolContext, args: &serde_json::Value) -> String {
    let Some(ref mcp) = ctx.mcp else {
        return "Aucun serveur MCP configure".to_string();
    };

    let server = args["server"].as_str().unwrap_or("");
    let tool = args["tool"].as_str().unwrap_or("");
    let tool_args = args.get("arguments").cloned().unwrap_or(serde_json::json!({}));

    if server.is_empty() || tool.is_empty() {
        return "Erreur: parametres 'server' et 'tool' requis".to_string();
    }

    let prefixed = format!("mcp_{server}_{tool}");
    let mut manager = mcp.write().await;

    match manager.call_tool(&prefixed, tool_args).await {
        Ok(result) => {
            println!("  [Tool:call_mcp_tool] {server}/{tool}");
            // Tronquer a 4KB pour le contexte LLM
            if result.len() > 4096 {
                format!("{}... (tronque a 4KB)", &result[..4096])
            } else {
                result
            }
        }
        Err(e) => format!("Erreur MCP: {e}"),
    }
}

async fn tool_call_mcp_direct(ctx: &ToolContext, name: &str, args: &serde_json::Value) -> String {
    let Some(ref mcp) = ctx.mcp else {
        return "Aucun serveur MCP configure".to_string();
    };

    let mut manager = mcp.write().await;

    match manager.call_tool(name, args.clone()).await {
        Ok(result) => {
            println!("  [Tool:mcp] {name}");
            if result.len() > 4096 {
                format!("{}... (tronque a 4KB)", &result[..4096])
            } else {
                result
            }
        }
        Err(e) => format!("Erreur MCP: {e}"),
    }
}
