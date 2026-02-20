# MGE — Paramètres de déplacement d'une entité

Référence exhaustive des paramètres qu'une entité doit posséder pour pouvoir se déplacer dans le MGE.

**Index documentaire :** [deplacement-orientation/_index.md](deplacement-orientation/_index.md) — index structuré et pratique pour les IA. **Cursor Skill :** [miyukini-deplacement-orientation](../../.cursor/skills/miyukini-deplacement-orientation/SKILL.md).

Regroupe les spécifications des points [déplacement-8-directions](points/03-deplacement-locomotion/deplacement-8-directions.md), [accélération-décélération](points/03-deplacement-locomotion/acceleration-deceleration.md), [vitesse-max](points/03-deplacement-locomotion/vitesse-max.md), [orientation-rotation](points/03-deplacement-locomotion/orientation-rotation.md), [pathfinding](points/03-deplacement-locomotion/pathfinding.md), [hitbox](points/02-physique-collisions/hitbox.md) et [collision-layers](points/02-physique-collisions/collision-layers.md).

---

## Vue d'ensemble

| Catégorie | Obligatoire | Description |
|-----------|-------------|-------------|
| **Position et transform** | Oui | Où est l'entité, taille |
| **Vitesse** | Oui | État actuel du mouvement |
| **Locomotion** | Oui | Paramètres accélération, friction, max speed |
| **Orientation** | Oui | Direction face, vitesse rotation |
| **Hitbox** | Si collision | Forme pour blocage et détection |
| **Collision layers** | Si collision | Qui collisionne avec qui |
| **Pathfinding** | Si déplacement autonome | Waypoints, config A* |

---

# 1. Position et transform (obligatoire)

## 1.1 Position

| Paramètre | Type | Unité | Description |
|-----------|------|-------|-------------|
| `position` | `Vec2` | px ou tiles | Position dans le monde (système de coordonnées MGE) |
| `scale` | `Vec2` | facteur | Échelle de l'entité (défaut 1, 1) — affecte hitbox si scale_with_entity |

**Référence :** [MGE - Référence Commune](MGE%20-%20Reference%20Commune.md) — `Vec2`, coordonnées monde.

---

# 2. Vitesse (état, obligatoire)

| Paramètre | Type | Unité | Description |
|-----------|------|-------|-------------|
| `velocity` | `Vec2` | px/s (ou tiles/s) | Vecteur vitesse actuel ; (0, 0) = à l'arrêt |

**Mise à jour :** Calculée par le système de locomotion à partir de la direction d'input et des paramètres d'accélération/friction. Le déplacement appliqué = `velocity * dt`.

---

# 3. Locomotion (paramètres obligatoires)

## 3.1 LocomotionParams

| Paramètre | Type | Valeur typique | Description |
|-----------|------|----------------|-------------|
| `max_speed` | `f32` | 80–150 px/s | Vitesse maximale (norme) |
| `acceleration_rate` | `f32` | 5–20 /s | Vitesse de convergence vers la cible |
| `friction` | `f32` | 4–12 /s | Décélération quand input = 0 |
| `stop_threshold` | `f32` | 0.01 | Seuil sous lequel velocity = 0 |

**Formule accélération :** `vitesse_cible = direction × max_speed` ; `velocity` tend vers cette cible.

**Formule friction :** `velocity *= (1 - friction * dt)` quand pas d'input.

**Clamp :** `velocity` est clampée à `max_speed` (norme).

**Référence :** [accélération-décélération](points/03-deplacement-locomotion/acceleration-deceleration.md), [vitesse-max](points/03-deplacement-locomotion/vitesse-max.md).

## 3.2 Modificateurs (optionnel)

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `speed_multiplier` | `f32` | 1.0 | Buffs (course), debuffs (slow) |

`vitesse_effective_max = max_speed × speed_multiplier`

---

# 4. Orientation (obligatoire pour rendu directionnel)

