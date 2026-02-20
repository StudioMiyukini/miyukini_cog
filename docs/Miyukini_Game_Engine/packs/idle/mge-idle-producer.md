# mge-idle-producer

> @id mge.idle.producer.v1  
> @role plugin  
> @domain idle  
> @do manage_automatic_producers_and_resource_generation  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-idle-producer` |
| @id MSCM | `mge.idle.producer.v1` |
| Domaine | idle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-idle-multiplier` |
| Hot path | Oui (tick chaque frame) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites Producer |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ResourceType` | `Gold, Gem, Wood, Stone, Food, Energy, Custom(u32)` | Type de ressource generee par un producteur |
| `ProducerTier` | `Tier1, Tier2, Tier3, Tier4, Tier5` | Palier du producteur, influence le cout et le rendement de base |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Producer` | `mge.idle.producer.v1.component.producer` | `tier: ProducerTier, resource_type: ResourceType, base_rate: f64, enabled: bool` | Definition d'un producteur. base_rate = unites/tick avant multiplicateurs |
| `ProducerCount` | `mge.idle.producer.v1.component.producer_count` | `count: u64` | Nombre d'instances du producteur. Multiplie la production |
| `ProductionRate` | `mge.idle.producer.v1.component.production_rate` | `effective_rate: f64` | Taux effectif apres application multiplicateurs. Recalcule chaque tick |
| `ResourceOutput` | `mge.idle.producer.v1.component.resource_output` | `resource_type: ResourceType, accumulated: f64, total_produced: f64` | Ressources accumulees non recoltees et total historique |

---

## 4. Formules

```
effective_rate = base_rate * count * global_multiplier
accumulated += effective_rate * dt
```

Le `global_multiplier` provient de `mge-idle-multiplier`. Si absent, vaut 1.0.

`dt` est le delta time du tick (en secondes pour temps reel, ou 1.0 pour tick fixe).

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_producers` | `mge.idle.producer.v1.fn.tick_producers` | 2100 | Producer, ProducerCount, GlobalMultiplier | ProductionRate | none | O(n) | Recalcule effective_rate pour chaque producteur actif (enabled = true) |
| `accumulate_resources` | `mge.idle.producer.v1.fn.accumulate_resources` | 2101 | ProductionRate | ResourceOutput | ResourceProduced | O(n) | Ajoute effective_rate * dt a accumulated et total_produced. Emet ResourceProduced |
| `apply_production_rate` | `mge.idle.producer.v1.fn.apply_production_rate` | 2102 | Producer, ProducerCount | ProductionRate | none | O(n) | Recalcule le taux de base sans multiplicateurs (appele apres achat de producteur) |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ResourceProduced` | `mge.idle.producer.v1.event.resource_produced` | `producer: EntityId, resource_type: ResourceType, amount: f64` | `accumulate_resources` | UI, upgrade (validation cout), prestige (evaluation) |

---

## 7. Invariants

- `ProductionRate.effective_rate` est toujours >= 0.0 apres `tick_producers`.
- `ResourceOutput.accumulated` est toujours >= 0.0.
- `ResourceOutput.total_produced` est monotone croissant.
- Un producteur avec `enabled = false` ne contribue jamais a la production.
- `ProducerCount.count` est toujours >= 0 (u64).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `base_rate_tier1` | `f64` | 1.0 | [0.001, 1_000_000.0] | Production de base Tier1/tick |
| `base_rate_tier2` | `f64` | 8.0 | [0.001, 1_000_000.0] | Production de base Tier2/tick |
| `base_rate_tier3` | `f64` | 47.0 | [0.001, 1_000_000.0] | Production de base Tier3/tick |
| `base_rate_tier4` | `f64` | 260.0 | [0.001, 1_000_000.0] | Production de base Tier4/tick |
| `base_rate_tier5` | `f64` | 1400.0 | [0.001, 1_000_000.0] | Production de base Tier5/tick |
| `producer_cost_base` | `f64` | 10.0 | [1.0, 1e12] | Cout de base pour l'achat du premier producteur |
| `producer_cost_scaling` | `f64` | 1.15 | [1.01, 2.0] | Facteur multiplicatif du cout par unite achetee |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Genere des ressources automatiquement | Ne gere pas le stockage global de ressources (→ Core) |
| Calcule le taux effectif avec multiplicateurs | Ne calcule pas les multiplicateurs (→ multiplier) |
| Emet ResourceProduced | Ne gere pas les achats (→ upgrade) |
| Gere le compteur d'instances | Ne gere pas le prestige (→ prestige) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Producer, ProducerCount, GlobalMultiplier |
| Ecrit | ProductionRate, ResourceOutput |
| Emet | ResourceProduced |
| Ne touche jamais | UpgradeState, PrestigeState, OfflineState, Multiplier |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-idle-producer/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.idle.producer.v1, trait Plugin impl
    ├── components.rs     # Producer, ProducerCount, ProductionRate, ResourceOutput
    ├── systems.rs        # tick_producers, accumulate_resources, apply_production_rate
    └── events.rs         # ResourceProduced
```

### Annotations MSCM requises

**lib.rs** :
```rust
//! @id mge.idle.producer.v1
//! @role plugin
//! @layer plugin
//! @domain idle
//! @do manage_automatic_producers_and_resource_generation
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (ResourceType, ProducerTier)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : tick production, accumulation, taux effectif
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.idle.producer.v1","k":"p","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.producer.v1.component.producer","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.producer.v1.component.producer_count","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.producer.v1.component.production_rate","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.producer.v1.component.resource_output","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.producer.v1.fn.tick_producers","k":"s","d":"idle","r":["Producer","ProducerCount","GlobalMultiplier"],"w":["ProductionRate"],"e":[],"p":2100,"c":"O(n)"},
  {"i":"mge.idle.producer.v1.fn.accumulate_resources","k":"s","d":"idle","r":["ProductionRate"],"w":["ResourceOutput"],"e":["ResourceProduced"],"p":2101,"c":"O(n)"},
  {"i":"mge.idle.producer.v1.fn.apply_production_rate","k":"s","d":"idle","r":["Producer","ProducerCount"],"w":["ProductionRate"],"e":[],"p":2102,"c":"O(n)"},
  {"i":"mge.idle.producer.v1.event.resource_produced","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let gold_mine = world.spawn();
world.insert(gold_mine, Producer { tier: ProducerTier::Tier1, resource_type: ResourceType::Gold, base_rate: 1.0, enabled: true });
world.insert(gold_mine, ProducerCount { count: 5 });
world.insert(gold_mine, ProductionRate { effective_rate: 0.0 });
world.insert(gold_mine, ResourceOutput { resource_type: ResourceType::Gold, accumulated: 0.0, total_produced: 0.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Idle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
