# mge-sh-ammo

> @id mge.sh.ammo.v1  
> @role plugin  
> @domain shooter  
> @do manage_ammunition_magazines_reloading  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sh-ammo` |
| @id MSCM | `mge.sh.ammo.v1` |
| Domaine | shooter |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(n) par tick, n = entites avec Magazine |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `AmmoType` | `Standard, Piercing, Explosive, Incendiary` | Type de munitions. Influe sur les degats appliques par health |
| `ReloadPhase` | `Idle, Reloading, Chambering` | Phase du rechargement. Chambering = animation finale avant tir possible |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Magazine` | `mge.sh.ammo.v1.component.magazine` | `current: u32, capacity: u32, ammo_type: AmmoType` | Chargeur actif. current = munitions restantes |
| `AmmoReserve` | `mge.sh.ammo.v1.component.ammo_reserve` | `reserves: HashMap<AmmoType, u32>, max_per_type: u32` | Reserves de munitions par type. Alimentent le Magazine au rechargement |
| `ReloadState` | `mge.sh.ammo.v1.component.reload_state` | `phase: ReloadPhase, ticks_remaining: u32, reload_duration: u32` | Etat du rechargement. ticks_remaining decremente par tick |

---

## 4. Formules

```
Rechargement :
  ammo_to_load = min(magazine.capacity - magazine.current, ammo_reserve[magazine.ammo_type])
  ammo_reserve[type] -= ammo_to_load
  magazine.current += ammo_to_load

Consommation par tir :
  magazine.current -= 1     (par tir, pellets comptes comme 1 tir)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `consume_ammo` | `mge.sh.ammo.v1.fn.consume_ammo` | 1720 | WeaponFired (event), Magazine | Magazine | AmmoConsumed, AmmoEmpty | O(f) | Pour chaque WeaponFired, decremente magazine.current. Si current == 0, emet AmmoEmpty |
| `process_reload` | `mge.sh.ammo.v1.fn.process_reload` | 1721 | ReloadRequest (event), Magazine, AmmoReserve, ReloadState | ReloadState | ReloadStarted | O(r) | Demarre le rechargement si phase == Idle et reserve > 0. Set ticks_remaining |
| `tick_reload` | `mge.sh.ammo.v1.fn.tick_reload` | 1722 | ReloadState, Magazine, AmmoReserve | ReloadState, Magazine, AmmoReserve | ReloadCompleted | O(n) | Decremente ticks_remaining. Phase Reloading → Chambering → Idle. Transfert munitions a la fin |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AmmoConsumed` | `mge.sh.ammo.v1.event.ammo_consumed` | `entity: EntityId, remaining: u32, ammo_type: AmmoType` | `consume_ammo` | ui (affichage compteur) |
| `ReloadStarted` | `mge.sh.ammo.v1.event.reload_started` | `entity: EntityId, duration_ticks: u32` | `process_reload` | ui (barre rechargement), weapon (bloquer tir) |
| `ReloadCompleted` | `mge.sh.ammo.v1.event.reload_completed` | `entity: EntityId, loaded: u32, ammo_type: AmmoType` | `tick_reload` | ui, weapon (debloquer tir) |
| `AmmoEmpty` | `mge.sh.ammo.v1.event.ammo_empty` | `entity: EntityId, ammo_type: AmmoType` | `consume_ammo` | ui (alerte), ai (switch arme/fuite) |

---

## 7. Invariants

- `Magazine.current` ne descend jamais en dessous de 0.
- Un rechargement ne peut demarrer que si `ReloadState.phase == Idle`.
- Un rechargement est annule si l'entite meurt (composant retire).
- `AmmoReserve` ne peut pas devenir negatif.
- Le tir est bloque pendant `ReloadPhase != Idle` (verifie par weapon).
- Un rechargement avec reserve == 0 et magazine.current > 0 est refuse silencieusement.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_magazine_capacity` | `u32` | 30 | [1, 999] | Taille chargeur par defaut |
| `default_reload_duration` | `u32` | 60 | [10, 300] | Duree rechargement (ticks) |
| `chambering_ratio` | `f32` | 0.2 | [0.0, 0.5] | Part du rechargement consacree au chambering |
| `auto_reload_on_empty` | `bool` | true | {true, false} | Declenche rechargement auto quand magazine vide |
| `max_reserve_per_type` | `u32` | 999 | [1, 9999] | Maximum munitions en reserve par type |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les chargeurs et reserves | Ne gere pas le tir (→ weapon) |
| Gere le rechargement (phases, timing) | Ne gere pas la visee (→ aim) |
| Emet AmmoConsumed/AmmoEmpty | Ne gere pas les degats (→ health) |
| Supporte plusieurs types de munitions | Ne gere pas les effets des types (→ health applique) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | WeaponFired (event), ReloadRequest (event), Magazine, AmmoReserve, ReloadState |
| Ecrit | Magazine, AmmoReserve, ReloadState |
| Emet | AmmoConsumed, ReloadStarted, ReloadCompleted, AmmoEmpty |
| Ne touche jamais | Weapon, WeaponSlots, AimDirection, ShooterHealth, TargetLock |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sh-ammo/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sh.ammo.v1, trait Plugin impl
    ├── components.rs     # Magazine, AmmoReserve, ReloadState
    ├── systems.rs        # consume_ammo, process_reload, tick_reload
    └── events.rs         # AmmoConsumed, ReloadStarted, ReloadCompleted, AmmoEmpty
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
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (AmmoType, ReloadPhase)
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : consume, reload phases, empty trigger, reserve depletion
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sh.ammo.v1","k":"p","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.component.magazine","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.component.ammo_reserve","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.component.reload_state","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.fn.consume_ammo","k":"s","d":"shooter","r":["Magazine"],"w":["Magazine"],"e":["AmmoConsumed","AmmoEmpty"],"p":1720,"c":"O(f)"},
  {"i":"mge.sh.ammo.v1.fn.process_reload","k":"s","d":"shooter","r":["Magazine","AmmoReserve","ReloadState"],"w":["ReloadState"],"e":["ReloadStarted"],"p":1721,"c":"O(r)"},
  {"i":"mge.sh.ammo.v1.fn.tick_reload","k":"s","d":"shooter","r":["ReloadState","Magazine","AmmoReserve"],"w":["ReloadState","Magazine","AmmoReserve"],"e":["ReloadCompleted"],"p":1722,"c":"O(n)"},
  {"i":"mge.sh.ammo.v1.event.ammo_consumed","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.event.reload_started","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.event.reload_completed","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.ammo.v1.event.ammo_empty","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, Magazine {
    current: 30,
    capacity: 30,
    ammo_type: AmmoType::Standard,
});
world.insert(player, AmmoReserve {
    reserves: HashMap::from([(AmmoType::Standard, 120)]),
    max_per_type: 999,
});
world.insert(player, ReloadState {
    phase: ReloadPhase::Idle,
    ticks_remaining: 0,
    reload_duration: 60,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Shooter - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sh-weapon](mge-sh-weapon.md) | Plugin armes (consomme les munitions via WeaponFired) |
