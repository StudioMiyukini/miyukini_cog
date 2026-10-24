//! @id     miou_llm_bridge_skills_shell_exec
//! @do     definir_default_timeout_secs
//! @role   orchestration
//! @layer  app
//! @human  miou-llm-bridge — module « skills shell exec ».

//! Skill : Exécution de commandes shell (sandboxée).
//!
//! Exécute des commandes système avec des restrictions strictes :
//! - Timeout configurable (défaut 30s, max 120s)
//! - Whitelist de commandes autorisées
//! - Répertoire de travail restreint au sandbox
//! - Taille de sortie limitée
//!
//! Sécurité (lot 1, fail-closed) : AUCUNE exécution via un interpréteur de
//! commandes (`sh -c`, `cmd /C`). Le paramètre `shell=true` est refusé ;
//! seul le lancement direct d'un binaire de la whitelist (nom nu, sans
//! chemin ni espace) est possible.

use super::{SkillInfo, SkillResult};
use std::collections::HashMap;

/// Timeout par défaut (secondes).
const DEFAULT_TIMEOUT_SECS: u64 = 30;
/// Timeout maximum autorisé.
const MAX_TIMEOUT_SECS: u64 = 120;
/// Taille max de la sortie (64 KB).
const MAX_OUTPUT_SIZE: usize = 65_536;

/// Commandes autorisées (whitelist).
/// Les commandes non listées ici sont refusées.
///
/// Uniquement des outils de lecture/filtrage SANS capacité d'exécuter du code :
/// les interpréteurs (python, node, deno, bun), gestionnaires de paquets et
/// outils de build (cargo, npm, npx, pip, rustc : build.rs / proc-macros / scripts),
/// git (`-c`, alias `!`), find/fd (`-exec`, `-x`), curl/wget (écriture, SSRF)
/// et cp/mv/mkdir (écriture hors sandbox) ont été retirés.
const ALLOWED_COMMANDS: &[&str] = &[
    "ls", "cat", "head", "tail", "wc", "sort", "uniq", "grep", "rg", "echo", "printf",
];

/// Options refusées même pour une commande de la whitelist, car elles
/// exécutent un programme externe ou lisent une liste de fichiers arbitraire.
const BLOCKED_OPTIONS: &[&str] = &[
    "--pre",              // rg : préprocesseur = commande arbitraire
    "--pre-glob",
    "--compress-program", // sort : programme de compression arbitraire
    "--files0-from",      // wc/sort : liste de fichiers hors sandbox
    "--search-zip",       // rg -z : lance des décompresseurs externes
    "-z",
];

/// Commandes interdites (blocklist — même si elles passent la whitelist via shell).
const BLOCKED_PATTERNS: &[&str] = &[
    "rm -rf /",
    "rm -rf ~",
    "format ",
    "del /s /q",
    "rd /s /q",
    "shutdown",
    "reboot",
    "mkfs",
    "dd if=",
    "> /dev/sd",
    ":(){ :|:& };:", // Fork bomb
];

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert("command".into(), "Commande à exécuter (obligatoire)".into());
    params.insert("args".into(), "Arguments (tableau de strings)".into());
    params.insert(
        "cwd".into(),
        "Répertoire de travail relatif au sandbox (défaut : racine du sandbox)".into(),
    );
    params.insert(
        "timeout".into(),
        format!("Timeout en secondes (défaut {DEFAULT_TIMEOUT_SECS}, max {MAX_TIMEOUT_SECS})"),
    );
    params.insert(
        "shell".into(),
        "REFUSÉ si true — l'exécution via shell n'est pas supportée".into(),
    );

    SkillInfo {
        id: "shell_exec".into(),
        name: "Exécution Shell".into(),
        description:
            "Exécute des commandes système sandboxées (whitelist, timeout, sortie limitée).".into(),
        parameters: params,
        min_security_level: "extended",
    }
}

fn refuse(error: String) -> SkillResult {
    SkillResult {
        success: false,
        output: serde_json::Value::Null,
        error: Some(error),
        duration_ms: 0,
    }
}

