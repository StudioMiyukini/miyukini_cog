# mge-pl-hazard

> @id mge.platformer.hazard.v1  
> @role plugin  
> @domain platformer  
> @do manage_hazard_zones_environmental_damage_crushers  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-pl-hazard` |
| @id MSCM | `mge.platformer.hazard.v1` |
| Domaine | platformer |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-pl-collision` |
| Hot path | Oui (detect_hazard_contact chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * h) n = entites, h = hazards |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `HazardType` | `Spikes, Lava, Pit, Projectile, Crusher` | Type de danger |
| `DamageMode` | `InstantKill, FixedDamage, PercentDamage` | Mode de degats |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `HazardZone` | `mge.platformer.hazard.v1.component.hazard_zone` | `hazard_type: HazardType, damage_mode: DamageMode, damage_value: f32` | Zone de danger. damage_value = degats fixes ou pourcentage |
| `HazardTrigger` | `mge.platformer.hazard.v1.component.hazard_trigger` | `cooldown_ticks: u32, remaining_cooldown: u32, active: bool` | Declencheur. cooldown entre deux activations. active = peut faire des degats |
| `Crusher` | `mge.platformer.hazard.v1.component.crusher` | `open_ticks: u32, close_ticks: u32, current_tick: u32, is_closing: bool` | Broyeur cyclique. Alterne ouvert/ferme |

---

## 4. Formules

```
InstantKill:
  Respawn immediatement (→ checkpoint.process_respawn)

FixedDamage:
  health -= damage_value

PercentDamage:
  health -= health.max * (damage_value / 100.0)

Crusher cycle:
  open_ticks → is_closing = false, safe
  close_ticks → is_closing = true, damage active
  current_tick cycles modulo (open_ticks + close_ticks)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `detect_hazard_contact` | `mge.platformer.hazard.v1.fn.detect_hazard_contact` | 1630 | Position, HazardZone, HazardTrigger, RespawnState | HazardTrigger | HazardContactDetected | O(n*h) | Detecte la collision entite/hazard. Ignore si invincible. Active le trigger |
| `apply_hazard_damage` | `mge.platformer.hazard.v1.fn.apply_hazard_damage` | 1631 | HazardContactDetected (event), HazardZone | Health, RespawnState | HazardDamageApplied, PlayerKilled | O(d) | Applique les degats selon DamageMode. InstantKill → PlayerKilled + respawn |
| `update_crushers` | `mge.platformer.hazard.v1.fn.update_crushers` | 1632 | Crusher | Crusher, HazardTrigger | none | O(h) | Avance le timer du broyeur. Active/desactive HazardTrigger selon phase |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `HazardContactDetected` | `mge.platformer.hazard.v1.event.hazard_contact_detected` | `entity: EntityId, hazard: EntityId, hazard_type: HazardType` | `detect_hazard_contact` | apply_hazard_damage, ui |
| `HazardDamageApplied` | `mge.platformer.hazard.v1.event.hazard_damage_applied` | `entity: EntityId, damage: f32, damage_mode: DamageMode` | `apply_hazard_damage` | ui (flash), audio, camera (shake) |
| `PlayerKilled` | `mge.platformer.hazard.v1.event.player_killed` | `entity: EntityId, hazard_type: HazardType` | `apply_hazard_damage` | checkpoint (respawn), ui (death screen), audio |

---

## 7. Invariants

- Un hazard avec `HazardTrigger.active == false` ne fait pas de degats.
- Un joueur avec `RespawnState.respawning == true` est ignore par tous les hazards.
- `HazardTrigger.remaining_cooldown` decremente de 1 par tick. A 0 → active = true.
- Un `Crusher` alterne strictement entre phase ouverte et fermee.
- `InstantKill` declenche toujours un respawn, meme si le joueur a de la vie.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `hazard_cooldown_ticks` | `u32` | 30 | [10, 120] | Cooldown entre deux contacts (evite spam) |
| `crusher_open_ticks` | `u32` | 90 | [30, 300] | Duree ouvert par defaut |
| `crusher_close_ticks` | `u32` | 30 | [10, 120] | Duree ferme par defaut |
| `pit_instant_kill` | `bool` | true | {true, false} | Les puits tuent instantanement |
| `lava_damage_per_tick` | `f32` | 5.0 | [1.0, 50.0] | Degats lave par tick |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Detecte le contact joueur/danger | Ne gere pas les collisions physiques (→ collision) |
| Applique les degats selon le mode | Ne gere pas le respawn (→ checkpoint) |
| Gere les broyeurs cycliques | Ne gere pas la vie du joueur (→ stats/health) |
| Respecte l'invincibilite post-respawn | Ne gere pas les projectiles complexes (→ shooter pack) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Position, HazardZone, HazardTrigger, Crusher, RespawnState |
| Ecrit | HazardTrigger, Crusher, Health, RespawnState |
| Emet | HazardContactDetected, HazardDamageApplied, PlayerKilled |
| Ne touche jamais | PlatformerMovement, JumpAbility, Platform, CameraTarget, Checkpoint |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-pl-hazard/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.platformer.hazard.v1
    ├── components.rs     # HazardZone, HazardTrigger, Crusher
    ├── systems.rs        # detect_hazard_contact, apply_hazard_damage, update_crushers
    └── events.rs         # HazardContactDetected, HazardDamageApplied, PlayerKilled
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (detect_hazard_contact) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (HazardType, DamageMode)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : spike contact, lava damage, pit kill, crusher cycle, invincibility bypass
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.platformer.hazard.v1","k":"p","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.hazard.v1.component.hazard_zone","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.hazard.v1.component.hazard_trigger","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.hazard.v1.component.crusher","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.hazard.v1.fn.detect_hazard_contact","k":"s","d":"platformer","r":["Position","HazardZone","HazardTrigger","RespawnState"],"w":["HazardTrigger"],"e":["HazardContactDetected"],"p":1630,"c":"O(n*h)"},
  {"i":"mge.platformer.hazard.v1.fn.apply_hazard_damage","k":"s","d":"platformer","r":["HazardZone"],"w":["Health","RespawnState"],"e":["HazardDamageApplied","PlayerKilled"],"p":1631,"c":"O(d)"},
  {"i":"mge.platformer.hazard.v1.fn.update_crushers","k":"s","d":"platformer","r":["Crusher"],"w":["Crusher","HazardTrigger"],"e":[],"p":1632,"c":"O(h)"},
  {"i":"mge.platformer.hazard.v1.event.hazard_contact_detected","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.hazard.v1.event.hazard_damage_applied","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.hazard.v1.event.player_killed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let spikes = world.spawn();
world.insert(spikes, HazardZone { hazard_type: HazardType::Spikes, damage_mode: DamageMode::InstantKill, damage_value: 0.0 });
world.insert(spikes, HazardTrigger { cooldown_ticks: 30, remaining_cooldown: 0, active: true });

let crusher = world.spawn();
world.insert(crusher, HazardZone { hazard_type: HazardType::Crusher, damage_mode: DamageMode::InstantKill, damage_value: 0.0 });
world.insert(crusher, Crusher { open_ticks: 90, close_ticks: 30, current_tick: 0, is_closing: false });
world.insert(crusher, HazardTrigger { cooldown_ticks: 0, remaining_cooldown: 0, active: false });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Platformer - Index](_index.md) | Vue d'ensemble du pack |
| [mge-pl-collision](mge-pl-collision.md) | Plugin collision (dependance) |
| [mge-pl-checkpoint](mge-pl-checkpoint.md) | Plugin checkpoint (respawn) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
