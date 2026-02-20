# mge-gfp-timer

> @id mge.foundation.timer.v1  
> @role plugin  
> @domain foundation  
> @do generic_timers_oneshot_repeating  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-timer` |
| @id MSCM | `mge.foundation.timer.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(t), t = timers actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TimerMode` | `OneShot`, `Repeating` | Mode du timer : execution unique ou repetee |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Timer` | `mge.foundation.timer.v1.component.timer` | `remaining_ticks: u32, duration_ticks: u32, mode: TimerMode, active: bool` | Timer generique. `remaining_ticks` decremente chaque tick, `duration_ticks` = duree totale, `mode` = OneShot ou Repeating, `active` = en cours |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_timers` | `mge.foundation.timer.v1.fn.tick_timers` | 180 | Timer | Timer | TimerFinished, TimerTick | O(t) | Decremente `remaining_ticks` des timers actifs, emet `TimerFinished` quand remaining atteint 0, reinitialise si Repeating, desactive si OneShot |

---

## 5. Flux de donnees

```
Timer (remaining_ticks, mode, active)
       │
       ▼
 ┌──────────────────────────────┐
 │         tick_timers           │  Phase 180
 │  (remaining-- → check zero)  │
 └─────┬──────────┬─────────────┘
       │          │
       ▼          ▼
 Timer (maj)   TimerFinished (event)
               TimerTick (event)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TimerFinished` | `mge.foundation.timer.v1.event.timer_finished` | `entity: EntityId` | `tick_timers` | Systemes gameplay, cooldown, spawner |
| `TimerTick` | `mge.foundation.timer.v1.event.timer_tick` | `entity: EntityId, remaining: u32` | `tick_timers` | UI, barres de progression |

---

## 7. Invariants

- Seuls les timers avec `active = true` sont decrements.
- `TimerFinished` est emis exactement une fois quand `remaining_ticks` atteint 0.
- En mode `Repeating`, `remaining_ticks` est reinitialise a `duration_ticks` apres emission de `TimerFinished`.
- En mode `OneShot`, `active` passe a `false` apres emission de `TimerFinished`.
- `duration_ticks` doit etre >= 1. Une valeur de 0 est invalide.
- `TimerTick` est emis a chaque tick pour chaque timer actif (permet le suivi de progression).

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Decremente des timers generiques en ticks | Ne gere pas les cooldowns (→ cooldown) |
| Supporte les modes OneShot et Repeating | Ne gere pas les durees de vie d'entites (→ lifetime) |
| Emet des evenements a l'expiration et a chaque tick | Ne gere pas le despawn (→ despawn) |
| Permet le suivi de progression via TimerTick | Ne convertit pas les ticks en secondes (responsabilite appelant) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Timer |
| Ecrit | Timer |
| Emet | TimerFinished, TimerTick |
| Ne touche jamais | Cooldown, Lifetime, DespawnMarker, Transform2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-timer/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.timer.v1, trait Plugin impl
    ├── components.rs     # Timer, TimerMode
    ├── systems.rs        # tick_timers
    └── events.rs         # TimerFinished, TimerTick
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
- [ ] 1 enum `TimerMode` dans `components.rs`
- [ ] 1 composant dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : OneShot expiration, Repeating reset, timer inactif ignore, TimerTick emis chaque tick
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.timer.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.timer.v1.component.timer","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.timer.v1.event.timer_finished","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.timer.v1.event.timer_tick","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.timer.v1.fn.tick_timers","k":"s","d":"foundation","r":["Timer"],"w":["Timer"],"e":["TimerFinished","TimerTick"],"p":180,"c":"O(t)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Timer {
    remaining_ticks: 60, duration_ticks: 60, mode: TimerMode::Repeating, active: true,
});
// Tick 1 : remaining_ticks = 59, TimerTick { entity, remaining: 59 }
// Tick 60 : remaining_ticks = 0, TimerFinished { entity }, remaining reset a 60
// Tick 120 : TimerFinished emis a nouveau (Repeating)
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-cooldown](mge-gfp-cooldown.md) | Plugin cooldown (pattern similaire, specialise) |
| [mge-gfp-lifetime](mge-gfp-lifetime.md) | Plugin duree de vie (utilise un pattern similaire) |
