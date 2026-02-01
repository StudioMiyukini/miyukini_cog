# Odoo Expenses — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Expenses** (Notes de frais) d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Code source GitHub Odoo 19.0, documentation utilisateur Odoo 19

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs (Employé, Manager, Officer, Comptable)
- Parcours d'onboarding
- Scénarios d'usage principaux (saisie, rapport, approbation, remboursement)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Employé (Employee)

**Profil :**
- Rôle opérationnel : Saisie et soumission des notes de frais
- Responsabilités :
  - Créer des dépenses (manuel, pièce jointe, email)
  - Renseigner catégorie, montant, date, reçu
  - Soumettre les dépenses au manager
  - Consulter le statut (To Submit, Waiting Approval, Waiting Reimbursement)
  - Répondre aux refus ou demandes de précision

**Besoins :**
- Saisie rapide (formulaire, glisser-déposer reçu, email)
- Dashboard « Mes dépenses » par statut
- Notifications (soumis, approuvé, refusé, payé)
- Historique et pièces jointes

**Permissions :**
- Peut créer/modifier ses propres dépenses en brouillon
- Peut soumettre ses dépenses
- Ne peut pas approuver ses propres dépenses
- Accès limité à ses dépenses et rapports

### 1.2 Manager (Expense Manager / HR Manager)

**Profil :**
- Rôle de validation : Approbation des notes de frais de son équipe ou département
- Responsabilités :
  - Recevoir les dépenses soumises (activité « Review this expense »)
  - Approuver ou refuser avec motif
  - Réinitialiser en brouillon si besoin (avant post)
  - Consulter les rapports et dépenses de son périmètre

**Besoins :**
- Vue « À approuver » / « To process »
- Filtres par employé, département, période
- Alertes doublons / reçus identiques
- Historique des décisions

**Permissions :**
- `group_hr_expense_user` ou `group_hr_expense_manager`
- Peut approuver/refuser selon hiérarchie (expense_manager_id, department manager, parent)
- Ne peut pas approuver ses propres dépenses (sauf Manager global)
- Peut éditer dépenses soumises/approuvées dans son périmètre

### 1.3 Officer (HR Officer / Team Approver)

**Profil :**
- Rôle intermédiaire : Approbation des dépenses de son équipe ou département
- Responsabilités :
  - Même flux que Manager mais avec périmètre limité (équipe / département)
  - Soumission possible pour les dépenses des employés dont il est manager/officer

**Besoins :**
- Vue ciblée sur les dépenses de son périmètre
- Règles claires (qui peut approuver qui)

**Permissions :**
- `group_hr_expense_team_approver`
- Périmètre : employés dont il est expense_manager, manager de département, ou parent (child_of employee_ids)

### 1.4 Comptable (Billing Accountant)

**Profil :**
- Rôle financier : Comptabilisation et paiement
- Responsabilités :
  - Poster les dépenses approuvées (écritures, paiements)
  - Enregistrer les remboursements employé
  - Vérifier journal, compte, analytique

**Besoins :**
- Liste des dépenses approuvées à poster
  - Company-paid : création paiement + move
  - Employee-paid : wizard « Post expenses » (in_receipt)
- Accès aux écritures et paiements liés

**Permissions :**
- Droits comptables (account) + accès aux dépenses en état approved
- Post possible uniquement si state == approved et payment_mode défini

---

## 2. Parcours d'Onboarding

### 2.1 Premier accès Employé

1. **Activation module** : Admin installe Expenses (hr_expense).
2. **Employé** : L’employé doit exister (hr.employee) et être lié à l’utilisateur ; `filter_for_expense = True` si restriction par catégorie.
3. **Produits expensables** : Au moins un `product.product` avec `can_be_expensed = True` (catégories de dépenses).
4. **Configuration société** : Compte de charge par défaut (expense_account_id), journal achat / dépenses si besoin.
5. **Optionnel** : Alias email pour envoyer les reçus par mail (use_mailgateway, mail_alias_expense).

### 2.2 Premier accès Manager / Officer

