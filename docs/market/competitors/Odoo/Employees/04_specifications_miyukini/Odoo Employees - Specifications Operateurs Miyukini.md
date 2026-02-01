# Odoo Employees — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Employees** (Employés) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Employees
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **EmployeeOperator** | Gestion des fiches employés (création, édition, hiérarchie, contact) | Opérateur de Service |
| **DepartmentOperator** | Gestion des départements (structure, manager, parent) | Opérateur de Domaine |
| **EmployeeSkillsOperator** | Compétences, certifications, résumé (Skill Types, niveaux) | Opérateur de Domaine |
| **EmployeePresenceOperator** | Présence (pointage, statut utilisateur, contrôle avancé) | Opérateur de Service |
| **EmployeeEquipmentOperator** | Équipements attribués aux employés | Opérateur de Domaine |
| **EmployeeOffboardingOperator** | Processus offboarding (sortie, récupération équipements) | Opérateur de Service |
| **EmployeeUI** | Interface utilisateur Employees | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : EmployeeService (HR Base)

**Définition :**
> **EmployeeService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des employés (fiches, départements, compétences, présence, équipements, offboarding).**

**Composition :**
- EmployeeOperator (niveau sécurité 2–3 selon données)
- DepartmentOperator (niveau sécurité 1–2)
- EmployeeSkillsOperator (niveau sécurité 2)
- EmployeePresenceOperator (niveau sécurité 1–2)
- EmployeeEquipmentOperator (niveau sécurité 2)
- EmployeeOffboardingOperator (niveau sécurité 3)
- EmployeeUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 EmployeeOperator

**Rôle :** Gestion des fiches employés (création, édition, identité, hiérarchie, contact travail, informations travail, privées, paie, paramètres).

**Capacités :**
- Création / modification de fiches employé (général, work information, private, payroll, settings)
- Gestion hiérarchie (parent_id, coach_id, department_id, manager via département)
- Lien utilisateur (user_id) optionnel ; création utilisateur depuis fiche
- Approbateurs par domaine (expense, time_off, timesheet, attendance)
- Horaires (resource_calendar_id), fuseau, télétravail (lieu par jour)
- Données privées (adresse, banque, urgence, citoyenneté, permis de travail)
- Paramètres (type employé, PIN, Badge ID, Hourly Cost, Fleet Mobility Card)

**Niveau de sécurité :** 2 (Sensitive) pour données générales ; 3 (Critical) pour données privées et paie.

**Gouvernance :**
- **StrongFather** : Décisions (création, modification, désactivation) ; validation hiérarchie et approbateurs
- **KindMother** : Persistance des fiches (WriteIntent)
- **Master Butler** : Permissions (Officer, Manager, Employé sur sa fiche si Employee Editing)
- **WorrySentinel** : Niveau sécurité données (privées, paie) ; isolation multi-société
- **Ever Buddy** : Cycle de vie (actif / inactif, offboarding)

**Contrat d'équipe :**
- Consomme : DepartmentOperator (départements), MiyuContacts (work_contact, partenaires), MiyuNotify (notifications si chatter)
- Expose : `employee.create`, `employee.update`, `employee.deactivate`, `employee.get_approvers`, `employee.get_hierarchy`

**Mandat de Permission requis :**
- Création fiche : Mandat avec KindMother (WriteIntent) + Master Butler (employee.create)
- Modification fiche : Mandat avec KindMother (WriteIntent) + Master Butler (employee.update ou self_edit si Employee Editing)
- Données privées / paie : Mandat avec niveau sécurité 3 (WorrySentinel)

### 2.2 DepartmentOperator

**Rôle :** Gestion des départements (structure, manager, parent, société).

**Capacités :**
- Création / modification / suppression de départements
- Hiérarchie (parent_id)
- Manager et coach par défaut (propagation sur fiche employé à la sélection)

