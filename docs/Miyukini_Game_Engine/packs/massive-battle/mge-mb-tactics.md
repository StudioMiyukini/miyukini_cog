# mge-mb-tactics

> @id mge.mb.tactics.v1  
> @role plugin  
> @domain massive-battle  
> @do manage_flanks_charge_retreat_tactical_maneuvers  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-mb-tactics` |
| @id MSCM | `mge.mb.tactics.v1` |
| Domaine | massive-battle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-mb-formation`, `mge-mb-morale` |
| Hot path | Oui (detection flancs chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n²) pour detection flancs entre squads |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TacticalManeuver` | `Flank, Charge, Feint, Envelop, Retreat, HoldLine` | Type de manoeuvre tactique |
| `StanceType` | `Aggressive, Defensive, Balanced, Skirmish` | Posture tactique du squad |
| `FlankDirection` | `Left, Right, Rear, None` | Direction du flanc detecte |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TacticalStance` | `mge.mb.tactics.v1.component.tactical_stance` | `stance: StanceType, bonus_attack: f32, bonus_defense: f32` | Posture affectant les bonus combat |
| `FlankBonus` | `mge.mb.tactics.v1.component.flank_bonus` | `direction: FlankDirection, damage_multiplier: f32, morale_penalty: f32` | Bonus applique quand un flanc est exploite |
| `ChargeState` | `mge.mb.tactics.v1.component.charge_state` | `charging: bool, charge_distance: f32, impact_damage: f32, momentum: f32` | Etat de charge en cours. momentum decroit avec la distance |
| `ManeuverOrder` | `mge.mb.tactics.v1.component.maneuver_order` | `maneuver: TacticalManeuver, target: Option<EntityId>, progress: f32` | Manoeuvre en cours d'execution. progress de 0.0 a 1.0 |

---

## 4. Formules de derivation

