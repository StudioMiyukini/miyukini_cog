# Odoo Project — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Project** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Code source GitHub Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Chef de Projet (Project Manager)

**Profil :**
- Rôle stratégique : Gestion globale des projets
- Responsabilités :
  - Créer et configurer des projets
  - Assigner des tâches aux membres d'équipe
  - Suivre l'avancement des projets
  - Gérer les jalons (milestones)
  - Publier des mises à jour projet
  - Gérer la visibilité et les collaborateurs
  - Analyser la rentabilité projet

**Besoins :**
- Tableau de bord projet avec KPIs
- Vue d'ensemble des tâches et leur statut
- Gestion des jalons et échéances
- Rapports d'avancement
- Gestion des collaborateurs externes (portail)
- Configuration de la visibilité projet

**Permissions :**
- `group_project_manager` : Manager Project
- Peut créer/modifier/supprimer projets
- Peut gérer tous les projets de l'entreprise
- Accès aux rapports et analyses

### 1.2 Membre d'Équipe (Team Member / Project User)

**Profil :**
- Rôle opérationnel : Exécution des tâches
- Responsabilités :
  - Consulter ses tâches assignées
  - Mettre à jour le statut des tâches
  - Ajouter des commentaires et pièces jointes
  - Créer des sous-tâches
  - Gérer ses stages personnels
  - Saisir du temps (si Timesheet installé)

**Besoins :**
- Vue "Mes Tâches" personnalisée
- Interface simple pour mettre à jour les tâches
- Notifications sur nouvelles assignations
- Vue Kanban pour organiser son travail
- Stages personnels (Inbox, Today, This Week, etc.)

**Permissions :**
- `group_project_user` : Utilisateur Project standard
- Peut créer/modifier ses tâches assignées
- Peut créer des tâches dans les projets accessibles
- Accès limité aux projets où il est membre

### 1.3 Client (Customer / Portal User)

**Profil :**
- Rôle externe : Suivi de projet client
- Responsabilités :
  - Consulter les projets qui le concernent
  - Voir l'avancement des tâches
  - Commenter sur les tâches (selon permissions)
  - Recevoir des notifications projet

**Besoins :**
- Accès portail aux projets clients
- Vue simplifiée des tâches
- Notifications par email
- Possibilité de commenter (selon configuration)

**Permissions :**
- Utilisateur portail (`share=True`)
- Accès uniquement aux projets où il est collaborateur/follower
- Permissions limitées selon `privacy_visibility` et `limited_access`

### 1.4 Collaborateur Externe (External Collaborator)

**Profil :**
- Rôle externe : Contribution limitée au projet
- Responsabilités :
  - Consulter les tâches assignées
  - Mettre à jour le statut (selon permissions)
  - Commenter sur les tâches

**Besoins :**
- Accès portail avec permissions limitées
- Vue simplifiée des tâches pertinentes
- Notifications sur changements

**Permissions :**
- Collaborateur portail (`project.collaborator`)
- `limited_access` : Accès limité ou complet selon configuration
- Permissions selon `privacy_visibility` projet

---

## 2. Parcours d'Onboarding

### 2.1 Chef de Projet — Création Premier Projet

**Scénario :**
1. **Accès initial :**
   - Connexion à Odoo
   - Menu "Project" → "Projects"
   - Vue liste vide avec message d'aide

2. **Création projet :**
   - Clic "Create" → Formulaire projet
   - Saisie nom projet (obligatoire)
   - Configuration :
     - Client (`partner_id`) — optionnel
     - Dates début/fin — optionnels
     - Visibilité (`privacy_visibility`) — défaut "portal"
     - Fonctionnalités (dépendances, jalons, récurrence) — selon groupes
   - Sauvegarde → Projet créé

3. **Configuration stages tâches :**
   - Par défaut : Stages génériques créés automatiquement
   - Personnalisation possible via `type_ids`

4. **Création première tâche :**
   - Depuis projet → Bouton "Tasks" → "Create"
   - Formulaire rapide ou complet
   - Saisie titre, assignation, échéance
   - Sauvegarde → Tâche créée

