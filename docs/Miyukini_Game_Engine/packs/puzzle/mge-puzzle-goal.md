# mge-puzzle-goal

> @id mge.puzzle.goal.v1  
> @role plugin  
> @domain puzzle  
> @do track_level_objectives_win_lose_conditions  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-goal` |
| @id MSCM | `mge.puzzle.goal.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-match` |
| Hot path | Non (verification en fin de tick) |
| Headless safe | Oui |
| Complexite globale | O(g) par tick, g = nombre d'objectifs actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `GoalType` | `ScoreTarget, ClearCount, CollectColor, SurviveTime, ReachCombo, MovesLimit` | Type d'objectif du niveau |
| `GoalStatus` | `Active, Completed, Failed` | Etat courant de l'objectif |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Goal` | `mge.puzzle.goal.v1.component.goal` | `goal_type: GoalType, target_value: u64, current_value: u64, status: GoalStatus` | Objectif individuel. current_value progresse vers target_value |
| `GoalSet` | `mge.puzzle.goal.v1.component.goal_set` | `goals: Vec<EntityId>, all_required: bool` | Ensemble d'objectifs. all_required = true → tous doivent etre completes |
| `LevelResult` | `mge.puzzle.goal.v1.component.level_result` | `won: bool, stars: u32, final_score: u64` | Resultat du niveau. stars de 0 a 3. Ecrit une seule fois |

---

## 4. Formules

```
stars:
  final_score >= star_thresholds[2] → 3 etoiles
  final_score >= star_thresholds[1] → 2 etoiles
  final_score >= star_thresholds[0] → 1 etoile
  sinon → 0 etoile (victoire sans etoile possible)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_goal_progress` | `mge.puzzle.goal.v1.fn.update_goal_progress` | 1430 | Goal, Score, ComboChain, PuzzleTimer | Goal | GoalProgressUpdated | O(g) | Met a jour current_value selon goal_type : lit Score, ComboChain, timer |
| `check_goal_completion` | `mge.puzzle.goal.v1.fn.check_goal_completion` | 1431 | Goal, GoalSet | Goal | GoalCompleted | O(g) | Si current_value >= target_value → status = Completed. Emet GoalCompleted |
| `evaluate_level_result` | `mge.puzzle.goal.v1.fn.evaluate_level_result` | 1432 | GoalSet, Goal, Score | LevelResult | LevelWon, LevelFailed | O(g) | Si tous les goals requis Completed → LevelWon. Si un MovesLimit ou timer fail → LevelFailed |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `GoalProgressUpdated` | `mge.puzzle.goal.v1.event.goal_progress_updated` | `goal: EntityId, current: u64, target: u64` | `update_goal_progress` | ui (barre progression) |
| `GoalCompleted` | `mge.puzzle.goal.v1.event.goal_completed` | `goal: EntityId, goal_type: GoalType` | `check_goal_completion` | ui (animation), board |
| `LevelWon` | `mge.puzzle.goal.v1.event.level_won` | `final_score: u64, stars: u32` | `evaluate_level_result` | board (→ GameOver), ui (ecran victoire) |
| `LevelFailed` | `mge.puzzle.goal.v1.event.level_failed` | `reason: GoalType` | `evaluate_level_result` | board (→ GameOver), ui (ecran defaite) |

---

## 7. Invariants

- `Goal.current_value` ne depasse jamais `Goal.target_value` (clampe).
- `LevelResult` est ecrit une seule fois par partie. Apres ecriture, le board passe en `GameOver`.
- Un Goal avec status `Completed` ne revient jamais a `Active`.
- `GoalSet.goals` ne change pas apres initialisation du niveau.
- Un GoalType `MovesLimit` fait echouer le niveau quand current_value atteint target_value.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `star_threshold_1` | `u64` | 1000 | [100, 999999] | Seuil pour 1 etoile |
| `star_threshold_2` | `u64` | 5000 | [500, 999999] | Seuil pour 2 etoiles |
| `star_threshold_3` | `u64` | 10000 | [1000, 999999] | Seuil pour 3 etoiles |
| `default_moves_limit` | `u32` | 30 | [5, 999] | Limite de coups par defaut |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Suit la progression des objectifs | Ne calcule pas le score (→ match) |
| Determine victoire/defaite | Ne gere pas les combos (→ combo) |
| Calcule les etoiles | Ne gere pas le timer (→ timer) |
| Supporte plusieurs objectifs simultanement | Ne gere pas le contenu des niveaux (→ level design) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Goal, GoalSet, Score, ComboChain, PuzzleTimer |
| Ecrit | Goal, LevelResult |
| Emet | GoalProgressUpdated, GoalCompleted, LevelWon, LevelFailed |
| Ne touche jamais | Tile, Board, Cell, SwapAction, MatchGroup, BlockGravity |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-goal/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.goal.v1, trait Plugin impl
    ├── components.rs     # Goal, GoalSet, LevelResult
    ├── systems.rs        # update_goal_progress, check_goal_completion, evaluate_level_result
    └── events.rs         # GoalProgressUpdated, GoalCompleted, LevelWon, LevelFailed
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
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (GoalType, GoalStatus)
- [ ] Formule etoiles parametrable via GCL
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : goal progress, completion, level won, level failed, stars
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.goal.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.component.goal","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.component.goal_set","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.component.level_result","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.fn.update_goal_progress","k":"s","d":"puzzle","r":["Goal","Score","ComboChain","PuzzleTimer"],"w":["Goal"],"e":["GoalProgressUpdated"],"p":1430,"c":"O(g)"},
  {"i":"mge.puzzle.goal.v1.fn.check_goal_completion","k":"s","d":"puzzle","r":["Goal","GoalSet"],"w":["Goal"],"e":["GoalCompleted"],"p":1431,"c":"O(g)"},
  {"i":"mge.puzzle.goal.v1.fn.evaluate_level_result","k":"s","d":"puzzle","r":["GoalSet","Goal","Score"],"w":["LevelResult"],"e":["LevelWon","LevelFailed"],"p":1432,"c":"O(g)"},
  {"i":"mge.puzzle.goal.v1.event.goal_progress_updated","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.event.goal_completed","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.event.level_won","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.goal.v1.event.level_failed","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let goal_score = world.spawn();
world.insert(goal_score, Goal {
    goal_type: GoalType::ScoreTarget,
    target_value: 5000,
    current_value: 0,
    status: GoalStatus::Active,
});

let goal_set = world.spawn();
world.insert(goal_set, GoalSet { goals: vec![goal_score], all_required: true });
world.insert(goal_set, LevelResult { won: false, stars: 0, final_score: 0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-match](mge-puzzle-match.md) | Plugin match (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
