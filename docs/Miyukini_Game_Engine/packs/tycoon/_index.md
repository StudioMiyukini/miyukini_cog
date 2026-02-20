# MGE — Pack Tycoon

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/tycoon/`  
**Nombre de crates** : 4  

---

## 1. Contexte

Le Pack Tycoon couvre les mecaniques des jeux de gestion/tycoon : installations, employes, clients et revenus. Il modelise une economie de services ou le joueur construit et optimise une entreprise. Il s'associe au Pack Social pour les besoins et horaires des PNJ.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Tycoon, simulation economique, gestion d'entreprise, business sim.
- **Hors portee** : Rendu, UI, logique de jeu specifique, simulation de ville (→ Pack Sandbox).
- **Audience** : Developpeurs moteur, designers, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack recommande. Pack Social recommande (need, schedule).

---

## 3. Vision

Le Pack Tycoon est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/tycoon/
├── mge-tycoon-facility/    # Installations, capacite, maintenance
├── mge-tycoon-employee/    # Employes, competences, salaires
├── mge-tycoon-customer/    # Clients, arrivee, satisfaction, depenses
└── mge-tycoon-revenue/     # Revenus, couts, profit, transactions
```

### Graphe de dependances intra-pack

```
mge-tycoon-customer ──────► mge-tycoon-revenue
      │
mge-tycoon-employee ──────► mge-tycoon-facility
                                   │
                                   └──► mge-tycoon-revenue
```

Crates feuilles (sans dependance intra-pack) : `mge-tycoon-revenue`.

---

## 5. Sous-packs

Aucun. Les 4 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-tycoon-facility` | `mge.tycoon.facility.v1` | [mge-tycoon-facility.md](mge-tycoon-facility.md) | Installations, capacite, niveaux, maintenance |
| 2 | `mge-tycoon-employee` | `mge.tycoon.employee.v1` | [mge-tycoon-employee.md](mge-tycoon-employee.md) | Employes, competences, salaires, affectation |
| 3 | `mge-tycoon-customer` | `mge.tycoon.customer.v1` | [mge-tycoon-customer.md](mge-tycoon-customer.md) | Clients, arrivee, satisfaction, depenses |
| 4 | `mge-tycoon-revenue` | `mge.tycoon.revenue.v1` | [mge-tycoon-revenue.md](mge-tycoon-revenue.md) | Revenus, couts, profit, bilan |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|--------------------|------------------------------|
| facility | Facility, FacilityLevel, Capacity, MaintenanceCost, FacilityState | FacilityDef |
| employee | Employee, EmployeeSkill, Wage, Efficiency, Assignment | EmployeeRoleDef |
| customer | Customer, Satisfaction, Spending, ArrivalTimer, ServiceQueue | CustomerProfileDef |
| revenue | Revenue, Expense, Profit, Transaction, Budget | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 2200-2203 | facility | tick_maintenance, apply_degradation, process_facility_upgrade, update_capacity |
| 2210-2213 | employee | assign_employees, compute_efficiency, process_payroll, evaluate_employee_satisfaction |
| 2220-2223 | customer | spawn_customers, process_service_queue, update_satisfaction, process_departure |
| 2230-2233 | revenue | record_transaction, compute_revenue, compute_expenses, compute_profit |

**Ordre d'execution** : facility (2200) → employee (2210) → customer (2220) → revenue (2230).

**Justification** : les installations definissent la capacite. Les employes s'y affectent et determinent l'efficacite. Les clients arrivent et sont servis. Les revenus sont calcules en dernier a partir des transactions.

**Total** : 16 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| facility | UpgradeFacilityRequest | FacilityUpgraded, FacilityDegraded, FacilityBroken |
| employee | HireEmployeeRequest, FireEmployeeRequest | EmployeeHired, EmployeeFired, EmployeeAssigned |
| customer | (aucun, spawn automatique) | CustomerArrived, CustomerServed, CustomerLeft, CustomerUnsatisfied |
| revenue | (aucun, calcul automatique) | TransactionRecorded, ProfitCalculated, BudgetExceeded |

**Total** : 3 requests + 10 events = 13 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 4 crates | `mge-ecs`, `mge-event` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-tycoon-employee` | `mge-tycoon-facility` |
| `mge-tycoon-customer` | `mge-tycoon-revenue` |
| `mge-tycoon-facility` | `mge-tycoon-revenue` |

### Dependances inter-pack (optionnelles)

| Crate | Depend de (optionnel) |
|-------|----------------------|
| `mge-tycoon-employee` | Pack Social (`Need`, `Schedule`) |
| `mge-tycoon-customer` | Pack Social (`Need`, `Schedule`) |

### Dependances externes (aucune)

Le Pack Tycoon n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

**Parametres exposables :**

- Taux d'arrivee des clients, profils clients
- Salaires de base, echelle competences
- Couts de maintenance, taux de degradation
- Formule de satisfaction client
- Budget initial, seuils d'alerte

---

## 12. Exemple d'assemblage

### Minimal (headless, facility + revenue)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeTycoonFacilityPlugin);
engine.add_plugin(MgeTycoonRevenuePlugin);
engine.build();
```

### Complet (Tycoon jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginSaveLoad::default());
// Pack Social (optionnel)
engine.add_plugin(MgeSocialNeedPlugin);
engine.add_plugin(MgeSocialSchedulePlugin);
// Pack Tycoon
engine.add_plugin(MgeTycoonFacilityPlugin);
engine.add_plugin(MgeTycoonEmployeePlugin);
engine.add_plugin(MgeTycoonCustomerPlugin);
engine.add_plugin(MgeTycoonRevenuePlugin);
engine.build();
```

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
