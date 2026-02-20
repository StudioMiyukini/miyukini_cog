# mge-gfp-gravity

> @id mge.foundation.gravity.v1  
> @role plugin  
> @domain foundation  
> @do apply_configurable_gravity_force  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-gravity` |
| @id MSCM | `mge.foundation.gravity.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n) |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `GravityAffected` | `mge.foundation.gravity.v1.component.gravity_affected` | `enabled: bool` | Marqueur pour les entites soumises a la gravite. `enabled` permet de desactiver temporairement |
| `GravityScale` | `mge.foundation.gravity.v1.component.gravity_scale` | `scale: f32` | Multiplicateur individuel de gravite. 1.0 = normal, 0.0 = pas de gravite, 2.0 = double, -1.0 = inversee |
| `GravityConfig` | `mge.foundation.gravity.v1.component.gravity_config` | `gravity_x: f32, gravity_y: f32` | Configuration globale de gravite (composant statique). Vecteur de force gravitationnelle |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_gravity` | `mge.foundation.gravity.v1.fn.apply_gravity` | 131 | GravityAffected, GravityScale, GravityConfig, Velocity2D | Velocity2D | none | O(n) | Pour chaque entite avec GravityAffected (enabled=true), applique `velocity += gravity * scale * dt` |

---

## 5. Flux de donnees

```
GravityConfig ────────┐
GravityAffected ─────►│ apply_gravity ──► Velocity2D (modifie)
GravityScale ────────►│
Velocity2D ──────────►│
                      └── vel += (gravity_x * scale * dt, gravity_y * scale * dt)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `GravityConfig` est un composant singleton (une seule instance dans le World).
- Si `GravityConfig` est absent, la gravite par defaut GCL est utilisee.
- Si `GravityScale` est absent, le scale par defaut est 1.0.
- Si `GravityAffected.enabled = false`, l'entite est ignoree.
- La gravite est appliquee apres l'integration physique de base (phase 131 > phase 130).
- `GravityScale.scale` peut etre negatif (gravite inversee).
- L'application utilise `dt` fixe (pas de variable timestep).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `gravity_x` | `f32` | 0.0 | [-9999.0, 9999.0] | Composante horizontale de la gravite. 0 = pas de gravite laterale |
| `gravity_y` | `f32` | -980.0 | [-9999.0, 9999.0] | Composante verticale de la gravite. Negatif = vers le bas |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Applique la gravite sur la velocite | Ne resout pas les collisions (→ physics-basic) |
| Scale individuel par entite | Ne gere pas la friction (→ friction) |
| Gravite configurable (direction, force) | Ne gere pas le rebond (→ bounce) |
| Support gravite inversee / zero-G | Ne detecte pas le sol (→ kinematic-controller) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | GravityAffected, GravityScale, GravityConfig |
| Ecrit | Velocity2D |
| Emet | Aucun |
| Ne touche jamais | PhysicsBody, CollisionPair, CollisionManifold, FrictionCoefficient, Bounciness, Camera2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-gravity/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.gravity.v1, trait Plugin impl
    ├── components.rs    # GravityAffected, GravityScale, GravityConfig
    ├── systems.rs       # apply_gravity
    └── events.rs        # (vide)
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
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` vide (aucun evenement)
- [ ] GravityConfig singleton
- [ ] Fallback vers valeurs GCL si GravityConfig absent
- [ ] GravityScale optionnel (defaut 1.0)
- [ ] Parametres GCL exposes (gravity_x, gravity_y)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : gravite standard, scale double, scale inverse, enabled=false, GravityConfig absent
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.gravity.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.gravity.v1.component.gravity_affected","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.gravity.v1.component.gravity_scale","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.gravity.v1.component.gravity_config","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.gravity.v1.fn.apply_gravity","k":"s","d":"foundation","r":["GravityAffected","GravityScale","GravityConfig","Velocity2D"],"w":["Velocity2D"],"e":[],"p":131,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
// Configuration globale de gravite
let config = world.spawn();
world.insert(config, GravityConfig {
    gravity_x: 0.0,
    gravity_y: -980.0,
});

// Entite affectee par la gravite (scale normal)
let ball = world.spawn();
world.insert(ball, GravityAffected { enabled: true });
world.insert(ball, GravityScale { scale: 1.0 });
world.insert(ball, Velocity2D { x: 100.0, y: 0.0 });

// Entite avec gravite reduite (flottante)
let feather = world.spawn();
world.insert(feather, GravityAffected { enabled: true });
world.insert(feather, GravityScale { scale: 0.3 });
world.insert(feather, Velocity2D { x: 20.0, y: 0.0 });

// Entite en zero-G temporaire
let astronaut = world.spawn();
world.insert(astronaut, GravityAffected { enabled: false });
world.insert(astronaut, Velocity2D { x: 50.0, y: 50.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
