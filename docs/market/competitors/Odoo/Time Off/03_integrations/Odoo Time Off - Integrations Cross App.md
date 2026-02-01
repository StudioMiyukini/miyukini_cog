# Odoo Time Off — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Time Off** (Congés / Absences) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** `__manifest__.py` hr_holidays, documentation Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs et hooks (conceptuels)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (`__manifest__.py`)

**Dépendances explicites du module `hr_holidays` :**

| Module | Rôle |
|--------|------|
| **hr** | Employés, départements, fiche employé (Work Information — Time Off Approver) |
| **calendar** | Synchronisation agenda : création d'événement/réunion quand un congé est validé (si type de congé configuré avec meeting type) |
| **resource** | Calendrier de travail (`resource.calendar`), jours non travaillés (`resource.calendar.leaves`), calcul des jours/heures de congé |

**Données partagées :**
- **hr.employee** : employé, département, manager, Time Off Approver (pour validation)
- **resource.calendar** : plage horaire de travail (calcul nombre de jours/heures entre date_from et date_to)
- **resource.calendar.leaves** : jours fériés et absences globales (exclusion du calcul et option "Public Holiday Included")
- **calendar** : événements créés à la validation d'un congé (optionnel, selon configuration type)

### 1.2 Modules optionnels (intégrations si installés)

| Module | Rôle |
|--------|------|
| **hr_timesheet** | Lignes de timesheet pour les jours de congé (projet/tâche définis sur le type de congé — souvent en mode développeur) |
| **hr_payroll** | Work Entry Type sur `hr.leave.type` → génération de work entries pour la paie |
| **mail** | Chatter, activités, notifications sur demandes et allocations |

**Flux :**
- **Timesheet** : à la validation d'un congé, création de lignes `account.analytic.line` (projet/tâche du type) pour les jours/heures de congé
- **Payroll** : les work entries (heures de congé) sont générées à partir du type de congé et du calendrier ; rémunération selon règles paie
- **Mail** : abonnements, notifications (demande soumise, validée, refusée, annulée)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec HR (hr)

**Flux :**
```
Time Off ← hr.employee (employé, département, manager, time_off_approver)
         ← hr.department (filtres, rapports par département)
```

**Mécanismes :**
- **hr.leave** et **hr.leave.allocation** : champs `employee_id`, `department_id` (related), `manager_id` (related)
- **Validation** : si type "By Employee's Approver", l'approbateur est lu depuis `hr.employee` (onglet Work Information — Time Off Approver)
- **Allocations par département** : champs `department_id` sur allocation pour cibler un département
- **Rapports** : Time Off by Department utilise `hr.department`

**Champs liés :**
- `employee_id`, `department_id`, `company_id` (souvent related employee)
- `hr.employee.time_off_approver_id` (ou champ équivalent) pour l'approbateur congés

**Recommandations Miyukini :**
- Opérateur Time Off consomme l'Opérateur HR (Employés) pour employé, département, approbateur
- Contrat d'équipe TimeOffService ↔ MiyuHR (lecture fiche employé, approbateur)
- Pas de duplication : une seule source de vérité pour "qui valide les congés" (fiche employé)

### 2.2 Intégration avec Resource (resource)

**Flux :**
```
Time Off → resource.calendar (calcul jours/heures travaillés)
         → resource.calendar.leaves (jours fériés, exclusions)
```

**Mécanismes :**
- **Calcul du nombre de jours/heures** d'une demande : entre `date_from` et `date_to`, en excluant les jours non travaillés (week-end, jours fériés si "Public Holiday Included")
- **Unité** : Day (8h), Half Day (4h), Hours selon `hr.leave.type`
- **Jours fériés** : configurés dans Time Off (Configuration > Public Holidays) et reflétés dans `resource.calendar.leaves` (ou modèle dédié selon version) ; utilisés aussi par Planning, Manufacturing, etc.
- **Calendrier employé** : `resource.calendar_id` sur `hr.employee` ou sur contrat de travail

**Hooks (conceptuels) :**
- Fonction de calcul "nombre de jours entre deux dates selon calendrier et unité"
- Exclusion des jours fériés (et optionnellement mandatory days) du décompte

**Recommandations Miyukini :**
- Utiliser un Kit ou Opérateur Calendar/Resource pour le calcul "jours travaillés entre deux dates" et la liste des jours fériés
- Jours fériés et jours obligatoires : une seule source (partagée Time Off, Planning, Manufacturing)
- KindMother pour persistance des congés ; calcul en lecture des soldes et des quantités

### 2.3 Intégration avec Calendar (calendar)

**Flux :**
```
Time Off (congé validé) → calendar.event (création réunion/événement)
```

**Mécanismes :**
- Sur **validation** d'un `hr.leave`, création optionnelle d'un événement dans l'agenda interne (CRM Meetings ou calendar)
- Configuration par **type de congé** : type de réunion/événement associé (ex. "Congés", "Absence")
- Objectif : affichage des congés dans l'agenda et évitement des conflits de réunions

**Champs / modèles :**
- Lien `hr.leave` → `calendar.event` (one2many ou many2one selon implémentation)
- Données copiées : dates, employé, libellé, type d'événement

**Recommandations Miyukini :**
- Option "Créer un événement à la validation" sur le type de congé
- BondingBrother : intention "congé validé" → création événement via Opérateur Calendar/Agenda
- Mandat entre TimeOffOperator et CalendarOperator pour créer/supprimer l’événement à la validation/annulation

### 2.4 Intégration avec Mail (mail)

