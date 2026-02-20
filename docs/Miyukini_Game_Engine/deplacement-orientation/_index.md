# Déplacement et orientation — Index documentaire

Index centralisé de la documentation MGE sur le déplacement et l'orientation des entités. Destiné aux développeurs et aux IA pour une navigation rapide et structurée.

**Statut :** Référence normative. Chemins relatifs à `docs/Miyukini_Game_Engine/`.

---

## Accès rapide par besoin

| Besoin | Document principal | Documents complémentaires |
|--------|-------------------|---------------------------|
| **Tous les paramètres d'une entité pour se déplacer** | [MGE - Paramètres déplacement entité](../MGE%20-%20Parametres%20Deplacement%20Entite.md) | — |
| **Direction d'input (8 directions)** | [deplacement-8-directions](../points/03-deplacement-locomotion/deplacement-8-directions.md) | Paramètres déplacement |
| **Accélération, friction, vitesse** | [acceleration-deceleration](../points/03-deplacement-locomotion/acceleration-deceleration.md), [vitesse-max](../points/03-deplacement-locomotion/vitesse-max.md) | Paramètres déplacement |
| **Orientation, rotation, axes** | [orientation-rotation](../points/03-deplacement-locomotion/orientation-rotation.md) | Référence Commune §2.6 |
| **Pathfinding (A*, waypoints)** | [pathfinding](../points/03-deplacement-locomotion/pathfinding.md) | Guide Entités Groupes |
| **Hitbox et collisions** | [MGE - Hitbox et collisions - Référence](../MGE%20-%20Hitbox%20et%20Collisions%20-%20Reference.md) | hitbox, collision, collision-layers |
| **Pathfinding groupes (RTS, musou)** | [MGE - Pathfinding Collisions - Guide Entités Groupes](../MGE%20-%20Pathfinding%20Collisions%20-%20Guide%20Entites%20Groupes.md) | pathfinding, comportement-foule |

---

## Arborescence des documents

```
docs/Miyukini_Game_Engine/
├── deplacement-orientation/
│   └── _index.md              ← CE FICHIER (index)
├── MGE - Parametres Deplacement Entite.md   ← Entrée principale
├── MGE - Reference Commune.md               ← Types Vec2, Rect, coordonnées, orientation §2.6
├── MGE - Hitbox et Collisions - Reference.md
├── MGE - Pathfinding Collisions - Guide Entites Groupes.md
└── points/
    ├── 02-physique-collisions/
    │   ├── hitbox.md
    │   ├── collision.md
    │   └── collision-layers.md
    └── 03-deplacement-locomotion/
        ├── deplacement-8-directions.md
        ├── acceleration-deceleration.md
        ├── vitesse-max.md
        ├── orientation-rotation.md
        ├── pathfinding.md
        ├── run-walk.md
        └── stamina.md
```

**Chemins relatifs :** Depuis `deplacement-orientation/`, utiliser `../points/03-deplacement-locomotion/xxx.md`.

---

## Chaîne de locomotion (ordre conceptuel)

```
1. Input (direction)     → deplacement-8-directions
2. Locomotion            → acceleration-deceleration + vitesse-max
3. Déplacement           → position += velocity * dt
4. Orientation           → orientation-rotation (angle, turn_rate)
5. Pathfinding (optionnel) → pathfinding (waypoints)
6. Collision (optionnel)  → hitbox, collision, collision-layers
```

---

## Paramètres obligatoires (rappel)

| Paramètre | Type | Document source |
|-----------|------|-----------------|
| position | Vec2 | Référence Commune |
| velocity | Vec2 | acceleration-deceleration |
| max_speed | f32 | vitesse-max |
| acceleration_rate | f32 | acceleration-deceleration |
| friction | f32 | acceleration-deceleration |
| stop_threshold | f32 | acceleration-deceleration |
| angle | f32 (rad) | orientation-rotation |
| turn_rate | f32 (°/s) | orientation-rotation |

---

## Formules clés

| Formule | Usage |
|---------|-------|
| `direction = input / max(||input||, ε)` | Normalisation (déplacement-8-directions) |
| `vitesse_cible = direction × max_speed` | Cible locomotion |
| `velocity *= (1 - friction × dt)` | Friction (pas d'input) |
| `velocity = clamp(velocity, max_speed)` | Limite vitesse |
| `angle = atan2(dir.y, dir.x)` | Angle depuis Vec2 |
| `direction = Vec2(cos(angle), sin(angle))` | Vec2 depuis angle |
| `direction = (waypoint - position).normalize()` | Direction pathfinding |

---

## Flux de mise à jour par frame

1. **Direction** = input OU (waypoint - position).normalize()
2. **Locomotion::update**(direction, dt) → velocity
3. velocity = **clamp**(velocity, max_speed)
4. displacement = velocity × dt
5. position += displacement (ou après résolution collision)
6. **Orientation::rotate_towards**(angle_cible, dt)
7. Si pathfinding : waypoint atteint ? → avancer index

---

## Conventions MGE

- **Axes :** X+ = Est, Y+ = Sud (écran)
- **Angle :** 0 = Est, π/2 = Sud (radians)
- **Unités :** px/s (vitesse), °/s (rotation)
- **Normalisation :** Direction toujours norme 1 (ou 0)

---

## Cursor Skill

Pour une référence condensée utilisable par les IA : [miyukini-deplacement-orientation](../../../.cursor/skills/miyukini-deplacement-orientation/SKILL.md) (Cursor Skill)

---

**Document** : Index déplacement et orientation  
**Version** : 1.0  
**Date** : 2026-02-18
