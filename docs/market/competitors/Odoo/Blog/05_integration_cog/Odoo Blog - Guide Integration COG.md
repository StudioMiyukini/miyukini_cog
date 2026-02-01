# Odoo Blog — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Blog dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG (Blog + Website)
- Patterns WriteIntent et Mandates (blog, post, tag, publication)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (publication, visiteur, commentaires)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
BlogUI → BondingBrother → BlogPostOperator    → StrongFather (décision)
                          → BlogContainerOperator → KindMother (WriteIntent)
                          → BlogTagOperator    → Master Butler (permissions)
                          → WebsiteMenuOperator (menu Blog)
                          → WebsitePageOperator (pages dynamiques)
                          → WebsiteBlockOperator (contenu blocs)

Lecture publique (visiteur) → Façade Publique Gouvernée → BlogPostOperator (read published)
                           → Mandat Public d'Accès
```

### 1.2 Flux Typique — Création d'article

1. **Intention utilisateur** → BlogUI (+ New ‣ Blog Post ; choix blog, titre)
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather (créer article)
4. **Vérification permissions** → Master Butler (post.create)
5. **Persistance** → KindMother (WriteIntent Post)
6. **Mise à jour page dynamique** → WebsitePageOperator (si besoin de créer/rafraîchir la page)
7. **Contenu et personnalisation** → BlogUI + WebsiteBlockOperator (blocs) ; édition supplémentaire → WriteIntent Post (KindMother)

### 1.3 Flux Typique — Publication d'article

1. **Intention utilisateur** → BlogUI (toggle Unpublished → Published)
2. **Traduction intention** → BondingBrother
3. **Demande décision** → StrongFather (publier)
4. **Vérification sécurité** → WorrySentinel (niveau, visibilité)
5. **Persistance** → KindMother (WriteIntent : post.published = true)
6. **Cycle de vie** → Ever Buddy (brouillon → publié)

### 1.4 Flux Typique — Lecture publique (visiteur)

1. **Requête visiteur** → Façade Publique Gouvernée (URL /blog, /blog/[blog]/post/[post])
2. **Mandat Public d'Accès** → Vérification accès autorisé (lecture blog)
3. **Lecture** → BlogPostOperator (read published only) ; BlogContainerOperator (read) ; BlogTagOperator (read)
4. **Rendu** → BlogUI + WebsiteBlockOperator (pages dynamiques, blocs)
5. **Pas d'écriture** : visiteur ne modifie rien

---

## 2. Patterns d'Intégration

### 2.1 Création d'article

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreatePostIntent {
    pub blog_id: BlogId,
    pub title: String,
    pub content: Option<String>,  // HTML ou ref blocs
    pub cover_ref: Option<AssetId>,
    pub tag_ids: Vec<TagId>,
    pub author_id: UserId,
}

impl BlogPostOperator {
    pub async fn create_post(
        &self,
        intent: CreatePostIntent,
        mandate: Mandate,
    ) -> Result<BlogPost, BlogError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["post.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_post",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(BlogError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "post.create",
                resource: None,
            })
            .await?;

        if !permission.granted {
            return Err(BlogError::PermissionDenied);
        }

        let write_intent = PostWriteIntent {
            blog_id: intent.blog_id,
            title: intent.title,
            content: intent.content,
            cover_ref: intent.cover_ref,
            tag_ids: intent.tag_ids,
            author_id: intent.author_id,
            published: false,  // Brouillon par défaut
        };

        self.kind_mother
            .persist(WriteIntent::Create(write_intent), mandate)
            .await
    }
}
```

### 2.2 Publication d'article

**Pattern :** WriteIntent (update) + Mandate

