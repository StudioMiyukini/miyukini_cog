# Projectiles

**Catégorie :** 07. Combat  
**Description :** Création ; trajectoire ; collision ; dégâts ; durée de vie.

## Contexte

Les projectiles sont des entités qui se déplacent dans le monde et appliquent des effets (dégâts, CC) à l'impact. Lié aux [zones d'effet](zone-effet-aoe.md) et à la [collision](../02-physique-collisions/collision.md).

**Rôle :** Attaques à distance, sorts directionnels. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Trajectoire

| Type | Description |
|------|-------------|
| Ligne droite | Cible ou direction |
| Arc | Parabolique (gravité) |
| Homing | Suit la cible |

### Collision

- Premier contact : cible unique ou multiple
- Zone d'explosion à l'impact (AOE)

### Durée de vie

- Max distance ou max temps
- Désactivation si hors écran (culling)

---

## Modèle de données / API

```rust
pub struct Projectile {
    pub id: EntityId,
    pub trajectory: TrajectoryType,
    pub speed: f32,
    pub max_lifetime_ms: u32,
    pub on_hit: ProjectileEffect,
}
```

---

## Références

- [Index 07](_index.md)
- [Collision](../02-physique-collisions/collision.md)
- [Zone effet AOE](zone-effet-aoe.md)