1. **Groupes** : Attribution `group_hr_expense_team_approver` ou `group_hr_expense_user` / `group_hr_expense_manager`.
2. **Hiérarchie** : Définir expense_manager_id et/ou département (manager_id) sur les employés pour que les approbateurs voient les bonnes dépenses.
3. **Activités** : Les dépenses soumises créent une activité « Review this expense » assignée au manager_id (ou responsable par défaut).

### 2.3 Premier accès Comptable

1. **Comptabilité** : Module account installé et configuré (journaux, comptes, méthodes de paiement).
2. **Société** : company_expense_allowed_payment_method_line_ids ou journal achat avec méthode de paiement outbound pour company_account.
3. **Employee-paid** : expense_journal_id ou journal type purchase pour in_receipt.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Saisie d’une dépense (Employé)

1. Aller dans **Expenses → Mes dépenses → Mes dépenses** (ou dashboard).
2. **Créer** une dépense :
   - Catégorie (produit expensable)
   - Date, montant (ou quantité × prix si produit avec coût)
   - Devise si multi-devises
   - Pièce jointe (reçu) — optionnel mais recommandé
   - Notes internes
3. **Enregistrer** (brouillon).
4. Option : **Soumettre** directement ou ajouter à un rapport puis soumettre le rapport (selon doc Odoo 19 « Expense reports »).

**Variantes :**
- **Création depuis pièce jointe** : Glisser-déposer un ou plusieurs reçus → création d’une dépense par fichier (catégorie par défaut, nom « Untitled Expense [date] »).
- **Création par email** : Envoyer un email à l’alias avec objet contenant code produit et montant (ex. « Lunch with customer $12.32 ») → création automatique + mail de confirmation.

### 3.2 Création et soumission d’un rapport de dépenses (Employé)

1. **Expenses → Mes dépenses → Mes dépenses** (ou dashboard).
2. Cocher les dépenses à inclure (statut To Report / To Submit uniquement ; les Approved ne sont pas sélectionnables).
3. Cliquer **Create Report** : toutes les dépenses en To Submit non déjà sur un autre rapport sont ajoutées au nouveau rapport.
4. Renseigner **Expense Report Summary** (ex. « Client Trip NYC »).
5. Sélectionner un **Manager** pour la revue.
6. **Submit To Manager** : le rapport passe en soumis ; les dépenses passent en submitted ; activité créée pour le manager.

**Contraintes (doc)** : Un rapport doit être soumis individuellement (pas de soumission en lot). Si toutes les dépenses sont déjà sur un autre rapport, message « You have no expenses to report ».

### 3.3 Approbation / Refus (Manager / Officer)

1. **Expenses → To Process** (ou activité « Review this expense »).
2. Ouvrir la dépense (ou le rapport).
3. Vérifier catégorie, montant, reçu, analytique.
4. **Approuver** : si doublons détectés, wizard de confirmation ; sinon passage en approved.
5. Ou **Refuser** : saisir le motif dans le wizard ; la dépense passe en refused, notification à l’employé.

**Règles** : Pas d’approbation de sa propre dépense (sauf Manager global). Reset possible uniquement si aucune écriture postée.

### 3.4 Comptabilisation et remboursement (Comptable)

1. **Dépenses payées par l’entreprise (company_account)** :
   - Sélectionner les dépenses approuvées, **Post** : création du paiement + move, état passé à paid (court-circuit).
2. **Dépenses à rembourser (own_account)** :
   - **Post** ouvre le wizard « Post expenses paid by the employee » : création des in_receipt (un par employé ou regroupement), puis action_post sur les moves.
   - Ensuite **Register Payment** sur la facture (in_receipt) pour enregistrer le remboursement → in_payment puis paid.

**Contraintes** : Post uniquement pour dépenses approved ; payment_mode obligatoire ; company_account et employee-paid ne peuvent pas être mélangés dans le même wizard (employee-paid uniquement dans le wizard).

### 3.5 Consultation du statut et historique (Employé)

1. **Dashboard** : To Submit, Waiting Approval, Waiting Reimbursement (montants par statut).
2. **Mes dépenses** : liste avec statut, montant, manager, date.
3. **Mes rapports** : liste des rapports avec statut (To Submit, Submitted, Approved).
4. **Chatter** : commentaires, pièces jointes, historique des états (soumis, approuvé, refusé, payé).

