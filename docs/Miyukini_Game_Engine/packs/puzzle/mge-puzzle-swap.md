# mge-puzzle-swap

> @id mge.puzzle.swap.v1  
> @role plugin  
> @domain puzzle  
> @do validate_execute_tile_swap_moves  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-swap` |
| @id MSCM | `mge.puzzle.swap.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-board` |
| Hot path | Non (1 action par input joueur) |
| Headless safe | Oui |
| Complexite globale | O(1) par swap |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `SwapDirection` | `Up, Down, Left, Right` | Direction de l'echange |
| `SwapResult` | `Valid, Invalid, Reverted` | Resultat apres validation |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `SwapAction` | `mge.puzzle.swap.v1.component.swap_action` | `source_row: u32, source_col: u32, direction: SwapDirection` | Requete d'echange posee par l'input. Consommee dans le tick |
| `SwapState` | `mge.puzzle.swap.v1.component.swap_state` | `result: Option<SwapResult>, animating: bool` | Etat du swap en cours. Permet le revert si invalide |

---

## 4. Formules

Aucune formule. Le swap est une operation discrete de permutation.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `validate_swap` | `mge.puzzle.swap.v1.fn.validate_swap` | 1410 | SwapAction, Board, Cell, Tile | SwapState | SwapValidated | O(1) | Verifie que le swap cible une case valide, non Blocked/Hole, et (si require_match) qu'un match en resulte |
| `execute_swap` | `mge.puzzle.swap.v1.fn.execute_swap` | 1411 | SwapAction, SwapState, Cell, Tile | Cell, Tile | SwapExecuted | O(1) | Echange les positions (row, col) des deux Tiles et les occupants des Cell |
| `revert_invalid_swap` | `mge.puzzle.swap.v1.fn.revert_invalid_swap` | 1412 | SwapState, Cell, Tile | Cell, Tile | SwapReverted | O(1) | Si SwapResult == Invalid, remet les Tiles a leur position d'origine |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `SwapValidated` | `mge.puzzle.swap.v1.event.swap_validated` | `source_row: u32, source_col: u32, target_row: u32, target_col: u32, valid: bool` | `validate_swap` | ui (animation), board |
| `SwapExecuted` | `mge.puzzle.swap.v1.event.swap_executed` | `source_row: u32, source_col: u32, target_row: u32, target_col: u32` | `execute_swap` | board (→ Resolving), match, ui |
| `SwapReverted` | `mge.puzzle.swap.v1.event.swap_reverted` | `source_row: u32, source_col: u32, target_row: u32, target_col: u32` | `revert_invalid_swap` | ui (animation retour) |

---

## 7. Invariants

- Un SwapAction est consomme dans le tick ou il est pose. Jamais reporte.
- Un swap ne peut s'executer que si `Board.phase == PlayerInput`.
- Un swap vers une Cell Blocked ou Hole est toujours invalide.
- Apres `revert_invalid_swap`, l'etat du board est identique a avant le swap.
- Un seul SwapAction actif a la fois.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `allow_diagonal_swap` | `bool` | false | {true, false} | Autorise les swaps en diagonale |
| `require_match_to_validate` | `bool` | true | {true, false} | Un swap doit produire un match pour etre valide |
| `swap_animation_ticks` | `u32` | 6 | [1, 30] | Duree animation swap (ticks) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Valide les echanges de tuiles | Ne detecte pas les matchs (→ match) |
| Execute la permutation des positions | Ne gere pas la grille (→ board) |
| Reverte les swaps invalides | Ne gere pas l'input brut (→ core input) |
| Respecte les phases du board | Ne fait pas tomber les blocs (→ block) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | SwapAction, Board, Cell, Tile, SwapState |
| Ecrit | Cell, Tile, SwapState |
| Emet | SwapValidated, SwapExecuted, SwapReverted |
| Ne touche jamais | Score, ComboChain, Goal, PuzzleTimer, MatchGroup, BlockGravity |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-swap/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.swap.v1, trait Plugin impl
    ├── components.rs     # SwapAction, SwapState
    ├── systems.rs        # validate_swap, execute_swap, revert_invalid_swap
    └── events.rs         # SwapValidated, SwapExecuted, SwapReverted
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
- [ ] 2 enumerations (SwapDirection, SwapResult)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : swap valid, swap invalid, swap revert, diagonal swap
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.swap.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.swap.v1.component.swap_action","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.swap.v1.component.swap_state","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.swap.v1.fn.validate_swap","k":"s","d":"puzzle","r":["SwapAction","Board","Cell","Tile"],"w":["SwapState"],"e":["SwapValidated"],"p":1410,"c":"O(1)"},
  {"i":"mge.puzzle.swap.v1.fn.execute_swap","k":"s","d":"puzzle","r":["SwapAction","SwapState","Cell","Tile"],"w":["Cell","Tile"],"e":["SwapExecuted"],"p":1411,"c":"O(1)"},
  {"i":"mge.puzzle.swap.v1.fn.revert_invalid_swap","k":"s","d":"puzzle","r":["SwapState","Cell","Tile"],"w":["Cell","Tile"],"e":["SwapReverted"],"p":1412,"c":"O(1)"},
  {"i":"mge.puzzle.swap.v1.event.swap_validated","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.swap.v1.event.swap_executed","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.swap.v1.event.swap_reverted","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let swap = world.spawn();
world.insert(swap, SwapAction {
    source_row: 3,
    source_col: 4,
    direction: SwapDirection::Right,
});
world.insert(swap, SwapState { result: None, animating: false });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-board](mge-puzzle-board.md) | Plugin board (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
