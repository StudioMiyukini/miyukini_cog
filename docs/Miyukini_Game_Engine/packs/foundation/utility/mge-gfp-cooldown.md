# mge-gfp-cooldown

> @id mge.foundation.cooldown.v1  
> @role plugin  
> @domain foundation  
> @do rechargeable_cooldown_tracking  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-cooldown` |
| @id MSCM | `mge.foundation.cooldown.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(c), c = entites avec cooldowns |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Cooldown` | `mge.foundation.cooldown.v1.component.cooldown` | `remaining_ticks: u32, max_ticks: u32, ready: bool` | Cooldown rechargeable. `remaining_ticks` decremente chaque tick, `max_ticks` = duree totale du cooldown, `ready` = pret a etre utilise |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_cooldowns` | `mge.foundation.cooldown.v1.fn.tick_cooldowns` | 181 | Cooldown | Cooldown | none | O(c) | Decremente `remaining_ticks` des cooldowns non prets, met `ready = true` quand remaining atteint 0 |

---

## 5. Flux de donnees

```
Cooldown (remaining_ticks, max_ticks, ready)
       │
       ▼
 ┌────────────────────────────────┐
 │        tick_cooldowns           │  Phase 181
 │  (remaining-- → ready si 0)    │
 └────────────┬───────────────────┘
              │
              ▼
       Cooldown (remaining maj, ready maj)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- Seuls les cooldowns avec `ready = false` sont decrements.
- `ready` passe a `true` exactement quand `remaining_ticks` atteint 0.
- Quand le gameplay consomme le cooldown, il doit remettre `remaining_ticks = max_ticks` et `ready = false`.
- `max_ticks` doit etre >= 1. Une valeur de 0 signifie un cooldown toujours pret.
- Le systeme ne reinitialise jamais le cooldown : c'est la responsabilite du gameplay.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Decremente des cooldowns en ticks | Ne reinitialise pas le cooldown (responsabilite gameplay) |
| Met le flag `ready` quand le cooldown est ecoule | Ne gere pas les timers generiques (→ timer) |
| Fournit un pattern simple consommable par le gameplay | Ne gere pas les durees de vie (→ lifetime) |
| Permet la lecture directe de l'etat pret/pas pret | N'emet pas d'evenements (polling par le gameplay) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Cooldown |
| Ecrit | Cooldown |
| Emet | rien |
| Ne touche jamais | Timer, Lifetime, DespawnMarker, Transform2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-cooldown/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.cooldown.v1, trait Plugin impl
    ├── components.rs     # Cooldown
    ├── systems.rs        # tick_cooldowns
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
- [ ] Tests unitaires : cooldown normal, ready=true a 0, deja pret ignore, reset par gameplay
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.cooldown.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.cooldown.v1.component.cooldown","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.cooldown.v1.fn.tick_cooldowns","k":"s","d":"foundation","r":["Cooldown"],"w":["Cooldown"],"e":[],"p":181,"c":"O(c)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Cooldown { remaining_ticks: 30, max_ticks: 30, ready: false });
// Tick 1-29 : remaining_ticks decremente, ready = false
// Tick 30 : remaining_ticks = 0, ready = true
// Gameplay consomme : remaining_ticks = 30, ready = false (reset manuel)
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-timer](mge-gfp-timer.md) | Plugin timer generique |
| [mge-gfp-lifetime](mge-gfp-lifetime.md) | Plugin duree de vie |
