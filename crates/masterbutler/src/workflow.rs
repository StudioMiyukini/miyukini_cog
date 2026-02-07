//! Module de définition de workflow de MasterButler

/// @id: masterbutler_workflow_id
/// @role: data
/// @layer: core
/// @human: Identifiant unique d'un workflow.
/// @do: represent_workflow_id
pub type WorkflowId = String;

/// @id: masterbutler_workflow
/// @role: data
/// @layer: core
/// @human: Définition d'un workflow avec ses étapes.
/// @do: represent_workflow
#[derive(Debug, Clone)]
pub struct Workflow {
    /// @id: masterbutler_workflow_id_field
    /// @role: data
    /// @layer: core
    /// @human: Identifiant unique du workflow.
    /// @do: store_workflow_id
    /// @depends: masterbutler_workflow
    pub id: WorkflowId,
    /// @id: masterbutler_workflow_name
    /// @role: data
    /// @layer: core
    /// @human: Nom descriptif du workflow.
    /// @do: store_workflow_name
    /// @depends: masterbutler_workflow
    pub name: String,
}

impl Workflow {
    /// @id: masterbutler_workflow_new
    /// @role: infrastructure
    /// @layer: core
    /// @human: Crée un nouveau workflow.
    /// @do: create_workflow
    /// @depends: masterbutler_workflow
    #[must_use] 
    pub fn new(id: WorkflowId, name: String) -> Self {
        Self { id, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: masterbutler_workflow_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'un workflow.
    /// @do: verify_workflow_creation
    /// @depends: masterbutler_workflow_new
    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("wf-1".to_string(), "Test Workflow".to_string());
        assert_eq!(workflow.id, "wf-1");
    }
}
