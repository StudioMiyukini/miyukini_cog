//! Module d'état de santé de CaringNanny

/// @id caringnanny_health_status
/// @role data
/// @layer core
/// @do represent_health_status
//@human / @human État de santé d'un composant ou du système.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// @id caringnanny_health_status_healthy
    /// @role data
    /// @layer core
    /// @do represent_healthy_status
    //@human / @human Composant en bonne santé.
    /// @depends caringnanny_health_status
    Healthy,
    /// @id caringnanny_health_status_degraded
    /// @role data
    /// @layer core
    /// @do represent_degraded_status
    //@human / @human Composant dégradé mais fonctionnel.
    /// @depends caringnanny_health_status
    Degraded,
    /// @id caringnanny_health_status_unhealthy
    /// @role data
    /// @layer core
    /// @do represent_unhealthy_status
    //@human / @human Composant non fonctionnel.
    /// @depends caringnanny_health_status
    Unhealthy,
}

/// @id caringnanny_health_checker_trait
/// @role infrastructure
/// @layer core
/// @do define_health_checker_contract
//@human / @human Trait de vérification de l'état de santé.
pub trait HealthChecker {
    /// @id caringnanny_health_checker_check
    /// @role accessor
    /// @layer core
    /// @do check_component_health
    //@human / @human Vérifie l'état de santé d'un composant.
    /// @depends caringnanny_health_checker_trait
    fn check(&self, component: &str) -> HealthStatus;
}

/// Vérificateur par défaut : registre composant -> statut, défaut Healthy.
#[derive(Debug, Default)]
pub struct DefaultHealthChecker {
    status_by_component: std::collections::HashMap<String, HealthStatus>,
}

impl DefaultHealthChecker {
    /// Crée un vérificateur vide (tous les composants considérés Healthy).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre le statut d'un composant (pour tests ou mise à jour par observations).
    pub fn set_status(&mut self, component: impl Into<String>, status: HealthStatus) {
        self.status_by_component.insert(component.into(), status);
    }
}

impl HealthChecker for DefaultHealthChecker {
    fn check(&self, component: &str) -> HealthStatus {
        self.status_by_component
            .get(component)
            .copied()
            .unwrap_or(HealthStatus::Healthy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id caringnanny_health_test_status_equality
    /// @role test
    /// @layer core
    /// @do verify_health_status_equality
    //@human / @human Test d'égalité des états de santé.
    /// @depends caringnanny_health_status
    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_default_health_checker() {
        let mut checker = DefaultHealthChecker::new();
        assert_eq!(checker.check("unknown"), HealthStatus::Healthy);
        checker.set_status("db", HealthStatus::Degraded);
        assert_eq!(checker.check("db"), HealthStatus::Degraded);
    }
}
