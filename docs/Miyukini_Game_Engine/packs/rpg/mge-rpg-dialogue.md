# mge-rpg-dialogue

> @id mge.rpg.dialogue.v1  
> @role plugin  
> @domain rpg  
> @do manage_conversation_trees_choices_conditions_effects  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-dialogue` |
| @id MSCM | `mge.rpg.dialogue.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-inventory` |
| Hot path | Non (evenementiel, actif seulement en dialogue) |
| Headless safe | Oui |
| Complexite globale | O(c) par avancement, c = conditions a evaluer |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ConditionType` | `HasItem, HasLevel, HasQuest, QuestCompleted, FlagSet, StatAbove, StatBelow, Custom` | Condition de branchement |
| `EffectType` | `GiveItem, TakeItem, GiveXp, SetFlag, StartQuest, ModifyReputation, Custom` | Effet d'un choix |
| `DialogueNodeType` | `Text, Choice, Branch, End` | Type de noeud. Branch = auto-evaluation |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `DialogueTree` | `mge.rpg.dialogue.v1.component.dialogue_tree` | `id: u32, nodes: Vec<DialogueNode>, entry_node: u32` | Arbre complet. Donnee statique |
| `DialogueNode` | `mge.rpg.dialogue.v1.component.dialogue_node` | `id: u32, node_type: DialogueNodeType, speaker_key: u32, text_key: u32, choices: Vec<DialogueChoice>, next_node: Option<u32>, conditions: Vec<Condition>` | Noeud unique. speaker_key/text_key = cles localisation |
| `DialogueChoice` | `mge.rpg.dialogue.v1.component.dialogue_choice` | `text_key: u32, next_node: u32, conditions: Vec<Condition>, effects: Vec<DialogueEffect>` | Option de reponse. Filtree par conditions |
| `DialogueState` | `mge.rpg.dialogue.v1.component.dialogue_state` | `tree_id: u32, current_node: u32, active: bool, available_choices: Vec<u32>` | Etat courant. available_choices = indices valides |
| `Condition` | `mge.rpg.dialogue.v1.component.condition` | `condition_type: ConditionType, target: u32, value: i32` | target = item_id/level/quest_id/flag_id selon type |
| `DialogueEffect` | `mge.rpg.dialogue.v1.component.dialogue_effect` | `effect_type: EffectType, target: u32, value: i32` | Effet au choix. target/value dependant du type |
| `DialogueFlags` | `mge.rpg.dialogue.v1.component.dialogue_flags` | `flags: HashSet<u32>` | Drapeaux narratifs persistants |

---

## 4. Flux d'un dialogue

