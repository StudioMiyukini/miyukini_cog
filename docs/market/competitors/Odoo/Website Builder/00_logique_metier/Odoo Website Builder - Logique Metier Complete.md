# Odoo Website Builder — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Website Builder** (Site web) d'Odoo (version 19.0). Il identifie les modèles de données, règles métier, workflows, mécanismes de pages et de menus pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module `website`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (website.page, website.menu, website, ir.ui.view)
- Pages statiques vs dynamiques
- Gestion des menus et navigation
- Building blocks et éditeur visuel
- Publication / dépublication, visibilité, SEO
- Redirections URL
- Thèmes et personnalisation
- Intégration avec Blog, eCommerce, CRM, etc.

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `website` (Site web)

**Rôle :** Représente une **instance de site web** — conteneur multi-site (multi-website) avec domaine, nom, configuration.

**Concepts clés :**
- Multi-website : plusieurs sites possibles par base Odoo
- Chaque site a son domaine, sa homepage, ses pages, ses menus
- Configuration globale (SEO, tracking, cookies, etc.)

**Champs typiques (logique métier) :**
- `name` : Nom du site
- `domain` : Domaine associé
- `homepage_id` : Page d'accueil (website.page)
- Configuration SEO, analytics, favicon, etc.

---

### 1.2 Modèle `website.page` (Page)

**Rôle :** Représente une **page web** — contenu fixe (statique) ou référence vers contenu dynamique.

**Types de pages :**
- **Statiques** : Contenu fixe, créées manuellement (frontend ou backend), URL et propriétés configurables
- **Dynamiques** : Générées automatiquement (ex. /shop, /blog), gérées par les modules (eCommerce, Blog, etc.)

**Propriétés de page (gestion) :**
- **Page URL** : Modification d’URL avec option « Redirect old URL » (301, 302)
- **In Menu** : Affichage ou non dans le menu
- **Is Homepage** : Définir comme page d'accueil du site
- **Published** : Publiée ou non (visible visiteurs)
- **Publishing Date** : Date/heure de publication planifiée
- **Indexed** : Indexation moteurs de recherche (SEO)
- **Visibility** : Public / Signed In / Restricted Group / With Password
- **Is a template** : Sauvegarde comme bloc personnalisé (catégorie Custom)

**Règles métier :**
- Une page doit être publiée pour être visible aux visiteurs
- Duplication de page : nouveau nom, position dans le menu configurable
- Suppression : vérification des liens référents, possibilité de redirection

---

### 1.3 Modèle `website.menu` (Menu)

**Rôle :** Structure la **navigation** du site (header, footer, menus personnalisés).

**Concepts clés :**
- Hiérarchie via `parent_id` (ex. `website.main_menu`)
- `sequence` : ordre d'affichage
- Lien vers page (website.page) ou URL externe
- Un menu peut être lié à une page, une URL, ou être un conteneur (sous-menus)

**Règles métier :**
- Les items de menu sont ordonnés par séquence
- Création possible depuis backend (XML) ou frontend (éditeur)
- Header et footer : structure dédiée (documentation Odoo Structure)

---

### 1.4 Vues et QWeb (`ir.ui.view`)

**Rôle :** Stockage des **templates QWeb** utilisés pour le rendu des pages et blocs.

**Concepts clés :**
- Héritage de vues (xpath) pour personnaliser sans modifier le noyau
- Layout commun : héritage de `website.layout` (header, footer, structure)
- Type `qweb` pour le frontend website
- Édition possible depuis l’éditeur de site (personnalisation thème)

**Règles métier :**
- La priorité (priority) et l’héritage déterminent la vue finale
- Pas de logique métier lourde dans les vues ; données fournies par contrôleurs/modèles

---

### 1.5 Building Blocks (Blocs de construction)

**Rôle :** Unités réutilisables de **contenu** pour l’éditeur drag-and-drop.

**Catégories de blocs (exemples) :**
- **Basic** : Page vierge, multi-usage
- **About** : À propos, marque
- **Landing Pages** : Résumé contenu / offre
- **Gallery** : Médias, photos
- **Services** : Offres et contact
- **Pricing Plans** : Abonnements, tarifs
- **Team** : Équipe
- **Contact & Forms** : Formulaires (contact, CRM, recrutement, etc.)
- **Custom** : Blocs enregistrés par l’utilisateur

**Inner Content :**
- Blocs « contenu » (vidéo, image, boutons sociaux, etc.) insérables **dans** les blocs de catégorie
- Hiérarchie : Catégorie → Inner Content

**Règles métier :**
- Formulaires : action configurable (email, créer client, ticket, opportunité CRM, candidature, newsletter, tâche projet)
- Champs de formulaire : type, label, position, requis, placeholder, visibilité, animation
- Bloc Embed : code tiers (YouTube, Google Maps, etc.) — risque sécurité si code non maîtrisé
- Layout : Grid (repositionnement, redimensionnement) ou Cols (colonnes par ligne)
- Blocs personnalisés : enregistrement et réutilisation
- Ancres : liens vers une section précise d’une page

