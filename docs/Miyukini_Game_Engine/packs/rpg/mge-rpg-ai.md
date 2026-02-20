# mge-rpg-ai

> @id mge.rpg.ai.v1  
> @role plugin  
> @domain rpg  
> @do manage_npc_decisions_targeting_tactics_fsm  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-ai` |
| @id MSCM | `mge.rpg.ai.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-combat`, `mge-rpg-stats` |
| Dependance optionnelle | `mge-plugin-spatial` (distances, pathfinding) |
| Hot path | Oui (evalue chaque tick pour PNJ actifs) |
| Headless safe | Oui |
| Complexite globale | O(n * e) avec n = PNJ actifs, e = entites dans zone detection |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `BehaviorType` | `Passive, Aggressive, Defensive, Support, Patrol, Guard, Vendor, Scripted` | Profil comportemental |
| `AIState` | `Idle, Patrol, Chase, Combat, Flee, Return, Dead` | Etat FSM courant |
| `StanceType` | `Aggressive, Defensive, Flee` | Stance de combat |
| `GoalType` | `AttackTarget, HealAlly, Retreat, HoldPosition, FollowPath` | Objectif tactique |

---

## 3. Machine a etats (FSM)

```
                    ┌──────────────────────────────────┐
                    │                                  │
                    ▼                                  │
┌──────┐    ┌───────────┐    ┌─────────┐    ┌─────────┴──┐
│ Idle │───►│  Patrol   │───►│  Chase  │───►│   Combat   │
└──┬───┘    └───────────┘    └────┬────┘    └──┬─────┬───┘
   │                              │            │     │
   │              aggro_range     │            │     │ HP < flee_threshold
   │                              │            │     ▼
   │                              │            │  ┌──────┐
   │                              │            │  │ Flee │
   │                              │            │  └──┬───┘
   │                              │            │     │
   │                              ▼            ▼     ▼
   │                           ┌──────────────────────┐
   └──────────────────────────►│       Return         │
                               └──────────────────────┘
```

Transitions :
- `Idle → Patrol` : BehaviorType == Patrol et pathing configure
- `Idle/Patrol → Chase` : hostile dans aggro_range
- `Chase → Combat` : cible dans range d'attaque
- `Combat → Flee` : Health.current / Health.max < flee_threshold
- `Combat/Chase → Return` : cible hors leash_range OU cible morte
- `Return → Idle` : position spawn atteinte
- Tout etat → `Dead` : DeathEvent recu

---

## 4. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `AIBehavior` | `mge.rpg.ai.v1.component.ai_behavior` | `behavior_type: BehaviorType, state: AIState, spawn_position: Vec2` | Profil + etat + point de retour |
| `AIGoal` | `mge.rpg.ai.v1.component.ai_goal` | `goal_type: GoalType, priority: u32, target: Option<EntityId>` | Objectif tactique courant |
| `ThreatTable` | `mge.rpg.ai.v1.component.threat_table` | `entries: Vec<ThreatEntry>` | Tableau d'aggro. Trie par threat descendant |
| `ThreatEntry` | `mge.rpg.ai.v1.component.threat_entry` | `entity: EntityId, threat: f32` | Threat accumule |
| `CombatStance` | `mge.rpg.ai.v1.component.combat_stance` | `stance: StanceType` | Influence decide_action |
| `AIConfig` | `mge.rpg.ai.v1.component.ai_config` | `aggro_range: f32, leash_range: f32, flee_threshold: f32, return_heal: bool` | Parametres par entite |
| `PatrolPath` | `mge.rpg.ai.v1.component.patrol_path` | `waypoints: Vec<Vec2>, current_index: u32, loop_mode: bool` | Chemin de patrouille |

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `detect_hostiles` | `mge.rpg.ai.v1.fn.detect_hostiles` | Logic (800) | AIBehavior, AIConfig, Position, Combatant | none | AggroTriggered | O(n * e) | Cherche hostiles dans aggro_range |
| `update_threat_table` | `mge.rpg.ai.v1.fn.update_threat_table` | Logic (801) | DamageDealt (event), ThreatTable | ThreatTable | none | O(d) | Ajoute threat, applique decay |
| `select_target` | `mge.rpg.ai.v1.fn.select_target` | Logic (802) | ThreatTable, AIBehavior | AIGoal, Target | none | O(n * t) | Choisit cible avec plus haut threat |
| `evaluate_stance` | `mge.rpg.ai.v1.fn.evaluate_stance` | Logic (803) | Health, AIConfig, AIBehavior | CombatStance, AIBehavior (state) | FleeTriggered | O(n) | HP < flee_threshold → Flee |
| `decide_action` | `mge.rpg.ai.v1.fn.decide_action` | Logic (804) | AIBehavior, AIGoal, CombatStance, SkillSlots, Target | CombatAction | none | O(n * s) | Choisit skill selon stance. Pose CombatAction |
| `update_ai_state` | `mge.rpg.ai.v1.fn.update_ai_state` | Logic (805) | AIBehavior, AIGoal, Position, Target | AIBehavior (state) | AIStateChanged | O(n) | Evalue transitions FSM |
| `process_return` | `mge.rpg.ai.v1.fn.process_return` | Logic (806) | AIBehavior (Return), Position | Velocity | none | O(n) | Deplace vers spawn_position |

---

## 6. Flux de donnees

```
detect_hostiles ──► AggroTriggered
                         │
