# mge-gs-military

> @id mge.gs.military.v1  
> @role plugin  
> @domain grand-strategy  
> @do manage_armies_recruitment_maintenance_attrition  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gs-military` |
| @id MSCM | `mge.gs.military.v1` |
| Domaine | grand-strategy |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gs-economy`, `mge-gs-population`, `mge-rpg-stats`, `mge-rpg-combat` |
| Hot path | Non (militaire calcule par tick de jour) |
| Headless safe | Oui |
| Complexite globale | O(a) ou a=armees actives |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ArmyState` | `Recruiting, Idle, Marching, Engaged, Retreating, Disbanded` | Etat courant de l'armee |
| `UnitType` | `Infantry, Cavalry, Artillery, Naval, Militia` | Type d'unite militaire |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Army` | `mge.gs.military.v1.component.army` | `owner: EntityId, state: ArmyState, morale: f32, commander: Option<EntityId>, location: EntityId` | Armee avec etat, moral et localisation (province) |
| `MilitaryUnit` | `mge.gs.military.v1.component.military_unit` | `unit_type: UnitType, count: u32, quality: f32, experience: f32` | Unite au sein d'une armee. quality et experience [0, 100] |
| `Recruitment` | `mge.gs.military.v1.component.recruitment` | `unit_type: UnitType, progress: f32, duration: f32, cost_gold: f32, cost_manpower: u32` | Recrutement en cours |
| `Maintenance` | `mge.gs.military.v1.component.maintenance` | `cost_per_tick: f32, attrition_rate: f32, supply_limit: u32` | Cout d'entretien et limites logistiques |

---

## 4. Formules

