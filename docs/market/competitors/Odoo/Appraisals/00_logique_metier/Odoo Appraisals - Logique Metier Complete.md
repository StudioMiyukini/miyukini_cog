# Odoo Appraisals — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Appraisals** (Évaluations / Entretiens annuels) d'Odoo (versions 18.0 / 19.0), à partir de la documentation officielle. Il identifie les modèles de données, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18.0/19.0 (Appraisals, Schedule appraisals, Conduct appraisals, Templates, 360 Feedback, Goals, Appraisal analysis, Skills evolution)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (appraisal, appraisal plan, template, goals, 360 feedback, evaluation scale)
- Règles métier et contraintes (planification, visibilité employé/manager, final rating)
- Workflows (planification → auto-évaluation → feedback manager → revue → clôture)
- Gestion des objectifs (goals) et des compétences (skills) dans le cadre des évaluations
- 360 Feedback et templates d'appraisal
- Analyse et rapports (appraisal analysis, skills evolution)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle Appraisal (Évaluation)

**Rôle :** Représente une évaluation de performance — cycle complet d'une revue entre un employé et son manager.

**Champs clés (synthèse documentée) :**
- **Employé** : employee_id (employé évalué)
- **Manager / Département** : manager_id, department_id, job_position (propagés depuis la fiche employé à la sélection)
- **Date** : appraisal_date (date prévue de complétion ; mise à jour à la clôture)
- **Next Appraisal Date** : date de la prochaine évaluation (Ongoing si plan actif ; mise à jour à la clôture)
- **Template** : appraisal_template_id (modèle d'évaluation — Default Template par défaut)
- **Statut** : draft → confirmed → done (avec possibilité Reopen)
- **Visibilité** :
  - Employee's Feedback : visible / not visible to manager (toggle employé)
  - Manager's Feedback : visible / not visible to employee (toggle manager)
- **Final Rating** : note finale (Needs improvement, Meets expectations, Exceeds expectations, Strongly Exceeds Expectations, Good — ou échelle personnalisée)
- **Private Note** : notes privées manager (onglet dédié ; invisible à l'employé)
- **Meeting** : lien vers activité Meeting (calendrier) pour l'entretien

**Règles métier :**
- Seules les évaluations **confirmées** sont éditables (champs, feedback, compétences).
- Une fois **Done**, l'appraisal est verrouillée sauf Reopen → Confirm → modifications → Mark as Done.
- Next Appraisal Date : vide avant plan ; « Ongoing » si plan actif ; date calculée après clôture selon plan.

### 1.2 Appraisal Plan (Plan d'évaluations)

**Rôle :** Définit la fréquence automatique des évaluations.

**Paramètres (Configuration → Settings) :**
- **Appraisals Plans** : champs en mois — par défaut 6 mois après embauche, puis 6 mois après, puis tous les 12 mois.
- **Appraisals Automation** : case à cocher pour planifier et confirmer automatiquement les appraisals selon le plan.

**Règles métier :**
- La modification du plan met à jour **tous** les employés dont Next Appraisal Date est vide.
- Plan utilisé par le cron / automatisation pour créer et éventuellement confirmer les appraisals.

### 1.3 Appraisal Template (Modèle d'évaluation)

**Rôle :** Structure des questions et sections pour l'auto-évaluation (Employee's Feedback) et le feedback manager (Manager's Feedback). Hébergé dans l'app **Surveys**.

**Structure par défaut (Default Template) :**
- **Employee's Feedback** :
  - My work (réalisations, défis, besoins d'amélioration)
  - My future (objectifs court/long terme, parties du poste aimées / moins aimées)
  - My feelings (culture, communication, contenu du poste, organisation, rémunération)
- **Manager's Feedback** :
  - Feedback (réalisations positives, forces)
  - Evaluation (Stress Resistance, Time Management, Teamwork, Autonomy, Pro-activity)
  - Improvements (axes d'amélioration, actions court/long terme, alignement vision)

**Règles métier :**
- Un template doit être marqué « Appraisal » dans l'app Surveys pour apparaître dans le sélecteur Appraisals.
- Templates par département possibles (ex. bureau vs techniciens terrain).
- Création possible par duplication du template par défaut.

### 1.4 Goals (Objectifs)

**Rôle :** Objectifs assignés aux employés ; suivis entre les évaluations et revus pendant l'appraisal.

**Champs clés :**
- **name** : intitulé de l'objectif
- **employee_id** : employé concerné
- **manager_id** : manager (auto-rempli depuis l'employé)
- **progress** : 0 %, 25 %, 50 %, 75 %, 100 %
- **deadline** : date d'échéance
- **tags** : étiquettes (External, Hard Skills, Internal, Programming, Soft Skills, Training — ou personnalisées)
- **description** : détails ; checklist possible (étapes)

**Règles métier :**
- Les objectifs sont revus pendant l'entretien d'évaluation.
- Le manager peut mettre à jour le pourcentage d'avancement à tout moment (pas seulement pendant l'appraisal).
- Mark as Done : passage à 100 % et statut terminé.

### 1.5 Evaluation Scale (Échelle d'évaluation)

**Rôle :** Référentiel des notes possibles pour le Final Rating.

**Valeurs par défaut :** Needs improvement, Meets expectations, Exceeds expectations, Strongly Exceeds Expectations, Good.

**Règles métier :** Configuration → Evaluation Scale ; ajout de lignes pour personnaliser l'échelle.

### 1.6 360 Feedback (Surveys)

**Rôle :** Sondages permettant au manager de demander des retours sur un employé à des collègues (pair, subordonnés, etc.).

**Mécanisme :**
- Depuis une appraisal **confirmée** : bouton **Ask Feedback** → sélection des destinataires (employés) → envoi email (template Appraisal: Ask Feedback).
- **Answer Deadline** : par défaut jour après Appraisal Date ; modifiable.
- Résultats : 360 Feedback dashboard (Configuration → 360 Feedback) — statistiques (Questions, Average Duration, Registered, Completed, Certified) ; See Results pour voir les réponses ; export PDF.

**Règles métier :**
- Les surveys 360 sont des surveys Odoo (app Surveys) configurées pour Appraisals.
- Nouveaux surveys créables depuis la page 360 Feedback.

### 1.7 Skills dans l'appraisal

**Rôle :** Les compétences de l'employé (fiche Employees, onglet Résumé) sont reprises dans l'onglet **Skills** de l'appraisal une fois celui-ci confirmé.

**Champs affichés / éditables :**
- Skill (type, compétence)
- Skill Level (niveau)
- Progress (barre / pourcentage)
- Justification (texte — évolution depuis dernière évaluation)

**Règles métier :**
- Si le niveau a changé depuis la dernière appraisal, l'employé (ou le manager) met à jour le niveau et peut renseigner une justification.
- Les compétences mises à jour dans l'appraisal sont reflétées sur la fiche employé après clôture.
- L'onglet Skills n'apparaît qu'après confirmation de l'appraisal.

---

## 2. Workflow d'une Évaluation

### 2.1 Planification

1. **Automatique** : Appraisals Automation activée → création (et confirmation optionnelle) selon Appraisal Plan (6 mois, 6 mois, puis 12 mois).
2. **Manuelle** : Appraisals → New → choix employé (Manager, Job Position, Department auto-remplis) → Appraisal Date → Template → **Confirm**. L'employé est notifié par email.

### 2.2 Auto-évaluation (Employee)

1. Employé reçoit notification (lien vers l'appraisal).
2. Remplit **Employee's Feedback** (sections My work, My future, My feelings).
3. Met à jour **Skills** (niveau, justification si changement).
4. Passe le toggle **Not Visible to Manager** → **Visible to Manager** (réponses alors visibles par le manager). Indicateur vert sur la carte appraisal (dashboard).

### 2.3 Feedback Manager

1. Manager peut **Ask Feedback** (360) auprès de collègues (bouton sur appraisal confirmée).
2. Manager remplit **Manager's Feedback** (Feedback, Evaluation, Improvements).
3. Optionnel : garder le feedback caché jusqu'à l'entretien (toggle **Not Visible to Employee** → **Visible to Employee** quand souhaité).

### 2.4 Revue (Appraisal Review)

1. **Planifier une réunion** : depuis le dashboard (icône activité sous la date) ou depuis la fiche appraisal (smart button **Meetings** / **No Meeting**) → Activity Type = Meeting → création événement calendrier (Odoo meeting / vidéocall possible).
2. **Contenu de l'entretien** : discussion Employee's Feedback + Manager's Feedback ; revue des **Skills** et **Goals** ; mise à jour des compétences et objectifs si besoin.

### 2.5 Clôture

1. Manager renseigne **Final Rating** (liste déroulante ; échelle configurable).
2. Manager peut ajouter une **Private Note** (onglet dédié ; invisible employé).
3. **Mark as Done** → statut passe à Done ; bouton devient **Reopen**. Plus de modification possible sauf Reopen.
4. **Next Appraisal Date** mise à jour selon plan (si activé).

---

## 3. Règles Métier et Contraintes

### 3.1 Visibilité et confidentialité

- **Employee's Feedback** : caché au manager tant que l'employé n'a pas mis « Visible to Manager ».
- **Manager's Feedback** : caché à l'employé tant que le manager n'a pas mis « Visible to Employee » (certains managers attendent l'entretien).
- **Private Note** : visible uniquement par les managers ; jamais par l'employé.

### 3.2 Droits et périmètre

- **Manager** : accès aux appraisals de ses subordonnés (hiérarchie / département) ; peut demander 360 feedback, remplir son feedback, planifier réunion, noter, clôturer.
- **Employé** : accès à ses propres appraisals ; remplissage auto-évaluation et skills ; pas d'accès Private Note.
- **HR / Admin** : accès étendu selon droits ; configuration (plans, templates, 360, Evaluation Scale, Tags).

### 3.3 Intégration Employés

- **Next Appraisal Date** sur la fiche employé (app Employees) : affichée si Appraisals installé ; mise à jour par le module Appraisals (plan ou clôture).
- **Skills** : lus depuis la fiche employé ; modifications dans l'appraisal reportées sur la fiche après clôture.

### 3.4 Analyse et rapports

- **Appraisal analysis** : vues / filtres par statut (draft, confirmed, done) ; cas d'usage « view only the user's appraisals » ; groupement par statut.
- **Skills evolution** : rapport d'évolution des compétences (identify employees with specific skills, assess highest improvement).

---

## 4. Synthèse des Entités

| Entité | Rôle |
|--------|------|
| **Appraisal** | Cycle d'évaluation (employé, manager, date, template, statut, rating, notes privées) |
| **Appraisal Plan** | Fréquence auto (6m, 6m, 12m) et automatisation |
| **Appraisal Template** | Structure des questions (Employee + Manager Feedback) — Surveys |
| **Goals** | Objectifs employé (avancement, deadline, tags) |
| **Evaluation Scale** | Notes possibles pour Final Rating |
| **360 Feedback** | Sondages pour retours pairs / collègues ; dashboard et résultats |
| **Skills (in appraisal)** | Copie des compétences fiche employé ; mise à jour niveau + justification |

---

## 5. Points d'attention pour Miyukini

- **Mandats et gouvernance** : création / confirmation / clôture d'appraisal et accès aux notes privées doivent être soumis à StrongFather et Master Butler ; données sensibles (feedback, rating) à WorrySentinel.
- **WriteIntent** : toute modification d'appraisal, goals, skills (dans le cadre appraisal) et next_appraisal_date doit passer par KindMother.
- **Collaboration mandatée** : employé et manager ne communiquent pas hors cadre ; BondingBrother comme médiation pour les flux (notification, visibilité, réunion).
- **Ever Buddy** : cycle de vie des templates et des plans (versions, dépréciation).
- **TAMR** : point d'intervention humaine pour réunion physique/virtuelle et décision finale (rating, private note).

---

**Document** : Odoo Appraisals — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
