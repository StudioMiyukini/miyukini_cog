# MGE — Pack Puzzle

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/puzzle/`  
**Nombre de crates** : 9  

---

## 1. Contexte

Le Pack Puzzle fournit les mecaniques generiques des jeux de puzzle : tuiles, grille, match-3, swap, blocs/gravite, combos, objectifs, timer et indices. Il est autonome par rapport aux autres packs genre. Les jeux Match-3, Tetris-like, et puzzle games en general l'utilisent comme fondation.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Match-3, block puzzle, tile puzzle, Tetris-like, puzzle bobble, word puzzle.
- **Hors portee** : Logique specifique a un jeu, rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack recommande (input, render-2d).

---

## 3. Vision

Le Pack Puzzle est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/puzzle/
├── mge-puzzle-tile/         # Types tuiles, etats, cycle de vie
├── mge-puzzle-board/        # Grille, dimensions, cases, phases
├── mge-puzzle-swap/         # Echange tuiles, validation
├── mge-puzzle-block/        # Blocs, gravite, chute
├── mge-puzzle-match/        # Detection matchs, scoring
├── mge-puzzle-combo/        # Chaines combo, multiplicateurs
├── mge-puzzle-goal/         # Objectifs niveau, victoire/defaite
├── mge-puzzle-timer/        # Limite temps, decompte, bonus
└── mge-puzzle-hint/         # Suggestions coups, detection impasse
```

### Graphe de dependances intra-pack

```
mge-puzzle-hint ──────► mge-puzzle-board ──────► mge-puzzle-tile
                              ▲
mge-puzzle-swap ──────────────┘
                              ▲
mge-puzzle-block ─────────────┘
                              ▲
mge-puzzle-match ─────────────┘
     │
     ▼
mge-puzzle-combo ──► mge-puzzle-match
     │
     ▼
mge-puzzle-goal ──► mge-puzzle-match
```

Crates feuilles (sans dependance intra-pack) : `mge-puzzle-tile`, `mge-puzzle-timer`.

---

## 5. Sous-packs

Aucun. Les 9 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-puzzle-tile` | `mge.puzzle.tile.v1` | [mge-puzzle-tile.md](mge-puzzle-tile.md) | Types tuiles, etats, cycle de vie |
| 2 | `mge-puzzle-board` | `mge.puzzle.board.v1` | [mge-puzzle-board.md](mge-puzzle-board.md) | Grille, dimensions, cases, phases |
| 3 | `mge-puzzle-swap` | `mge.puzzle.swap.v1` | [mge-puzzle-swap.md](mge-puzzle-swap.md) | Echange tuiles, validation |
| 4 | `mge-puzzle-block` | `mge.puzzle.block.v1` | [mge-puzzle-block.md](mge-puzzle-block.md) | Blocs, gravite, chute |
| 5 | `mge-puzzle-match` | `mge.puzzle.match.v1` | [mge-puzzle-match.md](mge-puzzle-match.md) | Detection matchs, scoring |
| 6 | `mge-puzzle-combo` | `mge.puzzle.combo.v1` | [mge-puzzle-combo.md](mge-puzzle-combo.md) | Chaines combo, multiplicateurs |
| 7 | `mge-puzzle-goal` | `mge.puzzle.goal.v1` | [mge-puzzle-goal.md](mge-puzzle-goal.md) | Objectifs niveau, victoire/defaite |
| 8 | `mge-puzzle-timer` | `mge.puzzle.timer.v1` | [mge-puzzle-timer.md](mge-puzzle-timer.md) | Limite temps, decompte, bonus |
| 9 | `mge-puzzle-hint` | `mge.puzzle.hint.v1` | [mge-puzzle-hint.md](mge-puzzle-hint.md) | Suggestions coups, detection impasse |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| tile | Tile, TileModifier | aucun |
| board | Board, Cell | aucun |
| swap | SwapAction, SwapState | aucun |
| block | BlockGravity, FallState | aucun |
| match | MatchGroup, Score, MatchConfig | aucun |
| combo | ComboChain, ComboConfig | aucun |
| goal | Goal, GoalSet, LevelResult | aucun |
| timer | PuzzleTimer, TimerBonus | aucun |
| hint | HintableMove, HintState | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1400-1402 | tile | update_tile_states, spawn_new_tiles, despawn_matched_tiles |
| 1405-1407 | board | update_board_phase, sync_cell_occupancy, fill_empty_cells |
| 1410-1412 | swap | validate_swap, execute_swap, revert_invalid_swap |
| 1415-1417 | block | apply_gravity, resolve_falls, settle_blocks |
| 1420-1422 | match | detect_matches, resolve_matches, update_score |
| 1425-1427 | combo | track_combo, apply_combo_multiplier, decay_combo |
| 1430-1432 | goal | update_goal_progress, check_goal_completion, evaluate_level_result |
| 1440-1442 | timer | tick_timer, apply_timer_bonus, check_timer_expiry |
| 1445-1447 | hint | compute_hints, show_hint, detect_no_moves |

**Ordre d'execution** : tile (1400) → board (1405) → swap (1410) → block (1415) → match (1420) → combo (1425) → goal (1430) → timer (1440) → hint (1445).

**Justification** : Les tuiles sont mises a jour en premier. Le board synchronise la grille. Le swap traite l'input joueur. La gravite fait tomber les blocs. Les matchs sont detectes sur la grille stable. Les combos sont calcules apres les matchs. Les objectifs verifient l'etat final. Le timer tourne independamment. Les hints sont calcules en dernier sur l'etat final du tick.

**Total** : 27 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| tile | (aucun) | TileSpawned, TileDespawned, TileStateChanged |
| board | (aucun) | BoardPhaseChanged, CellFillRequested, BoardInitialized |
| swap | SwapAction (composant) | SwapValidated, SwapExecuted, SwapReverted |
| block | (aucun) | BlockLanded, BlockSettled |
| match | (aucun, lit Board/Tile) | MatchDetected, MatchResolved, ScoreUpdated |
| combo | (aucun, lit MatchResolved) | ComboIncremented, ComboReset |
| goal | (aucun, lit Score/Combo) | GoalProgressUpdated, GoalCompleted, LevelWon, LevelFailed |
| timer | (aucun) | TimerExpired, TimerWarning, TimerBonusApplied |
| hint | (aucun, lit Board) | HintShown, NoMovesAvailable |

**Total** : 22 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 9 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| swap | `mge-plugin-input` (lecture input joueur) |
| tile, board, match | `mge-plugin-render-2d` (optionnel, pour rendu) |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-puzzle-board` | `mge-puzzle-tile` |
| `mge-puzzle-swap` | `mge-puzzle-board` |
| `mge-puzzle-block` | `mge-puzzle-board` |
| `mge-puzzle-match` | `mge-puzzle-board` |
| `mge-puzzle-combo` | `mge-puzzle-match` |
| `mge-puzzle-goal` | `mge-puzzle-match` |
| `mge-puzzle-hint` | `mge-puzzle-board` |

