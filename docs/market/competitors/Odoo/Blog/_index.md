# Odoo Blog — Index de l'Analyse

## Statut

✅ **Analyse complète à 100% (7/7 documents)**

---

## Documents de l'Analyse

### 1. Logique Métier
📄 [Odoo Blog - Logique Metier Complete.md](./00_logique_metier/Odoo%20Blog%20-%20Logique%20Metier%20Complete.md)

**Contenu :**
- Modèles de données (blog.blog, blog.post, blog.tag, blog.tag.category)
- Publication et visibilité (toggle Unpublished/Published)
- Personnalisation (homepage blog, page article)
- Workflows (création blog, article, tags)
- Intégration Website Builder

### 2. Parcours Utilisateur
📄 [Odoo Blog - Parcours Utilisateur Detailles.md](./01_parcours_utilisateur/Odoo%20Blog%20-%20Parcours%20Utilisateur%20Detailles.md)

**Contenu :**
- Personas (Administrateur site, Rédacteur, Visiteur, Lecteur engagé)
- Parcours d'onboarding (premier blog, premier article, tags)
- Scénarios d'usage principaux (créer/publier article, filtrer par tag/archive, personnaliser)
- Points de friction identifiés
- Recommandations pour Miyukini

### 3. UI/UX
📄 [Odoo Blog - Analyse UI UX.md](./02_ui_ux/Odoo%20Blog%20-%20Analyse%20UI%20UX.md)

**Contenu :**
- Vues backend (Configuration ‣ Blogs : Blogs, Tags, Tag Categories)
- Frontend : homepage blog, page article, mode Edit et Customize
- Options de personnalisation (Top Banner, Layout, Sidebar, Posts List, Bottom)
- Patterns de navigation (menu Blog, breadcrumb, filtres tag/archive, article suivant)
- Intégration Website Builder

### 4. Intégrations Cross-App
📄 [Odoo Blog - Integrations Cross App.md](./03_integrations/Odoo%20Blog%20-%20Integrations%20Cross%20App.md)

**Contenu :**
- Dépendances (website, mail, portal, Unsplash, Plausible)
- Flux de données (Website → Blog, Blog → Mail commentaires, Blog → Portal)
- Mécanismes (controllers, vues QWeb, modèles)
- APIs et hooks typiques
- Recommandations pour Miyukini

### 5. Spécifications Opérateurs Miyukini
📄 [Odoo Blog - Specifications Operateurs Miyukini.md](./04_specifications_miyukini/Odoo%20Blog%20-%20Specifications%20Operateurs%20Miyukini.md)

**Contenu :**
- Opérateurs identifiés (BlogContainerOperator, BlogPostOperator, BlogTagOperator, BlogUI)
- Équipe d'Opérateurs BlogService
- Contrat d'équipe et Mandats
- Intégration avec les Cores et avec l'Équipe Website
- Correspondance Odoo → Miyukini

### 6. Guide Intégration COG
📄 [Odoo Blog - Guide Integration COG.md](./05_integration_cog/Odoo%20Blog%20-%20Guide%20Integration%20COG.md)

**Contenu :**
- Architecture d'intégration COG (Blog + Website)
- Patterns WriteIntent et Mandates (blog, post, tag, publication)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (publication, visiteur, commentaires)

### 7. Guide Implémentation
📄 [Odoo Blog - Guide Implementation.md](./06_guides_implementation/Odoo%20Blog%20-%20Guide%20Implementation.md)

**Contenu :**
- Architecture technique (crates miyublog, extension miyuweb-ui)
- Schémas de données (Blog, Post, Tag, TagCategory)
- API et contrats
- Plan de développement par phases (MVP → Phase 2 → Phase 3)
- Bornage fonctionnel

---

## Service Miyukini Proposé

**Nom :** `MiyukiniBlog` ou `MiyuBlog`

**Opérateurs :**
- **BlogContainerOperator** : Gestion des blogs (conteneurs)
- **BlogPostOperator** : Gestion des articles (création, publication, contenu)
- **BlogTagOperator** : Gestion des tags et catégories
- **BlogUI** : Interface (homepage, article, Customize, création/édition frontend)

**Équipe d'Opérateurs :** BlogService

**Dépendance :** Équipe Website (WebsitePageOperator, WebsiteMenuOperator, WebsiteBlockOperator) pour pages dynamiques, menu et blocs de contenu.

---

## Source d'Analyse

**Documentation :** Odoo 19.0 — Blog (module website_blog)

**Version analysée :** Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Notes

- Application de la catégorie Websites ; dépend de Website Builder pour pages dynamiques, éditeur et menus
- Publication par toggle (Unpublished/Published) ; rappel explicite recommandé pour éviter les oublis
- Tags et catégories de tags pour filtrage et sidebar ; création depuis config et depuis l'article
- Personnalisation globale (Customize) s'applique à tous les blogs / tous les articles
- Commentaires et partage (Share Links, Select To Tweet) à aligner avec Façade Publique Gouvernée et modération (TAMR/WorrySentinel)
