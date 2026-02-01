# Odoo CRM — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **CRM** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les composants d'interface, patterns de navigation, formulaires, tableaux, rapports et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/crm/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Form, Calendar, Graph, Pivot, Activity)
- Composants d'interface spécialisés
- Patterns de navigation
- Formulaires et validations
- Tableaux et listes
- Rapports et exports
- Design responsive et accessibilité

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Vue Kanban — `crm.lead` (Opportunities)

**Fichier :** `crm_lead_views.xml` — `crm_case_kanban_view_leads`

**Caractéristiques :**
- Vue Kanban principale pour opportunités
- Groupement par défaut : `stage_id` (étapes du pipeline)
- Classe JS spécialisée (`js_class="crm_kanban"`)
- Quick create activé (`on_create="quick_create"`)
- Progressbar par activité (`activity_state`)
- Sample data activé

**Structure de la carte :**
- **Ribbon** : "Lost" (rouge) ou "Archived" (rouge) si applicable
- **Nom** : Titre de l'opportunité (fw-bold fs-5)
- **Revenu** : `expected_revenue` + `recurring_revenue` (si activé)
- **Partenaire** : Avatar + nom (si `partner_id` existe)
- **Contact** : `contact_name` (si pas de partenaire)
- **Company** : `partner_name` (si pas de contact ni partenaire)
- **Tags** : `tag_ids` avec couleurs
- **Properties** : `lead_properties` (propriétés dynamiques)
- **Footer** :
  - Priorité (`priority` widget)
  - Activités (`activity_ids` widget `kanban_activity`)
  - Jours de pourrissement (`rotting_days` widget `rotting`)
  - Utilisateur assigné (`user_id` widget `many2one_avatar_user`)

**Progressbar :**
- Champ : `activity_state`
- Couleurs : `planned` (success), `today` (warning), `overdue` (danger)
- Somme : `expected_revenue` (ou `recurring_revenue_monthly` si activé)

**Quick Create :**
- Vue spécialisée : `quick_create_opportunity_form`
- Champs essentiels : Company, Contact, Opportunity Name, Email, Phone, Expected Revenue, Priority
- Création rapide depuis Kanban

**Variantes :**
- **Forecast Kanban** (`crm_lead_view_kanban_forecast`) : Groupement par `date_deadline` au lieu de `stage_id`
- **Lead Kanban** (`view_crm_lead_kanban`) : Vue simplifiée pour leads (sans revenu)

### 1.2 Vue Liste (List View) — `crm.lead`

**Fichier :** `crm_lead_views.xml` — `crm_case_tree_view_oppor` (Opportunities) / `crm_case_tree_view_leads` (Leads)

**Caractéristiques :**
- Vue principale pour opportunités et leads
- Multi-édition supportée (`multi_edit="1"`)
- Sample data activé
- Décoration pour opportunités perdues (`decoration-muted="won_status == 'lost'"`)

**Colonnes principales (Opportunities) :**
- `name` : Nom opportunité (readonly)
- `partner_id` : Partenaire (optional)
- `contact_name` : Contact (optional)
- `email_from` : Email
- `phone` : Téléphone (optional, class `o_force_ltr`)
- `user_id` : Commercial (widget `many2one_avatar_user`, optional)
- `team_id` : Équipe (optional)
- `priority` : Priorité (widget `priority`, optional)
- `activity_ids` : Activités (widget `list_activity`, optional)
- `activity_user_id` : Activité par (optional)
- `my_activity_date_deadline` : Ma deadline (widget `remaining_days`, optional)
- `expected_revenue` : Revenu attendu (sum="Expected Revenues", widget `monetary`, optional)
- `recurring_revenue_monthly` : MRR attendu (sum="Expected MRR", optional, si activé)
- `date_deadline` : Date de clôture attendue (optional)
- `stage_id` : Étape (widget `badge_rotting`, optional)
- `probability` : Probabilité (%) (optional)
- `tag_ids` : Tags (widget `many2many_tags`, optional)
- `lead_properties` : Propriétés dynamiques

**Colonnes principales (Leads) :**
- `name` : Nom lead (readonly)
- `contact_name` : Contact (optional)
- `partner_name` : Entreprise (optional)
- `email_from` : Email (optional)
- `phone` : Téléphone (optional)
- `user_id` : Commercial (optional)
- `team_id` : Équipe (optional)
- `city` : Ville (optional)
- `country_id` : Pays (optional)
- `campaign_id` : Campagne (optional)
- `source_id` : Source (optional)
- `probability` : Probabilité (%) (optional)
- `tag_ids` : Tags (optional)
- `priority` : Priorité (optional)

