# mge-idle-prestige

> @id mge.idle.prestige.v1  
> @role plugin  
> @domain idle  
> @do manage_prestige_reset_currency_and_permanent_bonuses  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-idle-prestige` |
| @id MSCM | `mge.idle.prestige.v1` |
| Domaine | idle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-idle-producer` |
| Hot path | Non (evenementiel, sur prestige) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites avec PrestigeState |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `PrestigeBonusType` | `ProductionMultiplier, StartingResources, UnlockTier, CostReduction, OfflineEfficiency` | Type de bonus permanent accorde par le prestige |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `PrestigeState` | `mge.idle.prestige.v1.component.prestige_state` | `prestige_count: u32, lifetime_earnings: f64, can_prestige: bool` | Etat global du prestige. lifetime_earnings = total gagne depuis dernier prestige |
| `PrestigeCurrency` | `mge.idle.prestige.v1.component.prestige_currency` | `amount: f64` | Monnaie de prestige accumulee a travers les resets |
| `PermanentBonus` | `mge.idle.prestige.v1.component.permanent_bonus` | `bonus_type: PrestigeBonusType, value: f64` | Bonus permanent actif apres prestige |
| `PrestigeTier` | `mge.idle.prestige.v1.component.prestige_tier` | `tier: u32, threshold: f64, currency_formula_base: f64, currency_formula_exponent: f64` | Palier de prestige. Definit le seuil et la formule de monnaie |

---

## 4. Formules

```
prestige_currency_gained = floor(currency_formula_base * (lifetime_earnings / threshold) ^ currency_formula_exponent)
can_prestige = lifetime_earnings >= threshold
```

Le prestige reinitialise : tous les Producer, ProductionRate, ResourceOutput, UpgradeState, MultiplierStack (temporaires).
Il conserve : PrestigeCurrency (cumul), PermanentBonus, PrestigeState.prestige_count.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `evaluate_prestige_readiness` | `mge.idle.prestige.v1.fn.evaluate_prestige_readiness` | 2130 | PrestigeState, PrestigeTier | PrestigeState | none | O(1) | Met a jour can_prestige selon lifetime_earnings vs threshold |
| `execute_prestige_reset` | `mge.idle.prestige.v1.fn.execute_prestige_reset` | 2131 | PrestigeState, PrestigeTier | PrestigeState, PrestigeCurrency, Producer, UpgradeState, MultiplierStack | PrestigeExecuted | O(n) | Reset toutes les entites idle. Calcule et ajoute la monnaie prestige. Incremente prestige_count |
| `apply_permanent_bonus` | `mge.idle.prestige.v1.fn.apply_permanent_bonus` | 2132 | PermanentBonus, PrestigeCurrency | MultiplierStack, Producer | PrestigeBonusGranted | O(b) | Applique les bonus permanents comme multiplicateurs source Prestige |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `PrestigeRequest` | `mge.idle.prestige.v1.event.prestige_request` | `player: EntityId` | Externe (UI, input) | `execute_prestige_reset` |
| `PrestigeExecuted` | `mge.idle.prestige.v1.event.prestige_executed` | `prestige_count: u32, currency_gained: f64, total_currency: f64` | `execute_prestige_reset` | UI, analytics, save |
| `PrestigeBonusGranted` | `mge.idle.prestige.v1.event.prestige_bonus_granted` | `bonus_type: PrestigeBonusType, value: f64` | `apply_permanent_bonus` | UI (notification bonus) |

---

## 7. Invariants

- `PrestigeState.prestige_count` est monotone croissant.
- `PrestigeCurrency.amount` est toujours >= 0.0 et monotone croissant (pas de depense en v1).
- `execute_prestige_reset` ne s'execute que si `can_prestige = true`.
- Apres un reset, `lifetime_earnings` est remis a 0.0.
- Les PermanentBonus survivent toujours au reset.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `prestige_threshold` | `f64` | 1_000_000.0 | [100.0, 1e18] | Earnings minimum pour debloquer le prestige |
| `currency_formula_base` | `f64` | 1.0 | [0.01, 1000.0] | Base de la formule de monnaie prestige |
| `currency_formula_exponent` | `f64` | 0.5 | [0.1, 2.0] | Exposant de la formule (0.5 = racine carree) |
| `bonus_production_mult_per_prestige` | `f64` | 0.05 | [0.01, 1.0] | Bonus multiplicateur de production par prestige |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Evalue si le prestige est possible | Ne genere pas de ressources (→ producer) |
| Execute le reset et reinitialise les entites | Ne calcule pas les multiplicateurs (→ multiplier) |
| Calcule et attribue la monnaie prestige | Ne gere pas l'UI de prestige (→ UI layer) |
| Applique les bonus permanents | Ne gere pas le prestige multi-couche (v2) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | PrestigeState, PrestigeTier, PermanentBonus, PrestigeCurrency |
| Ecrit | PrestigeState, PrestigeCurrency, Producer, UpgradeState, MultiplierStack |
| Emet | PrestigeExecuted, PrestigeBonusGranted |
| Ne touche jamais | OfflineState, ProductionRate (recalcule par producer) |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-idle-prestige/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.idle.prestige.v1, trait Plugin impl
    ├── components.rs     # PrestigeState, PrestigeCurrency, PermanentBonus, PrestigeTier
    ├── systems.rs        # evaluate_prestige_readiness, execute_prestige_reset, apply_permanent_bonus
    └── events.rs         # PrestigeRequest, PrestigeExecuted, PrestigeBonusGranted
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (PrestigeBonusType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : evaluation seuil, reset, monnaie, bonus permanent
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.idle.prestige.v1","k":"p","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.component.prestige_state","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.component.prestige_currency","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.component.permanent_bonus","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.component.prestige_tier","k":"d","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.fn.evaluate_prestige_readiness","k":"s","d":"idle","r":["PrestigeState","PrestigeTier"],"w":["PrestigeState"],"e":[],"p":2130,"c":"O(1)"},
  {"i":"mge.idle.prestige.v1.fn.execute_prestige_reset","k":"s","d":"idle","r":["PrestigeState","PrestigeTier"],"w":["PrestigeState","PrestigeCurrency","Producer","UpgradeState","MultiplierStack"],"e":["PrestigeExecuted"],"p":2131,"c":"O(n)"},
  {"i":"mge.idle.prestige.v1.fn.apply_permanent_bonus","k":"s","d":"idle","r":["PermanentBonus","PrestigeCurrency"],"w":["MultiplierStack","Producer"],"e":["PrestigeBonusGranted"],"p":2132,"c":"O(b)"},
  {"i":"mge.idle.prestige.v1.event.prestige_request","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.event.prestige_executed","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.idle.prestige.v1.event.prestige_bonus_granted","k":"e","d":"idle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, PrestigeState { prestige_count: 0, lifetime_earnings: 0.0, can_prestige: false });
world.insert(player, PrestigeCurrency { amount: 0.0 });
world.insert(player, PrestigeTier {
    tier: 1,
    threshold: 1_000_000.0,
    currency_formula_base: 1.0,
    currency_formula_exponent: 0.5,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Idle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
