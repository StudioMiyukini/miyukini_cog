//! Registre en mémoire des frontières (INV-BG-5, INV-BG-8). Consultation uniquement ; pas de persistance directe (INV-BG-2).

use std::collections::HashMap;

use crate::boundary::Boundary;
use crate::crossing::CrossingRules;
use crate::metadata::BoundaryMetadata;

/// Registre des frontières et de leurs règles de franchissement. Maintenu en mémoire ; la persistance est déléguée à KindMother.
#[derive(Debug, Default)]
pub struct BoundaryRegistry {
    boundaries: HashMap<String, (Boundary, BoundaryMetadata)>,
    crossing_rules: HashMap<String, CrossingRules>,
}

impl BoundaryRegistry {
    /// Crée un registre vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre une frontière et ses métadonnées (appelé par le flux / adaptateur après réception depuis KindMother).
    pub fn register_boundary(&mut self, boundary: Boundary, meta: BoundaryMetadata) {
        let id = boundary.id.clone();
        self.boundaries.insert(id, (boundary, meta));
    }

    /// Enregistre les règles de franchissement pour une frontière.
    pub fn register_crossing_rules(&mut self, boundary_id: String, rules: CrossingRules) {
        self.crossing_rules.insert(boundary_id, rules);
    }

    /// Retourne la frontière et ses métadonnées par identifiant, ou `None` si non définie.
    #[must_use]
    pub fn get_boundary(&self, id: &str) -> Option<(&Boundary, &BoundaryMetadata)> {
        self.boundaries.get(id).map(|(b, m)| (b, m))
    }

    /// Retourne la liste des frontières avec leurs métadonnées.
    #[must_use]
    pub fn list_boundaries(&self) -> Vec<(&Boundary, &BoundaryMetadata)> {
        self.boundaries.values().map(|(b, m)| (b, m)).collect()
    }

    /// Retourne les règles de franchissement pour une frontière, ou `None` si non définies.
    #[must_use]
    pub fn get_crossing_rules(&self, boundary_id: &str) -> Option<&CrossingRules> {
        self.crossing_rules.get(boundary_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::BoundaryType;

    #[test]
    fn test_registry_register_and_get() {
        let mut reg = BoundaryRegistry::new();
        let boundary = Boundary::new(
            "b1".to_string(),
            BoundaryType::External,
            "Ext".to_string(),
        );
        let meta = BoundaryMetadata::new(
            "2026-01-01T00:00:00Z".to_string(),
            "system".to_string(),
            "Initial".to_string(),
            1,
        );
        reg.register_boundary(boundary, meta);
        let (b, m) = reg.get_boundary("b1").unwrap();
        assert_eq!(b.id, "b1");
        assert_eq!(m.version, 1);
    }

    #[test]
    fn test_registry_get_missing() {
        let reg = BoundaryRegistry::new();
        assert!(reg.get_boundary("missing").is_none());
    }
}
