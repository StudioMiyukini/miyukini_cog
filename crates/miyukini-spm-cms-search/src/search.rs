//! Types et trait SearchManager.

use crate::error::SearchError;
use miyukini_kernel::Id;

/// Identifiant d'entité (alias vers Id du kernel).
pub type EntityId = Id;

/// Type d'entité (opaque, défini par le produit).
pub type EntityType = String;

/// Nom de champ (opaque, défini par le produit).
pub type FieldName = String;

/// Valeur de champ (opaque, défini par le produit).
/// Format : String pour simplicité (le produit peut sérialiser ses types).
pub type FieldValue = String;

/// Type de recherche pour un champ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    /// Recherche par égalité exacte.
    Exact,
    /// Recherche par contient (substring).
    Contains,
    /// Recherche par supérieur à.
    GreaterThan,
    /// Recherche par inférieur à.
    LessThan,
    /// Recherche par supérieur ou égal à.
    GreaterThanOrEqual,
    /// Recherche par inférieur ou égal à.
    LessThanOrEqual,
}

/// Opérateur de recherche dans une requête.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchOperator {
    /// Égalité exacte.
    Equals,
    /// Contient (substring).
    Contains,
    /// Supérieur à.
    GreaterThan,
    /// Inférieur à.
    LessThan,
    /// Supérieur ou égal à.
    GreaterThanOrEqual,
    /// Inférieur ou égal à.
    LessThanOrEqual,
    /// Commence par (prefix).
    StartsWith,
    /// Se termine par (suffix).
    EndsWith,
    /// Entre deux valeurs (inclusif) - nécessite values[0] <= values[1].
    Between,
    /// Appartient à une liste de valeurs.
    In,
}

/// Opérateur logique pour combiner des critères.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    /// ET logique (tous les critères doivent être satisfaits).
    And,
    /// OU logique (au moins un critère doit être satisfait).
    Or,
}

/// Critère de recherche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchCriterion {
    /// Nom du champ à rechercher.
    pub field_name: FieldName,
    /// Opérateur de recherche.
    pub operator: SearchOperator,
    /// Valeur de recherche (utilisée pour les opérateurs simples).
    pub value: FieldValue,
    /// Valeurs multiples (utilisées pour Between et In).
    /// Pour Between : doit contenir exactement 2 valeurs [min, max].
    /// Pour In : peut contenir une ou plusieurs valeurs.
    pub values: Vec<FieldValue>,
}

impl SearchCriterion {
    /// Crée un nouveau critère de recherche avec une valeur unique.
    pub fn new(field_name: FieldName, operator: SearchOperator, value: FieldValue) -> Self {
        Self {
            field_name,
            operator,
            value,
            values: Vec::new(),
        }
    }

    /// Crée un critère avec valeurs multiples (pour Between et In).
    pub fn with_values(field_name: FieldName, operator: SearchOperator, values: Vec<FieldValue>) -> Self {
        Self {
            field_name,
            operator,
            value: String::new(), // Non utilisé pour Between/In
            values,
        }
    }
}

/// Requête de recherche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    /// Type d'entité à rechercher (None = tous les types).
    pub entity_type: Option<EntityType>,
    /// Liste des critères de recherche.
    pub criteria: Vec<SearchCriterion>,
    /// Opérateur logique pour combiner les critères (par défaut : ET).
    pub logical_operator: LogicalOperator,
}

impl SearchQuery {
    /// Crée une nouvelle requête de recherche.
    pub fn new() -> Self {
        Self {
            entity_type: None,
            criteria: Vec::new(),
            logical_operator: LogicalOperator::And,
        }
    }

    /// Crée une requête avec un type d'entité.
    pub fn with_entity_type(entity_type: EntityType) -> Self {
        Self {
            entity_type: Some(entity_type),
            criteria: Vec::new(),
            logical_operator: LogicalOperator::And,
        }
    }

    /// Ajoute un critère à la requête.
    pub fn add_criterion(mut self, criterion: SearchCriterion) -> Self {
        self.criteria.push(criterion);
        self
    }

    /// Définit l'opérateur logique.
    pub fn with_logical_operator(mut self, op: LogicalOperator) -> Self {
        self.logical_operator = op;
        self
    }
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Champ indexé d'une entité.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedField {
    /// Nom du champ.
    pub field_name: FieldName,
    /// Valeur du champ (pour compatibilité rétroactive).
    pub field_value: FieldValue,
    /// Valeurs multiples du champ (si vide, utiliser field_value).
    /// Si non vide, le champ est multi-valeurs et la recherche matche si AU MOINS une valeur correspond.
    pub field_values: Vec<FieldValue>,
    /// Type de recherche pour ce champ.
    pub search_type: SearchType,
}

impl IndexedField {
    /// Crée un nouveau champ indexé avec une valeur unique.
    pub fn new(field_name: FieldName, field_value: FieldValue, search_type: SearchType) -> Self {
        Self {
            field_name,
            field_value,
            field_values: Vec::new(),
            search_type,
        }
    }

