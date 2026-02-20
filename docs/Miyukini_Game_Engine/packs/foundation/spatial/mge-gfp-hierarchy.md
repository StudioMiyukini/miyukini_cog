# mge-gfp-hierarchy

> @id mge.foundation.hierarchy.v1  
> @role plugin  
> @domain foundation  
> @do parent_child_transform_propagation_reparent  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-hierarchy` |
| @id MSCM | `mge.foundation.hierarchy.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform` |
| Hot path | Oui (traversee de hierarchie) |
| Headless safe | Oui |
| Complexite globale | O(d*c), d = profondeur, c = enfants par noeud |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Parent` | `mge.foundation.hierarchy.v1.component.parent` | `entity: EntityId` | Reference vers l'entite parente. Absent si l'entite est racine |
| `Children` | `mge.foundation.hierarchy.v1.component.children` | `entities: Vec<EntityId>` | Liste ordonnee des entites enfants |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `propagate_parent_transform` | `mge.foundation.hierarchy.v1.fn.propagate_parent_transform` | 104 | Parent, Children, WorldTransform | WorldTransform | none | O(d*c) | Parcourt la hierarchie racine→feuilles et multiplie la WorldTransform parente avec le Transform2D local de l'enfant |
| `process_reparent` | `mge.foundation.hierarchy.v1.fn.process_reparent` | 105 | ReparentRequest | Parent, Children | ParentChanged, ChildAdded, ChildRemoved | O(r) r=requests | Traite les demandes de reparentage. Retire l'enfant de l'ancien parent, l'ajoute au nouveau |

---

## 5. Flux de donnees

```
ReparentRequest ─────────────────────────────┐
                                             │
Parent + Children + WorldTransform (parent)  │
       │                                     │
       ▼                                     ▼
 ┌─────────────────────────────┐  ┌────────────────────┐
 │ propagate_parent_transform  │  │  process_reparent  │
 │ (hierarchie → WorldTransform│  │  (reparentage)     │
 │  enfants)           Phase104│  │            Phase 105│
 └─────────────┬───────────────┘  └──────┬─────────────┘
               │                         │
               ▼                         ▼
     WorldTransform (enfants)    ParentChanged
                                 ChildAdded
                                 ChildRemoved
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ParentChanged` | `mge.foundation.hierarchy.v1.event.parent_changed` | `entity: EntityId, old_parent: Option<EntityId>, new_parent: Option<EntityId>` | `process_reparent` | ui (arbre scene), serialisation |
| `ChildAdded` | `mge.foundation.hierarchy.v1.event.child_added` | `parent: EntityId, child: EntityId` | `process_reparent` | ui (arbre scene), gameplay |
| `ChildRemoved` | `mge.foundation.hierarchy.v1.event.child_removed` | `parent: EntityId, child: EntityId` | `process_reparent` | ui (arbre scene), cleanup |

**Request :**

| Request | @id MSCM | Champs | Description |
|---------|----------|--------|-------------|
| `ReparentRequest` | `mge.foundation.hierarchy.v1.event.reparent_request` | `entity: EntityId, new_parent: Option<EntityId>` | Demande de reparentage. new_parent = None pour detacher |

---

## 7. Invariants

- Un enfant a exactement un `Parent` ou aucun (racine).
- `Children.entities` ne contient jamais de doublons.
- Pas de cycle : une entite ne peut pas etre son propre ancetre.
- La propagation parcourt toujours racine → feuilles (top-down).
- Apres Phase 104, toutes les `WorldTransform` enfants integrent la transformation parente.
- Les `ReparentRequest` sont consommees dans le tick courant, jamais reportees.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la relation parent/child | Ne gere pas la position locale (→ transform) |
| Propage WorldTransform dans la hierarchie | Ne gere pas le deplacement (→ velocity) |
| Traite les demandes de reparentage | Ne gere pas la serialisation de la hierarchie |
| Emet des evenements de changement de parent | Ne limite pas la profondeur de hierarchie |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Parent, Children, WorldTransform, ReparentRequest |
| Ecrit | Parent, Children, WorldTransform |
| Emet | ParentChanged, ChildAdded, ChildRemoved |
| Ne touche jamais | Transform2D, Velocity2D, SpatialIndex, Collider |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-hierarchy/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.hierarchy.v1, trait Plugin impl
    ├── components.rs     # Parent, Children
    ├── systems.rs        # propagate_parent_transform, process_reparent
    └── events.rs         # ParentChanged, ChildAdded, ChildRemoved, ReparentRequest
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
- [ ] 4 evenements dans `events.rs` (3 events + 1 request) avec @id et @fields
- [ ] Detection de cycles dans process_reparent
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : propagation, reparentage, detachement, cycle rejection
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.hierarchy.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.hierarchy.v1.component.parent","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.hierarchy.v1.component.children","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.hierarchy.v1.fn.propagate_parent_transform","k":"s","d":"foundation","r":["Parent","Children","WorldTransform"],"w":["WorldTransform"],"e":[],"p":104,"c":"O(d*c)"},
  {"i":"mge.foundation.hierarchy.v1.fn.process_reparent","k":"s","d":"foundation","r":["ReparentRequest"],"w":["Parent","Children"],"e":["ParentChanged","ChildAdded","ChildRemoved"],"p":105,"c":"O(r)"},
  {"i":"mge.foundation.hierarchy.v1.event.parent_changed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.hierarchy.v1.event.child_added","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.hierarchy.v1.event.child_removed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.hierarchy.v1.event.reparent_request","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let parent = world.spawn();
world.insert(parent, Transform2D { x: 100.0, y: 50.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(parent, Children { entities: vec![] });

let child = world.spawn();
world.insert(child, Transform2D { x: 10.0, y: 5.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(child, Parent { entity: parent });
// Apres Phase 104 : child.WorldTransform = parent.WorldTransform * child.Transform2D
// → position monde enfant = (110.0, 55.0)
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-transform](mge-gfp-transform.md) | Plugin transform (fournit WorldTransform a propager) |
| [mge-gfp-bounds](mge-gfp-bounds.md) | Plugin AABB (depend de WorldTransform propage) |
