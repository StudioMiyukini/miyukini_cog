# Odoo Expenses — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Expenses** (Notes de frais) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Expenses
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **ExpenseOperator** | Gestion des dépenses (création, édition, soumission) | Opérateur de Service |
| **ExpenseApprovalOperator** | Workflow d'approbation et refus | Opérateur de Service |
| **ExpensePostOperator** | Comptabilisation et paiement (post, remboursement) | Opérateur de Service |
| **ExpenseCategoryOperator** | Catégories expensables (produits / référentiel) | Opérateur de Domaine |
| **ExpenseUI** | Interface utilisateur Expenses | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : ExpenseService

**Définition :**
> **ExpenseService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de notes de frais (saisie, approbation, comptabilisation, remboursement).**

**Composition :**
- ExpenseOperator (niveau sécurité 2)
- ExpenseApprovalOperator (niveau sécurité 2)
- ExpensePostOperator (niveau sécurité 3)
- ExpenseCategoryOperator (niveau sécurité 1–2)
- ExpenseUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 ExpenseOperator

**Rôle :** Gestion des dépenses (création, édition, soumission, pièces jointes, création depuis email/fichiers).

**Capacités :**
- Création / modification de dépenses (draft)
- Saisie catégorie, montant, date, devise, taxes, analytique
- Gestion pièce jointe principale et pièces jointes
- Création depuis pièces jointes (create_expense_from_attachments)
- Création depuis email (alias → message_new)
- Soumission (action_submit) — passage submitted / approved si autovalidation
- Détection doublons et same receipt (exposition pour alertes)
- Split (action_split_wizard) — création de deux dépenses à partir d’une

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de soumission (qui peut soumettre pour qui)
- **KindMother** : Persistance des dépenses (WriteIntent)
- **Master Butler** : Permissions création / édition / soumission (employé, manager)
- **WorrySentinel** : Niveau sécurité données (montants, pièces jointes)
- **Ever Buddy** : Cycle de vie dépense (draft → submitted → …)

**Contrat d'équipe :**
- Consomme : ExpenseCategoryOperator (catégories), MiyuHR / MiyuContacts (employé, manager), MiyuNotify (notifications), MiyuMedia (pièces jointes), MiyuInvoice (analytique si applicable)
- Expose : `expense.create`, `expense.update`, `expense.submit`, `expense.split`, `expense.create_from_attachments`

**Mandat de Permission requis :**
- Création dépense : Mandat avec KindMother (WriteIntent) + Master Butler (expense.create)
- Modification dépense (draft) : Mandat avec KindMother (WriteIntent) + is_editable (Master Butler)
- Soumission : Mandat avec StrongFather (décision submit) + KindMother (WriteIntent state) + MiyuNotify (notification manager)
- Split : Mandat avec KindMother (WriteIntent) + ExpenseOperator (expense.split)

### 2.2 ExpenseApprovalOperator

**Rôle :** Workflow d'approbation et refus (approve, refuse, reset to draft).

**Capacités :**
- Approbation (action_approve) — vérification droits, doublons, _do_approve
- Refus avec motif (action_refuse → _do_refuse)
- Reset to draft (action_reset) — annulation moves si postés, _do_reset_approval
- Calcul responsable approbation (manager_id, expense_manager_id, département, parent)
- Auto-validation (pas de manager → approbation automatique à la soumission si configuré)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision approve / refuse / reset
- **KindMother** : Persistance des changements d’état (approval_state, manager_id, approval_date)
- **Master Butler** : Permissions can_approve, can_reset (groupes, hiérarchie)
- **WorrySentinel** : Vérification niveau sécurité, isolation cross-équipe / cross-société
- **TAMR** : Point d’intervention humaine (approbation / refus) avec traçabilité

**Contrat d'équipe :**
- Consommé par : ExpenseUI, ExpenseOperator (soumission peut déclencher activité)
- Consomme : ExpenseOperator (dépenses), MiyuHR (hiérarchie), MiyuNotify (notifications)
- Expose : `expense.approve`, `expense.refuse`, `expense.reset`