```
flank_damage_mult   = base_flank_mult * (1.0 + stance_bonus)
charge_impact       = base_charge_damage * momentum * charge_speed_factor
momentum            = clamp(1.0 - (distance_traveled / max_charge_distance), 0.0, 1.0)

Stance bonuses:
  Aggressive = attack +30%, defense -15%
  Defensive  = attack -10%, defense +30%
  Balanced   = attack +0%,  defense +0%
  Skirmish   = attack +10%, defense -5%, speed +20%
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `detect_flank_opportunity` | `mge.mb.tactics.v1.fn.detect_flank_opportunity` | Logic (930) | Formation, TacticalStance | FlankBonus | FlankDetected | O(n²) | Compare orientations des formations pour detecter les flancs exposes |
| `execute_charge` | `mge.mb.tactics.v1.fn.execute_charge` | Logic (931) | ChargeState, ManeuverOrder | ChargeState | ChargeImpact | O(n) | Avance les charges en cours, calcule l'impact a l'arrivee |
| `apply_tactical_bonuses` | `mge.mb.tactics.v1.fn.apply_tactical_bonuses` | Logic (932) | TacticalStance, FlankBonus | TacticalStance | none | O(n) | Applique les bonus de stance et de flanc aux stats de combat |
| `process_retreat` | `mge.mb.tactics.v1.fn.process_retreat` | Logic (933) | ManeuverOrder, Morale | ManeuverOrder | RetreatOrdered, ManeuverComplete | O(n) | Execute les retraites ordonnees. Emet ManeuverComplete a la fin |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique | Description |
|-----------|----------|--------|----------|----------------------|-------------|
| `FlankDetected` | `mge.mb.tactics.v1.event.flank_detected` | `attacker: EntityId, defender: EntityId, direction: FlankDirection` | `detect_flank_opportunity` | morale, ai | Flanc expose detecte |
| `ChargeImpact` | `mge.mb.tactics.v1.event.charge_impact` | `charger: EntityId, target: EntityId, damage: f32, momentum: f32` | `execute_charge` | combat, morale | Impact de charge sur la cible |
| `RetreatOrdered` | `mge.mb.tactics.v1.event.retreat_ordered` | `squad: EntityId, direction: (f32, f32)` | `process_retreat` | unit, formation | Retraite ordonnee pour un squad |
| `ManeuverComplete` | `mge.mb.tactics.v1.event.maneuver_complete` | `squad: EntityId, maneuver: TacticalManeuver, success: bool` | `process_retreat` | ai | Manoeuvre terminee (succes ou echec) |

---

## 7. Invariants

- `FlankBonus` n'est jamais applique a un squad face a l'attaquant (FlankDirection::None).
- `ChargeState.momentum` est toujours dans [0.0, 1.0].
- Un squad en `Retreat` ne peut pas executer de `Charge` simultanement.
- `TacticalStance` ne modifie pas directement `DerivedAttributes` — il fournit des multiplicateurs consultes par combat.
- `ManeuverOrder.progress` est toujours dans [0.0, 1.0]. 1.0 = manoeuvre terminee.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `base_flank_multiplier` | `f32` | 1.5 | [1.1, 3.0] | Multiplicateur de degats de base pour un flanc |
| `max_charge_distance` | `f32` | 50.0 | [10.0, 200.0] | Distance max d'une charge avant perte totale de momentum |
| `charge_speed_factor` | `f32` | 2.0 | [1.2, 4.0] | Multiplicateur de vitesse pendant une charge |
| `flank_morale_penalty` | `f32` | 15.0 | [5.0, 30.0] | Malus moral inflige par un flanc reussi |
| `retreat_speed_multiplier` | `f32` | 1.3 | [1.0, 2.0] | Multiplicateur de vitesse en retraite |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Detecte les flancs par comparaison d'orientations | Ne calcule pas les degats (→ rpg-combat) |
| Gere les charges et leur momentum | Ne deplace pas les entites (→ spatial) |
| Fournit les bonus tactiques | Ne modifie pas le moral directement (→ morale) |
| Execute les retraites ordonnees | Ne decide pas de la fuite (→ morale.rout) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Formation, TacticalStance, ChargeState, ManeuverOrder, Morale |
| Ecrit | FlankBonus, ChargeState, TacticalStance, ManeuverOrder |
| Emet | FlankDetected, ChargeImpact, RetreatOrdered, ManeuverComplete |
| Ne touche jamais | Squad, SquadMember, SupplyStock, WallSection, SiegeEngine |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-mb-tactics/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.mb.tactics.v1
    ├── components.rs     # TacticalStance, FlankBonus, ChargeState, ManeuverOrder
    ├── systems.rs        # detect_flank_opportunity, execute_charge, apply_tactical_bonuses, process_retreat
    └── events.rs         # FlankDetected, ChargeImpact, RetreatOrdered, ManeuverComplete
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (detect_flank_opportunity) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs`
- [ ] 4 systemes dans `systems.rs`
- [ ] 4 evenements dans `events.rs`
- [ ] 3 enumerations (TacticalManeuver, StanceType, FlankDirection)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : flank detection, charge momentum, stance bonuses, retreat
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.mb.tactics.v1","k":"p","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.component.tactical_stance","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.component.flank_bonus","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.component.charge_state","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.component.maneuver_order","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.fn.detect_flank_opportunity","k":"s","d":"massive-battle","r":["Formation","TacticalStance"],"w":["FlankBonus"],"e":["FlankDetected"],"p":930,"c":"O(n²)"},
  {"i":"mge.mb.tactics.v1.fn.execute_charge","k":"s","d":"massive-battle","r":["ChargeState","ManeuverOrder"],"w":["ChargeState"],"e":["ChargeImpact"],"p":931,"c":"O(n)"},
  {"i":"mge.mb.tactics.v1.fn.apply_tactical_bonuses","k":"s","d":"massive-battle","r":["TacticalStance","FlankBonus"],"w":["TacticalStance"],"e":[],"p":932,"c":"O(n)"},
  {"i":"mge.mb.tactics.v1.fn.process_retreat","k":"s","d":"massive-battle","r":["ManeuverOrder","Morale"],"w":["ManeuverOrder"],"e":["RetreatOrdered","ManeuverComplete"],"p":933,"c":"O(n)"},
  {"i":"mge.mb.tactics.v1.event.flank_detected","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.event.charge_impact","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.event.retreat_ordered","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.tactics.v1.event.maneuver_complete","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let squad = world.spawn();
world.insert(squad, TacticalStance { stance: StanceType::Aggressive, bonus_attack: 0.3, bonus_defense: -0.15 });
world.insert(squad, FlankBonus { direction: FlankDirection::None, damage_multiplier: 1.0, morale_penalty: 0.0 });
world.insert(squad, ChargeState { charging: false, charge_distance: 0.0, impact_damage: 0.0, momentum: 0.0 });
world.insert(squad, ManeuverOrder { maneuver: TacticalManeuver::HoldLine, target: None, progress: 0.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Massive Battle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
