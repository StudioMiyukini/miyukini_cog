# mge-puzzle-tile

> @id mge.puzzle.tile.v1  
> @role plugin  
> @domain puzzle  
> @do manage_tile_types_states_lifecycle  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-tile` |
| @id MSCM | `mge.puzzle.tile.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (update_tile_states chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) sur tuiles actives |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TileColor` | `Red, Blue, Green, Yellow, Purple, Orange, Wild` | Couleur de la tuile. Wild = joker |
| `TileState` | `Idle, Selected, Matched, Falling, Spawning, Locked` | Etat dans le cycle de vie |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Tile` | `mge.puzzle.tile.v1.component.tile` | `color: TileColor, state: TileState, row: u32, col: u32` | Tuile sur la grille. Position logique (row, col) |
| `TileModifier` | `mge.puzzle.tile.v1.component.tile_modifier` | `is_bomb: bool, is_line_clear: bool, is_color_bomb: bool` | Modificateur special. Optionnel sur une Tile |

---

## 4. Formules

Aucune formule de derivation. Les tuiles sont des donnees pures sans calcul derive.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_tile_states` | `mge.puzzle.tile.v1.fn.update_tile_states` | 1400 | Tile | Tile | TileStateChanged | O(n) | Evalue les transitions d'etat : Spawning→Idle, Matched→pending despawn. Emet TileStateChanged |
| `spawn_new_tiles` | `mge.puzzle.tile.v1.fn.spawn_new_tiles` | 1401 | Board, Cell | Tile | TileSpawned | O(w) | Cree des Tiles pour les Cell marquees CellFillRequested. Couleur aleatoire via mge-rng. w = largeur board |
| `despawn_matched_tiles` | `mge.puzzle.tile.v1.fn.despawn_matched_tiles` | 1402 | Tile | World | TileDespawned | O(m) | Supprime les entites Tile en etat Matched. m = tuiles matchees |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TileSpawned` | `mge.puzzle.tile.v1.event.tile_spawned` | `entity: EntityId, color: TileColor, row: u32, col: u32` | `spawn_new_tiles` | board (sync), ui (animation) |
| `TileDespawned` | `mge.puzzle.tile.v1.event.tile_despawned` | `entity: EntityId, row: u32, col: u32` | `despawn_matched_tiles` | board (sync), score, ui |
| `TileStateChanged` | `mge.puzzle.tile.v1.event.tile_state_changed` | `entity: EntityId, old_state: TileState, new_state: TileState` | `update_tile_states` | ui (animation), hint |

---

## 7. Invariants

- Une Tile a toujours un (row, col) valide dans les bornes du Board.
- Une Tile en etat `Matched` sera despawnee dans le tick courant par `despawn_matched_tiles`.
- `TileModifier` n'existe que sur des Tiles avec state != `Locked`.
- Une Tile `Wild` matche avec n'importe quelle couleur.
- Apres `spawn_new_tiles`, toute Tile creee est en etat `Spawning`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `tile_colors_count` | `u32` | 5 | [3, 7] | Nombre de couleurs actives (hors Wild) |
| `use_wild_tiles` | `bool` | false | {true, false} | Active les tuiles Wild |
| `wild_spawn_chance` | `f32` | 0.05 | [0.0, 0.3] | Probabilite de spawn d'une tuile Wild |
| `spawn_modifiers` | `bool` | false | {true, false} | Active les TileModifier (bomb, line clear) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke le type et l'etat des tuiles | Ne gere pas la grille (→ board) |
| Gere le cycle de vie des tuiles (spawn, despawn) | Ne detecte pas les matchs (→ match) |
| Supporte les tuiles Wild et modifiees | Ne gere pas les echanges (→ swap) |
| Emet les evenements de cycle de vie | Ne calcule pas le score (→ match) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Tile, Board, Cell |
| Ecrit | Tile, World (despawn) |
| Emet | TileSpawned, TileDespawned, TileStateChanged |
| Ne touche jamais | Score, ComboChain, Goal, PuzzleTimer, SwapAction |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-tile/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.puzzle.tile.v1, trait Plugin impl
    ├── components.rs     # Tile, TileModifier
    ├── systems.rs        # update_tile_states, spawn_new_tiles, despawn_matched_tiles
    └── events.rs         # TileSpawned, TileDespawned, TileStateChanged
```

### Annotations MSCM requises

**lib.rs** :
```rust
//! @id mge.puzzle.tile.v1
//! @role plugin
//! @layer plugin
//! @domain puzzle
//! @do manage_tile_types_states_lifecycle
```

**Chaque composant** dans components.rs :
```rust
//! @id mge.puzzle.tile.v1.component.{name}
//! @role data
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}
```

**Chaque systeme** dans systems.rs :
```rust
//! @id mge.puzzle.tile.v1.fn.{name}
//! @role system
//! @layer plugin
//! @do {description}
//! @requires {Comp1},{Comp2}
//! @writes {Comp1}
//! @emits {Event1} | none
//! @phase {N}
//! @complexity O(n)
```

**Chaque evenement** dans events.rs :
```rust
//! @id mge.puzzle.tile.v1.event.{name}
//! @role event
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire (pas de static mut, lazy_static, thread_local) |
| No dynamic dispatch hot path | Obligatoire (update_tile_states) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (TileColor, TileState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : tile spawn, state transitions, despawn, wild matching
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.tile.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.tile.v1.component.tile","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.tile.v1.component.tile_modifier","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.tile.v1.fn.update_tile_states","k":"s","d":"puzzle","r":["Tile"],"w":["Tile"],"e":["TileStateChanged"],"p":1400,"c":"O(n)"},
  {"i":"mge.puzzle.tile.v1.fn.spawn_new_tiles","k":"s","d":"puzzle","r":["Board","Cell"],"w":["Tile"],"e":["TileSpawned"],"p":1401,"c":"O(w)"},
  {"i":"mge.puzzle.tile.v1.fn.despawn_matched_tiles","k":"s","d":"puzzle","r":["Tile"],"w":["World"],"e":["TileDespawned"],"p":1402,"c":"O(m)"},
  {"i":"mge.puzzle.tile.v1.event.tile_spawned","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.tile.v1.event.tile_despawned","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.tile.v1.event.tile_state_changed","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let tile_entity = world.spawn();
world.insert(tile_entity, Tile {
    color: TileColor::Red,
    state: TileState::Idle,
    row: 3,
    col: 5,
});

// Optionnel : ajouter un modificateur
world.insert(tile_entity, TileModifier {
    is_bomb: true,
    is_line_clear: false,
    is_color_bomb: false,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
