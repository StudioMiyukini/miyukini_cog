# mge-puzzle-combo

> @id mge.puzzle.combo.v1  
> @role plugin  
> @domain puzzle  
> @do track_combo_chains_and_multipliers  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-puzzle-combo` |
| @id MSCM | `mge.puzzle.combo.v1` |
| Domaine | puzzle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-puzzle-match` |
| Hot path | Non (1 update par match) |
| Headless safe | Oui |
| Complexite globale | O(1) par tick |

---

## 2. Enumerations

Aucune enumeration dediee. Le combo est un compteur continu.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ComboChain` | `mge.puzzle.combo.v1.component.combo_chain` | `count: u32, multiplier: f32, active: bool` | Chaine de combo courante. active = au moins 1 match dans la fenetre |
| `ComboConfig` | `mge.puzzle.combo.v1.component.combo_config` | `base_multiplier: f32, increment_per_step: f32, max_multiplier: f32, decay_ticks: u32` | Parametres combo. Singleton |

---

## 4. Formules

```
multiplier = base_multiplier + (count - 1) * increment_per_step
multiplier = min(multiplier, max_multiplier)

Decay : si aucun MatchResolved pendant decay_ticks ticks consecutifs → reset combo
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `track_combo` | `mge.puzzle.combo.v1.fn.track_combo` | 1425 | MatchResolved (event), ComboChain, ComboConfig | ComboChain | ComboIncremented | O(1) | Incremente count a chaque MatchResolved. Recalcule multiplier |
| `apply_combo_multiplier` | `mge.puzzle.combo.v1.fn.apply_combo_multiplier` | 1426 | ComboChain, ComboConfig | ComboChain | none | O(1) | Clampe multiplier a max_multiplier. Active le flag active |
| `decay_combo` | `mge.puzzle.combo.v1.fn.decay_combo` | 1427 | ComboChain, ComboConfig | ComboChain | ComboReset | O(1) | Si aucun match depuis decay_ticks, reset count a 0, multiplier a base. Emet ComboReset |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ComboIncremented` | `mge.puzzle.combo.v1.event.combo_incremented` | `count: u32, multiplier: f32` | `track_combo` | score (update_score), goal, ui (animation combo) |
| `ComboReset` | `mge.puzzle.combo.v1.event.combo_reset` | `final_count: u32` | `decay_combo` | ui (fin animation), goal |

---

## 7. Invariants

- `ComboChain.multiplier` est toujours dans [base_multiplier, max_multiplier].
- `ComboChain.count` est 0 quand inactive, >= 1 quand active.
- Le decay ne se produit qu'en phase `Idle` (pas pendant les cascades).
- Un combo est incremente une seule fois par MatchResolved, meme si plusieurs matchs simultanes.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `base_combo_multiplier` | `f32` | 1.0 | [1.0, 3.0] | Multiplicateur de base (combo count = 1) |
| `combo_increment` | `f32` | 0.25 | [0.1, 2.0] | Increment par etape de combo |
| `max_combo_multiplier` | `f32` | 5.0 | [2.0, 20.0] | Plafond du multiplicateur |
| `combo_decay_ticks` | `u32` | 60 | [10, 600] | Ticks sans match avant reset |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Compte les matchs consecutifs | Ne detecte pas les matchs (→ match) |
| Calcule le multiplicateur | Ne calcule pas le score (→ match.update_score) |
| Reset apres inactivite | Ne gere pas la grille (→ board) |
| Expose le multiplicateur pour la formule de score | Ne gere pas les objectifs (→ goal) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ComboChain, ComboConfig, MatchResolved (event) |
| Ecrit | ComboChain |
| Emet | ComboIncremented, ComboReset |
| Ne touche jamais | Tile, Board, Cell, SwapAction, Score, Goal, PuzzleTimer |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-puzzle-combo/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.puzzle.combo.v1, trait Plugin impl
    ├── components.rs     # ComboChain, ComboConfig
    ├── systems.rs        # track_combo, apply_combo_multiplier, decay_combo
    └── events.rs         # ComboIncremented, ComboReset
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] Formule multiplier parametrable via GCL
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : combo increment, multiplier clamp, decay, reset
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.puzzle.combo.v1","k":"p","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.combo.v1.component.combo_chain","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.combo.v1.component.combo_config","k":"d","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.combo.v1.fn.track_combo","k":"s","d":"puzzle","r":["ComboChain","ComboConfig"],"w":["ComboChain"],"e":["ComboIncremented"],"p":1425,"c":"O(1)"},
  {"i":"mge.puzzle.combo.v1.fn.apply_combo_multiplier","k":"s","d":"puzzle","r":["ComboChain","ComboConfig"],"w":["ComboChain"],"e":[],"p":1426,"c":"O(1)"},
  {"i":"mge.puzzle.combo.v1.fn.decay_combo","k":"s","d":"puzzle","r":["ComboChain","ComboConfig"],"w":["ComboChain"],"e":["ComboReset"],"p":1427,"c":"O(1)"},
  {"i":"mge.puzzle.combo.v1.event.combo_incremented","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.puzzle.combo.v1.event.combo_reset","k":"e","d":"puzzle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let board = world.spawn();
world.insert(board, ComboChain { count: 0, multiplier: 1.0, active: false });
world.insert(board, ComboConfig {
    base_multiplier: 1.0,
    increment_per_step: 0.25,
    max_multiplier: 5.0,
    decay_ticks: 60,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Puzzle - Index](_index.md) | Vue d'ensemble du pack |
| [mge-puzzle-match](mge-puzzle-match.md) | Plugin match (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
