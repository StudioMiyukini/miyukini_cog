# Miyukini Framework - Feature Budget Apex

## Contexte

Le Miyukini Framework doit proposer une **fonctionnalité Budget** utilisable par plusieurs modules (Festival Orga, Booking Pro, Abonnement, BackOffice, …) afin de produire un **bilan comptable professionnel**, **dynamique**, **automatique** et **riche** :
- centralisation des recettes/dépenses,
- suivi budgétaire temps réel,
- rapprochement avec la facturation/paiement,
- reporting (P&L, prévisionnel vs réalisé),
- justificatifs (documents),
- exports comptables.

Cette spécification s’inspire :
- de Catakana Orga (`budget_entries`, `budget_categories`, génération automatique de recettes depuis factures payées),
- des pratiques courantes d’outils de gestion (budgeting, compta simplifiée, reporting).

## Portée / Scope

- **Capability** : Budget (indépendante, composable).
- **Niveaux** :
  - **Niveau 1** : budget “gestion” (entrées recettes/dépenses + catégories + stats).
  - **Niveau 2** : compta enrichie (TVA, comptes, centres de coûts, multi-devises, exports).
  - **Niveau 3** : automatisations (factures → recettes, récurrences, rapprochement).
- **Hors scope** : devenir un logiciel comptable certifié, déclaration fiscale automatisée.

---

## Benchmark (pratiques courantes “budget pro”)

Les bonnes pratiques observables dans les outils pro :
- **Séparer** : *prévisionnel* vs *réalisé* (budget annuel/édition + exécution).
- **Classer** : catégories, sous-catégories, comptes comptables, centres de coûts.
- **Tracer** : justificatifs, audits, liens vers factures/commandes.
- **Automatiser** : récurrences, import/exports, ingestion d’événements (paiement reçu).
- **Fiabiliser** : idempotence, prévention de doublons, rapprochement.
- **Piloter** : KPIs et dashboards, alertes (dépassement, trésorerie).

---

## Fonctionnalités (capabilities)

### 1) CRUD budget : entrées, catégories, pièces

- **Entrées** : `income` / `expense`, montants, dates, description, tags, notes.
- **Catégories** :
  - type (`income`, `expense`, `both`),
  - icône / couleur / statut actif,
  - optionnel : hiérarchie (parent/child).
- **Justificatifs** :
  - upload (bucket privé),
  - accès via URL signée,
  - traçabilité (qui a uploadé, quand).

> Signal Catakana : `budget_entries`, `budget_categories`, upload `receipts` + URL signée.

### 2) Budgets “prévisionnel vs réalisé”

- **Budget planifié** :
  - par période (mois/trimestre/année) ou par “contexte” (ex: `edition_id`, `workspace_id`),
  - par catégorie et/ou centre de coût.
- **Budget réalisé** :
  - agrégation des entrées (manuelles + auto).
- **Variance** :
  - `planned - actual`,
  - alertes seuil (ex: +10%).

### 3) Centres de coûts / projets / contextes (multi-modules)

Le budget doit être filtrable par :
- `workspace_id` (tenancy),
- `module_id` (origine),
- `context_id` (ex: `edition_id`, `booking_id`, `subscription_plan_id`),
- `cost_center_id` (ex: “Com”, “Logistique”, “RH”).

> Signal Catakana : ajout d’`edition_id` dans les entrées budgétaires.

### 4) Facturation/paiement → recettes automatiques (et dépenses)

Automatisations attendues :
- **Invoice paid → budget income** :
  - éviter doublons (dedupe),
  - catégorisation automatique (“Stands”, “Abonnements”, “Booking”…),
  - lien vers la facture.

> Signal Catakana : `create_budget_entry_from_invoice(invoice_id)` + trigger sur changement de statut `paid`.

Extension Miyukini :
- invoice `created` → **prévisionnel** (planned) ou “attendu”,
- invoice `paid` → **réalisé** (actual),
- refunds → correction (negative entry ou entry type “refund”).

### 5) TVA / taxes / multi-devises (niveau 2)

Pour un bilan “pro” :
- `tax_rate` / `tax_amount` / `net_amount` / `gross_amount`,
- mode TVA (incluse/exclue),
- multi-devises : `currency`, `fx_rate`, `amount_base_currency`.

### 6) Journal / audit / validations

- Historique des modifications (qui, quoi, quand),
- statuts : `draft`, `validated`, `reconciled`, `archived`,
- verrouillage (période close) : interdiction de modifier après clôture.

### 7) Reporting & exports

Exports :
- CSV (pour exploitation),
- format “compta” (FEC-like simplifié, ou mapping comptes),
- PDF (rapport / bilan).

KPIs :
- solde, trésorerie, burn rate,
- top catégories dépenses,
- revenus par source.

### 8) Alertes / automatisations (niveau 3)

- dépassement budget catégorie,
- cashflow négatif projeté,
- entrée manquante (facture payée sans budget entry),
- récurrences (loyer, salaires, subscriptions, charges),
- import bancaire (optionnel futur).

---

## APEX : Architecture / Processus / Expérience

