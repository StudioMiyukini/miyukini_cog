//! Implémentation KindMother Client (feature `kindmother-only`) pour JayManga.
//!
//! Utilise le client KindMother (IPC/gRPC) avec singleton global.
//! À implémenter quand la migration vers KindMother Client sera activée.

/// Erreur de la couche persistance JayManga (KindMother Client).
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayMangaDb client error: {}", self.0)
    }
}

impl std::error::Error for DbError {}

/// Base de données JayManga (KindMother Client).
/// Placeholder — structure identique à la version legacy-sqlite.
pub struct JayMangaDb;

impl JayMangaDb {
    /// Ouvre la connexion au client KindMother.
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        Err(DbError("KindMother Client not yet implemented for JayManga. Use legacy-sqlite.".to_string()))
    }
}