## 4.1 Orientation

| Paramètre | Type | Valeur typique | Description |
|-----------|------|----------------|-------------|
| `angle` | `f32` | radians [-π, π] | Direction dans laquelle l'entité « regarde » |
| `turn_rate` | `f32` | 180–720 °/s | Vitesse de rotation vers la cible |
| `mode` | `OrientationMode` | Interpolated | Interpolé ou instantané |

**Conversion :** `direction = Vec2(cos(angle), sin(angle))` ; `angle = atan2(dir.y, dir.x)`.

**Convention :** 0 = Est, π/2 = Sud.

**Référence :** [orientation-rotation](points/03-deplacement-locomotion/orientation-rotation.md).

---

# 5. Direction d'input (source du mouvement)

L'entité reçoit une **direction** à chaque frame. Source selon le type :

| Type d'entité | Source | Format |
|---------------|--------|--------|
| Joueur | Clavier (ZQSD), manette | `Vec2` normalisé |
| PNJ pathfinding | Vers waypoint[0] | `(waypoint - position).normalize()` |
| PNJ scripté | Script IA | `Vec2` |
| Entité poussée | Knockback, explosion | `Vec2` (impulsion) |

**Normalisation :** Direction doit être de norme 1 (ou 0 pour arrêt). Voir [déplacement-8-directions](points/03-deplacement-locomotion/deplacement-8-directions.md).

---

# 6. Pathfinding (si déplacement autonome)

## 6.1 État du chemin

| Paramètre | Type | Description |
|-----------|------|-------------|
| `waypoints` | `Vec<Vec2>` | Liste des points à atteindre |
| `waypoint_index` | `usize` | Index du waypoint courant |
| `path_recalc_timer` | `f32` | Timer avant recalcul (0.5–2 s) |

## 6.2 Configuration pathfinding (optionnel)

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `allow_diagonal` | `bool` | true | Mouvement 8 directions |
| `diagonal_cost` | `f32` | 1.414 | Coût √2 pour diagonales |
| `max_iterations` | `usize` | 10000 | Limite A* |

**Référence :** [pathfinding](points/03-deplacement-locomotion/pathfinding.md).

---

# 7. Hitbox (obligatoire si collision physique)

## 7.1 Hitbox

| Paramètre | Type | Description |
|-----------|------|-------------|
| `shape` | `HitboxShape` | AABB ou Cercle |
| `usage` | `HitboxUsage` | Collision (blocage) ou Hit (dégâts) |
| `scale_with_entity` | `bool` | true = hitbox suit le scale |