DamageDealt ────► update_threat_table ──► ThreatTable
                                               │
                                    select_target ──► AIGoal, Target
                                                          │
Health ──► evaluate_stance ──► CombatStance ──► decide_action ──► CombatAction
                    │                                                    │
                    ▼                                         (→ combat plugin)
             FleeTriggered
                    │
           update_ai_state ──► AIStateChanged
                    │
           process_return (si Return)
```

---

## 7. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AggroTriggered` | `mge.rpg.ai.v1.event.aggro_triggered` | `entity: EntityId, target: EntityId, distance: f32` | detect_hostiles | ui, audio |
| `AIStateChanged` | `mge.rpg.ai.v1.event.ai_state_changed` | `entity: EntityId, old: AIState, new_state: AIState` | update_ai_state | ui, audio |
| `FleeTriggered` | `mge.rpg.ai.v1.event.flee_triggered` | `entity: EntityId, health_ratio: f32` | evaluate_stance | ui, quest |

---

## 8. Invariants

- Un PNJ Dead n'execute aucun systeme AI.
- ThreatTable videe quand PNJ passe en Return.
- CombatAction pose seulement si state == Combat et stance != Flee.
- PNJ en Flee se deplace dans direction opposee a cible, aucune CombatAction.
- leash_range > aggro_range toujours.
- FSM deterministe : meme etat + meme inputs = meme transition.

---

## 9. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_aggro_range` | `f32` | 10.0 | [1.0, 100.0] | Portee detection |
| `default_leash_range` | `f32` | 30.0 | [5.0, 200.0] | Distance max avant retour |
| `default_flee_threshold` | `f32` | 0.15 | [0.0, 1.0] | Ratio HP declenchant fuite |
| `threat_decay_rate` | `f32` | 0.01 | [0.0, 1.0] | Reduction threat/tick |
| `return_heal` | `bool` | true | {true, false} | Regen pendant Return |
| `return_heal_rate` | `f32` | 1.0 | [0.0, 100.0] | HP/tick pendant Return |

---

## 10. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Decide les actions des PNJ | Ne resout pas les degats (→ combat) |
| Gere la threat table et le ciblage | Ne gere pas le pathfinding complet (→ spatial) |
| Evalue la FSM et les transitions | Ne modifie pas les stats (→ stats) |
| Gere les patrouilles | Ne gere pas les dialogues PNJ (→ dialogue) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | AIBehavior, AIConfig, AIGoal, ThreatTable, CombatStance, PatrolPath, Health, Position, Combatant, SkillSlots, Target, DamageDealt (event) |
| Ecrit | AIBehavior (state), AIGoal, ThreatTable, CombatStance, CombatAction, Target, Velocity |
| Emet | AggroTriggered, AIStateChanged, FleeTriggered |
| Ne touche jamais | Attributes, BuffStack, Inventory, QuestLog, DialogueState |

---

## 11. Guide d'implementation

### Structure fichiers

