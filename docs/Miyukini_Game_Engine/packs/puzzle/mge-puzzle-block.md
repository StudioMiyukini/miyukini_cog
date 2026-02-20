# mge-puzzle-block

> @id mge.puzzle.block.v1  
> @role plugin  
> @domain puzzle  
> @do manage_block_gravity_falling_settling  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-block` |
| @id MSCM | `mge.puzzle.block.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-board` |
| Hot path | Oui (apply_gravity chaque tick en phase Cascading) |
| Headless safe | Oui |
| Complexite globale | O(w*h) pire cas, O(f) cas moyen (f = blocs en chute) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `BlockWeight` | `Light, Normal, Heavy` | Poids du bloc, affecte la vitesse de chute |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `BlockGravity` | `mge.puzzle.block.v1.component.block_gravity` | `weight: BlockWeight, fall_speed: f32` | Proprietes de gravite. Attachable a toute Tile pour la rendre soumise a la gravite |
| `FallState` | `mge.puzzle.block.v1.component.fall_state` | `is_falling: bool, target_row: u32, progress: f32` | Etat de chute courant. progress va de 0.0 a 1.0 |

---

## 4. Formules

```
fall_duration = 1.0 / (fall_speed * weight_multiplier)
weight_multiplier:
  Light  = 1.5
  Normal = 1.0
  Heavy  = heavy_block_speed_mult (GCL)

position_interpolation = lerp(source_row, target_row, progress)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_gravity` | `mge.puzzle.block.v1.fn.apply_gravity` | 1415 | Cell, Tile, BlockGravity | FallState | none | O(w*h) | Scan colonne par colonne, de bas en haut. Si Cell en dessous vide, marque FallState |
| `resolve_falls` | `mge.puzzle.block.v1.fn.resolve_falls` | 1416 | FallState, Tile | Tile, FallState | BlockLanded | O(f) | Avance progress selon fall_speed * dt. Quand progress >= 1.0, met a jour Tile.row. Emet BlockLanded |
| `settle_blocks` | `mge.puzzle.block.v1.fn.settle_blocks` | 1417 | FallState, Cell | Cell, FallState | BlockSettled | O(f) | Met a jour Cell.occupant apres landing. Reset FallState. Emet BlockSettled |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `BlockLanded` | `mge.puzzle.block.v1.event.block_landed` | `entity: EntityId, row: u32, col: u32` | `resolve_falls` | board (sync), match (re-detect), ui (animation) |
| `BlockSettled` | `mge.puzzle.block.v1.event.block_settled` | `entity: EntityId, row: u32, col: u32` | `settle_blocks` | board (phase check), combo (cascade) |

---

## 7. Invariants

- Un bloc ne tombe que si la Cell en dessous est `Empty`.
- `FallState.progress` est toujours dans [0.0, 1.0].
- Un bloc avec `BlockWeight::Heavy` ne tombe pas a travers un bloc `Normal` (respect de l'empilement).
- Apres `settle_blocks`, `FallState.is_falling` est false et `Cell.occupant` est mis a jour.
- La gravite ne s'applique qu'en phase `Cascading` ou `Resolving`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `gravity_speed` | `f32` | 1.0 | [0.1, 10.0] | Vitesse de chute base (cases/tick) |
| `heavy_block_speed_mult` | `f32` | 0.5 | [0.1, 2.0] | Multiplicateur vitesse pour blocs Heavy |
| `instant_gravity` | `bool` | false | {true, false} | Si true, les blocs atterrissent instantanement (pas d'animation) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Applique la gravite aux blocs | Ne gere pas la grille (→ board) |
| Gere la chute et l'atterrissage | Ne detecte pas les matchs (→ match) |
| Met a jour les positions apres chute | Ne gere pas les tuiles elles-memes (→ tile) |
| Supporte les poids differents | Ne gere pas les echanges (→ swap) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Cell, Tile, BlockGravity, FallState |
| Ecrit | Tile, FallState, Cell |
| Emet | BlockLanded, BlockSettled |
| Ne touche jamais | Score, ComboChain, Goal, PuzzleTimer, SwapAction, MatchGroup |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-block/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.block.v1, trait Plugin impl
    ├── components.rs     # BlockGravity, FallState
    ├── systems.rs        # apply_gravity, resolve_falls, settle_blocks
    └── events.rs         # BlockLanded, BlockSettled
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (apply_gravity) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (BlockWeight)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : gravity apply, fall resolve, settle, heavy blocks
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.block.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.block.v1.component.block_gravity","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.block.v1.component.fall_state","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.block.v1.fn.apply_gravity","k":"s","d":"puzzle","r":["Cell","Tile","BlockGravity"],"w":["FallState"],"e":[],"p":1415,"c":"O(w*h)"},
  {"i":"mge.puzzle.block.v1.fn.resolve_falls","k":"s","d":"puzzle","r":["FallState","Tile"],"w":["Tile","FallState"],"e":["BlockLanded"],"p":1416,"c":"O(f)"},
  {"i":"mge.puzzle.block.v1.fn.settle_blocks","k":"s","d":"puzzle","r":["FallState","Cell"],"w":["Cell","FallState"],"e":["BlockSettled"],"p":1417,"c":"O(f)"},
  {"i":"mge.puzzle.block.v1.event.block_landed","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.block.v1.event.block_settled","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let tile = world.spawn();
world.insert(tile, Tile { color: TileColor::Red, state: TileState::Idle, row: 2, col: 3 });
world.insert(tile, BlockGravity { weight: BlockWeight::Normal, fall_speed: 1.0 });
world.insert(tile, FallState { is_falling: false, target_row: 2, progress: 0.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-board](mge-puzzle-board.md) | Plugin board (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
