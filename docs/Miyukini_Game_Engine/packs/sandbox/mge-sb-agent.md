# mge-sb-agent

> @id mge.sandbox.agent.v1  
> @role plugin  
> @domain sandbox  
> @do manage_autonomous_agent_routines_decisions  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-agent` |
| @id MSCM | `mge.sandbox.agent.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-sb-need` |
| Hot path | Oui (evaluate_decisions chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(a * d) a = agents, d = decisions candidates |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `AgentState` | `Idle, Working, Eating, Sleeping, Socializing, Traveling` | Etat courant de l'agent |
| `DecisionPriority` | `Low, Medium, High, Critical` | Priorite de la decision |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Agent` | `mge.sandbox.agent.v1.component.agent` | `state: AgentState, schedule_id: u32` | Agent autonome. schedule_id = planning journalier |
| `Routine` | `mge.sandbox.agent.v1.component.routine` | `actions: Vec<(u32, AgentState)>, current_step: u32, loop_: bool` | Sequence d'actions. (duree_ticks, etat). loop_ = recommencer |
| `Decision` | `mge.sandbox.agent.v1.component.decision` | `priority: DecisionPriority, target_state: AgentState, reason_need: Option<NeedType>` | Decision evaluee. Priorite la plus haute gagne |

---

## 4. Formules

```
decision_priority:
  NeedUrgency::Critical → DecisionPriority::Critical
  NeedUrgency::Urgent   → DecisionPriority::High
  NeedUrgency::Moderate  → DecisionPriority::Medium
  schedule_default       → DecisionPriority::Low

Selection : max(priority) parmi les decisions candidates
Egalite : besoin avec le current le plus bas gagne
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `evaluate_decisions` | `mge.sandbox.agent.v1.fn.evaluate_decisions` | 1525 | Agent, Need, Routine | Decision | DecisionMade | O(a*d) | Genere les decisions candidates selon besoins et routine. Selectionne la meilleure |
| `execute_routine_step` | `mge.sandbox.agent.v1.fn.execute_routine_step` | 1526 | Agent, Routine | Routine, Agent | RoutineCompleted | O(a) | Avance dans la routine. Passe au step suivant si duree ecoulee |
| `update_agent_state` | `mge.sandbox.agent.v1.fn.update_agent_state` | 1527 | Agent, Decision | Agent | AgentStateChanged | O(a) | Applique la decision selectionnee. Met a jour Agent.state |
| `process_agent_idle` | `mge.sandbox.agent.v1.fn.process_agent_idle` | 1528 | Agent | Agent | AgentStuck | O(a) | Detecte les agents Idle sans decision. Emet AgentStuck si bloque |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AgentStateChanged` | `mge.sandbox.agent.v1.event.agent_state_changed` | `entity: EntityId, old_state: AgentState, new_state: AgentState` | `update_agent_state` | ui, building (worker), wildlife (reaction) |
| `RoutineCompleted` | `mge.sandbox.agent.v1.event.routine_completed` | `entity: EntityId, schedule_id: u32` | `execute_routine_step` | agent (next routine), ui |
| `DecisionMade` | `mge.sandbox.agent.v1.event.decision_made` | `entity: EntityId, priority: DecisionPriority, target_state: AgentState` | `evaluate_decisions` | ui (thought bubble) |
| `AgentStuck` | `mge.sandbox.agent.v1.event.agent_stuck` | `entity: EntityId, current_state: AgentState` | `process_agent_idle` | debug, fallback logic |

---

## 7. Invariants

- Un agent a toujours exactement un `AgentState`.
- La decision de priorite la plus haute est toujours selectionnee.
- Un agent `Sleeping` ne genere pas de nouvelles decisions (besoin Rest satisfait).
- `Routine.current_step` est toujours dans [0, actions.len()).
- Un agent sans Routine execute uniquement les decisions basees sur les besoins.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `idle_timeout_ticks` | `u32` | 120 | [30, 600] | Ticks en Idle avant AgentStuck |
| `decision_cooldown_ticks` | `u32` | 10 | [1, 60] | Cooldown entre reevaluations |
| `max_agents_per_chunk` | `u32` | 20 | [5, 100] | Limite d'agents par chunk |
| `enable_social_need` | `bool` | true | {true, false} | Active le besoin Social |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Evalue les decisions des agents | Ne gere pas les besoins eux-memes (→ need) |
| Execute les routines journalieres | Ne deplace pas l'agent (→ spatial/movement) |
| Detecte les agents bloques | Ne gere pas les interactions sociales (→ social pack) |
| Selectionne l'etat optimal | Ne gere pas le pathfinding (→ core spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Agent, Routine, Decision, Need |
| Ecrit | Agent, Routine, Decision |
| Emet | AgentStateChanged, RoutineCompleted, DecisionMade, AgentStuck |
| Ne touche jamais | TerrainTile, Building, CraftingStation, Weather, Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-agent/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.agent.v1
    ├── components.rs     # Agent, Routine, Decision
    ├── systems.rs        # evaluate_decisions, execute_routine_step, update_agent_state, process_agent_idle
    └── events.rs         # AgentStateChanged, RoutineCompleted, DecisionMade, AgentStuck
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (evaluate_decisions) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (AgentState, DecisionPriority)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : decision priority, routine step, state change, idle stuck
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.agent.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.component.agent","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.component.routine","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.component.decision","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.fn.evaluate_decisions","k":"s","d":"sandbox","r":["Agent","Need","Routine"],"w":["Decision"],"e":["DecisionMade"],"p":1525,"c":"O(a*d)"},
  {"i":"mge.sandbox.agent.v1.fn.execute_routine_step","k":"s","d":"sandbox","r":["Agent","Routine"],"w":["Routine","Agent"],"e":["RoutineCompleted"],"p":1526,"c":"O(a)"},
  {"i":"mge.sandbox.agent.v1.fn.update_agent_state","k":"s","d":"sandbox","r":["Agent","Decision"],"w":["Agent"],"e":["AgentStateChanged"],"p":1527,"c":"O(a)"},
  {"i":"mge.sandbox.agent.v1.fn.process_agent_idle","k":"s","d":"sandbox","r":["Agent"],"w":["Agent"],"e":["AgentStuck"],"p":1528,"c":"O(a)"},
  {"i":"mge.sandbox.agent.v1.event.agent_state_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.event.routine_completed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.event.decision_made","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.agent.v1.event.agent_stuck","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let agent = world.spawn();
world.insert(agent, Agent { state: AgentState::Idle, schedule_id: 1 });
world.insert(agent, Routine {
    actions: vec![(480, AgentState::Working), (120, AgentState::Eating), (360, AgentState::Sleeping)],
    current_step: 0,
    loop_: true,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sb-need](mge-sb-need.md) | Plugin need (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