### Dependances externes (aucune)

Le Pack Puzzle n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins Puzzle sans recompilation.

**Parametres exposables :**

- Dimensions grille, presence de trous
- Nombre de couleurs tuiles, tuiles speciales
- Longueur minimum de match
- Formule de score, bonus de forme
- Multiplicateurs combo, decay
- Seuils etoiles, limites de coups
- Duree timer, seuils d'alerte
- Cooldown et delai des hints

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | Generation tuiles utilise mge-rng pour aleatoire deterministe |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 13. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | match (detection), block (gravite), board (sync) |
| **Budget cible** | < 1ms pour grille 20x20 a 60 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path |

---

## 14. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de match en diagonale | Simplification v1, extension v2 |
| Pas de power-ups combinables | Hors scope v1 |
| Pas de grille hexagonale | Simplification v1 |
| Pas de multijoueur competitif | Hors scope (reseau layer) |
| Pas de serialisation niveaux | Utiliser mge-plugin-save-load |

---

## 15. Exemple d'assemblage

### Minimal (headless, match-3 uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePuzzleTilePlugin);
engine.add_plugin(MgePuzzleBoardPlugin);
engine.add_plugin(MgePuzzleMatchPlugin);
engine.build();
```

### Complet (Match-3 jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
// Pack Puzzle
engine.add_plugin(MgePuzzleTilePlugin);
engine.add_plugin(MgePuzzleBoardPlugin);
engine.add_plugin(MgePuzzleSwapPlugin);
engine.add_plugin(MgePuzzleBlockPlugin);
engine.add_plugin(MgePuzzleMatchPlugin);
engine.add_plugin(MgePuzzleComboPlugin);
engine.add_plugin(MgePuzzleGoalPlugin);
engine.add_plugin(MgePuzzleTimerPlugin);
engine.add_plugin(MgePuzzleHintPlugin);
engine.build();
```

---

## 16. Organisation des crates

```
mge/crates/puzzle/
├── mge-puzzle-tile/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.puzzle.tile.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-puzzle-board/
│   └── (meme structure)
├── mge-puzzle-swap/
│   └── (meme structure)
├── mge-puzzle-block/
│   └── (meme structure)
├── mge-puzzle-match/
│   └── (meme structure)
├── mge-puzzle-combo/
│   └── (meme structure)
├── mge-puzzle-goal/
│   └── (meme structure)
├── mge-puzzle-timer/
│   └── (meme structure)
└── mge-puzzle-hint/
    └── (meme structure)
```

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
