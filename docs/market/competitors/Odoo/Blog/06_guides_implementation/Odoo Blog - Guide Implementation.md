# Odoo Blog — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent Blog dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates, modules)
- Spécifications des modèles de données (Blog, Post, Tag, TagCategory)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel

---

## 1. Architecture Technique

### 1.1 Positionnement par rapport à miyuweb

Le Blog s’appuie sur l’Équipe Website (miyuweb, miyuweb-blocks, miyuweb-ui). Il ajoute une couche métier « blog » (blogs, articles, tags) et des pages dynamiques /blog, /blog/[blog], /blog/[blog]/post/[post].

**Options d’organisation :**
- **Option A** : Crates dédiées `miyublog` (logique métier) + extension dans `miyuweb-ui` pour les vues blog
- **Option B** : Sous-modules dans `miyuweb` (blog comme domaine) + vues dans `miyuweb-ui`

**Recommandation :** Option A — crate `miyublog` pour BlogContainerOperator, BlogPostOperator, BlogTagOperator ; extension de `miyuweb-ui` (ou module frontend dédié) pour BlogUI (homepage, article, Customize, création/édition). Réutilisation de miyuweb pour pages dynamiques et menus.

### 1.2 Structure des crates (recommandée)

```
crates/
├── miyuweb/                    # Existant — WebsitePageOperator, WebsiteMenuOperator, etc.
│   └── ...
├── miyuweb-blocks/             # Existant — blocs de contenu
│   └── ...
├── miyuweb-ui/                 # Existant — éditeur, frontend ; extension pour Blog
│   └── src/
│       ├── blog_ui.rs          # BlogUI : homepage, page article, Customize, + New Blog Post
│       └── ...
├── miyublog/                   # Nouveau — Opérateurs Blog
│   ├── src/
│   │   ├── lib.rs
│   │   ├── blog.rs             # BlogContainerOperator + modèle Blog
│   │   ├── post.rs             # BlogPostOperator + modèle Post
│   │   ├── tag.rs              # BlogTagOperator + modèles Tag, TagCategory
│   │   ├── visibility.rs       # Published, brouillon
│   │   └── admin_cell.rs
│   └── Cargo.toml
```

### 1.3 Dépendances principales

**miyublog :**
- `miyukini-kernel`
- `miyukini-central` (Cores : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, BondingBrother)
- `miyuweb` (pages dynamiques, menus) — pour enregistrer les routes /blog et l’item de menu

**miyuweb-ui (extension blog) :**
- `miyublog` (lecture blog, post, tag pour affichage)
- `miyuweb` (layout, blocs)
- Cores via BondingBrother (intentions création, publication)

---

## 2. Schémas de Données

