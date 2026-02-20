//! EngineConfig — configuration du moteur.
//!
//! @id mge.core.config
//! @role simulation
//! @layer core
//! @human Configuration Engine
//! @do define_engine_configuration

/// Configuration de l'Engine au démarrage.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Seed globale pour le RNG déterministe.
    pub seed: u64,
    /// Mode headless (pas de fenêtre/rendu). Placeholder v0.1 — utilisé par les plugins.
    pub headless: bool,
    /// Si Some, delta fixe en ms (ex: 16 pour 60 ticks/s).
    pub fixed_timestep_ms: Option<u32>,
    /// Budget CPU par tick en ms (ex: 8). Placeholder v0.1 — détection overflow prévue v0.2.
    pub tick_budget_ms: Option<u32>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            headless: true,
            fixed_timestep_ms: None,
            tick_budget_ms: None,
        }
    }
}