```
maintenance_total   = sum(army.units.count * unit_maintenance_rate)
attrition_loss      = excess_over_supply * attrition_rate
morale_attrition    = attrition_loss * morale_attrition_factor

recruitment_delta   = dt * recruitment_speed
recruitment_done    = progress >= duration
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_recruitment` | `mge.gs.military.v1.fn.tick_recruitment` | Logic (1230) | Recruitment | Recruitment, Army | UnitRecruited, ArmyRaised | O(r) | Avance les recrutements en cours, cree les unites terminees |
| `compute_maintenance` | `mge.gs.military.v1.fn.compute_maintenance` | Logic (1231) | Army, MilitaryUnit, Maintenance | Treasury | none | O(a) | Calcule et deduit les couts d'entretien du tresor |
| `apply_attrition` | `mge.gs.military.v1.fn.apply_attrition` | Logic (1232) | Army, MilitaryUnit, Maintenance | MilitaryUnit, Army | AttritionApplied | O(a) | Applique les pertes d'attrition si l'armee depasse le supply limit |
| `update_army_state` | `mge.gs.military.v1.fn.update_army_state` | Logic (1233) | Army, MilitaryUnit | Army | ArmyDisbanded | O(a) | Met a jour l'etat de l'armee (disband si 0 unites, retreat si moral bas) |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ArmyRaised` | `mge.gs.military.v1.event.army_raised` | `owner: EntityId, army: EntityId, location: EntityId` | `tick_recruitment` | diplomacy, ui |
| `UnitRecruited` | `mge.gs.military.v1.event.unit_recruited` | `army: EntityId, unit_type: UnitType, count: u32` | `tick_recruitment` | ui |
| `ArmyDisbanded` | `mge.gs.military.v1.event.army_disbanded` | `owner: EntityId, army: EntityId, reason: String` | `update_army_state` | diplomacy, economy, ui |
| `AttritionApplied` | `mge.gs.military.v1.event.attrition_applied` | `army: EntityId, losses: u32, cause: String` | `apply_attrition` | ui |

---

## 7. Invariants

- `Army.morale` est toujours dans [0.0, 100.0].
- `MilitaryUnit.count` ne devient jamais negatif (min 0).
- Une armee avec 0 unites totales est automatiquement disbandee.
- Le recrutement ne peut commencer que si les ressources sont disponibles.
- L'attrition ne s'applique que si les troupes depassent le `supply_limit`.
- `Maintenance.cost_per_tick` est recalcule a chaque changement de composition.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `unit_maintenance_rate` | `f32` | 0.5 | [0.1, 5.0] | Cout d'entretien par unite par tick |
| `base_attrition_rate` | `f32` | 0.01 | [0.0, 0.1] | Taux de pertes par attrition au-dela du supply limit |
| `recruitment_speed` | `f32` | 1.0 | [0.1, 5.0] | Multiplicateur de vitesse de recrutement |
| `retreat_morale_threshold` | `f32` | 15.0 | [5.0, 30.0] | Seuil de moral declenchant la retraite automatique |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les armees, recrutement et entretien | Ne gere pas le combat tactique (→ rpg-combat, massive-battle) |
| Applique l'attrition et le supply | Ne gere pas le tresor (→ economy) |
| Suit le moral et l'etat des armees | Ne gere pas la demographie (→ population) |
| Recrute depuis le manpower de la province | Ne gere pas le mouvement des armees (→ spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Army, MilitaryUnit, Recruitment, Maintenance, Treasury |
| Ecrit | Army, MilitaryUnit, Recruitment, Treasury |
| Emet | ArmyRaised, UnitRecruited, ArmyDisbanded, AttritionApplied |
| Ne touche jamais | DiplomaticStance, TradeRoute, Province, Religion, Culture, CasusBelli, Decision |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gs-military/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.gs.military.v1
    ├── components.rs     # Army, MilitaryUnit, Recruitment, Maintenance
    ├── systems.rs        # tick_recruitment, compute_maintenance, apply_attrition, update_army_state
    └── events.rs         # ArmyRaised, UnitRecruited, ArmyDisbanded, AttritionApplied
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
- [ ] 4 composants avec @id et @fields
- [ ] 4 systemes avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements avec @id et @fields
- [ ] 2 enumerations (ArmyState, UnitType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : recruitment, maintenance, attrition, army state
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.gs.military.v1","k":"p","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.component.army","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.component.military_unit","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.component.recruitment","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.component.maintenance","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.fn.tick_recruitment","k":"s","d":"grand-strategy","r":["Recruitment"],"w":["Recruitment","Army"],"e":["UnitRecruited","ArmyRaised"],"p":1230,"c":"O(r)"},
  {"i":"mge.gs.military.v1.fn.compute_maintenance","k":"s","d":"grand-strategy","r":["Army","MilitaryUnit","Maintenance"],"w":["Treasury"],"e":[],"p":1231,"c":"O(a)"},
  {"i":"mge.gs.military.v1.fn.apply_attrition","k":"s","d":"grand-strategy","r":["Army","MilitaryUnit","Maintenance"],"w":["MilitaryUnit","Army"],"e":["AttritionApplied"],"p":1232,"c":"O(a)"},
  {"i":"mge.gs.military.v1.fn.update_army_state","k":"s","d":"grand-strategy","r":["Army","MilitaryUnit"],"w":["Army"],"e":["ArmyDisbanded"],"p":1233,"c":"O(a)"},
  {"i":"mge.gs.military.v1.event.army_raised","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.event.unit_recruited","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.event.army_disbanded","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.military.v1.event.attrition_applied","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let army = world.spawn();
world.insert(army, Army {
    owner: france,
    state: ArmyState::Idle,
    morale: 80.0,
    commander: Some(general),
    location: paris_province,
});
world.insert(army, MilitaryUnit {
    unit_type: UnitType::Infantry,
    count: 5000,
    quality: 60.0,
    experience: 20.0,
});
world.insert(army, Maintenance {
    cost_per_tick: 2500.0,
    attrition_rate: 0.01,
    supply_limit: 6000,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Grand Strategy - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
