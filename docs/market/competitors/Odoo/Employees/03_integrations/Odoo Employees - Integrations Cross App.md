# Odoo Employees — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Employees** (Employés) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0 (Employees, Payroll, Recruitment, Expenses, Time Off, Timesheets, Attendances, Appraisals, Planning, Fleet, Manufacturing)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres modules Odoo (base, mail, Payroll, Recruitment, Expenses, Time Off, Timesheets, Attendances, Appraisals, Planning, Fleet, Manufacturing)
- Flux de données inter-apps
- Mécanismes d'intégration (employé comme ressource centrale)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Module HR (hr) — base Employees

**Contenu :**
- `hr.employee` : Fiche employé (identité, hiérarchie, contact, paramètres)
- `hr.department` : Départements (manager, parent)
- Modèles liés : certifications, badges, équipements, offboarding, retention

**Dépendances typiques :**
- **base** : res.company, res.users, res.partner
- **mail** : Chatter, activités, notifications (optionnel sur fiche employé)

### 1.2 Modules consommateurs d’Employees

Les applications suivantes **consomment** le modèle employé et les champs Employees :

| App | Usage principal |
|-----|------------------|
| **Payroll** | Contrats (hr.contract), horaires (resource.calendar), paie, compte bancaire, champs légaux |
| **Recruitment** | Postes (hr.job) pour Job Position sur la fiche employé |
| **Expenses** | employee_id, department_id, manager_id, expense approver |
| **Time Off** | employee_id, time off approver |
| **Timesheets** | employee_id, timesheet approver, horaires |
| **Attendances** | employee_id, PIN, Badge ID, présence, kiosque |
| **Appraisals** | employee_id, Next Appraisal Date, historique évaluations |
| **Planning** | employee_id, Roles, Default Role |
| **Fleet** | Fleet Mobility Card |
| **Manufacturing** | Hourly Cost (work center) |

---

## 2. Flux de Données

### 2.1 Employees comme source

```
hr.employee (fiche centrale)
    ├── hr.department (department_id, manager via department)
    ├── res.users (user_id — optionnel)
    ├── res.partner (work_contact_id, address_id, address_home_id)
    ├── resource.calendar (resource_calendar_id — Payroll)
    ├── hr.contract (contrats — Payroll)
    ├── res.bank (bank_account_id)
    └── Approvers (Expense, Time Off, Timesheet, Attendance) → res.users
```

**Flux sortants (Employees → autres apps) :**
- **Expenses** : employee_id, department_id, manager_id (approbation), work_contact_id (remboursement)
- **Time Off** : employee_id, time off approver
- **Timesheets** : employee_id, timesheet approver, resource_calendar_id
- **Attendances** : employee_id, PIN, badge_id, présence
- **Payroll** : employee_id, contrats, horaires, compte bancaire, champs paie
- **Appraisals** : employee_id, next_appraisal_date
- **Planning** : employee_id, roles, default_role
- **Fleet** : fleet_mobility_card_id
- **Manufacturing** : hourly_cost (work center)

### 2.2 Employees comme consommateur

**Flux entrants (autres apps → Employees) :**
- **Payroll** : Working Schedules (resource.calendar) listés dans Employees pour « Company Working Hours » et « Working Hours » par employé
- **Recruitment** : hr.job (postes) pour liste déroulante Job Position
- **Configuration** : Presence Display (attendances ou user status ou avancé) ; Skills Management ; Remote Work ; Work Locations

---

## 3. Intégrations Détaillées

### 3.1 Payroll

**Données partagées :**
- **Contrats** : hr.contract lié à hr.employee (dates, type, salaire)
- **Horaires** : resource.calendar (Working Schedules dans Payroll) ; sélectionnables sur fiche employé (Working Hours) et en « Company Working Hours » (Settings)
- **Compte bancaire** : res.partner.bank (Trusted obligatoire pour paie)
- **Champs paie** : Legal Name, Payslip Language, Registration Number, localisation

**Règles :**
- Les horaires d’un employé doivent être parmi les horaires configurés pour la société.
- En multi-company, chaque société a ses propres Working Schedules.
- Compte bancaire non Trusted bloque les paiements / paie.

### 3.2 Recruitment

**Données partagées :**
- **hr.job** : Postes configurés dans Recruitment ; liste déroulante « Job Position » sur la fiche employé.
- Le champ texte libre « Job Position » sous le nom peut différer (ex. « Sales Representative - Subscriptions »).

### 3.3 Expenses

**Données partagées :**
- **employee_id** : Employé sur la dépense (requis).
- **department_id**, **manager_id** : Déduits de l’employé pour approbation.
- **Expense approver** : Champ approbateur sur la fiche employé ; l’utilisateur doit avoir droits Team Approver / All Approver / Administrator dans Expenses.

