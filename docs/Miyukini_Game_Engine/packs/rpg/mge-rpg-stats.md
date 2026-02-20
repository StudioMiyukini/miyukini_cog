# mge-rpg-stats

> @id mge.rpg.stats.v1  
> @role plugin  
> @domain rpg  
> @do manage_entity_attributes_pools_resistances_buffs  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-stats` |
| @id MSCM | `mge.rpg.stats.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (buffs actifs chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites avec buffs actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `StatTarget` | `Strength, Dexterity, Intelligence, Constitution, AttackPower, Defense, MagicPower, Speed, MaxHealth, MaxMana, MaxStamina` | Cible d'un buff |
| `ModifierMode` | `Additive, Multiplicative` | Mode d'application du buff |
| `PoolType` | `Health, Mana, Stamina` | Identifie un pool de ressources |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Attributes` | `mge.rpg.stats.v1.component.attributes` | `strength: f32, dexterity: f32, intelligence: f32, constitution: f32` | Attributs de base. Valeurs brutes avant modification |
| `DerivedAttributes` | `mge.rpg.stats.v1.component.derived_attributes` | `attack_power: f32, defense: f32, magic_power: f32, speed: f32` | Calcules par `compute_derived_attributes`. Lecture seule pour les autres plugins |
| `Health` | `mge.rpg.stats.v1.component.health` | `current: f32, max: f32` | Points de vie. current ne depasse jamais max. 0 = mort |
| `Mana` | `mge.rpg.stats.v1.component.mana` | `current: f32, max: f32` | Points de mana. Consommes par les competences |
| `Stamina` | `mge.rpg.stats.v1.component.stamina` | `current: f32, max: f32` | Points d'endurance. Consommes par actions physiques |
| `Resistances` | `mge.rpg.stats.v1.component.resistances` | `physical: f32, magical: f32, fire: f32, ice: f32, lightning: f32, poison: f32` | Reduction de degats par type. 0.0 = aucune, 1.0 = immunite |
| `Buff` | `mge.rpg.stats.v1.component.buff` | `id: u32, stat: StatTarget, modifier: f32, mode: ModifierMode, remaining_ticks: u32` | Modificateur temporaire. remaining_ticks = 0 → permanent |
| `BuffStack` | `mge.rpg.stats.v1.component.buff_stack` | `buffs: Vec<Buff>` | Pile de buffs actifs. Ordre d'insertion preserve. Limite configurable via GCL |

---

## 4. Formules de derivation

```
attack_power  = strength * 2.0 + (buffs additifs) * (1.0 + buffs multiplicatifs)
defense       = constitution * 1.5 + (buffs additifs) * (1.0 + buffs multiplicatifs)
magic_power   = intelligence * 2.0 + (buffs additifs) * (1.0 + buffs multiplicatifs)
speed         = dexterity * 1.0 + (buffs additifs) * (1.0 + buffs multiplicatifs)
```

Ordre d'application : additifs d'abord, multiplicatifs ensuite. Les buffs ne modifient pas Attributes, seulement DerivedAttributes.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_derived_attributes` | `mge.rpg.stats.v1.fn.compute_derived_attributes` | Logic (200) | Attributes, BuffStack | DerivedAttributes | none | O(n) | Recalcule DerivedAttributes. Applique buffs : additifs puis multiplicatifs |
| `tick_buffs` | `mge.rpg.stats.v1.fn.tick_buffs` | Logic (201) | BuffStack | BuffStack | BuffExpired | O(n * b) | Decremente remaining_ticks pour buffs non permanents. Supprime ceux a 0. Emet BuffExpired |
| `clamp_pools` | `mge.rpg.stats.v1.fn.clamp_pools` | Logic (202) | Health, Mana, Stamina | Health, Mana, Stamina | PoolDepleted | O(n) | Clampe current dans [0, max]. Emet PoolDepleted si current atteint 0 |
| `regen_pools` | `mge.rpg.stats.v1.fn.regen_pools` | Logic (203) | Health, Mana, Stamina | Health, Mana, Stamina | none | O(n) | Ajoute regen_rate_* a current si current > 0. Pas de regen sur pool a 0 |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique | Description |
|-----------|----------|--------|----------|----------------------|-------------|
| `BuffApplied` | `mge.rpg.stats.v1.event.buff_applied` | `target: EntityId, buff: Buff` | Externe (combat, dialogue) | UI, ai | Buff ajoute a la BuffStack. Emis par le code appelant |
| `BuffExpired` | `mge.rpg.stats.v1.event.buff_expired` | `target: EntityId, buff_id: u32` | `tick_buffs` | UI, ai | Buff retire car remaining_ticks atteint 0 |
| `PoolDepleted` | `mge.rpg.stats.v1.event.pool_depleted` | `target: EntityId, pool: PoolType` | `clamp_pools` | combat (check_death), ai (evaluate_flee) | Pool atteint 0. Pour Health, declenche verification de mort |

---

## 7. Invariants

- `Health.current` est toujours dans [0, Health.max] apres `clamp_pools`.
- `DerivedAttributes` est toujours coherent avec `Attributes` + `BuffStack` apres `compute_derived_attributes`.
- Un buff avec `remaining_ticks = 0` est permanent. Il n'est jamais supprime par `tick_buffs`.
- La suppression d'un buff permanent se fait uniquement par ecriture directe dans BuffStack.
- `BuffStack.buffs.len()` ne depasse jamais `max_buffs_per_entity`. Les buffs excedentaires sont ignores.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `regen_rate_hp` | `f32` | 0.0 | [0.0, 100.0] | Regeneration HP/tick. 0 = desactive |
| `regen_rate_mana` | `f32` | 0.0 | [0.0, 100.0] | Regeneration mana/tick. 0 = desactive |
| `regen_rate_stamina` | `f32` | 0.0 | [0.0, 100.0] | Regeneration stamina/tick. 0 = desactive |
| `max_buffs_per_entity` | `u32` | 16 | [1, 256] | Au-dela, les nouveaux buffs sont ignores |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke et met a jour les attributs de base | Ne calcule pas les degats (→ combat) |
| Gere les pools HP/Mana/Stamina | Ne decide pas de la mort (→ combat.check_death) |
| Applique et expire les buffs | Ne cree pas les buffs (le code appelant ecrit BuffStack) |
| Recalcule les attributs derives | Ne gere pas l'equipement (→ inventory) |
| Regenere les pools passivement | Ne gere pas les soins actifs (→ combat/skill) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Attributes, BuffStack, Health, Mana, Stamina |
| Ecrit | DerivedAttributes, BuffStack, Health, Mana, Stamina |
| Emet | BuffExpired, PoolDepleted |
| Ne touche jamais | Combatant, Inventory, QuestLog, AIBehavior, Position |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rpg-stats/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rpg.stats.v1, trait Plugin impl
    ├── components.rs     # Attributes, DerivedAttributes, Health, Mana, Stamina, Resistances, Buff, BuffStack
    ├── systems.rs        # compute_derived_attributes, tick_buffs, clamp_pools, regen_pools
    └── events.rs         # BuffApplied, BuffExpired, PoolDepleted
```

### Annotations MSCM requises

**lib.rs** :
```rust
//! @id mge.rpg.stats.v1
//! @role plugin
//! @layer plugin
//! @domain rpg
//! @do manage_entity_attributes_pools_resistances_buffs
```

**Chaque composant** dans components.rs :
```rust
//! @id mge.rpg.stats.v1.component.{name}
//! @role data
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}
```

**Chaque systeme** dans systems.rs :
```rust
//! @id mge.rpg.stats.v1.fn.{name}
//! @role system
//! @layer plugin
//! @do {description}
//! @requires {Comp1},{Comp2}
//! @writes {Comp1}
//! @emits {Event1} | none
//! @phase {N}
//! @complexity O(n)
```

**Chaque evenement** dans events.rs :
```rust
//! @id mge.rpg.stats.v1.event.{name}
//! @role event
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire (pas de static mut, lazy_static, thread_local) |
| No dynamic dispatch hot path | Obligatoire (compute_derived_attributes, tick_buffs) |
| No allocation hot path | Obligatoire (pre-allouer Vec<Buff>) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 8 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (StatTarget, ModifierMode, PoolType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : buff apply/expire, pool clamp, derived recalcul, regen
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.stats.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.attributes","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.derived_attributes","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.health","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.mana","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.stamina","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.resistances","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.buff","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.component.buff_stack","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.fn.compute_derived_attributes","k":"s","d":"rpg","r":["Attributes","BuffStack"],"w":["DerivedAttributes"],"e":[],"p":200,"c":"O(n)"},
  {"i":"mge.rpg.stats.v1.fn.tick_buffs","k":"s","d":"rpg","r":["BuffStack"],"w":["BuffStack"],"e":["BuffExpired"],"p":201,"c":"O(n*b)"},
  {"i":"mge.rpg.stats.v1.fn.clamp_pools","k":"s","d":"rpg","r":["Health","Mana","Stamina"],"w":["Health","Mana","Stamina"],"e":["PoolDepleted"],"p":202,"c":"O(n)"},
  {"i":"mge.rpg.stats.v1.fn.regen_pools","k":"s","d":"rpg","r":["Health","Mana","Stamina"],"w":["Health","Mana","Stamina"],"e":[],"p":203,"c":"O(n)"},
  {"i":"mge.rpg.stats.v1.event.buff_applied","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.event.buff_expired","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.stats.v1.event.pool_depleted","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Attributes { strength: 10.0, dexterity: 8.0, intelligence: 5.0, constitution: 12.0 });
world.insert(entity, DerivedAttributes::default());
world.insert(entity, Health { current: 100.0, max: 100.0 });
world.insert(entity, Mana { current: 50.0, max: 50.0 });
world.insert(entity, Stamina { current: 30.0, max: 30.0 });
world.insert(entity, Resistances { physical: 0.1, magical: 0.0, fire: 0.0, ice: 0.0, lightning: 0.0, poison: 0.0 });
world.insert(entity, BuffStack { buffs: vec![] });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
