# Odoo Time Off — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Time Off** (Congés / Absences) d'Odoo (version 19.0), extraite de la documentation officielle et du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, `https://github.com/odoo/odoo/tree/19.0/addons/hr_holidays`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (hr.leave.type, hr.leave, hr.leave.allocation, hr.leave.accrual, hr.leave.mandatory.day)
- Règles métier et contraintes (validation, allocations, soldes)
- Workflows et transitions d'état (demandes de congés, allocations)
- Types de congés et plans d'acquisition (accrual)
- Jours fériés et jours obligatoires
- Intégration avec HR, Calendar, Resource, Payroll, Timesheet

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `hr.leave.type` (Type de congé)

**Rôle :** Définit un **type de congé** — catégorie de temps non travaillé (congés payés, maladie, RTT, etc.).

**Champs clés (logique métier) :**

#### Identification
- `name` : Nom du type (obligatoire, ex. Congés payés, Maladie, RTT)
- `active` : Boolean (actif)
- `company_id` : Many2one vers `res.company` (entreprise, optionnel, multi-société)

#### Validation des demandes de congé
- **Time Off Requests** :
  - `approval` : Sélection — No Validation, By Time Off Officer, By Employee's Approver, By Employee's Approver and Time Off Officer
  - `time_off_officer_id` : Many2one vers `res.users` (officier congés notifié et responsable des validations)

#### Validation des demandes d'allocation
- **Allocation Requests** :
  - `requires_allocation` : Boolean — Yes / No Limit (allocation obligatoire ou non)
  - `employee_requests` : Sélection — Extra Days Requests Allowed / Not Allowed (demandes de jours supplémentaires)
  - `approval` : Même options que pour les demandes de congé

