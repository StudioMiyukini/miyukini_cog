# Odoo Project — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Project** d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents Project
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **ProjectOperator** | Gestion des projets | Opérateur de Service |
| **TaskOperator** | Gestion des tâches | Opérateur de Service |
| **MilestoneOperator** | Gestion des jalons | Opérateur de Service |
| **ProjectUpdateOperator** | Gestion des mises à jour projet | Opérateur de Service |
| **ProjectCollaboratorOperator** | Gestion des collaborateurs | Opérateur de Service |
| **ProjectUI** | Interface utilisateur Project | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : ProjectService

**Définition :**
> **ProjectService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion de projets et tâches.**

**Composition :**
- ProjectOperator (niveau sécurité 2)
- TaskOperator (niveau sécurité 2)
- MilestoneOperator (niveau sécurité 2)
- ProjectUpdateOperator (niveau sécurité 2)
- ProjectCollaboratorOperator (niveau sécurité 1-2)
- ProjectUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 ProjectOperator

**Rôle :** Gestion des projets (création, configuration, suivi).

**Capacités :**
- Création/modification de projets
- Configuration visibilité et collaborateurs
- Gestion des stages projet
- Gestion des fonctionnalités (jalons, dépendances, récurrence)
- Calcul des métriques projet (complétion, tâches ouvertes/fermées)
- Gestion des templates projet

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification projet, changement visibilité
- **KindMother** : Persistance des projets (WriteIntent)
- **Master Butler** : Permissions de création/modification projet
- **WorrySentinel** : Vérification niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Gestion du cycle de vie projet

**Contrat d'équipe :**
- Consomme : TaskOperator (tâches), MilestoneOperator (jalons), ProjectCollaboratorOperator (collaborateurs), MiyuContacts (clients), MiyuInvoice (compte analytique)
- Expose : `project.create`, `project.update`, `project.share`, `project.archive`

**Mandat de Permission requis :**
- Création projet : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification projet : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Partage projet : Mandat avec ProjectCollaboratorOperator + MiyuNotify (notifications)
- Changement visibilité : Mandat avec StrongFather (décision) + ProjectCollaboratorOperator

### 2.2 TaskOperator

**Rôle :** Gestion des tâches (création, assignation, suivi, fermeture).

**Capacités :**
- Création/modification de tâches
- Assignation de tâches
- Gestion des états (in_progress, changes_requested, approved, done, canceled, waiting)
- Gestion des dépendances
- Gestion des sous-tâches
- Gestion de la récurrence
- Calcul des métriques (complétion, temps travail)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification tâche, assignation
- **KindMother** : Persistance des tâches (WriteIntent)
- **Master Butler** : Permissions de création/modification/assignation
- **WorrySentinel** : Vérification niveau sécurité, isolation cross-équipe
- **Ever Buddy** : Gestion du cycle de vie tâche, transitions état

**Contrat d'équipe :**
- Consommé par : ProjectOperator
- Consomme : ProjectOperator (projet), MiyuContacts (client), MiyuClock (dates), MiyuNotify (notifications)
- Expose : `task.create`, `task.update`, `task.assign`, `task.close`, `task.cancel`

**Mandat de Permission requis :**
- Création tâche : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Modification tâche : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Assignation tâche : Mandat avec StrongFather (décision) + MiyuNotify (notification)
- Fermeture tâche : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Création sous-tâche : Mandat avec TaskOperator (récursif) + KindMother (WriteIntent)

### 2.3 MilestoneOperator

**Rôle :** Gestion des jalons (création, association tâches, suivi progression).

**Capacités :**
- Création/modification de jalons
- Association tâches aux jalons
- Calcul progression jalon (tâches fermées / total)
- Détection jalons en retard
- Marquage jalon atteint

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification jalon
- **KindMother** : Persistance des jalons (WriteIntent)
- **Master Butler** : Permissions de création/modification
- **WorrySentinel** : Vérification niveau sécurité

**Contrat d'équipe :**
- Consommé par : ProjectOperator
- Consomme : TaskOperator (tâches), MiyuClock (dates)
- Expose : `milestone.create`, `milestone.update`, `milestone.associate_tasks`

