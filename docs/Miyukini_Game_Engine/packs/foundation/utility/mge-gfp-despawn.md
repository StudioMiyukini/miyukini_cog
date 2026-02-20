# mge-gfp-despawn

> @id mge.foundation.despawn.v1  
> @role plugin  
> @domain foundation  
> @do deferred_entity_removal_cleanup  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-despawn` |
| @id MSCM | `mge.foundation.despawn.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(d), d = entites marquees pour despawn |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `DespawnReason` | `LifetimeExpired`, `Killed`, `Manual`, `OutOfBounds` | Raison pour laquelle l'entite est marquee pour suppression |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `DespawnMarker` | `mge.foundation.despawn.v1.component.despawn_marker` | `reason: DespawnReason` | Marqueur de suppression differe. L'entite sera supprimee par le systeme de despawn |
| `DespawnDelay` | `mge.foundation.despawn.v1.component.despawn_delay` | `remaining_ticks: u32` | Delai optionnel avant la suppression effective. Permet des animations de mort ou effets de sortie |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_despawn` | `mge.foundation.despawn.v1.fn.process_despawn` | 183 | DespawnMarker, DespawnDelay | DespawnDelay, World | EntityDespawned | O(d) | Decremente les delais, supprime les entites marquees quand le delai atteint 0 (ou immediatement sans delai), emet EntityDespawned |

---

## 5. Flux de donnees

```
DespawnMarker (reason) ──► DespawnDelay (remaining_ticks)
       │                          │
       ▼                          ▼
 ┌──────────────────────────────────────┐
 │          process_despawn             │  Phase 183
 │  (delay-- → remove entity si 0)     │
 └─────┬──────────────────────┬─────────┘
       │                      │
       ▼                      ▼
  World.despawn(entity)   EntityDespawned
                            (event)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `EntityDespawned` | `mge.foundation.despawn.v1.event.entity_despawned` | `entity: EntityId, reason: DespawnReason` | `process_despawn` | Systemes de scoring, statistiques, effets visuels, nettoyage references |

---

## 7. Invariants

- Une entite avec `DespawnMarker` sans `DespawnDelay` est supprimee immediatement a la Phase 183.
- Une entite avec `DespawnMarker` et `DespawnDelay` est supprimee quand `remaining_ticks` atteint 0.
- `EntityDespawned` est emis exactement une fois par entite supprimee, avant la suppression effective.
- La raison du despawn est preservee dans l'evenement pour permettre aux consommateurs de reagir differemment.
- Le despawn est differe : jamais execute en milieu de tick par un autre systeme.
- Apres Phase 183, toutes les entites sans delai restant sont supprimees du World.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Supprime les entites marquees du World | Ne marque pas les entites (responsabilite des autres plugins/gameplay) |
| Gere un delai optionnel avant suppression | Ne gere pas les durees de vie (→ lifetime) |
| Emet un evenement avec la raison du despawn | Ne gere pas les animations de mort (responsabilite renderer) |
| Centralise toute la suppression d'entites | Ne recycle pas les entites (pas de pooling) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | DespawnMarker, DespawnDelay |
| Ecrit | DespawnDelay, World (despawn) |
| Emet | EntityDespawned |
| Ne touche jamais | Lifetime, Timer, Cooldown, Transform2D, Velocity2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-despawn/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.despawn.v1, trait Plugin impl
    ├── components.rs     # DespawnMarker, DespawnDelay, DespawnReason
    ├── systems.rs        # process_despawn
    └── events.rs         # EntityDespawned
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
- [ ] 1 enum `DespawnReason` dans `components.rs`
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : despawn immediat, despawn avec delai, event emis, raison preservee, entite retiree du World
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.despawn.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.despawn.v1.component.despawn_marker","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.despawn.v1.component.despawn_delay","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.despawn.v1.event.entity_despawned","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.despawn.v1.fn.process_despawn","k":"s","d":"foundation","r":["DespawnMarker","DespawnDelay"],"w":["DespawnDelay","World"],"e":["EntityDespawned"],"p":183,"c":"O(d)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let enemy = world.spawn();
world.insert(enemy, Transform2D { x: 300.0, y: 200.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(enemy, DespawnMarker { reason: DespawnReason::Killed });
world.insert(enemy, DespawnDelay { remaining_ticks: 30 });
// Tick 1-29 : DespawnDelay.remaining_ticks decremente
// Tick 30 : EntityDespawned { entity: enemy, reason: Killed } emis
//           enemy supprime du World

let bullet = world.spawn();
world.insert(bullet, DespawnMarker { reason: DespawnReason::OutOfBounds });
// Pas de DespawnDelay → suppression immediate a la Phase 183
// → EntityDespawned { entity: bullet, reason: OutOfBounds }
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-lifetime](mge-gfp-lifetime.md) | Plugin duree de vie (marque pour despawn a l'expiration) |
| [mge-gfp-timer](mge-gfp-timer.md) | Plugin timer generique |