---

## 4. Points de Friction Identifiés

### 4.1 Saisie et reçus

- **Catégorie obligatoire à la soumission** : En brouillon, product_id peut être vide (création par email) ; à la soumission, « You can not submit an expense without a category ».
- **Reçus identiques** : Alerte same_receipt_expense_ids mais pas de blocage automatique ; risque de doublons si l’utilisateur ignore.
- **Doublons** : Alerte duplicate_expense_ids à l’approbation avec wizard de confirmation ; pas de fusion automatique.

### 4.2 Workflow et rapports

- **Rapports vs dépenses seules** : Documentation Odoo 19 décrit des « expense reports » (regroupement) ; le code analysé gère l’état au niveau de chaque hr.expense. Les deux logiques coexistent (rapports pour présentation, dépenses pour workflow).
- **Soumission une par une** : Les rapports doivent être soumis individuellement (pas de batch).
- **Reset** : Impossible de remettre en brouillon si une écriture est déjà postée ; nécessité d’annuler côté comptabilité d’abord.

### 4.3 Droits et hiérarchie

- **Responsable par défaut** : Si aucun manager_id ni expense_manager_id, la dépense peut être auto-validée à la soumission ; selon les organisations, cela peut être indésirable.
- **Multi-société** : can_reset / can_approve tiennent compte de valid_company_ids ; l’utilisateur doit avoir accès à la société de la dépense.
- **Édition** : Champs sensibles (tax_ids, analytic_distribution, account_id, manager_id) protégés ; message « You can’t edit this expense » si is_editable False.

### 4.4 Comptabilité

- **Compte de charge** : Si aucun compte trouvé (dépense, produit, société, journal), message d’erreur explicite pour configurer un compte de charge.
- **Company-paid** : Une seule dépense par paiement (contrainte _check_o2o_payment).
- **Employee sans work_contact_id** : Impossible de poster en own_account ; message « No work contact found for the employee ».

---

## 5. Recommandations pour Miyukini

### 5.1 Parcours Employé

- **MiyuExpense (Opérateur de Service)** : Saisie unifiée (formulaire + pièces jointes + option email), dashboard « Mes dépenses » par statut, notifications (MiyuNotify) à chaque changement d’état.
- **Validation catégorie** : Obligation de catégorie avant soumission ; aide à la catégorie depuis reçu (si module OCR/IA disponible plus tard).
- **Alertes** : Exposer same_receipt et duplicate comme avertissements clairs (Caring Nanny / WorrySentinel) sans bloquer tant que la gouvernance (StrongFather) n’exige pas de blocage.

### 5.2 Parcours Manager / Officer

- **Mandat de Permission** : Chaque action (approve, refuse, reset) sous Mandat StrongFather + Master Butler (permissions) + WorrySentinel (niveau sécurité).
- **Vue « À traiter »** : Filtre côté Opérateur sur dépenses submitted et manager_id / expense_manager_id / département selon droits.
- **Refus avec motif** : Toujours tracé (WriteIntent / KindMother) et notifié (MiyuNotify).

### 5.3 Parcours Comptable

- **Intégration MiyuInvoice / Comptabilité** : Post = WriteIntent vers KindMother (écritures) + intégration paiements ; pas de post sans Mandat et sans état approved.
- **Séparation company vs employee** : Deux flux explicites (company_account vs own_account) comme dans Odoo, avec contrats d’équipe clairs entre ExpenseOperator et l’Opérateur comptable.

### 5.4 Général

- **Rapports** : Modéliser un regroupement « Rapport de dépenses » (équipe d’Opérateurs ou agrégat) pour soumission en lot côté UI tout en gardant la granularité par dépense pour la gouvernance et la comptabilité.
- **Ever Buddy** : Gestion des états de vie (draft → submitted → approved → posted → paid / refused) et compatibilité des changements (reset, annulation) avec la comptabilité.
- **TAMR** : Points d’intervention humaine clairs (approbation, refus, reset, post) avec traçabilité pour audit.

Ces recommandations alimentent les spécifications Opérateurs Miyukini et le guide d’intégration COG pour Expenses.
