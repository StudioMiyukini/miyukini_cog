# mge-gfp-interaction-system

> @id mge.foundation.interaction_system.v1  
> @role plugin  
> @domain foundation  
> @do resolve_interaction_requests_priority  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-interaction-system` |
| @id MSCM | `mge.foundation.interaction_system.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-interactable`, `mge-gfp-proximity-check` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(r), r = requetes d'interaction |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `InteractionFailReason` | `OutOfRange`, `Disabled`, `AlreadyInUse`, `InvalidTarget` | Raison de l'echec d'une tentative d'interaction |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `InteractionRequest` | `mge.foundation.interaction_system.v1.component.interaction_request` | `source: EntityId, target: EntityId` | Demande d'interaction emise par une entite source vers une cible |
| `ActiveInteraction` | `mge.foundation.interaction_system.v1.component.active_interaction` | `source: EntityId, target: EntityId, started_tick: u32` | Interaction en cours, avec le tick de debut pour le suivi de duree |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_interaction_requests` | `mge.foundation.interaction_system.v1.fn.process_interaction_requests` | 151 | InteractionRequest, Interactable, InteractableState, NearbyEntities | ActiveInteraction, InteractableState | InteractionStarted, InteractionCompleted, InteractionFailed | O(r) | Valide la portee, verifie l'etat, resout par priorite et emet les evenements |

---

## 5. Flux de donnees

```
InteractionRequest (source, target)
       │
       ├──── Interactable (type, priorite, portee)
       ├──── InteractableState (etat courant)
       ├──── NearbyEntities (verification portee)
       │
       ▼
 ┌──────────────────────────────────┐
 │ process_interaction_requests      │  Phase 151
 │ (validation portee, etat,        │
 │  resolution par priorite)        │
 └──────────┬───────────────────────┘
            │
            ├──→ ActiveInteraction (si succes)
            ├──→ InteractableState (InProgress)
            ├──→ InteractionStarted (si succes)
            ├──→ InteractionCompleted (si action ponctuelle)
            └──→ InteractionFailed (si echec, avec raison)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `InteractionStarted` | `mge.foundation.interaction_system.v1.event.interaction_started` | `source: EntityId, target: EntityId, interact_type: InteractableType` | `process_interaction_requests` | Systemes animation, audio, UI, activation-event |
| `InteractionCompleted` | `mge.foundation.interaction_system.v1.event.interaction_completed` | `source: EntityId, target: EntityId` | `process_interaction_requests` | activation-event, logique de jeu |
| `InteractionFailed` | `mge.foundation.interaction_system.v1.event.interaction_failed` | `source: EntityId, target: EntityId, reason: InteractionFailReason` | `process_interaction_requests` | UI feedback, audio, debug |

---

## 7. Invariants

- Une `InteractionRequest` est consommee dans le tick ou elle est emise (pas de persistence).
- Un interactable `Disabled` ou `InProgress` (deja occupe) ne peut pas accepter de nouvelle interaction.
- La validation de portee utilise `NearbyEntities` (fourni par proximity-check, Phase 152 du tick precedent).
- Quand plusieurs requetes ciblent le meme interactable, la priorite de l'interactable et l'ordre d'arrivee determinent le gagnant.
- `ActiveInteraction` est cree uniquement en cas de succes.
- `InteractionFailed` indique toujours la raison precise de l'echec.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Resout les demandes d'interaction par priorite | Ne marque pas les entites comme interactables (→ interactable) |
| Valide la portee et l'etat avant de demarrer | Ne detecte pas la proximite (→ proximity-check) |
| Emet les evenements de succes, completion et echec | Ne gere pas l'activation post-interaction (→ activation-event) |
| Gere les interactions actives (en cours) | Ne contient aucune logique de jeu specifique |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | InteractionRequest, Interactable, InteractableState, NearbyEntities |
| Ecrit | ActiveInteraction, InteractableState |
| Emet | InteractionStarted, InteractionCompleted, InteractionFailed |
| Ne touche jamais | Transform2D, Velocity2D, Camera2D, ProximityRadius, ActivationTrigger |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-interaction-system/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.interaction_system.v1, trait Plugin impl
    ├── components.rs     # InteractionRequest, ActiveInteraction, InteractionFailReason
    ├── systems.rs        # process_interaction_requests
    └── events.rs         # InteractionStarted, InteractionCompleted, InteractionFailed
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 1 enumeration dans `components.rs` (InteractionFailReason)
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : succes basique, echec OutOfRange, echec Disabled, echec AlreadyInUse, echec InvalidTarget, resolution priorite
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.interaction_system.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interaction_system.v1.component.interaction_request","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interaction_system.v1.component.active_interaction","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interaction_system.v1.fn.process_interaction_requests","k":"s","d":"foundation","r":["InteractionRequest","Interactable","InteractableState","NearbyEntities"],"w":["ActiveInteraction","InteractableState"],"e":["InteractionStarted","InteractionCompleted","InteractionFailed"],"p":151,"c":"O(r)"},
  {"i":"mge.foundation.interaction_system.v1.event.interaction_started","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interaction_system.v1.event.interaction_completed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interaction_system.v1.event.interaction_failed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, Transform2D {
    x: 100.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});

let lever = world.spawn();
world.insert(lever, Transform2D {
    x: 120.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(lever, Interactable {
    interact_type: InteractableType::Toggle,
    priority: 0,
    range: 48.0,
});
world.insert(lever, InteractableState {
    state: InteractState::Available,
});

world.insert(player, InteractionRequest {
    source: player, target: lever,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-interactable](mge-gfp-interactable.md) | Marqueur interactable (prerequis) |
| [mge-gfp-proximity-check](mge-gfp-proximity-check.md) | Detection proximite (prerequis) |
| [mge-gfp-activation-event](mge-gfp-activation-event.md) | Activation post-interaction (depend de interaction-system) |
