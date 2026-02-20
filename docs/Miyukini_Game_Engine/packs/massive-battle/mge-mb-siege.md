# mge-mb-siege

> @id mge.mb.siege.v1  
> @role plugin  
> @domain massive-battle  
> @do manage_siege_warfare_walls_engines_assaults  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-mb-siege` |
| @id MSCM | `mge.mb.siege.v1` |
| Domaine | massive-battle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-mb-unit`, `mge-mb-supply`, `mge-plugin-basic-physics` |
| Hot path | Non (ticks strategiques, pas chaque frame) |
| Headless safe | Oui |
| Complexite globale | O(w + e) ou w=sections de mur, e=engins de siege |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `SiegeRole` | `Attacker, Defender` | Role d'un participant au siege |
| `WallState` | `Intact, Damaged, Breached, Destroyed` | Etat structurel d'une section de mur |
| `GateCondition` | `Closed, Open, Barred, Destroyed` | Etat d'une porte de fortification |
| `EngineType` | `Ram, Trebuchet, Catapult, SiegeTower, Ladder` | Type d'engin de siege |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `SiegeParticipant` | `mge.mb.siege.v1.component.siege_participant` | `role: SiegeRole, siege_id: EntityId, assigned_target: Option<EntityId>` | Marque une entite comme participant a un siege avec son role |
| `WallSection` | `mge.mb.siege.v1.component.wall_section` | `hp: f32, max_hp: f32, state: WallState, armor: f32, position_index: u16` | Section de mur avec points de vie et resistance |
| `SiegeEngine` | `mge.mb.siege.v1.component.siege_engine` | `engine_type: EngineType, damage: f32, rate: f32, range: f32, hp: f32, crew_required: u8, crew_current: u8` | Engin de siege avec ses statistiques d'attaque |
| `GateState` | `mge.mb.siege.v1.component.gate_state` | `condition: GateCondition, hp: f32, max_hp: f32, ram_resistance: f32` | Porte de fortification avec resistance au belier |

---

## 4. Formules

```
wall_damage_effective = engine.damage - wall.armor
wall_damage_effective = max(wall_damage_effective, 0.0)

wall.hp = wall.hp - wall_damage_effective * engine.rate * dt

wall.state =
  if wall.hp <= 0.0           => Destroyed
  if wall.hp < max_hp * 0.25  => Breached
  if wall.hp < max_hp * 0.60  => Damaged
  else                        => Intact

gate_ram_damage = ram.damage * (1.0 - gate.ram_resistance)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_siege_engines` | `mge.mb.siege.v1.fn.update_siege_engines` | Logic (950) | SiegeEngine, SiegeParticipant | SiegeEngine | none | O(e) | Met a jour le statut des engins. Verifie equipage minimum |
| `apply_wall_damage` | `mge.mb.siege.v1.fn.apply_wall_damage` | Logic (951) | SiegeEngine, WallSection | WallSection | none | O(e * w) | Applique les degats des engins aux sections de mur ciblees |
| `check_breach` | `mge.mb.siege.v1.fn.check_breach` | Logic (952) | WallSection | WallSection | WallBreached | O(w) | Detecte les breches. Emet WallBreached quand une section passe en Breached/Destroyed |
| `process_assault` | `mge.mb.siege.v1.fn.process_assault` | Logic (953) | SiegeParticipant, WallSection | SiegeParticipant | AssaultLaunched, SiegeLifted | O(n) | Traite les ordres d'assaut. Verifie conditions de breche. Emet SiegeLifted si tous murs detruits |
| `update_gate` | `mge.mb.siege.v1.fn.update_gate` | Logic (954) | GateState, SiegeEngine | GateState | GateDestroyed | O(g) | Applique degats beliers aux portes. Emet GateDestroyed si hp <= 0 |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `WallBreached` | `mge.mb.siege.v1.event.wall_breached` | `wall: EntityId, section_index: u16, siege_id: EntityId` | `check_breach` | tactics, ai, morale |
| `GateDestroyed` | `mge.mb.siege.v1.event.gate_destroyed` | `gate: EntityId, siege_id: EntityId` | `update_gate` | tactics, ai, unit |
| `SiegeEngineDestroyed` | `mge.mb.siege.v1.event.siege_engine_destroyed` | `engine: EntityId, engine_type: EngineType, siege_id: EntityId` | `apply_wall_damage` (retour feu) | supply, ai |
| `AssaultLaunched` | `mge.mb.siege.v1.event.assault_launched` | `siege_id: EntityId, breach_target: EntityId` | `process_assault` | unit, tactics, morale |
| `SiegeLifted` | `mge.mb.siege.v1.event.siege_lifted` | `siege_id: EntityId, reason: String` | `process_assault` | ai, morale |

---

## 7. Invariants

