# mge-sh-health

> @id mge.sh.health.v1  
> @role plugin  
> @domain shooter  
> @do manage_shooter_health_shield_damage_death  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sh-health` |
| @id MSCM | `mge.sh.health.v1` |
| Domaine | shooter |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial` |
| Hot path | Oui (application degats) |
| Headless safe | Oui |
| Complexite globale | O(d) par tick, d = degats en attente |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `HitZone` | `Head, Body, Limbs` | Zone touchee. Multiplicateur de degats configurable |
| `DamageSource` | `Projectile, Explosion, Environment` | Origine des degats. Pour statistiques et resistances |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ShooterHealth` | `mge.sh.health.v1.component.shooter_health` | `current: f32, max: f32, regen_rate: f32, regen_delay_ticks: u32, last_damage_tick: u32` | Sante. regen_delay = ticks apres dernier degat avant regen |
| `DamageBuffer` | `mge.sh.health.v1.component.damage_buffer` | `entries: Vec<DamageEntry>` | Buffer de degats a appliquer ce tick. DamageEntry = {amount, source, hit_zone, damage_source} |
| `Shield` | `mge.sh.health.v1.component.shield` | `current: f32, max: f32, regen_rate: f32, regen_delay_ticks: u32, absorb_ratio: f32` | Bouclier. absorb_ratio = fraction des degats absorbes (1.0 = tout) |
| `Hitbox` | `mge.sh.health.v1.component.hitbox` | `zones: Vec<HitboxZone>` | Definition des zones touchables. HitboxZone = {zone: HitZone, offset: Vec2, radius: f32, multiplier: f32} |

---

## 4. Formules

```
Degats avec bouclier :
  shield_absorb = min(damage * shield.absorb_ratio, shield.current)
  shield.current -= shield_absorb
  health_damage = damage - shield_absorb

Degats avec zone :
  effective_damage = base_damage * hitbox_zone.multiplier
  (Head = 2.0x, Body = 1.0x, Limbs = 0.75x par defaut GCL)

Regeneration (apres delay) :
  if current_tick - last_damage_tick >= regen_delay_ticks:
    health.current = min(health.current + regen_rate * dt, health.max)

