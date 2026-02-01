# Odoo Blog — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Blog** d'Odoo (version 19.0). Il identifie les modèles de données, règles métier, workflows et mécanismes de publication pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, module `website_blog`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (blog.blog, blog.post, blog.tag, blog.tag.category)
- Règles métier et contraintes (publication, visibilité, SEO)
- Workflows (création blog, ajout article, tags, personnalisation)
- Intégration avec Website Builder (pages dynamiques /blog, blocs, éditeur)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `blog.blog` (Blog)

**Rôle :** Représente un **blog** — conteneur pour organiser les articles. Un site peut avoir plusieurs blogs.

**Concepts clés :**
- **Nom** : Libellé du blog
- **Sous-titre (Subtitle)** : Texte d’accroche affiché sur la page d’accueil du blog
- **Menu** : L’item de menu « Blog » est ajouté au site à la première création d’un blog et regroupe tous les blogs
- **Page d’accueil** : Page dynamique listant les articles du blog (personnalisable via Edit ‣ Customize)

**Champs typiques (logique métier) :**
- `name` : Nom du blog
- `subtitle` : Sous-titre
- `website_id` : Site web associé (multi-website)
- Séquence, visibilité

**Règles métier :**
- Création : Website ‣ Configuration ‣ Blogs : Blogs ‣ New
- Un blog est lié à un site (website)
- La page d’accueil du blog est une page dynamique générée par le module

---

### 1.2 Modèle `blog.post` (Article)

**Rôle :** Représente un **article de blog** — contenu (titre, corps HTML, couverture, auteur, date).

**Concepts clés :**
- **Titre** : Titre de l’article
- **Contenu** : Corps de l’article (champ HTML) ; éditable en frontend via website builder (drag-and-drop, pas éditeur HTML brut dans le frontend blog)
- **Blog** : Chaque article appartient à un blog (blog.blog)
- **Couverture (Cover)** : Image de couverture optionnelle (affichage configurable : Cover / No Cover sur la liste des articles)
- **Publication** : Interrupteur « Unpublished » en haut à droite ; l’article n’est visible qu’une fois publié
- **Tags** : Étiquettes pour filtrage et regroupement (blog.tag)
- **Auteur** : Auteur de l’article (affichage configurable)
- **Commentaires / vues** : Statistiques (nombre de commentaires, vues) — affichage configurable

**Champs typiques (logique métier) :**
- `name` / `title` : Titre
- `content` : Corps HTML
- `blog_id` : Blog parent
- `cover_image` ou équivalent : Image de couverture
- `author_id` : Auteur (res.users ou res.partner)
- `published` / `website_published` : Publié ou non
- `tag_ids` : Tags (relation many2many avec blog.tag)
- Dates de création, mise à jour
- SEO : meta title, meta description (si exposé)

**Règles métier :**
- Création : Frontend ‣ + New ‣ Blog Post ; choix du blog, titre, puis Save ; contenu et personnalisation ensuite
- **Publication obligatoire** : Ne pas oublier de basculer « Unpublished » vers publié pour que l’article soit visible
- Illustrations : images libres de droit (ex. intégration Unsplash)
- Raccourci éditeur : `/` dans l’éditeur de texte pour formater et ajouter des éléments

---

### 1.3 Modèle `blog.tag` (Tag)

**Rôle :** Catégorisation des articles pour filtrage par les visiteurs.

**Concepts clés :**
- **Nom** : Libellé du tag
- **Catégorie** : Les tags peuvent être regroupés par thème (blog.tag.category) pour l’affichage dans la barre latérale
- **Used in** : Lien vers les articles utilisant ce tag (ajout de lignes pour associer des articles existants)
- Création depuis un article : Edit ‣ Customize ‣ sélection de la couverture du post ‣ Tags ‣ Choose a record… ‣ sélectionner ou créer un tag

**Règles métier :**
- Création : Website ‣ Configuration ‣ Blogs : Tags ‣ New
- Champs : Name, Category (catégorie de tags), Used in (lignes vers blog.post)
- Un article peut avoir plusieurs tags
- Les tags sont affichés en bas des articles par défaut ; option « Sidebar » pour les afficher aussi sur la page d’accueil du blog (Tags List)

---

### 1.4 Modèle `blog.tag.category` (Catégorie de tags)

**Rôle :** Regrouper les tags par thème pour l’affichage dans la barre latérale.

**Concepts clés :**
- Gestion : Website ‣ Configuration ‣ Blogs : Tag Categories
- Les tags d’une même catégorie sont regroupés dans la liste des tags (sidebar)

**Règles métier :**
- Optionnel ; les tags peuvent exister sans catégorie
- Améliore la navigation pour le visiteur (filtrage par thème)

---

## 2. Publication et Visibilité