#### Configuration
- **Take Time Off in** : Day (journée 8h), Half Day (demi-journée 4h), Hours (heures)
- **Deduct Extra Hours** : Boolean — déduire les heures sup des demandes de congé
- **Public Holiday Included** : Boolean — inclure les jours fériés dans la demande (ne pas consommer de jours congés pour un jour férié)
- **Allow To Attach Supporting Document** : Boolean — pièce jointe autorisée (ex. arrêt maladie)
- **Kind of Time Off** : Worked Time (compte pour l'acquisition) ou Absence (ne compte pas)
- **Negative cap** : Allow Negative Cap + Maximum Excess Amount (jours) — autoriser solde négatif avec plafond

#### Payroll
- **Work Entry Type** : Lien vers type d'entrée de travail (Payroll) pour génération des work entries

#### Timesheets (mode développeur)
- **Project** / **Task** : Projet et tâche pour les entrées timesheet liées au congé

#### Affichage
- **Color** / **Cover Image** : Couleur et icône dans le tableau de bord Time Off

**Règles métier :**
- Champs obligatoires minimaux : name, Take Time Off In, Kind of Time Off, et configuration Time Off Requests / Allocation Requests
- Si `requires_allocation` = No Limit, les options d'allocation (Employee Requests, etc.) ne s'appliquent pas
- Un type peut être limité à une company ou commun à toutes

---

### 1.2 Modèle `hr.leave` (Demande de congé)

**Rôle :** Représente une **demande de congé** soumise par un employé — période de temps non travaillé (dates, type, statut).

**Champs clés :**

#### Identification et employé
- `name` : Libellé (souvent calculé ou saisi)
- `employee_id` : Many2one vers `hr.employee` (employé, obligatoire)
- `company_id` : Many2one vers `res.company` (entreprise, dérivé de l'employé)

#### Type et période
- `holiday_status_id` : Many2one vers `hr.leave.type` (type de congé, obligatoire)
- `date_from` : Datetime (début du congé)
- `date_to` : Datetime (fin du congé)
- **Take Time Off in** : hérité du type (Day / Half Day / Hours)

#### Quantités
- `number_of_days` : Nombre de jours (calculé selon calendrier de travail et type)
- `number_of_hours` : Nombre d'heures (si prise en heures)
- Déduction des heures sup si configurée sur le type
- Jours fériés exclus du décompte si "Public Holiday Included" sur le type

#### Statut et validation
- **State** : draft, confirm, validate, validate1 (si double validation), refuse, cancel
- `state` : État du workflow
- `request_date_from` / `request_date_to` : Dates demandées (parfois distinctes de date_from/date_to pour affichage)
- `request_unit_half` : Boolean (demi-journées)
- `request_unit_hours` : Boolean (heures)
- Validateur(s) : selon type — Time Off Officer et/ou Employee's Approver (défini sur fiche employé, onglet Work Information)

#### Contexte
- `department_id` : Département (related employee)
- `manager_id` : Manager (related employee)
- Pièce jointe possible si type autorise
- `message_ids` / `activity_ids` : Chatter et activités (mail)

**Règles métier :**
- Vérification du solde disponible (allocation) pour le type avant validation
- Refus possible si solde insuffisant (sauf type "No Limit" ou Negative Cap autorisé)
- Double validation : Employee's Approver puis Time Off Officer (selon type)
- Une fois validé, le congé peut être synchronisé avec l'agenda (Calendar) et/ou générer des work entries (Payroll) et des lignes timesheet (Timesheet)
- Annulation possible via wizard `hr.holidays.cancel.leave`

**Workflow :**
- draft → confirm (envoi) → validate ou validate1 → validate (approuvé) ; ou refuse ; ou cancel

---

### 1.3 Modèle `hr.leave.allocation` (Allocation de congés)

**Rôle :** Représente une **allocation** de jours (ou heures) d'un type de congé accordée à un employé — enveloppe disponible pour poser des congés.

**Champs clés :**

#### Identification
- `name` : Libellé (ex. "Congés payés 2025")
- `employee_id` : Many2one vers `hr.employee` (employé) ou vide pour allocation collective
- `employee_ids` : Many2many (si allocation en lot)
- `company_id` : Many2one vers `res.company`
- `department_id` : Optionnel (allocation par département)

#### Type et période
- `holiday_status_id` : Many2one vers `hr.leave.type` (type de congé)
- `date_from` / `date_to` : Période de validité de l'allocation
- `accrual` : Boolean — allocation issue d'un plan d'acquisition (accrual plan)

#### Quantités
- `number_of_days` : Nombre de jours alloués
- `number_of_hours` : Nombre d'heures (si type en heures)
- `number_of_days_display` : Affichage (peut différer selon unité)
- Solde consommé : dérivé des `hr.leave` validés du même type sur la période

#### Statut et validation
- **State** : draft, confirm, validate, validate1, refuse, cancel
- Même logique de validation que hr.leave (Time Off Officer, Employee's Approver)
- Demande d'allocation supplémentaire par l'employé : même workflow que demande de congé

**Règles métier :**
- Une allocation est liée à un type (`hr.leave.type`) et à un ou plusieurs employés
- Le solde disponible = allocation(s) validée(s) − congés validés du même type
- Allocations par lot : wizard "Generate Allocations" pour créer des lignes par employé
- Plans d'acquisition (accrual) : génération automatique d'allocations selon règles (voir hr.leave.accrual)

---

### 1.4 Modèle `hr.leave.accrual` (Plan d'acquisition / Accrual plan)

**Rôle :** Définit un **plan d'acquisition** — règles selon lesquelles les employés acquièrent des jours de congé au fil du temps (ex. X jours par mois travaillé).

**Champs clés :**

#### Identification
- `name` : Nom du plan
- `company_id` : Entreprise (optionnel, multi-société)
- **Accrued Gain Time** : At the start of the accrual period / At the end of the accrual period
- **Carry-Over Time** : At the start of the year / At the allocation date / Other (date personnalisée)
- **Based on worked time** : Boolean — l'acquisition ne compte que le temps travaillé (exclut absences, congés)

#### Règles (milestones)
- **Rules** : Une ou plusieurs règles (milestones)
  - **Employee accrue** : Days/Hours × montant × fréquence (Hourly, Daily, Weekly, Twice a month, Monthly, Twice a year, Yearly)
  - **Cap accrued time** : Plafond d'acquisition (optionnel)
  - **Start Accruing** : Délai avant de commencer (ex. X mois)
  - **Carry over** : None (reset 0) / All accrued time carried over / Carry over with a maximum (Up to X days/hours)
  - **Milestone cap** : Plafond total par année civile (optionnel)
  - **Carry Over Validity** : Durée de validité du report (ex. X mois)
- **Milestone Transition** : Immediately / After this accrual's period (si plusieurs règles)

**Règles métier :**
- Le report (Carry over) défini sur une règle prime sur celui du plan
- Si "Based on worked time" : les jours non travaillés (congés, absences) ne génèrent pas d'acquisition
- Les allocations générées par le plan sont du type `hr.leave.allocation` avec `accrual = True`
- Cron / batch pour calculer et créer les allocations selon la fréquence

---

### 1.5 Modèle `resource.calendar.leaves` / Jours fériés (Public holidays)

**Rôle :** Jours **non travaillés** (fériés, ponts) — utilisés par le calcul des congés et des plannings.

**Champs clés (conceptuels) :**
- `name` : Nom (ex. "1er janvier")
- `date_from` / `date_to` : Début et fin (datetime, timezone company)
- `company_id` : Entreprise
- `resource_id` : Vide pour jour férié global, ou ressource spécifique
- **Working Hours** : Optionnel — appliquer le férié seulement à un calendrier de travail donné
- **Work Entry Type** : Pour Payroll (type d'entrée travail)

**Règles métier :**
- Si "Public Holiday Included" sur un type de congé : une demande qui couvre un jour férié ne consomme pas de jour de congé pour ce jour
- Les jours fériés sont partagés avec Calendar, Planning, Manufacturing, etc.

---

### 1.6 Modèle `hr.leave.mandatory.day` (Jours obligatoires)

**Rôle :** Jours où la présence est **obligatoire** — les demandes de congé sont interdites pour ces jours (entreprise ou département).

**Champs clés :**
- `name` : Nom (ex. "Inventaire annuel")
- `company_id` : Entreprise
- `department_ids` : Many2many vers `hr.department` — si vide, s'applique à toute l'entreprise
- `start_date` / `end_date` : Période
- `color` : Couleur dans le calendrier Time Off

**Règles métier :**
- Un employé ne peut pas poser de congé sur une date couverte par un jour obligatoire (pour son département ou company)
- Contrôle côté validation et/ou côté formulaire de demande

---

## 2. Workflows et États

### 2.1 Workflow Demande de congé (`hr.leave`)

| État       | Description        | Transition possible                          |
|-----------|--------------------|-----------------------------------------------|
| draft     | Brouillon          | confirm, cancel                              |
| confirm   | En attente         | validate, validate1, refuse, cancel           |
| validate1 | Validé niveau 1    | validate, refuse (si double validation)      |
| validate  | Validé             | cancel (wizard)                               |
| refuse    | Refusé             | —                                             |
| cancel    | Annulé             | —                                             |

**Acteurs :**
- **No Validation** : passage automatique en validate
- **By Time Off Officer** : Time Off Officer valide/refuse
- **By Employee's Approver** : Approbateur (défini sur fiche employé) valide/refuse
- **By Employee's Approver and Time Off Officer** : les deux doivent valider (validate1 puis validate)

### 2.2 Workflow Allocation (`hr.leave.allocation`)

- Même logique d'états et de validation que `hr.leave`
- draft → confirm → validate1 (si double) → validate
- Refuse / Cancel possibles

---

## 3. Calculs Métier

### 3.1 Calcul du nombre de jours/heures d'une demande

- **Période** : `date_from` à `date_to`
- **Calendrier** : Calendrier de travail de l'employé (`resource.calendar`)
- **Exclusions** : Jours fériés (si "Public Holiday Included"), jours non travaillés
- **Unité** : Day (8h), Half Day (4h), ou Hours selon le type
- **Déduction heures sup** : Si "Deduct Extra Hours", les heures sup sont déduites de la demande avant de consommer le solde

### 3.2 Solde disponible

- **Solde = Somme(allocations validées pour le type et l'employé) − Somme(congés validés pour le type et l'employé)**
- Période : généralement sur l'année ou la période d'allocation
- **Extra days** : Si le type autorise "Extra Days Requests Allowed", l'employé peut demander au-delà du solde (soumis à validation)
- **Negative cap** : Si "Allow Negative Cap" avec "Maximum Excess Amount", le solde peut aller en négatif jusqu'au plafond

### 3.3 Accrual (acquisition)

- Selon règles du plan : fréquence (mensuelle, hebdo, etc.), montant (jours/heures), plafonds, report
- "Based on worked time" : seuls les jours/heures travaillés comptent pour l'acquisition
- Kind of Time Off "Worked Time" : le congé pris compte pour l'acquisition ; "Absence" ne compte pas

---

## 4. Intégrations Métier (résumé)

- **HR (hr)** : Employés, départements, approbateur congés (Work Information)
- **Resource (resource)** : Calendrier de travail, jours fériés (`resource.calendar.leaves`)
- **Calendar (calendar)** : Création d’événement / réunion quand un congé est validé (si type configuré avec meeting type)
- **Payroll (hr_payroll)** : Work entry type sur type de congé → génération des work entries
- **Timesheet (hr_timesheet)** : Projet/tâche sur type de congé → lignes timesheet pour les jours de congé
- **Mail** : Chatter, activités, notifications sur demandes et allocations

---

## 5. Rapports et Statistiques

- **Time Off Summary** : Synthèse des congés (par employé, type, période)
- **Time Off by Department** : Congés par département
- **Time Off Analysis** : Analyse détaillée (pivot/graph)
- **Overview** : Calendrier coloré (mes congés / équipe), barre du nombre de personnes absentes par jour
- **My Time Off** : Mes demandes et soldes
- **Management** : Demandes à approuver, allocations
- **Allocations** : Liste des allocations (officiers / admin)

---

## 6. Droits d'accès (résumé)

- **Tous** : My Time Off, Overview (selon périmètre)
- **Time Off Officer / Administrator** : Configuration (types, plans, fériés, jours obligatoires), Management (validation), Allocations, Reporting
- **Employee's Approver** : Validation des demandes qui lui sont déléguées (selon type et fiche employé)

---

## Références

- Documentation Odoo 19.0 — Time Off
- Odoo GitHub — `addons/hr_holidays`
- Analyses Miyukini : Project, Employees (HR), Timesheet
