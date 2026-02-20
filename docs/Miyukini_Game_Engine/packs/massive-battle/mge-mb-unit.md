# mge-mb-unit

> @id mge.mb.unit.v1  
> @role plugin  
> @domain massive-battle  
> @do manage_squads_cohesion_group_orders  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-mb-unit` |
| @id MSCM | `mge.mb.unit.v1` |
| Domaine | massive-battle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-mb-formation` |
| Hot path | Oui (cohesion recalculee chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * m) ou n=squads, m=membres par squad |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `GroupOrderType` | `Advance, Hold, Retreat, Charge, Regroup, Skirmish` | Type d'ordre donne au squad |
| `UnitRole` | `Infantry, Cavalry, Archer, Support, Commander` | Role tactique de l'unite |
| `CohesionLevel` | `Tight, Normal, Loose, Scattered, Broken` | Niveau de cohesion du squad |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Squad` | `mge.mb.unit.v1.component.squad` | `id: u32, formation_id: EntityId, role: UnitRole, member_count: u16, max_members: u16` | Regroupement logique de soldats. Lie a une formation |
| `SquadMember` | `mge.mb.unit.v1.component.squad_member` | `squad_id: EntityId, role: UnitRole, alive: bool` | Appartenance d'une entite a un squad |
| `Cohesion` | `mge.mb.unit.v1.component.cohesion` | `value: f32, level: CohesionLevel, decay_rate: f32` | Cohesion du squad. 1.0 = parfait, 0.0 = disperse |
| `GroupOrder` | `mge.mb.unit.v1.component.group_order` | `order: GroupOrderType, target_position: Option<(f32, f32)>, priority: u8` | Ordre actif du squad. priority 0 = defaut |
| `UnitBanner` | `mge.mb.unit.v1.component.unit_banner` | `squad_id: EntityId, visible: bool, rally_radius: f32` | Banniere du squad servant de point de ralliement |

---

## 4. Formules de derivation

```
cohesion_value = 1.0 - (avg_distance_to_center / max_spread_radius)
cohesion_value = clamp(cohesion_value - decay_rate * dt, 0.0, 1.0)

CohesionLevel:
  Tight     si cohesion >= 0.9
  Normal    si cohesion >= 0.7
  Loose     si cohesion >= 0.4
  Scattered si cohesion >= 0.1
  Broken    si cohesion < 0.1
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_group_orders` | `mge.mb.unit.v1.fn.process_group_orders` | Logic (910) | Squad, GroupOrder | GroupOrder | OrderIssued | O(n) | Propage l'ordre du squad a ses membres. Emet OrderIssued |
| `update_cohesion` | `mge.mb.unit.v1.fn.update_cohesion` | Logic (911) | Squad, SquadMember, Cohesion | Cohesion | none | O(n * m) | Recalcule la cohesion en fonction des distances membres-centre |
| `check_squad_integrity` | `mge.mb.unit.v1.fn.check_squad_integrity` | Logic (912) | Squad, SquadMember | Squad | SquadBroken, MemberLost | O(n * m) | Verifie les membres vivants. Emet SquadBroken si < seuil |
| `rally_scattered` | `mge.mb.unit.v1.fn.rally_scattered` | Logic (913) | Squad, Cohesion, UnitBanner | GroupOrder | SquadRallied | O(n) | Envoie ordre Regroup aux squads Scattered pres de leur banniere |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique | Description |
|-----------|----------|--------|----------|----------------------|-------------|
| `OrderIssued` | `mge.mb.unit.v1.event.order_issued` | `squad: EntityId, order: GroupOrderType, target: Option<(f32, f32)>` | `process_group_orders` | tactics, ai | Ordre propage au squad |
| `SquadBroken` | `mge.mb.unit.v1.event.squad_broken` | `squad: EntityId, remaining: u16` | `check_squad_integrity` | morale, tactics | Squad en dessous du seuil de membres vivants |
| `SquadRallied` | `mge.mb.unit.v1.event.squad_rallied` | `squad: EntityId, new_cohesion: f32` | `rally_scattered` | morale, ai | Squad re-rallie autour de sa banniere |
| `MemberLost` | `mge.mb.unit.v1.event.member_lost` | `squad: EntityId, member: EntityId` | `check_squad_integrity` | morale | Membre du squad mort ou disparu |

---

## 7. Invariants

- `Squad.member_count` est toujours egal au nombre de `SquadMember` avec `alive = true` et `squad_id` correspondant.
- Un `SquadMember` ne peut appartenir qu'a un seul `Squad`.
- `Cohesion.value` est toujours dans [0.0, 1.0] apres `update_cohesion`.
- `CohesionLevel` est toujours coherent avec `Cohesion.value` selon les seuils.
- Un squad avec 0 membres vivants est marque `SquadBroken` et ne recoit plus d'ordres.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `max_members_per_squad` | `u16` | 20 | [4, 100] | Taille max d'un squad |
| `squad_break_threshold` | `f32` | 0.3 | [0.1, 0.5] | Ratio membres vivants/max declenchant SquadBroken |
| `cohesion_decay_rate` | `f32` | 0.01 | [0.001, 0.1] | Perte de cohesion par tick si non en formation |
| `rally_radius` | `f32` | 15.0 | [5.0, 50.0] | Rayon autour de la banniere pour le ralliement |
| `max_spread_radius` | `f32` | 30.0 | [10.0, 100.0] | Distance max du centre avant Broken |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les squads et leur composition | Ne gere pas les formations geometriques (→ formation) |
| Calcule la cohesion du squad | Ne gere pas le moral (→ morale) |
| Propage les ordres de groupe | Ne gere pas le pathfinding (→ spatial) |
| Detecte les squads brises | Ne decide pas du routage (→ morale) |
| Rallie les squads disperses | Ne gere pas les stats individuelles (→ rpg-stats) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Squad, SquadMember, Cohesion, GroupOrder, UnitBanner |
| Ecrit | Squad, Cohesion, GroupOrder |
| Emet | OrderIssued, SquadBroken, SquadRallied, MemberLost |
| Ne touche jamais | Formation, FormationSlot, Morale, TacticalStance, SupplyStock |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-mb-unit/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.mb.unit.v1, trait Plugin impl
    ├── components.rs     # Squad, SquadMember, Cohesion, GroupOrder, UnitBanner
    ├── systems.rs        # process_group_orders, update_cohesion, check_squad_integrity, rally_scattered
    └── events.rs         # OrderIssued, SquadBroken, SquadRallied, MemberLost
```

