//! Module de définition de workflow de MasterButler

/// @id masterbutler_workflow_id
/// @role data
/// @layer core
/// @do represent_workflow_id
//@human / @human Identifiant unique d'un workflow.
pub type WorkflowId = String;

/// @id masterbutler_workflow
/// @role data
/// @layer core
/// @do represent_workflow
//@human / @human Définition d'un workflow avec ses étapes.
#[derive(Debug, Clone)]
pub struct Workflow {
    /// @id masterbutler_workflow_id_field
    /// @role data
    /// @layer core
    /// @do store_workflow_id
    //@human / @human Identifiant unique du workflow.
    /// @depends masterbutler_workflow
    pub id: WorkflowId,
    /// @id masterbutler_workflow_name
    /// @role data
    /// @layer core
    /// @do store_workflow_name
    //@human / @human Nom descriptif du workflow.
    /// @depends masterbutler_workflow
    pub name: String,
}

impl Workflow {
    /// @id masterbutler_workflow_new
    /// @role infrastructure
    /// @layer core
    /// @do create_workflow
    //@human / @human Crée un nouveau workflow.
    /// @depends masterbutler_workflow
    #[must_use]
    pub fn new(id: WorkflowId, name: String) -> Self {
        Self { id, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id masterbutler_workflow_test_creation
    /// @role test
    /// @layer core
    /// @do verify_workflow_creation
    //@human / @human Test de création d'un workflow.
    /// @depends masterbutler_workflow_new
    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::new("wf-1".to_string(), "Test Workflow".to_string());
        assert_eq!(workflow.id, "wf-1");
    }
}
