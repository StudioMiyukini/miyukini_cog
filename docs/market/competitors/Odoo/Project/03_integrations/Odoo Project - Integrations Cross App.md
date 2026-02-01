# Odoo Project — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Project** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Code source GitHub Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs et hooks utilisés
- Événements partagés

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (`__manifest__.py`) :**
- `analytic` : Comptabilité analytique (compte analytique projet)
- `base_setup` : Configuration de base
- `mail` : Messagerie et activités (chatter, notifications)
- `portal` : Portail utilisateurs (collaborateurs externes)
- `rating` : Système de notation (ratings tâches/projets)
- `resource` : Gestion ressources (calendrier travail)
- `web` : Framework web
- `web_tour` : Tours guidés
- `digest` : Digests email

### 1.2 Modules Optionnels

**Dépendances optionnelles (intégrations si installés) :**
- `sale` : Ventes (création projet depuis commande)
- `purchase` : Achats (création projet depuis commande achat)
- `hr_timesheet` : Saisie temps (lignes analytiques, coûts)
- `account` : Comptabilité (rentabilité projet via comptabilité analytique)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Analytic (Comptabilité Analytique)

**Flux :**
```
Project → Analytic Account → Analytic Lines
```

**Mécanismes :**
- Création automatique compte analytique lors création projet
- Lien : `account_id` sur `project.project`
- Lignes analytiques liées aux tâches (si Timesheet installé)
- Calcul rentabilité projet depuis lignes analytiques

**Champs liés :**
- `account_id` : Compte analytique projet (Many2one vers `account.analytic.account`)
- `analytic_account_balance` : Solde compte analytique (related)
- `company_id` : Entreprise (calculé depuis compte analytique ou partenaire)

**Hooks utilisés :**
- `project.project._create_analytic_account()` : Création compte analytique
- `project.project._get_values_analytic_account_batch()` : Valeurs compte analytique
- `project.project._inverse_company_id()` : Mise à jour entreprise compte analytique

**Rentabilité projet :**
- Calcul depuis lignes analytiques (`account.analytic.line`)
- Revenus : Factures liées au projet
- Coûts : Temps, dépenses, achats
- Marge : Revenus - Coûts

**Recommandations pour Miyukini :**
- Intégration native avec comptabilité analytique Miyukini
- Compte analytique automatique par projet
- Calcul rentabilité depuis lignes analytiques
- Dashboard rentabilité projet

### 2.2 Intégration avec Mail (Messagerie)

**Flux :**
```
Project/Task → Mail Thread → Notifications
```

**Mécanismes :**
- Héritage `mail.thread` sur projets et tâches
- Chatter intégré (commentaires, pièces jointes)
- Notifications automatiques sur changements
- Abonnement automatique assignés

**Champs liés :**
- `message_ids` : Messages (One2many vers `mail.message`)
- `message_follower_ids` : Followers (One2many vers `mail.followers`)
- `message_partner_ids` : Partenaires followers (Many2many)
- `activity_ids` : Activités planifiées (One2many vers `mail.activity`)

**Hooks utilisés :**
- `project.project.message_subscribe()` : Abonnement projet
- `project.task.message_subscribe()` : Abonnement tâche (avec propagation projet)
- `project.task._task_message_auto_subscribe_notify()` : Notification assignation
- `project.task._creation_subtype()` : Subtype création tâche
- `project.task._track_subtype()` : Subtypes tracking

**Notifications :**
- Assignation tâche → Email assigné
- Changement stage → Notification followers
- Commentaire → Notification followers
- Changement projet → Notification followers projet

**Recommandations pour Miyukini :**
- Intégration avec système de messagerie Miyukini
- Chatter intégré projets/tâches
- Notifications intelligentes (selon contexte)
- Abonnement automatique assignés

### 2.3 Intégration avec Portal

**Flux :**
```
Project → Collaborators → Portal Access
```

**Mécanismes :**
- Partage projet avec utilisateurs portail
- Collaborateurs externes (`project.collaborator`)
- Visibilité projet (`privacy_visibility`)
- Accès limité ou complet selon configuration

