# mge-gfp-collision-detection

> @id mge.foundation.collision_detection.v1  
> @role plugin  
> @domain foundation  
> @do broad_narrow_phase_collision_pairs_manifolds  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-collision-detection` |
| @id MSCM | `mge.foundation.collision_detection.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-collider`, `mge-gfp-spatial2d`, `mge-gfp-layer-mask` |
| Hot path | Oui (critique) |
| Headless safe | Oui |
| Complexite globale | O(n) broad, O(p) narrow (p = paires candidates) |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `CollisionPair` | `mge.foundation.collision_detection.v1.component.collision_pair` | `entity_a: EntityId, entity_b: EntityId` | Paire d'entites en collision. Generee par la broad phase, raffinee par la narrow phase |
| `CollisionManifold` | `mge.foundation.collision_detection.v1.component.collision_manifold` | `normal_x: f32, normal_y: f32, penetration: f32, contact_x: f32, contact_y: f32` | Donnees de contact precise. Normale de separation, profondeur de penetration et point de contact |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `detect_broad_phase` | `mge.foundation.collision_detection.v1.fn.detect_broad_phase` | 121 | Collider, SpatialIndex | CollisionPair | none | O(n) | Utilise la grille spatiale pour trouver les paires candidates. Genere des CollisionPair sans manifold |
| `detect_narrow_phase` | `mge.foundation.collision_detection.v1.fn.detect_narrow_phase` | 122 | CollisionPair, Collider, ColliderAABB, ColliderCircle, ColliderCapsule | CollisionManifold | CollisionEnter, CollisionExit, CollisionStay | O(p) | Tests precis forme-contre-forme. Genere les manifolds et emet les evenements de collision |

---

## 5. Flux de donnees

```
SpatialIndex ──► detect_broad_phase ──► CollisionPair (candidates)
Collider         │
                 ▼
CollisionPair ──► detect_narrow_phase ──► CollisionManifold
Collider              │
ColliderAABB          ├── emet CollisionEnter (nouvelle paire)
ColliderCircle        ├── emet CollisionStay  (paire existante)
ColliderCapsule       └── emet CollisionExit  (paire disparue)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `CollisionEnter` | `mge.foundation.collision_detection.v1.event.collision_enter` | `entity_a: EntityId, entity_b: EntityId, manifold: CollisionManifold` | `detect_narrow_phase` | physics-basic, bounce, trigger-zone, gameplay |
| `CollisionExit` | `mge.foundation.collision_detection.v1.event.collision_exit` | `entity_a: EntityId, entity_b: EntityId` | `detect_narrow_phase` | trigger-zone (sortie), gameplay |
| `CollisionStay` | `mge.foundation.collision_detection.v1.event.collision_stay` | `entity_a: EntityId, entity_b: EntityId, manifold: CollisionManifold` | `detect_narrow_phase` | friction, physics-basic (contact continu) |

---

## 7. Invariants

- Les `CollisionPair` sont ordonnees : `entity_a.id < entity_b.id` pour eviter les doublons.
- La broad phase vide les paires du tick precedent avant d'en generer de nouvelles.
- La narrow phase ne traite que les paires issues de la broad phase du meme tick.
- Un `CollisionManifold` est genere uniquement si les formes se chevauchent reellement.
- `CollisionEnter` est emis une seule fois au premier tick de contact.
- `CollisionExit` est emis une seule fois au premier tick de separation.
- `CollisionStay` est emis a chaque tick tant que le contact persiste (apres le premier tick).
- La normale pointe de `entity_a` vers `entity_b`.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Broad phase via grille spatiale | Ne definit pas les formes (→ collider) |
| Narrow phase forme-contre-forme | Ne resout pas la penetration (→ physics-basic) |
| Genere les paires et manifolds | Ne filtre pas par couches (→ layer-mask) |
| Emet CollisionEnter/Exit/Stay | Ne fait pas de raycast (→ raycast) |
| Suivi des paires entre ticks | Ne gere pas les triggers (→ trigger-zone) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Collider, SpatialIndex, ColliderAABB, ColliderCircle, ColliderCapsule |
| Ecrit | CollisionPair, CollisionManifold |
| Emet | CollisionEnter, CollisionExit, CollisionStay |
| Ne touche jamais | Velocity2D, PhysicsBody, Camera2D, Transform2D, GravityAffected |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-collision-detection/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.collision_detection.v1, trait Plugin impl
    ├── components.rs    # CollisionPair, CollisionManifold
    ├── systems.rs       # detect_broad_phase, detect_narrow_phase
    └── events.rs        # CollisionEnter, CollisionExit, CollisionStay
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
- [ ] 2 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] Broad phase utilisant SpatialIndex (pas de O(n²))
- [ ] Narrow phase : AABB vs AABB, Circle vs Circle, AABB vs Circle, Capsule combinaisons
- [ ] Suivi des paires (previous frame set) pour Enter/Exit/Stay
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : broad phase filtering, narrow AABB overlap, narrow Circle overlap, Enter/Exit/Stay lifecycle
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.collision_detection.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collision_detection.v1.component.collision_pair","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collision_detection.v1.component.collision_manifold","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collision_detection.v1.fn.detect_broad_phase","k":"s","d":"foundation","r":["Collider","SpatialIndex"],"w":["CollisionPair"],"e":[],"p":121,"c":"O(n)"},
  {"i":"mge.foundation.collision_detection.v1.fn.detect_narrow_phase","k":"s","d":"foundation","r":["CollisionPair","Collider","ColliderAABB","ColliderCircle","ColliderCapsule"],"w":["CollisionManifold"],"e":["CollisionEnter","CollisionExit","CollisionStay"],"p":122,"c":"O(p)"},
  {"i":"mge.foundation.collision_detection.v1.event.collision_enter","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collision_detection.v1.event.collision_exit","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collision_detection.v1.event.collision_stay","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity_a = world.spawn();
world.insert(entity_a, Collider {
    shape: ColliderShape::AABB,
    offset_x: 0.0,
    offset_y: 0.0,
    is_trigger: false,
});
world.insert(entity_a, ColliderAABB {
    half_width: 8.0,
    half_height: 8.0,
});

let entity_b = world.spawn();
world.insert(entity_b, Collider {
    shape: ColliderShape::Circle,
    offset_x: 0.0,
    offset_y: 0.0,
    is_trigger: false,
});
world.insert(entity_b, ColliderCircle { radius: 5.0 });

// Apres execution des phases 121-122 :
// - CollisionPair genere si les entites se chevauchent
// - CollisionManifold avec normale et penetration
// - CollisionEnter emis au premier contact
for event in events.read::<CollisionEnter>() {
    println!("Contact: {:?} <-> {:?}", event.entity_a, event.entity_b);
}
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
