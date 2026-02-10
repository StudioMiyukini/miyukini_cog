# JayKonta - Mocks UI et Guide d'Implementation

## Contexte

Ce document fournit les **mocks ASCII** de tous les ecrans JayKonta (Purse et Account) et le **guide d'implementation** complet pour debloquer la Phase 2.2 du Parcours de Developpement. Il sert de reference pour l'implementation UI Dioxus et le schema de persistance KindMother.

**Blocage resolu** : Phase 2.2 du Parcours Developpement Purse (wireframes UI non disponibles).

## Portee / Scope

| Inclus | Exclu |
|--------|-------|
| Mocks ASCII tous ecrans Purse (P1–P6) | Maquettes haute-fidelite (MiyuLayoutBuilder) |
| Mocks ASCII tous ecrans Account (A1–A7) | Implementation code finale |
| Schemas SQL complets KindMother | Gamification (Phase 4) |
| Guide implementation par phase | Tests (Phase 3.2) |
| Composants Dioxus cibles | OCR / import bancaire (hors scope Phase 1) |

---

## Table des matieres

1. [Selection du point d'entree](#1-selection-du-point-dentree)
2. [Purse — Mocks ecrans](#2-purse--mocks-ecrans)
3. [Account — Mocks ecrans](#3-account--mocks-ecrans)
4. [Navigation et layouts](#4-navigation-et-layouts)
5. [Schema SQL KindMother complet](#5-schema-sql-kindmother-complet)
6. [Types Rust et services](#6-types-rust-et-services)
7. [Composants Dioxus](#7-composants-dioxus)
8. [Guide d'implementation par phase](#8-guide-dimplementation-par-phase)
9. [Checklist de conformite](#9-checklist-de-conformite)

---

## 1. Selection du point d'entree

### 1.1 Ecran de choix Purse / Account

```
┌─────────────────────────────────────────────────────────────────────┐
│                         JayKonta                                    │
│                  Comptabilite multi-echelle                          │
│                                                                     │
│   Bienvenue, [Nom Utilisateur]                                      │
│                                                                     │
│   Choisissez votre espace :                                         │
│                                                                     │
│   ┌─────────────────────────────┐  ┌─────────────────────────────┐  │
│   │                             │  │                             │  │
│   │     💰 JayBudget            │  │     📊 JayKonta             │  │
│   │        (Purse)              │  │        (Account)            │  │
│   │                             │  │                             │  │
│   │   Budget personnel          │  │   Comptabilite entreprise   │  │
│   │   Depenses quotidiennes     │  │   Devis et factures         │  │
│   │   Objectifs d'epargne       │  │   Rapports legaux           │  │
│   │   Budgets occasionnels      │  │   Integrations services     │  │
│   │                             │  │                             │  │
│   │       [ Acceder ]           │  │       [ Acceder ]           │  │
│   │                             │  │                             │  │
│   └─────────────────────────────┘  └─────────────────────────────┘  │
│                                                                     │
│   Vous pouvez changer d'espace a tout moment via le menu.           │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Contrats** : CK-SVC-01, CK-SVC-02, CK-TK-01

---

## 2. Purse — Mocks ecrans

### 2.1 P1 — Dashboard Purse

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayBudget (Purse)                    [Nom] ▾  ⚙  🔔           │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ ■ Tableau│  ┌──────────────────────────────────────────────────┐    │
│   de bord│  │ SOLDE ACTUEL                                     │    │
│          │  │                                                  │    │
│ □ Mouvem.│  │    2 847,35 EUR                                  │    │
│          │  │    ▲ +245,00 ce mois   (vs -1 203,50 depenses)   │    │
│ □ Budgets│  │                                                  │    │
│   occas. │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│ □ Object.│  ┌────────────────────────┐  ┌────────────────────────┐  │
│          │  │ DEPENSES CE MOIS       │  │ BUDGET RESTANT         │  │
│ □ Rappo. │  │                        │  │                        │  │
│          │  │  1 203,50 EUR          │  │   796,50 EUR           │  │
│ □ Alertes│  │  ████████░░ 60%        │  │   ████░░░░░░ 40%       │  │
│          │  │  du budget mensuel     │  │   sur 2 000 EUR/mois   │  │
│ ─────────│  │                        │  │                        │  │
│ □ Param. │  └────────────────────────┘  └────────────────────────┘  │
│          │                                                          │
│ ◇ Account│  ┌──────────────────────────────────────────────────┐    │
│          │  │ REPARTITION PAR CATEGORIE                        │    │
│          │  │                                                  │    │
│          │  │  Alimentation   ████████████░░░░  420,00   35%   │    │
│          │  │  Transport      ██████░░░░░░░░░░  180,00   15%   │    │
│          │  │  Logement       ████████████████  600,00   50%   │    │
│          │  │  Loisirs        ██░░░░░░░░░░░░░░   3,50    0%   │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ DERNIERS MOUVEMENTS                              │    │
│          │  │                                                  │    │
│          │  │  10/02  Courses Carrefour    Alimentation  -42,30│    │
│          │  │  09/02  Metro               Transport     -3,80 │    │
│          │  │  08/02  Salaire             Revenus     +2 500,00│    │
│          │  │  07/02  EDF                 Logement     -95,40 │    │
│          │  │  06/02  Restaurant midi     Alimentation  -15,50│    │
│          │  │                                                  │    │
│          │  │  [ Voir tous les mouvements ]                    │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ OBJECTIFS EN COURS                       2 actifs│    │
│          │  │                                                  │    │
│          │  │  Vacances ete     ████████░░░░  800/1200 EUR 67% │    │
│          │  │  Fonds urgence    ██████░░░░░░  3000/5000 EUR 60%│    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ BUDGETS OCCASIONNELS ACTIFS               1 actif│    │
│          │  │                                                  │    │
│          │  │  Noel 2026        ██████░░░░░░  120/200 EUR  60% │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │          [ + Nouveau mouvement ]                         │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : SoldeCard, DepensesBudgetCards, CategorieChart, MouvementsList (5 derniers), ObjectifsResume, BudgetsResume, QuickAddButton
**Contrats** : CK-OP-01, CK-TK-51
**Performance** : < 3s (NFR-PUR-04)

---

### 2.2 P2 — Formulaire mouvement rapide

```
┌─────────────────────────────────────────────────────────────────────┐
│ ← Retour                    Nouveau mouvement                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   Type :  ( ● Depense )  ( ○ Revenu )                               │
│                                                                     │
│   ┌─────────────────────────────────────────────────────────────┐   │
│   │ Montant *                                                   │   │
│   │                                                             │   │
│   │              42,30 EUR                                      │   │
│   │                                                             │   │
│   └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│   ┌─────────────────────────────────────────────────────────────┐   │
│   │ Categorie *                              [ Alimentation ▾ ] │   │
│   └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│   Categories rapides :                                              │
│   ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐              │
│   │ 🍕 Alim. │ │ 🚌 Transp│ │ 🏠 Logem.│ │ 🎮 Loisir│              │
│   └──────────┘ └──────────┘ └──────────┘ └──────────┘              │
│   ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐              │
│   │ 🏥 Sante │ │ 👕 Vetm. │ │ 📱 Tech  │ │ ➕ Autre │              │
│   └──────────┘ └──────────┘ └──────────┘ └──────────┘              │
│                                                                     │
│   ┌─────────────────────────────────────────────────────────────┐   │
│   │ Description (optionnel)                                     │   │
│   │ Courses Carrefour                                           │   │
│   └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│   ┌─────────────────────────────────────────────────────────────┐   │
│   │ Date                                           [ 10/02/26 ]│   │
│   └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│   Budget occasionnel :  [ Aucun ▾ ]                                 │
│                                                                     │
│                                                                     │
│   ┌─────────────────────────────────────────────────────────────┐   │
│   │                    [ Enregistrer ]                           │   │
│   └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│   ⚡ Saisie rapide : max 3 actions (montant → categorie → valider)  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Composants** : TypeToggle, AmountInput, CategoryPicker (grid rapide + dropdown), DescriptionInput, DatePicker, BudgetSelector, SubmitButton
**Contrats** : CK-OP-02, CK-TK-11, CK-AUD-01
**Performance** : < 2s saisie complete (NFR-PUR-05), max 3 actions (NFR-PUR-06)

---

### 2.3 P2 — Liste des mouvements

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayBudget > Mouvements                         🔍  [Filtres ▾] │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Filtres actifs : Fevrier 2026 │ Toutes categories       │
│   de bord│                                                          │
│          │  ┌─────────────────────────────────────────────────────┐  │
│ ■ Mouvem.│  │ Periode: [Fev 2026 ▾]  Cat: [Toutes ▾]  Type: [⊕⊖]│  │
│          │  └─────────────────────────────────────────────────────┘  │
│ □ Budgets│                                                          │
│          │  Solde debut de mois : 1 550,85 EUR                      │
│ □ Object.│  Solde actuel :        2 847,35 EUR (+1 296,50)          │
│          │                                                          │
│ □ Rappo. │  ┌──────┬───────────────────┬──────────────┬───────────┐ │
│          │  │ Date │ Description       │ Categorie    │   Montant │ │
│          │  ├──────┼───────────────────┼──────────────┼───────────┤ │
│          │  │10/02 │Courses Carrefour  │ Alimentation │   -42,30  │ │
│          │  │09/02 │Metro              │ Transport    │    -3,80  │ │
│          │  │08/02 │Salaire            │ Revenus      │+2 500,00  │ │
│          │  │07/02 │EDF                │ Logement     │   -95,40  │ │
│          │  │06/02 │Restaurant midi    │ Alimentation │   -15,50  │ │
│          │  │05/02 │Livres             │ Loisirs      │   -23,90  │ │
│          │  │04/02 │Pharmacie          │ Sante        │   -12,50  │ │
│          │  │03/02 │Abonnement metro   │ Transport    │   -75,00  │ │
│          │  │02/02 │Loyer              │ Logement     │  -850,00  │ │
│          │  │01/02 │Virement epargne   │ Epargne      │  -200,00  │ │
│          │  └──────┴───────────────────┴──────────────┴───────────┘ │
│          │                                                          │
│          │  ◀ Page 1 / 3 ▶                                         │
│          │                                                          │
│          │  Totaux : Revenus +2 500,00 │ Depenses -1 318,40        │
│          │           Solde net : +1 181,60                          │
│          │                                                          │
│          │            [ + Nouveau mouvement ]                       │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : FilterBar (periode, categorie, type), MouvementsTable (paginee), TotauxSummary, Pagination
**Contrats** : CK-TK-11

---

### 2.4 P3 — Budgets occasionnels

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayBudget > Budgets occasionnels                  [ + Creer ]   │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  ┌─────────────────────────────────────────────────────┐  │
│          │  │ ■ ACTIFS (2)                                        │  │
│ □ Mouvem.│  │                                                     │  │
│          │  │  ┌───────────────────────────────────────────────┐   │  │
│ ■ Budgets│  │  │ 🎄 Noel 2026                                 │   │  │
│          │  │  │                                               │   │  │
│ □ Object.│  │  │  Depense : 120,00 / 200,00 EUR               │   │  │
│          │  │  │  ████████████░░░░░░░░  60%                    │   │  │
│ □ Rappo. │  │  │                                               │   │  │
│          │  │  │  Restant : 80,00 EUR                          │   │  │
│          │  │  │  Mouvements : 4 │ Debut : 01/11 │ Fin : 25/12│   │  │
│          │  │  │                                               │   │  │
│          │  │  │  [ Voir detail ]  [ Ajouter depense ]         │   │  │
│          │  │  └───────────────────────────────────────────────┘   │  │
│          │  │                                                     │  │
│          │  │  ┌───────────────────────────────────────────────┐   │  │
│          │  │  │ ✈️ Vacances Japon 2026                        │   │  │
│          │  │  │                                               │   │  │
│          │  │  │  Depense : 450,00 / 3 000,00 EUR              │   │  │
│          │  │  │  ███░░░░░░░░░░░░░░░░░  15%                    │   │  │
│          │  │  │                                               │   │  │
│          │  │  │  Restant : 2 550,00 EUR                       │   │  │
│          │  │  │  Mouvements : 2 │ Debut : 15/01 │ Fin : 01/08│   │  │
│          │  │  │                                               │   │  │
│          │  │  │  [ Voir detail ]  [ Ajouter depense ]         │   │  │
│          │  │  └───────────────────────────────────────────────┘   │  │
│          │  │                                                     │  │
│          │  │ ■ TERMINES (1)                                      │  │
│          │  │                                                     │  │
│          │  │  ┌───────────────────────────────────────────────┐   │  │
│          │  │  │ 🎂 Anniversaire Lisa     CLOS ✓               │   │  │
│          │  │  │  Depense : 85,00 / 100,00 EUR   (sous budget)│   │  │
│          │  │  └───────────────────────────────────────────────┘   │  │
│          │  │                                                     │  │
│          │  └─────────────────────────────────────────────────────┘  │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

#### Detail budget occasionnel

```
┌─────────────────────────────────────────────────────────────────────┐
│ ← Budgets    🎄 Noel 2026                        [ Modifier ] [✕]  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Budget : 200,00 EUR │ Depense : 120,00 EUR │ Restant : 80,00 EUR  │
│  ████████████░░░░░░░░  60%                                          │
│                                                                     │
│  Periode : 01/11/2025 → 25/12/2025                                 │
│  Statut : Actif                                                     │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ MOUVEMENTS AFFECTES                                         │   │
│  │                                                             │   │
│  │  08/12  Jouets Amazon           -45,00 EUR                  │   │
│  │  02/12  Papier cadeau Cultura   -12,50 EUR                  │   │
│  │  25/11  Chocolats artisan       -35,00 EUR                  │   │
│  │  15/11  Decorations sapin       -27,50 EUR                  │   │
│  │                                                             │   │
│  │  Total : -120,00 EUR (4 mouvements)                         │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  [ + Ajouter depense ]  [ Clore le budget ]  [ Exporter ]          │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Composants** : BudgetCard, ProgressBar, BudgetDetail, BudgetMovementsList
**Contrats** : CK-OP-03, CK-TK-61

---

### 2.5 P4 — Objectifs

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayBudget > Objectifs                             [ + Creer ]   │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  ┌─────────────────────────────────────────────────────┐  │
│          │  │ ■ EN COURS (2)                                      │  │
│ □ Mouvem.│  │                                                     │  │
│          │  │  ┌───────────────────────────────────────────────┐   │  │
│ □ Budgets│  │  │ ✈️ Vacances ete 2026                          │   │  │
│          │  │  │                                               │   │  │
│ ■ Object.│  │  │  Type : Epargne                               │   │  │
│          │  │  │  Cible : 1 200,00 EUR │ Actuel : 800,00 EUR   │   │  │
│ □ Rappo. │  │  │  ████████████████░░░░░░░░  67%                │   │  │
│          │  │  │  Echeance : 01/07/2026                        │   │  │
│          │  │  │  Rythme : 200,00 EUR/mois restant             │   │  │
│          │  │  │  Statut : ✓ En bonne voie                     │   │  │
│          │  │  │                                               │   │  │
│          │  │  │  Historique epargne :                          │   │  │
│          │  │  │  Jan ██  Fev ██  Mar ──  Avr ──  Mai ──  Juin──│   │  │
│          │  │  │                                               │   │  │
│          │  │  │  [ Voir detail ]  [ Ajouter versement ]       │   │  │
│          │  │  └───────────────────────────────────────────────┘   │  │
│          │  │                                                     │  │
│          │  │  ┌───────────────────────────────────────────────┐   │  │
│          │  │  │ 🏦 Fonds urgence                              │   │  │
│          │  │  │                                               │   │  │
│          │  │  │  Type : Epargne long terme                    │   │  │
│          │  │  │  Cible : 5 000,00 EUR │ Actuel : 3 000,00 EUR│   │  │
│          │  │  │  ████████████░░░░░░░░  60%                    │   │  │
│          │  │  │  Echeance : 31/12/2026                        │   │  │
│          │  │  │  Statut : ⚠️ Leger retard                     │   │  │
│          │  │  │                                               │   │  │
│          │  │  │  [ Voir detail ]  [ Ajouter versement ]       │   │  │
│          │  │  └───────────────────────────────────────────────┘   │  │
│          │  │                                                     │  │
│          │  │ ■ ATTEINTS (1) ✓                                    │  │
│          │  │                                                     │  │
│          │  │  ┌───────────────────────────────────────────────┐   │  │
│          │  │  │ 📱 Nouvel iPhone    ████████████████████ 100% │   │  │
│          │  │  │  Cible : 900 EUR │ Epargne : 900 EUR  ATTEINT│   │  │
│          │  │  └───────────────────────────────────────────────┘   │  │
│          │  │                                                     │  │
│          │  └─────────────────────────────────────────────────────┘  │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : GoalCard, GoalProgressBar, GoalHistoryMini, GoalDetail
**Contrats** : CK-TK-61

---

### 2.6 P5 — Rapports et export

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayBudget > Rapports                                            │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Periode : [ Fevrier 2026 ▾ ]  [ Comparer : Janvier ▾ ] │
│          │                                                          │
│ □ Mouvem.│  ┌──────────────────────────────────────────────────┐    │
│          │  │ SYNTHESE MENSUELLE                               │    │
│ □ Budgets│  │                                                  │    │
│          │  │  Revenus :       +2 500,00 EUR                   │    │
│ □ Object.│  │  Depenses :      -1 203,50 EUR                   │    │
│          │  │  ────────────────────────────                    │    │
│ ■ Rappo. │  │  Solde net :     +1 296,50 EUR                   │    │
│          │  │                                                  │    │
│          │  │  vs Janvier : ▲ +340,00 EUR (+35%)               │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ DEPENSES PAR CATEGORIE                           │    │
│          │  │                                                  │    │
│          │  │  Categorie    │ Ce mois  │ Mois prec. │ Variation│    │
│          │  │  ─────────────┼──────────┼────────────┼──────────│    │
│          │  │  Logement     │  945,40  │   945,40   │    0%    │    │
│          │  │  Alimentation │  135,30  │   198,50   │  -32%    │    │
│          │  │  Transport    │   78,80  │    82,00   │   -4%    │    │
│          │  │  Sante        │   12,50  │    45,00   │  -72%    │    │
│          │  │  Loisirs      │   23,90  │    67,00   │  -64%    │    │
│          │  │  Epargne      │  200,00  │   200,00   │    0%    │    │
│          │  │  ─────────────┼──────────┼────────────┼──────────│    │
│          │  │  TOTAL        │ 1 203,50 │ 1 537,90   │  -22%    │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ EVOLUTION 6 MOIS                                 │    │
│          │  │                                                  │    │
│          │  │  2000 ┤                                          │    │
│          │  │  1500 ┤        ██                                │    │
│          │  │  1000 ┤  ██    ██  ██  ██  ██                    │    │
│          │  │   500 ┤  ██ ██ ██  ██  ██  ██                    │    │
│          │  │     0 ┼──Sept─Oct─Nov─Dec─Jan─Fev──              │    │
│          │  │         ■ Depenses  □ Revenus                    │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  [ Exporter PDF ]  [ Exporter CSV ]                      │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : SyntheseMensuelle, CategorieComparison, EvolutionChart, ExportButtons
**Contrats** : CK-TK-51, CK-AUD-02

---

### 2.7 P6 — Alertes et rappels

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayBudget > Alertes et rappels                                   │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  ┌──────────────────────────────────────────────────┐    │
│          │  │ ALERTES ACTIVES                          3 actives│    │
│ □ Mouvem.│  │                                                  │    │
│          │  │  ⚠️ Depenses Alimentation a 85% du seuil mensuel │    │
│ □ Budgets│  │     Seuil : 500 EUR │ Actuel : 420 EUR           │    │
│          │  │     Configuree le : 01/01/2026                   │    │
│ □ Object.│  │                                                  │    │
│          │  │  ⚠️ Objectif Fonds urgence en leger retard       │    │
│ ■ Alertes│  │     Cible : 5000 EUR au 31/12 │ Actuel : 3000   │    │
│          │  │     Rythme necessaire : 250 EUR/mois             │    │
│ □ Rappo. │  │                                                  │    │
│          │  │  ✓  Budget Noel 2026 a 60% (normal)              │    │
│          │  │     Depense : 120/200 EUR │ Echeance : 25/12     │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ CONFIGURATION DES ALERTES                        │    │
│          │  │                                                  │    │
│          │  │  Seuils par categorie :                          │    │
│          │  │  Alimentation    [ 500 EUR/mois ]  [Actif ✓]     │    │
│          │  │  Transport       [ 150 EUR/mois ]  [Actif ✓]     │    │
│          │  │  Loisirs         [ 100 EUR/mois ]  [Inactif]     │    │
│          │  │                                                  │    │
│          │  │  Alertes objectifs :  [Actif ✓]                  │    │
│          │  │  Alertes budgets :    [Actif ✓]                  │    │
│          │  │  Frequence :          [Hebdomadaire ▾]           │    │
│          │  │                                                  │    │
│          │  │  Rappels JayKoa :     [Activer ▾]                │    │
│          │  │  → Publie echeances vers JayKoa                  │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : AlerteCard, AlerteConfig, SeuilInput, JayKoaToggle
**Contrats** : CK-TK-61, CK-INT-03

---

## 3. Account — Mocks ecrans

### 3.1 A1 — Dashboard Account

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta (Account)                       [Entreprise] ▾  ⚙  🔔 │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ ■ Tableau│  ┌────────────┐ ┌────────────┐ ┌────────────┐           │
│   de bord│  │ CA CE MOIS │ │  IMPAYES   │ │ TAUX PAIE. │           │
│          │  │            │ │            │ │            │           │
│ □ Journal│  │ 12 450 EUR │ │  2 800 EUR │ │     78%    │           │
│          │  │ ▲ +15% vs  │ │ 3 factures │ │ ████████░░ │           │
│ □ Devis  │  │  mois prec │ │ en retard  │ │            │           │
│          │  └────────────┘ └────────────┘ └────────────┘           │
│ □ Factur.│                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│ □ Paiem. │  │ ACTIONS RAPIDES                                  │    │
│          │  │                                                  │    │
│ □ Rappo. │  │  [ + Nouveau devis ]   [ + Nouvelle facture ]    │    │
│          │  │  [ Enregistrer paiement ]  [ Voir impayes ]      │    │
│ □ Integr.│  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│ ─────────│                                                          │
│ □ Param. │  ┌──────────────────────────────────────────────────┐    │
│          │  │ DEVIS RECENTS                                    │    │
│ ◇ Purse  │  │                                                  │    │
│          │  │  DEV-2026-012  │ Client Dupont  │ 1 500 EUR │Envoy│    │
│          │  │  DEV-2026-011  │ Assoc. Cataka. │ 3 200 EUR │Acce.│    │
│          │  │  DEV-2026-010  │ SARL Martin    │   800 EUR │Brou.│    │
│          │  │                                                  │    │
│          │  │  [ Voir tous les devis ]                         │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ FACTURES EN ATTENTE                              │    │
│          │  │                                                  │    │
│          │  │  FAC-2026-045  │ Client Dupont  │ 1 500 EUR │Envoy│    │
│          │  │  FAC-2026-042  │ Expo. Bernard  │   600 EUR │Reta.│    │
│          │  │  FAC-2026-039  │ Pro. Laurent   │ 1 200 EUR │Part.│    │
│          │  │                                                  │    │
│          │  │  [ Voir toutes les factures ]                    │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ INTEGRATIONS ACTIVES                             │    │
│          │  │                                                  │    │
│          │  │  JayFestival  │ ● Connecte │ 3 ops ce mois      │    │
│          │  │  JayRDV       │ ● Connecte │ 8 ops ce mois      │    │
│          │  │  JayKoa       │ ○ Optionnel│ 0 rappels           │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : KpiCards (CA, impayes, taux paiement), QuickActions, DevisRecents, FacturesEnAttente, IntegrationsStatus
**Contrats** : CK-OP-11, CK-OP-13, CK-TK-51

---

### 3.2 A2 — Journal / Grand Livre

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta > Journal                    [Grand Livre ▾]  [Export] │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Periode : [ 01/02/2026 ] → [ 28/02/2026 ]  [Appliquer]│
│          │  Categorie : [Toutes ▾]  Type : [Tous ▾]                │
│ ■ Journal│                                                          │
│          │  ┌──────┬───────┬─────────────────┬──────────┬────────┐  │
│ □ Devis  │  │ Date │ Ref.  │ Description     │  Debit   │ Credit │  │
│          │  ├──────┼───────┼─────────────────┼──────────┼────────┤  │
│ □ Factur.│  │10/02 │JE-089 │Paiement Dupont  │          │1 500,00│  │
│          │  │09/02 │JE-088 │Facture FAC-045  │1 500,00  │        │  │
│ □ Paiem. │  │08/02 │JE-087 │Achat fourniture │  250,00  │        │  │
│          │  │07/02 │JE-086 │Paiement partiel │          │  600,00│  │
│ □ Rappo. │  │      │       │ FAC-039 Laurent │          │        │  │
│          │  │06/02 │JE-085 │Location stand   │          │  150,00│  │
│          │  │      │       │ JayFestival     │          │        │  │
│          │  │05/02 │JE-084 │Abonnement SaaS  │   49,99  │        │  │
│          │  │04/02 │JE-083 │Facture FAC-042  │  600,00  │        │  │
│          │  │      │       │ Bernard Expo    │          │        │  │
│          │  │03/02 │JE-082 │Devis converti   │          │  800,00│  │
│          │  │      │       │ DEV→FAC Martin  │          │        │  │
│          │  │02/02 │JE-081 │Paiement mensuel │          │  200,00│  │
│          │  │      │       │ JayRDV abonnt   │          │        │  │
│          │  │01/02 │JE-080 │Report solde     │3 200,00  │        │  │
│          │  └──────┴───────┴─────────────────┴──────────┴────────┘  │
│          │                                                          │
│          │  Total debits : 5 599,99 │ Total credits : 3 250,00     │
│          │  Solde debiteur : 2 349,99                               │
│          │                                                          │
│          │  ◀ Page 1 / 2 ▶                                         │
│          │                                                          │
│          │  [ + Ecriture manuelle ]  [ Exporter PDF ] [ Export CSV]│
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : JournalTable (debit/credit), FilterBar, TotauxBar, Pagination, ExportButtons
**Contrats** : CK-TK-11

---

### 3.3 A3 — Devis

#### Liste des devis

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta > Devis                               [ + Nouveau ]    │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Filtres : Statut [Tous ▾]  Periode [Ce mois ▾]         │
│          │                                                          │
│ □ Journal│  ┌──────────┬──────────────┬──────────┬────────┬───────┐ │
│          │  │ Numero   │ Client       │  Montant │ Statut │Actions│ │
│ ■ Devis  │  ├──────────┼──────────────┼──────────┼────────┼───────┤ │
│          │  │DEV-012   │Client Dupont │ 1 500,00 │ Envoye │ ⋮     │ │
│ □ Factur.│  │DEV-011   │Assoc. Catak. │ 3 200,00 │ Accept.│ ⋮     │ │
│          │  │DEV-010   │SARL Martin   │   800,00 │Brouill.│ ⋮     │ │
│ □ Paiem. │  │DEV-009   │Pro. Laurent  │ 1 200,00 │ Refuse │ ⋮     │ │
│          │  │DEV-008   │Expo. Bernard │   600,00 │ Accept.│ ⋮     │ │
│          │  └──────────┴──────────────┴──────────┴────────┴───────┘ │
│          │                                                          │
│          │  Menu ⋮ :                                                │
│          │  ├ Voir detail                                           │
│          │  ├ Envoyer                                               │
│          │  ├ Convertir en facture  (si Accepte)                    │
│          │  ├ Dupliquer                                             │
│          │  └ Supprimer (si Brouillon)                              │
│          │                                                          │
│          │  Statistiques :                                          │
│          │  Total : 5 │ Brouillon : 1 │ Envoyes : 1 │ Acceptes : 2│
│          │  Refuses : 1 │ Montant accepte : 3 800 EUR              │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

#### Creation / edition devis

```
┌─────────────────────────────────────────────────────────────────────┐
│ ← Devis                      Nouveau devis               [Brouillon│
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Numero : DEV-2026-013 (auto)          Date : 10/02/2026           │
│  Validite : [ 30 jours ▾ ]            Echeance : 12/03/2026       │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ CLIENT                                                      │   │
│  │  Nom / Raison sociale *  [ _________________________ ] 🔍   │   │
│  │  Email                   [ _________________________ ]       │   │
│  │  Adresse                 [ _________________________ ]       │   │
│  │  SIRET                   [ _____________ ]                   │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ LIGNES DU DEVIS                                             │   │
│  │                                                             │   │
│  │  Description          │ Qte │ PU HT  │ TVA  │  Total HT   │   │
│  │  ─────────────────────┼─────┼────────┼──────┼─────────────│   │
│  │  Location stand GS_01 │  1  │ 80,00  │ 20%  │    80,00    │   │
│  │  Option electricite   │  1  │ 15,00  │ 20%  │    15,00    │   │
│  │  Tables supplement.   │  2  │ 10,00  │ 20%  │    20,00    │   │
│  │                                                             │   │
│  │  [ + Ajouter une ligne ]                                    │   │
│  │                                                             │   │
│  │  ──────────────────────────────────────────────────────────│   │
│  │  Sous-total HT :                                   115,00  │   │
│  │  TVA 20% :                                          23,00  │   │
│  │  ══════════════════════════════════════════════════════════│   │
│  │  TOTAL TTC :                                       138,00  │   │
│  │                                                             │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  Notes / Conditions :                                               │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ Conditions de reglement : a reception de facture.            │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  [ Sauvegarder brouillon ]  [ Envoyer au client ]  [ Apercu PDF ]  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Composants** : DevisHeader, ClientForm, LignesTable (editable), TotauxCalcul, NotesTextarea, ActionButtons
**Contrats** : CK-OP-11, CK-TK-21

---

### 3.4 A4 — Factures

#### Liste des factures

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta > Factures                            [ + Nouvelle ]    │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Filtres : Statut [Tous ▾]  Periode [Ce mois ▾]         │
│          │                                                          │
│ □ Journal│  ┌──────────┬──────────────┬──────────┬────────┬───────┐ │
│          │  │ Numero   │ Client       │  Montant │ Statut │  Due  │ │
│ □ Devis  │  ├──────────┼──────────────┼──────────┼────────┼───────┤ │
│          │  │FAC-045   │Client Dupont │ 1 500,00 │ Envoyee│ 12/03 │ │
│ ■ Factur.│  │FAC-042   │Expo. Bernard │   600,00 │ Retard │ 01/02 │ │
│          │  │FAC-039   │Pro. Laurent  │ 1 200,00 │Partiel │ 15/02 │ │
│ □ Paiem. │  │FAC-036   │Assoc. Catak. │ 3 200,00 │ Payee  │  ---  │ │
│          │  │FAC-033   │SARL Martin   │   800,00 │ Payee  │  ---  │ │
│          │  └──────────┴──────────────┴──────────┴────────┴───────┘ │
│          │                                                          │
│          │  Legende statuts :                                       │
│          │  ■ Payee  ■ Envoyee  ■ Partielle  ■ En retard           │
│          │                                                          │
│          │  Synthese :                                              │
│          │  Total emis : 7 300 EUR │ Paye : 4 000 EUR │ Impaye :   │
│          │  2 100 EUR  │ Retard : 600 EUR                          │
│          │                                                          │
│          │  [ Relancer les impayes ]  [ Exporter ]                  │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

#### Detail facture

```
┌─────────────────────────────────────────────────────────────────────┐
│ ← Factures              FAC-2026-039                    [Partielle] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Client : Pro. Laurent │ Email : laurent@pro.fr                     │
│  Emise le : 15/01/2026 │ Echeance : 15/02/2026                     │
│  Source devis : DEV-2026-007 (converti)                             │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Description          │ Qte │ PU HT  │ TVA  │  Total HT   │   │
│  │  ─────────────────────┼─────┼────────┼──────┼─────────────│   │
│  │  Forfait mensuel RDV  │  1  │ 900,00 │ 20%  │   900,00    │   │
│  │  Option premium       │  1  │ 100,00 │ 20%  │   100,00    │   │
│  │  ──────────────────────────────────────────────────────────│   │
│  │  Total HT : 1 000,00 │ TVA : 200,00 │ TTC : 1 200,00     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ PAIEMENTS RECUS                                             │   │
│  │                                                             │   │
│  │  Date     │ Montant  │ Methode      │ Reference             │   │
│  │  ─────────┼──────────┼──────────────┼───────────────────────│   │
│  │  25/01    │  600,00  │ Virement     │ VIR-2026-0158         │   │
│  │                                                             │   │
│  │  Paye : 600,00 / 1 200,00 TTC   Restant : 600,00 EUR      │   │
│  │  ████████████░░░░░░░░░░  50%                                │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │ HISTORIQUE                                                  │   │
│  │                                                             │   │
│  │  15/01 Facture emise (conversion DEV-007)                   │   │
│  │  16/01 Facture envoyee par email                            │   │
│  │  25/01 Paiement partiel recu (600 EUR)                      │   │
│  │  01/02 Relance niveau 1 envoyee                             │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  [ Enregistrer paiement ] [ Relancer ] [ Telecharger PDF ]         │
│  [ Envoyer par email ]    [ Voir audit ]                           │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Composants** : FactureHeader, LignesReadOnly, PaiementsTable, HistoriqueTimeline, ActionButtons
**Contrats** : CK-OP-12, CK-OP-13, CK-TK-31

---

### 3.5 A5 — Paiements

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta > Paiements                    [ + Enregistrer ]       │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Periode : [ Fevrier 2026 ▾ ]                            │
│          │                                                          │
│ □ Journal│  ┌──────────────────────────────────────────────────┐    │
│          │  │ SYNTHESE                                         │    │
│ □ Devis  │  │                                                  │    │
│          │  │  Total recu ce mois :     4 300,00 EUR           │    │
│ □ Factur.│  │  Nombre paiements :       6                      │    │
│          │  │  Methode principale :     Virement (72%)         │    │
│ ■ Paiem. │  │  Factures payees :        4 / 7                  │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────┬──────────┬──────────────┬──────────┬────────┐  │
│          │  │ Date │  Montant │ Facture      │ Methode  │ Ref.   │  │
│          │  ├──────┼──────────┼──────────────┼──────────┼────────┤  │
│          │  │10/02 │ 1 500,00 │ FAC-045      │Virement  │VIR-162 │  │
│          │  │08/02 │   800,00 │ FAC-044      │CB        │ ****   │  │
│          │  │05/02 │   600,00 │ FAC-039(part)│Virement  │VIR-158 │  │
│          │  │03/02 │   200,00 │ FAC-041      │Cheque    │CHQ-089 │  │
│          │  │02/02 │ 1 200,00 │ FAC-038      │Virement  │VIR-155 │  │
│          │  └──────┴──────────┴──────────────┴──────────┴────────┘  │
│          │                                                          │
│          │  Methodes de paiement :                                  │
│          │  Virement ██████████████████  72% (3 100 EUR)            │
│          │  CB       █████░░░░░░░░░░░░  19% (800 EUR)              │
│          │  Cheque   ██░░░░░░░░░░░░░░░   5% (200 EUR)              │
│          │  Especes  █░░░░░░░░░░░░░░░░   5% (200 EUR)              │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

#### Enregistrer un paiement

```
┌─────────────────────────────────────────────────────────────────────┐
│ ← Paiements             Enregistrer un paiement                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Facture *  [ FAC-2026-039 — Pro. Laurent — 1 200,00 EUR    ▾ ]   │
│             Restant du : 600,00 EUR                                 │
│                                                                     │
│  Montant *  [ 600,00 ] EUR   ☐ Paiement total (600,00 EUR)         │
│                                                                     │
│  Date *     [ 10/02/2026 ]                                          │
│                                                                     │
│  Methode *  ( ○ Virement ) ( ● CB ) ( ○ Cheque ) ( ○ Especes )     │
│                                                                     │
│  Reference  [ **** ] (tokenise, jamais stocke en clair)             │
│                                                                     │
│  Notes      [ _________________________ ]                           │
│                                                                     │
│  ⚠️ Securite : les donnees de paiement sont classifiees niveau 3   │
│     (CK-SEC-03). Aucune donnee CB/IBAN n'est stockee en clair.     │
│                                                                     │
│  [ Enregistrer ]  [ Annuler ]                                       │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

**Composants** : PaiementForm, FactureSelector, MethodeRadio, TokenizedInput
**Contrats** : CK-OP-14, CK-TK-41, CK-SEC-03

---

### 3.6 A6 — Rapports legaux

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta > Rapports                                             │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  Periode : [ Janvier 2026 ▾ ] → [ Fevrier 2026 ▾ ]      │
│          │                                                          │
│ □ Journal│  ┌──────────────────────────────────────────────────┐    │
│          │  │ COMPTE DE RESULTAT (simplifie)                   │    │
│ □ Devis  │  │                                                  │    │
│          │  │  PRODUITS                                        │    │
│ □ Factur.│  │    Chiffre d'affaires          24 800,00 EUR    │    │
│          │  │    Autres produits                  250,00 EUR   │    │
│ □ Paiem. │  │    ──────────────────────────────────────────    │    │
│          │  │    Total produits               25 050,00 EUR    │    │
│ ■ Rappo. │  │                                                  │    │
│          │  │  CHARGES                                         │    │
│          │  │    Achats et fournitures         3 200,00 EUR    │    │
│          │  │    Services exterieurs           1 800,00 EUR    │    │
│          │  │    Charges de personnel          8 500,00 EUR    │    │
│          │  │    Amortissements                  400,00 EUR    │    │
│          │  │    ──────────────────────────────────────────    │    │
│          │  │    Total charges                13 900,00 EUR    │    │
│          │  │                                                  │    │
│          │  │  ══════════════════════════════════════════════  │    │
│          │  │  RESULTAT NET                   11 150,00 EUR    │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ BALANCE COMPTABLE                                │    │
│          │  │                                                  │    │
│          │  │  Compte    │ Intitule        │  Debit  │ Credit  │    │
│          │  │  ──────────┼─────────────────┼─────────┼─────────│    │
│          │  │  411000    │ Clients         │12 500   │         │    │
│          │  │  401000    │ Fournisseurs    │         │ 3 200   │    │
│          │  │  512000    │ Banque          │22 350   │         │    │
│          │  │  701000    │ Ventes          │         │24 800   │    │
│          │  │  607000    │ Achats          │ 3 200   │         │    │
│          │  │  ...       │ ...             │  ...    │  ...    │    │
│          │  │                                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  Types de rapports :                                     │
│          │  [Compte resultat] [Balance] [Grand Livre] [Tresorerie]  │
│          │                                                          │
│          │  [ Exporter PDF ]  [ Exporter CSV ]  [ Exporter FEC ]   │
│          │                                                          │
│          │  ⚠️ L'export est audite (CK-AUD-02) et controle en scope│
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : CompteResultat, BalanceTable, GrandLivreView, TresorerieView, ExportButtons
**Contrats** : CK-OP-15, CK-TK-51, CK-AUD-02

---

### 3.7 A7 — Integrations

```
┌─────────────────────────────────────────────────────────────────────┐
│ ☰  JayKonta > Integrations                                         │
├──────────┬──────────────────────────────────────────────────────────┤
│          │                                                          │
│ □ Tableau│  ┌──────────────────────────────────────────────────┐    │
│          │  │ JAYFESTIVAL (CK-INT-01)                ● Actif   │    │
│ □ Journal│  │                                                  │    │
│          │  │  Operations ce mois : 3                          │    │
│ □ Devis  │  │  ─ quote.create : 1                              │    │
│          │  │  ─ invoice.emit : 1                              │    │
│ □ Factur.│  │  ─ budget.movements.record : 1                   │    │
│          │  │                                                  │    │
│ □ Paiem. │  │  Derniere operation : 08/02 — budget.movements   │    │
│          │  │  Edition active : Catakana 2026                  │    │
│ □ Rappo. │  │                                                  │    │
│          │  │  [ Voir operations ]  [ Rapport par edition ]    │    │
│ ■ Integr.│  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ JAYRDV (CK-INT-02)                     ● Actif   │    │
│          │  │                                                  │    │
│          │  │  Operations ce mois : 8                          │    │
│          │  │  ─ quote.create : 2                              │    │
│          │  │  ─ invoice.emit : 3                              │    │
│          │  │  ─ payment.record : 3                            │    │
│          │  │                                                  │    │
│          │  │  Derniere operation : 10/02 — payment.record     │    │
│          │  │                                                  │    │
│          │  │  [ Voir operations ]  [ Rapport par pro. ]       │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ JAYKOA (CK-INT-03)                     ○ Option. │    │
│          │  │                                                  │    │
│          │  │  Rappels publies : 0                              │    │
│          │  │  Statut : Non configure                          │    │
│          │  │                                                  │    │
│          │  │  [ Configurer ]                                  │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
│          │  ┌──────────────────────────────────────────────────┐    │
│          │  │ JOURNAL D'AUDIT INTEGRATIONS                     │    │
│          │  │                                                  │    │
│          │  │  10/02 14:32 │ JayRDV  │payment.record │ OK     │    │
│          │  │  08/02 09:15 │ JayFest │budget.move.   │ OK     │    │
│          │  │  07/02 16:45 │ JayRDV  │invoice.emit   │ OK     │    │
│          │  │  06/02 11:20 │ JayRDV  │quote.create   │ OK     │    │
│          │  │  05/02 10:00 │ JayFest │invoice.emit   │ OK     │    │
│          │  │                                                  │    │
│          │  │  [ Voir journal complet ]                        │    │
│          │  └──────────────────────────────────────────────────┘    │
│          │                                                          │
└──────────┴──────────────────────────────────────────────────────────┘
```

**Composants** : IntegrationCard, OperationsCount, AuditJournal
**Contrats** : CK-INT-01, CK-INT-02, CK-INT-03, CK-AUD-01

---

## 4. Navigation et layouts

### 4.1 Layout principal (desktop)

```
┌──────────────────────────────────────────────────────────────────┐
│ Barre superieure : logo, nom espace (Purse/Account), user, notif│
├──────────┬───────────────────────────────────────────────────────┤
│ Sidebar  │ Zone de contenu                                       │
│ ─ liens  │ ─ ecran actif                                         │
│ ─ actif  │ ─ scrollable                                          │
│   marque │                                                       │
│          │                                                       │
│ ──────── │                                                       │
│ Switch   │                                                       │
│ Purse ↔  │                                                       │
│ Account  │                                                       │
├──────────┴───────────────────────────────────────────────────────┤
│ Barre inferieure : statut, version                               │
└──────────────────────────────────────────────────────────────────┘
```

### 4.2 Sidebar Purse

| Icone | Label | Route |
|-------|-------|-------|
| 📊 | Tableau de bord | `/purse/dashboard` |
| 📝 | Mouvements | `/purse/mouvements` |
| 🎯 | Budgets occasionnels | `/purse/budgets` |
| 🏆 | Objectifs | `/purse/objectifs` |
| 📈 | Rapports | `/purse/rapports` |
| 🔔 | Alertes | `/purse/alertes` |
| ── | ── | ── |
| ⚙️ | Parametres | `/purse/parametres` |
| ↔️ | Basculer vers Account | `/account/dashboard` |

### 4.3 Sidebar Account

| Icone | Label | Route |
|-------|-------|-------|
| 📊 | Tableau de bord | `/account/dashboard` |
| 📒 | Journal / GL | `/account/journal` |
| 📋 | Devis | `/account/devis` |
| 🧾 | Factures | `/account/factures` |
| 💳 | Paiements | `/account/paiements` |
| 📈 | Rapports | `/account/rapports` |
| 🔗 | Integrations | `/account/integrations` |
| ── | ── | ── |
| ⚙️ | Parametres | `/account/parametres` |
| ↔️ | Basculer vers Purse | `/purse/dashboard` |

### 4.4 Layout mobile (responsive)

```
┌─────────────────────────────┐
│ ☰ JayBudget    [N] ▾  🔔   │
├─────────────────────────────┤
│                             │
│  Contenu principal          │
│  (ecran actif, scrollable)  │
│                             │
│                             │
│                             │
│                             │
│                             │
├─────────────────────────────┤
│ 📊 │ 📝 │ 🎯 │ 📈 │ ⋮    │
│ Tab │Mouv│Budg│Rapp│Plus   │
└─────────────────────────────┘
```

**Principe** : sidebar → bottom tab bar sur mobile. "Plus" ouvre un menu pour les items non visibles.

---

## 5. Schema SQL KindMother complet

```sql
-- ═══════════════════════════════════════════════════════════════
-- JayKonta — Schema complet KindMother (libSQL)
-- Purse + Account
-- ═══════════════════════════════════════════════════════════════

-- ─── Comptes ─────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS accounts (
    id          TEXT PRIMARY KEY,
    scope       TEXT NOT NULL CHECK(scope IN ('purse', 'account')),
    user_id     TEXT NOT NULL,
    label       TEXT NOT NULL,
    currency    TEXT NOT NULL DEFAULT 'EUR',
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(user_id, scope)
);

-- ─── Categories ──────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS categories (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    icon        TEXT,
    color       TEXT,
    parent_id   TEXT REFERENCES categories(id),
    is_income   INTEGER NOT NULL DEFAULT 0,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(account_id, name)
);

-- Categories par defaut Purse
-- Alimentation, Transport, Logement, Sante, Loisirs, Vetements,
-- Technologie, Epargne, Revenus (salaire, primes), Divers

-- ─── Mouvements (Purse + Account) ───────────────────────────

CREATE TABLE IF NOT EXISTS movements (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    scope           TEXT NOT NULL CHECK(scope IN ('purse', 'account')),
    category_id     TEXT REFERENCES categories(id),
    amount          REAL NOT NULL,           -- positif=revenu, negatif=depense
    currency        TEXT NOT NULL DEFAULT 'EUR',
    description     TEXT,
    movement_date   TEXT NOT NULL,
    context_ref     TEXT,                    -- ex: edition_id, budget_id
    source_service  TEXT,                    -- ex: 'jayfestival', 'jayrdv', 'manual'
    budget_id       TEXT REFERENCES budgets(id),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_movements_account ON movements(account_id);
CREATE INDEX idx_movements_date ON movements(movement_date);
CREATE INDEX idx_movements_category ON movements(category_id);
CREATE INDEX idx_movements_budget ON movements(budget_id);
CREATE INDEX idx_movements_scope ON movements(scope);

-- ─── Budgets occasionnels (Purse) ───────────────────────────

CREATE TABLE IF NOT EXISTS budgets (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    target      REAL NOT NULL,              -- montant cible
    spent       REAL NOT NULL DEFAULT 0.0,  -- cumul depenses
    currency    TEXT NOT NULL DEFAULT 'EUR',
    start_date  TEXT NOT NULL,
    end_date    TEXT,
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK(status IN ('active', 'closed', 'cancelled')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_budgets_account ON budgets(account_id);

-- ─── Objectifs (Purse) ──────────────────────────────────────

CREATE TABLE IF NOT EXISTS goals (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    target      REAL NOT NULL,              -- montant cible
    current     REAL NOT NULL DEFAULT 0.0,  -- montant atteint
    currency    TEXT NOT NULL DEFAULT 'EUR',
    goal_type   TEXT NOT NULL DEFAULT 'savings'
                CHECK(goal_type IN ('savings', 'spending_limit')),
    deadline    TEXT,
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK(status IN ('active', 'reached', 'failed', 'cancelled')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_goals_account ON goals(account_id);

-- ─── Alertes (Purse) ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS alerts (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    alert_type      TEXT NOT NULL
                    CHECK(alert_type IN ('category_threshold', 'goal_warning', 'budget_warning')),
    category_id     TEXT REFERENCES categories(id),
    goal_id         TEXT REFERENCES goals(id),
    budget_id       TEXT REFERENCES budgets(id),
    threshold       REAL,                   -- seuil en EUR
    frequency       TEXT NOT NULL DEFAULT 'weekly'
                    CHECK(frequency IN ('daily', 'weekly', 'monthly')),
    is_active       INTEGER NOT NULL DEFAULT 1,
    last_triggered  TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ─── Contreparties (Account) ─────────────────────────────────

CREATE TABLE IF NOT EXISTS counterparties (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    email           TEXT,
    address         TEXT,
    siret           TEXT,
    tva_number      TEXT,
    phone           TEXT,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_counterparties_account ON counterparties(account_id);

-- ─── Devis (Account) ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS quotes (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    number          TEXT NOT NULL,           -- DEV-2026-001
    counterparty_id TEXT NOT NULL REFERENCES counterparties(id),
    context_ref     TEXT,                    -- ex: edition_id (JayFestival)
    total_ht        REAL NOT NULL,
    total_tva       REAL NOT NULL DEFAULT 0.0,
    total_ttc       REAL NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    status          TEXT NOT NULL DEFAULT 'draft'
                    CHECK(status IN ('draft', 'sent', 'accepted', 'rejected', 'converted')),
    validity_days   INTEGER NOT NULL DEFAULT 30,
    notes           TEXT,
    source_service  TEXT,                    -- 'jayfestival', 'jayrdv', 'manual'
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(account_id, number)
);

CREATE INDEX idx_quotes_account ON quotes(account_id);
CREATE INDEX idx_quotes_status ON quotes(status);

-- ─── Lignes de devis ─────────────────────────────────────────

CREATE TABLE IF NOT EXISTS quote_lines (
    id          TEXT PRIMARY KEY,
    quote_id    TEXT NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity    REAL NOT NULL DEFAULT 1.0,
    unit_price  REAL NOT NULL,
    tva_rate    REAL NOT NULL DEFAULT 20.0,  -- en %
    total_ht    REAL NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0
);

-- ─── Factures (Account) ─────────────────────────────────────

CREATE TABLE IF NOT EXISTS invoices (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    number          TEXT NOT NULL,           -- FAC-2026-001
    counterparty_id TEXT NOT NULL REFERENCES counterparties(id),
    quote_id        TEXT REFERENCES quotes(id),
    context_ref     TEXT,
    total_ht        REAL NOT NULL,
    total_tva       REAL NOT NULL DEFAULT 0.0,
    total_ttc       REAL NOT NULL,
    paid_amount     REAL NOT NULL DEFAULT 0.0,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    status          TEXT NOT NULL DEFAULT 'issued'
                    CHECK(status IN ('issued', 'sent', 'partial', 'paid', 'overdue', 'cancelled')),
    issued_at       TEXT NOT NULL DEFAULT (datetime('now')),
    due_at          TEXT,
    source_service  TEXT,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(account_id, number)
);

CREATE INDEX idx_invoices_account ON invoices(account_id);
CREATE INDEX idx_invoices_status ON invoices(status);
CREATE INDEX idx_invoices_due ON invoices(due_at);

-- ─── Lignes de facture ───────────────────────────────────────

CREATE TABLE IF NOT EXISTS invoice_lines (
    id          TEXT PRIMARY KEY,
    invoice_id  TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity    REAL NOT NULL DEFAULT 1.0,
    unit_price  REAL NOT NULL,
    tva_rate    REAL NOT NULL DEFAULT 20.0,
    total_ht    REAL NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0
);

-- ─── Paiements (Account) ────────────────────────────────────

CREATE TABLE IF NOT EXISTS payments (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    invoice_id      TEXT NOT NULL REFERENCES invoices(id),
    amount          REAL NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    method          TEXT NOT NULL
                    CHECK(method IN ('virement', 'cb', 'cheque', 'especes', 'autre')),
    reference_opaque TEXT,                   -- tokenise, jamais en clair
    paid_at         TEXT NOT NULL,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_payments_invoice ON payments(invoice_id);

-- ─── Rappels JayKoa (optionnel) ─────────────────────────────

CREATE TABLE IF NOT EXISTS reminders (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    deadline_ref    TEXT NOT NULL,           -- ex: invoice due date, goal deadline
    due_at          TEXT NOT NULL,
    label           TEXT NOT NULL,
    context_ref     TEXT,
    source_service  TEXT,
    published       INTEGER NOT NULL DEFAULT 0,
    published_at    TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ─── Audit ───────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS audit (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id),
    contract_id TEXT NOT NULL,               -- CK-xxx
    actor_ref   TEXT NOT NULL,
    operation   TEXT NOT NULL,
    scope       TEXT NOT NULL,
    object_ref  TEXT NOT NULL,
    result      TEXT NOT NULL CHECK(result IN ('ok', 'error', 'denied')),
    payload     TEXT,                        -- JSON
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX idx_audit_account ON audit(account_id);
CREATE INDEX idx_audit_contract ON audit(contract_id);
CREATE INDEX idx_audit_date ON audit(created_at);

-- ─── Triggers ────────────────────────────────────────────────

CREATE TRIGGER trg_movements_updated
AFTER UPDATE ON movements FOR EACH ROW
BEGIN UPDATE movements SET updated_at = datetime('now') WHERE id = NEW.id; END;

CREATE TRIGGER trg_budgets_updated
AFTER UPDATE ON budgets FOR EACH ROW
BEGIN UPDATE budgets SET updated_at = datetime('now') WHERE id = NEW.id; END;

CREATE TRIGGER trg_goals_updated
AFTER UPDATE ON goals FOR EACH ROW
BEGIN UPDATE goals SET updated_at = datetime('now') WHERE id = NEW.id; END;

CREATE TRIGGER trg_quotes_updated
AFTER UPDATE ON quotes FOR EACH ROW
BEGIN UPDATE quotes SET updated_at = datetime('now') WHERE id = NEW.id; END;

CREATE TRIGGER trg_invoices_updated
AFTER UPDATE ON invoices FOR EACH ROW
BEGIN UPDATE invoices SET updated_at = datetime('now') WHERE id = NEW.id; END;

-- Mise a jour automatique du cumul budget quand mouvement affecte
CREATE TRIGGER trg_budget_spent_insert
AFTER INSERT ON movements
WHEN NEW.budget_id IS NOT NULL
BEGIN
    UPDATE budgets
    SET spent = (SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements WHERE budget_id = NEW.budget_id)
    WHERE id = NEW.budget_id;
END;

-- Mise a jour paid_amount sur facture quand paiement enregistre
CREATE TRIGGER trg_invoice_paid_insert
AFTER INSERT ON payments
BEGIN
    UPDATE invoices
    SET paid_amount = (SELECT COALESCE(SUM(amount), 0) FROM payments WHERE invoice_id = NEW.invoice_id),
        status = CASE
            WHEN (SELECT COALESCE(SUM(amount), 0) FROM payments WHERE invoice_id = NEW.invoice_id) >= total_ttc
            THEN 'paid'
            ELSE 'partial'
        END
    WHERE id = NEW.invoice_id;
END;

-- ─── Vues ────────────────────────────────────────────────────

-- Synthese mouvements par categorie et mois
CREATE VIEW IF NOT EXISTS v_movements_by_category AS
SELECT
    m.account_id,
    m.scope,
    c.name AS category_name,
    strftime('%Y-%m', m.movement_date) AS month,
    SUM(CASE WHEN m.amount > 0 THEN m.amount ELSE 0 END) AS income,
    SUM(CASE WHEN m.amount < 0 THEN ABS(m.amount) ELSE 0 END) AS expense,
    COUNT(*) AS count
FROM movements m
LEFT JOIN categories c ON m.category_id = c.id
GROUP BY m.account_id, m.scope, c.name, strftime('%Y-%m', m.movement_date);

-- Synthese factures par statut
CREATE VIEW IF NOT EXISTS v_invoice_summary AS
SELECT
    account_id,
    status,
    COUNT(*) AS count,
    SUM(total_ttc) AS total,
    SUM(paid_amount) AS paid,
    SUM(total_ttc - paid_amount) AS remaining
FROM invoices
GROUP BY account_id, status;

-- Synthese devis par statut
CREATE VIEW IF NOT EXISTS v_quote_summary AS
SELECT
    account_id,
    status,
    COUNT(*) AS count,
    SUM(total_ttc) AS total
FROM quotes
GROUP BY account_id, status;
```

---

## 6. Types Rust et services

### 6.1 Types principaux (complement domain/model.rs)

```rust
// crates/jaykonta/src/domain/purse.rs

/// Compte Purse (budget personnel)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurseAccount {
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub currency: String,
    pub balance: f64,             // calcule : sum(movements.amount)
    pub month_income: f64,        // calcule : sum(positifs ce mois)
    pub month_expense: f64,       // calcule : sum(negatifs ce mois)
}

/// Mouvement (depense ou revenu)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    pub id: String,
    pub amount: f64,
    pub category: Option<Category>,
    pub description: Option<String>,
    pub movement_date: String,
    pub budget: Option<BudgetRef>,
    pub source_service: Option<String>,
}

/// Budget occasionnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccasionalBudget {
    pub id: String,
    pub name: String,
    pub target: f64,
    pub spent: f64,
    pub remaining: f64,           // calcule : target - spent
    pub progress_pct: f64,        // calcule : spent / target * 100
    pub start_date: String,
    pub end_date: Option<String>,
    pub status: BudgetStatus,
    pub movements_count: u32,
}

/// Objectif d'epargne ou de depense
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub name: String,
    pub target: f64,
    pub current: f64,
    pub progress_pct: f64,
    pub goal_type: GoalType,
    pub deadline: Option<String>,
    pub status: GoalStatus,
    pub monthly_required: Option<f64>,  // calcule si deadline
    pub is_on_track: bool,              // calcule
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalType { Savings, SpendingLimit }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus { Active, Reached, Failed, Cancelled }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetStatus { Active, Closed, Cancelled }
```

```rust
// crates/jaykonta/src/domain/account.rs

/// Devis (quote)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub number: String,
    pub counterparty: Counterparty,
    pub lines: Vec<QuoteLine>,
    pub total_ht: f64,
    pub total_tva: f64,
    pub total_ttc: f64,
    pub status: QuoteStatus,
    pub validity_days: u32,
    pub notes: Option<String>,
    pub source_service: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteStatus { Draft, Sent, Accepted, Rejected, Converted }

/// Facture (invoice)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub number: String,
    pub counterparty: Counterparty,
    pub quote_id: Option<String>,
    pub lines: Vec<InvoiceLine>,
    pub total_ht: f64,
    pub total_tva: f64,
    pub total_ttc: f64,
    pub paid_amount: f64,
    pub remaining: f64,            // calcule
    pub payment_progress: f64,     // calcule (paid / ttc * 100)
    pub status: InvoiceStatus,
    pub issued_at: String,
    pub due_at: Option<String>,
    pub payments: Vec<Payment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceStatus { Issued, Sent, Partial, Paid, Overdue, Cancelled }

/// Paiement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub amount: f64,
    pub method: PaymentMethod,
    pub reference_opaque: Option<String>,  // tokenise
    pub paid_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod { Virement, Cb, Cheque, Especes, Autre }
```

### 6.2 Services metier

| Service | Responsabilite | Contrats |
|---------|----------------|----------|
| `PurseService` | Dashboard, mouvements, categories | CK-OP-01, CK-OP-02 |
| `BudgetService` | Budgets occasionnels, affectation mouvements | CK-OP-03, CK-TK-61 |
| `GoalService` | Objectifs, progression, alertes | CK-TK-61 |
| `AlertService` | Configuration, evaluation, declenchement | CK-TK-61 |
| `QuoteService` | Devis CRUD, envoi, conversion | CK-OP-11, CK-TK-21 |
| `InvoiceService` | Factures, relances, statut | CK-OP-12, CK-OP-13, CK-TK-31 |
| `PaymentService` | Enregistrement, liaison facture | CK-OP-14, CK-TK-41 |
| `ReportService` | Dashboard, rapports, export | CK-OP-15, CK-TK-51, CK-AUD-02 |
| `IntegrationPipeline` | Ingestion JayFestival, JayRDV, JayKoa | CK-INT-01/02/03 |
| `AuditService` | Journalisation, lecture audit | CK-AUD-01 |

---

## 7. Composants Dioxus

### 7.1 Inventaire des composants

#### Atoms

| Composant | Description | Utilise dans |
|-----------|-------------|--------------|
| `AmountDisplay` | Montant formate + couleur (+vert, -rouge) | Partout |
| `ProgressBar` | Barre de progression coloree | Budgets, objectifs |
| `Badge` | Statut colore (Brouillon, Envoye, Paye...) | Devis, factures |
| `KpiCard` | Carte indicateur (valeur, delta, tendance) | Dashboards |
| `CategoryChip` | Pilule categorie avec icone + couleur | Mouvements |
| `DatePicker` | Selecteur de date | Formulaires |
| `CurrencyInput` | Champ montant avec devise | Mouvements, devis |

#### Molecules

| Composant | Description | Utilise dans |
|-----------|-------------|--------------|
| `MovementRow` | Ligne mouvement (date, desc, cat, montant) | Listes |
| `QuickCategoryGrid` | Grille 2x4 categories rapides | Saisie mouvement |
| `BudgetCard` | Carte budget (nom, barre, restant) | Liste budgets |
| `GoalCard` | Carte objectif (nom, barre, deadline) | Liste objectifs |
| `AlerteCard` | Carte alerte (icone, message, action) | Alertes |
| `DevisRow` | Ligne devis (numero, client, montant, statut) | Liste devis |
| `FactureRow` | Ligne facture (numero, client, montant, statut, echeance) | Liste factures |
| `PaiementRow` | Ligne paiement (date, montant, facture, methode) | Paiements |
| `IntegrationCard` | Carte integration (service, statut, ops) | Integrations |
| `AuditRow` | Ligne audit (date, service, op, result) | Journal audit |

#### Organisms

| Composant | Description | Utilise dans |
|-----------|-------------|--------------|
| `Sidebar` | Navigation laterale (Purse ou Account) | Layout |
| `TopBar` | Barre sup (logo, espace, user, notif) | Layout |
| `FilterBar` | Filtres (periode, categorie, statut, type) | Listes |
| `MouvementsTable` | Tableau mouvements pagine | P2 |
| `CategorieChart` | Repartition depenses par categorie | Dashboard |
| `EvolutionChart` | Graphique evolution 6 mois | Rapports |
| `CompteResultat` | P&L simplifie | Rapports Account |
| `BalanceTable` | Balance comptable debit/credit | Rapports Account |
| `LignesEditor` | Editeur de lignes devis/facture | Devis, factures |
| `PaiementsTimeline` | Historique paiements d'une facture | Detail facture |

#### Pages (ecrans complets)

| Page | Route | Composants principaux |
|------|-------|-----------------------|
| `EntryPointSelector` | `/` | EntryPointCard x2 |
| `PurseDashboard` | `/purse/dashboard` | SoldeCard, KpiCards, CategorieChart, MouvementsList, ObjectifsResume, BudgetsResume |
| `PurseMouvements` | `/purse/mouvements` | FilterBar, MouvementsTable, TotauxSummary |
| `PurseMouvementForm` | `/purse/mouvements/new` | TypeToggle, AmountInput, CategoryPicker, DatePicker, SubmitButton |
| `PurseBudgets` | `/purse/budgets` | BudgetCard list, CreateButton |
| `PurseBudgetDetail` | `/purse/budgets/:id` | BudgetCard, MouvementsList, ActionButtons |
| `PurseObjectifs` | `/purse/objectifs` | GoalCard list, CreateButton |
| `PurseRapports` | `/purse/rapports` | SyntheseMensuelle, CategorieComparison, EvolutionChart, ExportButtons |
| `PurseAlertes` | `/purse/alertes` | AlerteCard list, AlerteConfig |
| `AccountDashboard` | `/account/dashboard` | KpiCards, QuickActions, DevisRecents, FacturesEnAttente, IntegrationsStatus |
| `AccountJournal` | `/account/journal` | FilterBar, JournalTable, TotauxBar, ExportButtons |
| `AccountDevis` | `/account/devis` | FilterBar, DevisTable, StatsSummary |
| `AccountDevisForm` | `/account/devis/new` | ClientForm, LignesEditor, TotauxCalcul, ActionButtons |
| `AccountFactures` | `/account/factures` | FilterBar, FacturesTable, StatsSummary |
| `AccountFactureDetail` | `/account/factures/:id` | FactureHeader, LignesReadOnly, PaiementsTimeline, ActionButtons |
| `AccountPaiements` | `/account/paiements` | PaiementsSynthese, PaiementsTable, MethodeChart |
| `AccountPaiementForm` | `/account/paiements/new` | FactureSelector, AmountInput, MethodeRadio, SubmitButton |
| `AccountRapports` | `/account/rapports` | CompteResultat, BalanceTable, ExportButtons |
| `AccountIntegrations` | `/account/integrations` | IntegrationCard list, AuditJournal |

---

## 8. Guide d'implementation par phase

### Phase 1 : Fondations (P0)

**Prerequis** : KindMother fonctionnel, MiyuAuth operationnel

| Etape | Tache | Fichiers | Estimation |
|-------|-------|----------|------------|
| 1.1 | Creer schema SQL complet | `data/schema.sql` | 0.5j |
| 1.2 | Types Rust domain (Purse + Account) | `domain/purse.rs`, `domain/account.rs` | 1j |
| 1.3 | DB layer KindMother (CRUD movements) | `data/purse_db.rs`, `data/account_db.rs` | 1.5j |
| 1.4 | PurseService (dashboard, mouvements, categories) | `services/purse_service.rs` | 1j |
| 1.5 | Layout principal + sidebar + routing | `ui/layout.rs`, `ui/sidebar.rs` | 1j |
| 1.6 | Ecran selection Purse/Account | `ui/entry_point.rs` | 0.5j |
| 1.7 | Dashboard Purse (SoldeCard, KpiCards, derniers mouvements) | `ui/purse/dashboard.rs` | 1.5j |
| 1.8 | Formulaire mouvement rapide | `ui/purse/movement_form.rs` | 1j |
| 1.9 | Liste mouvements (filtrable, paginee) | `ui/purse/movements_list.rs` | 1j |
| 1.10 | AuditService (journalisation writes) | `services/audit_service.rs` | 0.5j |

**Critere de sortie Phase 1** : L'utilisateur peut acceder a Purse, voir son solde, saisir un mouvement en < 2s, filtrer ses mouvements.

### Phase 2 : Purse complet (P1)

| Etape | Tache | Fichiers | Estimation |
|-------|-------|----------|------------|
| 2.1 | BudgetService (CRUD, affectation mouvements) | `services/budget_service.rs` | 1j |
| 2.2 | Ecrans budgets occasionnels (liste + detail) | `ui/purse/budgets.rs`, `ui/purse/budget_detail.rs` | 1.5j |
| 2.3 | GoalService (CRUD, progression) | `services/goal_service.rs` | 1j |
| 2.4 | Ecrans objectifs (liste + detail) | `ui/purse/goals.rs` | 1j |
| 2.5 | CategorieChart (repartition) | `ui/components/category_chart.rs` | 0.5j |
| 2.6 | Dashboard Purse complet (budgets + objectifs resumes) | Enrichir `dashboard.rs` | 0.5j |

**Critere de sortie Phase 2** : Parcours P1–P4 complets. Budgets et objectifs fonctionnels.

### Phase 3 : Account fondations (P0–P1)

| Etape | Tache | Fichiers | Estimation |
|-------|-------|----------|------------|
| 3.1 | CounterpartyService (clients/fournisseurs) | `services/counterparty_service.rs` | 0.5j |
| 3.2 | QuoteService (CRUD, envoi, conversion) | `services/quote_service.rs` | 1.5j |
| 3.3 | InvoiceService (emission, relance, statut) | `services/invoice_service.rs` | 1.5j |
| 3.4 | PaymentService (enregistrement, liaison) | `services/payment_service.rs` | 1j |
| 3.5 | Dashboard Account | `ui/account/dashboard.rs` | 1j |
| 3.6 | Journal / Grand Livre | `ui/account/journal.rs` | 1.5j |
| 3.7 | Devis (liste + form + detail) | `ui/account/quotes.rs` | 2j |
| 3.8 | Factures (liste + detail) | `ui/account/invoices.rs` | 2j |
| 3.9 | Paiements (liste + form) | `ui/account/payments.rs` | 1j |

**Critere de sortie Phase 3** : Cycle devis → facture → paiement complet. Journal consultable.

### Phase 4 : Reporting et export (P2)

| Etape | Tache | Fichiers | Estimation |
|-------|-------|----------|------------|
| 4.1 | ReportService (Purse : synthese, categories, evolution) | `services/report_service.rs` | 1j |
| 4.2 | ReportService (Account : P&L, balance, GL) | Enrichir `report_service.rs` | 1.5j |
| 4.3 | Ecran rapports Purse | `ui/purse/reports.rs` | 1j |
| 4.4 | Ecran rapports Account | `ui/account/reports.rs` | 1.5j |
| 4.5 | Export PDF/CSV (avec audit CK-AUD-02) | `services/export_service.rs` | 1j |

**Critere de sortie Phase 4** : Parcours P5. Rapports Account consultables. Export audite.

### Phase 5 : Integrations (P2–P3)

| Etape | Tache | Fichiers | Estimation |
|-------|-------|----------|------------|
| 5.1 | IntegrationPipeline (JayFestival CK-INT-01) | Enrichir `integrations/pipeline.rs` | 1j |
| 5.2 | IntegrationPipeline (JayRDV CK-INT-02) | Enrichir `pipeline.rs` | 1j |
| 5.3 | JayKoa reminders (CK-INT-03) | Enrichir `pipeline.rs` | 0.5j |
| 5.4 | AlertService (Purse) | `services/alert_service.rs` | 1j |
| 5.5 | Ecran alertes Purse | `ui/purse/alerts.rs` | 0.5j |
| 5.6 | Ecran integrations Account | `ui/account/integrations.rs` | 1j |

**Critere de sortie Phase 5** : Parcours P6. CK-INT-01/02/03 fonctionnels.

### Phase 6 : Securite et polish (P3)

| Etape | Tache | Estimation |
|-------|-------|------------|
| 6.1 | Validation securite niveau 2/3 (CK-SEC-01) | 1j |
| 6.2 | Tokenisation paiements (CK-SEC-03) | 0.5j |
| 6.3 | Mandats StrongFather sur tous les writes (CK-SEC-02) | 1j |
| 6.4 | Responsive mobile (bottom tab bar) | 1j |
| 6.5 | Tests performance (NFR-PUR-04, NFR-PUR-05) | 0.5j |
| 6.6 | Revision audit complet | 0.5j |

**Critere de sortie Phase 6** : Securite validee. Mobile fonctionnel. Performance acceptable.

### Estimation totale

| Phase | Estimation | Cumul |
|-------|------------|-------|
| Phase 1 : Fondations | 9.5j | 9.5j |
| Phase 2 : Purse complet | 5.5j | 15j |
| Phase 3 : Account fondations | 12j | 27j |
| Phase 4 : Reporting | 6j | 33j |
| Phase 5 : Integrations | 5j | 38j |
| Phase 6 : Securite/polish | 4.5j | 42.5j |
| **TOTAL** | **42.5 jours** | |

---

## 9. Checklist de conformite

### Contrats a satisfaire par ecran

| Ecran | Contrats | Validation |
|-------|----------|------------|
| Selection Purse/Account | CK-SVC-01, CK-SVC-02, CK-TK-01 | Auth + scope |
| Dashboard Purse | CK-OP-01, CK-TK-51 | Lecture seule |
| Saisie mouvement | CK-OP-02, CK-TK-11, CK-AUD-01 | Mandat write + audit |
| Budgets occasionnels | CK-OP-03, CK-TK-61 | CRUD + affectation |
| Objectifs | CK-TK-61 | CRUD + progression |
| Rapports Purse | CK-TK-51, CK-AUD-02 | Lecture + audit export |
| Alertes | CK-TK-61, CK-INT-03 | Config + JayKoa |
| Dashboard Account | CK-OP-11, CK-OP-13, CK-TK-51 | Lecture seule |
| Devis | CK-OP-11, CK-TK-21, CK-AUD-01 | CRUD + envoi + conversion |
| Factures | CK-OP-12, CK-OP-13, CK-TK-31, CK-AUD-01 | Emission + relance |
| Paiements | CK-OP-14, CK-TK-41, CK-SEC-03 | Enregistrement + tokenisation |
| Rapports Account | CK-OP-15, CK-TK-51, CK-AUD-02 | P&L + export audite |
| Integrations | CK-INT-01, CK-INT-02, CK-INT-03 | Pipeline + audit |

### NFR (Non-Functional Requirements)

| NFR | Exigence | Verification |
|-----|----------|--------------|
| NFR-PUR-04 | Dashboard Purse < 3s | Mesure temps de chargement |
| NFR-PUR-05 | Saisie mouvement < 2s | Mesure saisie complete |
| NFR-PUR-06 | Max 3 actions pour saisir depense | Comptage interactions |
| NFR-PUR-07 | Mobile + desktop | Test responsive |
| NFR-MAC-05 | Dashboard Account < 3s | Mesure temps de chargement |
| NFR-MAC-06 | Emission facture < 3s | Mesure traitement |

### Regles de securite

| Regle | Description | Enforcement |
|-------|-------------|-------------|
| CK-SEC-01 | Donnees financieres minimum niveau 2 | Classification KindMother |
| CK-SEC-02 | Mandat sur toutes les ecritures | StrongFather check avant write |
| CK-SEC-03 | Pas de donnees paiement en clair | Tokenisation + chiffrement |
| CK-SEC-04 | Federation inter-COG reglee | Contrats CK-INT validees |

---

## Dependances inter-services

| Service / Kit | Role dans JayKonta |
|---------------|---------------------|
| **KindMother** | Persistance (libSQL) + broadcast evenements |
| **StrongFather** | Mandats de permission sur writes |
| **MasterButler** | Capacites exposees (export, alertes) |
| **WorrySentinel** | Niveaux securite (2-3) |
| **BorderGuard** | Frontieres donnees sensibles |
| **CaringNanny** | Monitoring soldes et seuils |
| **MiyuAuth** | Authentification + contexte Purse/Account |
| **JayFestival** | CK-INT-01 : devis, factures, budget editions |
| **JayRDV** | CK-INT-02 : facturation professionnels |
| **JayKoa** | CK-INT-03 : rappels echeances |

---

**Version** : 1.0
**Date** : 10 fevrier 2026
**Blocage debloque** : Phase 2.2 Parcours Developpement Purse (wireframes UI)
**Statut** : Pret pour implementation Phase 1