**Points d'aide :**
- Message d'aide dans vue liste vide
- Tooltips sur champs importants
- Tour guidé (si `web_tour` installé)

### 2.2 Membre d'Équipe — Première Utilisation

**Scénario :**
1. **Découverte :**
   - Menu "Project" → "My Tasks"
   - Vue Kanban avec stages personnels par défaut :
     - Inbox
     - Today
     - This Week
     - This Month
     - Later
     - Done
     - Cancelled

2. **Première assignation :**
   - Notification email (si configuré)
   - Tâche apparaît dans "Inbox"
   - Clic sur tâche → Formulaire détaillé

3. **Mise à jour tâche :**
   - Changement stage personnel
   - Ajout commentaire
   - Mise à jour statut (`state`)
   - Ajout pièces jointes

**Points d'aide :**
- Stages personnels créés automatiquement
- Notifications sur assignation
- Interface intuitive Kanban

### 2.3 Client — Accès Portail

**Scénario :**
1. **Invitation :**
   - Email d'invitation projet (si `privacy_visibility` = portal/invited_users)
   - Lien vers portail Odoo

2. **Connexion portail :**
   - Authentification portail
   - Menu "My Projects" → Liste projets accessibles

3. **Consultation projet :**
   - Clic projet → Vue projet portail
   - Onglet "Tasks" → Liste tâches
   - Clic tâche → Détails (selon permissions)

**Points d'aide :**
- Interface portail simplifiée
- Permissions clairement indiquées
- Notifications sur changements projet

---

## 3. Scénarios d'Usage Principaux

### 3.1 Création et Gestion Projet

**Acteur :** Chef de Projet

**Scénario :**
1. **Création projet :**
   - Menu "Project" → "Projects" → "Create"
   - Saisie nom, client, dates
   - Configuration visibilité
   - Activation fonctionnalités (jalons, dépendances)
   - Sauvegarde

2. **Configuration stages :**
   - Onglet projet → Configuration stages
   - Ajout/modification stages tâches
   - Ordre et couleurs

3. **Ajout collaborateurs :**
   - Bouton "Share Project"
   - Sélection utilisateurs internes/externes
   - Configuration accès limité/complet
   - Envoi invitations

4. **Création jalons :**
   - Onglet "Milestones" → "Create"
   - Saisie nom, échéance
   - Association tâches

**Points de friction Odoo :**
- Configuration visibilité peut être confuse
- Gestion collaborateurs via wizard séparé
- Jalons nécessitent groupe spécifique

**Recommandations Miyukini :**
- Interface unifiée pour configuration projet
- Gestion collaborateurs intégrée au formulaire projet
- Jalons activables simplement

### 3.2 Création et Assignation Tâches

**Acteur :** Chef de Projet / Membre Équipe

**Scénario :**
1. **Création rapide :**
   - Vue Kanban projet → Clic colonne stage → Saisie titre
   - Création immédiate avec valeurs par défaut
   - Ou formulaire complet via "Create"

2. **Création avec détails :**
   - Formulaire complet
   - Saisie titre, description, assignation, échéance
   - Ajout tags, propriétés personnalisées
   - Configuration récurrence (si activée)
   - Sauvegarde

3. **Assignation multiple :**
   - Vue liste → Sélection multiple tâches
   - Action "Assign" → Sélection utilisateurs
   - Application en lot

**Points de friction Odoo :**
- Création rapide limitée (titre uniquement)
- Assignation multiple nécessite action séparée
- Pas de création en masse depuis template

**Recommandations Miyukini :**
- Création rapide enrichie (assignation, échéance)
- Assignation multiple intégrée à la vue
- Templates de tâches réutilisables

### 3.3 Suivi Avancement Tâches

**Acteur :** Membre Équipe

**Scénario :**
1. **Vue "My Tasks" :**
   - Menu "Project" → "My Tasks"
   - Vue Kanban par stages personnels
   - Drag & drop entre stages

2. **Mise à jour statut :**
   - Clic tâche → Formulaire
   - Changement `state` (In Progress → Approved → Done)
   - Ou changement `stage_id` (automatique `state`)

