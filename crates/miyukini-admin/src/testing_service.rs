//! Service d'exécution de tests de MiyukiniAdmin

/// @id: miyukiniadmin_testing_service_trait
/// @role: infrastructure
/// @layer: operator
/// @human: Trait de service d'exécution de tests.
/// @do: define_testing_service_contract
pub trait TestingService {
    /// @id: miyukiniadmin_testing_service_run_tests
    /// @role: mutator
    /// @layer: operator
    /// @human: Exécute les tests techniques du système.
    /// @do: execute_system_tests
    /// @depends: miyukiniadmin_testing_service_trait
    /// Exécute les tests techniques.
    fn run_tests(&self) -> TestResults;
}

/// @id: miyukiniadmin_testing_test_results
/// @role: data
/// @layer: operator
/// @human: Résultats d'exécution des tests.
/// @do: represent_test_results
#[derive(Debug, Clone)]
pub struct TestResults {
    /// @id: miyukiniadmin_testing_test_results_passed
    /// @role: data
    /// @layer: operator
    /// @human: Nombre de tests passés.
    /// @do: store_passed_count
    /// @depends: miyukiniadmin_testing_test_results
    pub passed: u32,
    /// @id: miyukiniadmin_testing_test_results_failed
    /// @role: data
    /// @layer: operator
    /// @human: Nombre de tests échoués.
    /// @do: store_failed_count
    /// @depends: miyukiniadmin_testing_test_results
    pub failed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: miyukiniadmin_testing_test_results_creation
    /// @role: test
    /// @layer: operator
    /// @human: Test de création de résultats de tests.
    /// @do: verify_test_results_creation
    /// @depends: miyukiniadmin_testing_test_results
    #[test]
    fn test_results_creation() {
        let results = TestResults { passed: 10, failed: 0 };
        assert_eq!(results.passed, 10);
    }
}
