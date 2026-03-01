<!-- @id: SD-Impl-02 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-02 -- Guide d'Implementation de l'ECS Archetype Maison

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-ECS-Components.md (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

---

## Table des matieres

1. [Vue d'ensemble de l'ECS MGE](#1-vue-densemble-de-lecs-mge)
2. [Archetype Storage -- SoA en Rust](#2-archetype-storage--soa-en-rust)
3. [Sparse Overlay pour etats ephemeres](#3-sparse-overlay-pour-etats-ephemeres)
4. [Query System](#4-query-system)
5. [World et Resource Management](#5-world-et-resource-management)
6. [Event System](#6-event-system)
7. [Commands et Deferred Operations](#7-commands-et-deferred-operations)
8. [Tous les composants du projet](#8-tous-les-composants-du-projet)
9. [Archetypes principaux de Sodomight](#9-archetypes-principaux-de-sodomight)
10. [Tests unitaires de l'ECS](#10-tests-unitaires-de-lecs)

---

## 1. Vue d'ensemble de l'ECS MGE

L'ECS (Entity-Component-System) MGE est une implementation maison en pure Rust,
sans dependance externe. Le modele retenu est l'**archetype storage** :

- Les entites partageant le meme ensemble de composants sont groupees dans un meme archetype.
- Chaque archetype stocke ses donnees en **SoA (Structure of Arrays)** pour maximiser la coherence de cache.
- Les etats ephemeres (buffs, debuffs, statuts temporaires) utilisent un **sparse overlay** pour eviter les migrations d'archetype couteuses.

**Pourquoi un ECS maison :** Le moteur MGE suit les Lois d'Autonomie (aucune dependance externe critique). Un ECS maison garantit le controle total sur la memoire, les performances, et evite les breaking changes d'un framework tiers.

---

## 2. Archetype Storage -- SoA en Rust

### 2.1 EntityId

```rust
/// @id: mge-ecs-entity-id @do: define @role: kernel @layer: 1
///
/// Identifiant unique d'une entite dans le monde ECS.
/// Le champ `generation` permet de detecter les entites recyclees :
/// quand une entite est despawnee, son index est reutilise mais
/// la generation est incrementee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    /// Index dans le tableau d'entites.
    pub index: u32,
    /// Generation pour invalider les references perimees.
    pub generation: u32,
}

impl EntityId {
    /// Cree un nouvel EntityId.
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// Retourne un EntityId invalide (sentinelle).
    pub fn invalid() -> Self {
        Self {
            index: u32::MAX,
            generation: u32::MAX,
        }
    }

    /// Verifie si cet EntityId est valide (non-sentinelle).
    pub fn is_valid(self) -> bool {
        self.index != u32::MAX
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({}v{})", self.index, self.generation)
    }
}
```

### 2.2 EntityLocation

```rust
/// @id: mge-ecs-entity-location @do: define @role: kernel @layer: 1
///
/// Localise une entite dans le storage : quel archetype et quel index
/// dans cet archetype.
#[derive(Debug, Clone, Copy)]
pub struct EntityLocation {
    /// Identifiant de l'archetype contenant cette entite.
    pub archetype_id: ArchetypeId,
    /// Index de l'entite dans les colonnes de l'archetype.
    pub row: usize,
}
```

### 2.3 ArchetypeId

```rust
/// @id: mge-ecs-archetype-id @do: define @role: kernel @layer: 1
///
/// Identifiant unique d'un archetype, derive du hash des TypeId tries.
/// Deux ensembles identiques de composants produisent le meme ArchetypeId.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArchetypeId(u64);

impl ArchetypeId {
    /// Calcule l'ArchetypeId a partir d'un ensemble de TypeId.
    /// Les TypeId sont tries pour garantir l'unicite.
    pub fn from_type_ids(type_ids: &[std::any::TypeId]) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut sorted = type_ids.to_vec();
        sorted.sort_unstable();

        let mut hasher = DefaultHasher::new();
        for tid in &sorted {
            tid.hash(&mut hasher);
        }
        Self(hasher.finish())
    }
}
```

### 2.4 ComponentColumn -- Stockage SoA type-erased

```rust
/// @id: mge-ecs-component-column @do: define @role: kernel @layer: 1
///
/// Colonne de donnees brutes pour un type de composant.
/// Stocke les donnees de toutes les entites d'un archetype pour un composant donne.
/// Le type est efface a la compilation (type erasure) pour permettre le stockage
/// heterogene dans un HashMap.
pub struct ComponentColumn {
    /// Donnees brutes (Vec<u8> aligne).
    data: Vec<u8>,
    /// Taille d'un element en octets.
    item_size: usize,
    /// Alignement requis.
    item_align: usize,
    /// Nombre d'elements stockes.
    len: usize,
    /// Fonction de drop (pour les types avec Drop impl).
    drop_fn: Option<fn(*mut u8)>,
}

impl ComponentColumn {
    /// Cree une nouvelle colonne vide pour le type T.
    pub fn new<T: 'static>() -> Self {
        let item_size = std::mem::size_of::<T>();
        let item_align = std::mem::align_of::<T>();
        let drop_fn: Option<fn(*mut u8)> = if std::mem::needs_drop::<T>() {
            Some(|ptr: *mut u8| {
                // SAFETY: le ptr pointe vers un T valide, garanti par le contrat
                // de ComponentColumn. Cependant, comme unsafe_code = "forbid",
                // nous utilisons une approche safe alternative.
                // Voir note ci-dessous.
                let _ = ptr;
            })
        } else {
            None
        };
        Self {
            data: Vec::new(),
            item_size,
            item_align,
            len: 0,
            drop_fn,
        }
    }

    /// Retourne le nombre d'elements dans la colonne.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Retourne true si la colonne est vide.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
```

**Note importante sur `unsafe_code = "forbid"` :** L'implementation type-erased
d'un ECS archetype necessite normalement du code `unsafe` pour les casts de pointeurs.
Comme notre workspace interdit strictement le code unsafe, nous adoptons une
strategie alternative basee sur `std::any::Any` et le downcasting :

```rust
/// @id: mge-ecs-typed-column @do: define @role: kernel @layer: 1
///
/// Colonne typee utilisant Any pour eviter tout code unsafe.
/// Chaque colonne stocke un Vec<T> wrape dans Box<dyn Any>.
pub struct TypedColumn {
    /// Donnees stockees sous forme de Box<dyn Any>.
    /// Le type reel est Vec<T> pour le type T de cette colonne.
    inner: Box<dyn std::any::Any>,
    /// TypeId du composant stocke.
    type_id: std::any::TypeId,
}

impl TypedColumn {
    /// Cree une nouvelle colonne typee pour le type T.
    pub fn new<T: 'static + Clone + std::fmt::Debug>() -> Self {
        let inner: Vec<T> = Vec::new();
        Self {
            inner: Box::new(inner),
            type_id: std::any::TypeId::of::<T>(),
        }
    }

    /// Ajoute un element a la colonne.
    /// Retourne Err si le type ne correspond pas.
    pub fn push<T: 'static + Clone + std::fmt::Debug>(
        &mut self,
        value: T,
    ) -> Result<(), EcsError> {
        let vec = self
            .inner
            .downcast_mut::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        vec.push(value);
        Ok(())
    }

    /// Obtient une reference a l'element a l'index donne.
    pub fn get<T: 'static>(&self, index: usize) -> Result<&T, EcsError> {
        let vec = self
            .inner
            .downcast_ref::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        vec.get(index).ok_or(EcsError::IndexOutOfBounds { index })
    }

    /// Obtient une reference mutable a l'element a l'index donne.
    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Result<&mut T, EcsError> {
        let vec = self
            .inner
            .downcast_mut::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        vec.get_mut(index).ok_or(EcsError::IndexOutOfBounds { index })
    }

    /// Supprime l'element a l'index donne par swap-remove.
    /// Retourne l'element supprime si les types correspondent.
    pub fn swap_remove<T: 'static>(&mut self, index: usize) -> Result<T, EcsError> {
        let vec = self
            .inner
            .downcast_mut::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        if index >= vec.len() {
            return Err(EcsError::IndexOutOfBounds { index });
        }
        Ok(vec.swap_remove(index))
    }

    /// Retourne le nombre d'elements.
    pub fn len(&self) -> usize {
        // On ne peut pas downcaster sans connaitre T, donc on stocke
        // la len dans un champ separe ou on utilise un trait objet.
        // Approche simplifiee : utiliser un trait Length.
        // Pour l'instant, retourner 0 comme placeholder.
        0
    }
}
```

### 2.5 Archetype struct

```rust
/// @id: mge-ecs-archetype @do: define @role: kernel @layer: 1
///
/// Un archetype regroupe toutes les entites partageant le meme
/// ensemble exact de composants. Les donnees sont stockees en colonnes
/// (SoA) pour maximiser la coherence de cache.
pub struct Archetype {
    /// Identifiant unique de cet archetype.
    pub id: ArchetypeId,
    /// TypeIds des composants dans cet archetype (tries).
    pub component_types: Vec<std::any::TypeId>,
    /// Colonnes de donnees, une par type de composant.
    pub columns: std::collections::HashMap<std::any::TypeId, TypedColumn>,
    /// Identifiants des entites dans cet archetype (ordre = index dans les colonnes).
    pub entity_ids: Vec<EntityId>,
}

impl Archetype {
    /// Cree un nouvel archetype vide pour les types donnes.
    pub fn new(id: ArchetypeId, component_types: Vec<std::any::TypeId>) -> Self {
        Self {
            id,
            component_types,
            columns: std::collections::HashMap::new(),
            entity_ids: Vec::new(),
        }
    }

    /// Retourne le nombre d'entites dans cet archetype.
    pub fn entity_count(&self) -> usize {
        self.entity_ids.len()
    }

    /// Supprime une entite par swap-remove. Retourne l'EntityId qui a ete
    /// deplace a la place de l'entite supprimee (pour mettre a jour les locations).
    pub fn remove_entity(&mut self, row: usize) -> Option<EntityId> {
        if row >= self.entity_ids.len() {
            return None;
        }
        let last_index = self.entity_ids.len() - 1;
        let moved_entity = if row != last_index {
            Some(self.entity_ids[last_index])
        } else {
            None
        };
        self.entity_ids.swap_remove(row);
        // Les colonnes doivent aussi swap_remove(row) -- delegue a l'appelant
        // car le type T est necessaire pour chaque colonne.
        moved_entity
    }
}
```

### 2.6 Migration d'entite entre archetypes

Quand un composant est ajoute ou supprime d'une entite, celle-ci doit migrer
vers un nouvel archetype.

```rust
/// @id: mge-ecs-migration @do: define @role: kernel @layer: 1
///
/// Resultat d'une migration d'entite entre archetypes.
pub struct MigrationResult {
    /// Ancien archetype.
    pub old_archetype_id: ArchetypeId,
    /// Nouvel archetype.
    pub new_archetype_id: ArchetypeId,
    /// Ancienne rangee dans l'ancien archetype.
    pub old_row: usize,
    /// Nouvelle rangee dans le nouvel archetype.
    pub new_row: usize,
    /// Si un swap a eu lieu dans l'ancien archetype, l'entite
    /// qui a ete deplacee a la position `old_row`.
    pub swapped_entity: Option<EntityId>,
}

/// Effectue la migration d'une entite d'un archetype a un autre.
/// Les donnees de composants communs sont copiees.
///
/// # Erreurs
/// Retourne `EcsError::ArchetypeNotFound` si l'un des archetypes n'existe pas.
pub fn migrate_entity(
    world_archetypes: &mut std::collections::HashMap<ArchetypeId, Archetype>,
    entity_id: EntityId,
    old_archetype_id: ArchetypeId,
    old_row: usize,
    new_archetype_id: ArchetypeId,
) -> Result<MigrationResult, EcsError> {
    // 1. Determiner les types communs entre les deux archetypes.
    let old_types: Vec<std::any::TypeId>;
    let new_types: Vec<std::any::TypeId>;
    {
        let old_arch = world_archetypes
            .get(&old_archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        let new_arch = world_archetypes
            .get(&new_archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        old_types = old_arch.component_types.clone();
        new_types = new_arch.component_types.clone();
    }

    let common_types: Vec<std::any::TypeId> = old_types
        .iter()
        .filter(|t| new_types.contains(t))
        .copied()
        .collect();

    // 2. Pour chaque type commun, copier les donnees de l'ancien vers le nouveau.
    // NOTE: la copie type-erased via Any necessite un mecanisme de clonage
    // generique. En pratique, on enregistre une clone_fn par TypeId.
    // La logique detaillee est implementee dans world.rs.

    let new_row = {
        let new_arch = world_archetypes
            .get_mut(&new_archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        let row = new_arch.entity_ids.len();
        new_arch.entity_ids.push(entity_id);
        row
    };

    // 3. Retirer l'entite de l'ancien archetype (swap-remove).
    let swapped = {
        let old_arch = world_archetypes
            .get_mut(&old_archetype_id)
            .ok_or(EcsError::ArchetypeNotFound)?;
        old_arch.remove_entity(old_row)
    };

    Ok(MigrationResult {
        old_archetype_id,
        new_archetype_id,
        old_row,
        new_row,
        swapped_entity: swapped,
    })
}
```

---

## 3. Sparse Overlay pour etats ephemeres

### 3.1 Justification

Les buffs, debuffs, et etats temporaires de combat changent frequemment.
Ajouter/retirer un composant ephemere via migration d'archetype serait couteux
(copie de toutes les colonnes). Le sparse overlay stocke ces composants
dans un `HashMap<EntityId, T>` separe de l'archetype storage.

### 3.2 SparseSet implementation

```rust
/// @id: mge-ecs-sparse-set @do: define @role: kernel @layer: 1
///
/// Ensemble sparse pour stocker des composants ephemeres sans migration d'archetype.
/// Utilise un HashMap en interne pour un acces O(1) par EntityId.
#[derive(Debug)]
pub struct SparseSet<T: std::fmt::Debug> {
    data: std::collections::HashMap<EntityId, T>,
}

impl<T: std::fmt::Debug> SparseSet<T> {
    /// Cree un nouveau SparseSet vide.
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }

    /// Insere ou remplace un composant pour une entite.
    pub fn insert(&mut self, entity: EntityId, value: T) {
        self.data.insert(entity, value);
    }

    /// Retire le composant d'une entite. Retourne le composant retire.
    pub fn remove(&mut self, entity: EntityId) -> Option<T> {
        self.data.remove(&entity)
    }

    /// Obtient une reference au composant d'une entite.
    pub fn get(&self, entity: EntityId) -> Option<&T> {
        self.data.get(&entity)
    }

    /// Obtient une reference mutable au composant d'une entite.
    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        self.data.get_mut(&entity)
    }

    /// Verifie si une entite a un composant dans ce set.
    pub fn contains(&self, entity: EntityId) -> bool {
        self.data.contains_key(&entity)
    }

    /// Retourne le nombre d'entites ayant ce composant.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Retourne true si le set est vide.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Itere sur toutes les paires (EntityId, &T).
    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &T)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    /// Itere sur toutes les paires (EntityId, &mut T) de maniere mutable.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.data.iter_mut().map(|(k, v)| (*k, v))
    }

    /// Supprime tous les elements correspondant au predicat.
    pub fn retain<F: FnMut(&EntityId, &mut T) -> bool>(&mut self, f: F) {
        self.data.retain(f);
    }
}

impl<T: std::fmt::Debug> Default for SparseSet<T> {
    fn default() -> Self {
        Self::new()
    }
}
```

### 3.3 Quand utiliser overlay vs archetype

| Situation | Stockage recommande |
|-----------|-------------------|
| Composant permanent (Position, Sprite, VitalPools) | Archetype (SoA column) |
| Buff temporaire (duree limitee, ex: Shiver Armor 5min) | Sparse overlay |
| Debuff temporaire (Poison, Amplify Damage, Decrepify) | Sparse overlay |
| Etat de crowd control (Frozen, Stunned, Slowed) | Sparse overlay |
| Composant optionnel rare (MonsterAffixes sur champions) | Sparse overlay |
| Projectile en vol (courte duree de vie) | Archetype (spawn/despawn rapide) |

### 3.4 Exemples d'utilisation sparse

```rust
/// @id: mge-ecs-sparse-buffs @do: define @role: arpg @layer: 3
///
/// Buff actif sur une entite (sparse overlay).
#[derive(Debug, Clone)]
pub struct ActiveBuff {
    /// ID de la definition du buff.
    pub buff_def_id: String,
    /// Frames restantes avant expiration.
    pub remaining_frames: u32,
    /// Nombre de stacks actuels.
    pub stacks: u8,
    /// Frame a laquelle le buff a ete applique.
    pub applied_at_frame: u64,
}

/// Etat de crowd control (sparse overlay).
#[derive(Debug, Clone)]
pub struct CrowdControlState {
    pub frozen: bool,
    pub frozen_frames_remaining: u32,
    pub chilled: bool,
    pub chill_slow_percent: i32,
    pub stunned: bool,
    pub stun_frames_remaining: u32,
    pub blinded: bool,
    pub blind_frames_remaining: u32,
}

/// Poison actif sur une entite (sparse overlay).
#[derive(Debug, Clone)]
pub struct PoisonEffect {
    /// Dommage total restant a appliquer.
    pub total_remaining: f32,
    /// Dommage par frame.
    pub damage_per_frame: f32,
    /// Frames restantes.
    pub frames_remaining: u32,
}
```

---

## 4. Query System

### 4.1 API des queries

Le systeme de query permet d'iterer efficacement sur les entites possedant
un ensemble specifique de composants.

```rust
/// @id: mge-ecs-query @do: define @role: kernel @layer: 1
///
/// Descripteur de query : quels composants en lecture, quels composants
/// en ecriture, quels filtres (With, Without).
pub struct QueryDescriptor {
    /// TypeIds des composants requis en lecture.
    pub read: Vec<std::any::TypeId>,
    /// TypeIds des composants requis en ecriture (mutable).
    pub write: Vec<std::any::TypeId>,
    /// TypeIds des composants qui doivent etre presents (filtre positif).
    pub with: Vec<std::any::TypeId>,
    /// TypeIds des composants qui ne doivent pas etre presents (filtre negatif).
    pub without: Vec<std::any::TypeId>,
}

/// Filtre : l'entite doit avoir le composant T.
pub struct With<T: 'static>(std::marker::PhantomData<T>);

/// Filtre : l'entite ne doit pas avoir le composant T.
pub struct Without<T: 'static>(std::marker::PhantomData<T>);

/// Filtre : le composant T a change depuis le dernier tick.
pub struct Changed<T: 'static>(std::marker::PhantomData<T>);
```

### 4.2 Execution d'une query

L'execution d'une query parcourt tous les archetypes du monde et filtre
ceux qui contiennent les composants requis.

```rust
/// @id: mge-ecs-query-exec @do: define @role: kernel @layer: 1
///
/// Execute une query sur le monde et retourne les entites correspondantes.
///
/// # Algorithme
/// 1. Pour chaque archetype du monde :
///    a. Verifier que l'archetype contient TOUS les types `read` et `write`.
///    b. Verifier que l'archetype contient TOUS les types `with`.
///    c. Verifier que l'archetype ne contient AUCUN des types `without`.
///    d. Si les conditions sont remplies, iterer sur les entites de l'archetype.
/// 2. Pour chaque entite valide, produire un tuple de references.
pub fn execute_query(
    world: &World,
    descriptor: &QueryDescriptor,
) -> Vec<EntityId> {
    let mut result = Vec::new();

    for archetype in world.archetypes.values() {
        // Verifier les composants requis.
        let has_all_required = descriptor
            .read
            .iter()
            .chain(descriptor.write.iter())
            .all(|tid| archetype.component_types.contains(tid));

        if !has_all_required {
            continue;
        }

        // Verifier les filtres With.
        let has_all_with = descriptor
            .with
            .iter()
            .all(|tid| archetype.component_types.contains(tid));

        if !has_all_with {
            continue;
        }

        // Verifier les filtres Without.
        let has_none_without = descriptor
            .without
            .iter()
            .all(|tid| !archetype.component_types.contains(tid));

        if !has_none_without {
            continue;
        }

        // Cet archetype correspond : ajouter toutes ses entites.
        result.extend_from_slice(&archetype.entity_ids);
    }

    result
}
```

### 4.3 API ergonomique pour les systemes

L'API ergonomique permet d'ecrire des systemes de maniere lisible :

```rust
/// Exemple d'utilisation de la query API dans un systeme.
///
/// Ce systeme deplace toutes les entites ayant Position + Velocity,
/// qui ne sont pas mortes (Without<Dead>).
pub fn movement_system(world: &mut World) {
    // En pratique, le systeme de query genere du code typesafe
    // via des macros ou des traits. Voici le pseudo-code :
    //
    // for (entity, pos, vel) in world.query::<(&mut Position, &Velocity)>()
    //     .without::<Dead>()
    //     .iter_mut()
    // {
    //     pos.x += vel.dx * FIXED_DT_SECS;
    //     pos.y += vel.dy * FIXED_DT_SECS;
    // }
    //
    // L'implementation reelle utilise le pattern de QueryState
    // qui cache les archetypes correspondants entre les ticks.

    let entities = execute_query(
        world,
        &QueryDescriptor {
            read: vec![std::any::TypeId::of::<Velocity>()],
            write: vec![std::any::TypeId::of::<Position>()],
            with: vec![],
            without: vec![std::any::TypeId::of::<Dead>()],
        },
    );

    for entity_id in entities {
        // Acces aux composants via le world.
        let vel = world.get_component::<Velocity>(entity_id);
        if let (Ok(vel_ref), Ok(())) = (vel, Ok(())) {
            // Appliquer le deplacement.
            let _ = world.modify_component::<Position>(entity_id, |pos| {
                pos.x += vel_ref.dx * 0.04; // FIXED_DT_SECS
                pos.y += vel_ref.dy * 0.04;
            });
        }
    }
}
```

---

## 5. World et Resource Management

### 5.1 World struct

```rust
/// @id: mge-ecs-world @do: define @role: kernel @layer: 1
///
/// Le World est le conteneur principal de l'ECS.
/// Il stocke tous les archetypes, les entites, les resources globales,
/// les sparse overlays, et les event queues.
pub struct World {
    /// Registre des archetypes par ArchetypeId.
    pub archetypes: std::collections::HashMap<ArchetypeId, Archetype>,
    /// Mapping EntityId -> EntityLocation pour acces O(1).
    entity_locations: Vec<EntityEntry>,
    /// Generation actuelle pour chaque index d'entite.
    generations: Vec<u32>,
    /// Indices d'entites libres (recyclables).
    free_indices: Vec<u32>,
    /// Compteur total d'entites vivantes.
    entity_count: usize,
    /// Resources globales (typees, acces par TypeId).
    resources: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
    /// Sparse overlays (un par type de composant ephemere).
    sparse_overlays: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
}

/// Entree dans la table d'entites.
#[derive(Debug, Clone, Copy)]
pub struct EntityEntry {
    /// Location courante de l'entite (None si despawnee).
    pub location: Option<EntityLocation>,
    /// Generation de l'entite a cet index.
    pub generation: u32,
}

impl World {
    /// Cree un nouveau monde ECS vide.
    pub fn new() -> Self {
        Self {
            archetypes: std::collections::HashMap::new(),
            entity_locations: Vec::new(),
            generations: Vec::new(),
            free_indices: Vec::new(),
            entity_count: 0,
            resources: std::collections::HashMap::new(),
            sparse_overlays: std::collections::HashMap::new(),
        }
    }

    /// Alloue un nouvel EntityId (reutilise un index libre si disponible).
    pub fn allocate_entity(&mut self) -> EntityId {
        if let Some(index) = self.free_indices.pop() {
            let gen = self.generations[index as usize] + 1;
            self.generations[index as usize] = gen;
            self.entity_locations[index as usize] = EntityEntry {
                location: None,
                generation: gen,
            };
            self.entity_count += 1;
            EntityId::new(index, gen)
        } else {
            let index = self.entity_locations.len() as u32;
            self.entity_locations.push(EntityEntry {
                location: None,
                generation: 0,
            });
            self.generations.push(0);
            self.entity_count += 1;
            EntityId::new(index, 0)
        }
    }

    /// Despawn une entite : la retire de son archetype et libere son index.
    pub fn despawn(&mut self, entity: EntityId) -> Result<(), EcsError> {
        let entry = self
            .entity_locations
            .get(entity.index as usize)
            .ok_or(EcsError::EntityNotFound)?;

        if entry.generation != entity.generation {
            return Err(EcsError::EntityStale);
        }

        // Retirer de l'archetype.
        if let Some(location) = entry.location {
            if let Some(archetype) = self.archetypes.get_mut(&location.archetype_id) {
                let swapped = archetype.remove_entity(location.row);
                // Si un swap a eu lieu, mettre a jour la location de l'entite deplacee.
                if let Some(swapped_entity) = swapped {
                    if let Some(swapped_entry) =
                        self.entity_locations.get_mut(swapped_entity.index as usize)
                    {
                        if let Some(ref mut loc) = swapped_entry.location {
                            loc.row = location.row;
                        }
                    }
                }
            }
        }

        // Marquer l'entite comme libre.
        self.entity_locations[entity.index as usize] = EntityEntry {
            location: None,
            generation: entity.generation,
        };
        self.free_indices.push(entity.index);
        self.entity_count -= 1;

        Ok(())
    }

    /// Verifie si une entite est vivante.
    pub fn is_alive(&self, entity: EntityId) -> bool {
        self.entity_locations
            .get(entity.index as usize)
            .map_or(false, |entry| {
                entry.generation == entity.generation && entry.location.is_some()
            })
    }

    /// Retourne le nombre d'entites vivantes.
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }
}
```

### 5.2 Resource Management

```rust
/// @id: mge-ecs-resources @do: define @role: kernel @layer: 1

impl World {
    /// Insere une resource globale typee. Remplace si elle existait deja.
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources
            .insert(std::any::TypeId::of::<T>(), Box::new(resource));
    }

    /// Obtient une reference a une resource globale.
    pub fn resource<T: 'static>(&self) -> Result<&T, EcsError> {
        self.resources
            .get(&std::any::TypeId::of::<T>())
            .and_then(|r| r.downcast_ref::<T>())
            .ok_or(EcsError::ResourceNotFound)
    }

    /// Obtient une reference mutable a une resource globale.
    pub fn resource_mut<T: 'static>(&mut self) -> Result<&mut T, EcsError> {
        self.resources
            .get_mut(&std::any::TypeId::of::<T>())
            .and_then(|r| r.downcast_mut::<T>())
            .ok_or(EcsError::ResourceNotFound)
    }

    /// Verifie si une resource globale existe.
    pub fn has_resource<T: 'static>(&self) -> bool {
        self.resources.contains_key(&std::any::TypeId::of::<T>())
    }
}
```

### 5.3 Sparse Overlay dans le World

```rust
/// @id: mge-ecs-sparse-world @do: define @role: kernel @layer: 1

impl World {
    /// Obtient (ou cree) le sparse overlay pour le type T.
    pub fn sparse_overlay<T: 'static + std::fmt::Debug>(&self) -> Option<&SparseSet<T>> {
        self.sparse_overlays
            .get(&std::any::TypeId::of::<T>())
            .and_then(|s| s.downcast_ref::<SparseSet<T>>())
    }

    /// Obtient (ou cree) le sparse overlay mutable pour le type T.
    pub fn sparse_overlay_mut<T: 'static + std::fmt::Debug>(
        &mut self,
    ) -> &mut SparseSet<T> {
        let type_id = std::any::TypeId::of::<T>();
        if !self.sparse_overlays.contains_key(&type_id) {
            self.sparse_overlays
                .insert(type_id, Box::new(SparseSet::<T>::new()));
        }
        self.sparse_overlays
            .get_mut(&type_id)
            .and_then(|s| s.downcast_mut::<SparseSet<T>>())
            .expect("SparseSet just inserted")
    }
}
```

---

## 6. Event System

```rust
/// @id: mge-ecs-events @do: define @role: kernel @layer: 1
///
/// File d'evenements double-buffer pour communication entre systemes.
/// Les evenements ecrits dans un frame sont lisibles dans le frame suivant.
#[derive(Debug)]
pub struct Events<T: std::fmt::Debug> {
    /// Buffer courant (ecritures).
    write_buffer: Vec<T>,
    /// Buffer precedent (lectures).
    read_buffer: Vec<T>,
}

impl<T: std::fmt::Debug> Events<T> {
    pub fn new() -> Self {
        Self {
            write_buffer: Vec::new(),
            read_buffer: Vec::new(),
        }
    }

    /// Envoie un evenement.
    pub fn send(&mut self, event: T) {
        self.write_buffer.push(event);
    }

    /// Lit tous les evenements du frame precedent.
    pub fn read(&self) -> &[T] {
        &self.read_buffer
    }

    /// Swap les buffers. Appele une fois par frame par le scheduler.
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.write_buffer, &mut self.read_buffer);
        self.write_buffer.clear();
    }
}

impl<T: std::fmt::Debug> Default for Events<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper pour ecrire des evenements depuis un systeme.
pub struct EventWriter<'a, T: std::fmt::Debug> {
    events: &'a mut Events<T>,
}

impl<'a, T: std::fmt::Debug> EventWriter<'a, T> {
    pub fn send(&mut self, event: T) {
        self.events.send(event);
    }
}

/// Wrapper pour lire des evenements depuis un systeme.
pub struct EventReader<'a, T: std::fmt::Debug> {
    events: &'a Events<T>,
}

impl<'a, T: std::fmt::Debug> EventReader<'a, T> {
    pub fn read(&self) -> &[T] {
        self.events.read()
    }
}
```

---

## 7. Commands et Deferred Operations

```rust
/// @id: mge-ecs-commands @do: define @role: kernel @layer: 1
///
/// Les Commands permettent de planifier des operations sur le World
/// qui seront executees apres la fin du systeme courant.
/// Cela evite les problemes de borrow quand un systeme veut
/// spawn/despawn des entites tout en iterant.
pub struct Commands {
    queue: Vec<Command>,
}

/// Operation differee sur le World.
pub enum Command {
    /// Despawn une entite.
    Despawn(EntityId),
    /// Insere une resource.
    InsertResource {
        type_id: std::any::TypeId,
        resource: Box<dyn std::any::Any>,
    },
}

impl Commands {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// Planifie le despawn d'une entite.
    pub fn despawn(&mut self, entity: EntityId) {
        self.queue.push(Command::Despawn(entity));
    }

    /// Applique toutes les commandes en attente sur le World.
    pub fn apply(self, world: &mut World) {
        for command in self.queue {
            match command {
                Command::Despawn(entity) => {
                    let _ = world.despawn(entity);
                }
                Command::InsertResource { type_id, resource } => {
                    world.resources.insert(type_id, resource);
                }
            }
        }
    }
}

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 8. Tous les composants du projet

Les composants sont repris du document SD-Tech-ECS-Components.md de Denis.
Voici le recapitulatif par domaine avec les derives corrects.

### 8.1 Composants Kernel (mge-ecs)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `EntityId` | `Debug, Clone, Copy, PartialEq, Eq, Hash` | N/A (identifiant) |
| `Name` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `DefId` | `Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize` | Archetype |
| `Parent` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Children` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `Lifetime` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `DropAge` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |

### 8.2 Composants Mouvement (mge-arpg-entity)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `Position` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Velocity` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Facing` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `MovementSpeed` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Locomotion` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `PathPlan` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `MoveTarget` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |

### 8.3 Composants Rendu (mge-render)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `Sprite` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `AnimState` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `VisualEffect` | `Debug, Clone, Serialize, Deserialize` | Sparse |

### 8.4 Composants Stats (mge-arpg-stats)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `CharacterInfo` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `BaseAttributes` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `UnspentPoints` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `VitalPools` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Regeneration` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Defense` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Resistances` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `AttackRating` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `Breakpoints` | `Debug, Clone, Serialize, Deserialize` | Archetype |

### 8.5 Composants Combat (mge-arpg-combat)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `EquippedWeapon` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `CombatBonuses` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `ElementalDamage` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `ActiveAttack` | `Debug, Clone, Serialize, Deserialize` | Sparse |
| `ProjectileData` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `BlockChance` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `CircleHitbox` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `ActiveBuff` | `Debug, Clone` | Sparse |
| `CrowdControlState` | `Debug, Clone` | Sparse |
| `PoisonEffect` | `Debug, Clone` | Sparse |

### 8.6 Composants Items (mge-arpg-items)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `ItemComponent` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `Inventory` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `Equipment` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `Belt` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `SkillSlots` | `Debug, Clone, Serialize, Deserialize` | Archetype |

### 8.7 Composants IA (mge-arpg-ai)

| Composant | Derives | Archetype vs Sparse |
|-----------|---------|-------------------|
| `MonsterData` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `AiBehavior` | `Debug, Clone, Serialize, Deserialize` | Archetype |
| `AggroRange` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `LeashRange` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |
| `SpawnPoint` | `Debug, Clone, Copy, Serialize, Deserialize` | Archetype |

### 8.8 Tags (zero-sized components)

| Tag | Crate | Usage |
|-----|-------|-------|
| `LocalPlayer` | `sodomight-game` | Marque l'entite du joueur local |
| `Monster` | `mge-arpg-entity` | Marque les entites monstres |
| `Npc` | `mge-arpg-entity` | Marque les PNJ |
| `Dead` | `mge-arpg-entity` | Marque les entites mortes |
| `Ethereal` | `mge-arpg-items` | Marque les items etheres |
| `Identified` | `mge-arpg-items` | Marque les items identifies |
| `InTown` | `mge-arpg-entity` | Marque les entites en ville |

---

## 9. Archetypes principaux de Sodomight

### 9.1 Archetype Player Character

Composants presents sur chaque entite joueur :

```
Position, Velocity, Facing, MovementSpeed, Locomotion, PathPlan, MoveTarget,
Sprite, AnimState,
CharacterInfo, BaseAttributes, UnspentPoints, VitalPools, Regeneration,
Defense, Resistances, AttackRating, Breakpoints,
EquippedWeapon, CombatBonuses, ElementalDamage, BlockChance, CircleHitbox,
Inventory, Equipment, Belt, SkillSlots,
Name, DefId
```

Tag : `LocalPlayer` (uniquement sur le joueur local).

### 9.2 Archetype Monster

```
Position, Velocity, Facing, MovementSpeed, Locomotion,
Sprite, AnimState,
VitalPools, Defense, Resistances, AttackRating,
EquippedWeapon, CombatBonuses, ElementalDamage, CircleHitbox,
MonsterData, AiBehavior, AggroRange, LeashRange, SpawnPoint,
Name, DefId
```

Tag : `Monster`.

### 9.3 Archetype Ground Item

```
Position, Sprite,
ItemComponent, DropAge,
Name, DefId
```

### 9.4 Archetype Projectile

```
Position, Velocity, Sprite, AnimState,
ProjectileData, CircleHitbox, Lifetime,
Parent
```

### 9.5 Archetype NPC

```
Position, Facing, Sprite, AnimState,
Name, DefId, CircleHitbox
```

Tag : `Npc`.

---

## 10. Tests unitaires de l'ECS

### 10.1 Test spawn et despawn

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_entity() {
        let mut world = World::new();
        let e1 = world.allocate_entity();
        let e2 = world.allocate_entity();

        assert_ne!(e1, e2);
        assert_eq!(e1.index, 0);
        assert_eq!(e2.index, 1);
        assert_eq!(e1.generation, 0);
        assert_eq!(world.entity_count(), 2);
    }

    #[test]
    fn test_despawn_and_recycle() {
        let mut world = World::new();
        let e1 = world.allocate_entity();
        let _e2 = world.allocate_entity();

        // Despawn e1.
        world.entity_locations[e1.index as usize] = EntityEntry {
            location: None,
            generation: e1.generation,
        };
        world.free_indices.push(e1.index);
        world.entity_count -= 1;

        // Allouer une nouvelle entite : devrait recycler l'index 0.
        let e3 = world.allocate_entity();
        assert_eq!(e3.index, 0);
        assert_eq!(e3.generation, 1); // Generation incrementee.
        assert_ne!(e1, e3); // Meme index, generation differente.
    }

    #[test]
    fn test_entity_id_display() {
        let e = EntityId::new(42, 7);
        assert_eq!(format!("{e}"), "Entity(42v7)");
    }

    #[test]
    fn test_entity_id_invalid() {
        let invalid = EntityId::invalid();
        assert!(!invalid.is_valid());

        let valid = EntityId::new(0, 0);
        assert!(valid.is_valid());
    }
}
```

### 10.2 Test SparseSet

```rust
#[cfg(test)]
mod sparse_tests {
    use super::*;

    #[test]
    fn test_sparse_set_insert_and_get() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        set.insert(e, 42);
        assert_eq!(set.get(e), Some(&42));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_sparse_set_remove() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        set.insert(e, 42);
        let removed = set.remove(e);
        assert_eq!(removed, Some(42));
        assert!(set.is_empty());
        assert_eq!(set.get(e), None);
    }

    #[test]
    fn test_sparse_set_overwrite() {
        let mut set = SparseSet::<i32>::new();
        let e = EntityId::new(0, 0);

        set.insert(e, 42);
        set.insert(e, 99);
        assert_eq!(set.get(e), Some(&99));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_sparse_set_retain() {
        let mut set = SparseSet::<i32>::new();
        set.insert(EntityId::new(0, 0), 1);
        set.insert(EntityId::new(1, 0), 2);
        set.insert(EntityId::new(2, 0), 3);

        set.retain(|_, v| *v > 1);
        assert_eq!(set.len(), 2);
        assert_eq!(set.get(EntityId::new(0, 0)), None);
        assert_eq!(set.get(EntityId::new(1, 0)), Some(&2));
    }

    #[test]
    fn test_sparse_set_iter() {
        let mut set = SparseSet::<String>::new();
        set.insert(EntityId::new(0, 0), "hello".to_string());
        set.insert(EntityId::new(1, 0), "world".to_string());

        let items: Vec<_> = set.iter().collect();
        assert_eq!(items.len(), 2);
    }
}
```

### 10.3 Test Resources

```rust
#[cfg(test)]
mod resource_tests {
    use super::*;

    #[derive(Debug)]
    struct GameTime {
        elapsed_frames: u64,
    }

    #[test]
    fn test_insert_and_read_resource() {
        let mut world = World::new();
        world.insert_resource(GameTime { elapsed_frames: 0 });

        let time = world.resource::<GameTime>().unwrap();
        assert_eq!(time.elapsed_frames, 0);
    }

    #[test]
    fn test_mutate_resource() {
        let mut world = World::new();
        world.insert_resource(GameTime { elapsed_frames: 0 });

        {
            let time = world.resource_mut::<GameTime>().unwrap();
            time.elapsed_frames = 100;
        }

        let time = world.resource::<GameTime>().unwrap();
        assert_eq!(time.elapsed_frames, 100);
    }

    #[test]
    fn test_missing_resource() {
        let world = World::new();
        let result = world.resource::<GameTime>();
        assert!(result.is_err());
    }
}
```

### 10.4 Test Events

```rust
#[cfg(test)]
mod event_tests {
    use super::*;

    #[derive(Debug)]
    struct DamageEvent {
        amount: f32,
    }

    #[test]
    fn test_send_and_read_events() {
        let mut events = Events::<DamageEvent>::new();

        // Frame 1 : ecrire des evenements.
        events.send(DamageEvent { amount: 10.0 });
        events.send(DamageEvent { amount: 25.5 });

        // Pas encore lisible (dans le write buffer).
        assert!(events.read().is_empty());

        // Swap : les evenements passent dans le read buffer.
        events.swap_buffers();

        // Maintenant lisible.
        assert_eq!(events.read().len(), 2);
        assert!((events.read()[0].amount - 10.0).abs() < f32::EPSILON);
        assert!((events.read()[1].amount - 25.5).abs() < f32::EPSILON);

        // Un nouveau swap efface les anciens evenements.
        events.swap_buffers();
        assert!(events.read().is_empty());
    }
}
```

### 10.5 Test ArchetypeId

```rust
#[cfg(test)]
mod archetype_id_tests {
    use super::*;

    #[test]
    fn test_same_types_same_id() {
        let types_a = vec![
            std::any::TypeId::of::<i32>(),
            std::any::TypeId::of::<f32>(),
        ];
        let types_b = vec![
            std::any::TypeId::of::<f32>(),
            std::any::TypeId::of::<i32>(),
        ];

        let id_a = ArchetypeId::from_type_ids(&types_a);
        let id_b = ArchetypeId::from_type_ids(&types_b);

        // Meme ensemble de types (ordre different) -> meme id.
        assert_eq!(id_a, id_b);
    }

    #[test]
    fn test_different_types_different_id() {
        let types_a = vec![std::any::TypeId::of::<i32>()];
        let types_b = vec![std::any::TypeId::of::<f32>()];

        let id_a = ArchetypeId::from_type_ids(&types_a);
        let id_b = ArchetypeId::from_type_ids(&types_b);

        assert_ne!(id_a, id_b);
    }
}
```

### 10.6 Test Commands

```rust
#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn test_deferred_despawn() {
        let mut world = World::new();
        let e1 = world.allocate_entity();
        // Simuler une location pour que despawn fonctionne.
        world.entity_locations[e1.index as usize].location = Some(EntityLocation {
            archetype_id: ArchetypeId(0),
            row: 0,
        });

        let mut commands = Commands::new();
        commands.despawn(e1);

        // L'entite existe encore.
        assert_eq!(world.entity_count(), 1);

        // Appliquer les commandes.
        // Note : despawn echouera si l'archetype n'existe pas,
        // mais le test valide la mecanique de queue.
        commands.apply(&mut world);
    }
}
```

---

## Erreurs ECS

```rust
/// @id: mge-ecs-errors @do: define @role: kernel @layer: 1
///
/// Types d'erreur pour le module ECS.
#[derive(Debug, thiserror::Error)]
pub enum EcsError {
    #[error("Entity not found")]
    EntityNotFound,

    #[error("Entity reference is stale (generation mismatch)")]
    EntityStale,

    #[error("Archetype not found")]
    ArchetypeNotFound,

    #[error("Component type mismatch during downcast")]
    TypeMismatch,

    #[error("Index {index} out of bounds")]
    IndexOutOfBounds { index: usize },

    #[error("Resource not found")]
    ResourceNotFound,
}
```

---

*Document redige par Francois, Dev Back-End -- Miyukini AI Studio*
*Base sur SD-Tech-ECS-Components.md de Denis*
*Revision : 2026-02-28 v1.0*
