# mge-gfp-animation-state

> @id mge.foundation.animation_state.v1  
> @role plugin  
> @domain foundation  
> @do basic_animation_state_machine_transitions  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-animation-state` |
| @id MSCM | `mge.foundation.animation_state.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(a), a = entites animees |

> **NOTE** : Ce plugin est une machine a etats simple. Ce n'est PAS un Animator complexe style Unity. Transitions lineaires uniquement, pas de blend trees ni de layers.

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TransitionCondition` | `Immediate`, `AfterLoop`, `OnEvent` | Determine quand une transition entre etats d'animation est declenchee |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `AnimationStateMachine` | `mge.foundation.animation_state.v1.component.animation_state_machine` | `states: Vec<u32>, current: u32, default: u32` | Definition de la machine a etats. `states` = liste des IDs d'etats disponibles, `current` = etat actif, `default` = etat de repli |
| `CurrentState` | `mge.foundation.animation_state.v1.component.current_state` | `state_id: u32, elapsed_ticks: u32, looped: bool` | Etat courant de l'animation : ID, temps ecoule en ticks, et si une boucle complete a ete effectuee |
| `StateTransition` | `mge.foundation.animation_state.v1.component.state_transition` | `from: u32, to: u32, condition: TransitionCondition` | Definition d'une transition entre deux etats avec sa condition de declenchement |
| `AnimationDef` (statique) | `mge.foundation.animation_state.v1.component.animation_def` | `id: u32, frame_count: u32, loop_: bool, speed: f32` | Definition statique d'une animation : nombre de frames, boucle, vitesse de lecture |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_animation_state` | `mge.foundation.animation_state.v1.fn.tick_animation_state` | 170 | AnimationStateMachine, CurrentState, StateTransition, AnimationDef, StateChangeRequest | CurrentState, AnimationStateMachine | AnimationStateChanged | O(a) | Avance l'etat courant, evalue les transitions et traite les requetes de changement d'etat |

---

## 5. Flux de donnees

```
AnimationDef (statique) ──► StateTransition (regles)
                                    │
                                    ▼
CurrentState ──► AnimationStateMachine
       │                    │
       ▼                    ▼
 StateChangeRequest (events entrants)
       │
       ▼
 ┌────────────────────────────────────┐
 │       tick_animation_state         │  Phase 170
 │  (avance etat, evalue transitions) │
 └─────┬──────────────────────┬───────┘
       │                      │
       ▼                      ▼
 CurrentState (maj)    AnimationStateChanged
 AnimationStateMachine    (event)
   (maj current)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AnimationStateChanged` | `mge.foundation.animation_state.v1.event.animation_state_changed` | `entity: EntityId, old_state: u32, new_state: u32` | `tick_animation_state` | sprite-flip, frame-timer, systemes audio |
| `StateChangeRequest` | `mge.foundation.animation_state.v1.event.state_change_request` | `entity: EntityId, target_state: u32` | Systemes gameplay externes | `tick_animation_state` |

---

## 7. Invariants

- `AnimationStateMachine.current` est toujours un ID present dans `states`.
- `CurrentState.elapsed_ticks` est remis a 0 lors d'un changement d'etat.
- `CurrentState.looped` passe a `true` quand `elapsed_ticks` atteint `AnimationDef.frame_count` et `loop_ = true`.
- Une transition `AfterLoop` ne se declenche que si `CurrentState.looped = true`.
- Une transition `Immediate` se declenche au tick suivant son evaluation.
- `StateChangeRequest` est prioritaire sur les transitions automatiques.
- Si l'etat cible d'une requete n'existe pas dans `states`, la requete est ignoree.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere une machine a etats d'animation simple | Ne gere pas les blend trees ou layers (hors scope) |
| Evalue les transitions entre etats | Ne gere pas le rendu des sprites (→ renderer) |
| Traite les requetes de changement d'etat | Ne gere pas le flip horizontal/vertical (→ sprite-flip) |
| Emet un evenement lors des changements | Ne gere pas le timing des frames (→ frame-timer) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | AnimationStateMachine, CurrentState, StateTransition, AnimationDef, StateChangeRequest |
| Ecrit | CurrentState, AnimationStateMachine |
| Emet | AnimationStateChanged |
| Ne touche jamais | SpriteFlip, FrameTimer, FrameIndex, Transform2D, Velocity2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-animation-state/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.animation_state.v1, trait Plugin impl
    ├── components.rs     # AnimationStateMachine, CurrentState, StateTransition, AnimationDef, TransitionCondition
    ├── systems.rs        # tick_animation_state
    └── events.rs         # AnimationStateChanged, StateChangeRequest
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
- [ ] 1 enum `TransitionCondition` dans `components.rs`
- [ ] 4 composants dans `components.rs` avec @id et @fields (dont 1 statique)
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : transition Immediate, AfterLoop, OnEvent, StateChangeRequest, etat invalide ignore
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.animation_state.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.component.animation_state_machine","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.component.current_state","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.component.state_transition","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.component.animation_def","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.event.animation_state_changed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.event.state_change_request","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.animation_state.v1.fn.tick_animation_state","k":"s","d":"foundation","r":["AnimationStateMachine","CurrentState","StateTransition","AnimationDef","StateChangeRequest"],"w":["CurrentState","AnimationStateMachine"],"e":["AnimationStateChanged"],"p":170,"c":"O(a)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let anim_idle = world.spawn();
world.insert(anim_idle, AnimationDef { id: 1, frame_count: 4, loop_: true, speed: 1.0 });

let anim_run = world.spawn();
world.insert(anim_run, AnimationDef { id: 2, frame_count: 6, loop_: true, speed: 1.5 });

let entity = world.spawn();
world.insert(entity, AnimationStateMachine { states: vec![1, 2], current: 1, default: 1 });
world.insert(entity, CurrentState { state_id: 1, elapsed_ticks: 0, looped: false });
world.insert(entity, StateTransition {
    from: 1, to: 2, condition: TransitionCondition::OnEvent,
});
// Envoi d'un StateChangeRequest { entity, target_state: 2 }
// → AnimationStateChanged { entity, old_state: 1, new_state: 2 }
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-sprite-flip](mge-gfp-sprite-flip.md) | Plugin flip sprite (consomme AnimationStateChanged) |
| [mge-gfp-frame-timer](mge-gfp-frame-timer.md) | Plugin timer de frames (depend de l'etat d'animation) |