---

### 1.6 Redirections URL

**Rôle :** Gérer les **redirections** (ancienne URL → nouvelle URL ou page d’erreur).

**Types d’action :**
- **404 Not found** : page supprimée ou non publiée
- **301 Moved Permanently** : redirection permanente (page statique)
- **302 Moved Temporarily** : redirection temporaire
- **308 Redirect/Rewrite** : renommage permanent de page dynamique (ex. /shop → /market)

**Règles métier :**
- 301/302 : migration de trafic depuis pages supprimées ou dépubliées
- 308 : renommage d’URL de pages dynamiques existantes
- Séquence pour ordonner les chaînes de redirection
- Champ « URL from » / « URL to », site (multi-website), actif/inactif

---

## 2. Workflows et États

### 2.1 Cycle de vie d’une page

1. **Création** : Frontend (+ New → Page) ou Backend (Website ‣ Site ‣ Pages ‣ New)
2. **Choix du template** : Basic, About, Landing, Gallery, Services, Pricing, Team, Custom
3. **Saisie du titre** : Utilisé pour menu et URL
4. **Édition** : Éditeur visuel (blocs, texte, couleurs, mise en page)
5. **Propriétés** : Site ‣ Properties (URL, menu, homepage, publication, visibilité, SEO, template)
6. **Publication** : Bascule Published / Unpublished (ou date planifiée)
7. **Duplication / Suppression** : Via propriétés, avec gestion des liens et redirections

### 2.2 Publication

- **Unpublished** : invisible aux visiteurs (sauf si accès restreint avec mot de passe / groupe)
- **Published** : visible selon les règles de visibilité (Public, Signed In, Restricted Group, With Password)
- Publication en masse : Website ‣ Site ‣ Pages → Action ‣ Publish / Unpublish
- Homepage : une seule page peut être « Use as Homepage » (Website ‣ Site ‣ Properties ‣ Publish)

### 2.3 Visibilité

- **Public** : tout le monde
- **Signed In** : utilisateurs connectés uniquement
- **Restricted Group** : groupes d’accès sélectionnés
- **With Password** : mot de passe saisi pour accéder à la page

---

## 3. Thèmes et Personnalisation

**Rôle :** Thèmes = personnalisation complète sans toucher aux fichiers cœur du module website.

**Concepts (documentation Odoo) :**
- Modules de thème, options par défaut, assets
- Layout : header, footer, arrière-plans, responsive
- Navigation : personnalisation des menus
- Les options d’édition du Website Builder restent disponibles (building blocks, etc.)

---

## 4. Intégration Données et Formulaires

### 4.1 Formulaires

- Soumission → action configurable :
  - Envoi email (défaut)
  - Créer un client (eCommerce)
  - Créer un ticket (Helpdesk)
  - Créer une opportunité (CRM)
  - Candidature (Recruitment)
  - Newsletter (Email Marketing)
  - Créer une tâche (Project)
- Champs : types standards + champs existants du modèle cible (selon l’action)
- « On Success » : redirection vers une URL, rien, ou message

### 4.2 Formulaire Odoo sur site externe

- Page Odoo contenant uniquement le formulaire (sans header/footer)
- URL mise dans un iframe sur un site non-Odoo
- Pas de logique métier supplémentaire : même modèle de formulaire que sur le site Odoo

---

## 5. Résumé des Règles Métier Clés

| Domaine            | Règle |
|--------------------|--------|
| Pages              | Visibilité visiteurs = Published + respect Visibility |
| Homepage           | Une seule page « Use as Homepage » par site |
| Menus              | Hiérarchie parent_id + sequence |
| Building blocks    | Catégories contiennent Inner Content ; formulaires liés à des actions métier |
| Redirections       | 301/302 pour pages supprimées/dépubliées ; 308 pour renommage pages dynamiques |
| SEO                | Indexed = oui/non par page |
| Duplication page   | Nouveau titre, liens à mettre à jour ou redirection |
| Suppression page   | Vérification des liens, redirection recommandée |

---

## 6. Correspondance Miyukini (orientations)

- **MiyuWeb** / **MiyukiniWeb** : Opérateur(s) pour site vitrine, pages, menus, building blocks
- **KindMother** : Persistance des pages, menus, options (WriteIntent)
- **StrongFather** : Décision publication, visibilité, suppression
- **Master Butler** : Permissions édition / publication / configuration
- **Ever Buddy** : Cycle de vie (brouillon → publié → déprécié / archivé)
- Formulaires : intégration avec MiyuContacts, MiyuForum/Helpdesk, MiyuCRM, etc., via Mandats et Contrats d’équipe

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
