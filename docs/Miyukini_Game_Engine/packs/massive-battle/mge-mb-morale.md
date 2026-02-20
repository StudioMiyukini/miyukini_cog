# mge-mb-morale

> @id mge.mb.morale.v1  
> @role plugin  
> @domain massive-battle  
> @do manage_morale_panic_rout_break  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-mb-morale` |
| @id MSCM | `mge.mb.morale.v1` |
| Domaine | massive-battle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-mb-unit` |
| Hot path | Oui (moral mis a jour chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) sur squads avec moral actif |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `MoraleState` | `Steady, Wavering, Panicked, Broken, Routed` | Etat moral du squad |
| `MoraleModifier` | `CasualtyNearby, LeaderDeath, FlankAttack, Victory, Reinforcement, ChargeReceived, AllyRouted` | Source de modification du moral |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Morale` | `mge.mb.morale.v1.component.morale` | `value: f32, state: MoraleState, base_value: f32` | Moral du squad. 100.0 = max, 0.0 = route |
| `PanicState` | `mge.mb.morale.v1.component.panic_state` | `panicking: bool, duration_ticks: u32, spread_radius: f32` | Etat de panique. Se propage aux squads proches |
| `RoutBehavior` | `mge.mb.morale.v1.component.rout_behavior` | `fleeing: bool, flee_direction: (f32, f32), speed_multiplier: f32` | Comportement de fuite quand le moral est Routed |
| `MoraleConfig` | `mge.mb.morale.v1.component.morale_config` | `panic_threshold: f32, rout_threshold: f32, recovery_rate: f32` | Seuils et taux configurables par squad |

---

## 4. Formules de derivation

```
morale_delta = sum(modifiers) * resistance_factor
morale_value = clamp(morale_value + morale_delta, 0.0, 100.0)

MoraleState:
  Steady    si morale >= 70.0
  Wavering  si morale >= 40.0
  Panicked  si morale >= panic_threshold (defaut 20.0)
  Broken    si morale >= rout_threshold (defaut 5.0)
  Routed    si morale < rout_threshold

