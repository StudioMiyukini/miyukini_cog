# mge-gfp-trigger-zone

> @id mge.foundation.trigger_zone.v1  
> @role plugin  
> @domain foundation  
> @do define_trigger_zones_track_occupants  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-trigger-zone` |
| @id MSCM | `mge.foundation.trigger_zone.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-collider`, `mge-gfp-collision-detection` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(t*n), t = zones trigger, n = entites proches (via index spatial) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TriggerShape` | `AABB`, `Circle` | Definit la forme geometrique de la zone trigger |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TriggerZone` | `mge.foundation.trigger_zone.v1.component.trigger_zone` | `shape: TriggerShape, width: f32, height: f32, radius: f32, enabled: bool` | Definition d'une zone trigger avec sa forme et ses dimensions. `width`/`height` utilises pour AABB, `radius` pour Circle |
| `TriggerOccupants` | `mge.foundation.trigger_zone.v1.component.trigger_occupants` | `entities: Vec<EntityId>` | Liste des entites actuellement presentes a l'interieur de la zone trigger |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_trigger_occupancy` | `mge.foundation.trigger_zone.v1.fn.update_trigger_occupancy` | 160 | TriggerZone, WorldTransform, Collider | TriggerOccupants | none | O(t*n) | Met a jour la liste des entites presentes dans chaque zone trigger en utilisant le chevauchement de collision |

---

## 5. Flux de donnees

```
TriggerZone (shape, dimensions)
       │
       ▼
WorldTransform ──► Collider
       │                │
       ▼                ▼
 ┌─────────────────────────────────┐
 │     update_trigger_occupancy    │  Phase 160
 │  (overlap test zone ↔ entites) │
 └────────────────┬────────────────┘
                  │
                  ▼
       TriggerOccupants (entities[])
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `TriggerOccupants.entities` est recalcule entierement a chaque tick (pas d'accumulation entre ticks).
- Seules les zones avec `TriggerZone.enabled = true` sont evaluees.
- Pour `TriggerShape::AABB`, `width` et `height` doivent etre > 0.0.
- Pour `TriggerShape::Circle`, `radius` doit etre > 0.0.
- Une entite ne peut apparaitre qu'une seule fois dans `TriggerOccupants.entities` par zone.
- Le test d'overlap utilise la geometrie du collider et la forme de la zone, pas un simple point.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Definit des zones trigger avec forme geometrique | Ne detecte pas les collisions physiques (→ collision-detection) |
| Calcule quelles entites sont a l'interieur d'une zone | N'emet pas d'evenements enter/exit (→ enter-exit-event) |
| Maintient la liste des occupants par zone | Ne gere pas les conditions d'activation (→ area-condition) |
| Supporte les formes AABB et Circle | Ne gere pas les formes complexes (polygones) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | TriggerZone, WorldTransform, Collider |
| Ecrit | TriggerOccupants |
| Emet | rien |
| Ne touche jamais | Velocity2D, Transform2D, TriggerTracker, ConditionState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-trigger-zone/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.trigger_zone.v1, trait Plugin impl
    ├── components.rs     # TriggerZone, TriggerOccupants, TriggerShape
    ├── systems.rs        # update_trigger_occupancy
    └── events.rs         # (vide)
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | N/A |
| No allocation hot path | N/A |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 1 enum `TriggerShape` dans `components.rs`
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : overlap AABB, overlap Circle, zone desactivee, entites multiples
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.trigger_zone.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.trigger_zone.v1.component.trigger_zone","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.trigger_zone.v1.component.trigger_occupants","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.trigger_zone.v1.fn.update_trigger_occupancy","k":"s","d":"foundation","r":["TriggerZone","WorldTransform","Collider"],"w":["TriggerOccupants"],"e":[],"p":160,"c":"O(t*n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let zone = world.spawn();
world.insert(zone, TriggerZone {
    shape: TriggerShape::AABB, width: 64.0, height: 64.0, radius: 0.0, enabled: true,
});
world.insert(zone, Transform2D { x: 200.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(zone, WorldTransform { matrix: [1.0, 0.0, 0.0, 1.0, 200.0, 100.0] });
world.insert(zone, TriggerOccupants { entities: vec![] });
// Apres Phase 160 : TriggerOccupants.entities contient les entites chevauchant la zone
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-enter-exit-event](mge-gfp-enter-exit-event.md) | Evenements enter/exit bases sur les occupants |
| [mge-gfp-area-condition](mge-gfp-area-condition.md) | Conditions d'activation basees sur les zones |
