//! Métadonnées de traçabilité pour les définitions BorderGuard (INV-BG-8).

/// Métadonnées obligatoires pour toute frontière ou règle (traçabilité complète).
#[derive(Debug, Clone)]
pub struct BoundaryMetadata {
    /// Date de création (ex. timestamp ou ISO).
    pub created_at: String,
    /// Identifiant de l'entité ayant créé la définition.
    pub created_by: String,
    /// Justification de la définition.
    pub justification: String,
    /// Version de la définition.
    pub version: u32,
}

impl BoundaryMetadata {
    /// Crée des métadonnées de traçabilité.
    #[must_use]
    pub fn new(
        created_at: String,
        created_by: String,
        justification: String,
        version: u32,
    ) -> Self {
        Self {
            created_at,
            created_by,
            justification,
            version,
        }
    }
}
