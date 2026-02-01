# Odoo Project — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Project** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les composants d'interface, patterns de navigation, formulaires, tableaux, rapports et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/project/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Form, Calendar, Graph, Pivot, Activity)
- Composants d'interface spécialisés
- Patterns de navigation
- Formulaires et validations
- Tableaux et listes
- Design responsive et accessibilité

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales — Projets

### 1.1 Vue Liste (List View) — `project.project`

**Fichier :** `project_project_views.xml` — `view_project`

**Caractéristiques :**
- Vue principale pour projets
- Multi-édition supportée (`multi_edit="1"`)
- Sample data activé
- Classe JS spécialisée (`js_class="project_project_list"`)
- Ordre par défaut : favoris, séquence, nom

**Colonnes principales :**
- `is_favorite` : Favori (widget `project_is_favorite`, optional)
- `name` : Nom projet (fw-bold)
- `partner_id` : Client (optional, invisible si template)
- `company_id` : Entreprise (optional, groups multi-company)
- `date_start` / `date` : Dates planifiées (widget `daterange`, optional)
- `milestone_progress` : Progression jalons (widget `progressbar`, invisible si 0 ou template)
- `next_milestone_id` : Prochain jalon (optional, decoration-danger/success)
- `user_id` : Chef de projet (widget `many2one_avatar_user`, optional)
- `tag_ids` : Tags (widget `many2many_tags`, optional)
- `last_update_status` : Statut (widget `project_state_selection`, optional, invisible si template)
- `stage_id` : Stage (widget `badge`, optional, couleur depuis `stage_id_color`)

**Décorations visuelles :**
- `decoration-muted` : Projet archivé (`active == False`)

**Filtres principaux :**
- My Projects : Projets dont je suis chef
- My Favorites : Projets favoris
- Unassigned : Projets sans chef
- Late Milestones : Jalons en retard (groups milestone)
- Start Date / End Date : Par dates
- Templates : Projets templates
- Archived : Projets archivés

**Groupements :**
- Par Project Manager (`user_id`)
- Par Stage (`stage_id`, groups stages)
- Par Status (`last_update_status`)
- Par Tags (`tag_ids`)
- Par Company (`company_id`, groups multi-company)

### 1.2 Vue Kanban — `project.project`

**Fichier :** `project_project_views.xml` — `view_project_kanban`

**Caractéristiques :**
- Vue mobile-friendly (`class="o_project_kanban"`)
- Classe JS spécialisée (`js_class="project_project_kanban"`)
- Progressbar par statut (`last_update_status`)
- Quick create activé (`quick_create_view="project.quick_create_project_form"`)
- Sample data activé
- Action par défaut : `action_view_tasks` (ouvrir tâches)

**Structure de la carte :**
- **En-tête :** Favori + Nom projet (fs-4 fw-bold)
- **Corps :**
  - Client (si présent)
  - Dates (date_start → date)
  - Alias email (si présent)
  - Ratings (si activés)
  - Tags
- **Footer :**
  - Compteurs : Tâches ouvertes + Jalons (si activés)
  - Activités (`kanban_activity`)
  - Chef de projet (`many2one_avatar_user`)
  - Statut (`project_state_selection`)

**Menu contextuel :**
- **View :** Tasks, Milestones (si activés)
- **Reporting :** Tasks Analysis, Burndown Chart (groups user)
- **Settings :** Couleur, Share Project, Duplicate, Settings

**Widgets spécialisés :**
- `project_is_favorite` : Toggle favori
- `many2one_avatar_user` : Avatar utilisateur
- `project_state_selection` : Sélection statut projet
- `kanban_activity` : Activités planifiées
- `many2many_tags` : Tags avec couleurs

**Variantes :**
- **Groupé par Stage** (`project_kanban_view_group_stage`) : `default_group_by="stage_id"`

### 1.3 Vue Formulaire (Form View) — `project.project`

**Fichier :** `project_project_views.xml` — `edit_project`

**Caractéristiques :**
- Classe CSS spécialisée (`class="o_form_project_project"`)
- Classe JS spécialisée (`js_class="project_project_form"`)
- Header avec boutons d'action
- Formulaire structuré en groupes et onglets