**AABB :** `offset: Vec2`, `size: Vec2` (relatif à l'ancre).

**Cercle :** `center_offset: Vec2`, `radius: f32`.

**Référence :** [hitbox](points/02-physique-collisions/hitbox.md).

---

# 8. Collision layers (obligatoire si collision)

## 8.1 CollisionLayers

| Paramètre | Type | Description |
|-----------|------|-------------|
| `layer` | `LayerId` | Couche de l'entité (Player, Enemy, Terrain, etc.) |
| `mask` | `LayerMask` | Layers avec lesquels elle peut collisionner |

Sans hitbox ou avec mask vide : pas de blocage physique (entité traversable).

**Référence :** [collision-layers](points/02-physique-collisions/collision-layers.md).

---

# 9. Paramètres optionnels (run/walk, stamina)

## 9.1 Run / walk (optionnel)

| Paramètre | Type | Description |
|-----------|------|-------------|
| `is_running` | `bool` | true = mode course |
| `run_speed_multiplier` | `f32` | ex. 1.5 pour course |

**Effet :** `max_speed_effective = max_speed × (run_speed_multiplier si running, sinon 1)`

**Référence :** [run-walk](points/03-deplacement-locomotion/run-walk.md).

## 9.2 Stamina (optionnel)

| Paramètre | Type | Description |
|-----------|------|-------------|
| `stamina_current` | `f32` | Jauge actuelle |
| `stamina_max` | `f32` | Capacité max |
| `stamina_drain_run` | `f32` | Coût par seconde en course |

Si stamina = 0, forcer `is_running = false`.

**Référence :** [stamina](points/03-deplacement-locomotion/stamina.md).

---

# 10. Structure agrégée (proposition Rust)

```rust
/// Composants requis pour qu'une entité puisse se déplacer
pub struct MoveableEntity {
    // 1. Position et transform
    pub position: Vec2,
    pub scale: Vec2,

    // 2. Vitesse (état)
    pub velocity: Vec2,

    // 3. Locomotion
    pub locomotion: LocomotionParams,

    // 4. Orientation
    pub orientation: Orientation,

    // 5. Pathfinding (si autonome)
    pub waypoints: Vec<Vec2>,
    pub waypoint_index: usize,

    // 6. Physique (si collision)
    pub hitbox: Option<Hitbox>,
    pub collision_layers: Option<CollisionLayers>,

    // 7. Optionnels
    pub is_running: bool,
    pub stamina: Option<Stamina>,
}

/// Minimum viable pour déplacement sans collision
pub struct MinimalMoveable {
    pub position: Vec2,
    pub velocity: Vec2,
    pub locomotion: LocomotionParams,
    pub orientation: Orientation,
}
```

---

# 11. Flux de mise à jour par frame

```
1. Direction = input (clavier) OU (waypoint - position).normalize()
2. Locomotion::update(direction, dt) → velocity
3. velocity = clamp(velocity, max_speed)
4. displacement = velocity * dt
5. position += displacement (ou appliquer collisions avant)
6. Orientation::rotate_towards(angle_from(velocity ou waypoint), dt)
7. Si pathfinding : vérifier waypoint atteint, avancer index
```

---

# 12. Tableau récapitulatif

| Paramètre | Obligatoire | Type | Point de référence |
|-----------|-------------|------|-------------------|
| position | Oui | Vec2 | Référence Commune |
| scale | Oui (défaut 1,1) | Vec2 | Référence Commune |
| velocity | Oui | Vec2 | accélération-décélération |
| max_speed | Oui | f32 | vitesse-max |
| acceleration_rate | Oui | f32 | accélération-décélération |
| friction | Oui | f32 | accélération-décélération |
| stop_threshold | Oui | f32 | accélération-décélération |
| angle | Oui | f32 | orientation-rotation |
| turn_rate | Oui | f32 | orientation-rotation |
| waypoints | Si pathfinding | Vec<Vec2> | pathfinding |
| waypoint_index | Si pathfinding | usize | pathfinding |
| hitbox | Si collision | Hitbox | hitbox |
| layer | Si collision | LayerId | collision-layers |
| mask | Si collision | LayerMask | collision-layers |
| is_running | Optionnel | bool | run-walk |
| stamina | Optionnel | Stamina | stamina |

---

# 13. Références

| Document | Rôle |
|----------|------|
| [déplacement-8-directions](points/03-deplacement-locomotion/deplacement-8-directions.md) | Direction input |
| [accélération-décélération](points/03-deplacement-locomotion/acceleration-deceleration.md) | LocomotionParams |
| [vitesse-max](points/03-deplacement-locomotion/vitesse-max.md) | max_speed, clamp |
| [orientation-rotation](points/03-deplacement-locomotion/orientation-rotation.md) | Orientation |
| [pathfinding](points/03-deplacement-locomotion/pathfinding.md) | Waypoints |
| [hitbox](points/02-physique-collisions/hitbox.md) | Forme collision |
| [collision-layers](points/02-physique-collisions/collision-layers.md) | Layer, mask |
| [MGE - Référence Commune](MGE%20-%20Reference%20Commune.md) | Vec2, types de base |

---

**Document** : MGE — Paramètres déplacement entité  
**Version** : 1.0  
**Date** : 2026-02-18
