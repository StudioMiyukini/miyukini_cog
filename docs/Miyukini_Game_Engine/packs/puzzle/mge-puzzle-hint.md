# mge-puzzle-hint

> @id mge.puzzle.hint.v1  
> @role plugin  
> @domain puzzle  
> @do compute_display_available_move_hints  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-hint` |
| @id MSCM | `mge.puzzle.hint.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-board` |
| Hot path | Non (calcule uniquement en phase Idle) |
| Headless safe | Oui |
| Complexite globale | O(w*h) pour scan complet des coups possibles |

---

## 2. Enumerations

Aucune enumeration dediee.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `HintableMove` | `mge.puzzle.hint.v1.component.hintable_move` | `source_row: u32, source_col: u32, target_row: u32, target_col: u32, estimated_score: u32` | Coup possible detecte. Score estime pour prioriser |
| `HintState` | `mge.puzzle.hint.v1.component.hint_state` | `available_hints: Vec<EntityId>, cooldown_remaining: u32, active_hint: Option<EntityId>` | Etat du systeme de hints. Singleton |

---

## 4. Formules

```
estimated_score = match_length * base_points_per_tile * shape_bonus_estimate

Priorite des hints : trie par estimated_score decroissant.
Le meilleur coup est propose en premier.
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_hints` | `mge.puzzle.hint.v1.fn.compute_hints` | 1445 | Board, Cell, Tile | HintState, HintableMove | none | O(w*h) | Scan toutes les positions, simule chaque swap, detecte les matchs. Cree HintableMove. Limite a max_hints_computed |
| `show_hint` | `mge.puzzle.hint.v1.fn.show_hint` | 1446 | HintState | HintState | HintShown | O(1) | Si cooldown ecoulé et auto_hint_delay atteint, active le meilleur hint. Emet HintShown |
| `detect_no_moves` | `mge.puzzle.hint.v1.fn.detect_no_moves` | 1447 | HintState | Board | NoMovesAvailable | O(1) | Si available_hints est vide apres compute, emet NoMovesAvailable. Permet shuffle |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `HintShown` | `mge.puzzle.hint.v1.event.hint_shown` | `source_row: u32, source_col: u32, target_row: u32, target_col: u32` | `show_hint` | ui (highlight tuiles) |
| `NoMovesAvailable` | `mge.puzzle.hint.v1.event.no_moves_available` | `board: EntityId` | `detect_no_moves` | board (shuffle ou game over) |

---

## 7. Invariants

- `compute_hints` ne s'execute qu'en phase `Idle` ou `PlayerInput`.
- `HintState.available_hints` est recalcule a chaque changement de board.
- Un hint est invalide des qu'un swap est execute (recalcul au tick suivant).
- `HintState.cooldown_remaining` decremente de 1 par tick.
- `NoMovesAvailable` n'est emis qu'une seule fois par etat de board sans solution.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `hint_cooldown_ticks` | `u32` | 300 | [60, 1800] | Cooldown entre deux hints affiches |
| `auto_hint_delay_ticks` | `u32` | 180 | [0, 600] | Ticks d'inactivite avant affichage auto. 0 = desactive |
| `max_hints_computed` | `u32` | 5 | [1, 20] | Nombre max de coups analyses et stockes |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Calcule les coups disponibles | Ne valide pas les swaps (→ swap) |
| Detecte les impasses (no moves) | Ne detecte pas les matchs (→ match) |
| Propose le meilleur coup | Ne gere pas l'input joueur (→ core input) |
| Gere le cooldown et l'auto-hint | Ne fait pas de shuffle automatique |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Board, Cell, Tile, HintState |
| Ecrit | HintState, HintableMove, Board (NoMoves → flag) |
| Emet | HintShown, NoMovesAvailable |
| Ne touche jamais | Score, ComboChain, Goal, PuzzleTimer, SwapAction, MatchGroup |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-hint/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.hint.v1, trait Plugin impl
    ├── components.rs     # HintableMove, HintState
    ├── systems.rs        # compute_hints, show_hint, detect_no_moves
    └── events.rs         # HintShown, NoMovesAvailable
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
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : hint compute, best hint selection, no moves detection, cooldown
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.hint.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.hint.v1.component.hintable_move","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.hint.v1.component.hint_state","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.hint.v1.fn.compute_hints","k":"s","d":"puzzle","r":["Board","Cell","Tile"],"w":["HintState","HintableMove"],"e":[],"p":1445,"c":"O(w*h)"},
  {"i":"mge.puzzle.hint.v1.fn.show_hint","k":"s","d":"puzzle","r":["HintState"],"w":["HintState"],"e":["HintShown"],"p":1446,"c":"O(1)"},
  {"i":"mge.puzzle.hint.v1.fn.detect_no_moves","k":"s","d":"puzzle","r":["HintState"],"w":["Board"],"e":["NoMovesAvailable"],"p":1447,"c":"O(1)"},
  {"i":"mge.puzzle.hint.v1.event.hint_shown","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.hint.v1.event.no_moves_available","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let hint_state = world.spawn();
world.insert(hint_state, HintState {
    available_hints: vec![],
    cooldown_remaining: 0,
    active_hint: None,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-board](mge-puzzle-board.md) | Plugin board (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
