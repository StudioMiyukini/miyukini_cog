# mge-puzzle-board

> @id mge.puzzle.board.v1  
> @role plugin  
> @domain puzzle  
> @do manage_grid_layout_cell_occupancy_phases  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-board` |
| @id MSCM | `mge.puzzle.board.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-tile` |
| Hot path | Oui (sync_cell_occupancy chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(w*h) sur grille |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `CellState` | `Empty, Occupied, Blocked, Hole` | Etat d'une case de la grille |
| `BoardPhase` | `Idle, PlayerInput, Resolving, Cascading, Filling, GameOver` | Phase globale du board |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Board` | `mge.puzzle.board.v1.component.board` | `width: u32, height: u32, phase: BoardPhase` | Grille de jeu. Dimensions fixes apres initialisation |
| `Cell` | `mge.puzzle.board.v1.component.cell` | `row: u32, col: u32, state: CellState, occupant: Option<EntityId>` | Case individuelle. Relie position logique a entite Tile |

---

## 4. Formules

Aucune formule de derivation. Le board gere la topologie, pas les calculs.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_board_phase` | `mge.puzzle.board.v1.fn.update_board_phase` | 1405 | Board, Cell | Board | BoardPhaseChanged | O(w*h) | Evalue l'etat global (toutes les Cell stables → Idle, actions en cours → Resolving, etc.) |
| `sync_cell_occupancy` | `mge.puzzle.board.v1.fn.sync_cell_occupancy` | 1406 | Cell, Tile | Cell | none | O(w*h) | Synchronise Cell.occupant avec les Tiles presentes. Detecte Cell devenues Empty |
| `fill_empty_cells` | `mge.puzzle.board.v1.fn.fill_empty_cells` | 1407 | Board, Cell | Cell | CellFillRequested | O(w*h) | Marque les Cell Empty de la rangee superieure pour spawn. Emet CellFillRequested |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `BoardPhaseChanged` | `mge.puzzle.board.v1.event.board_phase_changed` | `board: EntityId, old_phase: BoardPhase, new_phase: BoardPhase` | `update_board_phase` | swap (bloque si Resolving), ui, hint |
| `CellFillRequested` | `mge.puzzle.board.v1.event.cell_fill_requested` | `row: u32, col: u32` | `fill_empty_cells` | tile (spawn_new_tiles) |
| `BoardInitialized` | `mge.puzzle.board.v1.event.board_initialized` | `board: EntityId, width: u32, height: u32` | Externe (setup) | tile, match, ui |

---

## 7. Invariants

- `Board.width` et `Board.height` ne changent jamais apres initialisation.
- Un Cell avec state `Blocked` ou `Hole` n'a jamais d'occupant.
- Le nombre de Cell entites = `Board.width * Board.height`.
- En phase `GameOver`, aucun systeme ne modifie les Cell.
- `Cell.occupant` est toujours `None` si `Cell.state == Empty`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_board_width` | `u32` | 8 | [3, 20] | Largeur grille par defaut |
| `default_board_height` | `u32` | 8 | [3, 20] | Hauteur grille par defaut |
| `allow_holes` | `bool` | false | {true, false} | Autorise les cases Hole (non jouables) |
| `fill_from_top` | `bool` | true | {true, false} | Les tuiles tombent du haut. false = spawn in place |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la topologie de la grille (dimensions, cases) | Ne gere pas les tuiles elles-memes (→ tile) |
| Synchronise l'occupation des cases | Ne detecte pas les matchs (→ match) |
| Gere les phases du board (Idle, Resolving, etc.) | Ne traite pas l'input joueur (→ swap) |
| Demande le remplissage des cases vides | Ne fait pas tomber les blocs (→ block) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Board, Cell, Tile |
| Ecrit | Board, Cell |
| Emet | BoardPhaseChanged, CellFillRequested, BoardInitialized |
| Ne touche jamais | Score, ComboChain, Goal, PuzzleTimer, SwapAction, MatchGroup |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-board/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.board.v1, trait Plugin impl
    ├── components.rs     # Board, Cell
    ├── systems.rs        # update_board_phase, sync_cell_occupancy, fill_empty_cells
    └── events.rs         # BoardPhaseChanged, CellFillRequested, BoardInitialized
```

### Annotations MSCM requises

**lib.rs** :
```rust
//! @id mge.puzzle.board.v1
//! @role plugin
//! @layer plugin
//! @domain puzzle
//! @do manage_grid_layout_cell_occupancy_phases
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (sync_cell_occupancy) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (CellState, BoardPhase)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : board init, cell sync, phase transitions, fill requests
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.board.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.board.v1.component.board","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.board.v1.component.cell","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.board.v1.fn.update_board_phase","k":"s","d":"puzzle","r":["Board","Cell"],"w":["Board"],"e":["BoardPhaseChanged"],"p":1405,"c":"O(w*h)"},
  {"i":"mge.puzzle.board.v1.fn.sync_cell_occupancy","k":"s","d":"puzzle","r":["Cell","Tile"],"w":["Cell"],"e":[],"p":1406,"c":"O(w*h)"},
  {"i":"mge.puzzle.board.v1.fn.fill_empty_cells","k":"s","d":"puzzle","r":["Board","Cell"],"w":["Cell"],"e":["CellFillRequested"],"p":1407,"c":"O(w*h)"},
  {"i":"mge.puzzle.board.v1.event.board_phase_changed","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.board.v1.event.cell_fill_requested","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.board.v1.event.board_initialized","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let board = world.spawn();
world.insert(board, Board { width: 8, height: 8, phase: BoardPhase::Idle });

for row in 0..8 {
    for col in 0..8 {
        let cell = world.spawn();
        world.insert(cell, Cell {
            row,
            col,
            state: CellState::Empty,
            occupant: None,
        });
    }
}
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-tile](mge-puzzle-tile.md) | Plugin tile (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
