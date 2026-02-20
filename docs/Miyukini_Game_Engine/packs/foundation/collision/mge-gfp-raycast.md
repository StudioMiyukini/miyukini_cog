# mge-gfp-raycast

> @id mge.foundation.raycast.v1  
> @role plugin  
> @domain foundation  
> @do raycast_2d_queries_sorted_hits  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-raycast` |
| @id MSCM | `mge.foundation.raycast.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-collider`, `mge-gfp-spatial2d`, `mge-gfp-layer-mask` |
| Hot path | Non (a la demande) |
| Headless safe | Oui |
| Complexite globale | O(r*n) r = requetes, optimise via grille spatiale |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `RaycastRequest` | `mge.foundation.raycast.v1.component.raycast_request` | `origin_x: f32, origin_y: f32, dir_x: f32, dir_y: f32, max_distance: f32, mask: u32` | Demande de raycast. Origine, direction (normalisee), distance maximale et masque de couches |
| `RaycastHit` | `mge.foundation.raycast.v1.component.raycast_hit` | `entity: EntityId, point_x: f32, point_y: f32, normal_x: f32, normal_y: f32, distance: f32` | Resultat d'un raycast. Entite touchee, point d'impact, normale de surface et distance |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_raycast_requests` | `mge.foundation.raycast.v1.fn.process_raycast_requests` | 124 | RaycastRequest, Collider, SpatialIndex, CollisionLayer | RaycastHit | RaycastCompleted | O(r*n) | Traite chaque RaycastRequest. Lance le rayon contre les colliders filtres par masque. Trie les hits par distance croissante. Emet RaycastCompleted |

---

## 5. Flux de donnees

```
RaycastRequest ──► process_raycast_requests ──► RaycastHit (tries par distance)
Collider              │
SpatialIndex          ├── filtre par mask vs CollisionLayer
CollisionLayer        └── emet RaycastCompleted
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `RaycastCompleted` | `mge.foundation.raycast.v1.event.raycast_completed` | `request_entity: EntityId, hits: Vec<RaycastHit>` | `process_raycast_requests` | gameplay (visibilite, tir), IA (detection), interaction (line of sight) |

---

## 7. Invariants

- Les `RaycastHit` sont tries par distance croissante (le plus proche en premier).
- Un rayon ne touche pas l'entite qui porte le `RaycastRequest`.
- La direction du rayon doit etre normalisee. Si `length(dir) == 0`, le request est ignore.
- `max_distance > 0`, sinon le request est ignore.
- Le masque du raycast est compare au `CollisionLayer` des colliders (meme semantique que `CollisionMask`).
- Les `RaycastRequest` sont consommes (supprimes) apres traitement. Ils ne persistent pas.
- Les triggers (`is_trigger = true`) sont inclus dans les resultats du raycast.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Lance des rayons 2D contre les colliders | Ne detecte pas les collisions continues (→ extension v2) |
| Filtre par masque de couche | Ne modifie pas les colliders (→ collider) |
| Trie les resultats par distance | Ne resout pas de physique (→ physics-basic) |
| Utilise la grille spatiale pour optimiser | Ne fournit pas de shape cast (sphere/box cast) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | RaycastRequest, Collider, SpatialIndex, CollisionLayer, ColliderAABB, ColliderCircle, ColliderCapsule |
| Ecrit | RaycastHit |
| Emet | RaycastCompleted |
| Ne touche jamais | Velocity2D, PhysicsBody, CollisionPair, CollisionManifold, Camera2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-raycast/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.raycast.v1, trait Plugin impl
    ├── components.rs    # RaycastRequest, RaycastHit
    ├── systems.rs       # process_raycast_requests
    └── events.rs        # RaycastCompleted
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
- [ ] Ray vs AABB intersection
- [ ] Ray vs Circle intersection
- [ ] Ray vs Capsule intersection
- [ ] Tri des hits par distance croissante
- [ ] Filtrage par masque vs CollisionLayer
- [ ] Utilisation de SpatialIndex pour eviter O(n²)
- [ ] Consommation des RaycastRequest apres traitement
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : ray hit AABB, ray hit Circle, ray miss, tri par distance, filtrage masque
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.raycast.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.raycast.v1.component.raycast_request","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.raycast.v1.component.raycast_hit","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.raycast.v1.fn.process_raycast_requests","k":"s","d":"foundation","r":["RaycastRequest","Collider","SpatialIndex","CollisionLayer"],"w":["RaycastHit"],"e":["RaycastCompleted"],"p":124,"c":"O(r*n)"},
  {"i":"mge.foundation.raycast.v1.event.raycast_completed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let ray = world.spawn();
world.insert(ray, RaycastRequest {
    origin_x: 100.0,
    origin_y: 50.0,
    dir_x: 1.0,
    dir_y: 0.0,
    max_distance: 500.0,
    mask: 0xFFFFFFFF,
});

// Apres execution de la phase 124 :
for event in events.read::<RaycastCompleted>() {
    if let Some(closest) = event.hits.first() {
        println!(
            "Premier impact : entite {:?} a distance {:.1} au point ({:.1}, {:.1})",
            closest.entity, closest.distance, closest.point_x, closest.point_y
        );
    }
}
// Le composant RaycastRequest est automatiquement supprime apres traitement
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
