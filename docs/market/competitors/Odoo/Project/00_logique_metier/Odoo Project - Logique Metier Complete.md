# Odoo Project — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Project** (Gestion de Projet) d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/project`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (ProjectProject, ProjectTask, ProjectMilestone, ProjectUpdate)
- Règles métier et contraintes
- Workflows et transitions d'état (tâches, projets)
- Gestion des dépendances de tâches
- Gestion des jalons (milestones)
- Tâches récurrentes
- Sous-tâches et hiérarchie
- Gestion des collaborateurs et visibilité
- Intégration avec Accounting (comptabilité analytique), Sales, Purchase

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `project.project` (Projet)

**Rôle :** Représente un **projet** — conteneur de tâches avec configuration, visibilité, et suivi.

**Champs clés :**

#### Identification
- `name` : Nom du projet (obligatoire, indexé trigram, tracking, traduisible)
- `description` : Description HTML (aide)
- `active` : Boolean (actif, défaut True, copié=False)
- `sequence` : Integer (ordre d'affichage, défaut 10)
- `is_template` : Boolean (modèle de projet, copié=False)

#### Client et entreprise
- `partner_id` : Many2one vers `res.partner` (client, bypass_search_access, tracking, domaine company)
- `company_id` : Many2one vers `res.company` (entreprise, calculé, stocké, readonly=False)
- `currency_id` : Many2one vers `res.currency` (devise, calculé depuis company, readonly)

#### Comptabilité analytique
- `account_id` : Many2one vers `account.analytic.account` (compte analytique, copié=False, ondelete set null)
- `analytic_account_balance` : Monetary (solde compte analytique, related)

#### Gestionnaire et favoris
- `user_id` : Many2one vers `res.users` (chef de projet, défaut utilisateur courant, tracking)
- `favorite_user_ids` : Many2many vers `res.users` (utilisateurs favoris, copié=False)
- `is_favorite` : Boolean (favori utilisateur courant, calculé, recherche, compute_sudo)

#### Configuration
- `label_tasks` : Char (libellé tâches, défaut "Tasks", traduisible, aide)
- `color` : Integer (index couleur)
- `privacy_visibility` : Selection (visibilité : followers, invited_users, employees, portal, obligatoire, défaut portal, tracking)
- `privacy_visibility_warning` : Char (avertissement visibilité, calculé)
- `access_instruction_message` : Char (message instructions accès, calculé)

#### Dates
- `date_start` : Date (date début, copié=False)
- `date` : Date (date expiration, copié=False, indexé, tracking, aide)

#### Fonctionnalités activables
- `allow_task_dependencies` : Boolean (dépendances tâches, inverse)
- `allow_milestones` : Boolean (jalons, inverse)
- `allow_recurring_tasks` : Boolean (tâches récurrentes, inverse)

#### Stages et types
- `stage_id` : Many2one vers `project.project.stage` (stage projet, groups stages, tracking, indexé, copié=False, défaut)
- `stage_id_color` : Integer (couleur stage, related)
- `type_ids` : Many2many vers `project.task.type` (stages tâches)
- `duration_tracking` : Json (suivi durée, groups stages)

#### Tâches
- `tasks` : One2many vers `project.task` (activités tâches)
- `task_ids` : One2many vers `project.task` (tâches, domaine is_closed=False)
- `task_count` : Integer (nombre tâches, calculé)
- `open_task_count` : Integer (nombre tâches ouvertes, calculé)
- `closed_task_count` : Integer (nombre tâches fermées, calculé)
- `task_completion_percentage` : Float (pourcentage complétion, calculé)

#### Tags
- `tag_ids` : Many2many vers `project.tags` (tags projet)

#### Propriétés
- `task_properties_definition` : PropertiesDefinition (définition propriétés tâches)

#### Mises à jour projet
- `update_ids` : One2many vers `project.update` (mises à jour)
- `update_count` : Integer (nombre mises à jour, calculé)
- `last_update_id` : Many2one vers `project.update` (dernière mise à jour, copié=False)
- `last_update_status` : Selection (statut dernière mise à jour : on_track, at_risk, off_track, on_hold, to_define, done, calculé, stocké, readonly=False, obligatoire, défaut to_define)
- `last_update_color` : Integer (couleur dernière mise à jour, calculé)

#### Jalons
- `milestone_ids` : One2many vers `project.milestone` (jalons, copié)
- `milestone_count` : Integer (nombre jalons, calculé, groups milestone)
- `milestone_count_reached` : Integer (nombre jalons atteints, calculé, groups milestone)
- `is_milestone_exceeded` : Boolean (jalon dépassé, calculé, recherche)
- `milestone_progress` : Integer (progression jalons %, calculé, groups milestone)
- `next_milestone_id` : Many2one vers `project.milestone` (prochain jalon, calculé, groups milestone)
- `can_mark_milestone_as_done` : Boolean (peut marquer jalon fait, calculé, groups milestone)
- `is_milestone_deadline_exceeded` : Boolean (échéance jalon dépassée, calculé, groups milestone)

#### Partage projet
- `collaborator_ids` : One2many vers `project.collaborator` (collaborateurs, copié=False)
- `collaborator_count` : Integer (nombre collaborateurs, calculé, compute_sudo)

#### Email et alias
- `alias_id` : Many2one vers `mail.alias` (alias email interne, aide)

#### Ressources
- `resource_calendar_id` : Many2one vers `resource.calendar` (calendrier travail, calculé)

#### Ratings
- `show_ratings` : Boolean (afficher ratings, calculé)

**Règles métier :**
- Contrainte `_project_date_greater` : `date >= date_start` (date fin >= date début)
- Un projet ne peut être supprimé que si toutes ses tâches sont supprimées (`unlink`)
- Si `account_id` existe et n'a pas de lignes analytiques, il est supprimé avec le projet
- Si `account_id` a des lignes analytiques ou plusieurs projets, changement `company_id` interdit (`_inverse_company_id`)
- `stage_id` doit appartenir à la même entreprise que le projet (`_ensure_stage_has_same_company`)
- Si `privacy_visibility` change vers portal/invited_users, clients ajoutés comme followers
- Si `privacy_visibility` change depuis portal/invited_users, utilisateurs portail désabonnés

**Workflow :**
- Pas d'états explicites sur `project.project`
- Gestion via `stage_id` (stages projet)
- Stages projet : Nouveau, En cours, Terminé, etc.

---

### 1.2 Modèle `project.task` (Tâche)

**Rôle :** Représente une **tâche** — unité de travail dans un projet.

**États (state) :**
- `01_in_progress` : En cours
- `02_changes_requested` : Modifications demandées
- `03_approved` : Approuvée
- `1_done` : Terminée
- `1_canceled` : Annulée
- `04_waiting_normal` : En attente (si dépendances)

**Champs clés :**

#### Identification
- `name` : Char (titre, tracking, obligatoire, indexé trigram)
- `description` : Html (description, sanitize_attributes=False)
- `active` : Boolean (actif, défaut True)
- `sequence` : Integer (ordre, défaut 10)
- `display_name` : Char (nom affiché, inverse pour extraction tags/users/priority)

#### Priorité
- `priority` : Selection (priorité : 0=Low, 1=Medium, 2=High, 3=Urgent, défaut '0', indexé, tracking)

#### Stage et état
- `stage_id` : Many2one vers `project.task.type` (stage tâche, calculé, stocké, readonly=False, tracking, indexé, défaut, group_expand)
- `stage_id_color` : Integer (couleur stage, related)
- `state` : Selection (état technique, copié=False, défaut '01_in_progress', obligatoire, calculé, inverse, readonly=False, stocké, indexé, récursif, tracking)
- `is_closed` : Boolean (état fermé, calculé, recherche)

#### Dates
- `create_date` : Datetime (créé le, readonly, indexé)
- `write_date` : Datetime (modifié le, readonly)
- `date_end` : Datetime (date fin, indexé, copié=False)
- `date_assign` : Datetime (date assignation, copié=False, readonly, aide)
- `date_deadline` : Datetime (échéance, indexé, tracking, copié=False)
- `date_last_stage_update` : Datetime (dernière mise à jour stage, indexé, copié=False, readonly, aide)

#### Projet et hiérarchie
- `project_id` : Many2one vers `project.project` (projet, calculé, stocké, precompute, récursif, readonly=False, indexé, tracking, change_default)
- `display_in_project` : Boolean (afficher dans projet, calculé, stocké)
- `parent_id` : Many2one vers `project.task` (tâche parente, inverse, indexé, tracking, domaine récursif)
- `child_ids` : One2many vers `project.task` (sous-tâches, domaine recurring_task=False)
- `subtask_count` : Integer (nombre sous-tâches, calculé)
- `closed_subtask_count` : Integer (nombre sous-tâches fermées, calculé)
- `subtask_completion_percentage` : Float (pourcentage complétion sous-tâches, calculé)
- `subtask_allocated_hours` : Float (heures allouées sous-tâches, calculé)

#### Assignation
- `user_ids` : Many2many vers `res.users` (assignés, tracking, défaut utilisateur courant, domaine share=False)
- `portal_user_names` : Char (noms utilisateurs portail, calculé, compute_sudo, recherche)
- `role_ids` : Many2many vers `project.role` (rôles projet, aide)

#### Stage personnel
- `personal_stage_type_ids` : Many2many vers `project.task.type` (stages personnels, group_expand)
- `personal_stage_id` : Many2one vers `project.task.stage.personal` (stage personnel utilisateur, calculé, recherche, group_expand)
- `personal_stage_type_id` : Many2one vers `project.task.type` (stage personnel, related, readonly=False, domaine user)

#### Client
- `partner_id` : Many2one vers `res.partner` (client, récursif, tracking, calculé, stocké, readonly=False, indexé btree_not_null, domaine company)
- `partner_phone` : Char (téléphone contact, calculé, inverse, stocké, copié=False)

#### Email
- `email_from` : Char (email expéditeur)
- `email_cc` : Char (CC emails, aide)

#### Entreprise
- `company_id` : Many2one vers `res.company` (entreprise, calculé, stocké, readonly=False, récursif, copié, défaut)

#### Tags
- `tag_ids` : Many2many vers `project.tags` (tags)

#### Propriétés
- `task_properties` : Properties (propriétés tâches, définition depuis projet, copié)

#### Temps alloué
- `allocated_hours` : Float (heures allouées, tracking)

#### Jalon
- `milestone_id` : Many2one vers `project.milestone` (jalon, domaine projet, calculé, readonly=False, stocké, tracking, indexé btree_not_null, aide)
- `has_late_and_unreached_milestone` : Boolean (jalon en retard non atteint, calculé, recherche)

#### Dépendances
- `allow_task_dependencies` : Boolean (dépendances activées, related projet)
- `depend_on_ids` : Many2many vers `project.task` (bloquée par, tracking, copié=False, domaine projet/id)
- `depend_on_count` : Integer (nombre dépendances, calculé, compute_sudo)
- `closed_depend_on_count` : Integer (nombre dépendances fermées, calculé, compute_sudo)
- `dependent_ids` : Many2many vers `project.task` (bloque, copié=False, domaine projet/id)
- `dependent_tasks_count` : Integer (nombre tâches dépendantes, calculé)

#### Récurrence
- `allow_recurring_tasks` : Boolean (récurrence activée, related projet)
- `recurring_task` : Boolean (tâche récurrente)
- `recurring_count` : Integer (nombre tâches récurrence, calculé)
- `recurrence_id` : Many2one vers `project.task.recurrence` (récurrence, copié=False, indexé btree_not_null)
- `repeat_interval` : Integer (répéter tous les X, défaut 1, calculé, compute_sudo, readonly=False)
- `repeat_unit` : Selection (unité répétition : day, week, month, year, défaut week, calculé, compute_sudo, readonly=False)
- `repeat_type` : Selection (type répétition : forever, until, défaut forever, calculé, compute_sudo, readonly=False)
- `repeat_until` : Date (répéter jusqu'à, calculé, compute_sudo, readonly=False)

#### Visibilité projet
- `project_privacy_visibility` : Selection (visibilité projet, related, tracking=False)

#### Métriques temps
- `working_hours_open` : Float (heures travail jusqu'assignation, calculé, stocké, digits 16,2, aggregator avg)
- `working_hours_close` : Float (heures travail jusqu'à fermeture, calculé, stocké, digits 16,2, aggregator avg)
- `working_days_open` : Float (jours travail jusqu'assignation, calculé, stocké, aggregator avg)
- `working_days_close` : Float (jours travail jusqu'à fermeture, calculé, stocké, aggregator avg)

#### Autres
- `color` : Integer (index couleur)
- `rating_active` : Boolean (rating stage actif, related)
- `attachment_ids` : One2many vers `ir.attachment` (pièces jointes, calculé, aide)
- `displayed_image_id` : Many2one vers `ir.attachment` (image couverture, domaine res_model/res_id/mimetype image)
- `is_template` : Boolean (modèle tâche)
- `has_project_template` : Boolean (projet est modèle, related)
- `has_template_ancestor` : Boolean (ancêtre modèle, calculé, recherche, récursif, stocké)
- `link_preview_name` : Char (nom aperçu lien, calculé)
- `access_token` : Char (token accès portail)
- `access_url` : Char (URL accès portail, calculé)

**Règles métier :**
- Contrainte `_recurring_task_has_no_parent` : Tâche récurrente ne peut pas avoir de parent
- Contrainte `_private_task_has_no_parent` : Tâche privée (sans projet) ne peut pas avoir de parent
- Contrainte `_ensure_company_consistency_with_partner` : Entreprise tâche doit être compatible avec entreprise partenaire
- Contrainte `_ensure_super_task_is_not_private` : Tâche avec sous-tâches ne peut pas être privée
- Contrainte `_check_no_cyclic_dependencies` : Pas de dépendances cycliques
- Contrainte `_check_parent_id` : Pas de hiérarchie récursive
- `state` calculé depuis `stage_id` et `depend_on_ids` :
  - Si dépendances ouvertes → `04_waiting_normal`
  - Sinon → `01_in_progress` (si pas fermé)
- `is_closed` = True si `state` dans CLOSED_STATES (`1_done`, `1_canceled`)

**Workflow d'états :**
```
01_in_progress → 02_changes_requested → 03_approved → 1_done
              ↓
        04_waiting_normal (si dépendances)
              ↓
        1_canceled
```

---

### 1.3 Modèle `project.milestone` (Jalon)

**Rôle :** Représente un **jalon** — point de repère dans un projet avec date d'échéance.

**Champs clés :**
- `name` : Char (nom jalon, obligatoire)
- `project_id` : Many2one vers `project.project` (projet, obligatoire)
- `deadline` : Date (échéance)
- `is_reached` : Boolean (atteint, calculé)
- `reached_date` : Date (date atteinte, calculé)

**Règles métier :**
- Un jalon est atteint si toutes ses tâches sont fermées
- Calcul automatique depuis `task_ids.state`

---

### 1.4 Modèle `project.update` (Mise à Jour Projet)

**Rôle :** Représente une **mise à jour de statut** d'un projet.

**Champs clés :**
- `name` : Char (nom mise à jour)
- `project_id` : Many2one vers `project.project` (projet)
- `status` : Selection (statut : on_track, at_risk, off_track, on_hold, done)
- `description` : Html (description)

**Statuts :**
- `on_track` : Sur la bonne voie
- `at_risk` : À risque
- `off_track` : Hors piste
- `on_hold` : En attente
- `done` : Terminé

---

### 1.5 Modèle `project.collaborator` (Collaborateur)

**Rôle :** Représente un **collaborateur externe** (portail) sur un projet.

**Champs clés :**
- `partner_id` : Many2one vers `res.partner` (partenaire collaborateur)
- `project_id` : Many2one vers `project.project` (projet)
- `limited_access` : Boolean (accès limité)

---

### 1.6 Modèle `project.role` (Rôle Projet)

**Rôle :** Représente un **rôle** dans un projet (pour templates).

**Champs clés :**
- `name` : Char (nom rôle)
- `project_ids` : Many2many vers `project.project` (projets)

---

### 1.7 Modèle `project.tags` (Tags)

**Rôle :** Représente un **tag** pour catégoriser projets et tâches.

**Champs clés :**
- `name` : Char (nom tag)
- `color` : Integer (couleur)

---

### 1.8 Modèle `project.task.type` (Stage Tâche)

**Rôle :** Représente un **stage** (état) de tâche.

**Champs clés :**
- `name` : Char (nom stage)
- `sequence` : Integer (ordre)
- `fold` : Boolean (replié dans Kanban)
- `project_ids` : Many2many vers `project.project` (projets)
- `rating_active` : Boolean (rating activé)
- `rating_status` : Selection (statut rating : stage, closing)
- `color` : Integer (couleur)

---

### 1.9 Modèle `project.project.stage` (Stage Projet)

**Rôle :** Représente un **stage** (état) de projet.

**Champs clés :**
- `name` : Char (nom stage)
- `sequence` : Integer (ordre)
- `fold` : Boolean (replié)
- `company_id` : Many2one vers `res.company` (entreprise)
- `color` : Integer (couleur)
- `mail_template_id` : Many2one vers `mail.template` (template email)

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow Tâche

**États et transitions :**

```
01_in_progress (En Cours)
  ↓
02_changes_requested (Modifications Demandées)
  ↓
03_approved (Approuvée)
  ↓
1_done (Terminée)

01_in_progress
  ↓ [si dépendances ouvertes]
04_waiting_normal (En Attente)
  ↓ [quand dépendances fermées]
01_in_progress

→ 1_canceled (Annulée)
```

**Calcul automatique état :**
- Si `allow_task_dependencies` = True et `depend_on_ids` contient tâches ouvertes → `state` = `04_waiting_normal`
- Sinon, si pas fermé → `state` = `01_in_progress`
- `state` peut être forcé manuellement mais sera recalculé si dépendances changent

**Transitions stage :**
- Changement `stage_id` → mise à jour `date_last_stage_update`
- Si stage `fold` = True → `date_end` = maintenant
- Si stage `fold` = False → `date_end` = False

### 2.2 Workflow Projet

**Stages projet :**
- Gestion via `stage_id` (project.project.stage)
- Pas d'états explicites
- Stages configurables par entreprise

---

## 3. Règles Métier et Contraintes

### 3.1 Contraintes de Données

**Contraintes sur `project.project` :**
- `name` : Obligatoire
- `date >= date_start` : Date fin >= date début
- `stage_id.company_id` doit correspondre à `company_id` (si stage a entreprise)

**Contraintes sur `project.task` :**
- `name` : Obligatoire
- `parent_id` : Pas de récursion (`_check_parent_id`)
- `depend_on_ids` : Pas de cycles (`_check_no_cyclic_dependencies`)
- `recurring_task` + `parent_id` : Incompatibles (`_recurring_task_has_no_parent`)
- `project_id` NULL + `parent_id` : Incompatibles (`_private_task_has_no_parent`)
- `subtask_count` > 0 → `project_id` obligatoire (`_ensure_super_task_is_not_private`)
- `company_id` doit correspondre à `partner_id.company_id` (`_ensure_company_consistency_with_partner`)

### 3.2 Règles de Calcul

**Calcul état tâche (`_compute_state`) :**
- Si `allow_task_dependencies` = True :
  - Si `depend_on_ids` contient tâches ouvertes → `state` = `04_waiting_normal`
  - Sinon → `state` = `01_in_progress` (si pas fermé)
- Sinon → `state` = `01_in_progress` (si pas fermé)

**Calcul complétion (`_compute_subtask_count`, `_compute_task_completion_percentage`) :**
- `subtask_count` : Nombre total sous-tâches
- `closed_subtask_count` : Nombre sous-tâches fermées
- `subtask_completion_percentage` : `closed_subtask_count / subtask_count * 100`
- `task_completion_percentage` : `1 - open_task_count / task_count`

**Calcul temps travail (`_compute_elapsed`) :**
- `working_hours_open` : Heures travail entre `create_date` et `date_assign` (via `resource_calendar_id`)
- `working_hours_close` : Heures travail entre `create_date` et `date_end`
- `working_days_open` : Jours travail entre `create_date` et `date_assign`
- `working_days_close` : Jours travail entre `create_date` et `date_end`

**Calcul jalon (`_compute_next_milestone_id`) :**
- `next_milestone_id` : Premier jalon non atteint du projet
- `can_mark_milestone_as_done` : True si toutes tâches jalon fermées
- `is_milestone_deadline_exceeded` : True si échéance jalon dépassée et jalon non atteint

### 3.3 Gestion des Dépendances

**Dépendances (`allow_task_dependencies`) :**
- `depend_on_ids` : Tâches qui bloquent cette tâche
- `dependent_ids` : Tâches bloquées par cette tâche
- Calcul automatique `state` = `04_waiting_normal` si dépendances ouvertes
- Pas de cycles autorisés

**Résolution dépendances :**
- Quand toutes dépendances fermées → `state` passe automatiquement à `01_in_progress`
- Si dépendances réactivées → `state` repasse à `04_waiting_normal`

### 3.4 Gestion des Sous-tâches

**Hiérarchie :**
- `parent_id` : Tâche parente
- `child_ids` : Sous-tâches
- Récursion interdite
- Sous-tâches héritent `project_id` du parent si pas défini
- Sous-tâches ne peuvent pas être récurrentes

**Calculs :**
- `subtask_count` : Nombre total sous-tâches (récursif)
- `closed_subtask_count` : Nombre sous-tâches fermées
- `subtask_allocated_hours` : Somme heures allouées sous-tâches

### 3.5 Gestion de la Récurrence

**Récurrence (`allow_recurring_tasks`) :**
- `recurring_task` : Tâche récurrente
- `recurrence_id` : Récurrence liée
- `repeat_interval`, `repeat_unit`, `repeat_type`, `repeat_until` : Paramètres récurrence
- Création automatique occurrences suivantes quand dernière tâche fermée

**Règles :**
- Tâche récurrente ne peut pas avoir de parent
- Dernière tâche récurrence → création suivante automatique
- Suppression récurrence → toutes tâches deviennent non récurrentes

---

## 4. Gestion de la Visibilité et Partage

### 4.1 Niveaux de Visibilité (`privacy_visibility`)

**Niveaux :**
- `followers` : Utilisateurs internes invités uniquement
- `invited_users` : Utilisateurs internes et portail invités
- `employees` : Tous utilisateurs internes
- `portal` : Tous utilisateurs internes + portail invités

**Règles :**
- Changement visibilité → abonnement/désabonnement automatique
- Portail → clients ajoutés comme followers
- Portail désactivé → utilisateurs portail désabonnés

### 4.2 Collaborateurs (`project.collaborator`)

**Rôle :**
- Collaborateurs externes (portail) sur projet
- `limited_access` : Accès limité ou complet
- Ajout automatique comme followers

---

## 5. Intégrations avec Autres Modules

### 5.1 Accounting (Comptabilité Analytique)

**Intégration :**
- `account_id` : Compte analytique lié au projet
- `analytic_account_balance` : Solde compte analytique
- Lignes analytiques liées aux tâches (si Timesheet installé)
- Rentabilité projet calculée depuis lignes analytiques

**Champs liés :**
- `account.analytic.account` : Compte analytique projet
- `account.analytic.line` : Lignes analytiques (temps, coûts, revenus)

### 5.2 Sales (si installé)

**Intégration :**
- Création projet depuis commande de vente
- Lien projet ↔ commande
- Facturation projet depuis commande

### 5.3 Purchase (si installé)

**Intégration :**
- Création projet depuis commande d'achat
- Lien projet ↔ commande achat

### 5.4 Timesheet (si installé)

**Intégration :**
- Saisie temps sur tâches
- Lignes analytiques générées
- Calcul coûts projet

---

## 6. Considérations pour Miyukini COG

### 6.1 Architecture Opérateurs

**Opérateurs proposés :**
1. **ProjectOperator** : Gestion des projets
2. **TaskOperator** : Gestion des tâches
3. **MilestoneOperator** : Gestion des jalons
4. **ProjectUpdateOperator** : Gestion des mises à jour projet
5. **ProjectCollaboratorOperator** : Gestion des collaborateurs
6. **ProjectUI** : Interface utilisateur Project

### 6.2 Gouvernance COG

**StrongFather (Décisions) :**
- Autorisation création/modification projet
- Autorisation création/modification tâche
- Autorisation changement visibilité
- Validation dépendances

**KindMother (Persistance) :**
- Toutes les écritures via `WriteIntent`
- Projets, tâches, jalons, mises à jour

**Master Butler (Permissions) :**
- Permissions création/modification projet/tâche
- Permissions visibilité selon `privacy_visibility`
- Isolation cross-équipe

**WorrySentinel (Sécurité) :**
- Niveau sécurité : 1-2 (Standard à Sensitive) selon données
- Vérification visibilité cross-équipe
- Audit des changements visibilité

**Ever Buddy (Cycle de Vie) :**
- Gestion transitions état tâches
- Gestion cycle de vie projets
- Gestion dépréciation/retrait fonctionnalités

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