**Header — Boutons d'action :**
- **Share Project** (`action_open_share_project_wizard`) : Partager projet (hotkey `r`, groups manager, invisible si visibilité followers/employees ou template)
- **Statusbar** (`stage_id`) : Stage projet (widget `statusbar_duration`, groups stages, clickable)

**Button Box (statistiques) :**
- **Tasks** (`action_view_tasks`) : Compteur tâches (fermées / total + pourcentage)
- **Dashboard** (`project_update_all_action`) : Statut dernière mise à jour (widget `status_with_color`, groups user, invisible si template)

**Ribbons :**
- "Archived" : Si projet archivé (bg-danger)
- "Template" : Si projet template (bg-info)

**Structure du formulaire :**

1. **Titre :**
   - Favori (`project_is_favorite`)
   - Nom projet (widget `text`, placeholder)

2. **Groupe principal :**
   - **Colonne gauche :**
     - `label_tasks` : Libellé tâches
     - `partner_id` : Client (widget `res_partner_many2one`)
     - `tag_ids` : Tags
     - `company_id` : Entreprise (groups multi-company)
   - **Colonne droite :**
     - `user_id` : Chef de projet (widget `many2one_avatar_user`, readonly si archivé)
     - `date_start` / `date` : Dates planifiées (widget `daterange`, required si l'un rempli)

3. **Onglets :**
   - **Description :** Description projet (HTML, resizable)
   - **Settings :**
     - **Email Alias :** Configuration alias email (invisible si template)
     - **Privacy Visibility :** Visibilité (widget `radio`)
     - **Analytic Account :** Compte analytique (groups analytic)
     - **Tasks Management :** Settings fonctionnalités (recurring, dependencies, milestones)

**Chatter :** Reload sur changement followers

---

## 2. Vues Principales — Tâches

### 2.1 Vue Liste (List View) — `project.task`

**Fichier :** `project_task_views.xml` — `project_task_view_tree_main_base`, `project_task_view_tree_base`, `view_task_tree2`

**Caractéristiques :**
- Vue principale pour tâches
- Multi-édition supportée (`multi_edit="1"`)
- Classe JS spécialisée (`js_class="project_task_list"`)
- Sample data activé
- Ordre par défaut : priorité desc, séquence, état, échéance asc, id desc
- Groupement par défaut : `stage_id`

**Colonnes principales :**
- `sequence` : Ordre (readonly, column_invisible)
- `id` : ID (optional hide)
- `name` : Titre (widget `name_with_subtask_count`)
- `project_id` : Projet (widget `project`, optional show, readonly, column_invisible si default_project_id)
- `milestone_id` : Jalon (optional hide, invisible si milestones désactivés, groups milestone)
- `partner_id` : Client (optional hide, widget `res_partner_many2one`, invisible si pas projet)
- `user_ids` : Assignés (optional show, widget `many2many_avatar_user`)
- `company_id` : Entreprise (optional hide, groups multi-company)
- `date_deadline` : Échéance (optional hide, widget `remaining_days`, invisible si fermée)
- `priority` : Priorité (widget `priority`, optional show, width 70px)
- `tag_ids` : Tags (widget `many2many_tags`, optional show)
- `create_date` : Date création (optional hide)
- `date_last_stage_update` : Dernière mise à jour stage (optional hide)
- `state` : État (widget `project_task_state_selection`, width 20px, options is_toggle_mode=false)
- `stage_id` : Stage (optional show, widget `badge_rotting`)

**Décorations visuelles :**
- `decoration-danger` : Échéance dépassée (`date_deadline < today` et état pas fermé)

**Filtres principaux :**
- My Tasks : Mes tâches assignées
- Unassigned : Tâches non assignées
- Favorite Projects : Projets favoris
- Blocking : Tâches bloquantes (groups dependencies)
- Open / Closed : Tâches ouvertes/fermées
- Deadline : Par échéance (Future, This Week, Today, Overdue)
- Creation Date : Par date création
- Closed On : Par date fermeture
- Templates : Tâches templates
- Private Tasks : Tâches privées (sans projet)

**Groupements :**
- Par Stage (`stage_id`)
- Par Milestone (`milestone_id`, groups milestone)
- Par Priority (`priority`)
- Par Tags (`tag_ids`)
- Par Customer (`partner_id`)
- Par Company (`company_id`)
- Par Creation Date (`create_date`)
- Par Deadline (`date_deadline`)

**Variantes :**
- **My Tasks** (`open_view_my_tasks_list_view`) : Groupement par `personal_stage_type_id`
- **All Tasks** (`open_view_all_tasks_list_view`) : Sans groupement par défaut

### 2.2 Vue Kanban — `project.task`

**Fichier :** `project_task_views.xml` — `view_task_kanban`

**Caractéristiques :**
- Vue mobile-friendly (`class="o_kanban_small_column o_kanban_project_tasks"`)
- Classe JS spécialisée (`js_class="project_task_kanban"`)
- Progressbar par état (`state`)
- Quick create activé (`quick_create_view="project.quick_create_task_form"`)
- Sample data activé
- Groupement par défaut : `stage_id`
- Ordre par défaut : priorité desc, séquence, état, échéance asc, id desc

**Structure de la carte :**
- **Main :**
  - Titre (fw-bold fs-5)
  - Tâche parente (si présente)
  - Projet (si pas default_project_id)
  - Client (si présent)
  - Jalon (si présent, decoration-danger si en retard)
  - Tags
  - Échéance (widget `remaining_days`, invisible si fermée)
  - Propriétés (`task_properties`)
  - Image couverture (`displayed_image_id`, widget `attachment_image`)
- **Footer :**
  - Activités (`kanban_activity`)
  - Ratings (si activés, icônes smile/meh/frown)
  - Icône lock (si tâche privée)
  - Compteur sous-tâches (`subtask_counter`)
  - Priorité (`priority`)
  - Rotting (`rotting`, widget)
  - Assignés (`many2many_avatar_user`)
  - État (`project_task_state_selection`)

**Menu contextuel :**
- Set Cover Image
- Share Task
- Duplicate
- Couleur

**Widgets spécialisés :**
- `name_with_subtask_count` : Nom avec compteur sous-tâches
- `remaining_days` : Jours restants jusqu'à échéance
- `properties` : Propriétés personnalisées
- `attachment_image` : Image couverture
- `kanban_activity` : Activités planifiées
- `many2many_avatar_user` : Avatars assignés
- `project_task_state_selection` : Sélection état tâche
- `subtask_counter` : Compteur sous-tâches
- `rotting` : Indicateur tâche pourrissante

**Variantes :**
- **My Tasks** (`view_task_kanban_inherit_my_task`) : Groupement par `personal_stage_type_id`
- **All Tasks** (`view_task_kanban_inherit_all_task`) : Groupement par `project_id`
- **Par Jalon** (`project_task_kanban_view_project_milestone`) : Groupement par `milestone_id`

### 2.3 Vue Formulaire (Form View) — `project.task`

**Fichier :** `project_task_views.xml` — `view_task_form2`

**Caractéristiques :**
- Classe CSS spécialisée (`class="o_form_project_tasks"`)
- Classe JS spécialisée (`js_class="project_task_form"`)
- Header avec statusbar (stage ou état)
- Formulaire structuré en groupes et onglets

**Header — Statusbar :**
- `stage_id` : Stage projet (widget `rotting_statusbar_duration`, invisible si pas projet/stage)
- `state` : État technique (widget `statusbar`, invisible, clickable)
- `personal_stage_type_id` : Stage personnel (widget `statusbar`, invisible si projet, domaine user)

**Button Box (statistiques) :**
- **Ratings** (`action_open_ratings`) : Dernier rating (icône smile/meh/frown, invisible si pas ratings ou template)
- **Parent Task** (`action_open_parent_task`) : Tâche parente (invisible si pas parent)
- **Recurring Tasks** (`action_recurring_tasks`) : Tâches récurrence (invisible si pas récurrente, groups recurring)
- **Sub-tasks** (`project_task_action_sub_task`) : Sous-tâches (fermées / total + pourcentage, invisible si pas de sous-tâches)
- **Blocked Tasks** (`action_dependent_tasks`) : Tâches bloquées (invisible si pas dépendances, groups dependencies)

**Ribbons :**
- "Archived" : Si tâche archivée (bg-danger)
- "Template" : Si tâche template (bg-info)

**Structure du formulaire :**

1. **Titre et contrôles :**
   - Titre (widget `text`, placeholder)
   - Priorité (widget `priority_switch`, invisible si template)
   - État (widget `project_task_state_selection`, invisible si template)

2. **Groupe principal :**
   - **Colonne gauche :**
     - `project_id` : Projet (widget `project`, required si parent/enfants/template)
     - `milestone_id` : Jalon (invisible si pas projet ou milestones désactivés)
     - `user_ids` : Assignés (widget `many2many_avatar_user`, options no_open)
     - `role_ids` : Rôles projet (invisible si pas template projet ou template)
   - **Colonne droite :**
     - `tag_ids` : Tags
     - `partner_id` : Client (widget `res_partner_many2one`, invisible si pas projet ou template)
     - `date_deadline` : Échéance (decoration-danger si dépassée)
     - `recurring_task` : Récurrente (widget `boolean_icon`, invisible si pas actif/parent/récurrence désactivée, groups recurring)
     - Paramètres récurrence (si récurrente)
     - `allocated_hours` : Heures allouées (invisible, utilisé par hr_timesheet)

3. **Propriétés :** `task_properties` (2 colonnes)

4. **Onglets :**
   - **Description :** Description (HTML, collaborative, resizable)
   - **Sub-tasks :** Sous-tâches (widget `subtasks_one2many`, mode list/kanban, invisible si pas projet)
   - **Blocked By :** Dépendances (widget `notebook_task_one2many`, invisible si dépendances désactivées, groups dependencies)
   - **Extra Info :** Infos supplémentaires (groups no_one) :
     - Tâche parente
     - Entreprise
     - Séquence
     - Email CC
     - Dates assignation/mise à jour
     - Temps travail (heures/jours)

**Chatter :** Reload sur changement followers

### 2.4 Vue Calendrier — `project.task`

**Fichier :** `project_task_views.xml` — `view_task_calendar`, `view_task_all_calendar`

**Caractéristiques :**
- Date de référence : `date_deadline`
- Mode : month (par défaut)
- Scales : day, week, month, year
- Couleur : `stage_id_color`
- Event limit : 5
- Hide time : true
- Event open popup : true
- Quick create : 0 (désactivé)
- Classe JS spécialisée (`js_class="project_task_calendar"`)

**Champs affichés :**
- `project_id` : Projet (widget `project`, invisible si default_project_id)
- `milestone_id` : Jalon (invisible si milestones désactivés ou pas jalon)
- `user_ids` : Assignés (widget `many2many_avatar_user`)
- `partner_id` : Client
- `priority` : Priorité (widget `priority`, readonly)
- `tag_ids` : Tags
- `stage_id` : Stage (widget `task_stage_with_state_selection`, invisible si pas projet/stage)
- `personal_stage_id` : Stage personnel (invisible si projet ou pas stage personnel)
- `task_properties` : Propriétés

**Variantes :**
- **All Tasks** (`view_task_all_calendar`) : Couleur par `project_id`, filtres sur projet

### 2.5 Vue Graphique — `project.task`

**Fichier :** `project_task_views.xml` — `view_project_task_graph`, `view_project_task_graph_inherit`

**Caractéristiques :**
- Sample data activé
- Classe JS spécialisée (`js_class="project_task_graph"`)

**Champs :**
- `project_id` : Projet (invisible si default_project_id)
- `stage_id` : Stage
- `user_ids` : Assignés (ajouté dans variante)
- `working_hours_open` : Heures travail jusqu'assignation (widget `float_time`)
- `working_hours_close` : Heures travail jusqu'à fermeture (widget `float_time`)
- `rating_last_value` : Rating (/5)

**Variantes :**
- **Par Jalon** (`project_task_graph_view_project_milestone`) : `project_id` remplacé par `milestone_id`

### 2.6 Vue Pivot — `project.task`

**Fichier :** `project_task_views.xml` — `view_project_task_pivot`, `view_project_task_pivot_inherit`

**Caractéristiques :**
- Sample data activé
- Classe JS spécialisée (`js_class="project_task_pivot"`)

**Champs :**
- `project_id` : Projet (type row, invisible si default_project_id)
- `user_ids` : Assignés (type row, ajouté dans variante)
- `stage_id` : Stage (type col, ajouté dans variante)
- `allocated_hours` : Heures allouées (widget `float_time`)
- `working_hours_close` : Heures travail jusqu'à fermeture (widget `float_time`)
- `working_hours_open` : Heures travail jusqu'assignation (widget `float_time`)

**Variantes :**
- **Par Jalon** (`project_task_pivot_view_project_milestone`) : `project_id` remplacé par `milestone_id`, `stage_id` en colonne

### 2.7 Vue Activity — `project.task`

**Fichier :** `project_task_views.xml` — `project_task_view_activity`

**Caractéristiques :**
- Classe JS spécialisée (`js_class="project_task_activity"`)

**Template :**
- Assignés (`user_ids`)
- Projet (`project_id`)
- Nom tâche (`name`, display full)

---

## 3. Widgets Spécialisés

### 3.1 Widgets Projet

**`project_is_favorite` :**
- Toggle favori projet
- Options : `autosave=False` (sauvegarde manuelle)

**`project` :**
- Sélection projet avec icône
- Options : `no_open=True` (pas d'ouverture)

**`project_state_selection` :**
- Sélection statut projet (On Track, At Risk, Off Track, On Hold, Done)
- Couleur selon statut

**`statusbar_duration` :**
- Statusbar avec durée dans stage
- Options : `clickable=True`, `fold_field='fold'`

### 3.2 Widgets Tâche

**`name_with_subtask_count` :**
- Nom tâche avec compteur sous-tâches intégré

**`project_task_state_selection` :**
- Sélection état tâche (In Progress, Changes Requested, Approved, Done, Cancelled, Waiting)
- Options : `is_toggle_mode=false` (pas de toggle direct)

**`rotting_statusbar_duration` :**
- Statusbar avec durée et indicateur pourrissement

**`subtasks_one2many` :**
- One2many spécialisé pour sous-tâches
- Mode list/kanban
- Contexte avec valeurs par défaut

**`notebook_task_one2many` :**
- One2many spécialisé pour dépendances
- Mode list/kanban
- Contexte avec recherche

**`badge_rotting` :**
- Badge stage avec indicateur pourrissement

**`task_stage_with_state_selection` :**
- Stage avec sélection état intégrée

---

## 4. Patterns de Navigation

### 4.1 Navigation Projets

**Flux principal :**
1. Menu "Project" → "Projects"
2. Vue Kanban/Liste projets
3. Clic projet → Formulaire projet
4. Bouton "Tasks" → Vue tâches projet
5. Clic tâche → Formulaire tâche

**Actions rapides :**
- Quick create projet depuis Kanban
- Quick create tâche depuis Kanban projet
- Drag & drop tâches entre stages

### 4.2 Navigation Tâches

**Flux principaux :**
1. **Mes Tâches :**
   - Menu "Project" → "My Tasks"
   - Vue Kanban par stages personnels
   - Drag & drop entre stages personnels

2. **Toutes Tâches :**
   - Menu "Project" → "All Tasks"
   - Vue Liste/Kanban toutes tâches
   - Filtres et groupements

3. **Tâches Projet :**
   - Depuis projet → Bouton "Tasks"
   - Vue tâches filtrées par projet

**Actions rapides :**
- Quick create depuis Kanban
- Multi-édition depuis Liste
- Assignation multiple

---

## 5. Design Responsive et Accessibilité

### 5.1 Responsive Design

**Caractéristiques :**
- Classes Bootstrap utilisées (`d-flex`, `w-100`, `text-truncate`)
- Vue Kanban mobile-friendly (`class="o_kanban_mobile"`)
- Colonnes optionnelles (`optional="show"/"hide"`)
- Widgets adaptatifs (avatars, tags)

### 5.2 Accessibilité

**Caractéristiques :**
- Labels ARIA (`aria-label`, `aria-haspopup`)
- Rôles (`role="menuitem"`, `role="button"`)
- Hotkeys supportés (`data-hotkey`)
- Tooltips (`title`)

---

## 6. Recommandations pour Miyukini

### 6.1 Simplification Interface

**Recommandations :**
- Unifier états/stages pour réduire confusion
- Interface unifiée configuration projet
- Création rapide enrichie (assignation, échéance)
- Widgets réutilisables entre projets et tâches

### 6.2 Amélioration UX

**Recommandations :**
- Drag & drop amélioré (multi-sélection)
- Prévisualisation avant action
- Undo/Redo pour actions importantes
- Notifications contextuelles

### 6.3 Visualisation Données

**Recommandations :**
- Dashboard projet avec KPIs clairs
- Vue graphe dépendances
- Timeline projet avec jalons
- Rapports visuels améliorés

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
