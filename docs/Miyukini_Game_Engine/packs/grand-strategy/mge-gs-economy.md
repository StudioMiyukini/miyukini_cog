# mge-gs-economy

> @id mge.gs.economy.v1  
> @role plugin  
> @domain grand-strategy  
> @do manage_treasury_taxation_inflation_budget  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gs-economy` |
| @id MSCM | `mge.gs.economy.v1` |
| Domaine | grand-strategy |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (taxes et depenses calculees chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(p) ou p=provinces/sources de revenu |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `EconomicPolicy` | `Free, Regulated, Planned` | Politique economique du joueur |
| `ExpenseCategory` | `Military, Court, Infrastructure, Debt, Subsidy` | Categorie de depense budgetaire |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Treasury` | `mge.gs.economy.v1.component.treasury` | `gold: f32, income: f32, expenses: f32, balance: f32` | Tresor de la faction. balance = income - expenses par tick |
| `EconomicOutput` | `mge.gs.economy.v1.component.economic_output` | `production_value: f32, tax_rate: f32, efficiency: f32, policy: EconomicPolicy` | Sortie economique d'une province. tax_rate [0, 1] |
| `Inflation` | `mge.gs.economy.v1.component.inflation` | `rate: f32, accumulated: f32, threshold: f32` | Inflation cumulee. Effets negatifs au-dela de threshold |

---

## 4. Formules

```
tax_income      = sum(province.production_value * province.tax_rate * province.efficiency)
total_income    = tax_income + trade_income + vassal_income
total_expenses  = military_cost + court_cost + infrastructure_cost + debt_interest
balance         = total_income - total_expenses
gold_new        = gold + balance

inflation_delta = (gold_income_excess / economy_size) * inflation_rate
inflation_new   = max(inflation + inflation_delta - natural_deflation, 0.0)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `collect_taxes` | `mge.gs.economy.v1.fn.collect_taxes` | Logic (1210) | EconomicOutput | Treasury | TaxCollected | O(p) | Collecte les impots de toutes les provinces |
| `pay_expenses` | `mge.gs.economy.v1.fn.pay_expenses` | Logic (1211) | Treasury | Treasury | TreasuryBankrupt | O(e) | Deduit les depenses du tresor. Emet bankrupt si gold < 0 |
| `update_inflation` | `mge.gs.economy.v1.fn.update_inflation` | Logic (1212) | Treasury, Inflation | Inflation | InflationCrisis | O(1) | Met a jour l'inflation en fonction des revenus et depenses |
| `compute_economic_output` | `mge.gs.economy.v1.fn.compute_economic_output` | Logic (1213) | EconomicOutput, Inflation | EconomicOutput, Treasury | EconomicBoom | O(p) | Recalcule la sortie economique en appliquant inflation et politique |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TaxCollected` | `mge.gs.economy.v1.event.tax_collected` | `faction: EntityId, amount: f32` | `collect_taxes` | ui |
| `TreasuryBankrupt` | `mge.gs.economy.v1.event.treasury_bankrupt` | `faction: EntityId, deficit: f32` | `pay_expenses` | military, diplomacy, ui |
| `InflationCrisis` | `mge.gs.economy.v1.event.inflation_crisis` | `faction: EntityId, rate: f32` | `update_inflation` | population, ui |
| `EconomicBoom` | `mge.gs.economy.v1.event.economic_boom` | `faction: EntityId, growth_pct: f32` | `compute_economic_output` | population, ui |

---

## 7. Invariants

- `Treasury.gold` peut devenir negatif (dette).
- `EconomicOutput.tax_rate` est toujours dans [0.0, 1.0].
- `EconomicOutput.efficiency` est toujours dans [0.0, 2.0].
- `Inflation.accumulated` est toujours >= 0.0.
- `TreasuryBankrupt` est emis une seule fois par passage en negatif (pas chaque tick).
- `EconomicBoom` n'est emis que si la croissance depasse un seuil configurable.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `base_tax_rate` | `f32` | 0.15 | [0.0, 0.5] | Taux de taxation par defaut |
| `inflation_threshold` | `f32` | 10.0 | [1.0, 50.0] | Seuil d'inflation declenchant InflationCrisis |
| `natural_deflation` | `f32` | 0.1 | [0.0, 1.0] | Deflation naturelle par tick |
| `boom_growth_threshold` | `f32` | 5.0 | [1.0, 20.0] | Seuil de croissance pour EconomicBoom (%) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le tresor, les taxes et les depenses | Ne gere pas le commerce (→ trade) |
| Calcule l'inflation | Ne gere pas le recrutement militaire (→ military) |
| Detecte la banqueroute et les booms | Ne gere pas les provinces (→ province) |
| Supporte 3 politiques economiques | Ne gere pas les decisions economiques (→ decision) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Treasury, EconomicOutput, Inflation |
| Ecrit | Treasury, EconomicOutput, Inflation |
| Emet | TaxCollected, TreasuryBankrupt, InflationCrisis, EconomicBoom |
| Ne touche jamais | DiplomaticStance, Army, Population, Province, TradeRoute, Religion, Culture, CasusBelli |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gs-economy/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.gs.economy.v1, trait Plugin impl
    ├── components.rs     # Treasury, EconomicOutput, Inflation
    ├── systems.rs        # collect_taxes, pay_expenses, update_inflation, compute_economic_output
    └── events.rs         # TaxCollected, TreasuryBankrupt, InflationCrisis, EconomicBoom
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (collect_taxes) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (EconomicPolicy, ExpenseCategory)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : tax collection, expenses, inflation, bankruptcy
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.gs.economy.v1","k":"p","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.component.treasury","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.component.economic_output","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.component.inflation","k":"d","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.fn.collect_taxes","k":"s","d":"grand-strategy","r":["EconomicOutput"],"w":["Treasury"],"e":["TaxCollected"],"p":1210,"c":"O(p)"},
  {"i":"mge.gs.economy.v1.fn.pay_expenses","k":"s","d":"grand-strategy","r":["Treasury"],"w":["Treasury"],"e":["TreasuryBankrupt"],"p":1211,"c":"O(e)"},
  {"i":"mge.gs.economy.v1.fn.update_inflation","k":"s","d":"grand-strategy","r":["Treasury","Inflation"],"w":["Inflation"],"e":["InflationCrisis"],"p":1212,"c":"O(1)"},
  {"i":"mge.gs.economy.v1.fn.compute_economic_output","k":"s","d":"grand-strategy","r":["EconomicOutput","Inflation"],"w":["EconomicOutput","Treasury"],"e":["EconomicBoom"],"p":1213,"c":"O(p)"},
  {"i":"mge.gs.economy.v1.event.tax_collected","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.event.treasury_bankrupt","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.event.inflation_crisis","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.gs.economy.v1.event.economic_boom","k":"e","d":"grand-strategy","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let kingdom = world.spawn();
world.insert(kingdom, Treasury {
    gold: 500.0,
    income: 0.0,
    expenses: 0.0,
    balance: 0.0,
});
world.insert(kingdom, Inflation {
    rate: 0.02,
    accumulated: 0.0,
    threshold: 10.0,
});

let province_eco = world.spawn();
world.insert(province_eco, EconomicOutput {
    production_value: 100.0,
    tax_rate: 0.15,
    efficiency: 1.0,
    policy: EconomicPolicy::Regulated,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Grand Strategy - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
