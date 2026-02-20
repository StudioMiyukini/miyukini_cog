# mge-gfp-sprite-flip

> @id mge.foundation.sprite_flip.v1  
> @role plugin  
> @domain foundation  
> @do flip_sprite_horizontal_vertical_from_velocity  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-sprite-flip` |
| @id MSCM | `mge.foundation.sprite_flip.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites avec SpriteFlip et Velocity2D |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `SpriteFlip` | `mge.foundation.sprite_flip.v1.component.sprite_flip` | `flip_h: bool, flip_v: bool` | Indicateurs de retournement horizontal et vertical du sprite |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_sprite_flip` | `mge.foundation.sprite_flip.v1.fn.update_sprite_flip` | 171 | Velocity2D | SpriteFlip | none | O(n) | Met a jour `flip_h` en fonction du signe de la composante horizontale de la velocite |

---

## 5. Flux de donnees

```
Velocity2D (vx, vy)
       │
       ▼
 ┌──────────────────────────┐
 │    update_sprite_flip     │  Phase 171
 │  (signe vx → flip_h)     │
 └────────────┬──────────────┘
              │
              ▼
       SpriteFlip (flip_h, flip_v)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `SpriteFlip.flip_h` est `true` quand `Velocity2D.vx < 0.0` et `false` quand `vx > 0.0`.
- Quand `Velocity2D.vx == 0.0`, `flip_h` conserve sa valeur precedente (pas de changement).
- `SpriteFlip.flip_v` n'est pas modifie par ce systeme (reserve a une extension future ou au gameplay).
- Le systeme ne s'execute que sur les entites possedant a la fois `Velocity2D` et `SpriteFlip`.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Retourne le sprite horizontalement selon la direction | Ne gere pas le rendu du sprite (→ renderer) |
| Preserve le flip quand la velocite est nulle | Ne gere pas les animations (→ animation-state) |
| Fournit un composant flip_h/flip_v lisible par le renderer | Ne gere pas la velocite (→ velocity) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Velocity2D |
| Ecrit | SpriteFlip |
| Emet | rien |
| Ne touche jamais | Transform2D, AnimationStateMachine, CurrentState, FrameIndex |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-sprite-flip/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.sprite_flip.v1, trait Plugin impl
    ├── components.rs     # SpriteFlip
    ├── systems.rs        # update_sprite_flip
    └── events.rs         # (vide)
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | N/A |
| No allocation hot path | N/A |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 1 composant dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : flip gauche, flip droite, velocite nulle (conservation), flip_v non modifie
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.sprite_flip.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.sprite_flip.v1.component.sprite_flip","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.sprite_flip.v1.fn.update_sprite_flip","k":"s","d":"foundation","r":["Velocity2D"],"w":["SpriteFlip"],"e":[],"p":171,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Velocity2D { vx: -5.0, vy: 0.0 });
world.insert(entity, SpriteFlip { flip_h: false, flip_v: false });
// Apres Phase 171 : SpriteFlip { flip_h: true, flip_v: false }
// Le sprite est retourne horizontalement car vx < 0
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-animation-state](mge-gfp-animation-state.md) | Plugin machine a etats d'animation |
| [mge-gfp-frame-timer](mge-gfp-frame-timer.md) | Plugin timer de frames d'animation |
