//! @id     masterbutler_orchestrator
//! @do     definir_contrat_orchestration
//! @role   orchestration
//! @layer  core
//! @human  masterbutler — contrat d'orchestration ; aucune exécution réelle dans le Core.
//!
//! Module d'orchestration de MasterButler.
//!
//! Selon D1 / C-31, les Cores décident et n'exécutent pas : l'exécution des étapes
//! revient aux opérateurs (Douane, Vigies) et au Kernel. Tant que cette délégation
//! n'est pas câblée, [`DefaultOrchestrator`] refuse explicitement (C-14, C-12) au lieu
//! de renvoyer un faux succès (constat K-15).

use crate::{Step, StepResult, Workflow};

/// @id masterbutler_orchestrator_trait
/// @role infrastructure
/// @layer core
/// @do define_orchestrator_contract
//@human / @human Trait d'orchestration d'exécution de workflows.
pub trait Orchestrator {
    /// @id masterbutler_orchestrator_execute
    /// @role mutator
    /// @layer core
    /// @do execute_workflow
    //@human / @human Exécute un workflow avec ses étapes.
    /// @depends masterbutler_orchestrator_trait
    fn execute(
        &mut self,
        workflow: &Workflow,
        steps: &[Step],
    ) -> Result<Vec<StepResult>, OrchestrationError>;
}

/// @id masterbutler_orchestration_error
/// @role error
/// @layer core
/// @do represent_orchestration_error
//@human / @human Erreur d'orchestration (refus explicite, fail-closed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrchestrationError {
    /// @id masterbutler_orchestration_error_non_implemente
    /// @role error
    /// @layer core
    /// @do represent_not_implemented
    //@human / @human Exécution non implémentée : la délégation aux opérateurs / Kernel
    //@human n'existe pas encore (D1). Aucune étape n'a été exécutée.
    /// @depends masterbutler_orchestration_error
    NonImplemente(String),
}

impl std::fmt::Display for OrchestrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrchestrationError::NonImplemente(msg) => {
                write!(f, "orchestration non implémentée : {msg}")
            }
        }
    }
}

impl std::error::Error for OrchestrationError {}

/// Orchestrateur par défaut : refuse toute exécution (K-15, C-14).
///
/// L'exécution réelle sera déléguée aux opérateurs et au Kernel (D1, lot C du plan).
/// En attendant, `execute` renvoie toujours [`OrchestrationError::NonImplemente`]
/// et n'exécute aucune étape.
#[derive(Debug, Default)]
pub struct DefaultOrchestrator;

impl DefaultOrchestrator {
    /// Crée un orchestrateur.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Orchestrator for DefaultOrchestrator {
    fn execute(
        &mut self,
        workflow: &Workflow,
        steps: &[Step],
    ) -> Result<Vec<StepResult>, OrchestrationError> {
        Err(OrchestrationError::NonImplemente(format!(
            "workflow « {} » ({} étape(s)) : l'exécution est réservée aux opérateurs et au Kernel (D1)",
            workflow.id,
            steps.len()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id masterbutler_orchestrator_test_workflow_execution
    /// @role test
    /// @layer core
    /// @do verify_workflow_execution_structure
    //@human / @human Test d'exécution d'un workflow (structure de base).
    /// @depends masterbutler_orchestrator_trait
    #[test]
    fn test_workflow_execution_structure() {
        let workflow = Workflow::new("wf-1".to_string(), "Test".to_string());
        let steps = [Step {
            id: "step-1".to_string(),
            name: "Step 1".to_string(),
        }];
        assert_eq!(workflow.id, "wf-1");
        assert_eq!(steps.len(), 1);
    }

    /// @id masterbutler_orchestrator_test_refus_explicite
    /// @role test
    /// @layer core
    /// @do verify_default_orchestrator_refuses
    //@human / @human L'orchestrateur par défaut refuse au lieu d'annoncer un faux succès (K-15).
    /// @depends masterbutler_orchestrator_execute
    #[test]
    fn test_default_orchestrator_refuse_explicitement() {
        let mut orch = DefaultOrchestrator::new();
        let workflow = Workflow::new("wf-1".to_string(), "Test".to_string());
        let steps = [
            Step {
                id: "s1".to_string(),
                name: "Step 1".to_string(),
            },
            Step {
                id: "s2".to_string(),
                name: "Step 2".to_string(),
            },
        ];
        let result = orch.execute(&workflow, &steps);
        assert!(matches!(result, Err(OrchestrationError::NonImplemente(_))));
    }

    /// @id masterbutler_orchestrator_test_refus_workflow_vide
    /// @role test
    /// @layer core
    /// @do verify_refusal_on_empty_workflow
    //@human / @human Même un workflow sans étape est refusé (pas de succès vide).
    /// @depends masterbutler_orchestrator_execute
    #[test]
    fn test_default_orchestrator_refuse_workflow_vide() {
        let mut orch = DefaultOrchestrator::new();
        let workflow = Workflow::new("wf-vide".to_string(), "Vide".to_string());
        let err = orch.execute(&workflow, &[]).unwrap_err();
        assert!(err.to_string().contains("non implémentée"));
    }
}
