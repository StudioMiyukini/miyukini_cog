//! Service d'exécution de tests de MiyukiniAdmin

/// @id miyukiniadmin_testing_service_trait
/// @role infrastructure
/// @layer operator
/// @do define_testing_service_contract
//@human / @human Trait de service d'exécution de tests.
pub trait TestingService {
    /// @id miyukiniadmin_testing_service_run_tests
    /// @role mutator
    /// @layer operator
    /// @do execute_system_tests
    //@human / @human Exécute les tests techniques du système.
    /// @depends miyukiniadmin_testing_service_trait
    /// Exécute les tests techniques.
    fn run_tests(&self) -> TestResults;
}

/// @id miyukiniadmin_testing_test_results
/// @role data
/// @layer operator
/// @do represent_test_results
//@human / @human Résultats d'exécution des tests.
#[derive(Debug, Clone)]
pub struct TestResults {
    /// @id miyukiniadmin_testing_test_results_passed
    /// @role data
    /// @layer operator
    /// @do store_passed_count
    //@human / @human Nombre de tests passés.
    /// @depends miyukiniadmin_testing_test_results
    pub passed: u32,
    /// @id miyukiniadmin_testing_test_results_failed
    /// @role data
    /// @layer operator
    /// @do store_failed_count
    //@human / @human Nombre de tests échoués.
    /// @depends miyukiniadmin_testing_test_results
    pub failed: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyukiniadmin_testing_test_results_creation
    /// @role test
    /// @layer operator
    /// @do verify_test_results_creation
    //@human / @human Test de création de résultats de tests.
    /// @depends miyukiniadmin_testing_test_results
    #[test]
    fn test_results_creation() {
        let results = TestResults {
            passed: 10,
            failed: 0,
        };
        assert_eq!(results.passed, 10);
    }
}
