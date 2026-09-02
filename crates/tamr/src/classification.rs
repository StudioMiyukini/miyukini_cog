//! Module de classification de TAMR

/// @id tamr_classification
/// @role data
/// @layer core
/// @do represent_classification
//@human / @human Classification d'une entité selon une taxonomie.
#[derive(Debug, Clone)]
pub struct Classification {
    /// @id tamr_classification_entity_id
    /// @role data
    /// @layer core
    /// @do store_entity_id
    //@human / @human Identifiant de l'entité classifiée.
    /// @depends tamr_classification
    pub entity_id: String,
    /// @id tamr_classification_taxonomy_id
    /// @role data
    /// @layer core
    /// @do store_taxonomy_id
    //@human / @human Identifiant de la taxonomie utilisée.
    /// @depends tamr_classification
    pub taxonomy_id: String,
    /// @id tamr_classification_terms
    /// @role data
    /// @layer core
    /// @do store_classification_terms
    //@human / @human Termes de classification assignés.
    /// @depends tamr_classification
    pub terms: Vec<String>,
}

/// @id tamr_classifier_trait
/// @role infrastructure
/// @layer core
/// @do define_classifier_contract
//@human / @human Trait de classification d'entités.
pub trait Classifier {
    /// @id tamr_classifier_classify
    /// @role mutator
    /// @layer core
    /// @do classify_entity
    //@human / @human Classifie une entité selon une taxonomie.
    /// @depends tamr_classifier_trait
    fn classify(
        &mut self,
        entity_id: &str,
        taxonomy_id: &str,
        terms: Vec<String>,
    ) -> Classification;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id tamr_classification_test_creation
    /// @role test
    /// @layer core
    /// @do verify_classification_creation
    //@human / @human Test de création d'une classification.
    /// @depends tamr_classification
    #[test]
    fn test_classification_creation() {
        let classification = Classification {
            entity_id: "entity-1".to_string(),
            taxonomy_id: "tax-1".to_string(),
            terms: vec!["term1".to_string()],
        };
        assert_eq!(classification.entity_id, "entity-1");
    }
}
