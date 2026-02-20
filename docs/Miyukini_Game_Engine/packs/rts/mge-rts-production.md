# mge-rts-production

> @id mge.rts.production.v1  
> @role plugin  
> @domain rts  
> @do manage_production_queues_build_times_cancellation  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-production` |
| @id MSCM | `mge.rts.production.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rts-resource` |
| Hot path | Oui (tick chaque frame pour toutes les queues actives) |
| Headless safe | Oui |
| Complexite globale | O(p) ou p=producteurs actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ProductionState` | `Idle, Producing, Paused, Cancelled` | Etat courant d'un producteur |
| `ProductionKind` | `Unit, Building, Upgrade` | Type d'element produit |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ProductionQueue` | `mge.rts.production.v1.component.production_queue` | `entries: Vec<ProductionEntry>, max_size: u8, state: ProductionState` | File de production ordonnee. max_size limite les elements en attente |
| `ProductionEntry` | `mge.rts.production.v1.component.production_entry` | `kind: ProductionKind, type_id: u32, progress: f32, duration: f32, cost: ResourceCost` | Element en production. progress [0, duration] en secondes |
| `Producer` | `mge.rts.production.v1.component.producer` | `entity: EntityId, rally_point: Option<(f32, f32)>` | Lie un batiment/entite a sa capacite de production. rally_point = point de ralliement |

---

## 4. Formules

```
progress_delta  = dt * production_speed_modifier
progress_new    = min(progress + progress_delta, duration)
complete        = progress_new >= duration
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_production` | `mge.rts.production.v1.fn.tick_production` | Logic (1110) | ProductionQueue, ProductionEntry | ProductionEntry | none | O(p) | Avance le progress de l'entree en tete de chaque queue active |
| `start_production` | `mge.rts.production.v1.fn.start_production` | Logic (1111) | ProductionQueue, ResourceDepot | ProductionQueue, ResourceDepot | ProductionStarted, InsufficientResources | O(r) | Ajoute une entree a la queue si ressources suffisantes. Deduit le cout |
| `cancel_production` | `mge.rts.production.v1.fn.cancel_production` | Logic (1112) | ProductionQueue | ProductionQueue, ResourceDepot | ProductionCancelled | O(1) | Annule une entree et rembourse un pourcentage du cout |
| `complete_production` | `mge.rts.production.v1.fn.complete_production` | Logic (1113) | ProductionQueue, ProductionEntry | ProductionQueue | ProductionCompleted, QueueFull | O(p) | Retire l'entree terminee et emet l'event de completion |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ProductionStarted` | `mge.rts.production.v1.event.production_started` | `producer: EntityId, kind: ProductionKind, type_id: u32` | `start_production` | ui, audio |
| `ProductionCompleted` | `mge.rts.production.v1.event.production_completed` | `producer: EntityId, kind: ProductionKind, type_id: u32, rally_point: Option<(f32, f32)>` | `complete_production` | spawner, ui |
| `ProductionCancelled` | `mge.rts.production.v1.event.production_cancelled` | `producer: EntityId, type_id: u32, refund: ResourceCost` | `cancel_production` | ui, resource |
| `QueueFull` | `mge.rts.production.v1.event.queue_full` | `producer: EntityId, max_size: u8` | `complete_production` | ui |

---

## 7. Invariants

- `ProductionEntry.progress` est toujours dans [0.0, duration].
- `ProductionQueue.entries.len()` ne depasse jamais `max_size`.
- Un producteur `Idle` a une queue vide ou toutes les entrees sont terminees.
- L'annulation rembourse `cancel_refund_pct` du cout (jamais plus de 100%).
- Les ressources sont deduites a l'ajout (`start_production`), pas a la completion.
- Un producteur detruit annule automatiquement toute sa queue sans remboursement.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `default_queue_size` | `u8` | 5 | [1, 20] | Taille par defaut de la queue de production |
| `production_speed_modifier` | `f32` | 1.0 | [0.1, 5.0] | Multiplicateur global de vitesse de production |
| `cancel_refund_pct` | `f32` | 0.75 | [0.0, 1.0] | Pourcentage du cout rembourse a l'annulation |
| `auto_rally` | `bool` | true | — | Les unites produites se dirigent auto vers le rally point |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les files de production et le temps de build | Ne gere pas les ressources (→ resource) |
| Annule et rembourse partiellement | Ne spawn pas les entites produites (→ spawner externe) |
| Emet ProductionCompleted pour le spawner | Ne gere pas le placement des batiments (→ building) |
| Supporte rally point | Ne deplace pas les unites produites (→ unit-ai) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ProductionQueue, ProductionEntry, ResourceDepot |
| Ecrit | ProductionQueue, ProductionEntry, ResourceDepot |
| Emet | ProductionStarted, ProductionCompleted, ProductionCancelled, QueueFull |
| Ne touche jamais | Selection, Building, OrderQueue, FogGrid, TechNode, VisionSource |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-production/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.production.v1, trait Plugin impl
    ├── components.rs     # ProductionQueue, ProductionEntry, Producer
    ├── systems.rs        # tick_production, start_production, cancel_production, complete_production
    └── events.rs         # ProductionStarted, ProductionCompleted, ProductionCancelled, QueueFull
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (tick_production) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (ProductionState, ProductionKind)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : tick progression, start/cancel, completion, refund
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.production.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.component.production_queue","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.component.production_entry","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.component.producer","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.fn.tick_production","k":"s","d":"rts","r":["ProductionQueue","ProductionEntry"],"w":["ProductionEntry"],"e":[],"p":1110,"c":"O(p)"},
  {"i":"mge.rts.production.v1.fn.start_production","k":"s","d":"rts","r":["ProductionQueue","ResourceDepot"],"w":["ProductionQueue","ResourceDepot"],"e":["ProductionStarted","InsufficientResources"],"p":1111,"c":"O(r)"},
  {"i":"mge.rts.production.v1.fn.cancel_production","k":"s","d":"rts","r":["ProductionQueue"],"w":["ProductionQueue","ResourceDepot"],"e":["ProductionCancelled"],"p":1112,"c":"O(1)"},
  {"i":"mge.rts.production.v1.fn.complete_production","k":"s","d":"rts","r":["ProductionQueue","ProductionEntry"],"w":["ProductionQueue"],"e":["ProductionCompleted","QueueFull"],"p":1113,"c":"O(p)"},
  {"i":"mge.rts.production.v1.event.production_started","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.event.production_completed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.event.production_cancelled","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.production.v1.event.queue_full","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let barracks = world.spawn();
world.insert(barracks, Producer {
    entity: barracks,
    rally_point: Some((200.0, 150.0)),
});
world.insert(barracks, ProductionQueue {
    entries: vec![],
    max_size: 5,
    state: ProductionState::Idle,
});

let entry = ProductionEntry {
    kind: ProductionKind::Unit,
    type_id: 1,
    progress: 0.0,
    duration: 10.0,
    cost: ResourceCost { gold: 50, wood: 0, stone: 0, food: 25 },
};
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
