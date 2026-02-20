# mge-gfp-physics-basic

> @id mge.foundation.physics_basic.v1  
> @role plugin  
> @domain foundation  
> @do lightweight_physics_integration_deterministic  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-physics-basic` |
| @id MSCM | `mge.foundation.physics_basic.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity`, `mge-gfp-collision-detection` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n*p) n = corps dynamiques, p = contacts |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `PhysicsType` | `Static, Kinematic, Dynamic` | Type de corps physique. Static = immobile, Kinematic = deplacement programme, Dynamic = soumis aux forces |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `PhysicsBody` | `mge.foundation.physics_basic.v1.component.physics_body` | `physics_type: PhysicsType, mass: f32, inv_mass: f32` | Corps physique. Le type determine le comportement. `inv_mass` = 1/mass (0 pour Static/Kinematic) |
| `PhysicsMaterial` | `mge.foundation.physics_basic.v1.component.physics_material` | `restitution: f32, friction: f32` | Materiau physique. `restitution` = elasticite du rebond, `friction` = coefficient de friction de surface |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `integrate_physics` | `mge.foundation.physics_basic.v1.fn.integrate_physics` | 130 | PhysicsBody, PhysicsMaterial, Velocity2D, CollisionManifold, CollisionPair | Velocity2D | none | O(n*p) | Applique les forces accumulees. Pour les corps Dynamic en collision, resout la reponse (separation et ajustement de vitesse via restitution). Simulation deterministe simple |

---

## 5. Flux de donnees

```
PhysicsBody ──────────┐
PhysicsMaterial ──────►│ integrate_physics ──► Velocity2D (ajuste)
Velocity2D ───────────►│      │
CollisionManifold ────►│      └── separation positionnelle via penetration
CollisionPair ────────┘
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- Ceci n'est PAS un moteur physique complet. Simulation deterministe simplifiee uniquement.
- Les corps `Static` ne sont jamais deplaces par la physique (inv_mass = 0).
- Les corps `Kinematic` ne sont pas affectes par les collisions mais poussent les corps `Dynamic`.
- `mass` doit etre > 0 pour les corps `Dynamic`. `inv_mass` = 1.0 / mass.
- `inv_mass` = 0.0 pour `Static` et `Kinematic`.
- `restitution` est borne entre 0.0 (pas de rebond) et 1.0 (rebond parfait).
- La reponse de collision utilise la restitution minimale des deux materiaux.
- L'integration est deterministe : meme entrees = memes sorties, independamment de la plateforme.
- Le nombre de substeps est configure via GCL (max 4) pour la stabilite.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `physics_substeps` | `u32` | 1 | [1, 4] | Nombre de sous-etapes par tick. Plus = plus stable mais plus couteux |
| `fixed_dt` | `f32` | 0.01667 | [0.001, 0.05] | Pas de temps fixe (1/60 par defaut). Utilise pour l'integration deterministe |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Integration physique simple (forces → vitesse) | N'est PAS un moteur physique complet (pas de Box2D/Rapier) |
| Reponse collision (separation, rebond) | Ne gere pas la gravite (→ gravity) |
| Support Static/Kinematic/Dynamic | Ne gere pas la friction sol/air (→ friction) |
| Simulation deterministe | Ne gere pas les joints ou contraintes |
| Substeps configurables | Ne gere pas la CCD (collision continue) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | PhysicsBody, PhysicsMaterial, CollisionManifold, CollisionPair |
| Ecrit | Velocity2D |
| Emet | Aucun |
| Ne touche jamais | GravityAffected, FrictionCoefficient, Bounciness, Camera2D, Collider |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-physics-basic/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.physics_basic.v1, trait Plugin impl
    ├── components.rs    # PhysicsBody, PhysicsMaterial
    ├── systems.rs       # integrate_physics
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 enum `PhysicsType` (Static, Kinematic, Dynamic)
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` vide (aucun evenement)
- [ ] Reponse collision : separation positionnelle + ajustement vitesse
- [ ] Restitution : min(mat_a.restitution, mat_b.restitution)
- [ ] Substeps : boucle interne configurable via GCL
- [ ] Determinisme garanti (pas de HashMap, pas de float non-deterministe)
- [ ] Parametres GCL exposes (physics_substeps, fixed_dt)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : Static immobile, Dynamic rebond, Kinematic pousse, substeps stabilite, determinisme
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.physics_basic.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.physics_basic.v1.component.physics_body","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.physics_basic.v1.component.physics_material","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.physics_basic.v1.fn.integrate_physics","k":"s","d":"foundation","r":["PhysicsBody","PhysicsMaterial","Velocity2D","CollisionManifold","CollisionPair"],"w":["Velocity2D"],"e":[],"p":130,"c":"O(n*p)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let ground = world.spawn();
world.insert(ground, PhysicsBody {
    physics_type: PhysicsType::Static,
    mass: 0.0,
    inv_mass: 0.0,
});
world.insert(ground, PhysicsMaterial {
    restitution: 0.2,
    friction: 0.8,
});

let crate_entity = world.spawn();
world.insert(crate_entity, PhysicsBody {
    physics_type: PhysicsType::Dynamic,
    mass: 10.0,
    inv_mass: 0.1,
});
world.insert(crate_entity, PhysicsMaterial {
    restitution: 0.3,
    friction: 0.6,
});
world.insert(crate_entity, Velocity2D { x: 0.0, y: -50.0 });
// La caisse tombe, entre en collision avec le sol, rebondit legerement (restitution min = 0.2)
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
