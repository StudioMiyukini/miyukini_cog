//! Service de monitoring de MiyukiniAdmin

/// @id miyukiniadmin_monitoring_service_trait
/// @role infrastructure
/// @layer operator
/// @do define_monitoring_service_contract
//@human / @human Trait de service de monitoring. Collecte et agrège les métriques de tous les cores.
pub trait MonitoringService {
    /// @id miyukiniadmin_monitoring_service_collect_metrics
    /// @role infrastructure
    /// @layer operator
    /// @do collect_all_core_metrics
    //@human / @human Collecte les métriques de tous les cores système.
    /// @depends miyukiniadmin_monitoring_service_trait, caringnanny_metrics_collector_get
    /// Collecte les métriques de tous les cores.
    fn collect_metrics(&self) -> SystemMetrics;
}

/// @id miyukiniadmin_monitoring_system_metrics
/// @role data
/// @layer operator
/// @do represent_system_metrics
//@human / @human Métriques système agrégées de tous les cores.
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// @id miyukiniadmin_monitoring_system_metrics_cores_health
    /// @role data
    /// @layer operator
    /// @do store_cores_health
    //@human / @human État de santé de chaque core.
    /// @depends miyukiniadmin_monitoring_system_metrics
    pub cores_health: std::collections::HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyukiniadmin_monitoring_test_system_metrics
    /// @role test
    /// @layer operator
    /// @do verify_system_metrics_creation
    //@human / @human Test de création de métriques système.
    /// @depends miyukiniadmin_monitoring_system_metrics
    #[test]
    fn test_system_metrics() {
        let metrics = SystemMetrics {
            cores_health: std::collections::HashMap::new(),
        };
        assert!(metrics.cores_health.is_empty());
    }
}
