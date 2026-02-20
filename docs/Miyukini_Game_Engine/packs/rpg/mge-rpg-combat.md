# mge-rpg-combat

> @id mge.rpg.combat.v1  
> @role plugin  
> @domain rpg  
> @do resolve_damage_skills_targeting_cooldowns  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-combat` |
| @id MSCM | `mge.rpg.combat.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-stats`, `mge-rpg-inventory` |
| Hot path | Oui (resolution degats) |
| Headless safe | Oui |
| Complexite globale | O(a) par tick, a = actions en attente |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `CombatState` | `Idle, InCombat, Dead` | Etat du combattant |
| `DamageType` | `Physical, Magical, Fire, Ice, Lightning, Poison, True` | Type de degats. True = ignore resistances |
| `DamageFormula` | `Flat, Multiplicative` | Mode de calcul. GCL configurable |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Combatant` | `mge.rpg.combat.v1.component.combatant` | `faction_id: u32, combat_state: CombatState` | Marque entite comme combattant. faction_id pour friendly_fire |
| `Skill` | `mge.rpg.combat.v1.component.skill` | `id: u32, damage_base: f32, damage_type: DamageType, cost_mana: f32, cost_stamina: f32, cooldown_ticks: u32, range: f32` | Definition competence (donnee statique) |
| `SkillSlots` | `mge.rpg.combat.v1.component.skill_slots` | `skills: Vec<Skill>, cooldowns: Vec<u32>` | Competences equipees + cooldowns courants. cooldowns[i] correspond a skills[i] |
| `Target` | `mge.rpg.combat.v1.component.target` | `entity: EntityId` | Cible courante |
| `CombatAction` | `mge.rpg.combat.v1.component.combat_action` | `skill_index: u32, target: EntityId` | Action en attente. Consomme dans le tick courant |

---

## 4. Formules de degats

```
Mode Multiplicative (defaut) :
  raw_damage = skill.damage_base * (source.attack_power / 10.0)
  resistance = target.resistances[skill.damage_type]
  final_damage = raw_damage * (1.0 - resistance)

Mode Flat :
  raw_damage = skill.damage_base
  defense_reduction = target.defense * 0.5
  final_damage = max(min_damage, raw_damage - defense_reduction)

DamageType::True :
  final_damage = skill.damage_base (aucune reduction)
```

Le combat consomme mana/stamina avant le calcul de degats. Si ressource insuffisante, l'action est annulee silencieusement.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `resolve_combat_actions` | `mge.rpg.combat.v1.fn.resolve_combat_actions` | Logic (300) | CombatAction, SkillSlots, Combatant, Mana, Stamina | Mana, Stamina, SkillSlots (cooldown) | SkillUsed | O(a) | Verifie cooldown et ressource, consomme, declenche cooldown. Retire CombatAction |
| `apply_damage` | `mge.rpg.combat.v1.fn.apply_damage` | Logic (301) | SkillUsed (event), DerivedAttributes, Resistances, Health | Health | DamageDealt | O(a) | Calcule degats finaux selon formule, reduit Health.current |
| `tick_cooldowns` | `mge.rpg.combat.v1.fn.tick_cooldowns` | Logic (302) | SkillSlots | SkillSlots | none | O(n * s) | Decremente chaque cooldown > 0 |
| `check_death` | `mge.rpg.combat.v1.fn.check_death` | Logic (303) | Health, Combatant | Combatant (combat_state) | DeathEvent | O(n) | Si Health.current <= 0 et state != Dead → Dead + DeathEvent |

---

## 6. Flux de donnees

```
CombatAction ──► resolve_combat_actions ──► SkillUsed (event)
                                                │
                                                ▼
                                       apply_damage ──► DamageDealt (event)
                                                              │
                                                              ▼
                                                     check_death ──► DeathEvent (event)
```

---

## 7. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `SkillUsed` | `mge.rpg.combat.v1.event.skill_used` | `source: EntityId, skill_id: u32, target: EntityId, damage_type: DamageType` | `resolve_combat_actions` | `apply_damage`, quest, ui |
| `DamageDealt` | `mge.rpg.combat.v1.event.damage_dealt` | `source: EntityId, target: EntityId, amount: f32, damage_type: DamageType, resisted: f32` | `apply_damage` | ai (threat), quest, ui |
| `DeathEvent` | `mge.rpg.combat.v1.event.death` | `entity: EntityId, killer: Option<EntityId>` | `check_death` | quest, progression (xp), ai, loot |