**Variantes :**
- **Forecast List** (`crm_lead_view_tree_forecast`) : Colonne `prorated_revenue` au lieu de `expected_revenue`
- **Activities List** (`crm_lead_view_list_activities`) : Tri par défaut `my_activity_date_deadline`

### 1.3 Vue Formulaire (Form View) — `crm.lead`

**Fichier :** `crm_lead_views.xml` — `crm_lead_view_form`

**Caractéristiques :**
- Classe JS spécialisée (`js_class="crm_form"`)
- Header avec boutons d'action contextuels
- Statusbar avec étapes (`rotting_statusbar_duration`)
- Ribbons de statut (Archived, Lost, Won)
- Formulaire structuré avec onglets

**Header — Boutons d'action :**
- **Won** (`action_set_won_rainbowman`) : Marquer comme gagné (hotkey `w`)
- **Convert to Opportunity** (`crm.action_crm_lead2opportunity_partner`) : Convertir en opportunité (hotkey `v`)
- **Restore** (`action_restore`) : Restaurer (hotkey `x`)
- **Lost** (`crm.crm_lead_lost_action`) : Marquer comme perdu (hotkey `l`)

**Statusbar :**
- Widget : `rotting_statusbar_duration`
- Options : `clickable='1'`, `fold_field='fold'`
- Domain : Étapes selon équipe (`team_id`)
- Readonly si `won_status == 'lost'` ou `not active`

**Ribbons de statut :**
- **Archived** : Rouge (`active=False` et `won_status` pas lost/won)
- **Lost** : Rouge (`won_status == 'lost'`)
- **Won** : Vert (`won_status == 'won'`)

**Button Box (statistiques) :**
- **Meetings** (`action_schedule_meeting`) : Nombre de réunions planifiées
- **Similar Leads** (`action_show_potential_duplicates`) : Nombre de doublons potentiels

**Structure du formulaire :**

1. **En-tête** :
   - **Revenu attendu** : `expected_revenue` + `recurring_revenue` + `recurring_plan` (si activé)
   - **Probabilité** : `probability` avec switch AI (`action_set_automated_probability`)
   - Widget PLS tooltip pour probabilité automatisée

2. **Groupe Lead (si type='lead')** :
   - Partenaire (`partner_id` avec widget `res_partner_many2one`)
   - Nom partenaire (`partner_name`)
   - Adresse complète (street, street2, city, state_id, zip, country_id)
   - Site web (`website`)
   - Langue (`lang_id`)

3. **Groupe Opportunity (si type='opportunity')** :
   - Partenaire (`partner_id` avec widget `res_partner_many2one`)
   - Email (`email_from` avec widget `email`, gestion blacklist)
   - Téléphone (`phone` avec widget `phone`, gestion blacklist)
   - Raison de perte (`lost_reason_id` si perdu)

4. **Groupe Lead Info (si type='lead')** :
   - Nom contact (`contact_name`)
   - Email (`email_from` avec gestion blacklist)
   - Fonction (`function`)
   - Téléphone (`phone` avec gestion blacklist)

5. **Groupe Opportunity/Lead commun** :
   - Commercial (`user_id` avec widget `many2one_avatar_leader_user`)
   - Date de clôture attendue (`date_deadline`)
   - Priorité (`priority` widget `priority`)
   - Tags (`tag_ids` widget `many2many_tags`)
   - Équipe (`team_id` si lead)

6. **Propriétés dynamiques** :
   - `lead_properties` : Propriétés personnalisables (2 colonnes)

7. **Onglet Notes** :
   - Description (`description` avec options `collaborative: true`)

8. **Onglet Extra Info (si type='lead')** :
   - Email (bounce) : `message_bounce`
   - Marketing : `campaign_id`, `medium_id`, `source_id`, `referred`
   - Analysis : `date_open`, `date_closed`
   - Company : `company_id` (si multi-company)

9. **Onglet Contacts (si type='opportunity')** :
   - Informations entreprise
   - Informations contact
   - Marketing
   - Ownership