### 2.1 Modèle Blog

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blog {
    pub id: BlogId,
    pub website_id: WebsiteId,
    pub name: String,
    pub subtitle: Option<String>,
    pub sequence: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.2 Modèle Post

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: PostId,
    pub blog_id: BlogId,
    pub title: String,
    pub content: String,           // HTML ou ref structure blocs
    pub content_ref: Option<BlockStructureId>,  // Si contenu = blocs
    pub cover_ref: Option<AssetId>,
    pub author_id: UserId,
    pub tag_ids: Vec<TagId>,
    pub published: bool,
    pub published_at: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPostSummary {
    pub id: PostId,
    pub blog_id: BlogId,
    pub title: String,
    pub cover_ref: Option<AssetId>,
    pub author_id: UserId,
    pub tag_ids: Vec<TagId>,
    pub published: bool,
    pub published_at: Option<DateTime>,
    pub teaser: Option<String>,
    pub comments_count: Option<u32>,
    pub views_count: Option<u32>,
}
```

### 2.3 Modèle Tag et TagCategory

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogTag {
    pub id: TagId,
    pub name: String,
    pub category_id: Option<TagCategoryId>,
    pub post_ids: Vec<PostId>,   // Ou relation inverse selon requêtes
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogTagCategory {
    pub id: TagCategoryId,
    pub name: String,
    pub sequence: i32,
}
```

---

## 3. API et Contrats

### 3.1 BlogContainerOperator

- `create_blog(intent, mandate) -> Result<Blog>`
- `update_blog(blog_id, intent, mandate) -> Result<Blog>`
- `read_blog(blog_id) -> Result<Blog>`
- `list_blogs(website_id) -> Result<Vec<Blog>>`

### 3.2 BlogPostOperator

- `create_post(intent, mandate) -> Result<BlogPost>`
- `update_post(post_id, intent, mandate) -> Result<BlogPost>`
- `publish_post(post_id, publish: bool, mandate) -> Result<BlogPost>`
- `read_post(post_id) -> Result<BlogPost>` (avec contrôle published si visiteur)
- `list_posts(blog_id, filter: ListFilter) -> Result<Vec<BlogPostSummary>>` (filter : all / published_only ; tag ; archive month)

**ListFilter :**
- `tag_id: Option<TagId>`
- `archive_month: Option<(Year, Month)>`
- `published_only: bool`

### 3.3 BlogTagOperator

- `create_tag(intent, mandate) -> Result<BlogTag>`
- `update_tag(tag_id, intent, mandate) -> Result<BlogTag>`
- `create_tag_category(intent, mandate) -> Result<BlogTagCategory>`
- `associate_tags_to_post(post_id, tag_ids, mandate) -> Result<()>`
- `list_tags(blog_id?) -> Result<Vec<BlogTag>>`
- `list_tag_categories() -> Result<Vec<BlogTagCategory>>`

---

## 4. Routes et Pages Dynamiques

- **GET /blog** : Liste des blogs (ou redirection vers un blog par défaut)
- **GET /blog/[blog_slug ou blog_id]** : Homepage du blog (liste des articles ; filtres Archives, Tags)
- **GET /blog/[blog_slug]/post/[post_slug]** : Page article (contenu, sidebar, breadcrumb, next article, commentaires, partage)

Enregistrement des routes dans miyuweb ou miyublog (controllers) avec `website=True` équivalent (layout commun header/footer).

---

## 5. Plan de Développement par Phases

### Phase 1 — MVP

- **Modèles** : Blog, Post, Tag, TagCategory (schémas ci-dessus)
- **BlogContainerOperator** : create_blog, read_blog, list_blogs
- **BlogPostOperator** : create_post, update_post, publish_post, read_post, list_posts (published_only pour frontend public)
- **BlogTagOperator** : create_tag, update_tag, associate_tags_to_post, list_tags, list_tag_categories
- **Routes** : /blog, /blog/[blog], /blog/[blog]/post/[post] (lecture seule)
- **BlogUI (minimal)** : Homepage blog (liste articles), page article (contenu, titre, couverture, tags) ; pas de Customize ni édition frontend en MVP
- **Menu** : Ajout de l’item « Blog » au menu du site à la première création d’un blog
- **Gouvernance** : WriteIntent + Mandate pour création/publication ; Mandat Public d’Accès pour lecture

**Livrable MVP :** Un blog avec articles publiés, tags, listage et lecture publique ; création/édition en backend (API ou écrans admin).

### Phase 2 — Édition frontend et personnalisation

- **BlogUI** : Création/édition d’article en frontend (+ New ‣ Blog Post ; choix blog, titre ; contenu via blocs website builder)
- **Publication** : Toggle Unpublished/Published avec rappel si non publié
- **Customize** : Options globales homepage blog (Top Banner, Layout, Sidebar, Archives, Tags List, Posts List) et options globales article (Layout, Sidebar, Breadcrumb, Bottom, Select To Tweet)
- **Tags** : Ajout/création de tags depuis l’article (Edit ‣ Customize ‣ couverture ‣ Tags)

**Livrable Phase 2 :** Parité fonctionnelle avec Odoo Blog (création frontend, personnalisation, publication).

### Phase 3 — Avancé

- **Commentaires** : Intégration MiyuForum ou module commentaires (option Bottom) ; modération (TAMR)
- **Partage** : Share Links, Select To Tweet (liens configurés, pas de logique métier lourde)
- **Analytics** : Compteurs vues/commentaires ; intégration Plausible ou équivalent (Façade Publique Gouvernée)
- **Archives** : Filtre par mois déjà prévu en Phase 2 (list_posts avec archive_month)
- **Multi-site** : website_id déjà dans Blog ; filtrage menus et URLs par site

---

## 6. Bornage Fonctionnel

| Fonctionnalité Odoo | MVP | Phase 2 | Phase 3 |
|---------------------|-----|--------|--------|
| Création blog (backend) | Oui | Oui | Oui |
| Création article (backend) | Oui | Oui | Oui |
| Création article (frontend + New) | Non | Oui | Oui |
| Publication (toggle) | Oui | Oui | Oui |
| Tags et catégories | Oui | Oui | Oui |
| Homepage blog (liste, filtres tag/archive) | Oui | Oui | Oui |
| Page article (contenu, sidebar, breadcrumb, next) | Oui | Oui | Oui |
| Customize (options globales) | Non | Oui | Oui |
| Commentaires | Non | Non | Oui |
| Partage (Share Links, Select To Tweet) | Non | Optionnel | Oui |
| Analytics (vues, commentaires) | Non | Optionnel | Oui |
| Menu Blog (premier blog) | Oui | Oui | Oui |
| Intégration Unsplash (images) | Optionnel | Optionnel | Optionnel |

---

## 7. Risques et Points d’Attention

- **Contenu** : Stockage HTML vs structure blocs — décision à aligner avec miyuweb-blocks (contenu article = structure blocs ou HTML brut).
- **Customize global** : Les options s’appliquent à tous les blogs / tous les articles ; pas de surcharge par blog/article dans Odoo — à décider pour Miyukini (MVP : global uniquement).
- **Commentaires** : Périmètre distinct (MiyuForum ou module dédié) ; gouvernance et modération à définir (TAMR, WorrySentinel).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
