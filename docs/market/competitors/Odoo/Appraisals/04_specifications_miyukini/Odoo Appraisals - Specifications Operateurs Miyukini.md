# Odoo Appraisals — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Appraisals** (Évaluations) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Appraisals
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores et avec EmployeeService (MiyuHR)

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **AppraisalOperator** | Gestion du cycle d'évaluation (création, confirmation, feedback, clôture, rating) | Opérateur de Service |
| **AppraisalPlanOperator** | Planification automatique (plans 6m/6m/12m, automatisation) | Opérateur de Domaine |
| **AppraisalTemplateOperator** | Modèles d'évaluation (sections Employee/Manager Feedback) | Opérateur de Domaine |
| **AppraisalGoalsOperator** | Objectifs employé (avancement, deadline, tags, Mark as Done) | Opérateur de Domaine |
| **Appraisal360Operator** | 360 Feedback (demande, envoi, résultats, dashboard) | Opérateur de Service |
| **AppraisalUI** | Interface utilisateur Appraisals (dashboard, formulaire, goals, config) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : AppraisalService

**Définition :**
> **AppraisalService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service d'évaluation des performances (planification, auto-évaluation, feedback manager, 360, objectifs, clôture).**

**Composition :**
- AppraisalOperator (niveau sécurité 2–3 selon données)
- AppraisalPlanOperator (niveau sécurité 2)
- AppraisalTemplateOperator (niveau sécurité 1–2)
- AppraisalGoalsOperator (niveau sécurité 2)
- Appraisal360Operator (niveau sécurité 2)
- AppraisalUI (niveau sécurité 1)

**Contrat d'équipe :**
- Consomme : EmployeeOperator / EmployeeSkillsOperator (fiche employé, hiérarchie, compétences, next_appraisal_date)
- Consomme : MiyuNotify (notifications confirmation, Ask Feedback, réunion)
- Consomme : MiyuSurveys ou équivalent (templates, 360) si externalisé
- Consomme : MiyuPlanning / Calendar (événement Meeting)
- Expose : cycle appraisal (create, confirm, feedback, review, rate, done), goals (CRUD), 360 (request, results), plans et templates (config)

---

## 2. Opérateurs Détaillés

### 2.1 AppraisalOperator

**Rôle :** Gestion du cycle d'évaluation (création, confirmation, auto-évaluation, feedback manager, réunion, note finale, note privée, clôture).

**Capacités :**
- Création / modification appraisal (employé, date, template, statut)
- Confirm (passage draft → confirmed ; notification employé)
- Gestion visibilité (Employee's Feedback visible to manager ; Manager's Feedback visible to employee)
- Final Rating (échelle configurable)
- Private Note (visible managers uniquement)
- Mark as Done / Reopen
- Lecture / mise à jour Skills dans le cadre de l'appraisal (délégation à EmployeeSkillsOperator avec Mandat)
- Mise à jour next_appraisal_date sur fiche employé (via contrat EmployeeOperator / KindMother)

**Niveau de sécurité :** 2 (Sensitive) pour données générales ; 3 (Critical) pour Private Note et rating.

**Gouvernance :**
- **StrongFather** : Décisions (création, confirmation, clôture, réouverture)
- **KindMother** : Persistance appraisal, rating, notes (WriteIntent)
- **Master Butler** : Permissions (Manager sur ses subordonnés, Employé sur sa propre appraisal, HR/Admin config)
- **WorrySentinel** : Niveau sécurité (Private Note, rating, données sensibles)
- **TAMR** : Point d'intervention humaine (réunion, décision finale rating / note privée)
- **Ever Buddy** : Cycle de vie (draft → confirmed → done ; réouverture)

**Contrat d'équipe :**
- Consomme : EmployeeOperator (employé, manager, département ; next_appraisal_date), EmployeeSkillsOperator (skills dans appraisal)
- Consomme : AppraisalTemplateOperator (template), AppraisalPlanOperator (plan pour next_appraisal_date)
- Expose : `appraisal.create`, `appraisal.confirm`, `appraisal.update_feedback`, `appraisal.rate`, `appraisal.private_note`, `appraisal.done`, `appraisal.reopen`

**Mandat de Permission requis :**
- Création / confirmation : Mandat avec KindMother (WriteIntent) + Master Butler (appraisal.create / appraisal.confirm)
- Feedback / rating / done : Mandat avec Master Butler (appraisal.update pour manager) et niveau sécurité 2–3 pour Private Note
- Lecture employé : Mandat limité à ses propres appraisals (appraisal.read_own)

### 2.2 AppraisalPlanOperator

**Rôle :** Planification automatique des évaluations (plans 6m, 6m, 12m ; automatisation).

**Capacités :**
- Lecture / modification Appraisals Plan (mois pour première, deuxième, puis annuelles)
- Activation / désactivation Appraisals Automation (création et confirmation automatiques)
- Mise à jour en masse next_appraisal_date (tous les employés avec date vide) lors de modification du plan

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision modification plan
- **KindMother** : Persistance plan (WriteIntent)
- **Master Butler** : Permissions (HR/Admin uniquement)
- **WorrySentinel** : Isolation multi-société

**Contrat d'équipe :**
- Consomme : EmployeeOperator (next_appraisal_date en écriture pour mise à jour masse)
- Expose : `plan.get`, `plan.update`, `plan.run_automation`

### 2.3 AppraisalTemplateOperator

**Rôle :** Modèles d'évaluation (sections et questions Employee's Feedback et Manager's Feedback).

**Capacités :**
- CRUD templates (Default Template, templates par département)
- Structure : sections My work, My future, My feelings (employé) ; Feedback, Evaluation, Improvements (manager)
- Liste des templates disponibles pour le sélecteur Appraisals