3. **Gestion dépendances :**
   - Onglet "Blocked By" → Ajout dépendances
   - Tâche passe automatiquement en "Waiting" si dépendances ouvertes
   - Notification quand dépendances fermées

4. **Sous-tâches :**
   - Onglet "Sub-tasks" → Création sous-tâches
   - Suivi complétion automatique
   - Hiérarchie visible dans Kanban

**Points de friction Odoo :**
- États (`state`) vs Stages (`stage_id`) peuvent être confus
- Dépendances nécessitent activation projet
- Sous-tâches pas toujours visibles dans Kanban principal

**Recommandations Miyukini :**
- Unification état/stage pour simplicité
- Dépendances activables par défaut
- Sous-tâches visibles dans vue parente

### 3.4 Gestion Jalons (Milestones)

**Acteur :** Chef de Projet

**Scénario :**
1. **Création jalon :**
   - Projet → Onglet "Milestones" → "Create"
   - Saisie nom, échéance
   - Sauvegarde

2. **Association tâches :**
   - Tâche → Champ `milestone_id`
   - Sélection jalon
   - Tâche liée au jalon

3. **Suivi progression :**
   - Dashboard projet → Section "Milestones"
   - Affichage jalons avec progression
   - Indicateur jalons en retard

4. **Marquage jalon atteint :**
   - Automatique si toutes tâches jalon fermées
   - Ou manuel via bouton "Mark as Done"

**Points de friction Odoo :**
- Jalons nécessitent groupe spécifique
- Association tâches manuelle
- Pas de vue dédiée jalons dans Kanban

**Recommandations Miyukini :**
- Jalons intégrés par défaut
- Vue Kanban par jalons disponible
- Association automatique si possible

### 3.5 Mises à Jour Projet (Project Updates)

**Acteur :** Chef de Projet

**Scénario :**
1. **Création mise à jour :**
   - Dashboard projet → Section "Status"
   - Sélection statut (On Track, At Risk, Off Track, On Hold)
   - Description optionnelle
   - Sauvegarde → Mise à jour créée

2. **Historique :**
   - Onglet "Updates" → Liste mises à jour
   - Filtres par statut, date
   - Consultation détails

3. **Notifications :**
   - Followers projet notifiés automatiquement
   - Email selon préférences

**Points de friction Odoo :**
- Mises à jour peu visibles dans interface
- Pas de template de mise à jour
- Pas de rapports automatiques

**Recommandations Miyukini :**
- Mises à jour proéminentes dans dashboard
- Templates de mise à jour réutilisables
- Rapports automatiques périodiques

### 3.6 Partage Projet avec Clients

**Acteur :** Chef de Projet

**Scénario :**
1. **Configuration visibilité :**
   - Projet → Champ `privacy_visibility`
   - Sélection "portal" ou "invited_users"
   - Sauvegarde → Clients ajoutés automatiquement comme followers

2. **Ajout collaborateurs :**
   - Bouton "Share Project" → Wizard
   - Sélection partenaires clients
   - Configuration accès limité/complet
   - Envoi invitations

3. **Accès client :**
   - Client reçoit email invitation
   - Connexion portail → Menu "My Projects"
   - Consultation projet et tâches
   - Commentaires selon permissions

**Points de friction Odoo :**
- Wizard partage séparé du formulaire
- Configuration visibilité peut être oubliée
- Permissions portail pas toujours claires

**Recommandations Miyukini :**
- Partage intégré au formulaire projet
- Avertissements si visibilité incompatible avec collaborateurs
- Permissions clairement expliquées

### 3.7 Tâches Récurrentes

**Acteur :** Chef de Projet / Membre Équipe

**Scénario :**
1. **Activation récurrence projet :**
   - Projet → Configuration → Activer "Recurring Tasks"

2. **Création tâche récurrente :**
   - Tâche → Cocher "Recurrent"
   - Configuration :
     - Intervalle (tous les X jours/semaines/mois/années)
     - Type (Forever / Until date)
   - Sauvegarde → Récurrence créée