```
mge-rpg-ai/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rpg.ai.v1
    ├── components.rs     # AIBehavior, AIGoal, ThreatTable, ThreatEntry, CombatStance, AIConfig, PatrolPath
    ├── systems.rs        # 7 systemes (si > 300 lignes → systems/mod.rs + sous-fichiers)
    └── events.rs         # AggroTriggered, AIStateChanged, FleeTriggered
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 (systems.rs necessera probablement decoupe) |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (detect_hostiles, decide_action) |
| No allocation hot path | Obligatoire (pre-allouer ThreatTable) |
| No unsafe | Obligatoire |

### Decoupe systems/ si necessaire

```
mge-rpg-ai/src/systems/
├── mod.rs            # re-exports
├── detection.rs      # detect_hostiles
├── threat.rs         # update_threat_table, select_target
├── stance.rs         # evaluate_stance
├── decision.rs       # decide_action
├── state.rs          # update_ai_state
└── movement.rs       # process_return
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 7 composants dans `components.rs` avec @id et @fields
- [ ] 7 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 4 enumerations (BehaviorType, AIState, StanceType, GoalType)
- [ ] FSM implementee avec transitions deterministes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : detection, threat, target selection, stance switch, flee, return
- [ ] AI-Native Score >= 8/10

---

## 12. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.ai.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.component.ai_behavior","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.component.ai_goal","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.component.threat_table","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.component.combat_stance","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.component.ai_config","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.component.patrol_path","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.fn.detect_hostiles","k":"s","d":"rpg","r":["AIBehavior","AIConfig","Position","Combatant"],"w":[],"e":["AggroTriggered"],"p":800,"c":"O(n*e)"},
  {"i":"mge.rpg.ai.v1.fn.update_threat_table","k":"s","d":"rpg","r":["ThreatTable"],"w":["ThreatTable"],"e":[],"p":801,"c":"O(d)"},
  {"i":"mge.rpg.ai.v1.fn.select_target","k":"s","d":"rpg","r":["ThreatTable","AIBehavior"],"w":["AIGoal","Target"],"e":[],"p":802,"c":"O(n*t)"},
  {"i":"mge.rpg.ai.v1.fn.evaluate_stance","k":"s","d":"rpg","r":["Health","AIConfig","AIBehavior"],"w":["CombatStance","AIBehavior"],"e":["FleeTriggered"],"p":803,"c":"O(n)"},
  {"i":"mge.rpg.ai.v1.fn.decide_action","k":"s","d":"rpg","r":["AIBehavior","AIGoal","CombatStance","SkillSlots","Target"],"w":["CombatAction"],"e":[],"p":804,"c":"O(n*s)"},
  {"i":"mge.rpg.ai.v1.fn.update_ai_state","k":"s","d":"rpg","r":["AIBehavior","AIGoal","Position","Target"],"w":["AIBehavior"],"e":["AIStateChanged"],"p":805,"c":"O(n)"},
  {"i":"mge.rpg.ai.v1.fn.process_return","k":"s","d":"rpg","r":["AIBehavior","Position"],"w":["Velocity"],"e":[],"p":806,"c":"O(n)"},
  {"i":"mge.rpg.ai.v1.event.aggro_triggered","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.event.ai_state_changed","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.ai.v1.event.flee_triggered","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 13. Exemple d'utilisation

```rust
let goblin = world.spawn();
world.insert(goblin, Combatant { faction_id: 2, combat_state: CombatState::Idle });
world.insert(goblin, Attributes { strength: 5.0, dexterity: 7.0, intelligence: 2.0, constitution: 6.0 });
world.insert(goblin, Health { current: 40.0, max: 40.0 });
world.insert(goblin, BuffStack { buffs: vec![] });
world.insert(goblin, SkillSlots {
    skills: vec![Skill {
        id: 10, damage_base: 8.0, damage_type: DamageType::Physical,
        cost_mana: 0.0, cost_stamina: 2.0, cooldown_ticks: 2, range: 1.5,
    }],
    cooldowns: vec![0],
});
world.insert(goblin, AIBehavior {
    behavior_type: BehaviorType::Aggressive,
    state: AIState::Patrol,
    spawn_position: Vec2 { x: 100.0, y: 50.0 },
});
world.insert(goblin, AIConfig {
    aggro_range: 8.0, leash_range: 25.0, flee_threshold: 0.2, return_heal: true,
});
world.insert(goblin, ThreatTable { entries: vec![] });
world.insert(goblin, CombatStance { stance: StanceType::Aggressive });
world.insert(goblin, PatrolPath {
    waypoints: vec![Vec2 { x: 90.0, y: 50.0 }, Vec2 { x: 110.0, y: 50.0 }],
    current_index: 0, loop_mode: true,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
| [mge-rpg-combat](mge-rpg-combat.md) | Plugin combat (dependance) |
| [mge-rpg-stats](mge-rpg-stats.md) | Plugin stats (dependance) |
