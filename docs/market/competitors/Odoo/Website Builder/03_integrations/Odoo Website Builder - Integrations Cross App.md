# Odoo Website Builder — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Website Builder** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0, module Website

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps (formulaires, blocs, pages dynamiques)
- Mécanismes d'intégration (controllers, vues QWeb, modèles)
- APIs et hooks typiques
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (typiques du module `website`)

**Dépendances explicites (structure classique Odoo) :**
- **base** : Modèles de base, res.company, res.config.settings
- **web** : Framework web, assets, frontend
- **mail** (souvent) : Chatter, notifications (si pages ou contenu discutables)
- **portal** (souvent) : Utilisateurs portail (visiteurs connectés)
- **auth_signup** (optionnel) : Inscription depuis le site
- **website** (cœur) : Modèles website, website.page, website.menu, controllers, QWeb

### 1.2 Modules optionnels (intégrations si installés)

**Apps qui étendent le Website Builder :**
- **website_blog** : Pages dynamiques /blog, blocs blog
- **website_sale** (eCommerce) : Pages /shop, blocs produits, panier, formulaire « Create a Customer »
- **website_slides** (eLearning) : Contenu slides
- **website_helpdesk** : Formulaire « Create a Ticket »
- **website_crm** : Formulaire « Create an Opportunity »
- **hr_recruitment** : Formulaire « Apply for a Job »
- **mass_mailing** (Email Marketing) : Formulaire « Subscribe to Newsletter »
- **project** : Formulaire « Create a Task »
- **website_live_chat** : Widget live chat
- **website_event** : Pages événements
- **website_survey** : Enquêtes intégrées au site

---

## 2. Flux de Données Inter-Apps

### 2.1 Formulaires → Création d’enregistrements

**Flux générique :**
```
Visiteur (frontend) → Formulaire Website → Controller website
  → Vérification / validation
  → Création enregistrement dans l’app cible (CRM, Helpdesk, Recruitment, etc.)
  → Réponse (redirection, message, email)
```

**Exemples :**
- **Contact / Email** : envoi email (mail ou module dédié), pas de création modèle métier.
- **Create an Opportunity** : création `crm.lead` (CRM).
- **Create a Ticket** : création ticket Helpdesk.
- **Apply for a Job** : création candidature (hr_recruitment).
- **Create a Customer** : création `res.partner` (et éventuellement `sale.order` ou inscription).
- **Subscribe to Newsletter** : création / mise à jour abonnement (mass_mailing).
- **Create a Task** : création `project.task` (Project).

**Données échangées :**
- Champs du formulaire (mappés sur le modèle cible).
- Contexte : website_id, utilisateur (si connecté), UTM, etc.

### 2.2 Pages dynamiques

**Flux :**
- **Route** : controller avec `website=True` ; rendu QWeb avec données du modèle (ex. `product.template` pour /shop).
- **Données** : lecture depuis l’app (eCommerce, Blog, Events, etc.) ; pas d’écriture directe depuis le module website (sauf formulaires).
- **Menus** : `website.menu` peut pointer vers des URLs générées par d’autres modules (/shop, /blog, /event).

### 2.3 Building blocks dépendants

- **Bloc Products** : nécessite **eCommerce** (website_sale) ; affiche des produits depuis `product.template` / `product.product`.
- **Blocs Blog / Forum / Events / Slides / Survey** : dépendent des modules respectifs.
- **Formulaires avec actions métier** : dépendent de CRM, Helpdesk, Recruitment, Project, Email Marketing.

---

## 3. Mécanismes d'Intégration

### 3.1 Controllers

- **Routing** : routes HTTP avec `@http.route(..., website=True)`.
- **Rendu** : `request.render('module.template_name', values)` ; `values` contient les données pour QWeb.
- **Soumission de formulaires** : route dédiée (ex. `/website/form/`) qui lit les champs, valide, appelle le modèle cible (CRM, Helpdesk, etc.) puis redirige ou renvoie un message.

### 3.2 Vues QWeb