Modifiers par defaut:
  CasualtyNearby  = -2.0
  LeaderDeath     = -25.0
  FlankAttack     = -15.0
  Victory         = +20.0
  Reinforcement   = +10.0
  ChargeReceived  = -10.0
  AllyRouted      = -8.0
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_morale` | `mge.mb.morale.v1.fn.update_morale` | Logic (920) | Morale, MoraleConfig | Morale | none | O(n) | Applique les modificateurs accumules et recalcule MoraleState |
| `check_panic_threshold` | `mge.mb.morale.v1.fn.check_panic_threshold` | Logic (921) | Morale, MoraleConfig | PanicState | PanicTriggered, MoraleBroken | O(n) | Declenche panique si moral < panic_threshold. Emet MoraleBroken si < rout |
| `process_rout` | `mge.mb.morale.v1.fn.process_rout` | Logic (922) | Morale, PanicState | RoutBehavior | RoutStarted | O(n) | Active la fuite pour les squads Routed. Calcule direction de fuite |
| `spread_panic` | `mge.mb.morale.v1.fn.spread_panic` | Logic (923) | PanicState, Morale | Morale | MoraleRestored | O(n²) | Propage la panique aux squads dans le rayon. Restaure si source disparue |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique | Description |
|-----------|----------|--------|----------|----------------------|-------------|
| `MoraleBroken` | `mge.mb.morale.v1.event.morale_broken` | `squad: EntityId, final_value: f32` | `check_panic_threshold` | tactics, ai | Moral brise, squad ne peut plus combattre |
| `PanicTriggered` | `mge.mb.morale.v1.event.panic_triggered` | `squad: EntityId, cause: MoraleModifier` | `check_panic_threshold` | unit, ai | Squad entre en panique |
| `RoutStarted` | `mge.mb.morale.v1.event.rout_started` | `squad: EntityId, flee_direction: (f32, f32)` | `process_rout` | formation, ai | Squad en deroute, fuit le champ de bataille |
| `MoraleRestored` | `mge.mb.morale.v1.event.morale_restored` | `squad: EntityId, new_state: MoraleState` | `spread_panic` | unit, ai | Moral remonte au-dessus du seuil de panique |

---

## 7. Invariants

- `Morale.value` est toujours dans [0.0, 100.0] apres `update_morale`.
- `MoraleState` est toujours coherent avec `Morale.value` et les seuils dans `MoraleConfig`.
- Un squad `Routed` a toujours `RoutBehavior.fleeing = true`.
- `spread_panic` ne peut pas baisser le moral d'un squad deja `Routed`.
- Un squad `Steady` n'a jamais `PanicState.panicking = true`.
- `MoraleRestored` n'est emis que si le state passe d'un etat inferieur a un etat superieur.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `panic_threshold` | `f32` | 20.0 | [5.0, 50.0] | Moral en dessous duquel le squad panique |
| `rout_threshold` | `f32` | 5.0 | [0.0, 20.0] | Moral en dessous duquel le squad route |
| `recovery_rate` | `f32` | 0.5 | [0.0, 5.0] | Points de moral recuperes par tick hors combat |
| `panic_spread_radius` | `f32` | 20.0 | [5.0, 50.0] | Rayon de propagation de la panique |
| `panic_spread_penalty` | `f32` | 5.0 | [1.0, 20.0] | Malus moral applique aux squads dans le rayon de panique |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le moral par squad | Ne gere pas le moral individuel (v2) |
| Declenche panique et route | Ne deplace pas les unites en fuite (→ spatial) |
| Propage la panique entre squads | Ne calcule pas les degats (→ rpg-combat) |
| Restaure le moral hors combat | Ne decide pas de la victoire (→ externe) |
| Calcule les seuils configrables | Ne gere pas les bonus de commandant (→ tactics) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Morale, MoraleConfig, PanicState |
| Ecrit | Morale, PanicState, RoutBehavior |
| Emet | MoraleBroken, PanicTriggered, RoutStarted, MoraleRestored |
| Ne touche jamais | Formation, Squad, TacticalStance, SupplyStock, WallSection |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-mb-morale/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.mb.morale.v1, trait Plugin impl
    ├── components.rs     # Morale, PanicState, RoutBehavior, MoraleConfig
    ├── systems.rs        # update_morale, check_panic_threshold, process_rout, spread_panic
    └── events.rs         # MoraleBroken, PanicTriggered, RoutStarted, MoraleRestored
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (update_morale, spread_panic) |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs`
- [ ] 4 evenements dans `events.rs`
- [ ] 2 enumerations (MoraleState, MoraleModifier)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : morale update, panic trigger, rout, spread, restore
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.mb.morale.v1","k":"p","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.component.morale","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.component.panic_state","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.component.rout_behavior","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.component.morale_config","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.fn.update_morale","k":"s","d":"massive-battle","r":["Morale","MoraleConfig"],"w":["Morale"],"e":[],"p":920,"c":"O(n)"},
  {"i":"mge.mb.morale.v1.fn.check_panic_threshold","k":"s","d":"massive-battle","r":["Morale","MoraleConfig"],"w":["PanicState"],"e":["PanicTriggered","MoraleBroken"],"p":921,"c":"O(n)"},
  {"i":"mge.mb.morale.v1.fn.process_rout","k":"s","d":"massive-battle","r":["Morale","PanicState"],"w":["RoutBehavior"],"e":["RoutStarted"],"p":922,"c":"O(n)"},
  {"i":"mge.mb.morale.v1.fn.spread_panic","k":"s","d":"massive-battle","r":["PanicState","Morale"],"w":["Morale"],"e":["MoraleRestored"],"p":923,"c":"O(n²)"},
  {"i":"mge.mb.morale.v1.event.morale_broken","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.event.panic_triggered","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.event.rout_started","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.morale.v1.event.morale_restored","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let squad = world.spawn();
world.insert(squad, Morale { value: 80.0, state: MoraleState::Steady, base_value: 80.0 });
world.insert(squad, PanicState { panicking: false, duration_ticks: 0, spread_radius: 20.0 });
world.insert(squad, RoutBehavior { fleeing: false, flee_direction: (0.0, 0.0), speed_multiplier: 1.5 });
world.insert(squad, MoraleConfig { panic_threshold: 20.0, rout_threshold: 5.0, recovery_rate: 0.5 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Massive Battle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
