# mge-puzzle-match

> @id mge.puzzle.match.v1  
> @role plugin  
> @domain puzzle  
> @do detect_resolve_tile_matches_scoring  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-match` |
| @id MSCM | `mge.puzzle.match.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-board` |
| Hot path | Oui (detect_matches chaque tick en phase Resolving) |
| Headless safe | Oui |
| Complexite globale | O(w*h) pour detection |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `MatchShape` | `Horizontal, Vertical, LShape, TShape, Cross` | Forme du match detecte. Affecte le bonus de score |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `MatchGroup` | `mge.puzzle.match.v1.component.match_group` | `tiles: Vec<EntityId>, shape: MatchShape, length: u32` | Groupe de tuiles matchees. Entite ephemere, existe 1 tick |
| `Score` | `mge.puzzle.match.v1.component.score` | `points: u64, total_matches: u32` | Score cumule de la partie. Singleton sur le board |
| `MatchConfig` | `mge.puzzle.match.v1.component.match_config` | `min_length: u32` | Configuration de detection. Singleton |

---

## 4. Formules

```
points_per_match = base_points_per_tile * match_length * shape_bonus * combo_multiplier

shape_bonus:
  Horizontal = 1.0
  Vertical   = 1.0
  LShape     = shape_bonus_l (GCL, defaut 1.5)
  TShape     = shape_bonus_t (GCL, defaut 2.0)
  Cross      = shape_bonus_cross (GCL, defaut 3.0)

combo_multiplier = lu depuis ComboChain.multiplier (plugin combo)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `detect_matches` | `mge.puzzle.match.v1.fn.detect_matches` | 1420 | Board, Cell, Tile, MatchConfig | MatchGroup | MatchDetected | O(w*h) | Scan horizontal puis vertical. Fusionne L/T/Cross. Cree MatchGroup ephemeres |
| `resolve_matches` | `mge.puzzle.match.v1.fn.resolve_matches` | 1421 | MatchGroup, Tile | Tile | MatchResolved | O(m) | Passe les Tiles matchees en etat Matched. m = nombre de matchs |
| `update_score` | `mge.puzzle.match.v1.fn.update_score` | 1422 | MatchGroup, Score, ComboChain | Score | ScoreUpdated | O(m) | Calcule points selon formule. Met a jour Score |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `MatchDetected` | `mge.puzzle.match.v1.event.match_detected` | `match_group: EntityId, shape: MatchShape, length: u32` | `detect_matches` | combo, goal, ui |
| `MatchResolved` | `mge.puzzle.match.v1.event.match_resolved` | `tiles_cleared: u32, shape: MatchShape` | `resolve_matches` | combo (track_combo), block (cascade), ui |
| `ScoreUpdated` | `mge.puzzle.match.v1.event.score_updated` | `points_added: u64, new_total: u64` | `update_score` | goal, ui |

---

## 7. Invariants

- Un match requiert au minimum `MatchConfig.min_length` tuiles adjacentes de meme couleur.
- Les tuiles Wild matchent avec n'importe quelle couleur adjacente.
- Un MatchGroup est ephemere : cree par `detect_matches`, consomme par `resolve_matches`, despawn en fin de tick.
- `detect_matches` ne detecte que sur les Tiles en etat `Idle` (pas Falling, Spawning, Locked).
- `Score.points` est monotone croissant.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `min_match_length` | `u32` | 3 | [2, 7] | Nombre minimum de tuiles pour un match |
| `base_points_per_tile` | `u64` | 10 | [1, 1000] | Points de base par tuile matchee |
| `shape_bonus_l` | `f32` | 1.5 | [1.0, 5.0] | Multiplicateur bonus forme L |
| `shape_bonus_t` | `f32` | 2.0 | [1.0, 5.0] | Multiplicateur bonus forme T |
| `shape_bonus_cross` | `f32` | 3.0 | [1.0, 10.0] | Multiplicateur bonus forme croix |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Detecte les matchs (horizontal, vertical, formes) | Ne gere pas la grille (→ board) |
| Marque les tuiles matchees | Ne despawn pas les tuiles (→ tile) |
| Calcule et met a jour le score | Ne gere pas les combos (→ combo) |
| Reconnait les formes L, T, Cross | Ne fait pas tomber les blocs (→ block) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Board, Cell, Tile, MatchConfig, MatchGroup, Score, ComboChain |
| Ecrit | MatchGroup, Tile, Score |
| Emet | MatchDetected, MatchResolved, ScoreUpdated |
| Ne touche jamais | SwapAction, FallState, Goal, PuzzleTimer, HintState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-match/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.match.v1, trait Plugin impl
    ├── components.rs     # MatchGroup, Score, MatchConfig
    ├── systems.rs        # detect_matches, resolve_matches, update_score
    └── events.rs         # MatchDetected, MatchResolved, ScoreUpdated
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (detect_matches) |
| No allocation hot path | Obligatoire (pre-allouer Vec pour MatchGroup) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (MatchShape)
- [ ] Formule de score parametrable via GCL
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : match-3 horizontal, vertical, L-shape, T-shape, cross, wild tiles
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.match.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.match.v1.component.match_group","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.match.v1.component.score","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.match.v1.component.match_config","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.match.v1.fn.detect_matches","k":"s","d":"puzzle","r":["Board","Cell","Tile","MatchConfig"],"w":["MatchGroup"],"e":["MatchDetected"],"p":1420,"c":"O(w*h)"},
  {"i":"mge.puzzle.match.v1.fn.resolve_matches","k":"s","d":"puzzle","r":["MatchGroup","Tile"],"w":["Tile"],"e":["MatchResolved"],"p":1421,"c":"O(m)"},
  {"i":"mge.puzzle.match.v1.fn.update_score","k":"s","d":"puzzle","r":["MatchGroup","Score","ComboChain"],"w":["Score"],"e":["ScoreUpdated"],"p":1422,"c":"O(m)"},
  {"i":"mge.puzzle.match.v1.event.match_detected","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.match.v1.event.match_resolved","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.match.v1.event.score_updated","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let board = world.spawn();
world.insert(board, Board { width: 8, height: 8, phase: BoardPhase::Resolving });
world.insert(board, MatchConfig { min_length: 3 });
world.insert(board, Score { points: 0, total_matches: 0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-board](mge-puzzle-board.md) | Plugin board (dependance) |
| [mge-puzzle-combo](mge-puzzle-combo.md) | Plugin combo (lit ComboChain pour score) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
