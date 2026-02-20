# mge-gfp-bounce

> @id mge.foundation.bounce.v1  
> @role plugin  
> @domain foundation  
> @do bounce_on_collision_restitution  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-bounce` |
| @id MSCM | `mge.foundation.bounce.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity`, `mge-gfp-collision-detection` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(c) c = contacts de collision avec entites rebondissantes |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Bounciness` | `mge.foundation.bounce.v1.component.bounciness` | `restitution: f32, min_velocity: f32` | Parametres de rebond. `restitution` = facteur de reflexion (0=aucun, 1=parfait). `min_velocity` = seuil sous lequel le rebond est annule |
| `BounceState` | `mge.foundation.bounce.v1.component.bounce_state` | `bounced_this_frame: bool` | Etat de rebond du tick courant. Permet aux systemes en aval de savoir si un rebond a eu lieu |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `resolve_bounce` | `mge.foundation.bounce.v1.fn.resolve_bounce` | 133 | Bounciness, BounceState, Velocity2D, CollisionManifold, CollisionPair | Velocity2D, BounceState | BounceTriggered | O(c) | Pour chaque collision impliquant une entite avec Bounciness, reflechit la velocite le long de la normale * restitution. Ignore si velocite < min_velocity |

---

## 5. Flux de donnees

```
Bounciness ───────────┐
BounceState ─────────►│
Velocity2D ──────────►│ resolve_bounce ──► Velocity2D (reflechie)
CollisionManifold ───►│      │                BounceState (mis a jour)
CollisionPair ───────►│      │
                      │      └── emet BounceTriggered
                      │
                      └── vel_reflected = vel - 2 * dot(vel, normal) * normal
                          vel_out = vel_reflected * restitution
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `BounceTriggered` | `mge.foundation.bounce.v1.event.bounce_triggered` | `entity: EntityId, surface_entity: EntityId, velocity_x: f32, velocity_y: f32` | `resolve_bounce` | audio (son de rebond), VFX (particules), gameplay (compteur rebonds) |

---

## 7. Invariants

- `Bounciness.restitution` est borne entre 0.0 (aucun rebond) et 1.0 (rebond parfait, pas de perte d'energie).
- `Bounciness.min_velocity` est >= 0. Si la vitesse le long de la normale est inferieure, le rebond est annule et la velocite est mise a zero le long de la normale.
- `BounceState.bounced_this_frame` est reinitialise a `false` au debut de chaque tick.
- La reflexion utilise la normale du `CollisionManifold` : `v_out = (v - 2 * dot(v, n) * n) * restitution`.
- Un rebond n'est applique que si l'entite se deplace vers la surface (`dot(vel, normal) < 0`).
- Si les deux entites ont `Bounciness`, chacune rebondit avec sa propre restitution.
- `BounceTriggered` contient la velocite post-rebond.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_restitution` | `f32` | 0.5 | [0.0, 1.0] | Restitution par defaut si Bounciness est present sans valeur explicite |
| `min_bounce_velocity` | `f32` | 10.0 | [0.0, 100.0] | Seuil de vitesse minimum par defaut pour qu'un rebond se produise |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Reflechit la velocite sur collision | Ne detecte pas les collisions (→ collision-detection) |
| Applique la restitution (perte d'energie) | Ne resout pas la penetration (→ physics-basic) |
| Seuil de vitesse minimum | Ne gere pas la friction (→ friction) |
| Emet BounceTriggered pour feedback | Ne gere pas la gravite (→ gravity) |
| Suivi bounced_this_frame | Ne fait pas de simulation physique complete |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Bounciness, CollisionManifold, CollisionPair |
| Ecrit | Velocity2D, BounceState |
| Emet | BounceTriggered |
| Ne touche jamais | PhysicsBody, GravityAffected, FrictionCoefficient, Collider, Camera2D, Transform2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-bounce/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.bounce.v1, trait Plugin impl
    ├── components.rs    # Bounciness, BounceState
    ├── systems.rs       # resolve_bounce
    └── events.rs        # BounceTriggered
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] Reflexion vectorielle correcte (v - 2*dot(v,n)*n)
- [ ] Application restitution (perte d'energie)
- [ ] Seuil min_velocity respecte
- [ ] BounceState reinitialise chaque tick
- [ ] Verification direction (dot < 0) avant rebond
- [ ] Parametres GCL exposes (default_restitution, min_bounce_velocity)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : rebond mur vertical, rebond sol, restitution 0, restitution 1, sous seuil min_velocity
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.bounce.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.bounce.v1.component.bounciness","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.bounce.v1.component.bounce_state","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.bounce.v1.fn.resolve_bounce","k":"s","d":"foundation","r":["Bounciness","BounceState","Velocity2D","CollisionManifold","CollisionPair"],"w":["Velocity2D","BounceState"],"e":["BounceTriggered"],"p":133,"c":"O(c)"},
  {"i":"mge.foundation.bounce.v1.event.bounce_triggered","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let ball = world.spawn();
world.insert(ball, Bounciness {
    restitution: 0.8,
    min_velocity: 5.0,
});
world.insert(ball, BounceState { bounced_this_frame: false });
world.insert(ball, Velocity2D { x: 100.0, y: -200.0 });

// Si la balle touche un sol horizontal (normale = (0, 1)) :
// dot(vel, normal) = -200.0 < 0 → rebond
// vel_reflected = (100.0, 200.0)
// vel_out = (100.0 * 0.8, 200.0 * 0.8) = (80.0, 160.0)
// BounceState.bounced_this_frame = true
// BounceTriggered emis

let super_ball = world.spawn();
world.insert(super_ball, Bounciness {
    restitution: 1.0,
    min_velocity: 0.0,
});
world.insert(super_ball, BounceState { bounced_this_frame: false });
world.insert(super_ball, Velocity2D { x: 50.0, y: -100.0 });
// Rebond parfait, aucune perte d'energie, rebondit meme a tres faible vitesse

// Ecouter les rebonds pour jouer un son
for event in events.read::<BounceTriggered>() {
    let speed = (event.velocity_x.powi(2) + event.velocity_y.powi(2)).sqrt();
    play_bounce_sound(event.entity, speed);
}
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
