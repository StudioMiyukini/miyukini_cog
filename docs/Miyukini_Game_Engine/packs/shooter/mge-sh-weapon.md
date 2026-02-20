# mge-sh-weapon

> @id mge.sh.weapon.v1  
> @role plugin  
> @domain shooter  
> @do manage_weapons_fire_rate_projectile_spawning  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sh-weapon` |
| @id MSCM | `mge.sh.weapon.v1` |
| Domaine | shooter |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-plugin-basic-physics`, `mge-sh-aim`, `mge-sh-ammo` |
| Hot path | Oui (tir, spawn projectile) |
| Headless safe | Oui |
| Complexite globale | O(n) par tick, n = entites armees |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `WeaponType` | `Pistol, Rifle, Shotgun, Sniper, MachineGun, Launcher` | Classe de l'arme. Influe sur le spread de base et la cadence |
| `FireMode` | `Semi, Auto, Burst` | Mode de tir. Semi = 1 tir par input, Auto = maintien, Burst = N tirs |
| `ProjectileType` | `Bullet, Pellet, Rocket, Beam` | Type de projectile genere. Pellet = multi (shotgun) |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Weapon` | `mge.sh.weapon.v1.component.weapon` | `weapon_type: WeaponType, fire_mode: FireMode, projectile_type: ProjectileType, damage_base: f32, fire_rate_ticks: u32, range: f32, pellet_count: u8` | Definition d'arme. pellet_count > 1 pour Shotgun |
| `WeaponSlots` | `mge.sh.weapon.v1.component.weapon_slots` | `weapons: Vec<Weapon>, active_index: u8` | Inventaire d'armes de l'entite. active_index = arme en cours |
| `FireState` | `mge.sh.weapon.v1.component.fire_state` | `cooldown_remaining: u32, burst_remaining: u8, is_firing: bool` | Etat de tir courant. Cooldown decremente par tick |
| `ProjectileConfig` | `mge.sh.weapon.v1.component.projectile_config` | `speed: f32, lifetime_ticks: u32, pierce_count: u8` | Parametres du projectile spawne. pierce_count = 0 pour v1 |
| `WeaponDef` | `mge.sh.weapon.v1.component.weapon_def` | `id: u32, name_hash: u64, base_weapon: Weapon, projectile: ProjectileConfig` | Definition statique d'arme (catalogue) |

---

## 4. Formules

```
Cadence effective :
  ticks_between_shots = weapon.fire_rate_ticks

Mode Burst :
  burst_delay = fire_rate_ticks / 2
  burst_total_time = (burst_count - 1) * burst_delay + fire_rate_ticks

