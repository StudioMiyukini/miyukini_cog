//! Event bus simple -- pub/sub synchrone.
//!
//! <!-- @id: MGE-Core-Event @do: event-bus @role: back-end @layer: 1 @human: miyuk -->
//!
//! Bus d'evenements synchrone type-erased. Pas de threads, pas de locks.
//! Les evenements sont publies dans des queues par `TypeId` et consommes
//! via [`EventBus::drain`] a la fin de chaque tick.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Trait marqueur pour les evenements de jeu.
///
/// Tout type qui implemente `Event` peut etre publie et consomme via l'[`EventBus`].
/// Les contraintes `Send + Sync + 'static` permettent un usage futur multi-thread
/// sans casser l'API, meme si le bus est aujourd'hui synchrone.
pub trait Event: Send + Sync + 'static {}

/// Bus d'evenements synchrone -- pas de threads, pas de locks.
///
/// Chaque type d'evenement a sa propre queue. Les evenements sont publies
/// avec [`EventBus::publish`] et consommes avec [`EventBus::drain`].
/// En fin de tick, [`EventBus::clear_all`] nettoie toutes les queues restantes.
pub struct EventBus {
    queues: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
}

impl EventBus {
    /// Cree un bus d'evenements vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
        }
    }

    /// Publie un evenement (sera visible au prochain `drain`).
    pub fn publish<E: Event>(&mut self, event: E) {
        self.queues
            .entry(TypeId::of::<E>())
            .or_default()
            .push(Box::new(event));
    }

    /// Lit tous les evenements d'un type -- consomme la queue.
    ///
    /// Apres un appel a `drain`, la queue pour ce type est vide.
    /// Les evenements sont retournes dans l'ordre de publication.
    pub fn drain<E: Event>(&mut self) -> Vec<E> {
        self.queues
            .remove(&TypeId::of::<E>())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|b| b.downcast::<E>().ok().map(|e| *e))
            .collect()
    }

    /// Nombre d'evenements en attente pour un type donne.
    #[must_use]
    pub fn count<E: Event>(&self) -> usize {
        self.queues
            .get(&TypeId::of::<E>())
            .map_or(0, Vec::len)
    }

    /// Nettoie toutes les queues (fin de tick).
    pub fn clear_all(&mut self) {
        self.queues.clear();
    }

    /// Retourne `true` si aucun evenement n'est en attente.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.queues.values().all(Vec::is_empty)
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("queue_count", &self.queues.len())
            .finish()
    }
}
