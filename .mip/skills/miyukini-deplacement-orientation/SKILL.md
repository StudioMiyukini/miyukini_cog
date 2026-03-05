---
name: miyukini-deplacement-orientation
description: Deplacement et orientation des entites MGE. Parametres obligatoires (position, velocity, locomotion, orientation), chaine locomotion (input -> accel/friction -> clamp -> displacement -> rotation), pathfinding (waypoints, A*), hitbox et collision. Utiliser quand on implemente le deplacement d'une entite, le pathfinding, l'orientation PNJ, la locomotion (acceleration, friction, max_speed), ou quand on cherche les parametres requis pour qu'une entite puisse se deplacer.
---

# Deplacement et orientation — MGE (Miyukini Game Engine)

## Quand utiliser ce skill

- Implementation du deplacement d'une entite (joueur, PNJ)
- Recherche des parametres requis pour qu'une entite puisse se deplacer
- Pathfinding (A*, waypoints, click-to-move)
- Orientation et rotation (facing, turn_rate)
- Locomotion (acceleration, friction, vitesse max)
- Hitbox et collision pour le deplacement

## Index documentation

**Entree principale :** `docs/Miyukini_Game_Engine/deplacement-orientation/_index.md`

**Document de reference :** `docs/Miyukini_Game_Engine/MGE - Parametres Deplacement Entite.md`

---

## Parametres obligatoires (entite pouvant se deplacer)

| Parametre | Type | Unite | Valeur typique |
|-----------|------|-------|----------------|
| position | Vec2 | px | — |
| velocity | Vec2 | px/s | (0,0) a l'arret |
| max_speed | f32 | px/s | 80-150 |
| acceleration_rate | f32 | /s | 5-20 |
| friction | f32 | /s | 4-12 |
| stop_threshold | f32 | — | 0.01 |
| angle | f32 | rad [-pi,pi] | — |
| turn_rate | f32 | °/s | 180-720 |

## Parametres optionnels

| Contexte | Parametres |
|----------|------------|
| Pathfinding | waypoints, waypoint_index |
| Collision | hitbox (AABB/cercle), collision_layers (layer, mask) |
| Run/walk | is_running, run_speed_multiplier |
| Stamina | stamina_current, stamina_max |

---

## Chaine de locomotion (ordre update)

```
1. direction = input OU (waypoint - position).normalize()
2. Locomotion::update(direction, dt) -> velocity
3. velocity = clamp(velocity, max_speed)
4. displacement = velocity * dt
5. position += displacement
6. Orientation::rotate_towards(angle_cible, dt)
7. Si pathfinding: waypoint atteint -> waypoint_index++
```

---

## Formules cles

**Normalisation direction :** `direction = input / max(||input||, epsilon)`

**Vitesse cible :** `vitesse_cible = direction * max_speed`

**Friction (pas d'input) :** `velocity *= (1 - friction * dt)`

**Clamp vitesse :** `velocity = velocity.clamp_length_max(max_speed)`

**Angle depuis Vec2 :** `angle = atan2(dir.y, dir.x)` — convention 0=Est, pi/2=Sud

**Vec2 depuis angle :** `Vec2::new(angle.cos(), angle.sin())`

**Direction pathfinding :** `(waypoint - position).normalize()`

---

## Conventions MGE

- **Axes :** X+ = Est (droite), Y+ = Sud (bas)
- **Angle :** 0 rad = Est, pi/2 = Sud
- **8 directions :** N, S, E, W, NE, NW, SE, SW
- **Unites :** px/s (vitesse), °/s (rotation)

---

## Structures Rust (extrait)

```rust
pub struct LocomotionParams {
    pub max_speed: f32,
    pub acceleration_rate: f32,
    pub friction: f32,
    pub stop_threshold: f32,
}

pub struct Orientation {
    pub angle: f32,
    pub turn_rate: f32,
    pub mode: OrientationMode, // Interpolated | Instant
}

// Update locomotion
fn update(direction: Vec2, dt: f32) {
    if direction.length_squared() < 1e-12 {
        velocity *= (1.0 - friction * dt).min(1.0);
        if velocity.length() < stop_threshold { velocity = Vec2::ZERO; }
    } else {
        let target = direction.normalize() * max_speed;
        velocity += (target - velocity) * (acceleration_rate * dt).min(1.0);
        velocity = velocity.clamp_length_max(max_speed);
    }
}

// Orientation
fn rotate_towards(&mut self, target_angle: f32, dt: f32) {
    let diff = shortest_angle_diff(self.angle, target_angle);
    let delta = diff.clamp(-turn_rate.to_radians() * dt, turn_rate.to_radians() * dt);
    self.angle += delta;
}
```

---

## Priorite orientation PNJ

1. Attaque en cours -> direction du coup
2. Cible verrouillee -> vers la cible
3. Mouvement actif -> direction de velocity
4. Waypoint -> vers waypoint[0]
5. Idle -> garder derniere orientation

---

## Vitesses rotation typiques

| Type | turn_rate (°/s) |
|------|-----------------|
| Joueur | 360-720 |
| PNJ | 180-360 |
| Boss/tank | 90-180 |
| Bateau | 45-90 |

---

## References detaillees

| Document | Chemin |
|----------|--------|
| Index deplacement/orientation | docs/Miyukini_Game_Engine/deplacement-orientation/_index.md |
| Parametres deplacement entite | docs/Miyukini_Game_Engine/MGE - Parametres Deplacement Entite.md |
| Deplacement 8 directions | docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/deplacement-8-directions.md |
| Acceleration-deceleration | docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/acceleration-deceleration.md |
| Vitesse max | docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/vitesse-max.md |
| Orientation rotation | docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/orientation-rotation.md |
| Pathfinding | docs/Miyukini_Game_Engine/points/03-deplacement-locomotion/pathfinding.md |
| Reference Commune | docs/Miyukini_Game_Engine/MGE - Reference Commune.md |
| Hitbox collisions | docs/Miyukini_Game_Engine/MGE - Hitbox et Collisions - Reference.md |
| Guide groupes (RTS, musou) | docs/Miyukini_Game_Engine/MGE - Pathfinding Collisions - Guide Entites Groupes.md |
