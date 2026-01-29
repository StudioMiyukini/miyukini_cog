//! Module d'état de santé de CaringNanny

/// @id: caringnanny_health_status
/// @role: data
/// @layer: core
/// @human: État de santé d'un composant ou du système.
/// @do: represent_health_status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// @id: caringnanny_health_status_healthy
    /// @role: data
    /// @layer: core
    /// @human: Composant en bonne santé.
    /// @do: represent_healthy_status
    /// @depends: caringnanny_health_status
    Healthy,
    /// @id: caringnanny_health_status_degraded
    /// @role: data
    /// @layer: core
    /// @human: Composant dégradé mais fonctionnel.
    /// @do: represent_degraded_status
    /// @depends: caringnanny_health_status
    Degraded,
    /// @id: caringnanny_health_status_unhealthy
    /// @role: data
    /// @layer: core
    /// @human: Composant non fonctionnel.
    /// @do: represent_unhealthy_status
    /// @depends: caringnanny_health_status
    Unhealthy,
}

/// @id: caringnanny_health_checker_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de vérification de l'état de santé.
/// @do: define_health_checker_contract
pub trait HealthChecker {
    /// @id: caringnanny_health_checker_check
    /// @role: accessor
    /// @layer: core
    /// @human: Vérifie l'état de santé d'un composant.
    /// @do: check_component_health
    /// @depends: caringnanny_health_checker_trait
    fn check(&self, component: &str) -> HealthStatus;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: caringnanny_health_test_status_equality
    /// @role: test
    /// @layer: core
    /// @human: Test d'égalité des états de santé.
    /// @do: verify_health_status_equality
    /// @depends: caringnanny_health_status
    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    }
}
