# mge-gfp-activation-event

> @id mge.foundation.activation_event.v1  
> @role plugin  
> @domain foundation  
> @do contextual_activation_trigger_cooldown  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-activation-event` |
| @id MSCM | `mge.foundation.activation_event.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-interaction-system` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(e), e = evenements InteractionCompleted |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ActivationTrigger` | `mge.foundation.activation_event.v1.component.activation_trigger` | `enabled: bool, single_use: bool` | Configure le declenchement d'activation : active ou non, usage unique ou reutilisable |
| `ActivationCooldown` | `mge.foundation.activation_event.v1.component.activation_cooldown` | `remaining_ticks: u32, max_ticks: u32` | Cooldown d'activation : ticks restants avant reactivation, duree maximale du cooldown |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_activations` | `mge.foundation.activation_event.v1.fn.process_activations` | 153 | ActivationTrigger, ActivationCooldown, InteractionCompleted (event) | ActivationTrigger, ActivationCooldown | ActivationTriggered | O(e) | Sur chaque InteractionCompleted, verifie les conditions (enabled, cooldown), emet ActivationTriggered si valide, tick les cooldowns |

---

## 5. Flux de donnees

```
InteractionCompleted (event entree)
       │
       ├──── ActivationTrigger (enabled, single_use)
       ├──── ActivationCooldown (remaining_ticks, max_ticks)
       │
       ▼
 ┌──────────────────────────────┐
 │ process_activations           │  Phase 153
 │ (verifie enabled + cooldown, │
 │  emet ActivationTriggered,   │
 │  tick cooldowns, desactive   │
 │  si single_use)              │
 └──────────┬───────────────────┘
            │
            ├──→ ActivationTriggered (si conditions remplies)
            ├──→ ActivationTrigger (enabled = false si single_use)
            └──→ ActivationCooldown (remaining_ticks mis a jour)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ActivationTriggered` | `mge.foundation.activation_event.v1.event.activation_triggered` | `source: EntityId, target: EntityId` | `process_activations` | Logique de jeu, triggers de zone, scripts, systemes genre-specifiques |

---

## 7. Invariants

- `ActivationTriggered` n'est emis que si `ActivationTrigger.enabled == true` et `ActivationCooldown.remaining_ticks == 0`.
- Si `single_use == true`, le trigger est desactive (`enabled = false`) apres la premiere activation reussie.
- `ActivationCooldown.remaining_ticks` est decremente chaque tick par le systeme, jamais inferieur a 0.
- Apres une activation reussie, `remaining_ticks` est remis a `max_ticks`.
- Ce plugin ne contient aucune logique de jeu — il fournit un signal d'activation generique.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Ecoute les InteractionCompleted et emet ActivationTriggered | Ne resout pas les interactions (→ interaction-system) |
| Gere le cooldown d'activation (tick, reset) | Ne marque pas les entites comme interactables (→ interactable) |
| Supporte le mode usage unique (single_use) | Ne detecte pas la proximite (→ proximity-check) |
| Fournit un evenement d'activation generique pour la logique de jeu | Ne contient aucune logique specifique a un genre |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ActivationTrigger, ActivationCooldown, InteractionCompleted (event) |
| Ecrit | ActivationTrigger, ActivationCooldown |
| Emet | ActivationTriggered |
| Ne touche jamais | Transform2D, Velocity2D, Camera2D, Interactable, ProximityRadius, NearbyEntities |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-activation-event/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.activation_event.v1, trait Plugin impl
    ├── components.rs     # ActivationTrigger, ActivationCooldown
    ├── systems.rs        # process_activations
    └── events.rs         # ActivationTriggered
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : activation basique, cooldown actif bloque, single_use desactive, cooldown tick, enabled = false bloque
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.activation_event.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.activation_event.v1.component.activation_trigger","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.activation_event.v1.component.activation_cooldown","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.activation_event.v1.fn.process_activations","k":"s","d":"foundation","r":["ActivationTrigger","ActivationCooldown"],"w":["ActivationTrigger","ActivationCooldown"],"e":["ActivationTriggered"],"p":153,"c":"O(e)"},
  {"i":"mge.foundation.activation_event.v1.event.activation_triggered","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let door = world.spawn();
world.insert(door, Transform2D {
    x: 500.0, y: 200.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(door, Interactable {
    interact_type: InteractableType::Action,
    priority: 0,
    range: 48.0,
});
world.insert(door, InteractableState {
    state: InteractState::Idle,
});
world.insert(door, ActivationTrigger {
    enabled: true, single_use: false,
});
world.insert(door, ActivationCooldown {
    remaining_ticks: 0, max_ticks: 60,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-interaction-system](mge-gfp-interaction-system.md) | Resolution interactions (prerequis) |
| [mge-gfp-interactable](mge-gfp-interactable.md) | Marqueur interactable (utilise indirectement) |