/// Vérifie que `command` est un nom nu présent dans la whitelist.
fn check_command_allowed(command: &str) -> Result<(), String> {
    let bare = !command.is_empty()
        && !command.chars().any(|c| c.is_whitespace() || c == '/' || c == '\\');
    if bare && ALLOWED_COMMANDS.iter().any(|c| command.eq_ignore_ascii_case(c)) {
        return Ok(());
    }
    Err(format!(
        "Commande '{command}' non autorisée (nom nu de la whitelist requis, sans chemin \
         ni arguments inline). Commandes permises : {}",
        ALLOWED_COMMANDS.join(", ")
    ))
}

/// Valide les arguments : pas de chemin absolu ni de `..` (tout reste relatif
/// au sandbox), pas d'option qui exécute un programme, et blocklist appliquée
/// à la ligne complète (commande + arguments).
fn check_args_allowed(args: &[String]) -> Result<(), String> {
    use std::path::{Component, Path};
    let joined = args.join(" ").to_lowercase();
    for blocked in BLOCKED_PATTERNS {
        if joined.contains(&blocked.to_lowercase()) {
            return Err("Arguments bloqués par la politique de sécurité".into());
        }
    }
    for arg in args {
        let opt = arg.split('=').next().unwrap_or(arg);
        if BLOCKED_OPTIONS.iter().any(|b| opt.eq_ignore_ascii_case(b)) {
            return Err(format!("Option refusée : {opt}"));
        }
        // Une valeur d'option (`--file=/etc/x`) est aussi un chemin potentiel.
        let value = arg.split_once('=').map(|(_, v)| v).unwrap_or(arg);
        for candidate in [arg.as_str(), value] {
            let path = Path::new(candidate);
            if path.is_absolute()
                || candidate.starts_with('~')
                || path.components().any(|c| matches!(c, Component::ParentDir | Component::Prefix(_) | Component::RootDir))
            {
                return Err(format!("Argument hors sandbox refusé : {arg}"));
            }
        }
    }
    Ok(())
}