**Mandat de Permission requis :**
- Création jalon : Mandat avec KindMother (WriteIntent) + StrongFather (décision)
- Association tâches : Mandat avec TaskOperator + KindMother (WriteIntent)

### 2.4 ProjectUpdateOperator

**Rôle :** Gestion des mises à jour de statut projet.

**Capacités :**
- Création de mises à jour projet
- Gestion des statuts (on_track, at_risk, off_track, on_hold, done)
- Historique des mises à jour
- Notifications aux followers

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création mise à jour
- **KindMother** : Persistance des mises à jour (WriteIntent)
- **Master Butler** : Permissions de création
- **MiyuNotify** : Notifications aux followers

**Contrat d'équipe :**
- Consommé par : ProjectOperator
- Consomme : MiyuNotify (notifications)
- Expose : `update.create`, `update.list`

**Mandat de Permission requis :**
- Création mise à jour : Mandat avec KindMother (WriteIntent) + MiyuNotify (notifications)

### 2.5 ProjectCollaboratorOperator

**Rôle :** Gestion des collaborateurs externes (portail).

**Capacités :**
- Ajout/suppression de collaborateurs
- Gestion des permissions (accès limité/complet)
- Gestion des followers portail
- Génération tokens d'accès

**Niveau de sécurité :** 1-2 (Standard à Sensitive selon données)

**Gouvernance :**
- **StrongFather** : Décision d'ajout collaborateur
- **KindMother** : Persistance des collaborateurs (WriteIntent)
- **Master Butler** : Permissions d'accès portail
- **WorrySentinel** : Vérification niveau sécurité selon visibilité projet

**Contrat d'équipe :**
- Consommé par : ProjectOperator
- Consomme : MiyuContacts (partenaires), MiyuPortal (accès portail)
- Expose : `collaborator.add`, `collaborator.remove`, `collaborator.update_permissions`

**Mandat de Permission requis :**
- Ajout collaborateur : Mandat avec KindMother (WriteIntent) + MiyuPortal (accès) + MiyuNotify (invitation)
- Suppression collaborateur : Mandat avec KindMother (WriteIntent) + MiyuPortal (révocation accès)

### 2.6 ProjectUI

**Rôle :** Interface utilisateur pour projets et tâches.

**Capacités :**
- Affichage projets (Kanban, Liste, Formulaire)
- Affichage tâches (Kanban, Liste, Formulaire, Calendrier, Graphique, Pivot)
- Création rapide projets/tâches
- Drag & drop tâches entre stages
- Filtres et groupements
- Recherche

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions d'accès aux vues
- **WorrySentinel** : Filtrage données selon permissions

**Contrat d'équipe :**
- Consomme : ProjectOperator, TaskOperator, MilestoneOperator, ProjectUpdateOperator
- Expose : `ui.render_project_kanban`, `ui.render_task_list`, `ui.render_task_form`

**Mandat de Permission requis :**
- Affichage projets/tâches : Mandat avec ProjectOperator/TaskOperator (lecture)
- Création projets/tâches : Mandat avec ProjectOperator/TaskOperator (création)

---

## 3. Contrats d'Équipe

### 3.1 Contrat ProjectService

**Opérateurs membres :**
- ProjectOperator
- TaskOperator
- MilestoneOperator
- ProjectUpdateOperator
- ProjectCollaboratorOperator
- ProjectUI

**Flux autorisés :**
- ProjectUI → ProjectOperator (lecture/écriture projets)
- ProjectUI → TaskOperator (lecture/écriture tâches)
- ProjectOperator → TaskOperator (création tâches projet)
- ProjectOperator → MilestoneOperator (création jalons)
- ProjectOperator → ProjectUpdateOperator (création mises à jour)
- ProjectOperator → ProjectCollaboratorOperator (gestion collaborateurs)
- TaskOperator → TaskOperator (création sous-tâches, dépendances)

**Types d'échanges :**
- Requêtes CRUD (Create, Read, Update, Delete)
- Notifications (assignation, changement état)
- Calculs (métriques, progression)

