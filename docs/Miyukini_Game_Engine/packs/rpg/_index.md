# MGE — Pack RPG

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/rpg/`  
**Nombre de crates** : 7  

---

## 1. Contexte

Le Pack RPG fournit les mecaniques generiques des jeux de role : statistiques, combat, inventaire, quetes, progression, dialogue et IA de PNJ. Il est le pack genre le plus mature du MGE. Allumina l'utilise comme fondation. Les packs Massive Battle et Grand Strategy en dependent.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Action RPG, J-RPG, dungeon crawler, hack & slash, tactical RPG.
- **Hors portee** : Logique specifique a un jeu (Allumina, etc.), rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack recommande (spatial, input, basic-physics).

---

## 3. Vision

Le Pack RPG est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/rpg/
├── mge-rpg-stats/          # Attributs, HP, mana, buffs
├── mge-rpg-combat/         # Degats, competences, ciblage
├── mge-rpg-inventory/      # Slots, equipement, objets
├── mge-rpg-quest/          # Objectifs, tracking, recompenses
├── mge-rpg-progression/    # XP, niveaux, skill tree
├── mge-rpg-dialogue/       # Arbres conversation, choix
└── mge-rpg-ai/             # Comportements PNJ, tactiques
```

### Graphe de dependances intra-pack

```
mge-rpg-ai ──────► mge-rpg-combat ──────► mge-rpg-stats
     │                   │
     │                   └──────────────► mge-rpg-inventory
     └──────────────────────────────────► mge-rpg-stats

mge-rpg-quest ──► mge-rpg-progression

mge-rpg-dialogue ──► mge-rpg-inventory
```

Crates feuilles (sans dependance intra-pack) : `mge-rpg-stats`, `mge-rpg-inventory`, `mge-rpg-progression`.

---

## 5. Sous-packs

Aucun. Les 7 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-rpg-stats` | `mge.rpg.stats.v1` | [mge-rpg-stats.md](mge-rpg-stats.md) | Attributs, pools (HP, mana), resistances, buffs/debuffs |
| 2 | `mge-rpg-combat` | `mge.rpg.combat.v1` | [mge-rpg-combat.md](mge-rpg-combat.md) | Resolution degats, competences, ciblage, tours |
| 3 | `mge-rpg-inventory` | `mge.rpg.inventory.v1` | [mge-rpg-inventory.md](mge-rpg-inventory.md) | Slots, equipement, stacks, conteneurs |
| 4 | `mge-rpg-quest` | `mge.rpg.quest.v1` | [mge-rpg-quest.md](mge-rpg-quest.md) | Objectifs, tracking, journal, recompenses |
| 5 | `mge-rpg-progression` | `mge.rpg.progression.v1` | [mge-rpg-progression.md](mge-rpg-progression.md) | XP, montee de niveau, deblocage competences |
| 6 | `mge-rpg-dialogue` | `mge.rpg.dialogue.v1` | [mge-rpg-dialogue.md](mge-rpg-dialogue.md) | Arbres conversationnels, choix, conditions |
| 7 | `mge-rpg-ai` | `mge.rpg.ai.v1` | [mge-rpg-ai.md](mge-rpg-ai.md) | Decision PNJ, ciblage, tactiques combat |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| stats | Attributes, DerivedAttributes, Health, Mana, Stamina, Resistances, BuffStack | aucun |
| combat | Combatant, SkillSlots, Target, CombatAction | Skill |
| inventory | Inventory, ItemStack, Equipment, Container, LootTable | ItemDef |
| quest | QuestLog, QuestInstance, ObjectiveProgress | QuestDef, ObjectiveDef |
| progression | Level, Experience, SkillTree, SkillPoints | SkillNode |
| dialogue | DialogueState, DialogueFlags | DialogueTree, DialogueNode |
| ai | AIBehavior, AIGoal, ThreatTable, CombatStance, PatrolPath | AIConfig |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 200-203 | stats | compute_derived_attributes, tick_buffs, clamp_pools, regen_pools |
| 300-303 | combat | resolve_combat_actions, apply_damage, tick_cooldowns, check_death |
| 400-404 | inventory | process_item_pickup, process_equip, process_unequip, process_item_transfer, resolve_loot |
| 500-503 | quest | update_objective_progress, check_quest_completion, check_quest_expiration, process_quest_accept |
| 600-602 | progression | process_xp_gain, apply_level_up, process_skill_unlock |
| 700-702 | dialogue | advance_dialogue, evaluate_conditions, apply_dialogue_effects |
| 800-806 | ai | detect_hostiles, update_threat_table, select_target, evaluate_stance, decide_action, update_ai_state, process_return |

**Ordre d'execution** : stats (200) → combat (300) → inventory (400) → quest (500) → progression (600) → dialogue (700) → ai (800).

**Justification** : les stats sont calculees en premier car tous les autres plugins les lisent. Le combat produit DeathEvent/DamageDealt, consommes par inventory (loot), quest (kill objectives), ai (threat). L'AI est en dernier car elle lit l'etat final du tick pour decider l'action du tick suivant.

**Total** : 27 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| stats | (aucun, ecriture directe BuffStack) | BuffApplied*, BuffExpired, PoolDepleted |
| combat | CombatAction (composant) | SkillUsed, DamageDealt, DeathEvent |
| inventory | PickupRequest, EquipRequest, UnequipRequest, TransferRequest, LootRequest | ItemPickedUp, ItemEquipped, ItemUnequipped, ItemDropped, ItemTransferred, InventoryFull |
| quest | AcceptQuestRequest, CustomObjectiveEvent | QuestAccepted, QuestCompleted, QuestFailed, ObjectiveUpdated, RewardGranted |
| progression | XpGainRequest, UnlockSkillRequest | XpGained, LevelUp, SkillUnlocked |
| dialogue | StartDialogueRequest, AdvanceRequest, ChoiceSelectRequest | DialogueStarted, DialogueNodeReached, DialogueChoiceMade, DialogueEnded |
| ai | (aucun, lit les events des autres plugins) | AggroTriggered, AIStateChanged, FleeTriggered |

*BuffApplied est emis par le code appelant, pas par un systeme stats.

**Total** : 16 requests + 22 events = 38 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 7 crates | `mge-ecs`, `mge-event` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-rpg-combat` | `mge-rpg-stats`, `mge-rpg-inventory` |
| `mge-rpg-quest` | `mge-rpg-progression` |
| `mge-rpg-dialogue` | `mge-rpg-inventory` |
| `mge-rpg-ai` | `mge-rpg-combat`, `mge-rpg-stats` |

