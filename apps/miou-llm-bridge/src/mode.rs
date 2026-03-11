//! Mode de fonctionnement de MAIA — Standalone ou Pro.
//!
//! - **Standalone** : Aucune connexion vers un hub TEAM. Peut être isolé
//!   (--isolated : aucun socket réseau sortant après démarrage).
//! - **Pro** : Enregistré auprès d'un hub TEAM. Reçoit des tâches orchestrées.
//!   Invalide avec --isolated.

use serde::{Deserialize, Serialize};

/// Mode de fonctionnement de MAIA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MaiaMode {
    /// Aucune connexion TEAM. Fonctionne en totale autonomie.
    Standalone,
    /// Enregistré auprès d'un hub TEAM — reçoit des tâches orchestrées.
    Pro,
}

impl std::fmt::Display for MaiaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standalone => write!(f, "standalone"),
            Self::Pro => write!(f, "pro"),
        }
    }
}

impl Default for MaiaMode {
    fn default() -> Self {
        Self::Standalone
    }
}

/// Configuration du mode MAIA parsée depuis les args CLI.
#[derive(Debug, Clone)]
pub struct ModeConfig {
    pub mode: MaiaMode,
    /// Isolation stricte — aucun réseau sortant après démarrage.
    pub isolated: bool,
    /// URL du hub TEAM (mode Pro uniquement).
    pub team_url: Option<String>,
}

impl Default for ModeConfig {
    fn default() -> Self {
        Self {
            mode: MaiaMode::Standalone,
            isolated: false,
            team_url: None,
        }
    }
}

/// Parse les arguments CLI de mode.
/// Lit : `--mode <standalone|pro>`, `--isolated`, `--team <url>`.
pub fn parse_mode_args() -> ModeConfig {
    let args: Vec<String> = std::env::args().collect();

    let mut config = ModeConfig::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => {
                if let Some(mode_str) = args.get(i + 1) {
                    config.mode = match mode_str.as_str() {
                        "pro" => MaiaMode::Pro,
                        "standalone" => MaiaMode::Standalone,
                        other => {
                            eprintln!("Mode inconnu '{other}' — modes valides: standalone, pro");
                            std::process::exit(1);
                        }
                    };
                    i += 1;
                }
            }
            "--isolated" => {
                config.isolated = true;
            }
            "--team" => {
                if let Some(url) = args.get(i + 1) {
                    config.team_url = Some(url.clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Dériver le mode depuis la persona si chargée (mais les args CLI ont priorité)
    config
}

/// Valide la configuration de mode et panique si incohérente.
///
/// Règles :
/// 1. `--isolated` + `--mode pro` → interdit (Pro nécessite une connexion TEAM).
/// 2. `--isolated` + `--team <url>` → interdit (isolation = zéro réseau sortant).
/// 3. `--mode pro` sans `--team <url>` → warning (TEAM_URL env fallback).
pub fn validate_mode(config: &ModeConfig) {
    if config.isolated && config.mode == MaiaMode::Pro {
        eprintln!("ERREUR : --isolated est incompatible avec --mode pro.");
        eprintln!("  Mode Pro nécessite une connexion vers le hub TEAM.");
        eprintln!("  Utilisez --mode standalone --isolated pour l'isolation totale.");
        std::process::exit(1);
    }

    if config.isolated && config.team_url.is_some() {
        eprintln!("ERREUR : --isolated est incompatible avec --team <url>.");
        eprintln!("  Mode isolation = aucun socket réseau sortant.");
        eprintln!("  Retirez --team ou retirez --isolated.");
        std::process::exit(1);
    }

    if config.mode == MaiaMode::Pro && config.team_url.is_none() {
        // Essayer TEAM_URL env
        if std::env::var("TEAM_URL").is_err() {
            tracing::warn!(
                "Mode Pro activé mais --team non spécifié et TEAM_URL non défini. \
                 MAIA ne pourra pas s'enregistrer auprès du hub TEAM."
            );
        }
    }
}

/// Résout l'URL TEAM depuis la config ou l'env TEAM_URL.
pub fn resolve_team_url(config: &ModeConfig) -> Option<String> {
    config
        .team_url
        .clone()
        .or_else(|| std::env::var("TEAM_URL").ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_display() {
        assert_eq!(MaiaMode::Standalone.to_string(), "standalone");
        assert_eq!(MaiaMode::Pro.to_string(), "pro");
    }

    #[test]
    fn test_default_mode_is_standalone() {
        let config = ModeConfig::default();
        assert_eq!(config.mode, MaiaMode::Standalone);
        assert!(!config.isolated);
        assert!(config.team_url.is_none());
    }
}
