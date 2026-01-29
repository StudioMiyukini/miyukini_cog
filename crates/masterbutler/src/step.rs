//! Module d'étapes de workflow de MasterButler

/// @id: masterbutler_step
/// @role: data
/// @layer: core
/// @human: Étape d'un workflow.
/// @do: represent_workflow_step
#[derive(Debug, Clone)]
pub struct Step {
    /// @id: masterbutler_step_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant unique de l'étape.
    /// @do: store_step_id
    /// @depends: masterbutler_step
    pub id: String,
    /// @id: masterbutler_step_name
    /// @role: data
    /// @layer: core
    /// @human: Nom descriptif de l'étape.
    /// @do: store_step_name
    /// @depends: masterbutler_step
    pub name: String,
}

/// @id: masterbutler_step_result
/// @role: data
/// @layer: core
/// @human: Résultat d'exécution d'une étape.
/// @do: represent_step_result
#[derive(Debug, Clone)]
pub enum StepResult {
    /// @id: masterbutler_step_result_success
    /// @role: data
    /// @layer: core
    /// @human: Étape exécutée avec succès.
    /// @do: represent_success_result
    /// @depends: masterbutler_step_result
    Success,
    /// @id: masterbutler_step_result_failure
    /// @role: data
    /// @layer: core
    /// @human: Étape échouée avec un message d'erreur.
    /// @do: represent_failure_result
    /// @depends: masterbutler_step_result
    Failure(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: masterbutler_step_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une étape.
    /// @do: verify_step_creation
    /// @depends: masterbutler_step
    #[test]
    fn test_step_creation() {
        let step = Step {
            id: "step-1".to_string(),
            name: "Test Step".to_string(),
        };
        assert_eq!(step.id, "step-1");
    }
}