### 2.1 Publication des articles

- **État Unpublished** : Article en brouillon, invisible sur le site
- **État Published** : Article visible (toggle en haut à droite)
- Pas de date de publication planifiée documentée dans la doc utilisateur 19.0 (à confirmer en code pour publication différée)

### 2.2 Visibilité et SEO

- Les articles publiés sont accessibles selon les règles du site (public par défaut)
- SEO : bonnes pratiques (titres, meta, URLs) ; possibilité d’intégration avec les mécanismes website (indexation, sitemap)
- Analytics : utilisation de Plausible (ou autre) pour suivre le trafic du blog

---

## 3. Personnalisation (Logique Métier)

### 3.1 Page d’accueil du blog (Blog Homepage)

**Paramètres applicables à toutes les homepages de blogs :**
- **Top Banner** : « Name/Latest Post » (titre du dernier article) ou « Drop Zone for Building Blocks » (remplacer par des blocs)
- **Layout** : Grille (Grid) ou Liste (List)
- **Cards** : Effet carte
- **Increase Readability** : Améliorer la lisibilité du texte
- **Sidebar** : Section « About us »
- **Archives** : Filtre par mois (articles créés ce mois-là)
- **Follow Us** : Liens réseaux sociaux (configurés via bloc Social Media ailleurs sur le site)
- **Tags List** : Liste des tags du blog ; clic pour filtrer les articles
- **Posts List** : Cover (afficher images) / No Cover ; Author ; Comments/Views Stats ; Teaser & Tags

### 3.2 Page article (Blog Post)

**Paramètres applicables à tous les articles :**
- **Layout** : Title Inside Cover / Title above Cover
- **Increase Readability**
- **Sidebar** : Archive (mois) ; Author (auteur + date) ; Blog List (liens vers tous les blogs) ; Share Links (partage réseaux sociaux) ; Tags
- **Breadcrumb** : Fil d’Ariane
- **Bottom** : Next Article (article suivant) ; Comments (commentaires)
- **Select To Tweet** : Proposer de tweeter le texte sélectionné

---

## 4. Workflows Principaux

### 4.1 Création d’un blog

1. Website ‣ Configuration ‣ Blogs : Blogs
2. New ‣ Saisir Nom et Sous-titre
3. Save
4. Le menu « Blog » est ajouté au site (si premier blog) et regroupe tous les blogs

### 4.2 Ajout d’un article

1. Aller sur le site ‣ + New ‣ Blog Post
2. Dans la popup : sélectionner le blog, saisir le titre, Save
3. Rédiger le contenu et personnaliser la page (website builder)
4. **Publier** : basculer l’interrupteur Unpublished → Published

### 4.3 Gestion des tags

- **Création centrale** : Website ‣ Configuration ‣ Blogs : Tags ‣ New (Name, Category, Used in)
- **Depuis un article** : Edit ‣ Customize ‣ couverture du post ‣ Tags ‣ Choose a record… ‣ sélectionner ou créer
- **Catégories** : Website ‣ Configuration ‣ Blogs : Tag Categories

### 4.4 Personnalisation des homepages et des articles

- **Homepage** : Ouvrir une homepage de blog ‣ Edit ‣ Customize ‣ options (Top Banner, Layout, Sidebar, Posts List, etc.)
- **Article** : Ouvrir un article ‣ Edit ‣ Customize ‣ options (Layout, Sidebar, Breadcrumb, Bottom, Select To Tweet)

---

## 5. Intégration Website Builder

- **Pages dynamiques** : Les URLs /blog, /blog/[blog_slug], /blog/[blog_slug]/post/[post_slug] sont gérées par le module website_blog (controllers, routes)
- **Éditeur** : Création et édition des articles en frontend avec le website builder (blocs, drag-and-drop)
- **Contenu** : Le corps de l’article est stocké en HTML ; l’éditeur de site permet d’ajouter des blocs et du contenu sans coder
- **Menu** : website.menu peut pointer vers /blog (ajouté automatiquement à la première création de blog)
- **Blocs** : Blocs spécifiques blog (liste d’articles, dernier article, etc.) utilisables sur d’autres pages du site si le module les expose

---

## 6. Synthèse des Règles Métier

| Domaine | Règle |
|--------|--------|
| Publication | Article visible uniquement si Published (toggle) |
| Appartenance | Chaque article appartient à un blog |
| Tags | Optionnels ; plusieurs tags par article ; catégories de tags pour la sidebar |
| Personnalisation | Paramètres globaux pour toutes les homepages ; paramètres globaux pour tous les articles |
| Menu | Premier blog créé → item « Blog » ajouté au menu du site |
| Contenu | Corps HTML ; édition via website builder (blocs, pas éditeur HTML brut frontend) |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
