# mge-idle-multiplier

> @id mge.idle.multiplier.v1  
> @role plugin  
> @domain idle  
> @do manage_multipliers_bonuses_and_global_production_scaling  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-idle-multiplier` |
| @id MSCM | `mge.idle.multiplier.v1` |
| Domaine | idle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (recalcul chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * m) n=producteurs, m=multiplicateurs actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `MultiplierSource` | `Upgrade, Prestige, Temporary, Achievement, Event` | Origine du multiplicateur |
| `MultiplierScope` | `Global, PerResource(ResourceType), PerProducer(EntityId)` | Portee d'application du multiplicateur |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Multiplier` | `mge.idle.multiplier.v1.component.multiplier` | `source: MultiplierSource, scope: MultiplierScope, value: f64, remaining_ticks: Option<u32>` | Un multiplicateur individuel. remaining_ticks = None → permanent |
| `MultiplierStack` | `mge.idle.multiplier.v1.component.multiplier_stack` | `multipliers: Vec<Multiplier>` | Pile de multiplicateurs actifs sur une entite |
| `GlobalMultiplier` | `mge.idle.multiplier.v1.component.global_multiplier` | `value: f64` | Multiplicateur global calcule. Lu par producer |

---

## 4. Formules

```
global_multiplier = produit(m.value pour m dans multipliers si scope == Global)
per_resource_mult = produit(m.value pour m dans multipliers si scope == PerResource(type))
effective_mult = global_multiplier * per_resource_mult * per_producer_mult
```

Les multiplicateurs sont multiplicatifs entre eux (pas additifs). Un multiplicateur de 1.0 est neutre.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_global_multiplier` | `mge.idle.multiplier.v1.fn.compute_global_multiplier` | 2120 | MultiplierStack | GlobalMultiplier | MultiplierChanged | O(m) | Recalcule GlobalMultiplier.value a partir de tous les multiplicateurs scope Global |
| `apply_multiplier_to_production` | `mge.idle.multiplier.v1.fn.apply_multiplier_to_production` | 2121 | MultiplierStack, GlobalMultiplier | GlobalMultiplier | none | O(n*m) | Combine multiplicateurs per-resource et per-producer avec le global |
| `expire_temporary_multipliers` | `mge.idle.multiplier.v1.fn.expire_temporary_multipliers` | 2122 | MultiplierStack | MultiplierStack | MultiplierExpired | O(m) | Decremente remaining_ticks. Supprime les multiplicateurs expires |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `MultiplierChanged` | `mge.idle.multiplier.v1.event.multiplier_changed` | `new_global: f64, old_global: f64` | `compute_global_multiplier` | UI (affichage bonus), analytics |
| `MultiplierExpired` | `mge.idle.multiplier.v1.event.multiplier_expired` | `source: MultiplierSource, scope: MultiplierScope, value: f64` | `expire_temporary_multipliers` | UI (notification expiration) |

---

## 7. Invariants

- `GlobalMultiplier.value` est toujours >= `multiplier_floor` (defaut 0.01, jamais 0).
- `GlobalMultiplier.value` ne depasse jamais `multiplier_cap`.
- Un multiplicateur permanent (`remaining_ticks = None`) n'est jamais supprime par `expire_temporary_multipliers`.
- La pile `MultiplierStack.multipliers` ne depasse jamais `max_multipliers`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `multiplier_cap` | `f64` | 1_000_000.0 | [1.0, 1e18] | Plafond du multiplicateur global |
| `multiplier_floor` | `f64` | 0.01 | [0.001, 1.0] | Plancher du multiplicateur global (evite 0) |
| `max_multipliers` | `u32` | 64 | [1, 256] | Nombre max de multiplicateurs empiles |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Calcule le multiplicateur global et par portee | Ne genere pas de ressources (→ producer) |
| Expire les multiplicateurs temporaires | Ne cree pas les multiplicateurs (le code appelant ecrit MultiplierStack) |
| Emet MultiplierChanged et MultiplierExpired | Ne gere pas les upgrades (→ upgrade) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | MultiplierStack |
| Ecrit | GlobalMultiplier, MultiplierStack |
| Emet | MultiplierChanged, MultiplierExpired |
| Ne touche jamais | Producer, UpgradeState, PrestigeState, OfflineState |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-idle-multiplier/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.idle.multiplier.v1, trait Plugin impl
    ├── components.rs     # Multiplier, MultiplierStack, GlobalMultiplier
    ├── systems.rs        # compute_global_multiplier, apply_multiplier_to_production, expire_temporary_multipliers
    └── events.rs         # MultiplierChanged, MultiplierExpired
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (MultiplierSource, MultiplierScope)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : calcul global, expiration, plafond, plancher
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.idle.multiplier.v1","k":"p","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.multiplier.v1.component.multiplier","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.multiplier.v1.component.multiplier_stack","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.multiplier.v1.component.global_multiplier","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.multiplier.v1.fn.compute_global_multiplier","k":"s","d":"idle","r":["MultiplierStack"],"w":["GlobalMultiplier"],"e":["MultiplierChanged"],"p":2120,"c":"O(m)"},
  {"i":"mge.idle.multiplier.v1.fn.apply_multiplier_to_production","k":"s","d":"idle","r":["MultiplierStack","GlobalMultiplier"],"w":["GlobalMultiplier"],"e":[],"p":2121,"c":"O(n*m)"},
  {"i":"mge.idle.multiplier.v1.fn.expire_temporary_multipliers","k":"s","d":"idle","r":["MultiplierStack"],"w":["MultiplierStack"],"e":["MultiplierExpired"],"p":2122,"c":"O(m)"},
  {"i":"mge.idle.multiplier.v1.event.multiplier_changed","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.multiplier.v1.event.multiplier_expired","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let mult_entity = world.spawn();
world.insert(mult_entity, MultiplierStack {
    multipliers: vec![
        Multiplier { source: MultiplierSource::Prestige, scope: MultiplierScope::Global, value: 2.0, remaining_ticks: None },
        Multiplier { source: MultiplierSource::Temporary, scope: MultiplierScope::Global, value: 1.5, remaining_ticks: Some(600) },
    ],
});
world.insert(mult_entity, GlobalMultiplier { value: 1.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Idle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