**Mandat de Permission requis :**
- Approbation : Mandat avec StrongFather (décision approve) + KindMother (WriteIntent approval_state) + Master Butler (can_approve)
- Refus : Mandat avec StrongFather (décision refuse) + KindMother (WriteIntent approval_state) + MiyuNotify (motif)
- Reset : Mandat avec StrongFather (décision reset) + KindMother (WriteIntent state) + pas d’écriture postée (Ever Buddy / Comptabilité)

### 2.3 ExpensePostOperator

**Rôle :** Comptabilisation et paiement (post des dépenses approuvées, remboursement employé).

**Capacités :**
- Post dépenses approved : company_account (création paiement + move) ou own_account (wizard in_receipt)
- Enregistrement remboursement (action_pay sur in_receipt)
- Préparation écritures (_prepare_receipts_vals, _prepare_payments_vals, _prepare_move_lines_vals)
- Compte de charge (_get_base_account), compte destination (_get_expense_account_destination)
- Contraintes : une dépense par paiement (company_account), work_contact_id obligatoire (own_account)

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision de post (qui peut poster)
- **KindMother** : Persistance des écritures et paiements (WriteIntent)
- **Master Butler** : Permissions post / pay (comptable)
- **WorrySentinel** : Niveau critique (données financières)
- **Ever Buddy** : Cohérence état dépense ↔ état move/payment

**Contrat d'équipe :**
- Consommé par : ExpenseUI (bouton Post / Pay)
- Consomme : ExpenseOperator (dépenses approved), MiyuInvoice / MiyuCptaLedger (écritures, paiements), MiyuHR (work_contact_id, primary_bank_account_id)
- Expose : `expense.post`, `expense.pay`

**Mandat de Permission requis :**
- Post : Mandat avec KindMother (WriteIntent account.move / account.payment) + StrongFather (décision post) + state == approved
- Pay : Mandat avec KindMother (WriteIntent payment) + StrongFather (décision pay) + move state posted

### 2.4 ExpenseCategoryOperator

**Rôle :** Catégories expensables (référentiel produit ou entité dédiée).

**Capacités :**
- Liste des catégories expensables (can_be_expensed)
- Prix par défaut (standard_price), UoM, taxes, compte de charge
- Utilisation par ExpenseOperator pour product_id, price_unit, tax_ids, account_id

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données)

**Gouvernance :**
- **KindMother** : Persistance des catégories
- **Master Butler** : Permissions lecture / configuration
- **WorrySentinel** : Niveau selon sensibilité (prix, comptes)

**Contrat d'équipe :**
- Consommé par : ExpenseOperator
- Expose : `category.list`, `category.get_defaults` (prix, taxes, compte)

**Mandat de Permission requis :**
- Lecture : Mandat avec Master Butler (expense.category.read)
- Configuration : Mandat avec KindMother (WriteIntent) + Master Butler (expense.category.write)

### 2.5 ExpenseUI

**Rôle :** Interface utilisateur Notes de frais (dashboard, listes, formulaire, wizards).

**Capacités :**
- Dashboard (To Submit, Waiting Approval, Waiting Reimbursement)
- Listes (My Expenses, To Process, All Expenses) avec filtres et groupements
- Formulaire dépense (saisie, reçu, chatter)
- Wizards : Refuse, Post (employee-paid), Split, Approve duplicate
- Création depuis pièces jointes (glisser-déposer)
- Lien alias email (astuce mailto)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **BondingBrother** : Traduction intentions utilisateur → appels Opérateurs
- **Master Butler** : Visibilité des vues et boutons selon permissions
- **Caring Nanny** : Affichage alertes (doublons, same receipt) sans décision

**Contrat d'équipe :**
- Consommé par : Utilisateur final
- Consomme : ExpenseOperator, ExpenseApprovalOperator, ExpensePostOperator, ExpenseCategoryOperator, MiyuNotify (activités)
- Expose : Écrans et actions UI (pas d’API métier directe)

**Mandat de Permission requis :**
- Toutes les actions UI sont exécutées sous Mandat délivré par StrongFather pour le flux concerné (saisie, approbation, post).

---

## 3. Contrats d'Équipe et Mandats

