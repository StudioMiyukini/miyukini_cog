# mge-gfp-proximity-check

> @id mge.foundation.proximity_check.v1  
> @role plugin  
> @domain foundation  
> @do detect_nearby_entities_within_radius  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-proximity-check` |
| @id MSCM | `mge.foundation.proximity_check.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform`, `mge-gfp-spatial2d` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(n*k), n = entites avec ProximityRadius, k = moyenne entites proches |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ProximityRadius` | `mge.foundation.proximity_check.v1.component.proximity_radius` | `radius: f32` | Rayon de detection de proximite autour de l'entite |
| `NearbyEntities` | `mge.foundation.proximity_check.v1.component.nearby_entities` | `entities: Vec<EntityId>` | Liste des entites detectees dans le rayon de proximite, mise a jour chaque frame |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_proximity` | `mge.foundation.proximity_check.v1.fn.update_proximity` | 152 | ProximityRadius, Transform2D, SpatialIndex | NearbyEntities | none | O(n*k) | Interroge l'index spatial pour trouver les entites dans le rayon de chaque entite equipe de ProximityRadius |

---

## 5. Flux de donnees

```
ProximityRadius (rayon)
       │
       ├──── Transform2D (position entite)
       ├──── SpatialIndex (index spatial 2D)
       │
       ▼
 ┌──────────────────────────┐
 │ update_proximity          │  Phase 152
 │ (requete index spatial   │
 │  → entites dans rayon)   │
 └──────────┬───────────────┘
            │
            ▼
      NearbyEntities (liste entites proches)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `NearbyEntities` est recalcule entierement chaque frame (pas de delta).
- `NearbyEntities.entities` ne contient jamais l'entite elle-meme.
- `ProximityRadius.radius` doit etre > 0.0 (rayon non nul).
- La detection utilise l'index spatial (`SpatialIndex` de mge-gfp-spatial2d) pour eviter O(n²).
- L'ordre des entites dans `NearbyEntities.entities` n'est pas garanti.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Detecte les entites dans un rayon donne | Ne resout pas les interactions (→ interaction-system) |
| Interroge l'index spatial pour performance | Ne marque pas les entites comme interactables (→ interactable) |
| Met a jour la liste de voisins chaque frame | Ne gere pas les evenements d'activation (→ activation-event) |
| Fournit les donnees de proximite pour tout le pipeline interaction | Ne gere pas les collisions (→ collision-detection) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ProximityRadius, Transform2D, SpatialIndex |
| Ecrit | NearbyEntities |
| Emet | rien |
| Ne touche jamais | Velocity2D, Camera2D, Collider, Interactable, InteractableState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-proximity-check/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.proximity_check.v1, trait Plugin impl
    ├── components.rs     # ProximityRadius, NearbyEntities
    ├── systems.rs        # update_proximity
    └── events.rs         # (vide)
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
- [ ] `events.rs` present (vide)
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : detection basique, exclusion self, rayon nul, aucun voisin, multiple voisins, spatial index query
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.proximity_check.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.proximity_check.v1.component.proximity_radius","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.proximity_check.v1.component.nearby_entities","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.proximity_check.v1.fn.update_proximity","k":"s","d":"foundation","r":["ProximityRadius","Transform2D","SpatialIndex"],"w":["NearbyEntities"],"e":[],"p":152,"c":"O(n*k)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, Transform2D {
    x: 100.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(player, ProximityRadius { radius: 64.0 });
world.insert(player, NearbyEntities { entities: Vec::new() });

let npc = world.spawn();
world.insert(npc, Transform2D {
    x: 130.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-interactable](mge-gfp-interactable.md) | Marqueur interactable (utilise NearbyEntities) |
| [mge-gfp-interaction-system](mge-gfp-interaction-system.md) | Resolution interactions (depend de proximity-check) |
| [mge-gfp-spatial2d](../spatial/mge-gfp-spatial2d.md) | Index spatial (prerequis) |
