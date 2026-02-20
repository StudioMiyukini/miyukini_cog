# mge-rl-procgen

> @id mge.rl.procgen.v1  
> @role plugin  
> @domain roguelike  
> @do procedural_dungeon_generation_rooms_corridors  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rl-procgen` |
| @id MSCM | `mge.rl.procgen.v1` |
| Domaine | roguelike |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial` |
| Hot path | Non (one-shot a la generation) |
| Headless safe | Oui |
| Complexite globale | O(r^2) par generation, r = nombre de salles |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RoomType` | `Start, Normal, Treasure, Boss, Shop, Secret` | Type de salle. Influe sur le contenu et la difficulte |
| `GenerationAlgo` | `BSP, Random, Cellular, WFC` | Algorithme de generation. BSP = Binary Space Partition |
| `CorridorStyle` | `Straight, LShape, Winding` | Style de corridor entre salles |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `DungeonSeed` | `mge.rl.procgen.v1.component.dungeon_seed` | `seed: u64, floor_number: u32` | Seed pour la generation deterministe. Combined avec floor_number |
| `RoomGraph` | `mge.rl.procgen.v1.component.room_graph` | `rooms: Vec<RoomNode>, edges: Vec<(usize, usize)>` | Graphe de connectivite des salles. RoomNode = {room_type, bounds, center} |
| `RoomConfig` | `mge.rl.procgen.v1.component.room_config` | `min_size: UVec2, max_size: UVec2, padding: u32` | Contraintes de taille pour une salle |
| `DungeonConfig` | `mge.rl.procgen.v1.component.dungeon_config` | `algo: GenerationAlgo, room_count_min: u32, room_count_max: u32, map_size: UVec2, corridor_style: CorridorStyle` | Configuration globale de generation |

---

## 4. Formules

```
BSP Split :
  split_axis = if width > height { Horizontal } else { Vertical }
  split_pos = rng.range(min_size + padding, axis_size - min_size - padding)

Nombre de salles :
  count = rng.range(config.room_count_min, config.room_count_max + 1)

Distance minimale corridors :
  corridor_length = manhattan_distance(room_a.center, room_b.center)
  
Difficulte par profondeur :
  room_difficulty = base_difficulty + (floor_number * difficulty_scaling)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `generate_dungeon` | `mge.rl.procgen.v1.fn.generate_dungeon` | 1800 | GenerateDungeonRequest (event), DungeonConfig, DungeonSeed | RoomGraph | DungeonGenerated | O(r^2) | Execute l'algorithme de generation. Produit le graphe de salles |
| `place_rooms` | `mge.rl.procgen.v1.fn.place_rooms` | 1801 | RoomGraph, RoomConfig, DungeonSeed | RoomGraph (bounds) | RoomGenerated | O(r) | Dimensionne et positionne chaque salle dans l'espace 2D |
| `connect_rooms` | `mge.rl.procgen.v1.fn.connect_rooms` | 1802 | RoomGraph, DungeonConfig | RoomGraph (edges) | RoomConnected | O(r^2) | Cree les corridors entre salles adjacentes dans le graphe |
| `populate_room` | `mge.rl.procgen.v1.fn.populate_room` | 1803 | RoomGraph, DungeonSeed | World (spawn) | none | O(r) | Place les marqueurs de contenu (ennemis, coffres, pieges) dans chaque salle |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `DungeonGenerated` | `mge.rl.procgen.v1.event.dungeon_generated` | `floor_number: u32, room_count: u32, seed: u64` | `generate_dungeon` | floor (init), ui (minimap) |
| `RoomGenerated` | `mge.rl.procgen.v1.event.room_generated` | `room_index: usize, room_type: RoomType, bounds: Rect, center: Vec2` | `place_rooms` | floor (tiles), item (spawn) |
| `RoomConnected` | `mge.rl.procgen.v1.event.room_connected` | `from_index: usize, to_index: usize, corridor_style: CorridorStyle` | `connect_rooms` | floor (corridor tiles) |

---

## 7. Invariants

