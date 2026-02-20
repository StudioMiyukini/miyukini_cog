# mge-rl-item

> @id mge.rl.item.v1  
> @role plugin  
> @domain roguelike  
> @do manage_roguelike_items_rarity_affixes  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rl-item` |
| @id MSCM | `mge.rl.item.v1` |
| Domaine | roguelike |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-inventory` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(a) par generation, a = nombre d'affixes |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RarityTier` | `Common, Uncommon, Rare, Epic, Legendary, Cursed` | Rarete. Influe sur le nombre d'affixes et la puissance |
| `AffixSlot` | `Prefix, Suffix` | Emplacement de l'affixe. 1 prefix + 1 suffix max en v1 |
| `AffixEffect` | `FlatDamage, PercentHealth, SpeedBoost, LifeSteal, PoisonOnHit, CritChance` | Effet de l'affixe. Valeur numerique dans ItemAffix |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `RogueItem` | `mge.rl.item.v1.component.rogue_item` | `base_item_id: u32, rarity: RarityTier, affixes: Vec<ItemAffix>, floor_found: u32, is_identified: bool` | Objet roguelike avec rarete et affixes. is_identified = false avant identification |
| `ItemPool` | `mge.rl.item.v1.component.item_pool` | `entries: Vec<PoolEntry>, total_weight: u32` | Pool pondere pour generation. PoolEntry = {item_id, weight, min_floor, max_floor} |
| `ItemRarity` | `mge.rl.item.v1.component.item_rarity` | `weights: HashMap<RarityTier, u32>` | Probabilites de rarete. Modifie par floor_number |
| `ItemAffix` | `mge.rl.item.v1.component.item_affix` | `slot: AffixSlot, effect: AffixEffect, value: f32, tier: u8` | Affixe d'un objet. tier = puissance (1-5) |

---

## 4. Formules

```
Selection rarete :
  adjusted_weight[tier] = base_weight[tier] * (1.0 + floor_number * rarity_scaling)
  roll = rng.range(0, total_adjusted_weight)
  tier = weighted_select(adjusted_weight, roll)

Nombre d'affixes par rarete :
  Common = 0, Uncommon = 1, Rare = 1-2, Epic = 2, Legendary = 2-3, Cursed = 1-2 (negatif)

Valeur d'affixe :
  value = base_value[effect] * (1.0 + (tier - 1) * 0.25) * floor_scaling
  floor_scaling = 1.0 + floor_number * 0.05

Selection dans le pool :
  candidates = pool.entries.filter(|e| e.min_floor <= floor && floor <= e.max_floor)
  item = weighted_select(candidates, rng)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `generate_item` | `mge.rl.item.v1.fn.generate_item` | 1840 | GenerateItemRequest (event), ItemPool, ItemRarity, DungeonSeed | World (spawn RogueItem) | ItemGenerated | O(p) | Selectionne un item dans le pool, determine la rarete, spawn l'entite |
| `roll_affixes` | `mge.rl.item.v1.fn.roll_affixes` | 1841 | ItemGenerated (event), RogueItem, DungeonSeed | RogueItem (affixes) | AffixRolled | O(a) | Genere les affixes selon la rarete. Cursed = au moins 1 affixe negatif |
| `apply_item_effect` | `mge.rl.item.v1.fn.apply_item_effect` | 1842 | ItemEquipped (event, from RPG), RogueItem | Attributes (RPG) | none | O(a) | Applique les effets des affixes sur les stats du porteur |
| `merge_duplicates` | `mge.rl.item.v1.fn.merge_duplicates` | 1843 | Inventory (RPG), RogueItem | Inventory | ItemConsumed | O(n) | Merge les consommables identiques dans l'inventaire |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ItemGenerated` | `mge.rl.item.v1.event.item_generated` | `entity: EntityId, base_item_id: u32, rarity: RarityTier, floor: u32` | `generate_item` | `roll_affixes`, ui (notification) |
| `AffixRolled` | `mge.rl.item.v1.event.affix_rolled` | `item_entity: EntityId, affix: ItemAffix` | `roll_affixes` | ui (tooltip), stats |
| `ItemConsumed` | `mge.rl.item.v1.event.item_consumed` | `entity: EntityId, base_item_id: u32, count: u32` | `merge_duplicates` | ui (notification) |

---

## 7. Invariants