**Champs liés :**
- `collaborator_ids` : Collaborateurs (One2many vers `project.collaborator`)
- `collaborator_count` : Nombre collaborateurs (calculé)
- `privacy_visibility` : Visibilité (followers, invited_users, employees, portal)
- `access_token` : Token accès portail
- `access_url` : URL accès portail (calculé)

**Hooks utilisés :**
- `project.project._change_privacy_visibility()` : Changement visibilité
- `project.project._add_collaborators()` : Ajout collaborateurs
- `project.project._add_followers()` : Ajout followers portail
- `project.project._check_project_sharing_access()` : Vérification accès partage

**Permissions portail :**
- Lecture seule : Consultation projets/tâches
- Édition limitée : Modification champs limités
- Édition complète : Modification tous champs autorisés

**Recommandations pour Miyukini :**
- Partage projet intégré au formulaire
- Gestion collaborateurs simplifiée
- Permissions clairement expliquées
- Interface portail optimisée

### 2.4 Intégration avec Rating

**Flux :**
```
Task Stage → Rating Trigger → Rating Request → Rating Response
```

**Mécanismes :**
- Rating activable par stage (`rating_active` sur `project.task.type`)
- Envoi automatique demande rating quand tâche atteint stage configuré
- Collecte ratings clients/utilisateurs
- Affichage ratings dans interface

**Champs liés :**
- `rating_active` : Rating activé (related depuis stage)
- `rating_count` : Nombre ratings (depuis `rating.mixin`)
- `rating_avg` : Moyenne ratings (depuis `rating.mixin`)
- `rating_ids` : Ratings (depuis `rating.mixin`)

**Hooks utilisés :**
- `project.task._send_task_rating_mail()` : Envoi demande rating
- `project.task.stage_id.rating_active` : Activation rating par stage
- `project.task.stage_id.rating_status` : Statut rating (stage ou closing)

**Recommandations pour Miyukini :**
- Système de rating intégré
- Rating activable par stage
- Collecte automatique feedback
- Affichage ratings dans dashboard

### 2.5 Intégration avec Resource (Calendrier Travail)

**Flux :**
```
Project → Resource Calendar → Working Time Calculation
```

**Mécanismes :**
- Calendrier travail lié au projet (`resource_calendar_id`)
- Calcul temps travail entre dates (heures/jours)
- Prise en compte jours fériés, congés

**Champs liés :**
- `resource_calendar_id` : Calendrier travail (Many2one vers `resource.calendar`, calculé depuis entreprise)

**Calculs :**
- `working_hours_open` : Heures travail jusqu'assignation
- `working_hours_close` : Heures travail jusqu'à fermeture
- `working_days_open` : Jours travail jusqu'assignation
- `working_days_close` : Jours travail jusqu'à fermeture

**Hooks utilisés :**
- `project.task._compute_elapsed()` : Calcul temps travail
- `project.project._compute_resource_calendar_id()` : Calcul calendrier projet

**Recommandations pour Miyukini :**
- Intégration avec gestion calendrier Miyukini
- Calcul temps travail automatique
- Prise en compte calendrier entreprise

### 2.6 Intégration avec Sale (si installé)

**Flux :**
```
Sale Order → Project Creation → Task Creation
```

**Mécanismes :**
- Création projet depuis commande de vente
- Lien projet ↔ commande
- Génération tâches depuis lignes commande
- Facturation projet depuis commande

**Champs liés :**
- `sale_order_id` : Commande de vente (sur projet, si sale installé)
- `sale_line_id` : Ligne commande (sur tâche, si sale installé)

**Hooks utilisés :**
- `sale.order._create_project()` : Création projet depuis commande
- `sale.order.line._create_task()` : Création tâche depuis ligne

**Recommandations pour Miyukini :**
- Intégration avec module Sales Miyukini
- Création projet automatique depuis commande
- Lien bidirectionnel projet ↔ commande

### 2.7 Intégration avec Purchase (si installé)

**Flux :**
```
Purchase Order → Project Creation → Task Creation
```

**Mécanismes :**
- Création projet depuis commande d'achat
- Lien projet ↔ commande achat
- Génération tâches depuis lignes commande

**Champs liés :**
- `purchase_order_id` : Commande achat (sur projet, si purchase installé)
- `purchase_line_id` : Ligne commande (sur tâche, si purchase installé)