- **Héritage** : `inherit_id="website.layout"` pour les pages qui utilisent le layout commun (header, footer).
- **Blocs** : templates QWeb par type de bloc ; chargés dans l’éditeur et rendus côté frontend.
- **Snippets / blocs** : enregistrés dans `website` ou dans les modules optionnels (website_sale, website_blog, etc.).

### 3.3 Modèles

- **website.page** : peut référencer une vue `ir.ui.view` (QWeb) pour le contenu statique.
- **website.menu** : `website_id`, `parent_id`, `page_id` ou URL, `sequence`.
- **website** : `res.company` ou modèle dédié multi-website ; lien avec `res.config.settings` pour la configuration.

### 3.4 Configuration (res.config.settings)

- Paramètres globaux du site (nom, domaine, favicon, analytics, etc.) souvent exposés via `res.config.settings` avec des champs dédiés au website.

---

## 4. APIs et Hooks Typiques

### 4.1 Côté backend (Python)

- **Création de page** : création `website.page` + `ir.ui.view` associée.
- **Création de menu** : création `website.menu` avec `parent_id`, `page_id`, `url`, `sequence`.
- **Rendu** : `request.website` pour accéder au site courant ; `request.website.with_context(...)` pour le multi-website.
- **Formulaires** : contrôleur qui reçoit POST, valide, puis appelle par exemple `request.env['crm.lead'].create(vals)` ou `request.env['project.task'].create(vals)`.

### 4.2 Côté frontend (JS / QWeb)

- **Éditeur** : JS pour drag & drop, panneau Customize, sauvegarde des blocs (appels RPC ou sauvegarde de la vue).
- **Assets** : bundles CSS/JS du module website et des modules optionnels (website_sale, etc.) chargés sur les pages avec `website=True`.

### 4.3 Redirections

- **website.redirect** (ou équivalent) : modèle de redirection (URL from, URL to, type 301/302/308, website_id, sequence, active).
- **Mode développeur** : accès au menu Website ‣ Configuration ‣ Redirects pour créer/éditer les redirections.

---

## 5. Schéma de Dépendances (Simplifié)

```
website (core)
  ├── base, web
  ├── mail (optionnel)
  ├── portal (optionnel)
  └── Extensions optionnelles :
        ├── website_sale     → eCommerce (produits, panier, Create a Customer)
        ├── website_blog     → Blog (pages /blog)
        ├── website_crm      → CRM (Create an Opportunity)
        ├── website_helpdesk → Helpdesk (Create a Ticket)
        ├── hr_recruitment   → Recruitment (Apply for a Job)
        ├── mass_mailing     → Newsletter (Subscribe)
        ├── project         → Project (Create a Task)
        ├── website_live_chat, website_event, website_survey, website_slides…
        └── thèmes (website_theme_*)
```

---

## 6. Recommandations pour Miyukini

- **Opérateur Website** : MiyuWeb ou MiyukiniWeb comme Opérateur d’Interface ; pas d’exécution métier directe — délégation aux Opérateurs métier via BondingBrother et Mandats.
- **Formulaires** : chaque action (créer contact, opportunité, ticket, tâche, etc.) = flux gouverné avec **Contrat d’équipe** et **Mandat de Permission** entre MiyuWeb et MiyuContacts, MiyuCRM, MiyuForum/Helpdesk, MiyuProject, MiyuNotify, etc.
- **Pages dynamiques** : équivalent « pages générées » = données lues depuis d’autres Opérateurs (MiyuStore, MiyuFeeds/Blog, etc.) avec Mandat en lecture ; MiyuWeb ne stocke que la structure (URL, menu, layout).
- **Building blocks** : catalogue de blocs déclarés (Master Butler) ; contenu et options de blocs persistés par KindMother (WriteIntent) ; pas de logique métier dans les blocs.
- **Redirections et SEO** : modèles gouvernés (KindMother, StrongFather) avec traçabilité (Ever Buddy) ; exposition limitée selon niveau de sécurité (WorrySentinel).
- **Multi-website** : si nécessaire, un « site » = périmètre de gouvernance (environnement ou contexte) avec frontières claires (Border Guard).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
