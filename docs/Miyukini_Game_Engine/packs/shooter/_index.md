# MGE — Pack Shooter

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/shooter/`  
**Nombre de crates** : 5  

---

## 1. Contexte

Le Pack Shooter fournit les mecaniques generiques des jeux de tir 2D/3D : armes, visee, munitions, ciblage automatique et sante projectile. Il est leger et composable, utilisable en standalone pour un twin-stick shooter ou en complement du Pack RPG pour un looter-shooter.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Top-down shooter, twin-stick, bullet hell, looter-shooter, third-person shooter.
- **Hors portee** : Balistique realiste 3D, destruction de terrain, vehicules armes (voir Pack Racing), systeme de couverture.
- **Audience** : Developpeurs moteur, designers, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack obligatoire (spatial, input, basic-physics).

---

## 3. Vision

Le Pack Shooter est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/shooter/
├── mge-sh-weapon/          # Armes, cadence, projectiles
├── mge-sh-aim/             # Visee, direction, spread, recul
├── mge-sh-ammo/            # Munitions, chargeurs, rechargement
├── mge-sh-target/          # Ciblage auto, lock-on
└── mge-sh-health/          # Sante projectile, bouclier, mort
```

### Graphe de dependances intra-pack

```
mge-sh-weapon ──────► mge-sh-aim
     │                     ▲
     └──────► mge-sh-ammo  │
                           │
mge-sh-target ─────────────┘

mge-sh-health (feuille — aucune dependance intra-pack)
```

Crates feuilles (sans dependance intra-pack) : `mge-sh-health`, `mge-sh-aim`.

---

## 5. Sous-packs

Aucun. Les 5 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-sh-weapon` | `mge.sh.weapon.v1` | [mge-sh-weapon.md](mge-sh-weapon.md) | Armes, cadence de tir, generation projectiles |
| 2 | `mge-sh-aim` | `mge.sh.aim.v1` | [mge-sh-aim.md](mge-sh-aim.md) | Visee, direction, spread, recul |
| 3 | `mge-sh-ammo` | `mge.sh.ammo.v1` | [mge-sh-ammo.md](mge-sh-ammo.md) | Munitions, chargeurs, rechargement |
| 4 | `mge-sh-target` | `mge.sh.target.v1` | [mge-sh-target.md](mge-sh-target.md) | Ciblage automatique, lock-on |
| 5 | `mge-sh-health` | `mge.sh.health.v1` | [mge-sh-health.md](mge-sh-health.md) | Sante, bouclier, degats projectiles, mort |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|--------------------|------------------------------|
| weapon | Weapon, WeaponSlots, FireState, ProjectileConfig | WeaponDef |
| aim | AimDirection, AimConfig, Spread, Recoil | aucun |
| ammo | Magazine, AmmoReserve, ReloadState | aucun |
| target | TargetLock, AutoAimConfig, ThreatLevel | aucun |
| health | ShooterHealth, DamageBuffer, Shield, Hitbox | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1700-1703 | weapon | process_fire_input, spawn_projectile, tick_fire_cooldown, process_weapon_switch |
| 1710-1712 | aim | update_aim_direction, apply_recoil, decay_spread |
| 1720-1722 | ammo | consume_ammo, process_reload, tick_reload |
| 1730-1733 | target | scan_targets, select_best_target, maintain_lock, break_lock |
| 1740-1743 | health | apply_projectile_damage, process_shield, check_death, tick_regen |

**Ordre d'execution** : weapon (1700) → aim (1710) → ammo (1720) → target (1730) → health (1740).

**Justification** : le tir est resolu en premier pour generer les projectiles. La visee et le recul mettent a jour la direction. Les munitions sont consommees apres le tir. Le ciblage auto prepare le prochain tick. La sante est appliquee en dernier car elle lit les projectiles du tick.

**Total** : 17 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| weapon | FireRequest (composant) | WeaponFired, ProjectileSpawned, WeaponSwitched |
| aim | (aucun, ecriture directe) | AimUpdated |
| ammo | ReloadRequest | AmmoConsumed, ReloadStarted, ReloadCompleted, AmmoEmpty |
| target | (aucun, scan automatique) | TargetAcquired, TargetLost, LockBroken |
| health | (aucun, lit projectiles) | DamageReceived, ShieldBroken, ShooterDeath, HealthRegenTick |

**Total** : 5 requests + 13 events = 18 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 5 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| `mge-sh-weapon` | `mge-plugin-spatial`, `mge-plugin-basic-physics` |
| `mge-sh-aim` | `mge-plugin-input`, `mge-plugin-spatial` |
| `mge-sh-target` | `mge-plugin-spatial` |
| `mge-sh-health` | `mge-plugin-spatial` (hitbox) |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-sh-weapon` | `mge-sh-aim`, `mge-sh-ammo` |
| `mge-sh-target` | `mge-sh-aim` |

### Dependances externes (aucune)

Le Pack Shooter n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL configure les plugins Shooter sans recompilation.

**Parametres exposables :**

- Cadence de tir, degats de base, portee
- Spread de base, facteur recul, vitesse decroissance
- Taille chargeur, temps rechargement
- Rayon scan ciblage, mode priorite
- Points de vie, regeneration, duree bouclier

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates Shooter utilises | Usage |
|----------------|------------------------|-------|
| (aucun actuellement) | — | — |

Packs pouvant s'integrer :

| Pack | Integration possible |
|------|----------------------|
| **RPG** | Stats d'attaque influencent degats arme. Health RPG remplace ShooterHealth |
| **Roguelike** | Armes generees proceduralement. Munitions limitees par run |

Le Pack Shooter ne depend d'aucun autre pack genre.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Seed RNG** | Spread utilise le RNG kernel (mge-rng) |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | weapon (tir), aim (direction), health (degats) |
| **Budget cible** | < 1ms pour 500 entites tirantes a 60 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de balistique realiste (gravite, vent) | Simplification v1, extension v2 |
| Pas de penetration multi-cibles | Simplification v1 |
| Pas de destruction environnement | Hors scope (voir Pack Sandbox) |
| Pas de recul d'arme visuel | Donnees seulement, pas de rendu |
| Pas de systeme de couverture | Hors scope v1 |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Balistique 3D | Gravite, vent, drop de balle |
| Penetration | Projectile traversant N entites avec perte de degats |
| Armes a charge | Maintenir tir pour augmenter degats |
| Ricochet | Rebond projectiles sur surfaces |
| Armes de melee | Integration stance melee dans le Pack Shooter |

---

## 17. Exemple d'assemblage

### Minimal (headless, weapon + health uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginBasicPhysics::default());
engine.add_plugin(MgeShWeaponPlugin);
engine.add_plugin(MgeShHealthPlugin);
engine.build();
```

### Complet (twin-stick shooter jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
engine.add_plugin(MgePluginBasicPhysics::default());
// Pack Shooter
engine.add_plugin(MgeShWeaponPlugin);
engine.add_plugin(MgeShAimPlugin);
engine.add_plugin(MgeShAmmoPlugin);
engine.add_plugin(MgeShTargetPlugin);
engine.add_plugin(MgeShHealthPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/shooter/
├── mge-sh-weapon/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.sh.weapon.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-sh-aim/
│   └── (meme structure)
├── mge-sh-ammo/
│   └── (meme structure)
├── mge-sh-target/
│   └── (meme structure)
└── mge-sh-health/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Shooter est la brique fondamentale des jeux de tir dans MGE. Il :

- Fournit 5 plugins couvrant armes, visee, munitions, ciblage et sante.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe, sans rendu.
- Peut se combiner avec le Pack RPG pour un looter-shooter.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 5 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