```
StartDialogueRequest ──► advance_dialogue (entry_node)
                                │
                    ┌───────────┴──────────────┐
                    ▼                          ▼
              node_type = Text           node_type = Choice
              → affiche texte            → evaluate_conditions
              → attend input             → filtre choix
              → avance a next_node       → attend selection
                    │                          │
                    │                   ChoiceSelectRequest
                    │                          │
                    │                   apply_dialogue_effects
                    │                          │
                    └──────────┬───────────────┘
                               ▼
                    node_type = Branch
                    → evaluate conditions automatiquement
                    → branche vers premier next_node valide
                               │
                               ▼
                    node_type = End
                    → DialogueEnded
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `advance_dialogue` | `mge.rpg.dialogue.v1.fn.advance_dialogue` | Logic (700) | StartDialogueRequest ou AdvanceRequest, DialogueTree, DialogueState | DialogueState | DialogueStarted, DialogueNodeReached, DialogueEnded | O(1) | Avance au noeud suivant |
| `evaluate_conditions` | `mge.rpg.dialogue.v1.fn.evaluate_conditions` | Logic (701) | DialogueState, DialogueTree, Inventory, Level, QuestLog, DialogueFlags | DialogueState (available_choices) | none | O(c * n) | Filtre choix par conditions |
| `apply_dialogue_effects` | `mge.rpg.dialogue.v1.fn.apply_dialogue_effects` | Logic (702) | ChoiceSelectRequest, DialogueTree, DialogueState | DialogueState, DialogueFlags | DialogueChoiceMade, PickupRequest, XpGainRequest, AcceptQuestRequest | O(e) | Execute effets du choix |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `StartDialogueRequest` | `mge.rpg.dialogue.v1.event.start_request` | `entity: EntityId, tree_id: u32, npc: EntityId` | Externe (interaction) | advance_dialogue |
| `AdvanceRequest` | `mge.rpg.dialogue.v1.event.advance_request` | `entity: EntityId` | Externe (input) | advance_dialogue |
| `ChoiceSelectRequest` | `mge.rpg.dialogue.v1.event.choice_select_request` | `entity: EntityId, choice_index: u32` | Externe (input) | apply_dialogue_effects |
| `DialogueStarted` | `mge.rpg.dialogue.v1.event.dialogue_started` | `entity: EntityId, tree_id: u32, npc: EntityId` | advance_dialogue | ui |
| `DialogueNodeReached` | `mge.rpg.dialogue.v1.event.node_reached` | `entity: EntityId, node_id: u32, speaker_key: u32, text_key: u32, node_type: DialogueNodeType` | advance_dialogue | ui |
| `DialogueChoiceMade` | `mge.rpg.dialogue.v1.event.choice_made` | `entity: EntityId, tree_id: u32, node_id: u32, choice_index: u32` | apply_dialogue_effects | quest (Talk), ui |
| `DialogueEnded` | `mge.rpg.dialogue.v1.event.dialogue_ended` | `entity: EntityId, tree_id: u32` | advance_dialogue | quest (Talk), ai |

---

## 7. Invariants

- Une seule DialogueState active par entite a la fois.
- StartDialogueRequest ignore si dialogue deja actif.
- available_choices ne contient jamais d'index invalide.
- Les effets sont appliques dans l'ordre de Vec<DialogueEffect>.
- DialogueFlags persiste entre les dialogues (gere par save-load).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `auto_advance_delay_ticks` | `u32` | 0 | [0, 600] | Delai avancement auto pour Text nodes. 0 = desactive |
| `show_hidden_choices` | `bool` | false | {true, false} | true = affiche choix dont conditions echouent (greyed out) |
| `max_dialogue_depth` | `u32` | 100 | [10, 1000] | Protection boucles infinies |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Avance dans les arbres de dialogue | Ne rend pas les dialogues (→ ui) |
| Evalue les conditions (items, level, flags) | Ne modifie pas les attributs directement (→ via events) |
| Applique les effets de choix | Ne gere pas la localisation (speaker_key/text_key) |
| Gere les drapeaux narratifs (DialogueFlags) | Ne cree pas les arbres (→ export pipeline) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | DialogueTree, DialogueState, DialogueFlags, Inventory, Level, QuestLog |
| Ecrit | DialogueState, DialogueFlags |
| Emet | DialogueStarted, DialogueNodeReached, DialogueChoiceMade, DialogueEnded, PickupRequest, XpGainRequest, AcceptQuestRequest |
| Ne touche jamais | Health, Attributes, BuffStack, ThreatTable, CombatAction |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rpg-dialogue/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rpg.dialogue.v1
    ├── components.rs     # DialogueTree, DialogueNode, DialogueChoice, DialogueState, Condition, DialogueEffect, DialogueFlags
    ├── systems.rs        # advance_dialogue, evaluate_conditions, apply_dialogue_effects
    └── events.rs         # StartDialogueRequest, AdvanceRequest, ChoiceSelectRequest, DialogueStarted, ...
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
- [ ] 7 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 7 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (ConditionType, EffectType, DialogueNodeType)
- [ ] Diagramme flux de dialogue implementable
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : start, advance, condition eval, choice select, effects, end
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.dialogue.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.component.dialogue_tree","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.component.dialogue_node","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.component.dialogue_state","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.component.dialogue_flags","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.fn.advance_dialogue","k":"s","d":"rpg","r":["DialogueTree","DialogueState"],"w":["DialogueState"],"e":["DialogueStarted","DialogueNodeReached","DialogueEnded"],"p":700,"c":"O(1)"},
  {"i":"mge.rpg.dialogue.v1.fn.evaluate_conditions","k":"s","d":"rpg","r":["DialogueState","DialogueTree","Inventory","Level","QuestLog","DialogueFlags"],"w":["DialogueState"],"e":[],"p":701,"c":"O(c*n)"},
  {"i":"mge.rpg.dialogue.v1.fn.apply_dialogue_effects","k":"s","d":"rpg","r":["DialogueTree","DialogueState"],"w":["DialogueState","DialogueFlags"],"e":["DialogueChoiceMade","PickupRequest","XpGainRequest","AcceptQuestRequest"],"p":702,"c":"O(e)"},
  {"i":"mge.rpg.dialogue.v1.event.dialogue_started","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.event.dialogue_ended","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.event.choice_made","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.dialogue.v1.event.node_reached","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
events.emit(StartDialogueRequest { entity: player, tree_id: 1, npc: npc_id });
// UI ecoute DialogueNodeReached pour afficher texte
// UI ecoute available_choices pour afficher choix
events.emit(ChoiceSelectRequest { entity: player, choice_index: 0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
| [mge-rpg-inventory](mge-rpg-inventory.md) | Plugin inventory (dependance) |