**Niveau de validation requis :**
- Création : StrongFather (décision) + KindMother (WriteIntent)
- Modification : StrongFather (décision) + KindMother (WriteIntent)
- Lecture : Master Butler (permissions)

---

## 4. Mandats de Permission

### 4.1 Mandat Création Projet

**Émis par :** StrongFather

**Opérateurs autorisés :** ProjectOperator, ProjectUI

**Flux autorisés :**
- ProjectUI → ProjectOperator → KindMother (WriteIntent)

**Durée :** Session

**Conditions de révocation :**
- Session terminée
- Violation de règle
- Alerte WorrySentinel

### 4.2 Mandat Création Tâche

**Émis par :** StrongFather

**Opérateurs autorisés :** TaskOperator, ProjectUI

**Flux autorisés :**
- ProjectUI → TaskOperator → KindMother (WriteIntent)
- ProjectOperator → TaskOperator → KindMother (WriteIntent)

**Durée :** Session

**Conditions de révocation :**
- Session terminée
- Violation de règle
- Alerte WorrySentinel

### 4.3 Mandat Assignation Tâche

**Émis par :** StrongFather

**Opérateurs autorisés :** TaskOperator, ProjectUI

**Flux autorisés :**
- TaskOperator → KindMother (WriteIntent)
- TaskOperator → MiyuNotify (notification)

**Durée :** Session

**Conditions de révocation :**
- Session terminée
- Tâche fermée
- Alerte WorrySentinel

### 4.4 Mandat Partage Projet

**Émis par :** StrongFather

**Opérateurs autorisés :** ProjectOperator, ProjectCollaboratorOperator

**Flux autorisés :**
- ProjectOperator → ProjectCollaboratorOperator → KindMother (WriteIntent)
- ProjectCollaboratorOperator → MiyuPortal (accès)
- ProjectCollaboratorOperator → MiyuNotify (invitation)

**Durée :** Session

**Conditions de révocation :**
- Session terminée
- Projet archivé
- Alerte WorrySentinel

---

## 5. Intégrations avec Kits Miyukini

### 5.1 MiyuContacts

**Usage :** Clients projets (`partner_id`)

**Intégration :**
- ProjectOperator consomme MiyuContacts (lecture clients)
- TaskOperator consomme MiyuContacts (lecture clients)

### 5.2 MiyuInvoice

**Usage :** Compte analytique projet (`account_id`)

**Intégration :**
- ProjectOperator consomme MiyuInvoice (compte analytique)
- Calcul rentabilité projet depuis lignes analytiques

### 5.3 MiyuClock

**Usage :** Dates projets/tâches, échéances

**Intégration :**
- ProjectOperator consomme MiyuClock (dates projet)
- TaskOperator consomme MiyuClock (échéances tâches)
- MilestoneOperator consomme MiyuClock (échéances jalons)

### 5.4 MiyuNotify

**Usage :** Notifications assignation, changements, mises à jour

**Intégration :**
- TaskOperator consomme MiyuNotify (notifications assignation)
- ProjectUpdateOperator consomme MiyuNotify (notifications mises à jour)
- ProjectCollaboratorOperator consomme MiyuNotify (invitations)

### 5.5 MiyuPortal

**Usage :** Accès portail collaborateurs

**Intégration :**
- ProjectCollaboratorOperator consomme MiyuPortal (gestion accès)

---

## 6. Sécurité Hétérogène

**Principe :** L'Équipe ProjectService combine différents niveaux de sécurité.

**Niveaux :**
- ProjectUI : 1 (Standard) — Interface seule
- ProjectCollaboratorOperator : 1-2 (Standard à Sensitive) — Données collaborateur
- ProjectOperator, TaskOperator, MilestoneOperator, ProjectUpdateOperator : 2 (Sensitive) — Données projet/tâche

**Règles :**
- Un Opérateur ne peut jamais élever son niveau
- Un flux ne peut jamais descendre en sécurité
- Les ponts entre niveaux sont explicites, rares, auditables
- Les ponts sont validés par WorrySentinel

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
