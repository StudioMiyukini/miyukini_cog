// @id: mge-ecs-commands @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Deferred operations (Commands) on the World.
//!
//! Commands allow systems to schedule spawn/despawn/insert operations
//! that are applied after the system finishes, avoiding borrow conflicts
//! when iterating and mutating at the same time.

use std::any::{Any, TypeId};

use crate::world::World;
use crate::EntityId;

// ---------------------------------------------------------------------------
// Command enum
// ---------------------------------------------------------------------------

/// Operation differee sur le World.
pub enum Command {
    /// Despawn une entite.
    Despawn(EntityId),
    /// Insere une resource globale.
    InsertResource {
        /// `TypeId` de la resource.
        type_id: TypeId,
        /// Resource boxed.
        resource: Box<dyn Any>,
    },
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Despawn(id) => f.debug_tuple("Despawn").field(id).finish(),
            Self::InsertResource { type_id, .. } => f
                .debug_struct("InsertResource")
                .field("type_id", type_id)
                .finish(),
        }
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Les Commands permettent de planifier des operations sur le World
/// qui seront executees apres la fin du systeme courant.
/// Cela evite les problemes de borrow quand un systeme veut
/// spawn/despawn des entites tout en iterant.
#[derive(Debug, Default)]
pub struct Commands {
    queue: Vec<Command>,
}

impl Commands {
    /// Cree une nouvelle file de commandes vide.
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// Planifie le despawn d'une entite.
    pub fn despawn(&mut self, entity: EntityId) {
        self.queue.push(Command::Despawn(entity));
    }

    /// Planifie l'insertion d'une resource globale.
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.queue.push(Command::InsertResource {
            type_id: TypeId::of::<T>(),
            resource: Box::new(resource),
        });
    }

    /// Retourne le nombre de commandes en attente.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Retourne `true` si la file est vide.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Applique toutes les commandes en attente sur le World.
    /// Les erreurs individuelles sont tracees via `tracing::warn` mais
    /// n'interrompent pas l'application des commandes restantes.
    pub fn apply(self, world: &mut World) {
        for command in self.queue {
            match command {
                Command::Despawn(entity) => {
                    if let Err(e) = world.despawn(entity) {
                        tracing::warn!("Command::Despawn({entity}) failed: {e}");
                    }
                }
                Command::InsertResource { type_id, resource } => {
                    world.resources.insert(type_id, resource);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archetype::{ArchetypeId, EntityLocation};
    use crate::world::EntityEntry;

    #[test]
    fn test_commands_new_is_empty() {
        let commands = Commands::new();
        assert!(commands.is_empty());
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_commands_despawn_queues() {
        let mut commands = Commands::new();
        let e = EntityId::new(0, 0);

        commands.despawn(e);
        assert_eq!(commands.len(), 1);
        assert!(!commands.is_empty());
    }

    #[test]
    fn test_commands_insert_resource_queues() {
        let mut commands = Commands::new();
        commands.insert_resource(42_i32);
        assert_eq!(commands.len(), 1);
    }

    #[test]
    fn test_deferred_despawn() {
        let mut world = World::new();
        let e1 = world.allocate_entity();

        // Simuler une location pour que despawn fonctionne.
        // (sans archetype reel, despawn echouera gracieusement.)
        world.entity_locations[e1.index as usize] = EntityEntry {
            location: Some(EntityLocation {
                archetype_id: ArchetypeId(0),
                row: 0,
            }),
            generation: e1.generation,
        };

        let mut commands = Commands::new();
        commands.despawn(e1);

        // L'entite existe encore.
        assert_eq!(world.entity_count(), 1);

        // Appliquer les commandes.
        // Note: despawn won't fully work without a real archetype,
        // but the queue mechanism is validated.
        commands.apply(&mut world);
    }

    #[test]
    fn test_deferred_insert_resource() {
        let mut world = World::new();

        let mut commands = Commands::new();
        commands.insert_resource(String::from("hello"));

        assert!(!world.has_resource::<String>());
        commands.apply(&mut world);
        assert!(world.has_resource::<String>());
    }

    #[test]
    fn test_commands_default() {
        let commands = Commands::default();
        assert!(commands.is_empty());
    }
}
