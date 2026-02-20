# mge-rpg-progression

> @id mge.rpg.progression.v1  
> @role plugin  
> @domain rpg  
> @do manage_xp_leveling_skill_trees_unlocks  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-progression` |
| @id MSCM | `mge.rpg.progression.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (evenementiel) |
| Headless safe | Oui |
| Complexite globale | O(1) par gain XP, O(n) pour multi-level-up |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `XpCurve` | `Linear, Exponential, Custom` | Type de courbe de progression |
| `XpSource` | `Combat, Quest, Discovery, Crafting, Custom` | Origine de l'XP |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Level` | `mge.rpg.progression.v1.component.level` | `current: u32, max: u32` | Niveau courant. current ne depasse jamais max |
| `Experience` | `mge.rpg.progression.v1.component.experience` | `current: u64, to_next_level: u64` | XP dans le niveau courant. to_next_level = seuil pour level up |
| `SkillTree` | `mge.rpg.progression.v1.component.skill_tree` | `nodes: Vec<SkillNode>` | Arbre de competences. Charge depuis donnees statiques |
| `SkillNode` | `mge.rpg.progression.v1.component.skill_node` | `id: u32, unlocked: bool, requires: Vec<u32>, cost: u32` | requires = ids de noeuds prerequis |
| `SkillPoints` | `mge.rpg.progression.v1.component.skill_points` | `available: u32, total_earned: u32` | Points disponibles et total |

---

## 4. Formules de courbe XP

```
Linear :
  to_next_level = xp_base * current_level

Exponential :
  to_next_level = xp_base * (current_level ^ xp_exponent)

Custom :
  to_next_level = xp_table[current_level]  (table fournie par export pipeline)
```

Multi-level-up : si un gain XP depasse to_next_level, la boucle continue jusqu'a epuisement de l'XP restante ou max_level.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_xp_gain` | `mge.rpg.progression.v1.fn.process_xp_gain` | Logic (600) | XpGainRequest (event), Experience, Level | Experience | XpGained | O(1) | Ajoute XP a Experience.current |
| `apply_level_up` | `mge.rpg.progression.v1.fn.apply_level_up` | Logic (601) | Experience, Level, SkillPoints | Experience, Level, SkillPoints | LevelUp | O(n) | Boucle tant que current >= to_next_level ET Level < max |
| `process_skill_unlock` | `mge.rpg.progression.v1.fn.process_skill_unlock` | Logic (602) | UnlockSkillRequest (event), SkillTree, SkillPoints | SkillTree, SkillPoints | SkillUnlocked | O(r) | Verifie requires et cost, debloque |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `XpGainRequest` | `mge.rpg.progression.v1.event.xp_gain_request` | `entity: EntityId, amount: u64, source: XpSource` | Externe (quest, combat) | process_xp_gain |
| `XpGained` | `mge.rpg.progression.v1.event.xp_gained` | `entity: EntityId, amount: u64, source: XpSource, new_total: u64` | process_xp_gain | ui |
| `LevelUp` | `mge.rpg.progression.v1.event.level_up` | `entity: EntityId, old_level: u32, new_level: u32, skill_points_awarded: u32` | apply_level_up | stats (recalcul max HP/mana), ui |
| `UnlockSkillRequest` | `mge.rpg.progression.v1.event.unlock_skill_request` | `entity: EntityId, skill_id: u32` | Externe (ui) | process_skill_unlock |
| `SkillUnlocked` | `mge.rpg.progression.v1.event.skill_unlocked` | `entity: EntityId, skill_id: u32, points_remaining: u32` | process_skill_unlock | ui, combat |

---

## 7. Invariants

- Level.current est toujours dans [1, max_level].
- Experience.current est toujours dans [0, to_next_level) apres apply_level_up.
- SkillPoints.available ne descend jamais en dessous de 0.
- Un SkillNode ne peut etre debloque que si tous ses requires sont deja unlocked.
- Un SkillNode deja unlocked ne peut pas etre re-debloque (idempotent).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `xp_curve` | `XpCurve` | Exponential | {Linear, Exponential, Custom} | Type de courbe |
| `xp_base` | `u64` | 100 | [1, 999999] | XP pour niveau 1 → 2 |
| `xp_exponent` | `f32` | 1.5 | [1.0, 5.0] | Facteur exponentiel |
| `max_level` | `u32` | 100 | [1, 9999] | Plafond de niveau |
| `skill_points_per_level` | `u32` | 1 | [0, 10] | 0 = pas de skill points auto |
| `xp_table` | `Option<Vec<u64>>` | None | longueur = max_level | Table custom si XpCurve::Custom |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere XP, niveaux, skill points | Ne modifie pas les attributs au level up (→ stats ecoute LevelUp) |
| Debloque les noeuds du skill tree | Ne definit pas les effets des skills (→ combat) |
| Supporte multi-level-up | Ne gere pas les arbres visuels (→ ui) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Experience, Level, SkillTree, SkillPoints |
| Ecrit | Experience, Level, SkillTree, SkillPoints |
| Emet | XpGained, LevelUp, SkillUnlocked |
| Ne touche jamais | Health, Attributes, Inventory, QuestLog, AIBehavior |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rpg-progression/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rpg.progression.v1
    ├── components.rs     # Level, Experience, SkillTree, SkillNode, SkillPoints
    ├── systems.rs        # process_xp_gain, apply_level_up, process_skill_unlock
    └── events.rs         # XpGainRequest, XpGained, LevelUp, UnlockSkillRequest, SkillUnlocked
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
- [ ] 5 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 5 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (XpCurve, XpSource)
- [ ] Formules de courbe XP implementees
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : xp gain, level up, multi-level, skill unlock, requires check
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.progression.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.component.level","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.component.experience","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.component.skill_tree","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.component.skill_node","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.component.skill_points","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.fn.process_xp_gain","k":"s","d":"rpg","r":["Experience","Level"],"w":["Experience"],"e":["XpGained"],"p":600,"c":"O(1)"},
  {"i":"mge.rpg.progression.v1.fn.apply_level_up","k":"s","d":"rpg","r":["Experience","Level","SkillPoints"],"w":["Experience","Level","SkillPoints"],"e":["LevelUp"],"p":601,"c":"O(n)"},
  {"i":"mge.rpg.progression.v1.fn.process_skill_unlock","k":"s","d":"rpg","r":["SkillTree","SkillPoints"],"w":["SkillTree","SkillPoints"],"e":["SkillUnlocked"],"p":602,"c":"O(r)"},
  {"i":"mge.rpg.progression.v1.event.xp_gained","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.event.level_up","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.progression.v1.event.skill_unlocked","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let hero = world.spawn();
world.insert(hero, Level { current: 1, max: 100 });
world.insert(hero, Experience { current: 0, to_next_level: 100 });
world.insert(hero, SkillPoints { available: 0, total_earned: 0 });
world.insert(hero, SkillTree { nodes: vec![
    SkillNode { id: 1, unlocked: false, requires: vec![], cost: 1 },
    SkillNode { id: 2, unlocked: false, requires: vec![1], cost: 2 },
]});
events.emit(XpGainRequest { entity: hero, amount: 250, source: XpSource::Combat });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
