# Odoo Time Off — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Time Off** (Congés / Absences) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l'équivalent Time Off
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **LeaveRequestOperator** | Gestion des demandes de congé | Opérateur de Service |
| **LeaveAllocationOperator** | Gestion des allocations de congés | Opérateur de Service |
| **LeaveTypeOperator** | Gestion des types de congé | Opérateur de Service |
| **LeaveAccrualOperator** | Gestion des plans d'acquisition | Opérateur de Service |
| **LeaveCalendarOperator** | Jours fériés et jours obligatoires | Opérateur de Service |
| **TimeOffUI** | Interface utilisateur Time Off | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : TimeOffService

**Définition :**
> **TimeOffService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des congés et absences.**

**Composition :**
- LeaveRequestOperator (niveau sécurité 2)
- LeaveAllocationOperator (niveau sécurité 2)
- LeaveTypeOperator (niveau sécurité 2)
- LeaveAccrualOperator (niveau sécurité 2)
- LeaveCalendarOperator (niveau sécurité 2)
- TimeOffUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 LeaveRequestOperator

**Rôle :** Gestion des demandes de congé (création, validation, refus, annulation).

**Capacités :**
- Création/modification de demandes de congé (employé ou délégué)
- Calcul du nombre de jours/heures selon calendrier et type (Day / Half Day / Hours)
- Vérification du solde disponible (allocation − congés validés)
- Workflow : draft → confirm → validate1 (si double) → validate ; refuse ; cancel
- Délégation de validation : Employee's Approver, Time Off Officer (selon type)
- Annulation (wizard) et recrédit du solde
- Déclenchement des intégrations à la validation (Calendar, Payroll, Timesheet)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de validation/refus
- **KindMother** : Persistance des demandes (WriteIntent)
- **Master Butler** : Permissions (poser une demande, valider, refuser)
- **WorrySentinel** : Niveau sécurité, isolation des données par entreprise/département
- **Ever Buddy** : Cycle de vie des demandes (états, transitions)

**Contrat d'équipe :**
- Consomme : LeaveTypeOperator (type), LeaveAllocationOperator (solde), MiyuHR (employé, approbateur), MiyuClock/Resource (calendrier, calcul jours), MiyuNotify (notifications)
- Expose : `leave_request.create`, `leave_request.submit`, `leave_request.approve`, `leave_request.refuse`, `leave_request.cancel`

**Mandat de Permission requis :**
- Création demande : Mandat avec KindMother (WriteIntent) + Master Butler (poser une demande)
- Validation : Mandat avec StrongFather (décision) + Master Butler (valider/refuser)
- Annulation : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.2 LeaveAllocationOperator

**Rôle :** Gestion des allocations de congés (création, validation, solde).

**Capacités :**
- Création/modification d'allocations (individuelles ou en lot)
- Calcul du solde disponible par employé et type (allocations validées − congés validés)
- Workflow : draft → confirm → validate1 → validate ; refuse ; cancel
- Génération en lot (wizard) : employés, type, montant, période
- Intégration avec LeaveAccrualOperator (allocations générées par plan)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/validation d'allocation
- **KindMother** : Persistance des allocations (WriteIntent)
- **Master Butler** : Permissions (créer allocation, valider)
- **WorrySentinel** : Niveau sécurité
- **Ever Buddy** : Cycle de vie des allocations

**Contrat d'équipe :**
- Consommé par : LeaveRequestOperator (solde)
- Consomme : LeaveTypeOperator (type), MiyuHR (employés, départements)
- Expose : `allocation.create`, `allocation.validate`, `allocation.batch_create`, `allocation.balance`

**Mandat de Permission requis :**
- Création allocation : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Validation : Mandat avec StrongFather (décision) + Master Butler (Time Off Officer)
- Batch create : Mandat avec LeaveAllocationOperator + KindMother (WriteIntent)

### 2.3 LeaveTypeOperator

