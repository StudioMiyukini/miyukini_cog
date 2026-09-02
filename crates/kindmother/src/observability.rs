//! Module d'observabilité de KindMother
//!
//! Ce module collecte les métriques, l'état de santé, et les informations d'observabilité
//! pour le monitoring et le diagnostic.

use crate::state::InstanceIdentity;

/// @id kindmother_observability_health_status
/// @role data
/// @layer core
/// @do represent_health_status
//@human / @human État de santé d'une instance.
/// État de santé d'une instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// @id kindmother_observability_health_healthy
    /// @role data
    /// @layer core
    /// @do represent_healthy_status
    //@human / @human Instance en bonne santé.
    /// @depends kindmother_observability_health_status
    Healthy,
    /// @id kindmother_observability_health_degraded
    /// @role data
    /// @layer core
    /// @do represent_degraded_status
    //@human / @human Instance dégradée mais fonctionnelle.
    /// @depends kindmother_observability_health_status
    Degraded,
    /// @id kindmother_observability_health_unhealthy
    /// @role data
    /// @layer core
    /// @do represent_unhealthy_status
    //@human / @human Instance non fonctionnelle.
    /// @depends kindmother_observability_health_status
    Unhealthy,
}

/// @id kindmother_observability_metrics
/// @role data
/// @layer core
/// @do represent_instance_metrics
//@human / @human Métriques d'une instance.
/// Métriques d'une instance.
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// @id kindmother_observability_metrics_read_count
    /// @role data
    /// @layer core
    /// @do store_read_count
    //@human / @human Nombre d'opérations de lecture.
    /// @depends kindmother_observability_metrics
    pub read_count: u64,
    /// @id kindmother_observability_metrics_write_count
    /// @role data
    /// @layer core
    /// @do store_write_count
    //@human / @human Nombre d'opérations d'écriture.
    /// @depends kindmother_observability_metrics
    pub write_count: u64,
    /// @id kindmother_observability_metrics_sync_count
    /// @role data
    /// @layer core
    /// @do store_sync_count
    //@human / @human Nombre de synchronisations.
    /// @depends kindmother_observability_metrics
    pub sync_count: u64,
}

/// @id kindmother_observability_trait
/// @role infrastructure
/// @layer core
/// @do define_observability_contract
//@human / @human Trait d'observabilité. Collecte les métriques et l'état de santé.
/// Trait d'observabilité.
pub trait Observability {
    /// @id kindmother_observability_get_health
    /// @role accessor
    /// @layer core
    /// @do get_instance_health
    //@human / @human Récupère l'état de santé d'une instance.
    /// @depends kindmother_observability_trait
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

    /// @id kindmother_observability_get_metrics
    /// @role accessor
    /// @layer core
    /// @do get_instance_metrics
    //@human / @human Récupère les métriques d'une instance.
    /// @depends kindmother_observability_trait
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

/// @id kindmother_observability_default
/// @role infrastructure
/// @layer core
/// @do provide_default_observability
//@human / @human Observabilité par défaut. Implémentation basique pour les tests.
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

    /// @id kindmother_observability_test_default_health
    /// @role test
    /// @layer core
    /// @do verify_default_observability
    //@human / @human Test de l'observabilité par défaut.
    /// @depends kindmother_observability_default, kindmother_observability_get_health
    #[test]
    fn test_default_health() {
        let obs = DefaultObservability;
        let instance = InstanceState::new(InstanceType::Mother);
        assert_eq!(obs.get_health(&instance.identity), HealthStatus::Healthy);
    }

    /// @id kindmother_observability_test_default_metrics
    /// @role test
    /// @layer core
    /// @do verify_default_metrics
    //@human / @human Test des métriques par défaut.
    /// @depends kindmother_observability_default, kindmother_observability_get_metrics
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
