# mge-gfp-frame-timer

> @id mge.foundation.frame_timer.v1  
> @role plugin  
> @domain foundation  
> @do tick_animation_frame_index_timer  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-frame-timer` |
| @id MSCM | `mge.foundation.frame_timer.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-animation-state` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(a), a = entites animees |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `FrameTimer` | `mge.foundation.frame_timer.v1.component.frame_timer` | `ticks_per_frame: u32, elapsed: u32` | Compteur de ticks entre chaque changement de frame. `ticks_per_frame` = duree d'une frame, `elapsed` = ticks ecoules depuis le dernier changement |
| `FrameIndex` | `mge.foundation.frame_timer.v1.component.frame_index` | `current: u32, total: u32` | Index de la frame courante et nombre total de frames de l'animation active |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_frame_timer` | `mge.foundation.frame_timer.v1.fn.tick_frame_timer` | 172 | FrameTimer, CurrentState | FrameTimer, FrameIndex | none | O(a) | Incremente `elapsed`, avance `FrameIndex.current` quand `elapsed >= ticks_per_frame`, boucle si l'animation est en mode loop |

---

## 5. Flux de donnees

```
CurrentState (looped, state_id)
       │
       ▼
FrameTimer (ticks_per_frame, elapsed)
       │
       ▼
 ┌────────────────────────────────┐
 │       tick_frame_timer          │  Phase 172
 │  (elapsed++ → avance frame)    │
 └─────────┬──────────────────────┘
           │
           ▼
    FrameTimer (elapsed remis a 0)
    FrameIndex (current avance)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `FrameIndex.current` est toujours dans l'intervalle `[0, FrameIndex.total)`.
- Quand `elapsed >= ticks_per_frame`, `FrameIndex.current` avance de 1 et `elapsed` est remis a 0.
- Si l'animation boucle (`CurrentState.looped`-compatible) et `current` atteint `total`, `current` revient a 0.
- Si l'animation ne boucle pas, `current` reste bloque a `total - 1`.
- `FrameTimer.ticks_per_frame` doit etre >= 1. Une valeur de 0 causerait une avance a chaque tick.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Avance l'index de frame selon un timer en ticks | Ne gere pas la machine a etats (→ animation-state) |
| Gere le wrapping pour les animations en boucle | Ne gere pas le rendu des sprites (→ renderer) |
| Fournit l'index de frame lisible par le renderer | Ne gere pas le flip du sprite (→ sprite-flip) |
| Synchronise le timing avec l'etat d'animation | Ne gere pas la vitesse variable (utilise ticks_per_frame fixe) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | FrameTimer, CurrentState |
| Ecrit | FrameTimer, FrameIndex |
| Emet | rien |
| Ne touche jamais | AnimationStateMachine, StateTransition, SpriteFlip, Transform2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-frame-timer/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.frame_timer.v1, trait Plugin impl
    ├── components.rs     # FrameTimer, FrameIndex
    ├── systems.rs        # tick_frame_timer
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : avance normale, wrapping boucle, arret fin d'animation, ticks_per_frame variable
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.frame_timer.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.frame_timer.v1.component.frame_timer","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.frame_timer.v1.component.frame_index","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.frame_timer.v1.fn.tick_frame_timer","k":"s","d":"foundation","r":["FrameTimer","CurrentState"],"w":["FrameTimer","FrameIndex"],"e":[],"p":172,"c":"O(a)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, FrameTimer { ticks_per_frame: 6, elapsed: 0 });
world.insert(entity, FrameIndex { current: 0, total: 4 });
world.insert(entity, CurrentState { state_id: 1, elapsed_ticks: 0, looped: false });
// Tick 1-5 : elapsed incremente de 1 a chaque tick
// Tick 6 : elapsed >= ticks_per_frame → FrameIndex.current = 1, elapsed = 0
// Tick 12 : FrameIndex.current = 2
// Tick 24 (boucle) : FrameIndex.current revient a 0
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-animation-state](mge-gfp-animation-state.md) | Plugin machine a etats (fournit CurrentState) |
| [mge-gfp-sprite-flip](mge-gfp-sprite-flip.md) | Plugin flip sprite |
