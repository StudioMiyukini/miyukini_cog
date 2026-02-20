# mge-puzzle-timer

> @id mge.puzzle.timer.v1  
> @role plugin  
> @domain puzzle  
> @do manage_puzzle_time_limits_and_bonuses  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-timer` |
| @id MSCM | `mge.puzzle.timer.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (O(1) par tick) |
| Headless safe | Oui |
| Complexite globale | O(1) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TimerMode` | `CountDown, CountUp, Unlimited` | Mode du timer. CountDown = temps limite, CountUp = chrono, Unlimited = pas de temps |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `PuzzleTimer` | `mge.puzzle.timer.v1.component.puzzle_timer` | `remaining_ticks: u32, max_ticks: u32, mode: TimerMode, paused: bool` | Timer principal. remaining_ticks decremente en CountDown |
| `TimerBonus` | `mge.puzzle.timer.v1.component.timer_bonus` | `bonus_ticks: u32` | Bonus de temps a appliquer. Consomme dans le tick |

---

## 4. Formules

```
CountDown : remaining_ticks -= 1 par tick (si !paused)
CountUp   : remaining_ticks += 1 par tick (si !paused)
Unlimited : remaining_ticks non modifie

Warning emis quand : remaining_ticks <= max_ticks * warning_threshold_pct
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_timer` | `mge.puzzle.timer.v1.fn.tick_timer` | 1440 | PuzzleTimer | PuzzleTimer | TimerWarning | O(1) | Decremente/incremente remaining_ticks selon mode. Emet TimerWarning si seuil atteint |
| `apply_timer_bonus` | `mge.puzzle.timer.v1.fn.apply_timer_bonus` | 1441 | PuzzleTimer, TimerBonus | PuzzleTimer | TimerBonusApplied | O(b) | Ajoute bonus_ticks a remaining_ticks. Clampe a max_ticks. Consomme TimerBonus |
| `check_timer_expiry` | `mge.puzzle.timer.v1.fn.check_timer_expiry` | 1442 | PuzzleTimer | PuzzleTimer | TimerExpired | O(1) | Si remaining_ticks == 0 en mode CountDown → emet TimerExpired |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TimerExpired` | `mge.puzzle.timer.v1.event.timer_expired` | `board: EntityId` | `check_timer_expiry` | goal (LevelFailed si SurviveTime), board (→ GameOver) |
| `TimerWarning` | `mge.puzzle.timer.v1.event.timer_warning` | `remaining_ticks: u32, threshold: u32` | `tick_timer` | ui (animation urgence) |
| `TimerBonusApplied` | `mge.puzzle.timer.v1.event.timer_bonus_applied` | `bonus: u32, new_remaining: u32` | `apply_timer_bonus` | ui (feedback bonus) |

---

## 7. Invariants

- `PuzzleTimer.remaining_ticks` ne descend jamais en dessous de 0 en CountDown.
- `PuzzleTimer.remaining_ticks` ne depasse jamais `max_ticks` apres bonus.
- En mode `Unlimited`, aucun evenement timer n'est emis.
- Un TimerBonus est consomme dans le tick ou il est cree.
- `TimerWarning` n'est emis qu'une seule fois par seuil franchi.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_time_limit_ticks` | `u32` | 3600 | [600, 36000] | Duree par defaut (60s a 60fps) |
| `warning_threshold_pct` | `f32` | 0.2 | [0.0, 1.0] | Pourcentage restant pour le warning |
| `default_timer_mode` | `TimerMode` | CountDown | {CountDown, CountUp, Unlimited} | Mode par defaut |
| `pause_during_cascade` | `bool` | true | {true, false} | Pause le timer pendant les cascades |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Decompte/chronometre le temps | Ne determine pas victoire/defaite (→ goal) |
| Emet les alertes de temps | Ne gere pas la grille (→ board) |
| Applique les bonus de temps | Ne gere pas le score (→ match) |
| Supporte pause pendant cascades | Ne gere pas l'input joueur |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | PuzzleTimer, TimerBonus |
| Ecrit | PuzzleTimer |
| Emet | TimerExpired, TimerWarning, TimerBonusApplied |
| Ne touche jamais | Tile, Board, Cell, Score, ComboChain, Goal, SwapAction |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-timer/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.timer.v1, trait Plugin impl
    ├── components.rs     # PuzzleTimer, TimerBonus
    ├── systems.rs        # tick_timer, apply_timer_bonus, check_timer_expiry
    └── events.rs         # TimerExpired, TimerWarning, TimerBonusApplied
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (TimerMode)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : countdown, countup, expiry, warning, bonus, pause
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.timer.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.timer.v1.component.puzzle_timer","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.timer.v1.component.timer_bonus","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.timer.v1.fn.tick_timer","k":"s","d":"puzzle","r":["PuzzleTimer"],"w":["PuzzleTimer"],"e":["TimerWarning"],"p":1440,"c":"O(1)"},
  {"i":"mge.puzzle.timer.v1.fn.apply_timer_bonus","k":"s","d":"puzzle","r":["PuzzleTimer","TimerBonus"],"w":["PuzzleTimer"],"e":["TimerBonusApplied"],"p":1441,"c":"O(b)"},
  {"i":"mge.puzzle.timer.v1.fn.check_timer_expiry","k":"s","d":"puzzle","r":["PuzzleTimer"],"w":["PuzzleTimer"],"e":["TimerExpired"],"p":1442,"c":"O(1)"},
  {"i":"mge.puzzle.timer.v1.event.timer_expired","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.timer.v1.event.timer_warning","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.timer.v1.event.timer_bonus_applied","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let timer = world.spawn();
world.insert(timer, PuzzleTimer {
    remaining_ticks: 3600,
    max_ticks: 3600,
    mode: TimerMode::CountDown,
    paused: false,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
