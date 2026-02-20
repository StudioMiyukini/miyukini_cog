# mge-gfp-layer-mask

> @id mge.foundation.layer_mask.v1  
> @role plugin  
> @domain foundation  
> @do collision_layer_mask_filtering  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-layer-mask` |
| @id MSCM | `mge.foundation.layer_mask.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(p) p = paires |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `CollisionLayer` | `mge.foundation.layer_mask.v1.component.collision_layer` | `layer: u32` | Couche de collision de l'entite. Bitmask representant la couche a laquelle l'entite appartient |
| `CollisionMask` | `mge.foundation.layer_mask.v1.component.collision_mask` | `mask: u32` | Masque de collision. Bitmask des couches avec lesquelles l'entite peut entrer en collision |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `filter_collision_pairs` | `mge.foundation.layer_mask.v1.fn.filter_collision_pairs` | 123 | CollisionPair, CollisionLayer, CollisionMask | CollisionPair | none | O(p) | Supprime les CollisionPair dont les entites ne correspondent pas au masque. Condition : `(a.layer & b.mask) != 0 && (b.layer & a.mask) != 0` |

---

## 5. Flux de donnees

```
CollisionPair ──► filter_collision_pairs ──► CollisionPair (filtre)
CollisionLayer       │
CollisionMask        └── supprime les paires ou (layer & mask) == 0
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- Le filtrage est symetrique : les deux entites de la paire doivent se "voir" mutuellement.
- `CollisionLayer` par defaut = `1` (couche 0 active).
- `CollisionMask` par defaut = `0xFFFFFFFF` (voit toutes les couches).
- Une entite sans `CollisionLayer` est traitee comme couche 1.
- Une entite sans `CollisionMask` est traitee comme masque `0xFFFFFFFF`.
- Le filtre s'execute apres la broad phase et avant la narrow phase dans l'ordre des phases.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `max_layers` | `u32` | 32 | [1, 32] | Nombre maximum de couches de collision utilisables. Limite logique, le bitmask supporte toujours 32 bits |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Filtre les paires de collision par couches | Ne detecte pas les collisions (→ collision-detection) |
| Supprime les paires non-compatibles | Ne definit pas les formes (→ collider) |
| Fournit les composants Layer et Mask | Ne resout pas la physique (→ physics-basic) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | CollisionPair, CollisionLayer, CollisionMask |
| Ecrit | CollisionPair (suppression des paires filtrees) |
| Emet | Aucun |
| Ne touche jamais | Collider, CollisionManifold, Velocity2D, PhysicsBody, Transform2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-layer-mask/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.layer_mask.v1, trait Plugin impl
    ├── components.rs    # CollisionLayer, CollisionMask
    ├── systems.rs       # filter_collision_pairs
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
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` vide (aucun evenement)
- [ ] Filtrage symetrique (a.layer & b.mask) && (b.layer & a.mask)
- [ ] Valeurs par defaut coherentes (layer=1, mask=all)
- [ ] Parametre GCL `max_layers` expose
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : filtrage basique, filtrage symetrique, valeurs par defaut, couches multiples
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.layer_mask.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.layer_mask.v1.component.collision_layer","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.layer_mask.v1.component.collision_mask","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.layer_mask.v1.fn.filter_collision_pairs","k":"s","d":"foundation","r":["CollisionPair","CollisionLayer","CollisionMask"],"w":["CollisionPair"],"e":[],"p":123,"c":"O(p)"}
]
```

---

## 12. Exemple d'utilisation

```rust
const LAYER_PLAYER: u32   = 1 << 0; // couche 0
const LAYER_ENEMY: u32    = 1 << 1; // couche 1
const LAYER_BULLET: u32   = 1 << 2; // couche 2
const LAYER_PLATFORM: u32 = 1 << 3; // couche 3

let player = world.spawn();
world.insert(player, CollisionLayer { layer: LAYER_PLAYER });
world.insert(player, CollisionMask { mask: LAYER_ENEMY | LAYER_PLATFORM });

let enemy = world.spawn();
world.insert(enemy, CollisionLayer { layer: LAYER_ENEMY });
world.insert(enemy, CollisionMask { mask: LAYER_PLAYER | LAYER_BULLET | LAYER_PLATFORM });

let bullet = world.spawn();
world.insert(bullet, CollisionLayer { layer: LAYER_BULLET });
world.insert(bullet, CollisionMask { mask: LAYER_ENEMY });
// Le bullet ne collisionne qu'avec les ennemis, pas le joueur ni les plateformes
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