- Toute generation avec le meme `DungeonSeed` produit le meme resultat (determinisme strict).
- Le graphe de salles est toujours connexe — chaque salle est accessible.
- Exactement une salle `Start` et une salle `Boss` par donjon.
- Les salles ne se chevauchent jamais (padding minimum garanti).
- `room_count` est borne entre `room_count_min` et `room_count_max` inclus.
- Les corridors ne traversent jamais une salle existante.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_algo` | `GenerationAlgo` | BSP | {BSP, Random, Cellular, WFC} | Algorithme de generation |
| `room_count_min` | `u32` | 6 | [3, 50] | Nombre minimum de salles |
| `room_count_max` | `u32` | 12 | [5, 100] | Nombre maximum de salles |
| `map_width` | `u32` | 80 | [40, 500] | Largeur carte en tiles |
| `map_height` | `u32` | 60 | [30, 500] | Hauteur carte en tiles |
| `room_min_size` | `u32` | 5 | [3, 20] | Taille minimale d'une salle |
| `room_max_size` | `u32` | 15 | [8, 40] | Taille maximale d'une salle |
| `difficulty_scaling` | `f32` | 0.1 | [0.0, 1.0] | Augmentation difficulte par etage |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Genere le layout du donjon (salles, corridors) | Ne gere pas les tiles individuelles (→ floor) |
| Place les marqueurs de contenu | Ne spawne pas les ennemis concrets (→ game logic) |
| Garantit la connexite du graphe | Ne gere pas le brouillard de guerre (→ floor) |
| Supporte plusieurs algorithmes | Ne gere pas le rendu de la carte |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | DungeonConfig, DungeonSeed, RoomConfig, GenerateDungeonRequest |
| Ecrit | RoomGraph, World (marqueurs) |
| Emet | DungeonGenerated, RoomGenerated, RoomConnected |
| Ne touche jamais | FloorState, Tile, DoorState, RogueItem, RunState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rl-procgen/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rl.procgen.v1, trait Plugin impl
    ├── components.rs     # DungeonSeed, RoomGraph, RoomConfig, DungeonConfig
    ├── systems.rs        # generate_dungeon, place_rooms, connect_rooms, populate_room
    └── events.rs         # DungeonGenerated, RoomGenerated, RoomConnected
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (RoomType, GenerationAlgo, CorridorStyle)
- [ ] Algorithme BSP implemente en priorite
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : connexite, no overlap, determinisme seed, room counts
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rl.procgen.v1","k":"p","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.component.dungeon_seed","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.component.room_graph","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.component.room_config","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.component.dungeon_config","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.fn.generate_dungeon","k":"s","d":"roguelike","r":["DungeonConfig","DungeonSeed"],"w":["RoomGraph"],"e":["DungeonGenerated"],"p":1800,"c":"O(r^2)"},
  {"i":"mge.rl.procgen.v1.fn.place_rooms","k":"s","d":"roguelike","r":["RoomGraph","RoomConfig","DungeonSeed"],"w":["RoomGraph"],"e":["RoomGenerated"],"p":1801,"c":"O(r)"},
  {"i":"mge.rl.procgen.v1.fn.connect_rooms","k":"s","d":"roguelike","r":["RoomGraph","DungeonConfig"],"w":["RoomGraph"],"e":["RoomConnected"],"p":1802,"c":"O(r^2)"},
  {"i":"mge.rl.procgen.v1.fn.populate_room","k":"s","d":"roguelike","r":["RoomGraph","DungeonSeed"],"w":["World"],"e":[],"p":1803,"c":"O(r)"},
  {"i":"mge.rl.procgen.v1.event.dungeon_generated","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.event.room_generated","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.procgen.v1.event.room_connected","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let dungeon = world.spawn();
world.insert(dungeon, DungeonSeed { seed: 42, floor_number: 1 });
world.insert(dungeon, DungeonConfig {
    algo: GenerationAlgo::BSP,
    room_count_min: 6,
    room_count_max: 12,
    map_size: UVec2::new(80, 60),
    corridor_style: CorridorStyle::LShape,
});
world.insert(dungeon, RoomConfig {
    min_size: UVec2::new(5, 5),
    max_size: UVec2::new(15, 15),
    padding: 2,
});
world.push_event(GenerateDungeonRequest { dungeon_entity: dungeon });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Roguelike - Index](_index.md) | Vue d'ensemble du pack |
| [mge-rl-floor](mge-rl-floor.md) | Plugin etages (consomme le graphe genere) |
