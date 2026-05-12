//! Erreurs unifiées du service JayCloud.

/// Erreurs renvoyées par les Opérateurs JayCloud.
#[derive(Debug, thiserror::Error)]
pub enum JayCloudError {
    /// Erreur d'I/O sur le filesystem.
    #[error("io : {0}")]
    Io(#[from] std::io::Error),

    /// Erreur de configuration.
    #[error("configuration invalide : {0}")]
    Config(String),

    /// Snapshot introuvable.
    #[error("snapshot introuvable : {0}")]
    SnapshotNotFound(String),

    /// Fichier introuvable dans un snapshot.
    #[error("fichier introuvable dans le snapshot {snapshot_id} : {file_path}")]
    FileNotFoundInSnapshot {
        /// Identifiant du snapshot consulté.
        snapshot_id: String,
        /// Chemin demandé.
        file_path: String,
    },

    /// Cible de backup introuvable.
    #[error("cible de backup introuvable : {0}")]
    TargetNotFound(String),

    /// Lien public introuvable ou expiré.
    #[error("lien public introuvable ou expiré : {0}")]
    ShareLinkInvalid(String),

    /// Authentification refusée.
    #[error("authentification refusée : {0}")]
    Auth(String),

    /// Échec d'intégrité (checksum invalide).
    #[error("intégrité : checksum invalide pour {0}")]
    IntegrityFailed(String),

    /// Erreur de sérialisation.
    #[error("sérialisation : {0}")]
    Serialization(String),

    /// Opération non encore implémentée (P2 skeleton).
    #[error("non implémenté en P2 : {0}")]
    NotImplemented(&'static str),
}
