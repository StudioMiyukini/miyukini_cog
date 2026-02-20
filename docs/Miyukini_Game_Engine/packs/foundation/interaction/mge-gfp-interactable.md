# mge-gfp-interactable

> @id mge.foundation.interactable.v1  
> @role plugin  
> @domain foundation  
> @do mark_entity_as_interactable_with_state  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-interactable` |
| @id MSCM | `mge.foundation.interactable.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites interactables |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `InteractableType` | `Action`, `Toggle`, `Hold` | Type d'interaction supportee par l'entite. Action = ponctuelle, Toggle = bascule, Hold = maintenu |
| `InteractState` | `Idle`, `Available`, `InProgress`, `Disabled` | Etat courant de l'interactable dans le cycle d'interaction |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Interactable` | `mge.foundation.interactable.v1.component.interactable` | `interact_type: InteractableType, priority: u8, range: f32` | Marque une entite comme interactable. Type d'interaction, priorite de resolution et portee d'interaction |
| `InteractableState` | `mge.foundation.interactable.v1.component.interactable_state` | `state: InteractState` | Etat courant de l'interactable, mis a jour chaque frame par le systeme |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_interactable_state` | `mge.foundation.interactable.v1.fn.update_interactable_state` | 150 | Interactable | InteractableState | none | O(n) | Reinitialise la disponibilite de chaque interactable en fonction de la proximite chaque frame |

---

## 5. Flux de donnees

```
Interactable (type, priorite, portee)
       │
       ▼
 ┌────────────────────────────┐
 │ update_interactable_state   │  Phase 150
 │ (reset dispo basee sur     │
 │  proximite chaque frame)   │
 └──────────┬─────────────────┘
            │
            ▼
      InteractableState (Idle / Available / InProgress / Disabled)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `InteractableState` est reinitialise chaque frame avant la resolution des interactions (Phase 151).
- `Interactable.priority` determine l'ordre de resolution quand plusieurs interactables sont a portee (0 = plus haute priorite).
- `Interactable.range` doit etre >= 0.0 (portee non negative).
- Un interactable avec `InteractState::Disabled` ne peut pas etre cible par une interaction.
- Ce plugin ne resout pas les interactions — il ne fait que marquer et maintenir l'etat.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Marque les entites comme interactables avec type et priorite | Ne resout pas les interactions (→ interaction-system) |
| Maintient l'etat courant de chaque interactable | Ne detecte pas la proximite (→ proximity-check) |
| Reinitialise la disponibilite chaque frame | Ne gere pas les evenements d'activation (→ activation-event) |
| Definit les types d'interaction (Action, Toggle, Hold) | Ne gere pas la logique de jeu specifique |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Interactable |
| Ecrit | InteractableState |
| Emet | rien |
| Ne touche jamais | Transform2D, Velocity2D, Camera2D, ProximityRadius, NearbyEntities |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-interactable/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.interactable.v1, trait Plugin impl
    ├── components.rs     # Interactable, InteractableState, InteractableType, InteractState
    ├── systems.rs        # update_interactable_state
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
- [ ] 2 enumerations dans `components.rs` (InteractableType, InteractState)
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : etat idle par defaut, transition available, disabled bloque, types d'interaction
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.interactable.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interactable.v1.component.interactable","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interactable.v1.component.interactable_state","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.interactable.v1.fn.update_interactable_state","k":"s","d":"foundation","r":["Interactable"],"w":["InteractableState"],"e":[],"p":150,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let chest = world.spawn();
world.insert(chest, Transform2D {
    x: 300.0, y: 200.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(chest, Interactable {
    interact_type: InteractableType::Action,
    priority: 0,
    range: 48.0,
});
world.insert(chest, InteractableState {
    state: InteractState::Idle,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-interaction-system](mge-gfp-interaction-system.md) | Resolution interactions (depend de interactable) |
| [mge-gfp-proximity-check](mge-gfp-proximity-check.md) | Detection proximite (alimente la disponibilite) |
