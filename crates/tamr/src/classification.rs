//! Module de classification de TAMR

/// @id: tamr_classification
/// @role: data
/// @layer: core
/// @human: Classification d'une entité selon une taxonomie.
/// @do: represent_classification
#[derive(Debug, Clone)]
pub struct Classification {
    /// @id: tamr_classification_entity_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de l'entité classifiée.
    /// @do: store_entity_id
    /// @depends: tamr_classification
    pub entity_id: String,
    /// @id: tamr_classification_taxonomy_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de la taxonomie utilisée.
    /// @do: store_taxonomy_id
    /// @depends: tamr_classification
    pub taxonomy_id: String,
    /// @id: tamr_classification_terms
    /// @role: data
    /// @layer: core
    /// @human: Termes de classification assignés.
    /// @do: store_classification_terms
    /// @depends: tamr_classification
    pub terms: Vec<String>,
}

/// @id: tamr_classifier_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de classification d'entités.
/// @do: define_classifier_contract
pub trait Classifier {
    /// @id: tamr_classifier_classify
    /// @role: mutator
    /// @layer: core
    /// @human: Classifie une entité selon une taxonomie.
    /// @do: classify_entity
    /// @depends: tamr_classifier_trait
    fn classify(&mut self, entity_id: &str, taxonomy_id: &str, terms: Vec<String>)
        -> Classification;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: tamr_classification_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une classification.
    /// @do: verify_classification_creation
    /// @depends: tamr_classification
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