Regeneration bouclier :
  if current_tick - last_damage_tick >= shield.regen_delay_ticks:
    shield.current = min(shield.current + shield.regen_rate * dt, shield.max)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_projectile_damage` | `mge.sh.health.v1.fn.apply_projectile_damage` | 1740 | DamageBuffer, Shield, ShooterHealth, Hitbox | Shield, ShooterHealth, DamageBuffer | DamageReceived | O(d) | Vide le DamageBuffer. Pour chaque entree, applique zone multiplier, shield absorb, puis reduit health |
| `process_shield` | `mge.sh.health.v1.fn.process_shield` | 1741 | Shield, ShooterHealth | Shield | ShieldBroken | O(n) | Si shield.current <= 0 et etait > 0 au tick precedent, emet ShieldBroken |
| `check_death` | `mge.sh.health.v1.fn.check_death` | 1742 | ShooterHealth | ShooterHealth | ShooterDeath | O(n) | Si health.current <= 0, emet ShooterDeath. Marque l'entite |
| `tick_regen` | `mge.sh.health.v1.fn.tick_regen` | 1743 | ShooterHealth, Shield | ShooterHealth, Shield | HealthRegenTick | O(n) | Regenere sante et bouclier si delai respecte |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `DamageReceived` | `mge.sh.health.v1.event.damage_received` | `target: EntityId, source: Option<EntityId>, amount: f32, hit_zone: HitZone, shield_absorbed: f32` | `apply_projectile_damage` | ui (hitmarker), ai (threat) |
| `ShieldBroken` | `mge.sh.health.v1.event.shield_broken` | `entity: EntityId` | `process_shield` | ui (feedback), ai (vulnerability) |
| `ShooterDeath` | `mge.sh.health.v1.event.shooter_death` | `entity: EntityId, killer: Option<EntityId>` | `check_death` | respawn, score, loot, ui |
| `HealthRegenTick` | `mge.sh.health.v1.event.health_regen_tick` | `entity: EntityId, healed: f32, new_current: f32` | `tick_regen` | ui (barre de vie) |

---

## 7. Invariants

- `ShooterHealth.current` est borne entre 0 et `ShooterHealth.max`.
- `Shield.current` est borne entre 0 et `Shield.max`.
- Le DamageBuffer est vide a la fin de `apply_projectile_damage`. Jamais reporte au tick suivant.
- Une entite morte (current <= 0) ne recoit plus de degats (DamageBuffer ignore).
- La regen ne demarre pas tant que `current_tick - last_damage_tick < regen_delay_ticks`.
- `Shield.absorb_ratio` est borne entre 0.0 et 1.0 inclus.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_max_health` | `f32` | 100.0 | [1.0, 9999.0] | Points de vie max par defaut |
| `default_regen_rate` | `f32` | 0.0 | [0.0, 50.0] | Regen/sec. 0 = pas de regen |
| `default_regen_delay` | `u32` | 180 | [0, 600] | Ticks sans degats avant regen |
| `head_multiplier` | `f32` | 2.0 | [1.0, 10.0] | Multiplicateur degats tete |
| `body_multiplier` | `f32` | 1.0 | [0.5, 5.0] | Multiplicateur degats corps |
| `limbs_multiplier` | `f32` | 0.75 | [0.1, 2.0] | Multiplicateur degats membres |
| `default_shield_max` | `f32` | 50.0 | [0.0, 9999.0] | Bouclier max par defaut. 0 = pas de bouclier |
| `shield_absorb_ratio` | `f32` | 1.0 | [0.0, 1.0] | Fraction de degats absorbes par le bouclier |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Applique les degats projectile via DamageBuffer | Ne gere pas les armes (→ weapon) |
| Gere le bouclier et ses degats | Ne gere pas les munitions (→ ammo) |
| Detecte la mort (health <= 0) | Ne gere pas le respawn (→ game logic) |
| Regenere sante et bouclier | Ne gere pas les buffs/debuffs (→ RPG stats si integre) |
| Supporte les zones de hitbox | Ne gere pas le rendu des hitbox |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | DamageBuffer, Shield, ShooterHealth, Hitbox |
| Ecrit | ShooterHealth, Shield, DamageBuffer (vidage) |
| Emet | DamageReceived, ShieldBroken, ShooterDeath, HealthRegenTick |
| Ne touche jamais | Weapon, WeaponSlots, Magazine, AimDirection, TargetLock |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sh-health/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sh.health.v1, trait Plugin impl
    ├── components.rs     # ShooterHealth, DamageBuffer, Shield, Hitbox
    ├── systems.rs        # apply_projectile_damage, process_shield, check_death, tick_regen
    └── events.rs         # DamageReceived, ShieldBroken, ShooterDeath, HealthRegenTick
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (apply_projectile_damage) |
| No allocation hot path | Obligatoire (DamageBuffer pre-alloue) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (HitZone, DamageSource)
- [ ] Formule degats avec shield et zone documentee
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : damage application, shield absorb, death check, regen delay, hitzone multiplier
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sh.health.v1","k":"p","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.component.shooter_health","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.component.damage_buffer","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.component.shield","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.component.hitbox","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.fn.apply_projectile_damage","k":"s","d":"shooter","r":["DamageBuffer","Shield","ShooterHealth","Hitbox"],"w":["Shield","ShooterHealth","DamageBuffer"],"e":["DamageReceived"],"p":1740,"c":"O(d)"},
  {"i":"mge.sh.health.v1.fn.process_shield","k":"s","d":"shooter","r":["Shield","ShooterHealth"],"w":["Shield"],"e":["ShieldBroken"],"p":1741,"c":"O(n)"},
  {"i":"mge.sh.health.v1.fn.check_death","k":"s","d":"shooter","r":["ShooterHealth"],"w":["ShooterHealth"],"e":["ShooterDeath"],"p":1742,"c":"O(n)"},
  {"i":"mge.sh.health.v1.fn.tick_regen","k":"s","d":"shooter","r":["ShooterHealth","Shield"],"w":["ShooterHealth","Shield"],"e":["HealthRegenTick"],"p":1743,"c":"O(n)"},
  {"i":"mge.sh.health.v1.event.damage_received","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.event.shield_broken","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.event.shooter_death","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.health.v1.event.health_regen_tick","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let enemy = world.spawn();
world.insert(enemy, ShooterHealth {
    current: 100.0,
    max: 100.0,
    regen_rate: 0.0,
    regen_delay_ticks: 180,
    last_damage_tick: 0,
});
world.insert(enemy, Shield {
    current: 50.0,
    max: 50.0,
    regen_rate: 5.0,
    regen_delay_ticks: 120,
    absorb_ratio: 1.0,
});
world.insert(enemy, DamageBuffer { entries: Vec::new() });
world.insert(enemy, Hitbox {
    zones: vec![
        HitboxZone { zone: HitZone::Head, offset: Vec2::new(0.0, 8.0), radius: 3.0, multiplier: 2.0 },
        HitboxZone { zone: HitZone::Body, offset: Vec2::ZERO, radius: 5.0, multiplier: 1.0 },
    ],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Shooter - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sh-weapon](mge-sh-weapon.md) | Plugin armes (genere les projectiles qui causent les degats) |
