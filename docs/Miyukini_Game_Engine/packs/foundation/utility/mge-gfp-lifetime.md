# mge-gfp-lifetime

> @id mge.foundation.lifetime.v1  
> @role plugin  
> @domain foundation  
> @do entity_lifetime_auto_expiration  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-lifetime` |
| @id MSCM | `mge.foundation.lifetime.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites avec Lifetime |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Lifetime` | `mge.foundation.lifetime.v1.component.lifetime` | `remaining_ticks: u32` | Duree de vie restante de l'entite en ticks. Quand elle atteint 0, l'entite est marquee pour despawn |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_lifetimes` | `mge.foundation.lifetime.v1.fn.tick_lifetimes` | 182 | Lifetime | Lifetime, DespawnMarker | LifetimeExpired | O(n) | Decremente `remaining_ticks`, emet `LifetimeExpired` et ajoute `DespawnMarker` quand remaining atteint 0 |

---

## 5. Flux de donnees

```
Lifetime (remaining_ticks)
       │
       ▼
 ┌──────────────────────────────────┐
 │         tick_lifetimes            │  Phase 182
 │  (remaining-- → despawn si 0)    │
 └─────┬──────────────┬─────────────┘
       │              │
       ▼              ▼
 DespawnMarker    LifetimeExpired
 (ajoute)            (event)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `LifetimeExpired` | `mge.foundation.lifetime.v1.event.lifetime_expired` | `entity: EntityId` | `tick_lifetimes` | despawn, systemes de cleanup, effets visuels |

---

## 7. Invariants

- `Lifetime.remaining_ticks` est decremente de 1 a chaque tick.
- Quand `remaining_ticks` atteint 0, `LifetimeExpired` est emis exactement une fois.
- Quand `remaining_ticks` atteint 0, un `DespawnMarker` avec `reason: LifetimeExpired` est ajoute a l'entite.
- Une entite sans composant `Lifetime` n'est jamais affectee par ce systeme.
- Le systeme n'effectue pas le despawn lui-meme : il delegue au plugin despawn via `DespawnMarker`.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Decremente la duree de vie des entites | Ne supprime pas les entites (→ despawn) |
| Marque les entites expirees pour despawn | Ne gere pas les timers generiques (→ timer) |
| Emet un evenement a l'expiration | Ne gere pas les cooldowns (→ cooldown) |
| Delegue le nettoyage au systeme de despawn | Ne prolonge pas la duree de vie (responsabilite gameplay) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Lifetime |
| Ecrit | Lifetime, DespawnMarker |
| Emet | LifetimeExpired |
| Ne touche jamais | Timer, Cooldown, Transform2D, Velocity2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-lifetime/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.lifetime.v1, trait Plugin impl
    ├── components.rs     # Lifetime
    ├── systems.rs        # tick_lifetimes
    └── events.rs         # LifetimeExpired
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
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : expiration normale, DespawnMarker ajoute, event emis une seule fois, entite sans Lifetime ignoree
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.lifetime.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.lifetime.v1.component.lifetime","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.lifetime.v1.event.lifetime_expired","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.lifetime.v1.fn.tick_lifetimes","k":"s","d":"foundation","r":["Lifetime"],"w":["Lifetime","DespawnMarker"],"e":["LifetimeExpired"],"p":182,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let bullet = world.spawn();
world.insert(bullet, Transform2D { x: 50.0, y: 50.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(bullet, Velocity2D { vx: 10.0, vy: 0.0 });
world.insert(bullet, Lifetime { remaining_ticks: 120 });
// Apres 120 ticks :
// → LifetimeExpired { entity: bullet }
// → DespawnMarker { reason: DespawnReason::LifetimeExpired } ajoute
// → Le plugin despawn supprimera l'entite
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-despawn](mge-gfp-despawn.md) | Plugin despawn (supprime les entites marquees) |
| [mge-gfp-timer](mge-gfp-timer.md) | Plugin timer generique |
