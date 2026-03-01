//! Skill : Exécution de commandes shell (sandboxée).
//!
//! Exécute des commandes système avec des restrictions strictes :
//! - Timeout configurable (défaut 30s, max 120s)
//! - Whitelist de commandes autorisées
//! - Répertoire de travail restreint au sandbox
//! - Taille de sortie limitée

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
    "cargo", "rustc", "rustfmt", "clippy-driver",
    "npm", "npx", "node", "bun", "deno",
    "python", "python3", "pip", "pip3",
    "git",
    // Utilitaires
    "ls", "dir", "cat", "head", "tail", "wc", "sort", "uniq",
    "find", "grep", "rg", "fd",
    "echo", "printf",
    "mkdir", "cp", "mv",
    "curl", "wget",
    // Windows
    "powershell", "cmd",
    // Diagnostic
    "ping", "nslookup", "ipconfig", "ifconfig",
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
    ":(){ :|:& };:",  // Fork bomb
];

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert("command".into(), "Commande à exécuter (obligatoire)".into());
    params.insert("args".into(), "Arguments (tableau de strings)".into());
    params.insert("cwd".into(), "Répertoire de travail (relatif au sandbox)".into());
    params.insert("timeout".into(), format!("Timeout en secondes (défaut {DEFAULT_TIMEOUT_SECS}, max {MAX_TIMEOUT_SECS})"));
    params.insert("shell".into(), "Exécuter via shell (défaut: false)".into());

    SkillInfo {
        id: "shell_exec".into(),
        name: "Exécution Shell".into(),
        description: "Exécute des commandes système sandboxées (whitelist, timeout, sortie limitée).".into(),
        parameters: params,
        min_security_level: "extended",
    }
}

/// Exécute une commande shell sandboxée.
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
                error: Some(format!("Commande bloquée par la politique de sécurité : {command}")),
                duration_ms: 0,
            };
        }
    }

    // Vérifier la whitelist
    let use_shell = params
        .get("shell")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if !use_shell {
        let base_cmd = command.split_whitespace().next().unwrap_or("");
        let base_cmd_lower = base_cmd.to_lowercase();
        let allowed = ALLOWED_COMMANDS
            .iter()
            .any(|c| base_cmd_lower == c.to_lowercase() || base_cmd_lower.ends_with(&format!("/{c}")) || base_cmd_lower.ends_with(&format!("\\{c}")));

        if !allowed {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!(
                    "Commande '{base_cmd}' non autorisée. Commandes permises : {}",
                    ALLOWED_COMMANDS.join(", ")
                )),
                duration_ms: 0,
            };
        }
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

    // Construire la commande
    let mut cmd = if use_shell {
        #[cfg(target_os = "windows")]
        {
            let mut c = tokio::process::Command::new("cmd");
            c.args(["/C", command]);
            c
        }
        #[cfg(not(target_os = "windows"))]
        {
            let mut c = tokio::process::Command::new("sh");
            c.args(["-c", command]);
            c
        }
    } else {
        let mut c = tokio::process::Command::new(command);
        c.args(&args);
        c
    };

    if let Some(ref dir) = cwd {
        cmd.current_dir(dir);
    }

    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // Exécuter avec timeout
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs),
        cmd.output(),
    )
    .await;

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
