# Odoo Appraisals — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Appraisals** (Évaluations) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18.0/19.0 (Appraisals, Employees, Surveys, Mail, Calendar)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres modules Odoo (hr, mail, survey, calendar)
- Flux de données inter-apps (Employees, Surveys, Mail, Calendar)
- Mécanismes d'intégration (employé comme ressource centrale, templates Surveys)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Module Appraisals (hr_appraisal)

**Contenu :**
- Modèle appraisal (évaluation : employé, manager, date, template, statut, rating, notes privées)
- Appraisal plan (fréquence automatique : 6m, 6m, 12m)
- Goals (objectifs employé : avancement, deadline, tags)
- Evaluation scale (échelle de notation)
- Intégration skills (reprise depuis fiche employé ; mise à jour après clôture)

**Dépendances typiques :**
- **hr** (Employees) : hr.employee (employé évalué, manager, département, job position) ; champ next_appraisal_date sur la fiche employé
- **mail** : Chatter, activités, notifications (confirmation appraisal, Ask Feedback, réunion)
- **survey** : Templates d'appraisal (structure des questions Employee's Feedback et Manager's Feedback) ; surveys 360
- **calendar** (ou équivalent) : Activité Meeting pour planifier l'entretien d'évaluation

---

## 2. Flux de Données

### 2.1 Appraisals comme consommateur

```
hr.employee (fiche employé)
    ├── employee_id (évalué)
    ├── parent_id / department_id (manager, département)
    ├── job_id / job_title (poste)
    └── next_appraisal_date (affichée si Appraisals installé ; mise à jour par Appraisals)

survey.survey (templates)
    └── option « Appraisal » pour être sélectionnable dans Appraisals

hr.employee (skills)
    └── Compétences (Skill Types, niveaux) reprises dans l'onglet Skills de l'appraisal
```

**Flux entrants (autres apps → Appraisals) :**
- **Employees** : Données employé (nom, manager, département, poste) à la sélection ; compétences (onglet Résumé) reprises dans l'onglet Skills de l'appraisal une fois confirmée ; next_appraisal_date mise à jour par Appraisals (plan ou clôture).
- **Surveys** : Templates d'appraisal (structure des questions) ; surveys 360 (Ask Feedback).
- **Mail** : Template email « Appraisal: Ask Feedback » ; notifications confirmation appraisal, réunion.

### 2.2 Appraisals comme source

**Flux sortants (Appraisals → autres apps) :**
- **Employees** : Mise à jour de next_appraisal_date (plan automatique ou après Mark as Done) ; mise à jour des compétences (Skill Level, Justification) sur la fiche employé après clôture de l'appraisal.
- **Mail** : Envoi email confirmation appraisal (lien vers l'appraisal) ; envoi demande 360 (Ask Feedback) ; invitation réunion.
- **Calendar** : Création d’événement Meeting (activité depuis dashboard ou smart button Meetings / No Meeting) ; option Odoo meeting (vidéocall URL).

---

## 3. Intégrations Détaillées

### 3.1 Employees (hr)

**Données partagées :**
- **Employé évalué** : employee_id ; à la sélection, Manager, Job Position, Department sont remplis depuis la fiche employé.
- **Next Appraisal Date** : Champ sur la fiche employé (visible si Appraisals installé). Vide avant plan ; « Ongoing » si plan actif ; date de la prochaine évaluation après clôture selon plan.
- **Skills** : Les compétences de la fiche employé (onglet Résumé, Skill Types, niveaux) sont copiées dans l’onglet Skills de l’appraisal à la confirmation. Les modifications (niveau, justification) dans l’appraisal sont reportées sur la fiche employé après Mark as Done.

**Règles :**
- Modification du plan (Appraisals Plans) met à jour tous les employés dont Next Appraisal Date est vide.
- Un employé sans utilisateur Odoo peut avoir une appraisal (notification par email vers work_email si configuré).

### 3.2 Surveys (survey)

**Données partagées :**
- **Templates d'appraisal** : Les templates sont des surveys. Pour être disponibles dans Appraisals, ils doivent être marqués « Appraisal » dans l’app Surveys (paramètre en haut du survey).
- **Structure** : Sections et questions (Employee's Feedback : My work, My future, My feelings ; Manager's Feedback : Feedback, Evaluation, Improvements).
- **360 Feedback** : Surveys dédiés (bottom-up, top-down, self-evaluations) ; créés depuis Appraisals → Configuration → 360 Feedback ou depuis Surveys ; résultats visibles dans le dashboard 360 (See Results, export PDF).

**Règles :**
- Toute création de template depuis Appraisals doit être configurée « Appraisal » dans Surveys pour apparaître dans le sélecteur Appraisals.
- Les réponses aux surveys 360 sont stockées dans Surveys ; le dashboard 360 dans Appraisals agrège les statistiques (Registered, Completed, Certified, Average Duration).

### 3.3 Mail (mail)

**Usage :**
- **Confirmation appraisal** : Après Confirm, envoi email à l’employé (lien vers l’appraisal).
- **Ask Feedback** : Pop-up email (template Appraisal: Ask Feedback) ; destinataires (employés), message, Answer Deadline ; envoi des demandes de feedback 360.
- **Réunion** : Invitation à l’événement Meeting (participants notifiés par email).
- **Chatter** : Activités et messages sur la fiche appraisal (optionnel selon configuration).

### 3.4 Calendar (calendar)

**Usage :**
- **Activité Meeting** : Depuis le dashboard Appraisals (icône activité sous la date) ou depuis la fiche appraisal (smart button Meetings / No Meeting) → Schedule an activity → Type = Meeting → création événement (Start, Attendees, option Odoo meeting / vidéocall) → Save & Close.
- **Lien** : L’événement est lié à l’appraisal (activité) ; les participants (employé par défaut) reçoivent l’invitation.

---

## 4. Schéma de Dépendances

```
Appraisals (hr_appraisal)
    │
    ├── hr (Employees) ...................... employé, manager, département, next_appraisal_date, skills
    ├── mail ................................ notifications, chatter, Ask Feedback email
    ├── survey .............................. templates appraisal, surveys 360
    └── calendar ............................ activité Meeting (entretien)
```

---

## 5. Recommandations pour Miyukini

- **EmployeeService (MiyuHR)** : Appraisals consomme les données employé (fiche, hiérarchie, compétences) ; l’Opérateur Appraisal doit obtenir un Mandat pour lire/mettre à jour next_appraisal_date et skills via KindMother et le contrat d’équipe avec EmployeeOperator / EmployeeSkillsOperator.
- **Templates / Surveys** : Équivalent Miyukini soit un Opérateur Survey dédié (MiyuSurveys ou intégration Surveys existante) avec contrat « template appraisal » et option « Appraisal » pour le sélecteur ; soit modèles d’appraisal natifs dans MiyuAppraisals sans dépendance Survey externe.
- **Notifications** : MiyuNotify pour confirmation appraisal, Ask Feedback, réunion ; respect des Mandats et pas de spam.
- **Calendrier** : Intégration avec un Opérateur Calendar / Planning (MiyuPlanning ou équivalent) pour créer l’événement « entretien d’évaluation » et notifier les participants.
- **WriteIntent** : Toute mise à jour next_appraisal_date, skills (depuis l’appraisal), goals et appraisal (statut, rating, notes) doit passer par KindMother (WriteIntent) avec validation StrongFather et permissions Master Butler.

---

**Document** : Odoo Appraisals — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
