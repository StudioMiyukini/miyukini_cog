//! Implémentation en mémoire du TaxonomyManager (pour tests et démo uniquement).
//!
//! Cette implémentation n'est pas destinée à la production. Elle sert uniquement
//! à valider le contrat et à fournir une base pour les tests et démos.
//!
//! Le produit fournira sa propre implémentation adaptée à sa stack technique.

use crate::taxonomy::{EntityId, TaxonomyError, TaxonomyId, TaxonomyManager, TermId};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use std::collections::{HashMap, HashSet};

/// Structure interne d'une taxonomie.
#[derive(Debug, Clone)]
struct Taxonomy {
    /// Nom de la taxonomie.
    #[allow(dead_code)] // Stocké pour référence future (Phase 1+)
    name: String,
    /// Liste des identifiants des termes de cette taxonomie.
    terms: Vec<TermId>,
}

impl Taxonomy {
    fn new(name: String) -> Self {
        Self {
            name,
            terms: Vec::new(),
        }
    }
}

/// Structure interne d'un terme.
#[derive(Debug, Clone)]
struct Term {
    /// Identifiant de la taxonomie à laquelle appartient ce terme.
    #[allow(dead_code)] // Stocké pour référence future (Phase 1+)
    taxonomy_id: TaxonomyId,
    /// Label du terme.
    #[allow(dead_code)] // Stocké pour référence future (Phase 1+)
    label: String,
}

impl Term {
    fn new(taxonomy_id: TaxonomyId, label: String) -> Self {
        Self {
            taxonomy_id,
            label,
        }
    }
}

/// Implémentation en mémoire du TaxonomyManager.
///
/// Stocke les taxonomies, termes et assignations dans des HashMaps.
/// Utilise UuidIdGenerator du kernel.
/// Pas de persistance : données perdues à l'arrêt.
pub struct MemoryTaxonomyManager {
    taxonomies: HashMap<TaxonomyId, Taxonomy>,
    terms: HashMap<TermId, Term>,
    /// Assignations terme → entités (terme → set d'entités)
    term_to_entities: HashMap<TermId, HashSet<EntityId>>,
    /// Assignations entité → termes (entité → set de termes)
    entity_to_terms: HashMap<EntityId, HashSet<TermId>>,
    id_generator: UuidIdGenerator,
}

impl MemoryTaxonomyManager {
    /// Crée un nouveau gestionnaire en mémoire.
    pub fn new() -> Self {
        Self {
            taxonomies: HashMap::new(),
            terms: HashMap::new(),
            term_to_entities: HashMap::new(),
            entity_to_terms: HashMap::new(),
            id_generator: UuidIdGenerator::new(),
        }
    }
}

impl Default for MemoryTaxonomyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyManager for MemoryTaxonomyManager {
    fn create_taxonomy(&mut self, name: String) -> TaxonomyId {
        let taxonomy_id = self.id_generator.generate();
        let taxonomy = Taxonomy::new(name);
        self.taxonomies.insert(taxonomy_id, taxonomy);
        taxonomy_id
    }

    fn add_term(&mut self, taxonomy: TaxonomyId, label: String) -> Result<TermId, TaxonomyError> {
        // Vérifier que la taxonomie existe
        if !self.taxonomies.contains_key(&taxonomy) {
            return Err(TaxonomyError::TaxonomyNotFound);
        }

        // Créer le terme
        let term_id = self.id_generator.generate();
        let term = Term::new(taxonomy, label);
        self.terms.insert(term_id, term);

        // Ajouter le terme à la taxonomie
        if let Some(taxonomy_ref) = self.taxonomies.get_mut(&taxonomy) {
            taxonomy_ref.terms.push(term_id);
        }

        Ok(term_id)
    }

    fn assign_term(&mut self, term: TermId, entity: EntityId) -> Result<(), TaxonomyError> {
        // Vérifier que le terme existe
        if !self.terms.contains_key(&term) {
            return Err(TaxonomyError::TermNotFound);
        }

        // Ajouter l'assignation terme → entité
        self.term_to_entities
            .entry(term)
            .or_insert_with(HashSet::new)
            .insert(entity);

        // Ajouter l'assignation entité → terme
        self.entity_to_terms
            .entry(entity)
            .or_insert_with(HashSet::new)
            .insert(term);

        Ok(())
    }

    fn unassign_term(&mut self, term: TermId, entity: EntityId) -> Result<(), TaxonomyError> {
        // Vérifier que le terme existe
        if !self.terms.contains_key(&term) {
            return Err(TaxonomyError::TermNotFound);
        }

        // Retirer l'assignation terme → entité
        if let Some(entities) = self.term_to_entities.get_mut(&term) {
            entities.remove(&entity);
            // Si le set est vide, on peut le retirer (optionnel, pour économiser la mémoire)
            if entities.is_empty() {
                self.term_to_entities.remove(&term);
            }
        }

        // Retirer l'assignation entité → terme
        if let Some(terms) = self.entity_to_terms.get_mut(&entity) {
            terms.remove(&term);
            // Si le set est vide, on peut le retirer (optionnel, pour économiser la mémoire)
            if terms.is_empty() {
                self.entity_to_terms.remove(&entity);
            }
        }

        Ok(())
    }

    fn terms_for_entity(&self, entity: EntityId) -> Vec<TermId> {
        self.entity_to_terms
            .get(&entity)
            .map(|terms| terms.iter().copied().collect())
            .unwrap_or_default()
    }

    fn entities_for_term(&self, term: TermId) -> Vec<EntityId> {
        self.term_to_entities
            .get(&term)
            .map(|entities| entities.iter().copied().collect())
            .unwrap_or_default()
    }
}

impl MemoryTaxonomyManager {
    /// Supprime une taxonomie et tous ses termes et assignations.
    ///
    /// Cette méthode n'est pas dans le trait car elle est spécifique à l'implémentation mémoire.
    /// En production, le produit gérera la suppression selon ses propres règles.
    #[allow(dead_code)] // Utilisée dans les tests
    pub fn remove_taxonomy(&mut self, taxonomy_id: TaxonomyId) -> Result<(), TaxonomyError> {
        // Vérifier que la taxonomie existe
        let taxonomy = match self.taxonomies.remove(&taxonomy_id) {
            Some(t) => t,
            None => return Err(TaxonomyError::TaxonomyNotFound),
        };

        // Supprimer tous les termes de la taxonomie
        for term_id in taxonomy.terms {
            // Retirer le terme
            self.terms.remove(&term_id);

            // Retirer toutes les assignations de ce terme
            if let Some(entities) = self.term_to_entities.remove(&term_id) {
                // Retirer aussi les références inverse (entité → terme)
                for entity in entities {
                    if let Some(terms) = self.entity_to_terms.get_mut(&entity) {
                        terms.remove(&term_id);
                        if terms.is_empty() {
                            self.entity_to_terms.remove(&entity);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