- Un objet `Cursed` a toujours au moins un affixe avec valeur negative.
- Le nombre d'affixes ne depasse jamais 3 (Legendary max).
- Un objet ne peut pas avoir deux affixes du meme `AffixSlot`.
- `is_identified == false` masque les affixes au joueur (mais ils s'appliquent quand meme).
- Le pool d'objets est filtre par floor_number — un objet hors plage n'est jamais genere.
- La generation est deterministe a seed identique.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `rarity_scaling` | `f32` | 0.05 | [0.0, 0.5] | Augmentation proba rarete par etage |
| `cursed_chance` | `f32` | 0.05 | [0.0, 0.3] | Probabilite qu'un objet Rare+ devienne Cursed |
| `max_affixes` | `u8` | 3 | [1, 5] | Nombre max d'affixes (Legendary) |
| `unidentified_ratio` | `f32` | 0.3 | [0.0, 1.0] | Proportion d'objets generes non identifies |
| `floor_scaling_factor` | `f32` | 0.05 | [0.0, 0.2] | Augmentation puissance affixes par etage |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Genere des objets roguelike avec rarete et affixes | Ne gere pas l'inventaire (→ RPG inventory) |
| Selectionne dans un pool pondere par etage | Ne gere pas l'equipement (→ RPG inventory) |
| Applique les effets d'affixes sur les stats | Ne gere pas le combat (→ RPG combat ou Shooter) |
| Merge les consommables identiques | Ne gere pas le crafting (hors scope v1) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | GenerateItemRequest, ItemPool, ItemRarity, DungeonSeed, ItemEquipped (RPG event), Inventory (RPG), RogueItem |
| Ecrit | World (spawn), RogueItem (affixes), Inventory (merge), Attributes (RPG) |
| Emet | ItemGenerated, AffixRolled, ItemConsumed |
| Ne touche jamais | FloorMap, DoorState, RunState, Tombstone |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rl-item/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rl.item.v1, trait Plugin impl
    ├── components.rs     # RogueItem, ItemPool, ItemRarity, ItemAffix
    ├── systems.rs        # generate_item, roll_affixes, apply_item_effect, merge_duplicates
    └── events.rs         # ItemGenerated, AffixRolled, ItemConsumed
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
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (RarityTier, AffixSlot, AffixEffect)
- [ ] Integration avec mge-rpg-inventory (ItemEquipped event)
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : rarity weighting, affix generation, cursed invariant, pool filtering
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rl.item.v1","k":"p","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.component.rogue_item","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.component.item_pool","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.component.item_rarity","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.component.item_affix","k":"d","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.fn.generate_item","k":"s","d":"roguelike","r":["ItemPool","ItemRarity","DungeonSeed"],"w":["World"],"e":["ItemGenerated"],"p":1840,"c":"O(p)"},
  {"i":"mge.rl.item.v1.fn.roll_affixes","k":"s","d":"roguelike","r":["RogueItem","DungeonSeed"],"w":["RogueItem"],"e":["AffixRolled"],"p":1841,"c":"O(a)"},
  {"i":"mge.rl.item.v1.fn.apply_item_effect","k":"s","d":"roguelike","r":["RogueItem"],"w":["Attributes"],"e":[],"p":1842,"c":"O(a)"},
  {"i":"mge.rl.item.v1.fn.merge_duplicates","k":"s","d":"roguelike","r":["Inventory","RogueItem"],"w":["Inventory"],"e":["ItemConsumed"],"p":1843,"c":"O(n)"},
  {"i":"mge.rl.item.v1.event.item_generated","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.event.affix_rolled","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rl.item.v1.event.item_consumed","k":"e","d":"roguelike","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
// Configurer le pool d'objets pour un etage
let pool = world.spawn();
world.insert(pool, ItemPool {
    entries: vec![
        PoolEntry { item_id: 1, weight: 50, min_floor: 1, max_floor: 99 },
        PoolEntry { item_id: 2, weight: 30, min_floor: 3, max_floor: 99 },
        PoolEntry { item_id: 3, weight: 10, min_floor: 5, max_floor: 99 },
    ],
    total_weight: 90,
});
world.insert(pool, ItemRarity {
    weights: HashMap::from([
        (RarityTier::Common, 60),
        (RarityTier::Uncommon, 25),
        (RarityTier::Rare, 10),
        (RarityTier::Epic, 4),
        (RarityTier::Legendary, 1),
    ]),
});
// Demander la generation
world.push_event(GenerateItemRequest { pool_entity: pool, floor: 5 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Roguelike - Index](_index.md) | Vue d'ensemble du pack |
| [Pack RPG - mge-rpg-inventory](../rpg/mge-rpg-inventory.md) | Plugin inventaire RPG (dependance) |