**Widgets spécialisés :**
- `res_partner_many2one` : Sélection partenaire avec recherche avancée
- `many2one_avatar_leader_user` : Sélection utilisateur avec avatar et leader
- `many2one_avatar_user` : Sélection utilisateur avec avatar
- `many2many_tags` : Tags avec couleurs (`color_field: 'color'`)
- `priority` : Widget priorité (étoiles)
- `rotting_statusbar_duration` : Statusbar avec durée de pourrissement
- `rotting` : Widget jours de pourrissement
- `kanban_activity` : Widget activités Kanban
- `list_activity` : Widget activités Liste
- `remaining_days` : Widget jours restants
- `badge_rotting` : Badge étape avec indicateur pourrissement
- `pls_tooltip_button` : Bouton tooltip PLS (probabilité automatisée)
- `properties` : Widget propriétés dynamiques

### 1.4 Vue Calendar — `crm.lead`

**Fichier :** `crm_lead_views.xml` — `crm_case_calendar_view_leads`

**Caractéristiques :**
- Vue calendrier pour leads/opportunités
- Mode : `month`
- Date de début : `activity_date_deadline`
- Couleur : `user_id`
- Masquage heure : `hide_time="true"`
- Limite événements : `event_limit="5"`
- Création désactivée : `create="0"`

**Champs affichés :**
- `expected_revenue` : Revenu attendu
- `partner_id` : Partenaire (avatar `avatar_128`)
- `user_id` : Utilisateur (filtre)
- `team_id` : Équipe (filtre)
- `lead_properties` : Propriétés

### 1.5 Vue Graph — `crm.lead`

**Fichier :** `crm_lead_views.xml` — `crm_lead_view_graph` / `crm_lead_view_graph_forecast`

**Caractéristiques :**
- Graphique par étape (`stage_id`) ou utilisateur (`user_id`)
- Mesure : `expected_revenue` (ou `prorated_revenue` pour forecast)
- Sample data activé

**Variantes :**
- **Standard** : Par étape ou utilisateur
- **Forecast** (`crm_lead_view_graph_forecast`) : Par `date_deadline`, mesure `prorated_revenue`, classe JS `forecast_graph`

### 1.6 Vue Pivot — `crm.lead`

**Fichier :** `crm_lead_views.xml` — `crm_lead_view_pivot` / `crm_lead_view_pivot_forecast`

**Caractéristiques :**
- Analyse par date de création (`create_date`) et étape (`stage_id`)
- Mesure : `expected_revenue` (ou `prorated_revenue` pour forecast)
- Sample data activé

**Variantes :**
- **Standard** : Par mois de création et étape
- **Forecast** (`crm_lead_view_pivot_forecast`) : Par `date_deadline` (mois) et étape, mesure `prorated_revenue`, classe JS `forecast_pivot`

### 1.7 Vue Activity — `crm.lead`

**Fichier :** `crm_lead_views.xml` — `crm_lead_view_activity`

**Caractéristiques :**
- Vue activité pour leads/opportunités
- Groupement par utilisateur (`user_id`)
- Affichage : Nom, Partenaire, Revenu attendu, Étape

**Structure :**
- Box activité avec avatar utilisateur
- Nom opportunité (bold)
- Revenu attendu (muted)
- Partenaire (muted)
- Badge étape avec couleur

---

## 2. Composants d'Interface Spécialisés

### 2.1 Widget `crm_kanban`

**Usage :** Vue Kanban spécialisée pour CRM

**Caractéristiques :**
- Gestion quick create
- Progressbar par activité
- Calcul revenus (standard + récurrent)
- Gestion pourrissement

### 2.2 Widget `crm_form`

**Usage :** Formulaire CRM avec logique métier

**Caractéristiques :**
- Gestion conversion Lead → Opportunity
- Gestion probabilité automatisée
- Gestion blacklist email/téléphone
- Synchronisation partenaire

### 2.3 Widget `rotting_statusbar_duration`

**Usage :** Statusbar avec durée de pourrissement

**Caractéristiques :**
- Affichage durée dans chaque étape
- Calcul automatique jours de pourrissement
- Indicateur visuel si pourrissant

### 2.4 Widget `rotting`

**Usage :** Affichage jours de pourrissement

**Caractéristiques :**
- Calcul automatique depuis `rotting_days`
- Affichage visuel (badge, couleur)
- Indicateur si opportunité pourrissante

### 2.5 Widget `badge_rotting`

**Usage :** Badge étape avec indicateur pourrissement

**Caractéristiques :**
- Badge étape standard
- Indicateur visuel si pourrissante
- Couleur selon état

### 2.6 Widget `pls_tooltip_button`

**Usage :** Bouton tooltip pour probabilité automatisée (PLS)

**Caractéristiques :**
- Affichage probabilité calculée par IA
- Tooltip explicatif
- Switch manuel/automatique

