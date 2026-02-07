//! État de l'environnement MiyukiniAdmin (Auth and First-Boot Contract §3).
//!
//! Référence : MiyukiniAdmin - Auth and First-Boot Contract §3.1 à 3.5

use serde::{Deserialize, Serialize};

/// @id: miyukiniadmin_environment_state_enum
/// @role: data
/// @layer: operator
/// @human: État de l'environnement : Vierge, Initialisé, ou Compromis.
/// @do: represent_environment_state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum EnvironmentState {
    /// Aucun artefact présent (EIP, registre admin, schéma bootstrap absents).
    /// Parcours Futur Admin (setup) uniquement.
    /// @id: miyukiniadmin_env_state_vierge
    #[default]
    Vierge,

    /// Tous les artefacts présents et valides. Environnement opérationnel.
    /// @id: miyukiniadmin_env_state_initialise
    Initialise,

    /// Au moins un artefact présent mais invalide ou incohérent.
    /// Réponse sécuritaire : page « Environnement compromis », recovery.
    /// @id: miyukiniadmin_env_state_compromis
    Compromis,
}


impl std::fmt::Display for EnvironmentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vierge => write!(f, "VIERGE"),
            Self::Initialise => write!(f, "INITIALISE"),
            Self::Compromis => write!(f, "COMPROMIS"),
        }
    }
}