**Flux :**
```
Time Off (demande/allocation) → mail.thread (chatter, followers, activities)
                              → mail.message (notifications validation/refus)
```

**Mécanismes :**
- **hr.leave** et **hr.leave.allocation** héritent de `mail.thread` (chatter)
- Abonnements : demandeur, approbateur(s), Time Off Officer
- **Notifications** : demande soumise (→ approbateur), demande validée/refusée (→ employé), allocation créée/validée
- **Activités** : rappels "demande en attente" pour l’approbateur
- **Subtypes** : types de messages (demande créée, validée, refusée, annulée)

**Recommandations Miyukini :**
- MiyuNotify pour toutes les notifications (soumission, validation, refus, annulation)
- Pas de duplication de la logique métier dans le canal : Time Off émet les événements, Notify diffuse
- Contrat d’équipe TimeOffService ↔ MiyuNotify (notification gouvernée, pas de spam)

### 2.5 Intégration avec Payroll (hr_payroll)

**Flux :**
```
Time Off (type avec Work Entry Type) → hr.work.entry (génération entrées travail congé)
```

**Mécanismes :**
- Sur **hr.leave.type** : champ **Work Entry Type** (lien vers `hr.work.entry.type`)
- Lors de la **validation** d’un congé (ou batch paie), génération de **work entries** pour la période du congé (dates, employé, type d’entrée = congé)
- La paie utilise ces work entries pour calculer la rémunération (congés payés, maintien de salaire, etc.)

**Données :**
- Période du congé (`date_from`, `date_to`), employé, type de congé → type d’entrée travail
- Calendrier de travail pour découper en jours/heures selon règles paie

**Recommandations Miyukini :**
- Si module Paie existe : un type de congé peut être lié à un "type d’entrée travail" ; génération des entrées à la validation ou en batch
- Contrat d’équipe TimeOffService ↔ MiyuPayroll (ou équivalent) : écriture des entrées travail en WriteIntent, sans duplication des règles de paie dans Time Off

### 2.6 Intégration avec Timesheet (hr_timesheet)

**Flux :**
```
Time Off (congé validé) → account.analytic.line (lignes timesheet "congé")
```

**Mécanismes :**
- **hr.leave.type** (souvent en mode développeur) : champs **Project** et **Task**
- À la **validation** d’un congé, création de lignes **analytic** (timesheet) pour chaque jour/heure de congé, sur le projet et la tâche du type
- Objectif : suivre le temps "non travaillé" (congé) dans les mêmes projets/analytics que le temps travaillé

**Données :**
- Projet, tâche, employé, dates, quantité (heures/jours), type de ligne "congé"

**Recommandations Miyukini :**
- Option sur le type de congé : projet + tâche pour les lignes de timesheet
- Création des lignes à la validation du congé via Opérateur Timesheet / Comptabilité analytique
- Contrat d’équipe TimeOffService ↔ MiyuTimesheet (ou MiyuInvoice analytic) ; WriteIntent pour les lignes

---

## 3. Synthèse des flux

| App cible | Données / événement | Sens |
|-----------|---------------------|------|
| **hr** | Employé, département, approbateur | Time Off lit |
| **resource** | Calendrier, jours fériés, calcul jours/heures | Time Off lit / écrit (fériés) |
| **calendar** | Événement "congé" | Time Off écrit à la validation |
| **mail** | Chatter, notifications | Time Off écrit (messages, abonnements) |
| **hr_payroll** | Work entry type, work entries | Time Off lit (type) ; Payroll écrit (entries) ou Time Off déclenche |
| **hr_timesheet** | Lignes analytic (congé) | Time Off déclenche écriture à la validation |

---

## 4. APIs et hooks (conceptuels)

- **Calcul jours/heures** : fonction utilisant `resource.calendar` et `resource.calendar.leaves` (ou équivalent)
- **Validation** : transition d’état `hr.leave` (confirm → validate1/validate) ; après validation : création événement calendar, work entries, timesheet, notifications
- **Annulation** : wizard ou action → état cancel ; recrédit solde ; suppression ou annulation des work entries / timesheet / événement selon règles
- **Cron / batch** : plans d’acquisition (accrual) — génération périodique d’allocations selon règles
- **Rapports** : lecture `hr.leave`, `hr.leave.allocation`, `hr.employee`, `hr.department` pour Time Off Summary, By Department, Analysis

---

## 5. Recommandations pour Miyukini

- **Équipe TimeOffService** : Opérateurs Time Off (demandes, allocations, types, accrual, fériés, jours obligatoires) en collaboration mandatée avec HR, Resource/Calendar, Notify, Payroll, Timesheet.
- **Contrats d’équipe** : définir qui lit/écrit quoi (employé, calendrier, fériés, événements, work entries, lignes analytic, notifications).
- **Mandats de Permission** : validation des congés et création d’allocations sous Mandat (StrongFather, Master Butler) ; création d’événements et de lignes timesheet sous Mandat avec les Opérateurs concernés.
- **Source unique** : employés et approbateurs dans HR ; jours fériés et calendrier dans Resource/Calendar ; pas de duplication des données maîtres.
- **WriteIntent** : toute création/modification de congé, allocation, type, férié, jour obligatoire passe par KindMother ; les écritures vers Calendar, Payroll, Timesheet sont des intentions ou appels gouvernés, pas des écritures directes non tracées.

---

## Références

- Odoo GitHub — `addons/hr_holidays/__manifest__.py`
- Documentation Odoo 19.0 — Time Off, Employees (Work Information), Payroll (Work entries), Timesheets
- Analyses Miyukini : Project (Integrations Cross App), Employees