### 2.7 Widget `forecast_kanban` / `forecast_list` / `forecast_graph` / `forecast_pivot`

**Usage :** Vues Forecast spécialisées

**Caractéristiques :**
- Groupement par `date_deadline` au lieu de `stage_id`
- Calcul revenus proratés (`prorated_revenue`)
- Filtre "Upcoming Closings"
- Visualisation temporelle

---

## 3. Patterns de Navigation

### 3.1 Navigation Principale

**Menu CRM :**
- Pipeline (Opportunities)
- Leads
- My Activities
- Forecast
- Teams
- Configuration

### 3.2 Navigation Contextuelle

**Depuis une opportunité :**
- Lien vers Partenaire
- Lien vers Commercial
- Lien vers Équipe
- Lien vers Sales Order (si convertie)
- Lien vers Meetings
- Lien vers Similar Leads

**Depuis un lead :**
- Conversion en Opportunity
- Lien vers Partenaire (si créé)
- Lien vers Commercial
- Lien vers Équipe

### 3.3 Actions Rapides

**Hotkeys :**
- `w` : Mark as Won
- `v` : Convert to Opportunity
- `x` : Restore
- `l` : Mark as Lost

**Actions depuis Kanban :**
- Quick create opportunité
- Drag & drop entre étapes
- Menu contextuel (Edit, Delete, Color)

---

## 4. Formulaires et Validations

### 4.1 Validation de Formulaire

**Validations automatiques :**
- Email valide (format)
- Téléphone valide (format)
- Date de clôture cohérente
- Probabilité entre 0 et 100
- Revenu récurrent nécessite plan si montant > 0

**Messages d'erreur :**
- Email invalide
- Téléphone invalide
- Date de clôture passée (avertissement)
- Probabilité hors limites

### 4.2 Champs Conditionnels

**Visibilité conditionnelle :**
- `invisible="type == 'lead'"` : Champs opportunité uniquement
- `invisible="type == 'opportunity'"` : Champs lead uniquement
- `invisible="won_status != 'lost'"` : Raison de perte si perdu
- `invisible="not partner_id"` : Champs partenaire si partenaire existe
- `invisible="groups='...'"` : Champs selon groupes utilisateurs

**Readonly conditionnel :**
- `readonly="won_status == 'lost' or not active"` : Statusbar si perdu/archivé
- `readonly="won_status != 'pending'"` : Probabilité si gagné/perdu

### 4.3 Auto-complétion

**Champs avec auto-complétion :**
- `partner_id` : Recherche partenaire avec contexte (customer, company, etc.)
- `contact_name` : Suggestion depuis partenaire
- `email_from` : Suggestion depuis partenaire
- `phone` : Suggestion depuis partenaire
- `team_id` : Suggestion depuis utilisateur

### 4.4 Gestion Blacklist

**Email/Téléphone blacklistés :**
- Badge visuel (icône ban rouge)
- Bouton unblacklist (`mail_action_blacklist_remove` / `phone_action_blacklist_remove`)
- Avertissement si modification synchronise partenaire

---

## 5. Tableaux et Listes

### 5.1 Colonnes Configurables

**Système optional :**
- `optional="show"` : Visible par défaut
- `optional="hide"` : Masqué par défaut

**Colonnes principales (Opportunities) :**
- Name, Partner, Contact, Email, Phone, User, Team, Priority, Activities, Expected Revenue, Date Deadline, Stage, Probability, Tags

### 5.2 Multi-édition

**Support multi-édition :**
- `multi_edit="1"` activé sur liste
- Sélection multiple avec cases à cocher
- Édition groupée des champs compatibles
- Actions groupées (Mark Lost, Email)

### 5.3 Groupement

**Groupements disponibles :**
- Par Salesperson (`user_id`)
- Par Sales Team (`team_id`)
- Par Stage (`stage_id`)
- Par City (`city`)
- Par Country (`country_id`)
- Par Company (`company_id`)
- Par Campaign (`campaign_id`)
- Par Medium (`medium_id`)
- Par Source (`source_id`)
- Par Creation Date (jour, mois)
- Par Conversion Date (`date_conversion`)
- Par Expected Closing (`date_deadline`)
- Par Closed Date (`date_closed`)
- Par Properties (`lead_properties`)

### 5.4 Filtres Avancés