**Rôle :** Gestion des types de congé (configuration, règles de validation et d'allocation).

**Capacités :**
- Création/modification des types (nom, unité Day/Half Day/Hours)
- Configuration validation : No Validation, By Time Off Officer, By Employee's Approver, Both
- Configuration allocation : Requires allocation (Yes/No Limit), Extra days allowed, Approval
- Options : Deduct Extra Hours, Public Holiday Included, Allow attachment, Kind (Worked Time / Absence), Negative cap (Allow + Max excess)
- Lien Payroll (Work Entry Type), Timesheet (Project/Task) si modules présents
- Affichage : Color, Cover Image

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification type
- **KindMother** : Persistance des types (WriteIntent)
- **Master Butler** : Permissions (configurer types)
- **WorrySentinel** : Niveau sécurité

**Contrat d'équipe :**
- Consommé par : LeaveRequestOperator, LeaveAllocationOperator
- Consomme : MiyuHR (Time Off Officer), MiyuPayroll (Work Entry Type) si présent
- Expose : `leave_type.create`, `leave_type.update`, `leave_type.config`

**Mandat de Permission requis :**
- Création/modification type : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.4 LeaveAccrualOperator

**Rôle :** Gestion des plans d'acquisition (accrual) — règles d'acquisition de jours/heures au fil du temps.

**Capacités :**
- Création/modification des plans (nom, Accrued Gain Time, Carry-Over Time, Based on worked time, Milestone Transition)
- Règles (milestones) : Employee accrue (Days/Hours × montant × fréquence), Cap, Start Accruing, Carry over, Milestone cap, Carry Over Validity
- Génération périodique d'allocations selon règles (cron/batch)
- Traçabilité des allocations générées

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification plan
- **KindMother** : Persistance des plans et des allocations générées (WriteIntent)
- **Master Butler** : Permissions (configurer plans)
- **WorrySentinel** : Niveau sécurité
- **Ever Buddy** : Cycle de vie des règles (report, validité)

**Contrat d'équipe :**
- Consommé par : LeaveAllocationOperator (allocations générées)
- Consomme : LeaveTypeOperator (type), MiyuHR (employés, temps travaillé si "Based on worked time"), MiyuClock (dates)
- Expose : `accrual_plan.create`, `accrual_plan.run`, `accrual_plan.allocate`

**Mandat de Permission requis :**
- Création plan : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Exécution batch : Mandat avec LeaveAccrualOperator + LeaveAllocationOperator + KindMother (WriteIntent)

### 2.5 LeaveCalendarOperator

**Rôle :** Jours fériés et jours obligatoires (partagés avec Resource, Planning, Manufacturing).

**Capacités :**
- Création/modification des jours fériés (nom, dates, company, Working Hours, Work Entry Type)
- Création/modification des jours obligatoires (nom, company, départements, dates, couleur)
- Fourniture de la liste des jours non travaillés / obligatoires pour le calcul des congés et le blocage des demandes
- Partage avec Resource/Calendar pour calendrier de travail et plannings

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions
- **WorrySentinel** : Niveau sécurité

**Contrat d'équipe :**
- Consommé par : LeaveRequestOperator (calcul jours, Public Holiday Included, blocage jours obligatoires), Resource/Planning/Manufacturing
- Consomme : MiyuHR (départements), MiyuClock (dates)
- Expose : `public_holiday.create`, `mandatory_day.create`, `calendar.leaves_list`

**Mandat de Permission requis :**
- Création férié/jour obligatoire : Mandat avec KindMother (WriteIntent) + StrongFather (décision)

### 2.6 TimeOffUI

**Rôle :** Interface utilisateur Time Off (Mes congés, Overview, Management, Configuration, Reporting).

**Capacités :**
- Vue "Mes congés" : soldes par type, liste des demandes, lien calendrier
- Calendrier Overview : congés (moi/équipe/entreprise), barres validé/à approuver, Total (nombre de personnes absentes par jour)
- Formulaires : demande de congé, détail demande/allocation, types, plans, fériés, jours obligatoires
- Management : liste des demandes à approuver, actions Approve/Refuse/Cancel
- Allocations : liste, création, wizard batch
- Configuration : types, accrual plans, fériés, jours obligatoires
- Rapports : Time Off Summary, By Department, Analysis (filtres, export)
- Notifications (via MiyuNotify) : soumission, validation, refus, annulation

**Niveau de sécurité :** 1 (Standard) pour lecture ; 2 pour actions (poser, valider, configurer)

**Gouvernance :**
- **BondingBrother** : Traduction des intentions utilisateur vers les Opérateurs
- **Master Butler** : Permissions d'affichage et d'action selon rôle
- **WorrySentinel** : Niveau sécurité par écran (Mes congés vs Management vs Configuration)

**Contrat d'équipe :**
- Consomme : LeaveRequestOperator, LeaveAllocationOperator, LeaveTypeOperator, LeaveAccrualOperator, LeaveCalendarOperator, MiyuHR, MiyuNotify
- Expose : écrans et actions UI (pas d'exposition directe de capacités métier)

**Mandat de Permission requis :**
- Affichage Mes congés / Overview : Mandat lecture TimeOffService (selon périmètre moi/équipe/tous)
- Validation / Configuration : Mandat avec StrongFather + Master Butler (rôle approbateur/officier)

---

## 3. Contrat d'Équipe TimeOffService

### 3.1 Règles statiques

- **Opérateurs membres** : LeaveRequestOperator, LeaveAllocationOperator, LeaveTypeOperator, LeaveAccrualOperator, LeaveCalendarOperator, TimeOffUI
- **Flux autorisés** :
  - TimeOffUI → LeaveRequestOperator, LeaveAllocationOperator, LeaveTypeOperator, LeaveAccrualOperator, LeaveCalendarOperator (intentions)
  - LeaveRequestOperator → LeaveAllocationOperator (solde), LeaveTypeOperator (type), MiyuHR (employé, approbateur), MiyuNotify (notifications)
  - LeaveAllocationOperator → LeaveTypeOperator (type), MiyuHR (employés)
  - LeaveAccrualOperator → LeaveAllocationOperator (création allocations), LeaveTypeOperator (type)
  - LeaveCalendarOperator → consommé par LeaveRequestOperator, Resource/Planning
- **Direction des flux** : UI vers Opérateurs ; Opérateurs vers HR, Notify, Payroll, Timesheet, Calendar selon contrats
- **Types d'échanges** : WriteIntent (KindMother), décision (StrongFather), permission (Master Butler), notification (MiyuNotify), lecture employé/calendrier (MiyuHR, Resource)
- **Conditions préalables** : Mandat de Permission valide pour chaque action (poser, valider, configurer)
- **Niveau de validation** : StrongFather pour toute décision de validation/refus/création allocation ; KindMother pour toute persistance

### 3.2 Sécurité hétérogène

- **TimeOffUI** : niveau 1 pour lecture "mes congés" ; niveau 2 pour actions (poser, valider, configurer)
- **LeaveRequestOperator, LeaveAllocationOperator, LeaveTypeOperator, LeaveAccrualOperator, LeaveCalendarOperator** : niveau 2
- **Ponts** : TimeOffUI ne peut pas élever son niveau ; les flux de validation passent par StrongFather et Master Butler (niveau 2)
- **WorrySentinel** : vérification niveau sécurité et isolation entreprise/département

---

## 4. Intégration avec les Cores

- **StrongFather** : Toute décision de validation/refus de demande ou d'allocation ; toute décision de création/modification type, plan, férié, jour obligatoire
- **KindMother** : Toute persistance (demande, allocation, type, plan, férié, jour obligatoire) via WriteIntent
- **Master Butler** : Permissions "poser une demande", "valider/refuser", "créer allocation", "configurer types/plans/fériés/jours obligatoires" ; rôles Employee, Approver, Time Off Officer
- **WorrySentinel** : Niveau de sécurité 2 sur données congés ; pas d'élévation de niveau ; audit des validations/refus
- **Ever Buddy** : Cycle de vie des demandes et allocations (états, transitions, dépréciation types)
- **BondingBrother** : Traduction des intentions UI vers les Opérateurs ; pas d'autorité, uniquement médiation
- **Caring Nanny** : Observation de l'état (demandes en attente, soldes) ; pas de modification
- **TAMR** : Points d'intervention humaine : validation/refus par l'approbateur ou l'officier (pas d'auto-validation sauf "No Validation")

---

## 5. Correspondance Miyukini

**Nom proposé :** `MiyukiniTimeOff` ou `MiyuTimeOff`

**Équipe :** TimeOffService

**Opérateurs :**
- LeaveRequestOperator (demandes de congé)
- LeaveAllocationOperator (allocations)
- LeaveTypeOperator (types de congé)
- LeaveAccrualOperator (plans d'acquisition)
- LeaveCalendarOperator (fériés, jours obligatoires)
- TimeOffUI (interface)

**Contrats externes :**
- MiyuHR (employés, départements, approbateur)
- MiyuNotify (notifications)
- MiyuClock / Resource (calendrier, calcul jours)
- MiyuPayroll (work entries si présent)
- MiyuTimesheet / MiyuInvoice (lignes analytic si présent)
- Calendar/Agenda (événements à la validation si configuré)

---

## Références

- Glossaire Miyukini (Opérateur, Équipe d'Opérateurs, Mandat de Permission, Contrat d'Équipe)
- Odoo Time Off — Logique Métier, Parcours Utilisateur, Intégrations Cross-App
- Odoo Project — Spécifications Opérateurs Miyukini (structure de document)
