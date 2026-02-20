# mge-idle-upgrade

> @id mge.idle.upgrade.v1  
> @role plugin  
> @domain idle  
> @do manage_purchasable_upgrades_and_effects  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-idle-upgrade` |
| @id MSCM | `mge.idle.upgrade.v1` |
| Domaine | idle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-idle-producer` |
| Hot path | Non (evenementiel, sur achat) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites UpgradeState |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `UpgradeTargetKind` | `ProducerRate, ProducerCount, MultiplierFlat, MultiplierPercent, UnlockProducer, UnlockFeature` | Type d'effet applique par l'upgrade |
| `UpgradeStatus` | `Locked, Available, Purchased, MaxLevel` | Etat courant de l'upgrade |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `UpgradeDef` | `mge.idle.upgrade.v1.component.upgrade_def` | `name_key: u32, target: UpgradeTargetKind, target_id: EntityId, value: f64, max_level: u32` | Definition statique d'un upgrade. Charge depuis GCL |
| `UpgradeState` | `mge.idle.upgrade.v1.component.upgrade_state` | `current_level: u32, status: UpgradeStatus` | Etat runtime de l'upgrade |
| `UpgradeCost` | `mge.idle.upgrade.v1.component.upgrade_cost` | `resource_type: ResourceType, base_cost: f64, scaling_factor: f64, current_cost: f64` | Cout actuel. current_cost = base_cost * scaling_factor^current_level |
| `UpgradeEffect` | `mge.idle.upgrade.v1.component.upgrade_effect` | `target: UpgradeTargetKind, target_id: EntityId, value_per_level: f64` | Effet applique a chaque achat. Cumulatif |

---

## 4. Formules

```
current_cost = base_cost * scaling_factor ^ current_level
total_effect = value_per_level * current_level
```

Le `scaling_factor` est typiquement entre 1.05 et 2.0. Plus il est eleve, plus le cout explose.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `validate_upgrade_purchase` | `mge.idle.upgrade.v1.fn.validate_upgrade_purchase` | 2110 | UpgradeState, UpgradeCost, ResourceOutput | UpgradeState, ResourceOutput | UpgradePurchased, UpgradeFailed | O(n) | Verifie que le joueur a les ressources et que l'upgrade n'est pas au max. Consomme les ressources si valide |
| `apply_upgrade_effect` | `mge.idle.upgrade.v1.fn.apply_upgrade_effect` | 2111 | UpgradeEffect, UpgradeState | Producer, Multiplier | none | O(n) | Applique l'effet de l'upgrade sur la cible (producteur ou multiplicateur) |
| `scale_upgrade_cost` | `mge.idle.upgrade.v1.fn.scale_upgrade_cost` | 2112 | UpgradeState | UpgradeCost | none | O(n) | Recalcule current_cost apres chaque achat selon la formule de scaling |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `PurchaseUpgradeRequest` | `mge.idle.upgrade.v1.event.purchase_upgrade_request` | `upgrade: EntityId, buyer: EntityId` | Externe (UI, input) | `validate_upgrade_purchase` |
| `UpgradePurchased` | `mge.idle.upgrade.v1.event.upgrade_purchased` | `upgrade: EntityId, new_level: u32, cost_paid: f64` | `validate_upgrade_purchase` | UI, analytics |
| `UpgradeFailed` | `mge.idle.upgrade.v1.event.upgrade_failed` | `upgrade: EntityId, reason: u8` | `validate_upgrade_purchase` | UI (afficher erreur) |

---

## 7. Invariants

- `UpgradeState.current_level` ne depasse jamais `UpgradeDef.max_level`.
- `UpgradeCost.current_cost` est toujours > 0.0.
- Un upgrade avec `status = Locked` ne peut pas etre achete.
- Un upgrade avec `status = MaxLevel` ne peut plus etre achete.
- L'achat est atomique : si les ressources sont insuffisantes, rien n'est modifie.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `upgrade_cost_scaling_default` | `f64` | 1.15 | [1.01, 3.0] | Facteur de scaling par defaut pour les upgrades |
| `max_upgrade_level_default` | `u32` | 100 | [1, 10_000] | Niveau max par defaut |
| `upgrade_refund_ratio` | `f64` | 0.0 | [0.0, 1.0] | Ratio de remboursement si downgrade (0 = pas de refund) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Valide les achats d'upgrades | Ne gere pas le stockage de ressources (→ producer/core) |
| Applique les effets sur producteurs/multiplicateurs | Ne calcule pas les multiplicateurs (→ multiplier) |
| Scale le cout apres chaque achat | Ne gere pas le prestige (→ prestige) |
| Gere le statut (Locked → Available → Purchased) | Ne gere pas l'UI d'achat (→ UI layer) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | UpgradeState, UpgradeCost, UpgradeDef, UpgradeEffect, ResourceOutput |
| Ecrit | UpgradeState, UpgradeCost, ResourceOutput, Producer, Multiplier |
| Emet | UpgradePurchased, UpgradeFailed |
| Ne touche jamais | PrestigeState, OfflineState, ProductionRate |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-idle-upgrade/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.idle.upgrade.v1, trait Plugin impl
    ├── components.rs     # UpgradeDef, UpgradeState, UpgradeCost, UpgradeEffect
    ├── systems.rs        # validate_upgrade_purchase, apply_upgrade_effect, scale_upgrade_cost
    └── events.rs         # PurchaseUpgradeRequest, UpgradePurchased, UpgradeFailed
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (UpgradeTargetKind, UpgradeStatus)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : achat valide, achat insuffisant, scaling cout, max level
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.idle.upgrade.v1","k":"p","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.component.upgrade_def","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.component.upgrade_state","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.component.upgrade_cost","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.component.upgrade_effect","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.fn.validate_upgrade_purchase","k":"s","d":"idle","r":["UpgradeState","UpgradeCost","ResourceOutput"],"w":["UpgradeState","ResourceOutput"],"e":["UpgradePurchased","UpgradeFailed"],"p":2110,"c":"O(n)"},
  {"i":"mge.idle.upgrade.v1.fn.apply_upgrade_effect","k":"s","d":"idle","r":["UpgradeEffect","UpgradeState"],"w":["Producer","Multiplier"],"e":[],"p":2111,"c":"O(n)"},
  {"i":"mge.idle.upgrade.v1.fn.scale_upgrade_cost","k":"s","d":"idle","r":["UpgradeState"],"w":["UpgradeCost"],"e":[],"p":2112,"c":"O(n)"},
  {"i":"mge.idle.upgrade.v1.event.purchase_upgrade_request","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.event.upgrade_purchased","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.upgrade.v1.event.upgrade_failed","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let speed_upgrade = world.spawn();
world.insert(speed_upgrade, UpgradeDef {
    name_key: 1001,
    target: UpgradeTargetKind::ProducerRate,
    target_id: gold_mine,
    value: 0.5,
    max_level: 50,
});
world.insert(speed_upgrade, UpgradeState { current_level: 0, status: UpgradeStatus::Available });
world.insert(speed_upgrade, UpgradeCost {
    resource_type: ResourceType::Gold,
    base_cost: 100.0,
    scaling_factor: 1.15,
    current_cost: 100.0,
});
world.insert(speed_upgrade, UpgradeEffect {
    target: UpgradeTargetKind::ProducerRate,
    target_id: gold_mine,
    value_per_level: 0.5,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Idle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
