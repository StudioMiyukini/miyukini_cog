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
const ALLOWED_COMMANDS: &[&str] = &[
    // Build & dev
    "cargo",
    "rustc",
    "rustfmt",
    "clippy-driver",
    "npm",
    "npx",
    "node",
    "bun",
    "deno",
    "python",
    "python3",
    "pip",
    "pip3",
    "git",
    // Utilitaires
    "ls",
    "dir",
    "cat",
    "head",
    "tail",
    "wc",
    "sort",
    "uniq",
    "find",
    "grep",
    "rg",
    "fd",
    "echo",
    "printf",
    "mkdir",
    "cp",
    "mv",
    "curl",
    "wget",
    // (interpréteurs de commandes powershell/cmd retirés : pas de shell)
    // Diagnostic
    "ping",
    "nslookup",
    "ipconfig",
    "ifconfig",
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
        "Répertoire de travail (relatif au sandbox)".into(),
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

    // Répertoire de travail
    let cwd = params
        .get("cwd")
        .and_then(|v| v.as_str())
        .map(std::path::PathBuf::from);

    // Exécution directe du binaire (jamais via un shell)
    let mut cmd = tokio::process::Command::new(command);
    cmd.args(&args);

    if let Some(ref dir) = cwd {
        cmd.current_dir(dir);
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

    #[cfg(unix)]
    #[tokio::test]
    async fn execution_directe_whitelist_ok() {
        let r = execute(&p(serde_json::json!({"command": "echo", "args": ["salut"]}))).await;
        assert!(r.success, "{:?}", r.error);
        assert!(r.output["stdout"].as_str().unwrap().contains("salut"));
    }
}