### Annotations MSCM requises

**lib.rs** :
```rust
//! @id mge.mb.unit.v1
//! @role plugin
//! @layer plugin
//! @domain massive-battle
//! @do manage_squads_cohesion_group_orders
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (update_cohesion) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 5 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (GroupOrderType, UnitRole, CohesionLevel)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : cohesion calc, squad break, rally, order propagation
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.mb.unit.v1","k":"p","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.component.squad","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.component.squad_member","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.component.cohesion","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.component.group_order","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.component.unit_banner","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.fn.process_group_orders","k":"s","d":"massive-battle","r":["Squad","GroupOrder"],"w":["GroupOrder"],"e":["OrderIssued"],"p":910,"c":"O(n)"},
  {"i":"mge.mb.unit.v1.fn.update_cohesion","k":"s","d":"massive-battle","r":["Squad","SquadMember","Cohesion"],"w":["Cohesion"],"e":[],"p":911,"c":"O(n*m)"},
  {"i":"mge.mb.unit.v1.fn.check_squad_integrity","k":"s","d":"massive-battle","r":["Squad","SquadMember"],"w":["Squad"],"e":["SquadBroken","MemberLost"],"p":912,"c":"O(n*m)"},
  {"i":"mge.mb.unit.v1.fn.rally_scattered","k":"s","d":"massive-battle","r":["Squad","Cohesion","UnitBanner"],"w":["GroupOrder"],"e":["SquadRallied"],"p":913,"c":"O(n)"},
  {"i":"mge.mb.unit.v1.event.order_issued","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.event.squad_broken","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.event.squad_rallied","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.unit.v1.event.member_lost","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let squad = world.spawn();
world.insert(squad, Squad { id: 1, formation_id: formation, role: UnitRole::Infantry, member_count: 0, max_members: 20 });
world.insert(squad, Cohesion { value: 1.0, level: CohesionLevel::Tight, decay_rate: 0.01 });
world.insert(squad, GroupOrder { order: GroupOrderType::Hold, target_position: None, priority: 0 });
world.insert(squad, UnitBanner { squad_id: squad, visible: true, rally_radius: 15.0 });

for _ in 0..20 {
    let soldier = world.spawn();
    world.insert(soldier, SquadMember { squad_id: squad, role: UnitRole::Infantry, alive: true });
}
```

---

## References

| Document | Role |
|----------|------|
| [Pack Massive Battle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
