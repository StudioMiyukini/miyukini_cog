//! Module d'observabilité de KindMother
//!
//! Ce module collecte les métriques, l'état de santé, et les informations d'observabilité
//! pour le monitoring et le diagnostic.

use crate::state::InstanceIdentity;

/// @id: kindmother_observability_health_status
/// @role: data
/// @layer: core
/// @human: État de santé d'une instance.
/// @do: represent_health_status
/// État de santé d'une instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// @id: kindmother_observability_health_healthy
    /// @role: data
    /// @layer: core
    /// @human: Instance en bonne santé.
    /// @do: represent_healthy_status
    /// @depends: kindmother_observability_health_status
    Healthy,
    /// @id: kindmother_observability_health_degraded
    /// @role: data
    /// @layer: core
    /// @human: Instance dégradée mais fonctionnelle.
    /// @do: represent_degraded_status
    /// @depends: kindmother_observability_health_status
    Degraded,
    /// @id: kindmother_observability_health_unhealthy
    /// @role: data
    /// @layer: core
    /// @human: Instance non fonctionnelle.
    /// @do: represent_unhealthy_status
    /// @depends: kindmother_observability_health_status
    Unhealthy,
}

/// @id: kindmother_observability_metrics
/// @role: data
/// @layer: core
/// @human: Métriques d'une instance.
/// @do: represent_instance_metrics
/// Métriques d'une instance.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Metrics {
    /// @id: kindmother_observability_metrics_read_count
    /// @role: data
    /// @layer: core
    /// @human: Nombre d'opérations de lecture.
    /// @do: store_read_count
    /// @depends: kindmother_observability_metrics
    pub read_count: u64,
    /// @id: kindmother_observability_metrics_write_count
    /// @role: data
    /// @layer: core
    /// @human: Nombre d'opérations d'écriture.
    /// @do: store_write_count
    /// @depends: kindmother_observability_metrics
    pub write_count: u64,
    /// @id: kindmother_observability_metrics_sync_count
    /// @role: data
    /// @layer: core
    /// @human: Nombre de synchronisations.
    /// @do: store_sync_count
    /// @depends: kindmother_observability_metrics
    pub sync_count: u64,
}


/// @id: kindmother_observability_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait d'observabilité. Collecte les métriques et l'état de santé.
/// @do: define_observability_contract
/// Trait d'observabilité.
pub trait Observability {
    /// @id: kindmother_observability_get_health
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère l'état de santé d'une instance.
    /// @do: get_instance_health
    /// @depends: kindmother_observability_trait
    /// Récupère l'état de santé d'une instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance
    ///
    /// # Returns
    ///
    /// État de santé de l'instance.
    fn get_health(&self, instance: &InstanceIdentity) -> HealthStatus;

    /// @id: kindmother_observability_get_metrics
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère les métriques d'une instance.
    /// @do: get_instance_metrics
    /// @depends: kindmother_observability_trait
    /// Récupère les métriques d'une instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance
    ///
    /// # Returns
    ///
    /// Métriques de l'instance.
    fn get_metrics(&self, instance: &InstanceIdentity) -> Metrics;
}

/// @id: kindmother_observability_default
/// @role: infrastructure
/// @layer: core
/// @human: Observabilité par défaut. Implémentation basique pour les tests.
/// @do: provide_default_observability
/// Observabilité par défaut.
#[derive(Debug, Default)]
pub struct DefaultObservability;

impl Observability for DefaultObservability {
    fn get_health(&self, _instance: &InstanceIdentity) -> HealthStatus {
        HealthStatus::Healthy
    }

    fn get_metrics(&self, _instance: &InstanceIdentity) -> Metrics {
        Metrics::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InstanceState, InstanceType};

    /// @id: kindmother_observability_test_default_health
    /// @role: test
    /// @layer: core
    /// @human: Test de l'observabilité par défaut.
    /// @do: verify_default_observability
    /// @depends: kindmother_observability_default, kindmother_observability_get_health
    #[test]
    fn test_default_health() {
        let obs = DefaultObservability;
        let instance = InstanceState::new(InstanceType::Mother);
        assert_eq!(obs.get_health(&instance.identity), HealthStatus::Healthy);
    }

    /// @id: kindmother_observability_test_default_metrics
    /// @role: test
    /// @layer: core
    /// @human: Test des métriques par défaut.
    /// @do: verify_default_metrics
    /// @depends: kindmother_observability_default, kindmother_observability_get_metrics
    #[test]
    fn test_default_metrics() {
        let obs = DefaultObservability;
        let instance = InstanceState::new(InstanceType::Mother);
        let metrics = obs.get_metrics(&instance.identity);
        assert_eq!(metrics.read_count, 0);
        assert_eq!(metrics.write_count, 0);
        assert_eq!(metrics.sync_count, 0);
    }
}