```rust
pub struct PublishPostIntent {
    pub post_id: PostId,
    pub publish: bool,  // true = publier, false = dépublier
}

impl BlogPostOperator {
    pub async fn publish_post(
        &self,
        intent: PublishPostIntent,
        mandate: Mandate,
    ) -> Result<BlogPost, BlogError> {
        mandate.validate_flows(&["post.publish"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "publish_post",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(BlogError::DecisionDenied);
        }

        self.worry_sentinel
            .check_visibility(VisibilityRequest {
                resource: "blog_post",
                level: SecurityLevel::Standard,
            })
            .await?;

        let write_intent = PostWriteIntent::Update {
            post_id: intent.post_id,
            published: Some(intent.publish),
            ..Default::default()
        };

        self.kind_mother
            .persist(WriteIntent::Update(write_intent), mandate)
            .await
    }
}
```

### 2.3 Création de blog (premier blog → menu)

**Pattern :** WriteIntent Blog + coordination WebsiteMenuOperator

```rust
impl BlogContainerOperator {
    pub async fn create_blog(
        &self,
        intent: CreateBlogIntent,
        mandate: Mandate,
    ) -> Result<Blog, BlogError> {
        // ... décision, permission, KindMother (blog) ...

        let blog = self.kind_mother.persist(WriteIntent::Create(intent), mandate).await?;

        // Premier blog ? Ajouter l'item "Blog" au menu du site
        if self.is_first_blog_for_site(blog.website_id).await? {
            self.bonding_brother
                .request(WebsiteMenuOperator::add_menu_item {
                    label: "Blog",
                    url: "/blog",
                    website_id: blog.website_id,
                }, mandate)
                .await?;
        }

        Ok(blog)
    }
}
```

### 2.4 Lecture publique (visiteur)

**Pattern :** Mandat Public d'Accès ; pas de WriteIntent

```rust
impl BlogPostOperator {
    pub async fn list_published_posts(
        &self,
        blog_id: BlogId,
        public_mandate: PublicAccessMandate,
    ) -> Result<Vec<BlogPostSummary>, BlogError> {
        public_mandate.validate_services(&["blog.read"])?;
        // Pas de StrongFather/KindMother en écriture ; lecture seule
        self.repo.list_published_by_blog(blog_id).await
    }
}
```

---

## 3. Gestion des Gouvernances

### 3.1 Publication

- **StrongFather** : Décision de publier ou dépublier
- **KindMother** : Persistance de l’état published (WriteIntent)
- **Ever Buddy** : Cycle de vie (brouillon → publié) ; pas de rétroaction automatique
- **WorrySentinel** : Niveau de sécurité et visibilité (public par défaut)

### 3.2 Visiteur (Utilisateur externe)

- **Façade Publique Gouvernée** : Lecture du blog et des articles publiés uniquement
- **Mandat Public d'Accès** : Encadrement de l’accès (quotas, rate limits si besoin)
- **Pas d'écriture** : Le visiteur ne crée pas de blog, article ni tag ; pas de WriteIntent pour lui

### 3.3 Commentaires (si implémentés)

- **TAMR** : Intervention humaine (modération) si politique stricte
- **WorrySentinel** : Niveau de sécurité du contenu généré par les utilisateurs
- **Opérateur dédié** : MiyuForum ou module commentaires ; flux soumission → StrongFather (autoriser) → KindMother (persister) sous Mandat

---

## 4. Récapitulatif des Mandats

| Action | Mandat | Cores sollicités |
|--------|--------|-------------------|
| Créer blog | Mandat Permission (BlogContainerOperator, WebsiteMenuOperator si premier) | StrongFather, KindMother, Master Butler |
| Créer article | Mandat Permission (BlogPostOperator) | StrongFather, KindMother, Master Butler |
| Publier article | Mandat Permission (BlogPostOperator) | StrongFather, KindMother, WorrySentinel, Ever Buddy |
| Créer tag | Mandat Permission (BlogTagOperator) | StrongFather, KindMother, Master Butler |
| Lire blog (visiteur) | Mandat Public d'Accès | Aucun (lecture seule) |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
