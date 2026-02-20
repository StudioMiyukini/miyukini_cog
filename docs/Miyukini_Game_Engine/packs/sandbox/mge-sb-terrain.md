# mge-sb-terrain

> @id mge.sandbox.terrain.v1  
> @role plugin  
> @domain sandbox  
> @do manage_terrain_tiles_modification_fertility  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-terrain` |
| @id MSCM | `mge.sandbox.terrain.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (modifications ponctuelles) |
| Headless safe | Oui |
| Complexite globale | O(m) par tick, m = modifications en attente |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TerrainType` | `Grass, Dirt, Sand, Stone, Water, Snow, Mud` | Type de sol |
| `TerrainLayer` | `Ground, Surface, Vegetation` | Couche du terrain (empilable) |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TerrainTile` | `mge.sandbox.terrain.v1.component.terrain_tile` | `terrain_type: TerrainType, layer: TerrainLayer, x: u32, z: u32, height: f32` | Tile de terrain. Position dans le chunk |
| `TerrainModification` | `mge.sandbox.terrain.v1.component.terrain_modification` | `target_x: u32, target_z: u32, new_type: TerrainType` | Requete de modification. Consommee dans le tick |
| `Fertility` | `mge.sandbox.terrain.v1.component.fertility` | `value: f32, moisture: f32` | Fertilite du sol. Affecte croissance (saisons). 0.0-1.0 |

---

## 4. Formules

```
fertility_update:
  value += (moisture - 0.5) * fertility_regen_rate * season_growth_multiplier
  value = clamp(value, 0.0, 1.0)

erosion:
  if terrain_type == Dirt && moisture > erosion_threshold → terrain_type = Mud
  if terrain_type == Sand && height > 0 → height -= erosion_rate
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_terrain_modifications` | `mge.sandbox.terrain.v1.fn.apply_terrain_modifications` | 1505 | TerrainModification, TerrainTile | TerrainTile | TerrainModified | O(m) | Applique les requetes de modification. Met a jour terrain_type. Consomme TerrainModification |
| `update_fertility` | `mge.sandbox.terrain.v1.fn.update_fertility` | 1506 | Fertility, TerrainTile, SeasonEffect | Fertility | FertilityChanged | O(t) | Recalcule fertility.value selon moisture et saison. t = tiles avec Fertility |
| `erode_terrain` | `mge.sandbox.terrain.v1.fn.erode_terrain` | 1507 | TerrainTile, Fertility | TerrainTile | TerrainModified | O(t) | Applique l'erosion naturelle selon les regles de terrain |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TerrainModified` | `mge.sandbox.terrain.v1.event.terrain_modified` | `x: u32, z: u32, old_type: TerrainType, new_type: TerrainType` | `apply_terrain_modifications`, `erode_terrain` | building (re-valider), world (chunk.modified), ui |
| `FertilityChanged` | `mge.sandbox.terrain.v1.event.fertility_changed` | `x: u32, z: u32, old_value: f32, new_value: f32` | `update_fertility` | season (growth), wildlife (habitat), ui |

---

## 7. Invariants

- `Fertility.value` est toujours dans [0.0, 1.0].
- `Fertility.moisture` est toujours dans [0.0, 1.0].
- Un TerrainTile de type `Water` a toujours `height <= 0.0`.
- Une TerrainModification est consommee dans le tick ou elle est creee.
- L'erosion ne transforme jamais `Stone` (resistant).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `fertility_regen_rate` | `f32` | 0.01 | [0.0, 0.1] | Vitesse de regeneration fertilite/tick |
| `erosion_rate` | `f32` | 0.001 | [0.0, 0.01] | Vitesse d'erosion naturelle |
| `erosion_threshold` | `f32` | 0.8 | [0.5, 1.0] | Seuil moisture pour erosion |
| `enable_erosion` | `bool` | true | {true, false} | Active/desactive l'erosion |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke et modifie les tiles terrain | Ne gere pas les chunks (→ world) |
| Calcule la fertilite | Ne gere pas les saisons (→ season) |
| Applique l'erosion naturelle | Ne gere pas la construction (→ building) |
| Emet les evenements de modification | Ne fait pas le rendu (→ core render) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | TerrainTile, TerrainModification, Fertility, SeasonEffect |
| Ecrit | TerrainTile, Fertility |
| Emet | TerrainModified, FertilityChanged |
| Ne touche jamais | Building, CraftingStation, Need, Agent, Weather, Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-terrain/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.terrain.v1, trait Plugin impl
    ├── components.rs     # TerrainTile, TerrainModification, Fertility
    ├── systems.rs        # apply_terrain_modifications, update_fertility, erode_terrain
    └── events.rs         # TerrainModified, FertilityChanged
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
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (TerrainType, TerrainLayer)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : modification apply, fertility update, erosion, clamp
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.terrain.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.terrain.v1.component.terrain_tile","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.terrain.v1.component.terrain_modification","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.terrain.v1.component.fertility","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.terrain.v1.fn.apply_terrain_modifications","k":"s","d":"sandbox","r":["TerrainModification","TerrainTile"],"w":["TerrainTile"],"e":["TerrainModified"],"p":1505,"c":"O(m)"},
  {"i":"mge.sandbox.terrain.v1.fn.update_fertility","k":"s","d":"sandbox","r":["Fertility","TerrainTile","SeasonEffect"],"w":["Fertility"],"e":["FertilityChanged"],"p":1506,"c":"O(t)"},
  {"i":"mge.sandbox.terrain.v1.fn.erode_terrain","k":"s","d":"sandbox","r":["TerrainTile","Fertility"],"w":["TerrainTile"],"e":["TerrainModified"],"p":1507,"c":"O(t)"},
  {"i":"mge.sandbox.terrain.v1.event.terrain_modified","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.terrain.v1.event.fertility_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let tile = world.spawn();
world.insert(tile, TerrainTile {
    terrain_type: TerrainType::Grass,
    layer: TerrainLayer::Ground,
    x: 5, z: 12, height: 1.0,
});
world.insert(tile, Fertility { value: 0.7, moisture: 0.5 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