**Recommandations pour Miyukini :**
- Intégration avec module Purchase Miyukini
- Création projet automatique depuis commande achat

### 2.8 Intégration avec HR Timesheet (si installé)

**Flux :**
```
Task → Timesheet Entry → Analytic Line → Project Cost
```

**Mécanismes :**
- Saisie temps sur tâches
- Génération lignes analytiques automatique
- Calcul coûts projet depuis temps
- Comparaison temps alloué vs réel

**Champs liés :**
- `timesheet_ids` : Entrées temps (One2many vers `account.analytic.line`, si timesheet installé)
- `allocated_hours` : Heures allouées (sur tâche)
- `subtask_allocated_hours` : Heures allouées sous-tâches (calculé)

**Recommandations pour Miyukini :**
- Intégration avec module Timesheet Miyukini
- Saisie temps intégrée aux tâches
- Calcul coûts automatique
- Rapports temps par projet/tâche

### 2.9 Intégration avec Account (si installé)

**Flux :**
```
Project → Analytic Account → Account Move Lines → Profitability
```

**Mécanismes :**
- Rentabilité projet calculée depuis lignes analytiques
- Revenus : Factures liées au projet
- Coûts : Temps, dépenses, achats
- Marge : Revenus - Coûts

**Champs liés :**
- `account_id` : Compte analytique (Many2one vers `account.analytic.account`)
- `analytic_account_balance` : Solde compte analytique (related)

**Hooks utilisés :**
- `project.project._get_profitability_items()` : Calcul rentabilité
- `project.project._get_profitability_aal_domain()` : Domaine lignes analytiques
- `project.project._show_profitability()` : Affichage rentabilité

**Recommandations pour Miyukini :**
- Intégration avec module Accounting Miyukini
- Calcul rentabilité automatique
- Dashboard rentabilité projet

---

## 3. Mécanismes d'Intégration

### 3.1 Héritage de Modèles

**Héritages principaux :**
- `project.project` hérite de :
  - `portal.mixin` : Accès portail
  - `mail.alias.mixin` : Alias email
  - `rating.parent.mixin` : Ratings
  - `mail.activity.mixin` : Activités
  - `mail.tracking.duration.mixin` : Tracking durée
  - `analytic.plan.fields.mixin` : Champs analytiques

- `project.task` hérite de :
  - `portal.mixin` : Accès portail
  - `mail.thread.cc` : Thread messagerie avec CC
  - `mail.activity.mixin` : Activités
  - `rating.mixin` : Ratings
  - `mail.tracking.duration.mixin` : Tracking durée
  - `html.field.history.mixin` : Historique HTML

### 3.2 Hooks et Overrides

**Hooks principaux :**
- `_create_analytic_account()` : Création compte analytique
- `_change_privacy_visibility()` : Changement visibilité
- `_task_message_auto_subscribe_notify()` : Notification assignation
- `_compute_elapsed()` : Calcul temps travail
- `_get_profitability_items()` : Calcul rentabilité

### 3.3 Événements Partagés

**Événements :**
- Création projet → Création compte analytique
- Changement visibilité → Abonnement/désabonnement utilisateurs
- Assignation tâche → Notification assigné
- Changement stage → Notification followers
- Fermeture dernière tâche récurrence → Création suivante

---

## 4. Recommandations pour Miyukini

### 4.1 Architecture Intégrations

**Recommandations :**
- Intégrations via Kits Miyukini (MiyuInvoice, MiyuContacts, etc.)
- Communication via BondingBrother
- Gouvernance via StrongFather
- Persistance via KindMother

### 4.2 Intégrations Prioritaires

**Priorité 1 :**
- Comptabilité analytique (rentabilité projet)
- Messagerie (chatter, notifications)
- Portail (collaborateurs externes)

**Priorité 2 :**
- Sales (création projet depuis commande)
- Timesheet (saisie temps)
- Rating (feedback clients)

**Priorité 3 :**
- Purchase (création projet depuis commande achat)
- Resource (calendrier travail)

### 4.3 Patterns d'Intégration

**Recommandations :**
- WriteIntent pour toutes écritures
- Mandates de Permission pour accès cross-module
- Équipes d'Opérateurs pour collaborations
- Sécurité hétérogène selon données

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
