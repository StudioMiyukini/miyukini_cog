//! Module de gestion de taxonomies de TAMR

/// @id: tamr_taxonomy
/// @role: data
/// @layer: core
/// @human: Taxonomie avec son identifiant et ses termes.
/// @do: represent_taxonomy
#[derive(Debug, Clone)]
pub struct Taxonomy {
    /// @id: tamr_taxonomy_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant unique de la taxonomie.
    /// @do: store_taxonomy_id
    /// @depends: tamr_taxonomy
    pub id: String,
    /// @id: tamr_taxonomy_name
    /// @role: data
    /// @layer: core
    /// @human: Nom de la taxonomie.
    /// @do: store_taxonomy_name
    /// @depends: tamr_taxonomy
    pub name: String,
    /// @id: tamr_taxonomy_terms
    /// @role: data
    /// @layer: core
    /// @human: Termes de la taxonomie.
    /// @do: store_taxonomy_terms
    /// @depends: tamr_taxonomy
    pub terms: Vec<String>,
}

/// @id: tamr_taxonomy_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion des taxonomies.
/// @do: define_taxonomy_manager_contract
pub trait TaxonomyManager {
    /// @id: tamr_taxonomy_manager_get
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère une taxonomie par son identifiant.
    /// @do: get_taxonomy_by_id
    /// @depends: tamr_taxonomy_manager_trait
    fn get(&self, taxonomy_id: &str) -> Option<&Taxonomy>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: tamr_taxonomy_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une taxonomie.
    /// @do: verify_taxonomy_creation
    /// @depends: tamr_taxonomy
    #[test]
    fn test_taxonomy_creation() {
        let taxonomy = Taxonomy {
            id: "tax-1".to_string(),
            name: "Test Taxonomy".to_string(),
            terms: vec!["term1".to_string(), "term2".to_string()],
        };
        assert_eq!(taxonomy.id, "tax-1");
        assert_eq!(taxonomy.terms.len(), 2);
    }
}
