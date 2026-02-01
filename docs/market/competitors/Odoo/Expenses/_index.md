# Odoo Expenses — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Expenses - Logique Metier Complete.md](./00_logique_metier/Odoo%20Expenses%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèle hr.expense (dépense), champs et états
- Règles métier et contraintes (montant, approbation, édition)
- Workflow draft → submitted → approved → posted → paid / refused
- Modes de paiement (employé à rembourser / entreprise)
- Calcul montants, taxes, multi-devises
- Détection doublons et same receipt, split, création depuis PJ/email
- Intégration Accounting, HR, Mail, Product, Analytic

### 2. Parcours Utilisateur
📄 [Odoo Expenses - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Expenses%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Employé, Manager, Officer, Comptable)
- Parcours d'onboarding
- Scénarios d'usage (saisie, rapport, approbation, remboursement)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Expenses - Analyse UI UX.md](./02_ui_ux/Odoo%20Expenses%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Dashboard et navigation (My Expenses, To Process, My Reports)
- Vues Liste, Kanban, Formulaire (hr.expense)
- Wizards (Refuse, Post, Split, Approve duplicate)
- Composants (pièces jointes, montants, statut)
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Expenses - Integrations Cross App.md](./03_integrations/Odoo%20Expenses%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (hr, account, mail, product, analytic)
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs et hooks utilisés
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Expenses - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Expenses%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (ExpenseOperator, ExpenseApprovalOperator, ExpensePostOperator, ExpenseCategoryOperator, ExpenseUI)
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité (1–3)
- Intégration avec les Cores

### 6. Guide Intégration COG
📄 [Odoo Expenses - Guide Integration COG.md](./05_integration_cog/Odoo%20Expenses%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (création, soumission, approbation, post)
- Exemples de code pseudo-Rust
- Gestion des gouvernances

### 7. Guide Implémentation
📄 [Odoo Expenses - Guide Implementation.md](./06_guides_implementation/Odoo%20Expenses%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique détaillée (crates miyuexpense, miyuexpense_post, miyuexpense_category, miyuexpense_ui)
- Spécifications des crates Rust
- Schémas de données (Expense, états, paiement)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## Service Miyukini Proposé

**Nom :** `MiyukiniExpenses` ou `MiyuExpense`

**Opérateurs :**
- **ExpenseOperator** : Gestion des dépenses (création, édition, soumission, split, création depuis PJ/email)
- **ExpenseApprovalOperator** : Workflow d'approbation et refus
- **ExpensePostOperator** : Comptabilisation et paiement (post, remboursement)
- **ExpenseCategoryOperator** : Catégories expensables
- **ExpenseUI** : Interface utilisateur Expenses

**Équipe d'Opérateurs :** `ExpenseService`

---

## Source d'Analyse

**Repository :** `https://github.com/odoo/odoo/tree/19.0/addons/hr_expense`

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application Notes de frais avec workflow draft → submitted → approved → posted → paid / refused
- Intégrations multiples (Accounting, HR, Mail, Product, Analytic)
- Deux modes de paiement : employé à rembourser (in_receipt) vs entreprise paie (paiement outbound)
- Droits par groupes (employee, officer, manager) et hiérarchie (manager_id, expense_manager_id, département)