    /// Crée un champ indexé avec valeurs multiples.
    pub fn with_values(field_name: FieldName, field_values: Vec<FieldValue>, search_type: SearchType) -> Self {
        Self {
            field_name,
            field_value: field_values.first().cloned().unwrap_or_default(),
            field_values,
            search_type,
        }
    }

    /// Retourne toutes les valeurs du champ (field_value + field_values).
    pub fn all_values(&self) -> Vec<&FieldValue> {
        if self.field_values.is_empty() {
            vec![&self.field_value]
        } else {
            self.field_values.iter().collect()
        }
    }
}

/// Entité indexée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedEntity {
    /// Identifiant de l'entité.
    pub entity_id: EntityId,
    /// Type d'entité.
    pub entity_type: EntityType,
    /// Champs indexés.
    pub fields: Vec<IndexedField>,
}

impl IndexedEntity {
    /// Crée une nouvelle entité indexée.
    pub fn new(entity_id: EntityId, entity_type: EntityType, fields: Vec<IndexedField>) -> Self {
        Self {
            entity_id,
            entity_type,
            fields,
        }
    }
}

/// Résultat de recherche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResult {
    /// Liste des identifiants d'entités correspondantes.
    pub entity_ids: Vec<EntityId>,
    /// Nombre total de résultats.
    pub total: usize,
    /// Offset de pagination.
    pub offset: usize,
    /// Limit de pagination.
    pub limit: usize,
    /// Métadonnées explicites sur la recherche (optionnel).
    pub metadata: Option<SearchMetadata>,
}

impl SearchResult {
    /// Crée un nouveau résultat de recherche.
    pub fn new(entity_ids: Vec<EntityId>, total: usize, offset: usize, limit: usize) -> Self {
        Self {
            entity_ids,
            total,
            offset,
            limit,
            metadata: None,
        }
    }

    /// Crée un résultat avec métadonnées.
    pub fn with_metadata(
        entity_ids: Vec<EntityId>,
        total: usize,
        offset: usize,
        limit: usize,
        metadata: SearchMetadata,
    ) -> Self {
        Self {
            entity_ids,
            total,
            offset,
            limit,
            metadata: Some(metadata),
        }
    }
}

/// Métadonnées explicites sur une recherche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMetadata {
    /// Nombre de critères utilisés dans la requête.
    pub criteria_count: usize,
    /// Type d'entité recherché (si spécifié).
    pub entity_type: Option<EntityType>,
    /// Opérateur logique utilisé.
    pub logical_operator: LogicalOperator,
}

impl SearchMetadata {
    /// Crée de nouvelles métadonnées de recherche.
    pub fn new(
        criteria_count: usize,
        entity_type: Option<EntityType>,
        logical_operator: LogicalOperator,
    ) -> Self {
        Self {
            criteria_count,
            entity_type,
            logical_operator,
        }
    }
}

/// Statistiques d'index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexStats {
    /// Nombre total d'entités indexées.
    pub total_entities: usize,
    /// Nombre d'entités par type (type -> count).
    pub entities_by_type: std::collections::HashMap<EntityType, usize>,
    /// Nombre total de champs indexés (tous types confondus).
    pub total_fields: usize,
}

impl IndexStats {
    /// Crée de nouvelles statistiques d'index.
    pub fn new(
        total_entities: usize,
        entities_by_type: std::collections::HashMap<EntityType, usize>,
        total_fields: usize,
    ) -> Self {
        Self {
            total_entities,
            entities_by_type,
            total_fields,
        }
    }
}

/// Trait pour gérer l'indexation et la recherche d'entités.
pub trait SearchManager {
    /// Indexe une entité (ou met à jour si elle existe déjà).
    ///
    /// Si l'entité existe déjà pour ce type, remplace ses champs indexés.
    fn index_entity(
        &self,
        entity_id: EntityId,
        entity_type: EntityType,
        fields: Vec<IndexedField>,
    ) -> Result<(), SearchError>;

    /// Désindexe une entité.
    fn unindex_entity(
        &self,
        entity_id: EntityId,
        entity_type: EntityType,
    ) -> Result<(), SearchError>;

    /// Recherche des entités selon une requête.
    fn search(&self, query: SearchQuery, offset: usize, limit: usize) -> Result<SearchResult, SearchError>;

    /// Liste toutes les entités indexées d'un type donné.
    fn list_indexed_entities(
        &self,
        entity_type: Option<EntityType>,
        offset: usize,
        limit: usize,
    ) -> Result<SearchResult, SearchError>;

    /// Vérifie si une entité est indexée.
    fn is_indexed(&self, entity_id: EntityId, entity_type: EntityType) -> bool;

    /// Récupère les champs indexés d'une entité.
    fn get_indexed_fields(
        &self,
        entity_id: EntityId,
        entity_type: EntityType,
    ) -> Result<Vec<IndexedField>, SearchError>;

    /// Vide l'index (toutes les entités ou d'un type donné).
    fn clear_index(&self, entity_type: Option<EntityType>) -> Result<(), SearchError>;
}
