//! Génération d'identifiants. Type `Id` opaque : format interne non garanti,
//! ne pas persister ou interpréter sans passer par les APIs du produit.

use std::fmt;

/// Identifiant opaque. Le format interne n'est pas garanti et ne doit pas
/// être persisté ou interprété sans passer par les APIs du produit.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Id {
    inner: uuid::Uuid,
}

impl Id {
    /// Parse une chaîne en `Id`. Le format supporté dépend de l'implémentation
    /// et peut changer entre versions.
    pub fn parse(s: &str) -> Result<Id, IdParseError> {
        uuid::Uuid::parse_str(s).map(|u| Id { inner: u }).map_err(|_| IdParseError)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.inner)
    }
}

/// Erreur de parsing d'un `Id`.
#[derive(Debug)]
pub struct IdParseError;

impl fmt::Display for IdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid id format")
    }
}

impl std::error::Error for IdParseError {}

/// Contrat : génération d'identifiants.
pub trait IdGenerator {
    fn generate(&self) -> Id;
}

/// Implémentation par défaut : UUID v4. Minimal et remplaçable.
#[derive(Debug, Default)]
pub struct UuidIdGenerator;

impl UuidIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl IdGenerator for UuidIdGenerator {
    fn generate(&self) -> Id {
        Id { inner: uuid::Uuid::new_v4() }
    }
}