### 3.1 Contrat d'équipe ExpenseService

**Membres :** ExpenseOperator, ExpenseApprovalOperator, ExpensePostOperator, ExpenseCategoryOperator, ExpenseUI.

**Flux autorisés :**
- ExpenseUI → ExpenseOperator (create, update, submit, split)
- ExpenseUI → ExpenseApprovalOperator (approve, refuse, reset)
- ExpenseUI → ExpensePostOperator (post, pay)
- ExpenseOperator → ExpenseCategoryOperator (category.list, get_defaults)
- ExpenseOperator → MiyuNotify (notifications)
- ExpenseApprovalOperator → ExpenseOperator (lecture dépenses), MiyuNotify (refus, approbation)
- ExpensePostOperator → ExpenseOperator (lecture dépenses), MiyuInvoice (écritures, paiements)

**Types d'échanges :** WriteIntent (KindMother), DecisionRequest (StrongFather), PermissionRequest (Master Butler), notifications (MiyuNotify).

**Conditions préalables :** Utilisateur identifié (employé ou approbateur/comptable) ; Mandat valide pour l’action.

**Niveau de validation :** StrongFather pour approve/refuse/post ; KindMother pour toute persistance ; Master Butler pour capacités.

### 3.2 Mandats typiques

- **Employé — Soumettre** : Mandat avec ExpenseOperator (expense.submit), KindMother (WriteIntent state), StrongFather (décision submit), MiyuNotify (notification manager).
- **Manager — Approuver** : Mandat avec ExpenseApprovalOperator (expense.approve), StrongFather (décision approve), KindMother (WriteIntent approval_state), Master Butler (can_approve).
- **Comptable — Poster** : Mandat avec ExpensePostOperator (expense.post), KindMother (WriteIntent move/payment), StrongFather (décision post), state == approved.

---

## 4. Niveaux de Sécurité

| Opérateur | Niveau | Justification |
|-----------|--------|---------------|
| ExpenseUI | 1 (Standard) | Exposition interface, pas de données sensibles directes |
| ExpenseCategoryOperator | 1–2 | Données référentielles, prix/comptes selon config |
| ExpenseOperator | 2 (Sensitive) | Montants, pièces jointes, données employé |
| ExpenseApprovalOperator | 2 (Sensitive) | Décisions approbation, motifs refus |
| ExpensePostOperator | 3 (Critical) | Écritures et paiements |

**Règles WorrySentinel :**
- Un flux ne peut pas descendre en niveau (ex. pas de post sans niveau 3 sur le Mandat).
- Données pièces jointes et montants : niveau 2 minimum ; écritures comptables : niveau 3.
- Isolation cross-société : can_approve / can_reset tiennent compte de valid_company_ids.

---

## 5. Intégration avec les Cores

- **StrongFather** : Décision submit, approve, refuse, reset, post ; émission des Mandats pour ExpenseService.
- **KindMother** : Toute persistance (dépense, état, écriture, paiement) via WriteIntent.
- **Master Butler** : Capacités expense.create, expense.update, expense.submit, expense.approve, expense.refuse, expense.reset, expense.post, expense.pay ; permissions selon groupes et hiérarchie.
- **WorrySentinel** : Niveaux sécurité 1–3 ; états de confiance (T0–T4) pour bloquer post si environnement dégradé si politique l’exige.
- **Caring Nanny** : Alertes doublons et same receipt (observation, pas de blocage automatique sauf politique).
- **Ever Buddy** : Transitions d’état (draft → submitted → approved → posted → paid / refused) ; compatibilité reset avec comptabilité.
- **TAMR** : Points d’intervention humaine (approbation, refus, reset, post) avec traçabilité pour audit.

---

## 6. Synthèse

Le service **MiyukiniExpenses** / **MiyuExpense** est porté par l’équipe **ExpenseService**, composée de cinq Opérateurs (Expense, Approval, Post, Category, UI), avec Contrats d’équipe et Mandats de Permission explicites, niveaux de sécurité 1–3, et intégration complète avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, TAMR). Ces spécifications servent de base au guide d’intégration COG et au guide d’implémentation.
