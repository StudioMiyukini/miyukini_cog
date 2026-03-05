//! Module d'orchestration d'exécution de MasterButler

use crate::{Step, StepResult, Workflow};

/// @id: masterbutler_orchestrator_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait d'orchestration d'exécution de workflows.
/// @do: define_orchestrator_contract
pub trait Orchestrator {
    /// @id: masterbutler_orchestrator_execute
    /// @role: mutator
    /// @layer: core
    /// @human: Exécute un workflow avec ses étapes.
    /// @do: execute_workflow
    /// @depends: masterbutler_orchestrator_trait
    fn execute(&mut self, workflow: &Workflow, steps: &[Step]) -> Vec<StepResult>;
}

/// Orchestrateur par défaut : exécute les étapes dans l'ordre, une par une (sans délégation réelle).
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
    fn execute(&mut self, _workflow: &Workflow, steps: &[Step]) -> Vec<StepResult> {
        steps.iter().map(|_step| StepResult::Success).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: masterbutler_orchestrator_test_workflow_execution
    /// @role: test
    /// @layer: core
    /// @human: Test d'exécution d'un workflow (structure de base).
    /// @do: verify_workflow_execution_structure
    /// @depends: masterbutler_orchestrator_trait
    #[test]
    fn test_workflow_execution_structure() {
        let workflow = Workflow::new("wf-1".to_string(), "Test".to_string());
        let steps = vec![Step {
            id: "step-1".to_string(),
            name: "Step 1".to_string(),
        }];
        assert_eq!(workflow.id, "wf-1");
        assert_eq!(steps.len(), 1);
    }

    #[test]
    fn test_default_orchestrator() {
        use super::{DefaultOrchestrator, Orchestrator, StepResult};
        let mut orch = DefaultOrchestrator::new();
        let workflow = Workflow::new("wf-1".to_string(), "Test".to_string());
        let steps = vec![
            Step {
                id: "s1".to_string(),
                name: "Step 1".to_string(),
            },
            Step {
                id: "s2".to_string(),
                name: "Step 2".to_string(),
            },
        ];
        let results = orch.execute(&workflow, &steps);
        assert_eq!(results.len(), 2);
        assert!(matches!(results[0], StepResult::Success));
        assert!(matches!(results[1], StepResult::Success));
    }
}