**Niveau de sécurité :** 1–2 (Standard à Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions (Officer, Admin)
- **WorrySentinel** : Isolation multi-société

**Contrat d'équipe :**
- Consommé par : EmployeeOperator (department_id)
- Expose : `department.create`, `department.update`, `department.list`, `department.get_manager`

### 2.3 EmployeeSkillsOperator

**Rôle :** Gestion des compétences, types de compétences, niveaux, résumé (expériences, certifications).

**Capacités :**
- Skill Types (catégories, compétences, niveaux avec pourcentage, défaut, couleur)
- Attribution compétences aux employés (skill type, skill, level)
- Lignes de résumé (expérience, éducation, certifications)
- Droits : seul Officer/Admin peut ajouter/éditer les compétences

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision modification référentiel
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions (Officer: Manage all employees ou Administrator)
- **Ever Buddy** : Évolution des Skill Types (versions, dépréciation)

**Contrat d'équipe :**
- Consommé par : EmployeeOperator, EmployeeUI
- Expose : `skill_type.create`, `skill_type.update`, `employee_skill.add`, `employee_skill.update`, `resume_line.add`

### 2.4 EmployeePresenceOperator

**Rôle :** Calcul et exposition du statut de présence (pointage, connexion utilisateur, contrôle avancé).

**Capacités :**
- Mode « Based on attendances » : lecture des pointages (MiyuAttendances ou équivalent)
- Mode « Based on user status » : statut de session utilisateur
- Mode « Advanced » : signaux (e-mails envoyés, IP) selon configuration
- Exposition statut (présent / absent / hors horaire) pour affichage (Kanban, indicateurs)

**Niveau de sécurité :** 1–2 (données de présence non sensibles sauf si géolocalisation / IP)

**Gouvernance :**
- **Caring Nanny** : Observation d’état (présence) ; pas de décision métier
- **Master Butler** : Qui peut voir les présences (par département, équipe)
- **WorrySentinel** : Niveau sécurité si données IP / géolocalisation

**Contrat d'équipe :**
- Consomme : MiyuAttendances (pointages), Session/Kernel (statut utilisateur), MiyuNotify ou signaux (e-mails), config IP
- Expose : `presence.get_status`, `presence.get_bulk_status`

### 2.5 EmployeeEquipmentOperator

**Rôle :** Gestion des équipements attribués aux employés (suivi, récupération).

**Capacités :**
- Attribution équipement → employé
- Liste des équipements par employé
- Récupération lors de l’offboarding

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision attribution / récupération
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions (Officer, Admin)
- **Ever Buddy** : Cycle de vie (attribué / récupéré)

**Contrat d'équipe :**
- Consommé par : EmployeeOperator, EmployeeOffboardingOperator
- Expose : `equipment.assign`, `equipment.recover`, `equipment.list_by_employee`

### 2.6 EmployeeOffboardingOperator

**Rôle :** Processus de sortie (désactivation, récupération équipements, archivage, révocation accès).

**Capacités :**
- Workflow offboarding (étapes, checklist)
- Désactivation fiche employé (active = false)
- Déclenchement récupération équipements (EmployeeEquipmentOperator)
- Révocation mandats / accès (StrongFather, Master Butler)
- Archivage / traçabilité (Ever Buddy)

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision lancement offboarding, révocation mandats
- **KindMother** : Persistance (état offboarding, désactivation)
- **Master Butler** : Permissions (Officer, Admin uniquement)
- **WorrySentinel** : Audit, isolation données
- **TAMR** : Point d’intervention humaine (validation sortie)

**Contrat d'équipe :**
- Consomme : EmployeeOperator, EmployeeEquipmentOperator, MiyuNotify
- Expose : `offboarding.start`, `offboarding.complete`, `offboarding.cancel`

### 2.7 EmployeeUI

**Rôle :** Interface utilisateur (liste, Kanban, formulaire employé, départements, configuration, rapports).

**Capacités :**
- Dashboard / liste / Kanban employés avec indicateurs présence si activé
- Formulaire employé (onglets Général, Résumé, Work Information, Private, Payroll, Settings)
- Gestion départements (liste, formulaire)
- Configuration (Presence Display, Skills Management, Remote Work, Employee Editing, Company Working Hours)
- Skill Types, Work Locations
- Rapports (retention)
- Création utilisateur depuis fiche employé

**Niveau de sécurité :** 1 (Standard) pour affichage ; écriture selon Mandats

**Gouvernance :**
- **BondingBrother** : Médiation entre utilisateur et Opérateurs (EmployeeOperator, DepartmentOperator, etc.)
- **Master Butler** : Permissions d’affichage et d’édition par écran

**Contrat d'équipe :**
- Consomme : Tous les Opérateurs de l’équipe EmployeeService
- Expose : écrans et actions UI (pas d’API métier directe)

---

## 3. Contrat d'Équipe EmployeeService

**Flux autorisés :**
- EmployeeUI → EmployeeOperator (CRUD employé)
- EmployeeUI → DepartmentOperator (CRUD département)
- EmployeeUI → EmployeeSkillsOperator (compétences, résumé)
- EmployeeUI → EmployeePresenceOperator (lecture présence)
- EmployeeUI → EmployeeEquipmentOperator (équipements)
- EmployeeUI → EmployeeOffboardingOperator (lancement offboarding)
- EmployeeOffboardingOperator → EmployeeOperator (désactivation), EmployeeEquipmentOperator (récupération)
- EmployeeOperator → DepartmentOperator (lecture département, manager)
- EmployeeOperator → MiyuContacts (work_contact)

**Types de données échangeables :** Identifiants employé, département, structure hiérarchie, statut présence, compétences, équipements, état offboarding.

**Conditions préalables :** Mandat de Permission valide émis par StrongFather pour l’équipe et le niveau de sécurité requis.

**Niveau de validation :** StrongFather valide le Contrat d’équipe une fois ; les Mandats encadrent chaque session / action.

---

## 4. Mandats de Permission

**Mandat Standard (lecture / liste) :**
- Opérateurs : EmployeeUI, EmployeePresenceOperator (lecture)
- Flux : employee.list, department.list, presence.get_status
- Niveau sécurité max : 1–2 selon périmètre (Manager = son équipe, Officer = tous)

**Mandat Édition (création / modification fiches) :**
- Opérateurs : EmployeeUI, EmployeeOperator, DepartmentOperator, EmployeeSkillsOperator
- Flux : employee.create, employee.update, department.create, skill_type.update, employee_skill.add
- Niveau sécurité max : 2 (données générales) ou 3 (données privées / paie)
- Conditions : Officer ou Manager (périmètre) ou Employee Editing (sa fiche uniquement)

**Mandat Offboarding :**
- Opérateurs : EmployeeUI, EmployeeOffboardingOperator, EmployeeOperator, EmployeeEquipmentOperator
- Flux : offboarding.start, offboarding.complete
- Niveau sécurité max : 3
- Conditions : Officer ou Administrator uniquement

---

## 5. Correspondance Miyukini

**Service Miyukini proposé :** **MiyuHR** (ou **MiyukiniHR**) — EmployeeService

**Crates existantes / à créer :**
- `miyuhr` : EmployeeOperator, DepartmentOperator (ou extension miyucontacts)
- `miyuhr_skills` : EmployeeSkillsOperator (ou intégré miyuhr)
- `miyuhr_presence` : EmployeePresenceOperator (ou miyuattendances)
- `miyuhr_equipment` : EmployeeEquipmentOperator
- `miyuhr_offboarding` : EmployeeOffboardingOperator
- Interface : miyukini-central ou front dédié (EmployeeUI)

**Intégration Cores :**
- StrongFather : Décisions création/modification/désactivation, offboarding, révocation mandats
- KindMother : Persistance fiches, départements, compétences, équipements (WriteIntent)
- Master Butler : Permissions (Officer, Manager, Employee self-edit)
- WorrySentinel : Niveaux sécurité 2–3 (données privées, paie, offboarding)
- Caring Nanny : État présence (observation)
- Ever Buddy : Cycle de vie (actif/inactif, Skill Types, équipements)
- TAMR : Validation humaine offboarding

---

**Document** : Odoo Employees — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
