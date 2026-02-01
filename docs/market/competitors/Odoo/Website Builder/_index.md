# Odoo Website Builder — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Website Builder - Logique Metier Complete.md](./00_logique_metier/Odoo%20Website%20Builder%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (website, website.page, website.menu, ir.ui.view)
- Pages statiques vs dynamiques
- Building blocks et éditeur visuel
- Publication, visibilité, SEO, redirections URL
- Thèmes et personnalisation
- Formulaires et intégration données

### 2. Parcours Utilisateur
📄 [Odoo Website Builder - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Website%20Builder%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Administrateur site, Créateur de contenu, Visiteur, Utilisateur connecté, Développeur)
- Parcours d'onboarding
- Scénarios d'usage principaux (création page, formulaire, homepage, redirections, visibilité)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Website Builder - Analyse UI UX.md](./02_ui_ux/Odoo%20Website%20Builder%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Éditeur visuel (mode Edit), barre d'outils, panneaux Blocks et Customize
- Building blocks (catégories, inner content, formulaires, embed)
- Propriétés de page (Site ‣ Properties)
- Menus (header, footer)
- Thèmes et personnalisation
- Design responsive et accessibilité
- Recommandations pour Miyukini

### 4. Intégrations Cross-App
📄 [Odoo Website Builder - Integrations Cross App.md](./03_integrations/Odoo%20Website%20Builder%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances avec autres apps Odoo (base, web, mail, portal, website_blog, website_sale, CRM, Helpdesk, Recruitment, Project, Email Marketing)
- Flux de données (formulaires → création enregistrements, pages dynamiques, blocs dépendants)
- Mécanismes (controllers, vues QWeb, modèles, res.config.settings)
- APIs et hooks typiques
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Website Builder - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Website%20Builder%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator, WebsiteRedirectOperator, WebsiteFormOperator, WebsiteUI)
- Contrat d'équipe WebsiteService
- Niveaux de sécurité
- Intégration avec les Cores
- Correspondance Odoo → Miyukini

### 6. Guide Intégration COG
📄 [Odoo Website Builder - Guide Integration COG.md](./05_integration_cog/Odoo%20Website%20Builder%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates (création page, publication, formulaire, redirection)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (visibilité, utilisateur externe, éditeur)

### 7. Guide Implémentation
📄 [Odoo Website Builder - Guide Implementation.md](./06_guides_implementation/Odoo%20Website%20Builder%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique détaillée (crates miyuweb, miyuweb-blocks, miyuweb-forms, miyuweb-ui)
- Spécifications des modèles (Page, Menu, Redirect, Form)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniWeb` ou `MiyuWeb`

**Opérateurs :**
- **WebsitePageOperator** : Gestion des pages (création, publication, propriétés)
- **WebsiteMenuOperator** : Gestion des menus (header, footer)
- **WebsiteBlockOperator** : Catalogue et rendu des building blocks
- **WebsiteRedirectOperator** : Gestion des redirections URL
- **WebsiteFormOperator** : Traitement des formulaires (délégation Opérateurs métier)
- **WebsiteUI** : Interface utilisateur (éditeur visuel, consultation)

**Équipe d'Opérateurs :** WebsiteService

---

## Source d'Analyse

**Documentation :** Odoo 19.0 — Website (module website)

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application centrale pour la catégorie Websites ; base pour Blog, eCommerce, Forum, Live Chat, eLearning
- Formulaires fortement intégrés (CRM, Helpdesk, Recruitment, Project, Email Marketing)
- Building blocks et thèmes permettent personnalisation sans modifier le cœur
- Visibilité (Public / Signed In / Restricted / Password) et redirections à aligner avec gouvernance Miyukini (Mandat Public d'Accès, Visa, WorrySentinel)