---

## 8. Invariants

- Un CombatAction est consomme dans le tick ou il est pose. Jamais reporte.
- Si mana ou stamina insuffisante, l'action est ignoree. Pas d'evenement.
- Un combattant Dead ne peut pas poser de CombatAction.
- Les cooldowns ne descendent jamais en dessous de 0.
- friendly_fire = false → CombatAction entre meme faction_id ignoree.

---

## 9. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `damage_formula` | `DamageFormula` | Multiplicative | {Flat, Multiplicative} | Mode calcul degats |
| `friendly_fire` | `bool` | false | {true, false} | Degats entre meme faction |
| `min_damage` | `f32` | 1.0 | [0.0, 9999.0] | Degats minimum par attaque (hors True) |
| `death_despawn_delay_ticks` | `u32` | 0 | [0, 600] | Delai avant suppression entite morte. 0 = pas de despawn auto |

---

## 10. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Resout les CombatAction en degats | Ne gere pas les attributs de base (→ stats) |
| Gere les cooldowns de competences | Ne decide pas de l'action a effectuer (→ ai ou input) |
| Detecte la mort (Health <= 0) | Ne genere pas le loot (→ inventory.resolve_loot) |
| Applique les resistances | Ne gere pas les buffs (→ stats) |
| Consomme mana/stamina | Ne gere pas les equipements (→ inventory) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | CombatAction, SkillSlots, Combatant, Mana, Stamina, DerivedAttributes, Resistances, Health |
| Ecrit | Mana, Stamina, SkillSlots (cooldowns), Health, Combatant (combat_state) |
| Emet | SkillUsed, DamageDealt, DeathEvent |
| Ne touche jamais | Attributes, BuffStack, Inventory, QuestLog, DialogueState, AIBehavior |

---

## 11. Guide d'implementation

### Structure fichiers

```
mge-rpg-combat/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rpg.combat.v1, trait Plugin impl
    ├── components.rs     # Combatant, Skill, SkillSlots, Target, CombatAction
    ├── systems.rs        # resolve_combat_actions, apply_damage, tick_cooldowns, check_death
    └── events.rs         # SkillUsed, DamageDealt, DeathEvent
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (resolve_combat_actions, apply_damage) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 5 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (CombatState, DamageType, DamageFormula)
- [ ] Formule de degats parametrable via GCL
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : damage resolution, cooldown tick, death check, friendly fire
- [ ] AI-Native Score >= 8/10

---

## 12. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.combat.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.component.combatant","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.component.skill","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.component.skill_slots","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.component.target","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.component.combat_action","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.fn.resolve_combat_actions","k":"s","d":"rpg","r":["CombatAction","SkillSlots","Combatant","Mana","Stamina"],"w":["Mana","Stamina","SkillSlots"],"e":["SkillUsed"],"p":300,"c":"O(a)"},
  {"i":"mge.rpg.combat.v1.fn.apply_damage","k":"s","d":"rpg","r":["DerivedAttributes","Resistances","Health"],"w":["Health"],"e":["DamageDealt"],"p":301,"c":"O(a)"},
  {"i":"mge.rpg.combat.v1.fn.tick_cooldowns","k":"s","d":"rpg","r":["SkillSlots"],"w":["SkillSlots"],"e":[],"p":302,"c":"O(n*s)"},
  {"i":"mge.rpg.combat.v1.fn.check_death","k":"s","d":"rpg","r":["Health","Combatant"],"w":["Combatant"],"e":["DeathEvent"],"p":303,"c":"O(n)"},
  {"i":"mge.rpg.combat.v1.event.skill_used","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.event.damage_dealt","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.combat.v1.event.death","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 13. Exemple d'utilisation

```rust
let warrior = world.spawn();
world.insert(warrior, Combatant { faction_id: 1, combat_state: CombatState::Idle });
world.insert(warrior, SkillSlots {
    skills: vec![Skill {
        id: 1, damage_base: 25.0, damage_type: DamageType::Physical,
        cost_mana: 0.0, cost_stamina: 5.0, cooldown_ticks: 3, range: 2.0,
    }],
    cooldowns: vec![0],
});
world.insert(warrior, CombatAction { skill_index: 0, target: enemy_id });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
| [mge-rpg-stats](mge-rpg-stats.md) | Plugin stats (dependance) |
| [mge-rpg-inventory](mge-rpg-inventory.md) | Plugin inventory (dependance) |
