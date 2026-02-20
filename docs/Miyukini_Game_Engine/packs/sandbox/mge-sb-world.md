# mge-sb-world

> @id mge.sandbox.world.v1  
> @role plugin  
> @domain sandbox  
> @do manage_world_chunks_loading_persistence  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-world` |
| @id MSCM | `mge.sandbox.world.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-sb-terrain` |
| Hot path | Oui (update_active_chunks chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(r^2) r = rayon de chargement |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ChunkState` | `Unloaded, Loading, Loaded, Modified, Saving` | Cycle de vie d'un chunk |
| `WorldGenType` | `Flat, Perlin, Islands, Custom` | Type de generation procedurale |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `WorldConfig` | `mge.sandbox.world.v1.component.world_config` | `seed: u64, chunk_size: u32, gen_type: WorldGenType` | Configuration du monde. Singleton. Immuable apres init |
| `Chunk` | `mge.sandbox.world.v1.component.chunk` | `x: i32, z: i32, state: ChunkState, modified: bool` | Chunk individuel. Position en coordonnees chunk |
| `ActiveChunks` | `mge.sandbox.world.v1.component.active_chunks` | `center_x: i32, center_z: i32, load_radius: u32` | Zone active centree sur le joueur. Singleton |

---

## 4. Formules

Aucune formule de derivation. La generation de terrain est deleguee au terrain plugin.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_active_chunks` | `mge.sandbox.world.v1.fn.update_active_chunks` | 1500 | ActiveChunks, Position | ActiveChunks | none | O(1) | Met a jour center_x/z selon la position du joueur |
| `load_chunks` | `mge.sandbox.world.v1.fn.load_chunks` | 1501 | ActiveChunks, Chunk, WorldConfig | Chunk | ChunkLoaded | O(r^2) | Charge les chunks dans le rayon. Cree les entites Chunk manquantes. Emet ChunkLoaded |
| `save_modified_chunks` | `mge.sandbox.world.v1.fn.save_modified_chunks` | 1502 | Chunk | Chunk | ChunkUnloaded | O(c) | Sauvegarde les chunks Modified hors rayon. Passe en Saving puis Unloaded. c = chunks modifies |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ChunkLoaded` | `mge.sandbox.world.v1.event.chunk_loaded` | `chunk: EntityId, x: i32, z: i32` | `load_chunks` | terrain (generer tiles), wildlife (spawn), ui |
| `ChunkUnloaded` | `mge.sandbox.world.v1.event.chunk_unloaded` | `chunk: EntityId, x: i32, z: i32` | `save_modified_chunks` | wildlife (despawn), ui |
| `WorldSeedChanged` | `mge.sandbox.world.v1.event.world_seed_changed` | `old_seed: u64, new_seed: u64` | Externe (new game) | terrain (regeneration) |

---

## 7. Invariants

- `WorldConfig` est immuable apres initialisation. Le seed ne change que via new game.
- Un Chunk a toujours des coordonnees (x, z) uniques.
- Un Chunk `Loading` ne peut pas etre accede pour lecture de terrain.
- Les chunks hors rayon avec `modified = true` sont sauvegardes avant decharge.
- Le nombre de chunks charges simultanement <= (2*load_radius + 1)^2.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `chunk_size` | `u32` | 16 | [8, 64] | Taille d'un chunk en tiles |
| `load_radius` | `u32` | 3 | [1, 8] | Rayon de chargement en chunks |
| `world_gen_type` | `WorldGenType` | Perlin | {Flat, Perlin, Islands, Custom} | Type de generation |
| `auto_save_interval_ticks` | `u32` | 6000 | [600, 36000] | Intervalle sauvegarde auto (100s@60fps) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le chargement/dechargement des chunks | Ne genere pas le contenu du terrain (→ terrain) |
| Persiste les chunks modifies | Ne gere pas les batiments (→ building) |
| Suit la position du joueur pour la zone active | Ne gere pas la meteo (→ weather) |
| Supporte la generation procedurale via seed | Ne fait pas le rendu (→ core render) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | WorldConfig, Chunk, ActiveChunks, Position |
| Ecrit | Chunk, ActiveChunks |
| Emet | ChunkLoaded, ChunkUnloaded, WorldSeedChanged |
| Ne touche jamais | TerrainTile, Building, Weather, Need, Agent, Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-world/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.world.v1, trait Plugin impl
    ├── components.rs     # WorldConfig, Chunk, ActiveChunks
    ├── systems.rs        # update_active_chunks, load_chunks, save_modified_chunks
    └── events.rs         # ChunkLoaded, ChunkUnloaded, WorldSeedChanged
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (update_active_chunks) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (ChunkState, WorldGenType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : chunk load/unload, radius, save modified, seed
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.world.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.world.v1.component.world_config","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.world.v1.component.chunk","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.world.v1.component.active_chunks","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.world.v1.fn.update_active_chunks","k":"s","d":"sandbox","r":["ActiveChunks","Position"],"w":["ActiveChunks"],"e":[],"p":1500,"c":"O(1)"},
  {"i":"mge.sandbox.world.v1.fn.load_chunks","k":"s","d":"sandbox","r":["ActiveChunks","Chunk","WorldConfig"],"w":["Chunk"],"e":["ChunkLoaded"],"p":1501,"c":"O(r^2)"},
  {"i":"mge.sandbox.world.v1.fn.save_modified_chunks","k":"s","d":"sandbox","r":["Chunk"],"w":["Chunk"],"e":["ChunkUnloaded"],"p":1502,"c":"O(c)"},
  {"i":"mge.sandbox.world.v1.event.chunk_loaded","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.world.v1.event.chunk_unloaded","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.world.v1.event.world_seed_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let world_entity = world.spawn();
world.insert(world_entity, WorldConfig { seed: 42, chunk_size: 16, gen_type: WorldGenType::Perlin });
world.insert(world_entity, ActiveChunks { center_x: 0, center_z: 0, load_radius: 3 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sb-terrain](mge-sb-terrain.md) | Plugin terrain (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