/// Exécute une commande sandboxée (exécution directe, jamais via shell).
pub async fn execute(params: &HashMap<String, serde_json::Value>) -> SkillResult {
    let command = match params.get("command").and_then(|v| v.as_str()) {
        Some(cmd) => cmd,
        None => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some("Paramètre 'command' requis".into()),
                duration_ms: 0,
            };
        }
    };

    // Vérifier les patterns bloqués
    let cmd_lower = command.to_lowercase();
    for blocked in BLOCKED_PATTERNS {
        if cmd_lower.contains(&blocked.to_lowercase()) {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!(
                    "Commande bloquée par la politique de sécurité : {command}"
                )),
                duration_ms: 0,
            };
        }
    }

    // `shell` : tout sauf un `false` explicite est refusé (fail-closed).
    if let Some(v) = params.get("shell") {
        if v.as_bool() != Some(false) {
            return refuse("Exécution via shell refusée (paramètre 'shell' non supporté)".into());
        }
    }

    // Whitelist : le nom de commande doit être un nom nu (ni espace, ni
    // séparateur de chemin) égal à une entrée de la whitelist.
    if let Err(msg) = check_command_allowed(command) {
        return refuse(msg);
    }

    // Timeout
    let timeout_secs = params
        .get("timeout")
        .and_then(|v| v.as_u64())
        .unwrap_or(DEFAULT_TIMEOUT_SECS)
        .min(MAX_TIMEOUT_SECS);

    // Arguments
    let args: Vec<String> = params
        .get("args")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    if let Err(msg) = check_args_allowed(&args) {
        return refuse(msg);
    }

    // Répertoire de travail : toujours dans le sandbox (fail-closed).
    let sandbox = super::file_ops::resolve_sandbox();
    let cwd_rel = params.get("cwd").and_then(|v| v.as_str()).unwrap_or("");
    let cwd = match super::file_ops::resolve_in_sandbox(&sandbox, cwd_rel) {
        Ok(dir) => dir,
        Err(msg) => return refuse(format!("cwd invalide : {msg}")),
    };

    // Exécution directe du binaire (jamais via un shell), environnement minimal.
    let mut cmd = tokio::process::Command::new(command);
    cmd.args(&args);
    cmd.current_dir(&cwd);
    cmd.env_clear();
    if let Ok(path) = std::env::var("PATH") {
        cmd.env("PATH", path);
    }

    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // Exécuter avec timeout
    let result =
        tokio::time::timeout(std::time::Duration::from_secs(timeout_secs), cmd.output()).await;

    match result {
        Ok(Ok(output)) => {
            let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();

            // Tronquer si trop long
            if stdout.len() > MAX_OUTPUT_SIZE {
                stdout.truncate(MAX_OUTPUT_SIZE);
                stdout.push_str("\n... [tronqué]");
            }
            if stderr.len() > MAX_OUTPUT_SIZE {
                stderr.truncate(MAX_OUTPUT_SIZE);
                stderr.push_str("\n... [tronqué]");
            }

            let exit_code = output.status.code().unwrap_or(-1);

            SkillResult {
                success: output.status.success(),
                output: serde_json::json!({
                    "stdout": stdout,
                    "stderr": stderr,
                    "exit_code": exit_code,
                    "command": command,
                }),
                error: if output.status.success() {
                    None
                } else {
                    Some(format!("Exit code {exit_code}"))
                },
                duration_ms: 0,
            }
        }
        Ok(Err(e)) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Erreur d'exécution : {e}")),
            duration_ms: 0,
        },
        Err(_) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Timeout après {timeout_secs}s")),
            duration_ms: timeout_secs * 1000,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(v: serde_json::Value) -> HashMap<String, serde_json::Value> {
        serde_json::from_value(v).unwrap()
    }

    #[tokio::test]
    async fn shell_true_refuse() {
        let r = execute(&p(serde_json::json!({"command": "echo", "args": ["x"], "shell": true}))).await;
        assert!(!r.success);
        assert!(r.error.unwrap().contains("shell"));
        // Valeur non booléenne : refusée aussi.
        let r = execute(&p(serde_json::json!({"command": "echo", "shell": "true"}))).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn injection_shell_dans_command_refusee() {
        let r = execute(&p(serde_json::json!({"command": "echo hi; id", "shell": true}))).await;
        assert!(!r.success);
        let r = execute(&p(serde_json::json!({"command": "echo hi; id"}))).await;
        assert!(!r.success);
    }

    #[test]
    fn whitelist_nom_nu_uniquement() {
        assert!(check_command_allowed("echo").is_ok());
        assert!(check_command_allowed("/tmp/evil/cargo").is_err());
        assert!(check_command_allowed("sh").is_err());
        assert!(check_command_allowed("cmd").is_err());
        assert!(check_command_allowed("echo hi").is_err());
        assert!(check_command_allowed("").is_err());
    }

    #[test]
    fn interpreteurs_et_outils_executants_refuses() {
        for c in ["python", "python3", "node", "deno", "bun", "npx", "npm", "pip", "cargo", "rustc", "git", "find", "fd", "curl", "wget", "cp", "mv"] {
            assert!(check_command_allowed(c).is_err(), "{c} devrait être refusé");
        }
    }

    #[test]
    fn arguments_hors_sandbox_refuses() {
        let a = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(check_args_allowed(&a(&["notes.txt"])).is_ok());
        assert!(check_args_allowed(&a(&["-n", "motif", "src"])).is_ok());
        assert!(check_args_allowed(&a(&["/etc/passwd"])).is_err());
        assert!(check_args_allowed(&a(&["../secret"])).is_err());
        assert!(check_args_allowed(&a(&["a/../../b"])).is_err());
        assert!(check_args_allowed(&a(&["~/.ssh/id_ed25519"])).is_err());
        assert!(check_args_allowed(&a(&["--file=/etc/shadow"])).is_err());
    }

    #[test]
    fn options_executantes_refusees() {
        let a = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(check_args_allowed(&a(&["--pre=sh", "x"])).is_err());
        assert!(check_args_allowed(&a(&["--pre", "sh", "x"])).is_err());
        assert!(check_args_allowed(&a(&["--compress-program=sh"])).is_err());
        assert!(check_args_allowed(&a(&["-z", "x"])).is_err());
    }

    #[tokio::test]
    async fn cwd_hors_sandbox_refuse() {
        let r = execute(&p(serde_json::json!({"command": "ls", "cwd": "/etc"}))).await;
        assert!(!r.success);
        let r = execute(&p(serde_json::json!({"command": "ls", "cwd": "../.."}))).await;
        assert!(!r.success);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn execution_directe_whitelist_ok() {
        let r = execute(&p(serde_json::json!({"command": "echo", "args": ["salut"]}))).await;
        assert!(r.success, "{:?}", r.error);
        assert!(r.output["stdout"].as_str().unwrap().contains("salut"));
    }
}
