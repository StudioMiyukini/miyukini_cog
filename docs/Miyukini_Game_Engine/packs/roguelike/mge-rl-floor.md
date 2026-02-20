# mge-rl-floor

> @id mge.rl.floor.v1  
> @role plugin  
> @domain roguelike  
> @do manage_floor_tiles_doors_fog_of_war  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rl-floor` |
| @id MSCM | `mge.rl.floor.v1` |
| Domaine | roguelike |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-rl-procgen` |
| Hot path | Oui (reveal_tiles chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(v^2) par tick, v = rayon de vision |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TileType` | `Wall, Floor, Door, StairsDown, StairsUp, Trap, Water` | Type de tile. Influe sur la traversabilite et les interactions |
| `FogState` | `Hidden, Revealed, Visible` | Etat du brouillard. Hidden = jamais vu, Revealed = deja vu mais hors vision, Visible = en vision |
| `DoorStatus` | `Closed, Open, Locked, Broken` | Etat d'une porte. Locked necessite une cle ou competence |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `FloorState` | `mge.rl.floor.v1.component.floor_state` | `floor_number: u32, rooms_cleared: u32, total_rooms: u32, is_clear: bool` | Etat global de l'etage. is_clear = true quand toutes les salles sont nettoyees |
| `FloorMap` | `mge.rl.floor.v1.component.floor_map` | `tiles: Vec<Tile>, width: u32, height: u32, fog: Vec<FogState>` | Carte de l'etage. tiles et fog indexes par y * width + x |
| `Tile` | `mge.rl.floor.v1.component.tile` | `tile_type: TileType, walkable: bool, blocks_sight: bool` | Definition d'une tile. walkable et blocks_sight derives du tile_type |
| `RoomInstance` | `mge.rl.floor.v1.component.room_instance` | `room_index: usize, bounds: Rect, is_cleared: bool, enemies_remaining: u32` | Instance d'une salle sur l'etage courant. Lie au RoomGraph |
| `DoorState` | `mge.rl.floor.v1.component.door_state` | `position: IVec2, status: DoorStatus, key_id: Option<u32>` | Etat d'une porte. key_id = Some si Locked |

---

## 4. Formules

```
Index tile :
  index = y * floor_map.width + x

Rayon de vision (reveal) :
  for dx in -vision_radius..=vision_radius:
    for dy in -vision_radius..=vision_radius:
      if dx*dx + dy*dy <= vision_radius*vision_radius:
        if line_of_sight(player_pos, (px+dx, py+dy)):
          fog[index] = Visible

Transition fog fin de tick :
  for each tile where fog == Visible and not in current vision:
    fog[tile] = Revealed
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `init_floor` | `mge.rl.floor.v1.fn.init_floor` | 1820 | DungeonGenerated (event), RoomGraph | FloorMap, FloorState, RoomInstance, DoorState, World (spawn) | FloorEntered | O(w*h) | Convertit le RoomGraph en tiles. Place portes et escaliers. Initialise le fog a Hidden |
| `reveal_tiles` | `mge.rl.floor.v1.fn.reveal_tiles` | 1821 | Position2D (joueur), FloorMap | FloorMap (fog) | TileRevealed | O(v^2) | Met a jour le brouillard autour du joueur. Utilise line-of-sight |
| `process_door_interaction` | `mge.rl.floor.v1.fn.process_door_interaction` | 1822 | DoorInteractRequest (event), DoorState, Inventory (opt) | DoorState | DoorOpened | O(1) | Ouvre, deverrouille ou casse une porte selon son statut et l'inventaire |
| `check_floor_clear` | `mge.rl.floor.v1.fn.check_floor_clear` | 1823 | RoomInstance, FloorState | FloorState | FloorCleared | O(r) | Verifie si toutes les salles sont nettoyees. Si oui, emet FloorCleared |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `FloorEntered` | `mge.rl.floor.v1.event.floor_entered` | `floor_number: u32, room_count: u32` | `init_floor` | ui (affichage etage), permadeath (tracking) |
| `FloorCleared` | `mge.rl.floor.v1.event.floor_cleared` | `floor_number: u32, rooms_cleared: u32` | `check_floor_clear` | progression (bonus XP), ui |
| `TileRevealed` | `mge.rl.floor.v1.event.tile_revealed` | `position: IVec2, tile_type: TileType` | `reveal_tiles` | ui (rendu minimap), quest (exploration) |
| `DoorOpened` | `mge.rl.floor.v1.event.door_opened` | `position: IVec2, previous_status: DoorStatus` | `process_door_interaction` | ui (animation), ai (pathing update) |

---

## 7. Invariants

- Le FloorMap est immutable apres `init_floor` (seul le fog change pendant l'exploration).
- Un tile `Wall` est toujours `walkable == false` et `blocks_sight == true`.
- Un tile `StairsDown` existe exactement une fois par etage (sauf dernier etage).
- Le fog ne revient jamais a `Hidden` une fois `Revealed` ou `Visible`.
- Une porte `Locked` ne peut etre ouverte que si `key_id` correspond a un objet en inventaire.
- `FloorState.is_clear` ne passe a `true` qu'une seule fois (irreversible).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `vision_radius` | `u32` | 8 | [3, 30] | Rayon de vision du joueur en tiles |
| `door_break_chance` | `f32` | 0.0 | [0.0, 1.0] | Probabilite de casser une porte fermee sans cle |
| `trap_visible_revealed` | `bool` | false | {true, false} | Les pieges sont-ils visibles sur tiles revelees |
| `stairs_require_clear` | `bool` | false | {true, false} | Escaliers bloquees tant que l'etage n'est pas clear |
| `fog_memory` | `bool` | true | {true, false} | Les tiles revelees restent visibles sur la minimap |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la carte de tiles et le brouillard | Ne genere pas le layout (→ procgen) |
| Gere les portes et leurs interactions | Ne gere pas les ennemis (→ game logic) |
| Detecte si l'etage est clear | Ne gere pas les objets (→ item) |
| Fournit le line-of-sight | Ne gere pas le pathfinding complet (→ spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | RoomGraph, DungeonGenerated, Position2D, DoorInteractRequest, RoomInstance, FloorState |
| Ecrit | FloorMap, FloorState, RoomInstance, DoorState |
| Emet | FloorEntered, FloorCleared, TileRevealed, DoorOpened |
| Ne touche jamais | DungeonSeed, DungeonConfig, RogueItem, RunState, Tombstone |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rl-floor/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rl.floor.v1, trait Plugin impl
    ├── components.rs     # FloorState, FloorMap, Tile, RoomInstance, DoorState
    ├── systems.rs        # init_floor, reveal_tiles, process_door_interaction, check_floor_clear
    └── events.rs         # FloorEntered, FloorCleared, TileRevealed, DoorOpened
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (reveal_tiles) |
| No allocation hot path | Obligatoire (fog pre-alloue) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 5 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (TileType, FogState, DoorStatus)
- [ ] Line-of-sight avec Bresenham
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : fog reveal, door interaction, floor clear, tile walkability
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rl.floor.v1","k":"p","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.component.floor_state","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.component.floor_map","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.component.tile","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.component.room_instance","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.component.door_state","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.fn.init_floor","k":"s","d":"roguelike","r":["RoomGraph"],"w":["FloorMap","FloorState","RoomInstance","DoorState","World"],"e":["FloorEntered"],"p":1820,"c":"O(w*h)"},
  {"i":"mge.rl.floor.v1.fn.reveal_tiles","k":"s","d":"roguelike","r":["Position2D","FloorMap"],"w":["FloorMap"],"e":["TileRevealed"],"p":1821,"c":"O(v^2)"},
  {"i":"mge.rl.floor.v1.fn.process_door_interaction","k":"s","d":"roguelike","r":["DoorState"],"w":["DoorState"],"e":["DoorOpened"],"p":1822,"c":"O(1)"},
  {"i":"mge.rl.floor.v1.fn.check_floor_clear","k":"s","d":"roguelike","r":["RoomInstance","FloorState"],"w":["FloorState"],"e":["FloorCleared"],"p":1823,"c":"O(r)"},
  {"i":"mge.rl.floor.v1.event.floor_entered","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.event.floor_cleared","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.event.tile_revealed","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.floor.v1.event.door_opened","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
// Apres reception de DungeonGenerated, init_floor est appele automatiquement.
// Pour interagir avec une porte :
world.push_event(DoorInteractRequest {
    entity: player,
    door_position: IVec2::new(15, 22),
});
// Pour verifier l'etat d'un tile :
let idx = 22 * floor_map.width + 15;
let tile = &floor_map.tiles[idx as usize];
let fog = floor_map.fog[idx as usize];
```

---

## References

| Document | Role |
|----------|------|
| [Pack Roguelike - Index](_index.md) | Vue d'ensemble du pack |
| [mge-rl-procgen](mge-rl-procgen.md) | Plugin procgen (fournit RoomGraph) |
