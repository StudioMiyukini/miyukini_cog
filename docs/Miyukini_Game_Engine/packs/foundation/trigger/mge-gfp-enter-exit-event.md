# mge-gfp-enter-exit-event

> @id mge.foundation.enter_exit_event.v1  
> @role plugin  
> @domain foundation  
> @do emit_enter_exit_events_for_trigger_zones  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-enter-exit-event` |
| @id MSCM | `mge.foundation.enter_exit_event.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-trigger-zone` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(t*o), t = zones trigger, o = occupants par zone |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TriggerTracker` | `mge.foundation.enter_exit_event.v1.component.trigger_tracker` | `previous_occupants: Vec<EntityId>` | Stocke la liste des occupants du tick precedent pour permettre le calcul du diff |
| `EntityPresence` | `mge.foundation.enter_exit_event.v1.component.entity_presence` | `entered_zones: Vec<EntityId>` | Liste des zones dans lesquelles l'entite est actuellement presente |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `emit_enter_exit_events` | `mge.foundation.enter_exit_event.v1.fn.emit_enter_exit_events` | 161 | TriggerOccupants, TriggerTracker | TriggerTracker, EntityPresence | TriggerEnter, TriggerExit | O(t*o) | Compare les occupants actuels aux precedents, emet TriggerEnter pour les nouvelles presences et TriggerExit pour les departs |

---

## 5. Flux de donnees

```
TriggerOccupants (tick courant)
       │
       ▼
TriggerTracker (tick precedent)
       │
       ▼
 ┌───────────────────────────────────┐
 │     emit_enter_exit_events        │  Phase 161
 │  (diff courant vs precedent)      │
 └────┬──────────┬──────────┬────────┘
      │          │          │
      ▼          ▼          ▼
 TriggerEnter  TriggerExit  TriggerTracker (maj)
  (event)       (event)     EntityPresence (maj)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TriggerEnter` | `mge.foundation.enter_exit_event.v1.event.trigger_enter` | `zone: EntityId, entity: EntityId` | `emit_enter_exit_events` | area-condition, scripts gameplay |
| `TriggerExit` | `mge.foundation.enter_exit_event.v1.event.trigger_exit` | `zone: EntityId, entity: EntityId` | `emit_enter_exit_events` | area-condition, scripts gameplay |

---

## 7. Invariants

- `TriggerTracker.previous_occupants` est mis a jour apres chaque diff pour refleter l'etat courant.
- Un evenement `TriggerEnter` est emis exactement une fois par entite et par zone lors de l'entree.
- Un evenement `TriggerExit` est emis exactement une fois par entite et par zone lors de la sortie.
- Si une entite entre et sort dans le meme tick (impossible en pratique), seul l'etat final compte.
- `EntityPresence.entered_zones` est coherent avec l'ensemble des `TriggerOccupants` apres Phase 161.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Detecte les entrees et sorties de zones trigger | Ne definit pas les zones trigger (→ trigger-zone) |
| Emet des evenements TriggerEnter et TriggerExit | Ne gere pas les conditions d'activation (→ area-condition) |
| Maintient un historique des occupants pour le diff | Ne gere pas la detection de collision (→ collision-detection) |
| Tient a jour la liste des zones dans lesquelles chaque entite se trouve | Ne declenche pas d'actions gameplay |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | TriggerOccupants, TriggerTracker |
| Ecrit | TriggerTracker, EntityPresence |
| Emet | TriggerEnter, TriggerExit |
| Ne touche jamais | TriggerZone, Collider, WorldTransform, ConditionState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-enter-exit-event/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.enter_exit_event.v1, trait Plugin impl
    ├── components.rs     # TriggerTracker, EntityPresence
    ├── systems.rs        # emit_enter_exit_events
    └── events.rs         # TriggerEnter, TriggerExit
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : entree simple, sortie simple, entree multiple, aucun changement (pas d'event)
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.enter_exit_event.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.enter_exit_event.v1.component.trigger_tracker","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.enter_exit_event.v1.component.entity_presence","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.enter_exit_event.v1.event.trigger_enter","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.enter_exit_event.v1.event.trigger_exit","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.enter_exit_event.v1.fn.emit_enter_exit_events","k":"s","d":"foundation","r":["TriggerOccupants","TriggerTracker"],"w":["TriggerTracker","EntityPresence"],"e":["TriggerEnter","TriggerExit"],"p":161,"c":"O(t*o)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let zone = world.spawn();
world.insert(zone, TriggerZone {
    shape: TriggerShape::Circle, width: 0.0, height: 0.0, radius: 32.0, enabled: true,
});
world.insert(zone, TriggerOccupants { entities: vec![] });
world.insert(zone, TriggerTracker { previous_occupants: vec![] });

let player = world.spawn();
world.insert(player, EntityPresence { entered_zones: vec![] });
// Quand le joueur entre dans la zone :
// → TriggerEnter { zone, entity: player } est emis
// → EntityPresence.entered_zones contient zone
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-trigger-zone](mge-gfp-trigger-zone.md) | Plugin zones trigger (fournit TriggerOccupants) |
| [mge-gfp-area-condition](mge-gfp-area-condition.md) | Conditions basees sur les evenements enter/exit |