**Filtres prédéfinis :**
- My Pipeline : Opportunités assignées à moi
- Unassigned : Opportunités non assignées
- Open Opportunities : Opportunités ouvertes
- Won : Opportunités gagnées
- Ongoing : Opportunités en cours
- Rotting : Opportunités pourrissantes
- Lost : Opportunités perdues
- Unread Messages : Messages non lus
- Creation Date : Par date de création
- Closed Date : Par date de clôture
- Late Activities : Activités en retard
- Today Activities : Activités aujourd'hui
- Future Activities : Activités futures

**Filtres personnalisés :**
- Recherche texte (nom, partenaire, email, contact)
- Filtres de domaine complexes
- Filtres de date avec périodes prédéfinies

---

## 6. Rapports et Exports

### 6.1 Rapports Graphiques

**Rapports disponibles :**
- Graphique par étape
- Graphique par utilisateur
- Graphique Forecast (par date de clôture)
- Pivot par création/étape
- Pivot Forecast (par date de clôture/étape)

**Métriques :**
- Revenu attendu (`expected_revenue`)
- Revenu proraté (`prorated_revenue`)
- MRR attendu (`recurring_revenue_monthly`)
- Nombre d'opportunités
- Probabilité moyenne

### 6.2 Exports

**Formats d'export :**
- CSV (depuis liste)
- Excel (depuis liste)

**Données exportables :**
- Toutes les colonnes visibles
- Filtres appliqués
- Groupements préservés

---

## 7. Design Responsive et Accessibilité

### 7.1 Responsive Design

**Vue mobile :**
- Kanban mobile-friendly (`o_kanban_mobile`)
- Formulaires adaptatifs
- Navigation simplifiée
- Touch-friendly buttons

**Breakpoints :**
- Desktop : Vue complète
- Tablet : Colonnes adaptatives
- Mobile : Vue Kanban prioritaire

### 7.2 Accessibilité

**Hotkeys :**
- Navigation clavier complète
- Raccourcis clavier pour actions principales
- Focus management

**ARIA :**
- Labels appropriés
- Roles sémantiques
- États annoncés

**Contraste :**
- Couleurs de décoration cohérentes
- Badges avec contrastes suffisants
- Alertes visuelles claires

---

## 8. Recommandations pour Miyukini

### 8.1 Composants à Implémenter

**Vues principales :**
1. **Kanban Pipeline** : Vue principale avec étapes
2. **Liste Opportunités** : Vue liste avec colonnes configurables
3. **Formulaire Lead/Opportunity** : Formulaire structuré avec onglets
4. **Vue Forecast** : Visualisation temporelle par date de clôture
5. **Vue Calendar** : Calendrier par activités
6. **Vue Activity** : Vue activité par utilisateur
7. **Graphiques** : Analyses par étape/utilisateur
8. **Pivot** : Analyses multidimensionnelles

**Widgets spécialisés :**
1. **CRM Kanban Widget** : Vue Kanban avec quick create
2. **Rotting Widget** : Indicateur pourrissement
3. **Statusbar Rotting** : Statusbar avec durée
4. **PLS Tooltip Widget** : Probabilité automatisée
5. **Forecast Widgets** : Vues Forecast spécialisées
6. **Properties Widget** : Propriétés dynamiques

### 8.2 Patterns à Adopter

**Navigation :**
- Menu hiérarchique clair (Pipeline, Leads, Activities, Forecast)
- Breadcrumbs contextuels
- Actions rapides accessibles
- Hotkeys pour actions fréquentes

**Formulaires :**
- Validation en temps réel
- Messages d'erreur clairs
- Auto-complétion intelligente
- Champs conditionnels selon type
- Gestion blacklist intégrée

**Kanban :**
- Quick create depuis Kanban
- Drag & drop entre étapes
- Progressbar par activité
- Calcul revenus automatique
- Indicateur pourrissement

**Tableaux :**
- Colonnes configurables
- Multi-édition
- Groupements flexibles
- Filtres avancés
- Actions groupées

**Responsive :**
- Design mobile-first
- Vue Kanban pour mobile
- Navigation adaptative
- Touch-friendly

### 8.3 Améliorations Possibles

**UX :**
- Assistant de conversion Lead → Opportunity
- Suggestions intelligentes (doublons, partenaires similaires)
- Workflow guidé pour création opportunité
- Prévisualisation avant conversion

**Performance :**
- Lazy loading des opportunités
- Pagination intelligente
- Cache des filtres fréquents
- Optimisation requêtes

**Accessibilité :**
- Support lecteurs d'écran complet
- Navigation clavier exhaustive
- Contraste amélioré
- Textes alternatifs complets

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