Degats projectile :
  projectile_damage = weapon.damage_base
  (les modificateurs viennent d'autres plugins : RPG stats, buffs, etc.)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_fire_input` | `mge.sh.weapon.v1.fn.process_fire_input` | 1700 | FireState, WeaponSlots, AimDirection, Magazine | FireState | WeaponFired | O(n) | Verifie cooldown, munitions et input. Si valide, marque le tir et emet WeaponFired |
| `spawn_projectile` | `mge.sh.weapon.v1.fn.spawn_projectile` | 1701 | WeaponFired (event), WeaponSlots, ProjectileConfig, Position2D, AimDirection, Spread | World (spawn) | ProjectileSpawned | O(f) | Pour chaque WeaponFired, spawn un projectile (ou N pour pellet). Direction = aim + spread offset |
| `tick_fire_cooldown` | `mge.sh.weapon.v1.fn.tick_fire_cooldown` | 1702 | FireState | FireState | none | O(n) | Decremente cooldown_remaining. Gere burst_remaining pour le mode Burst |
| `process_weapon_switch` | `mge.sh.weapon.v1.fn.process_weapon_switch` | 1703 | WeaponSlots, InputAction | WeaponSlots, FireState | WeaponSwitched | O(n) | Change active_index si input de switch. Reset le cooldown et burst |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `WeaponFired` | `mge.sh.weapon.v1.event.weapon_fired` | `source: EntityId, weapon_type: WeaponType, damage: f32, direction: Vec2` | `process_fire_input` | `spawn_projectile`, ammo (consume), ui |
| `ProjectileSpawned` | `mge.sh.weapon.v1.event.projectile_spawned` | `projectile: EntityId, source: EntityId, damage: f32, direction: Vec2, speed: f32` | `spawn_projectile` | physics, health, ui |
| `WeaponSwitched` | `mge.sh.weapon.v1.event.weapon_switched` | `entity: EntityId, from_index: u8, to_index: u8, weapon_type: WeaponType` | `process_weapon_switch` | ui, ammo (affichage) |

---

## 7. Invariants

- Un tir ne peut se produire que si `cooldown_remaining == 0` et `Magazine.current > 0`.
- En mode Semi, `is_firing` est reset a false apres chaque tir.
- En mode Burst, `burst_remaining` est decremente par tir. A 0, cooldown complet s'applique.
- Un projectile spawne herite de `Position2D` et `AimDirection` du tireur au moment du tir.
- Le switch d'arme reset `FireState` completement (pas de carry-over cooldown).
- `pellet_count` tirs sont generes pour un seul WeaponFired si `projectile_type == Pellet`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_fire_rate_ticks` | `u32` | 6 | [1, 120] | Cadence par defaut (ticks entre tirs) |
| `burst_count` | `u8` | 3 | [2, 10] | Nombre de tirs par burst |
| `projectile_base_speed` | `f32` | 800.0 | [100.0, 5000.0] | Vitesse de base projectile (units/sec) |
| `projectile_lifetime_ticks` | `u32` | 180 | [10, 600] | Duree de vie max projectile |
| `pellet_spread_angle` | `f32` | 15.0 | [1.0, 45.0] | Angle spread pour pellets (degres) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le tir (cooldown, cadence, mode) | Ne gere pas la visee (→ aim) |
| Spawne les projectiles | Ne gere pas les munitions (→ ammo) |
| Gere le switch d'armes | Ne gere pas les degats sur cible (→ health) |
| Definit les types d'armes | Ne gere pas le ciblage auto (→ target) |
| Emet WeaponFired/ProjectileSpawned | Ne gere pas le rendu des projectiles |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | FireState, WeaponSlots, AimDirection, Magazine, Position2D, Spread, InputAction, ProjectileConfig |
| Ecrit | FireState, WeaponSlots (active_index), World (spawn projectiles) |
| Emet | WeaponFired, ProjectileSpawned, WeaponSwitched |
| Ne touche jamais | ShooterHealth, Shield, TargetLock, AmmoReserve, ReloadState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sh-weapon/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sh.weapon.v1, trait Plugin impl
    ├── components.rs     # Weapon, WeaponSlots, FireState, ProjectileConfig, WeaponDef
    ├── systems.rs        # process_fire_input, spawn_projectile, tick_fire_cooldown, process_weapon_switch
    └── events.rs         # WeaponFired, ProjectileSpawned, WeaponSwitched
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire (spawn_projectile pre-alloue) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 5 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (WeaponType, FireMode, ProjectileType)
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : fire cooldown, burst mode, weapon switch, pellet spawn
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sh.weapon.v1","k":"p","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.component.weapon","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.component.weapon_slots","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.component.fire_state","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.component.projectile_config","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.component.weapon_def","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.fn.process_fire_input","k":"s","d":"shooter","r":["FireState","WeaponSlots","AimDirection","Magazine"],"w":["FireState"],"e":["WeaponFired"],"p":1700,"c":"O(n)"},
  {"i":"mge.sh.weapon.v1.fn.spawn_projectile","k":"s","d":"shooter","r":["WeaponSlots","ProjectileConfig","Position2D","AimDirection","Spread"],"w":["World"],"e":["ProjectileSpawned"],"p":1701,"c":"O(f)"},
  {"i":"mge.sh.weapon.v1.fn.tick_fire_cooldown","k":"s","d":"shooter","r":["FireState"],"w":["FireState"],"e":[],"p":1702,"c":"O(n)"},
  {"i":"mge.sh.weapon.v1.fn.process_weapon_switch","k":"s","d":"shooter","r":["WeaponSlots","InputAction"],"w":["WeaponSlots","FireState"],"e":["WeaponSwitched"],"p":1703,"c":"O(n)"},
  {"i":"mge.sh.weapon.v1.event.weapon_fired","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.event.projectile_spawned","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.weapon.v1.event.weapon_switched","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, WeaponSlots {
    weapons: vec![Weapon {
        weapon_type: WeaponType::Rifle,
        fire_mode: FireMode::Auto,
        projectile_type: ProjectileType::Bullet,
        damage_base: 15.0,
        fire_rate_ticks: 4,
        range: 500.0,
        pellet_count: 1,
    }],
    active_index: 0,
});
world.insert(player, FireState {
    cooldown_remaining: 0,
    burst_remaining: 0,
    is_firing: false,
});
world.insert(player, ProjectileConfig {
    speed: 800.0,
    lifetime_ticks: 180,
    pierce_count: 0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Shooter - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sh-aim](mge-sh-aim.md) | Plugin visee (dependance) |
| [mge-sh-ammo](mge-sh-ammo.md) | Plugin munitions (dependance) |
