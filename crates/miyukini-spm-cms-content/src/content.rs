//! Types et trait ContentManager.

use crate::error::ContentError;
use crate::relation::ContentRelation;
use crate::status::ContentStatus;
use crate::version::ContentVersion;
use miyukini_kernel::Id;
use std::time::SystemTime;

/// Identifiant de contenu (alias vers Id du kernel).
pub type ContentId = Id;

/// Contenu (entité principale).
#[derive(Debug, Clone)]
pub struct Content {
    /// Identifiant unique (fourni par le kernel).
    pub id: Id,
    /// Type de contenu (défini par le produit, ex. "page", "article", "bloc").
    pub content_type: String,
    /// Statut fonctionnel.
    pub status: ContentStatus,
    /// Date de création (fournie par le kernel).
    pub created_at: SystemTime,
    /// Date de modification (fournie par le kernel).
    pub updated_at: SystemTime,
    /// Métadonnées (structure définie par le produit, format opaque).
    /// Représentation sérialisée, format défini par le produit.
    pub metadata: Vec<u8>,
}

impl Content {
    /// Crée un nouveau contenu (utilisé en interne par les implémentations).
    pub fn new(
        id: Id,
        content_type: String,
        status: ContentStatus,
        created_at: SystemTime,
        updated_at: SystemTime,
        metadata: Vec<u8>,
    ) -> Self {
        Self {
            id,
            content_type,
            status,
            created_at,
            updated_at,
            metadata,
        }
    }
}

/// Données d'entrée pour créer un contenu.
#[derive(Debug, Clone)]
pub struct ContentInput {
    /// Type de contenu (défini par le produit).
    pub content_type: String,
    /// Statut initial (par défaut : Draft).
    pub status: Option<ContentStatus>,
    /// Métadonnées (structure définie par le produit, format opaque).
    pub metadata: Vec<u8>,
}

impl ContentInput {
    /// Crée une nouvelle entrée de contenu.
    pub fn new(content_type: String, metadata: Vec<u8>) -> Self {
        Self {
            content_type,
            status: None,
            metadata,
        }
    }

    /// Crée une nouvelle entrée avec statut explicite.
    pub fn with_status(content_type: String, status: ContentStatus, metadata: Vec<u8>) -> Self {
        Self {
            content_type,
            status: Some(status),
            metadata,
        }
    }
}

/// Mises à jour partielles d'un contenu.
#[derive(Debug, Clone, Default)]
pub struct ContentUpdates {
    /// Nouveau type de contenu (si modifié).
    pub content_type: Option<String>,
    /// Nouveau statut (si modifié).
    pub status: Option<ContentStatus>,
    /// Nouvelles métadonnées (si modifiées).
    pub metadata: Option<Vec<u8>>,
}

impl ContentUpdates {
    /// Crée des mises à jour vides.
    pub fn new() -> Self {
        Self::default()
    }

    /// Définit le type de contenu.
    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Définit le statut.
    pub fn with_status(mut self, status: ContentStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Définit les métadonnées.
    pub fn with_metadata(mut self, metadata: Vec<u8>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Filtres pour lister des contenus (définis par le produit).
/// Type opaque : le produit définit les critères de filtrage.
/// Pour une implémentation simple, on peut utiliser une structure avec des champs optionnels.
#[derive(Debug, Clone, Default)]
pub struct ContentFilters {
    /// Filtrer par type de contenu.
    pub content_type: Option<String>,
    /// Filtrer par statut.
    pub status: Option<ContentStatus>,
    /// Filtrer par date de création (min).
    pub created_after: Option<SystemTime>,
    /// Filtrer par date de création (max).
    pub created_before: Option<SystemTime>,
}

impl ContentFilters {
    /// Crée des filtres vides (liste tous les contenus).
    pub fn new() -> Self {
        Self::default()
    }

    /// Filtre par type de contenu.
    pub fn with_content_type(mut self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Filtre par statut.
    pub fn with_status(mut self, status: ContentStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Filtre par date de création (min).
    pub fn with_created_after(mut self, created_after: SystemTime) -> Self {
        self.created_after = Some(created_after);
        self
    }

    /// Filtre par date de création (max).
    pub fn with_created_before(mut self, created_before: SystemTime) -> Self {
        self.created_before = Some(created_before);
        self
    }
}

/// Résultat de liste avec pagination.
#[derive(Debug, Clone)]
pub struct ContentListResult {
    /// Liste des contenus.
    pub items: Vec<Content>,
    /// Nombre total de contenus correspondant aux filtres (sans pagination).
    pub total: usize,
    /// Offset utilisé.
    pub offset: usize,
    /// Limite utilisée.
    pub limit: usize,
}

impl ContentListResult {
    /// Crée un résultat de liste.
    pub fn new(items: Vec<Content>, total: usize, offset: usize, limit: usize) -> Self {
        Self {
            items,
            total,
            offset,
            limit,
        }
    }
}

/// Trait principal : gestion des contenus.
///
/// Le produit implémente ce trait pour adapter le contrat fonctionnel
/// vers sa stack technique (DB, sérialisation, etc.).
pub trait ContentManager {
    /// Crée un contenu.
    ///
    /// L'identifiant est généré par le kernel (IdGenerator).
    /// Les dates sont fournies par le kernel (Clock).
    fn create_content(&self, input: ContentInput) -> Result<Id, ContentError>;

    /// Lit un contenu par son identifiant.
    ///
    /// Option : version spécifique (si versioning activé).
    fn get_content(&self, id: Id) -> Result<Content, ContentError>;

    /// Modifie un contenu (mise à jour partielle).
    ///
    /// La date de modification est mise à jour automatiquement (Clock).
    /// Option : créer une nouvelle version si versioning activé.
    fn update_content(
        &self,
        id: Id,
        updates: ContentUpdates,
        create_version: bool,
    ) -> Result<(), ContentError>;

    /// Supprime un contenu.
    ///
    /// `soft = true` : archivage (changement de statut).
    /// `soft = false` : suppression définitive.
    fn delete_content(&self, id: Id, soft: bool) -> Result<(), ContentError>;

    /// Liste des contenus avec filtres et pagination.
    ///
    /// Les filtres et le tri sont définis par le produit.
    fn list_contents(
        &self,
        filters: ContentFilters,
        offset: usize,
        limit: usize,
    ) -> Result<ContentListResult, ContentError>;

    /// Ajoute une relation entre deux contenus.
    fn add_relation(
        &self,
        source_id: Id,
        relation_type: String,
        target_id: Id,
    ) -> Result<(), ContentError>;

    /// Supprime une relation entre deux contenus.
    fn remove_relation(
        &self,
        source_id: Id,
        relation_type: String,
        target_id: Id,
    ) -> Result<(), ContentError>;

    /// Liste les relations d'un contenu.
    fn list_relations(&self, content_id: Id) -> Result<Vec<ContentRelation>, ContentError>;

    /// Crée une version d'un contenu (snapshot).
    ///
    /// Retourne l'identifiant de la version créée.
    fn create_version(&self, content_id: Id) -> Result<Id, ContentError>;

    /// Lit une version spécifique d'un contenu.
    fn get_version(&self, content_id: Id, version_id: Id) -> Result<ContentVersion, ContentError>;

    /// Liste les versions d'un contenu.
    fn list_versions(&self, content_id: Id) -> Result<Vec<ContentVersion>, ContentError>;

    /// Restaure une version (crée une nouvelle version avec le snapshot restauré).
    ///
    /// Ne modifie pas l'historique : crée une nouvelle version.
    fn restore_version(&self, content_id: Id, version_id: Id) -> Result<(), ContentError>;
}