### A = Architecture

**Ports (interfaces)** :
- `BudgetEntriesPort` : CRUD entrées + attachements + tags
- `BudgetPlanningPort` : budget prévisionnel + périodes
- `BudgetReportingPort` : stats, exports, variance
- `BudgetAutomationPort` : handlers d’événements (invoice paid, refund, etc.)
- `BudgetStoragePort` : upload/download justificatifs (signed URLs)

**Events (domain budget)** :
- `budget.entry.created`
- `budget.entry.updated`
- `budget.entry.deleted`
- `budget.entry.validated`
- `budget.plan.updated`
- `budget.alert.threshold_exceeded`

**Events consommés (cross-modules)** :
- `billing.invoice.created`
- `billing.invoice.paid`
- `billing.refund.created`
- `documents.file.uploaded`

### P = Processus (exécution typique)

1) Le module (ex: Festival Orga) crée un contexte (`edition_id`).  
2) Le budget reçoit :
- entrées manuelles (dépenses diverses),
- entrées automatiques (factures payées).
3) Le back-office valide / rapproche / clôture.  
4) Reporting : variance, exports, rapports.

### E = Expérience (UX)

Back-office recommandé :
- **Budget → Aperçu** (KPIs + graphiques)
- **Budget → Entrées** (table, filtres, tags, import/export)
- **Budget → Prévisionnel** (grille par catégorie/période)
- **Budget → Catégories** (CRUD + mapping comptes)
- **Budget → Justificatifs** (liens/preview)
- **Budget → Clôture** (lock période)
- **Budget → Exports** (CSV/PDF/format compta)

---

## Spécification : data contracts (proposition)

### 1) Tables

#### `budget_entries`
- `id` UUID (recommandé) / SERIAL (Catakana)
- `workspace_id` UUID
- `module_id` TEXT (ex: `festival-orga`, `booking-pro`)
- `context_id` UUID/TEXT (ex: `edition_id`)
- `type` ENUM: `income`, `expense`, `refund`, `transfer`
- `status` ENUM: `draft`, `validated`, `reconciled`, `archived`
- `date` DATE
- `description` TEXT
- `amount_net`, `amount_tax`, `amount_gross` NUMERIC
- `currency` TEXT (EUR), `fx_rate` NUMERIC (optionnel)
- `category_id` UUID
- `cost_center_id` UUID (optionnel)
- `tags` TEXT[]
- `notes` TEXT
- `receipt_path` TEXT (private storage)
- liens :
  - `invoice_id` UUID (optionnel)
  - `payment_id` UUID (optionnel)
- `created_by`, `updated_by`, `created_at`, `updated_at`
- `dedupe_key` TEXT UNIQUE (idempotence)

#### `budget_categories`
- `id` UUID
- `workspace_id` UUID
- `name` TEXT UNIQUE (par workspace)
- `type` ENUM: `income`, `expense`, `both`
- `parent_id` UUID (optionnel)
- `icon`, `color`, `is_active`
- `account_code` TEXT (plan comptable) (optionnel)
- `created_at`, `updated_at`

#### `budget_plans`
- `id` UUID
- `workspace_id` UUID
- `module_id` TEXT
- `context_id` UUID/TEXT (ex: edition)
- `period_type` ENUM (`month`, `quarter`, `year`)
- `start_date`, `end_date`
- `currency`
- `created_by`, `created_at`, `updated_at`

#### `budget_plan_lines`
- `id` UUID
- `plan_id` UUID
- `category_id` UUID
- `period_start` DATE
- `amount_planned` NUMERIC

#### `budget_entry_audit` (optionnel)
- `id`, `entry_id`, `action`, `actor_id`, `payload`, `created_at`

### 2) Storage

- Bucket `budget_receipts` (privé)
- Accès via URL signée (60–300s)

---

## Policies / RLS (modèle recommandé)

Règle générale :
- lecture possible pour rôles autorisés (admin/backoffice),
- écriture réservée aux rôles “compta” (admin/super_admin) ou aux modules via service role/edge.

### Exemples (pseudo)

- `budget_categories`
  - `SELECT`: authenticated (ou admin uniquement selon produit)
  - `INSERT/UPDATE/DELETE`: `is_admin_user()` / `is_super_admin()`
- `budget_entries`
  - `SELECT`: `is_admin_user()` OU `workspace_id` match + role autorisé
  - `INSERT/UPDATE/DELETE`: `is_admin_user()` (ou `validated` lock = super_admin)
- `budget_plans`
  - `SELECT`: admin/super_admin
  - `UPDATE`: admin/super_admin

> Important : éviter les policies récursives (Catakana fait des EXISTS sur profiles).  
> Utiliser les helpers Miyukini `public.is_admin_user()` et `public.is_super_admin()`.

---

## Référence Catakana Orga (sources)

- `scripts/sql/budget-schema.sql` (tables + RLS + catégories par défaut)
- `src/lib/supabase/budgetService.ts` (CRUD + stats + receipts + filtres)
- `supabase/migrations/20251115_add_invoice_to_budget_income.sql` (facture payée → recette automatique)

