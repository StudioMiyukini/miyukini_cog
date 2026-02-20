# mge-rl-permadeath

> @id mge.rl.permadeath.v1  
> @role plugin  
> @domain roguelike  
> @do manage_permadeath_run_state_meta_progression  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rl-permadeath` |
| @id MSCM | `mge.rl.permadeath.v1` |
| Domaine | roguelike |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-progression` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(1) par tick (surveillance), O(s) a la fin de run |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RunEndReason` | `Death, Victory, Abandon` | Raison de fin de run. Victory = boss final vaincu |
| `MetaRewardType` | `UnlockCharacter, UnlockItem, PermanentBuff, UnlockStartingGear` | Type de recompense persistante entre runs |
| `RunPhase` | `NotStarted, InProgress, Ended` | Phase globale du run |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `RunState` | `mge.rl.permadeath.v1.component.run_state` | `phase: RunPhase, run_id: u64, start_tick: u64, floor_reached: u32, end_reason: Option<RunEndReason>` | Etat global du run. Un seul RunState actif a la fois |
| `RunStats` | `mge.rl.permadeath.v1.component.run_stats` | `kills: u32, items_found: u32, floors_cleared: u32, damage_dealt: f32, damage_taken: f32, gold_collected: u32` | Statistiques du run courant. Accumule tout au long du run |
| `Tombstone` | `mge.rl.permadeath.v1.component.tombstone` | `run_id: u64, floor_reached: u32, end_reason: RunEndReason, stats: RunStats, seed: u64, timestamp: u64` | Pierre tombale generee a la mort. Persistee via meta-save |
| `RunSeed` | `mge.rl.permadeath.v1.component.run_seed` | `seed: u64, character_id: u32` | Seed du run + personnage choisi. Determine toute la generation |

---

## 4. Formules

```
Meta-progression points :
  base_points = floors_cleared * 10 + kills * 2 + items_found * 5
  victory_bonus = if end_reason == Victory { base_points * 0.5 } else { 0 }
  total_points = base_points + victory_bonus

Condition de deverrouillage :
  unlock = meta_rewards.iter().find(|r| r.condition(run_stats))
  (conditions definies par GCL : ex. "reach floor 10", "kill 100 enemies total")
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `start_run` | `mge.rl.permadeath.v1.fn.start_run` | 1860 | StartRunRequest (event), RunSeed | RunState, RunStats | RunStarted | O(1) | Initialise un nouveau run. Reset les stats. Set phase = InProgress |
| `check_permadeath` | `mge.rl.permadeath.v1.fn.check_permadeath` | 1861 | DeathEvent (from combat/health), RunState | RunState | PermadeathTriggered, RunEnded | O(1) | Si le joueur meurt et RunState.phase == InProgress, declenche la fin de run |
| `generate_tombstone` | `mge.rl.permadeath.v1.fn.generate_tombstone` | 1862 | RunEnded (event), RunState, RunStats, RunSeed | World (spawn Tombstone) | none | O(1) | Cree un Tombstone avec les stats finales du run |
| `process_meta_progression` | `mge.rl.permadeath.v1.fn.process_meta_progression` | 1863 | RunEnded (event), RunStats | World (meta unlocks) | MetaUnlocked | O(u) | Calcule les points de meta-progression. Verifie les conditions de deverrouillage |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `RunStarted` | `mge.rl.permadeath.v1.event.run_started` | `run_id: u64, seed: u64, character_id: u32` | `start_run` | procgen (seed), ui (HUD), floor (init) |
| `RunEnded` | `mge.rl.permadeath.v1.event.run_ended` | `run_id: u64, reason: RunEndReason, floor_reached: u32, stats: RunStats` | `check_permadeath` | `generate_tombstone`, `process_meta_progression`, ui (ecran fin) |
| `PermadeathTriggered` | `mge.rl.permadeath.v1.event.permadeath_triggered` | `entity: EntityId, floor: u32, killer: Option<EntityId>` | `check_permadeath` | ui (animation mort), save (clean run data) |
| `MetaUnlocked` | `mge.rl.permadeath.v1.event.meta_unlocked` | `reward_type: MetaRewardType, reward_id: u32, description_hash: u64` | `process_meta_progression` | ui (notification), save (persist) |

---

## 7. Invariants

