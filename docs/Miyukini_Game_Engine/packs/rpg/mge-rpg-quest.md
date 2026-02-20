# mge-rpg-quest

> @id mge.rpg.quest.v1  
> @role plugin  
> @domain rpg  
> @do track_quest_objectives_rewards_completion  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-quest` |
| @id MSCM | `mge.rpg.quest.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-progression` |
| Hot path | Non (evenementiel) |
| Headless safe | Oui |
| Complexite globale | O(q * o) avec q = quetes actives, o = objectifs par quete |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `QuestState` | `Active, Completed, Failed, Expired` | Etat d'une quete |
| `ObjectiveType` | `Kill, Collect, Talk, Reach, Escort, Survive, Custom` | Type d'objectif |
| `RewardType` | `Xp, Item, Gold, SkillPoint, Reputation` | Type de recompense |
| `QuestFailReason` | `Expired, Custom` | Raison d'echec |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `QuestLog` | `mge.rpg.quest.v1.component.quest_log` | `active: Vec<QuestInstance>, completed: Vec<u32>, failed: Vec<u32>` | Journal complet |
| `QuestInstance` | `mge.rpg.quest.v1.component.quest_instance` | `quest_id: u32, state: QuestState, objectives: Vec<ObjectiveProgress>, accepted_tick: u64` | Quete en cours |
| `ObjectiveProgress` | `mge.rpg.quest.v1.component.objective_progress` | `objective_id: u32, current: u32, required: u32, completed: bool` | completed = true quand current >= required |
| `QuestDef` | `mge.rpg.quest.v1.component.quest_def` | `id: u32, name_key: u32, objectives: Vec<ObjectiveDef>, rewards: Vec<Reward>, time_limit_ticks: Option<u64>` | Definition statique |
| `ObjectiveDef` | `mge.rpg.quest.v1.component.objective_def` | `id: u32, objective_type: ObjectiveType, target_id: u32, required_count: u32` | target_id = entite/item/zone selon type |
| `Reward` | `mge.rpg.quest.v1.component.reward` | `reward_type: RewardType, id: Option<u32>, amount: u32` | id = item_id pour Item |

---

## 4. Resolution des objectifs

| ObjectiveType | Evenement ecoute | Logique |
|---------------|------------------|---------|
| `Kill` | `DeathEvent` | Si killer == entity et DeathEvent.entity.type == target_id → current++ |
| `Collect` | `ItemPickedUp` | Si item_id == target_id → current += quantity |
| `Talk` | `DialogueEnded` | Si tree_id == target_id → current = 1 |
| `Reach` | Position check (spatial) | Si distance(entity, target_id) < seuil → current = 1 |
| `Escort` | Position check (spatial) | Si escorte vivante et dans zone → current = 1 par tick valide |
| `Survive` | Tick count | current++ chaque tick si entity vivante |
| `Custom` | `CustomObjectiveEvent` | Code externe emet cet evenement |

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_objective_progress` | `mge.rpg.quest.v1.fn.update_objective_progress` | Logic (500) | QuestLog, evenements sources | QuestLog (ObjectiveProgress) | ObjectiveUpdated | O(q * o * e) | Ecoute evenements, incremente current sur objectifs matchant |
| `check_quest_completion` | `mge.rpg.quest.v1.fn.check_quest_completion` | Logic (501) | QuestLog | QuestLog (state) | QuestCompleted, RewardGranted | O(q) | Tous objectifs completed → QuestState::Completed + recompenses |
| `check_quest_expiration` | `mge.rpg.quest.v1.fn.check_quest_expiration` | Logic (502) | QuestLog, QuestDef | QuestLog (state) | QuestFailed | O(q) | time_limit_ticks depasse → Expired puis Failed |
| `process_quest_accept` | `mge.rpg.quest.v1.fn.process_quest_accept` | Logic (503) | AcceptQuestRequest (event), QuestDef, QuestLog | QuestLog | QuestAccepted | O(1) | Cree QuestInstance, refuse si max_active_quests atteint |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AcceptQuestRequest` | `mge.rpg.quest.v1.event.accept_quest_request` | `entity: EntityId, quest_id: u32` | Externe (dialogue, ui) | process_quest_accept |
| `QuestAccepted` | `mge.rpg.quest.v1.event.quest_accepted` | `entity: EntityId, quest_id: u32` | process_quest_accept | ui, dialogue |
| `QuestCompleted` | `mge.rpg.quest.v1.event.quest_completed` | `entity: EntityId, quest_id: u32` | check_quest_completion | progression (xp), ui |
| `QuestFailed` | `mge.rpg.quest.v1.event.quest_failed` | `entity: EntityId, quest_id: u32, reason: QuestFailReason` | check_quest_expiration | ui |
| `ObjectiveUpdated` | `mge.rpg.quest.v1.event.objective_updated` | `entity: EntityId, quest_id: u32, objective_id: u32, current: u32, required: u32` | update_objective_progress | ui |
| `RewardGranted` | `mge.rpg.quest.v1.event.reward_granted` | `entity: EntityId, reward: Reward` | check_quest_completion | inventory, progression |
| `CustomObjectiveEvent` | `mge.rpg.quest.v1.event.custom_objective` | `entity: EntityId, objective_id: u32, increment: u32` | Externe | update_objective_progress |

---

## 7. Invariants

- Une quete ne peut etre dans active et completed simultanement.
- ObjectiveProgress.completed ne repasse jamais a false.
- RewardGranted est emis une seule fois par quete.
- max_active_quests est verifie avant ajout. Si depasse, AcceptQuestRequest ignore.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `max_active_quests` | `u32` | 10 | [1, 100] | Quetes simultanees max |
| `auto_complete` | `bool` | true | {true, false} | true = complete auto quand tous objectifs valides |
| `track_failed` | `bool` | true | {true, false} | Stocker les quetes echouees dans QuestLog.failed |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Tracking objectifs, validation, recompenses | Ne definit pas le contenu des quetes (→ export pipeline) |
| Ecoute les evenements des autres plugins pour MAJ | Ne modifie pas l'inventaire directement (emet RewardGranted) |
| Gere l'expiration par time_limit_ticks | Ne gere pas l'affichage (→ ui) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | QuestLog, QuestDef, evenements sources (DeathEvent, ItemPickedUp, DialogueEnded) |
| Ecrit | QuestLog (state, ObjectiveProgress) |
| Emet | QuestAccepted, QuestCompleted, QuestFailed, ObjectiveUpdated, RewardGranted |
| Ne touche jamais | Health, Attributes, Inventory, Equipment, AIBehavior |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rpg-quest/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rpg.quest.v1
    ├── components.rs     # QuestLog, QuestInstance, ObjectiveProgress, QuestDef, ObjectiveDef, Reward
    ├── systems.rs        # update_objective_progress, check_quest_completion, check_quest_expiration, process_quest_accept
    └── events.rs         # AcceptQuestRequest, QuestAccepted, QuestCompleted, ...
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
- [ ] 6 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 7 evenements dans `events.rs` avec @id et @fields
- [ ] 4 enumerations (QuestState, ObjectiveType, RewardType, QuestFailReason)
- [ ] Table de resolution par ObjectiveType
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : accept, progress, completion, expiration, reward
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.quest.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.component.quest_log","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.component.quest_instance","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.component.objective_progress","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.component.quest_def","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.fn.update_objective_progress","k":"s","d":"rpg","r":["QuestLog"],"w":["QuestLog"],"e":["ObjectiveUpdated"],"p":500,"c":"O(q*o*e)"},
  {"i":"mge.rpg.quest.v1.fn.check_quest_completion","k":"s","d":"rpg","r":["QuestLog"],"w":["QuestLog"],"e":["QuestCompleted","RewardGranted"],"p":501,"c":"O(q)"},
  {"i":"mge.rpg.quest.v1.fn.check_quest_expiration","k":"s","d":"rpg","r":["QuestLog","QuestDef"],"w":["QuestLog"],"e":["QuestFailed"],"p":502,"c":"O(q)"},
  {"i":"mge.rpg.quest.v1.fn.process_quest_accept","k":"s","d":"rpg","r":["QuestDef","QuestLog"],"w":["QuestLog"],"e":["QuestAccepted"],"p":503,"c":"O(1)"},
  {"i":"mge.rpg.quest.v1.event.quest_accepted","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.event.quest_completed","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.event.quest_failed","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.event.objective_updated","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.quest.v1.event.reward_granted","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, QuestLog { active: vec![], completed: vec![], failed: vec![] });
events.emit(AcceptQuestRequest { entity: player, quest_id: 1 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
| [mge-rpg-progression](mge-rpg-progression.md) | Plugin progression (dependance) |
