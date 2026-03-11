//! Execution de commandes shell sandboxee pour Alicia.
//!
//! Whitelist de commandes, blocklist de patterns dangereux,
//! timeout, sortie tronquee, et execution directe (sans interpreteur shell).

use std::path::Path;

/// Timeout par defaut (secondes).
const DEFAULT_TIMEOUT_SECS: u64 = 10;
/// Timeout maximum autorise.
const MAX_TIMEOUT_SECS: u64 = 30;
/// Taille max de la sortie (4 KB — adapte au contexte 4B).
const MAX_OUTPUT_SIZE: usize = 4_096;

/// Commandes autorisees (whitelist).
/// S-04: powershell et cmd retires (interpreters shell = reintroduction des risques).
const ALLOWED_COMMANDS: &[&str] = &[
    // Build & dev
    "cargo", "rustc", "git", "python", "python3", "node", "npm", "npx",
    // Utilitaires
    "ls", "dir", "cat", "head", "tail", "wc", "sort", "find", "grep", "rg",
    "echo", "mkdir", "cp", "mv", "curl", "ping",
];

/// Patterns bloques (defense en profondeur).
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
    ":(){ :|:& };:",
];

/// S-04: Metacaracteres shell interdits — empeche pipe chains, command substitution, etc.
const SHELL_METACHARS: &[char] = &['|', ';', '&', '`', '$', '(', ')', '>', '<', '\n', '\r'];

/// Execute une commande sandboxee.
/// S-04: Execution directe (pas d'interpreteur shell sh -c / cmd /C).
/// S-09: Normalisation des espaces avant check blocklist.
pub async fn run_command(
    workspace: &Path,
    args: &serde_json::Value,
) -> String {
    let command = match args["command"].as_str() {
        Some(cmd) if !cmd.is_empty() => cmd,
        _ => return "Erreur: parametre 'command' requis".to_string(),
    };

    let timeout_secs = args["timeout"]
        .as_u64()
        .unwrap_or(DEFAULT_TIMEOUT_SECS)
        .min(MAX_TIMEOUT_SECS);

    // S-04: Bloquer les metacaracteres shell AVANT tout traitement
    if command.chars().any(|c| SHELL_METACHARS.contains(&c)) {
        return "Erreur: caracteres speciaux shell interdits (|;&`$()><). Utilise une commande simple.".to_string();
    }

    // S-09: Normaliser les espaces avant check blocklist
    let cmd_normalized: String = command.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ");
    for blocked in BLOCKED_PATTERNS {
        if cmd_normalized.contains(&blocked.to_lowercase()) {
            return format!("Erreur: commande bloquee par politique de securite: {command}");
        }
    }

    // Parser la commande en tokens (pas d'interpreteur shell)
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.is_empty() {
        return "Erreur: commande vide".to_string();
    }

    let program = tokens[0];
    let arguments = &tokens[1..];

    // Verifier la whitelist (premier token)
    let base_lower = program.to_lowercase();
    let base_name = std::path::Path::new(&base_lower)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&base_lower);
    let allowed = ALLOWED_COMMANDS.iter().any(|c| base_name == *c);

    if !allowed {
        return format!(
            "Erreur: commande '{program}' non autorisee. Permises: {}",
            ALLOWED_COMMANDS.join(", ")
        );
    }

    // S-04: Execution directe — Command::new(program).args(arguments)
    // Pas de sh -c ni cmd /C — aucune interpretation de metacaracteres
    let mut cmd = tokio::process::Command::new(program);
    cmd.args(arguments);
    cmd.current_dir(workspace);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    println!("  [Tool:run_command] {} {} (timeout={timeout_secs}s)", program, arguments.join(" "));

    let result = tokio::time::timeout(
        std::time::Duration::from_secs(timeout_secs),
        cmd.output(),
    )
    .await;

    match result {
        Ok(Ok(output)) => {
            let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if stdout.len() > MAX_OUTPUT_SIZE {
                stdout.truncate(MAX_OUTPUT_SIZE);
                stdout.push_str("\n... [tronque]");
            }
            if stderr.len() > MAX_OUTPUT_SIZE {
                stderr.truncate(MAX_OUTPUT_SIZE);
                stderr.push_str("\n... [tronque]");
            }

            let exit_code = output.status.code().unwrap_or(-1);
            let mut result_text = format!("Exit code: {exit_code}\n");
            if !stdout.is_empty() {
                result_text.push_str(&format!("stdout:\n{stdout}\n"));
            }
            if !stderr.is_empty() {
                result_text.push_str(&format!("stderr:\n{stderr}\n"));
            }
            if stdout.is_empty() && stderr.is_empty() {
                result_text.push_str("(pas de sortie)");
            }
            result_text
        }
        Ok(Err(e)) => format!("Erreur d'execution: {e}"),
        Err(_) => format!("Timeout apres {timeout_secs}s"),
    }
}