3. **Génération occurrences :**
   - Quand dernière tâche récurrence fermée → Création automatique suivante
   - Tâches liées via `recurrence_id`

4. **Gestion récurrence :**
   - Tâche → Bouton "Tasks in Recurrence"
   - Vue toutes occurrences
   - Action "Unlink Recurrence" pour arrêter

**Points de friction Odoo :**
- Récurrence nécessite activation projet
- Génération seulement à fermeture dernière tâche
- Pas de prévisualisation occurrences futures

**Recommandations Miyukini :**
- Récurrence activable par défaut
- Option génération anticipée
- Prévisualisation occurrences futures

---

## 4. Points de Friction Identifiés

### 4.1 Complexité États vs Stages

**Problème :**
- Deux systèmes parallèles : `state` (technique) et `stage_id` (métier)
- Confusion utilisateurs sur lequel utiliser
- Calcul automatique `state` depuis `stage_id` et dépendances

**Impact :**
- Courbe d'apprentissage élevée
- Erreurs de configuration
- Support utilisateur nécessaire

**Recommandations Miyukini :**
- Unifier en un seul système
- Ou clarifier rôles (état = automatique, stage = métier)

### 4.2 Gestion Visibilité Projet

**Problème :**
- 4 niveaux visibilité (`followers`, `invited_users`, `employees`, `portal`)
- Règles complexes d'abonnement automatique
- Changement visibilité peut désabonner utilisateurs

**Impact :**
- Configuration difficile
- Risque perte accès utilisateurs
- Support nécessaire

**Recommandations Miyukini :**
- Simplifier niveaux visibilité
- Avertissements clairs avant changement
- Migration automatique followers

### 4.3 Dépendances Tâches

**Problème :**
- Nécessite activation projet (`allow_task_dependencies`)
- Calcul automatique état peut surprendre
- Pas de visualisation graphe dépendances

**Impact :**
- Fonctionnalité peu utilisée
- Confusion sur états "Waiting"
- Pas de vue d'ensemble dépendances

**Recommandations Miyukini :**
- Dépendances activées par défaut
- Visualisation graphe dépendances
- Avertissements clairs sur blocages

### 4.4 Tâches Privées vs Projet

**Problème :**
- Tâches peuvent être sans projet (`project_id` = False)
- Appelées "Private Tasks"
- Mais peuvent avoir sous-tâches (contradiction)

**Impact :**
- Confusion conceptuelle
- Règles complexes (pas de parent si privée)
- Support nécessaire

**Recommandations Miyukini :**
- Clarifier concept tâches privées
- Ou supprimer si pas nécessaire
- Règles cohérentes

### 4.5 Templates Projets

**Problème :**
- Projets peuvent être templates (`is_template`)
- Création projet depuis template
- Mais gestion templates peu intuitive

**Impact :**
- Fonctionnalité peu utilisée
- Pas de marketplace templates
- Support nécessaire

**Recommandations Miyukini :**
- Templates proéminents dans interface
- Marketplace templates (si applicable)
- Guides d'utilisation

---

## 5. Recommandations pour Miyukini

### 5.1 Simplification Interface

**Recommandations :**
- Unifier états/stages pour simplicité
- Interface unifiée configuration projet
- Création rapide enrichie (assignation, échéance)
- Dépendances activées par défaut

### 5.2 Amélioration Collaboration

**Recommandations :**
- Partage intégré au formulaire projet
- Permissions clairement expliquées
- Notifications intelligentes (selon contexte)
- Chat intégré (si applicable)

### 5.3 Visualisation Données

**Recommandations :**
- Dashboard projet avec KPIs clairs
- Vue graphe dépendances
- Timeline projet avec jalons
- Rapports automatiques

### 5.4 Gestion Temps (si Timesheet)

**Recommandations :**
- Saisie temps intégrée aux tâches
- Rapports temps par projet/tâche
- Comparaison temps alloué vs réel
- Alertes dépassement budget

### 5.5 Intégrations

**Recommandations :**
- Intégration Sales (création projet depuis commande)
- Intégration Accounting (rentabilité projet)
- Intégration Purchase (si applicable)
- API REST pour intégrations externes

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