- Un seul RunState actif a la fois (pas de runs paralleles).
- `RunState.phase` ne peut transitionner que : NotStarted → InProgress → Ended.
- Un RunEnded est emis exactement une fois par run.
- Le Tombstone est immutable apres creation (pas de modification retroactive).
- Les meta-unlocks sont persistants (jamais perdus entre sessions).
- `RunStats` est reset a zero au debut de chaque run.
- Un run Abandon ne genere pas de Tombstone mais accorde des meta-points reduits (50%).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `meta_points_per_floor` | `u32` | 10 | [1, 100] | Points de meta-progression par etage franchi |
| `meta_points_per_kill` | `u32` | 2 | [0, 50] | Points par ennemi tue |
| `victory_bonus_ratio` | `f32` | 0.5 | [0.0, 2.0] | Bonus multiplicatif en cas de victoire |
| `abandon_penalty_ratio` | `f32` | 0.5 | [0.0, 1.0] | Ratio des points accordes en cas d'abandon |
| `max_tombstones` | `u32` | 50 | [10, 500] | Nombre max de tombstones conservees |
| `enable_daily_seed` | `bool` | false | {true, false} | Seed fixe par jour pour daily runs |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le cycle de vie d'un run (start → play → end) | Ne gere pas le combat (→ RPG combat ou Shooter) |
| Detecte la permadeath sur DeathEvent joueur | Ne gere pas la generation de donjons (→ procgen) |
| Genere les tombstones | Ne gere pas la persistance fichier (→ save-load) |
| Calcule et attribue les meta-unlocks | Ne gere pas l'inventaire du run (→ RPG inventory) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | StartRunRequest, DeathEvent (RPG/Shooter), RunState, RunStats, RunSeed, RunEnded |
| Ecrit | RunState, RunStats, World (Tombstone, meta unlocks) |
| Emet | RunStarted, RunEnded, PermadeathTriggered, MetaUnlocked |
| Ne touche jamais | FloorMap, DungeonConfig, RogueItem, Inventory, Attributes |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rl-permadeath/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rl.permadeath.v1, trait Plugin impl
    ├── components.rs     # RunState, RunStats, Tombstone, RunSeed
    ├── systems.rs        # start_run, check_permadeath, generate_tombstone, process_meta_progression
    └── events.rs         # RunStarted, RunEnded, PermadeathTriggered, MetaUnlocked
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
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (RunEndReason, MetaRewardType, RunPhase)
- [ ] Integration avec DeathEvent (RPG ou Shooter)
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : run lifecycle, permadeath trigger, tombstone generation, meta points
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rl.permadeath.v1","k":"p","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.component.run_state","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.component.run_stats","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.component.tombstone","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.component.run_seed","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.fn.start_run","k":"s","d":"roguelike","r":["RunSeed"],"w":["RunState","RunStats"],"e":["RunStarted"],"p":1860,"c":"O(1)"},
  {"i":"mge.rl.permadeath.v1.fn.check_permadeath","k":"s","d":"roguelike","r":["RunState"],"w":["RunState"],"e":["PermadeathTriggered","RunEnded"],"p":1861,"c":"O(1)"},
  {"i":"mge.rl.permadeath.v1.fn.generate_tombstone","k":"s","d":"roguelike","r":["RunState","RunStats","RunSeed"],"w":["World"],"e":[],"p":1862,"c":"O(1)"},
  {"i":"mge.rl.permadeath.v1.fn.process_meta_progression","k":"s","d":"roguelike","r":["RunStats"],"w":["World"],"e":["MetaUnlocked"],"p":1863,"c":"O(u)"},
  {"i":"mge.rl.permadeath.v1.event.run_started","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.event.run_ended","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.event.permadeath_triggered","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.permadeath.v1.event.meta_unlocked","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
// Demarrer un run
let run = world.spawn();
world.insert(run, RunSeed { seed: 12345, character_id: 1 });
world.push_event(StartRunRequest { run_entity: run });

// Apres RunStarted, RunState et RunStats sont initialises :
// RunState { phase: InProgress, run_id: 12345, start_tick: current_tick, floor_reached: 0, end_reason: None }
// RunStats { kills: 0, items_found: 0, floors_cleared: 0, damage_dealt: 0.0, damage_taken: 0.0, gold_collected: 0 }

// La mort du joueur declenche automatiquement check_permadeath → PermadeathTriggered → RunEnded
```

---

## References

| Document | Role |
|----------|------|
| [Pack Roguelike - Index](_index.md) | Vue d'ensemble du pack |
| [Pack RPG - mge-rpg-progression](../rpg/mge-rpg-progression.md) | Plugin progression RPG (dependance) |
