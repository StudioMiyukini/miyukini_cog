//! Module d'optimisation de LogisticsSteward

/// @id logisticssteward_optimization
/// @role data
/// @layer core
/// @do represent_optimization_result
//@human / @human Résultat d'optimisation avec recommandations.
#[derive(Debug, Clone)]
pub struct Optimization {
    /// @id logisticssteward_optimization_recommendations
    /// @role data
    /// @layer core
    /// @do store_optimization_recommendations
    //@human / @human Recommandations d'optimisation.
    /// @depends logisticssteward_optimization
    pub recommendations: Vec<String>,
}

/// @id logisticssteward_optimizer_trait
/// @role infrastructure
/// @layer core
/// @do define_optimizer_contract
//@human / @human Trait d'optimisation des ressources.
pub trait Optimizer {
    /// @id logisticssteward_optimizer_optimize
    /// @role infrastructure
    /// @layer core
    /// @do optimize_resources
    //@human / @human Optimise l'utilisation des ressources.
    /// @depends logisticssteward_optimizer_trait
    fn optimize(&self) -> Optimization;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id logisticssteward_optimization_test_creation
    /// @role test
    /// @layer core
    /// @do verify_optimization_creation
    //@human / @human Test de création d'une optimisation.
    /// @depends logisticssteward_optimization
    #[test]
    fn test_optimization_creation() {
        let optimization = Optimization {
            recommendations: vec!["rec1".to_string()],
        };
        assert_eq!(optimization.recommendations.len(), 1);
    }
}
