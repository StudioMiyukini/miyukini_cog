# mge-rts-fog-of-war

> @id mge.rts.fog-of-war.v1  
> @role plugin  
> @domain rts  
> @do manage_fog_visibility_team_vision_sharing  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-fog-of-war` |
| @id MSCM | `mge.rts.fog-of-war.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-social-faction` |
| Hot path | Oui (grille recalculee chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(v * r^2) ou v=sources de vision, r=rayon moyen |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `VisibilityState` | `Hidden, Explored, Visible` | Etat d'une tuile de brouillard |
| `VisionType` | `Normal, Extended, Detector` | Type de vision (standard, tour de guet, detection invisible) |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `VisionSource` | `mge.rts.fog-of-war.v1.component.vision_source` | `radius: f32, team: u8, vision_type: VisionType, active: bool` | Source de vision attachee a une entite. radius en tuiles |
| `FogTile` | `mge.rts.fog-of-war.v1.component.fog_tile` | `state: VisibilityState, last_seen_tick: u64, seen_by_teams: u16` | Tuile individuelle du brouillard. seen_by_teams est un bitmask |
| `FogGrid` | `mge.rts.fog-of-war.v1.component.fog_grid` | `width: u16, height: u16, tile_size: f32, tiles: Vec<FogTile>` | Grille complete du brouillard de guerre. Singleton |

---

## 4. Formules

```
tile_x       = floor(world_x / tile_size)
tile_y       = floor(world_y / tile_size)
dist         = sqrt((tile_x - source_tile_x)^2 + (tile_y - source_tile_y)^2)
in_vision    = dist <= radius
```

Optimisation : parcourir uniquement le carre englobant [source - radius, source + radius] par source de vision.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_vision_sources` | `mge.rts.fog-of-war.v1.fn.update_vision_sources` | PostLogic (1160) | VisionSource, Position2D | VisionSource | none | O(v) | Met a jour les positions des sources de vision |
| `compute_fog_grid` | `mge.rts.fog-of-war.v1.fn.compute_fog_grid` | PostLogic (1161) | VisionSource, FogGrid | FogGrid | AreaRevealed | O(v * r^2) | Recalcule la visibilite de chaque tuile. Visible→Explored si plus vue |
| `apply_fog_visibility` | `mge.rts.fog-of-war.v1.fn.apply_fog_visibility` | PostLogic (1162) | FogGrid | FogGrid | AreaHidden | O(w * h) | Marque les tuiles non-vues comme Explored (conserve la carte revelee) |
| `share_team_vision` | `mge.rts.fog-of-war.v1.fn.share_team_vision` | PostLogic (1163) | FogGrid, FactionRelation | FogGrid | EnemySpotted | O(t * w * h) | Partage la vision entre equipes alliees. Emet EnemySpotted |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AreaRevealed` | `mge.rts.fog-of-war.v1.event.area_revealed` | `tile_x: u16, tile_y: u16, team: u8` | `compute_fog_grid` | minimap, ui |
| `AreaHidden` | `mge.rts.fog-of-war.v1.event.area_hidden` | `tile_x: u16, tile_y: u16, team: u8` | `apply_fog_visibility` | minimap, ui |
| `EnemySpotted` | `mge.rts.fog-of-war.v1.event.enemy_spotted` | `entity: EntityId, team: u8, position: (f32, f32)` | `share_team_vision` | ai, minimap, audio |

---

## 7. Invariants

- `FogGrid` est un singleton : une seule instance par monde.
- `FogTile.state` ne peut passer de `Explored` a `Hidden` (la carte reste revelee).
- `FogTile.seen_by_teams` est un bitmask coherent avec les VisionSource actives.
- `VisionSource.active = false` desactive la contribution sans supprimer l'entite.
- La grille est recalculee entierement chaque tick (pas de cache persistant).
- Les entites dans une zone `Hidden` ne sont pas visibles pour l'equipe correspondante.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `grid_tile_size` | `f32` | 2.0 | [0.5, 10.0] | Taille d'une tuile de la grille de brouillard en unites monde |
| `default_vision_radius` | `f32` | 8.0 | [2.0, 30.0] | Rayon de vision par defaut en tuiles |
| `shared_vision_enabled` | `bool` | true | — | Active le partage de vision entre allies |
| `reveal_on_explore` | `bool` | true | — | Les tuiles explorees restent revelees (terrain visible, unites cachees) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Calcule la grille de brouillard de guerre | Ne gere pas le rendu du brouillard (→ ui) |
| Partage la vision entre allies | Ne gere pas les factions (→ social-faction) |
| Detecte les ennemis entrant en zone de vision | Ne gere pas la stealth / invisibilite (→ v2) |
| Conserve la carte exploree | Ne gere pas les hauteurs de terrain (→ v2) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | VisionSource, Position2D, FogGrid, FactionRelation |
| Ecrit | VisionSource, FogGrid, FogTile |
| Emet | AreaRevealed, AreaHidden, EnemySpotted |
| Ne touche jamais | Selection, ProductionQueue, ResourceNode, Building, OrderQueue, TechNode |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-fog-of-war/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.fog-of-war.v1, trait Plugin impl
    ├── components.rs     # VisionSource, FogTile, FogGrid
    ├── systems.rs        # update_vision_sources, compute_fog_grid, apply_fog_visibility, share_team_vision
    └── events.rs         # AreaRevealed, AreaHidden, EnemySpotted
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (compute_fog_grid) |
| No allocation hot path | Obligatoire (pre-allouer la grille) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (VisibilityState, VisionType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : vision circle, fog compute, team sharing, enemy spotting
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.fog-of-war.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.fog-of-war.v1.component.vision_source","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.fog-of-war.v1.component.fog_tile","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.fog-of-war.v1.component.fog_grid","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.fog-of-war.v1.fn.update_vision_sources","k":"s","d":"rts","r":["VisionSource","Position2D"],"w":["VisionSource"],"e":[],"p":1160,"c":"O(v)"},
  {"i":"mge.rts.fog-of-war.v1.fn.compute_fog_grid","k":"s","d":"rts","r":["VisionSource","FogGrid"],"w":["FogGrid"],"e":["AreaRevealed"],"p":1161,"c":"O(v*r^2)"},
  {"i":"mge.rts.fog-of-war.v1.fn.apply_fog_visibility","k":"s","d":"rts","r":["FogGrid"],"w":["FogGrid"],"e":["AreaHidden"],"p":1162,"c":"O(w*h)"},
  {"i":"mge.rts.fog-of-war.v1.fn.share_team_vision","k":"s","d":"rts","r":["FogGrid","FactionRelation"],"w":["FogGrid"],"e":["EnemySpotted"],"p":1163,"c":"O(t*w*h)"},
  {"i":"mge.rts.fog-of-war.v1.event.area_revealed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.fog-of-war.v1.event.area_hidden","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.fog-of-war.v1.event.enemy_spotted","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let fog = world.spawn();
let tiles = vec![FogTile { state: VisibilityState::Hidden, last_seen_tick: 0, seen_by_teams: 0 }; 128 * 128];
world.insert(fog, FogGrid {
    width: 128,
    height: 128,
    tile_size: 2.0,
    tiles,
});

let scout = world.spawn();
world.insert(scout, VisionSource {
    radius: 10.0,
    team: 1,
    vision_type: VisionType::Normal,
    active: true,
});
world.insert(scout, Position2D { x: 50.0, y: 50.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