**Flux :** Création dépense → employé et manager dérivés ; soumission → notification à l’approbateur (Expense) ; remboursement → work_contact_id, primary_bank_account_id.

### 3.4 Time Off

**Données partagées :**
- **employee_id** sur les demandes de congé.
- **Time Off approver** : Champ sur la fiche employé ; l’utilisateur doit avoir droits Officer: Manage all Requests ou Administrator dans Time Off.

### 3.5 Timesheets

**Données partagées :**
- **employee_id** sur les feuilles de temps.
- **Timesheet approver** : Champ sur la fiche employé ; l’utilisateur doit avoir droits Officer: Manage all contracts ou Administrator dans Payroll.
- **Working Hours** : resource_calendar_id pour calcul des heures attendues.

### 3.6 Attendances

**Données partagées :**
- **employee_id** : Pointages (check-in / check-out).
- **PIN**, **Badge ID** : Connexion kiosque et POS.
- **Presence Display** : Mode « Based on attendances » utilise les pointages pour statut présent/absent.
- **Attendance approver** : Champ sur la fiche employé ; nécessite droits Administrator dans Payroll pour apparaître.

### 3.7 Appraisals

**Données partagées :**
- **employee_id** : Évaluations.
- **Next Appraisal Date** : Champ sur la fiche employé (visible si app installée) ; calculé selon paramètres Appraisals (ex. 6 mois).

### 3.8 Planning

**Données partagées :**
- **employee_id** : Affectation aux plannings.
- **Roles**, **Default Role** : Champs dans Work Information (Planning) ; rôles configurés dans l’app Planning.

### 3.9 Fleet

**Données partagées :**
- **Fleet Mobility Card** : Champ sur la fiche employé (Settings → Application Settings).

### 3.10 Manufacturing

**Données partagées :**
- **Hourly Cost** : Coût horaire employé (Settings) ; utilisé dans les work centers pour coût de main-d’œuvre.

---

## 4. Mécanismes d’Intégration

### 4.1 Employé comme ressource centrale

- Une fiche **hr.employee** est la référence pour identité, hiérarchie, contact travail, approbateurs et paramètres par domaine.
- Les apps (Expenses, Time Off, etc.) lisent employee_id et champs dérivés (department_id, manager_id, approvers) sans dupliquer la structure.

### 4.2 Approbateurs et droits

- Les champs approbateurs (Expense, Time Off, Timesheet, Attendance) sont des **res.users**.
- Seuls les utilisateurs ayant les droits requis dans l’app correspondante apparaissent dans les listes (ou sont éligibles).
- Vérification des droits : Settings app → Manage Users → Access Rights → HUMAN RESOURCES et rôles par app.

### 4.3 Présence (Presence Display)

- **Based on attendances** : Données de l’app Attendances (check-in/out).
- **Based on user status** : Statut de connexion utilisateur (user_id).
- **Advanced** : Signaux opérationnels (e-mails envoyés, IP) — calcul côté Employees ou module dédié.

### 4.4 Horaires (Work organization)

- **Company Working Hours** (Settings) et **Working Hours** (fiche employé) utilisent **resource.calendar**.
- Création/modification des plannings dans Payroll (Working Schedules) ; sélection dans Employees.
- Contrainte : les horaires de l’employé doivent appartenir aux horaires de la société.

---

## 5. Recommandations pour Miyukini

1. **Opérateur HR central** : Un Opérateur (MiyuHR / EmployeeOperator) comme source de vérité pour employé, département, hiérarchie, contact travail et approbateurs ; les autres Opérateurs (Expenses, Time Off, etc.) consomment via BondingBrother et Mandats.
2. **Approbateurs** : Modéliser par domaine (expense_approver_id, time_off_approver_id, etc.) avec vérification des capacités (Master Butler) côté chaque app ; exposition claire des droits requis.
3. **Horaires** : Réutiliser un référentiel « Working Schedules » (Ever Buddy pour versions) partagé entre HR et Payroll/Timesheets ; contraintes de cohérence (société, employé) gérées par KindMother / StrongFather.
4. **Présence** : Traiter les trois modes (pointage, connexion, avancé) comme options configurables ; source de données (Attendances, Session, Signaux) injectée ou interrogée par un même service (Caring Nanny si états de confiance).
5. **Données sensibles** : Données privées et paie en niveau de sécurité élevé (WorrySentinel) ; accès en lecture/écriture strictement mandaté ; pas d’exposition transversale non gouvernée.
6. **Multi-société** : company_id cohérent avec COG / environnements ; isolation des référentiels (départements, horaires, approbateurs) par société.

---

**Document** : Odoo Employees — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
