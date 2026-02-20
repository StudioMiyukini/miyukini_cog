# mge-rts-building

> @id mge.rts.building.v1  
> @role plugin  
> @domain rts  
> @do manage_building_placement_construction_demolition  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-building` |
| @id MSCM | `mge.rts.building.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-rts-resource` |
| Hot path | Non (construction avance par tick mais peu de batiments simultanes) |
| Headless safe | Oui |
| Complexite globale | O(b) ou b=batiments en construction |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `BuildingState` | `Blueprint, UnderConstruction, Built, Damaged, Demolished` | Etat du cycle de vie du batiment |
| `PlacementValidity` | `Valid, Blocked, OutOfBounds, InsufficientResources` | Resultat de la validation du placement |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Building` | `mge.rts.building.v1.component.building` | `type_id: u32, state: BuildingState, hp: f32, max_hp: f32, build_progress: f32, build_duration: f32` | Batiment avec etat, HP et progression de construction |
| `BuildSite` | `mge.rts.building.v1.component.build_site` | `builder: Option<EntityId>, cost: ResourceCost, workers_assigned: u8` | Chantier de construction. builder = constructeur principal assigne |
| `Footprint` | `mge.rts.building.v1.component.footprint` | `width: u16, height: u16, blocked_tiles: Vec<(u16, u16)>` | Empreinte au sol du batiment en tuiles |

---

## 4. Formules

```
build_delta     = dt * build_speed * (1.0 + extra_workers * worker_bonus)
build_progress  = min(build_progress + build_delta, build_duration)
complete        = build_progress >= build_duration
hp_during_build = max_hp * (build_progress / build_duration)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `advance_construction` | `mge.rts.building.v1.fn.advance_construction` | Logic (1130) | Building, BuildSite | Building | ConstructionProgress | O(b) | Avance le build_progress des batiments en construction |
| `place_building` | `mge.rts.building.v1.fn.place_building` | Logic (1131) | Footprint, ResourceDepot | Building, BuildSite, Footprint, ResourceDepot | BuildingPlaced | O(f) | Valide le placement, deduit les ressources, cree le blueprint |
| `complete_building` | `mge.rts.building.v1.fn.complete_building` | Logic (1132) | Building | Building | BuildingCompleted | O(b) | Finalise la construction, passe le state a Built |
| `demolish_building` | `mge.rts.building.v1.fn.demolish_building` | Logic (1133) | Building, Footprint | Building, Footprint | BuildingDemolished | O(1) | Demolit un batiment, libere les tuiles, rembourse partiellement |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `BuildingPlaced` | `mge.rts.building.v1.event.building_placed` | `building: EntityId, type_id: u32, position: (f32, f32)` | `place_building` | ui, minimap, fog-of-war |
| `ConstructionProgress` | `mge.rts.building.v1.event.construction_progress` | `building: EntityId, progress_pct: f32` | `advance_construction` | ui |
| `BuildingCompleted` | `mge.rts.building.v1.event.building_completed` | `building: EntityId, type_id: u32` | `complete_building` | production, tech, ui, audio |
| `BuildingDemolished` | `mge.rts.building.v1.event.building_demolished` | `building: EntityId, refund: ResourceCost` | `demolish_building` | resource, ui |

---

## 7. Invariants

- `Building.build_progress` est toujours dans [0.0, build_duration].
- `Building.hp` est toujours dans [0.0, max_hp].
- Un batiment `Blueprint` n'est pas fonctionnel (pas de production, pas de vision).
- Le `Footprint` bloque les tuiles tant que le batiment existe (meme en construction).
- La demolition libere toutes les tuiles du footprint.
- Les ressources sont deduites au placement, pas a la completion.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `default_build_speed` | `f32` | 1.0 | [0.1, 5.0] | Vitesse de construction par defaut (multiplicateur) |
| `worker_bonus` | `f32` | 0.2 | [0.0, 1.0] | Bonus de vitesse par ouvrier supplementaire |
| `max_workers_per_site` | `u8` | 5 | [1, 10] | Nombre max d'ouvriers par chantier |
| `demolish_refund_pct` | `f32` | 0.5 | [0.0, 1.0] | Pourcentage de ressources remboursees a la demolition |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le placement, construction et demolition | Ne gere pas les ressources (→ resource) |
| Valide le placement sur la grille | Ne gere pas la production depuis le batiment (→ production) |
| Progresse la construction chaque tick | Ne gere pas les degats au batiment (→ combat externe) |
| Gere l'empreinte au sol | Ne gere pas le pathfinding autour (→ spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Building, BuildSite, Footprint, ResourceDepot |
| Ecrit | Building, BuildSite, Footprint, ResourceDepot |
| Emet | BuildingPlaced, ConstructionProgress, BuildingCompleted, BuildingDemolished |
| Ne touche jamais | Selection, ProductionQueue, OrderQueue, FogGrid, TechNode, ResourceNode |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-building/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.building.v1, trait Plugin impl
    ├── components.rs     # Building, BuildSite, Footprint
    ├── systems.rs        # advance_construction, place_building, complete_building, demolish_building
    └── events.rs         # BuildingPlaced, ConstructionProgress, BuildingCompleted, BuildingDemolished
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (BuildingState, PlacementValidity)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : placement, construction tick, completion, demolition
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.building.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.component.building","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.component.build_site","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.component.footprint","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.fn.advance_construction","k":"s","d":"rts","r":["Building","BuildSite"],"w":["Building"],"e":["ConstructionProgress"],"p":1130,"c":"O(b)"},
  {"i":"mge.rts.building.v1.fn.place_building","k":"s","d":"rts","r":["Footprint","ResourceDepot"],"w":["Building","BuildSite","Footprint","ResourceDepot"],"e":["BuildingPlaced"],"p":1131,"c":"O(f)"},
  {"i":"mge.rts.building.v1.fn.complete_building","k":"s","d":"rts","r":["Building"],"w":["Building"],"e":["BuildingCompleted"],"p":1132,"c":"O(b)"},
  {"i":"mge.rts.building.v1.fn.demolish_building","k":"s","d":"rts","r":["Building","Footprint"],"w":["Building","Footprint"],"e":["BuildingDemolished"],"p":1133,"c":"O(1)"},
  {"i":"mge.rts.building.v1.event.building_placed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.event.construction_progress","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.event.building_completed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.building.v1.event.building_demolished","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let tower = world.spawn();
world.insert(tower, Building {
    type_id: 10,
    state: BuildingState::Blueprint,
    hp: 0.0,
    max_hp: 500.0,
    build_progress: 0.0,
    build_duration: 30.0,
});
world.insert(tower, BuildSite {
    builder: None,
    cost: ResourceCost { amounts: HashMap::from([(ResourceKind::Wood, 100), (ResourceKind::Stone, 50)]) },
    workers_assigned: 0,
});
world.insert(tower, Footprint {
    width: 3,
    height: 3,
    blocked_tiles: vec![(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2)],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
