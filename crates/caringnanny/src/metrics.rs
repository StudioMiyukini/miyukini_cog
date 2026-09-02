//! Module de collecte de métriques de CaringNanny

/// @id caringnanny_metrics
/// @role data
/// @layer core
/// @do represent_component_metrics
//@human / @human Métriques collectées pour un composant.
#[derive(Debug, Clone, Default)]
pub struct Metrics {
    /// @id caringnanny_metrics_event_count
    /// @role data
    /// @layer core
    /// @do store_event_count
    //@human / @human Nombre d'événements observés.
    /// @depends caringnanny_metrics
    pub event_count: u64,
    /// @id caringnanny_metrics_error_count
    /// @role data
    /// @layer core
    /// @do store_error_count
    //@human / @human Nombre d'erreurs observées.
    /// @depends caringnanny_metrics
    pub error_count: u64,
}

/// @id caringnanny_metrics_collector_trait
/// @role infrastructure
/// @layer core
/// @do define_metrics_collector_contract
//@human / @human Trait de collecte de métriques.
pub trait MetricsCollector {
    /// @id caringnanny_metrics_collector_get
    /// @role accessor
    /// @layer core
    /// @do get_component_metrics
    //@human / @human Récupère les métriques d'un composant.
    /// @depends caringnanny_metrics_collector_trait
    fn get(&self, component: &str) -> Metrics;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id caringnanny_metrics_test_default
    /// @role test
    /// @layer core
    /// @do verify_default_metrics
    //@human / @human Test des métriques par défaut.
    /// @depends caringnanny_metrics
    #[test]
    fn test_default_metrics() {
        let metrics = Metrics::default();
        assert_eq!(metrics.event_count, 0);
        assert_eq!(metrics.error_count, 0);
    }
}