- Un `SiegeParticipant` appartient toujours a un siege actif (`siege_id` valide).
- `WallSection.hp` est toujours dans [0, max_hp] apres `apply_wall_damage`.
- `WallSection.state` est toujours coherent avec `hp` apres `check_breach`.
- Un `SiegeEngine` avec `crew_current < crew_required` ne tire pas (ignore par `apply_wall_damage`).
- `GateState.condition = Destroyed` implique `hp <= 0`. Irreversible sans systeme de reparation externe.
- `SiegeLifted` n'est emis qu'une seule fois par siege.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `wall_breach_threshold` | `f32` | 0.25 | [0.05, 0.5] | Ratio hp/max_hp declenchant l'etat Breached |
| `wall_damaged_threshold` | `f32` | 0.60 | [0.3, 0.9] | Ratio hp/max_hp declenchant l'etat Damaged |
| `ram_base_damage` | `f32` | 15.0 | [1.0, 100.0] | Degats de base du belier par tick |
| `siege_lift_check_interval` | `u32` | 10 | [1, 60] | Ticks entre chaque verification de levee de siege |
| `max_engines_per_siege` | `u32` | 12 | [1, 50] | Nombre max d'engins par siege |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les murs, portes et engins de siege | Ne gere pas les formations (-> formation) |
| Applique les degats aux fortifications | Ne gere pas le moral des assiegos (-> morale) |
| Detecte les breches et leve le siege | Ne gere pas le pathfinding des assaillants (-> spatial) |
| Suit l'equipage des engins | Ne gere pas le ravitaillement (-> supply) |
| Emet les evenements de siege | Ne gere pas le combat corps a corps (-> rpg-combat) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | SiegeEngine, WallSection, GateState, SiegeParticipant |
| Ecrit | SiegeEngine, WallSection, GateState, SiegeParticipant |
| Emet | WallBreached, GateDestroyed, SiegeEngineDestroyed, AssaultLaunched, SiegeLifted |
| Ne touche jamais | Formation, Morale, TacticalStance, SupplyStock, Squad |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-mb-siege/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.mb.siege.v1, trait Plugin impl
    ├── components.rs     # SiegeParticipant, WallSection, SiegeEngine, GateState
    ├── systems.rs        # update_siege_engines, apply_wall_damage, check_breach, process_assault, update_gate
    └── events.rs         # WallBreached, GateDestroyed, SiegeEngineDestroyed, AssaultLaunched, SiegeLifted
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire (pas de static mut, lazy_static, thread_local) |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 5 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 5 evenements dans `events.rs` avec @id et @fields
- [ ] 4 enumerations (SiegeRole, WallState, GateCondition, EngineType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : wall damage, breach detection, gate destruction, assault, siege lift
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.mb.siege.v1","k":"p","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.component.siege_participant","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.component.wall_section","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.component.siege_engine","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.component.gate_state","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.fn.update_siege_engines","k":"s","d":"massive-battle","r":["SiegeEngine","SiegeParticipant"],"w":["SiegeEngine"],"e":[],"p":950,"c":"O(e)"},
  {"i":"mge.mb.siege.v1.fn.apply_wall_damage","k":"s","d":"massive-battle","r":["SiegeEngine","WallSection"],"w":["WallSection"],"e":[],"p":951,"c":"O(e*w)"},
  {"i":"mge.mb.siege.v1.fn.check_breach","k":"s","d":"massive-battle","r":["WallSection"],"w":["WallSection"],"e":["WallBreached"],"p":952,"c":"O(w)"},
  {"i":"mge.mb.siege.v1.fn.process_assault","k":"s","d":"massive-battle","r":["SiegeParticipant","WallSection"],"w":["SiegeParticipant"],"e":["AssaultLaunched","SiegeLifted"],"p":953,"c":"O(n)"},
  {"i":"mge.mb.siege.v1.fn.update_gate","k":"s","d":"massive-battle","r":["GateState","SiegeEngine"],"w":["GateState"],"e":["GateDestroyed"],"p":954,"c":"O(g)"},
  {"i":"mge.mb.siege.v1.event.wall_breached","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.event.gate_destroyed","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.event.siege_engine_destroyed","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.event.assault_launched","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.siege.v1.event.siege_lifted","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let siege = world.spawn();

let wall = world.spawn();
world.insert(wall, WallSection {
    hp: 500.0,
    max_hp: 500.0,
    state: WallState::Intact,
    armor: 5.0,
    position_index: 0,
});

let gate = world.spawn();
world.insert(gate, GateState {
    condition: GateCondition::Barred,
    hp: 300.0,
    max_hp: 300.0,
    ram_resistance: 0.3,
});

let trebuchet = world.spawn();
world.insert(trebuchet, SiegeEngine {
    engine_type: EngineType::Trebuchet,
    damage: 40.0,
    rate: 0.2,
    range: 80.0,
    hp: 100.0,
    crew_required: 4,
    crew_current: 4,
});
world.insert(trebuchet, SiegeParticipant {
    role: SiegeRole::Attacker,
    siege_id: siege,
    assigned_target: Some(wall),
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Massive Battle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
