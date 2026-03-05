//! Module de définition de frontière de BorderGuard

/// @id: borderguard_boundary_type
/// @role: data
/// @layer: core
/// @human: Type de frontière du système.
/// @do: represent_boundary_type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundaryType {
    /// @id: borderguard_boundary_type_external
    /// @role: data
    /// @layer: core
    /// @human: Frontière externe - sépare le système de l'environnement externe.
    /// @do: represent_external_boundary
    /// @depends: borderguard_boundary_type
    External,
    /// @id: borderguard_boundary_type_internal
    /// @role: data
    /// @layer: core
    /// @human: Frontière interne - sépare les composants internes du système.
    /// @do: represent_internal_boundary
    /// @depends: borderguard_boundary_type
    Internal,
    /// @id: borderguard_boundary_type_integration
    /// @role: data
    /// @layer: core
    /// @human: Frontière d'intégration - sépare les intégrations externes.
    /// @do: represent_integration_boundary
    /// @depends: borderguard_boundary_type
    Integration,
}

/// @id: borderguard_boundary
/// @role: data
/// @layer: core
/// @human: Frontière du système avec son type et son identifiant.
/// @do: represent_boundary
#[derive(Debug, Clone)]
pub struct Boundary {
    /// @id: borderguard_boundary_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant unique de la frontière.
    /// @do: store_boundary_id
    /// @depends: borderguard_boundary
    pub id: String,
    /// @id: borderguard_boundary_type
    /// @role: data
    /// @layer: core
    /// @human: Type de la frontière.
    /// @do: store_boundary_type
    /// @depends: borderguard_boundary
    pub boundary_type: BoundaryType,
    /// @id: borderguard_boundary_name
    /// @role: data
    /// @layer: core
    /// @human: Nom descriptif de la frontière.
    /// @do: store_boundary_name
    /// @depends: borderguard_boundary
    pub name: String,
}

impl Boundary {
    /// @id: borderguard_boundary_new
    /// @role: infrastructure
    /// @layer: core
    /// @human: Crée une nouvelle frontière.
    /// @do: create_boundary
    /// @depends: borderguard_boundary
    #[must_use]
    pub fn new(id: String, boundary_type: BoundaryType, name: String) -> Self {
        Self {
            id,
            boundary_type,
            name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: borderguard_boundary_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une frontière.
    /// @do: verify_boundary_creation
    /// @depends: borderguard_boundary_new
    #[test]
    fn test_boundary_creation() {
        let boundary = Boundary::new(
            "boundary-1".to_string(),
            BoundaryType::External,
            "External Boundary".to_string(),
        );
        assert_eq!(boundary.id, "boundary-1");
        assert_eq!(boundary.boundary_type, BoundaryType::External);
    }
}