**Niveau de sécurité :** 1–2 (Standard à Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification template
- **KindMother** : Persistance templates (WriteIntent)
- **Master Butler** : Permissions (HR/Admin, éventuellement Manager lecture)
- **Ever Buddy** : Versions, dépréciation templates

**Contrat d'équipe :**
- Peut s’appuyer sur MiyuSurveys si templates = surveys ; sinon modèles natifs dans AppraisalService
- Expose : `template.list`, `template.get`, `template.create`, `template.update`

### 2.4 AppraisalGoalsOperator

**Rôle :** Objectifs assignés aux employés (avancement, deadline, tags, description, checklist).

**Capacités :**
- CRUD goals (Goal, Employee, Manager, Progress 0–100 %, Deadline, Tags, Description)
- Mark as Done (100 %, statut terminé)
- Mise à jour Progress à tout moment par le manager (pas seulement pendant l'appraisal)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification goal
- **KindMother** : Persistance goals (WriteIntent)
- **Master Butler** : Permissions (Manager sur ses subordonnés, lecture employé sur ses objectifs si politique)
- **WorrySentinel** : Isolation multi-société

**Contrat d'équipe :**
- Consomme : EmployeeOperator (employé, manager)
- Expose : `goal.create`, `goal.update`, `goal.done`, `goal.list_by_employee`

### 2.5 Appraisal360Operator

**Rôle :** 360 Feedback (demande de retours à des collègues, envoi email, agrégation résultats, dashboard).

**Capacités :**
- Demande feedback (Ask Feedback) : sélection destinataires (employés), message, Answer Deadline ; envoi via MiyuNotify (template email)
- Agrégation résultats (Registered, Completed, Certified, Average Duration)
- Dashboard 360 (liste surveys, Test, See Results, export PDF)
- Création surveys 360 (ou délégation à MiyuSurveys)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision envoi demande 360
- **KindMother** : Persistance réponses (si stockage local) ou lecture depuis MiyuSurveys
- **Master Butler** : Permissions (Manager peut demander 360 pour ses subordonnés)
- **WorrySentinel** : Anonymisation ou confidentialité des réponses selon politique

**Contrat d'équipe :**
- Consomme : MiyuNotify (email Ask Feedback), MiyuSurveys ou stockage interne
- Expose : `360.request`, `360.results`, `360.dashboard`

### 2.6 AppraisalUI

**Rôle :** Interface utilisateur (dashboard appraisals, formulaire appraisal, goals, configuration).

**Capacités :**
- Dashboard : cartes/liste appraisals, indicateur « visible to manager », icône activité (Meeting)
- Formulaire appraisal : onglets Appraisal (feedback employé/manager, toggles visibilité), Skills, Private Note ; boutons Confirm, Ask Feedback, Mark as Done, Reopen ; Final Rating
- Goals : liste groupée par employé, formulaire goal, Mark as Done
- Configuration : Settings (plan, automation), Templates, Evaluation Scale, 360 Feedback, Tags
- Analyse : Appraisal analysis (filtres statut), Skills evolution

**Niveau de sécurité :** 1 (Standard) pour affichage ; respect des permissions backend (Manager voit ses équipes, Employé voit ses appraisals).

**Gouvernance :**
- **Master Butler** : Permissions (affichage conditionné par droits)
- **BondingBrother** : Médiation entre utilisateur et AppraisalOperator / Goals / Plan / Template / 360

**Contrat d'équipe :**
- Consomme : Tous les Opérateurs AppraisalService ; MiyuPlanning pour réunion
- Expose : écrans et actions UI selon rôles

---

## 3. Contrat d'Équipe AppraisalService

**Flux autorisés :**
- AppraisalUI → BondingBrother → AppraisalOperator, AppraisalPlanOperator, AppraisalTemplateOperator, AppraisalGoalsOperator, Appraisal360Operator
- AppraisalOperator → EmployeeOperator (next_appraisal_date), EmployeeSkillsOperator (skills dans appraisal)
- AppraisalPlanOperator → EmployeeOperator (next_appraisal_date masse)
- Appraisal360Operator → MiyuNotify, MiyuSurveys (ou interne)

**Types de données échangeables :**
- Appraisal (employé, manager, date, template, statut, feedback, rating, note privée)
- Goal (employé, manager, progression, deadline, tags)
- Plan (mois, automation)
- Template (sections, questions)
- 360 (demande, résultats, stats)

**Conditions préalables :**
- EmployeeService (MiyuHR) déployé pour employés, hiérarchie, compétences
- MiyuNotify pour notifications
- Optionnel : MiyuSurveys pour templates/360 ; MiyuPlanning pour réunions

**Niveau de validation requis :** StrongFather pour création/confirmation/clôture ; Master Butler pour chaque action ; WorrySentinel pour Private Note et rating.

---

## 4. Correspondance Miyukini

**Service Miyukini proposé :** **MiyuAppraisals** (ou **MiyukiniAppraisals**) — AppraisalService

**Équipe d'Opérateurs :** AppraisalService (6 Opérateurs)

**Niveaux de sécurité :** 1–3 selon données (Standard à Critical pour Private Note et rating)

**Intégration Cores :**
- StrongFather : Décisions (création, confirmation, clôture, plan, 360)
- KindMother : Persistance (WriteIntent appraisal, goals, plan, templates)
- Master Butler : Permissions (Manager, Employé, HR/Admin)
- WorrySentinel : Sécurité (Private Note, rating, données sensibles)
- TAMR : Intervention humaine (réunion, décision finale)
- Ever Buddy : Cycle de vie (statuts appraisal, versions templates)

---

**Document** : Odoo Appraisals — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