### Dependances externes (aucune)

Le Pack RPG n'a aucune dependance vers des crates externes (pas de serde, pas de rand).

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins RPG sans recompilation.

**Parametres exposables :**

- Courbes XP, limites de niveau
- Formule de degats
- Taille inventaire, limite poids
- Portees IA (aggro, leash, fuite)
- Regeneration passive
- Limite de buffs

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates RPG utilises | Usage |
|----------------|---------------------|-------|
| **Massive Battle** | stats, combat | Stats unites, resolution degats |
| **Grand Strategy** | stats, combat | Stats armees, combat strategique |
| **Roguelike** | inventory, progression | Objets, montee de niveau par run |

Le Pack RPG ne depend d'aucun autre pack genre.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Utiliser operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | Combat utilise le RNG kernel (mge-rng) pour aleatoire deterministe |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | stats (buffs), combat (resolution), ai (decision) |
| **Budget cible** | < 2ms pour 1000 entites combattantes a 60 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de combat multi-cible (AoE) | Simplification v1, extension v2 |
| Pas de crafting | Hors scope RPG (voir Pack Sandbox) |
| Pas de dialogue voice-over | Hors scope (UI layer) |
| Pas de talent tree visuel | Donnees seulement, pas de rendu |
| Pas de serialisation quetes | Utiliser mge-plugin-save-load |
| Pas de quest chain | Une quete = un bloc, pas de chainage natif |
| Pas de faction system | Voir Pack Social |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| AoE / multi-cible | Competences touchant N entites dans un rayon |
| Quest chains | Quetes enchainees avec pre-requis |
| Cooldown temps reel | Cooldowns en secondes reelles en plus des ticks |
| Item rarity / affixes | Systeme de rarete et modificateurs aleatoires |
| Talent tree branching | Arbres avec branches exclusives |
| AI behavior trees | Remplacement du systeme simple par BT complet |
| Combat phases | Init → pre-combat → combat → post-combat |

---

## 17. Exemple d'assemblage

### Minimal (headless, stats + combat uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeRpgStatsPlugin);
engine.add_plugin(MgeRpgCombatPlugin);
engine.build();
```

### Complet (Action RPG jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
engine.add_plugin(MgePluginBasicPhysics::default());
engine.add_plugin(MgePluginSaveLoad::default());
// Pack RPG
engine.add_plugin(MgeRpgStatsPlugin);
engine.add_plugin(MgeRpgCombatPlugin);
engine.add_plugin(MgeRpgInventoryPlugin);
engine.add_plugin(MgeRpgQuestPlugin);
engine.add_plugin(MgeRpgProgressionPlugin);
engine.add_plugin(MgeRpgDialoguePlugin);
engine.add_plugin(MgeRpgAiPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/rpg/
├── mge-rpg-stats/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.rpg.stats.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-rpg-combat/
│   └── (meme structure)
├── mge-rpg-inventory/
│   └── (meme structure)
├── mge-rpg-quest/
│   └── (meme structure)
├── mge-rpg-progression/
│   └── (meme structure)
├── mge-rpg-dialogue/
│   └── (meme structure)
└── mge-rpg-ai/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack RPG est la brique fondamentale des jeux de role dans MGE. Il :

- Fournit 7 plugins couvrant stats, combat, inventaire, quetes, progression, dialogue et IA.
- Reste generique : aucune logique specifique a Allumina.
- S'execute en headless, en deterministe, sans rendu.
- Sert de dependance pour Massive Battle et Grand Strategy.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 7 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
