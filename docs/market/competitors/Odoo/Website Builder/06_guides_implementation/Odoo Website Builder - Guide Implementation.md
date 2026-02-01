# Odoo Website Builder — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Website Builder dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyuweb/                           # WebsitePageOperator + WebsiteMenuOperator + WebsiteRedirectOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── page.rs                    # Modèle Page, publication, propriétés
│   │   ├── menu.rs                    # Modèle Menu, hiérarchie
│   │   ├── redirect.rs                # Modèle Redirect
│   │   ├── visibility.rs              # Public, SignedIn, Restricted, Password
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuweb-blocks/                    # WebsiteBlockOperator (catalogue, rendu)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── block.rs                   # Définition blocs, catégories, inner content
│   │   ├── template.rs                # Rendu (structure, pas de logique métier)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuweb-forms/                     # WebsiteFormOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── form.rs                    # Config formulaire, champs, actions
│   │   ├── submit.rs                  # Validation, délégation Opérateurs métier
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuweb-ui/                        # WebsiteUI (éditeur + frontend)
    ├── src/
    │   ├── lib.rs
    │   ├── editor.rs                  # Mode Edit, barre d’outils, panneaux
    │   ├── properties.rs              # Propriétés de page (URL, menu, publication, visibilité)
    │   ├── menus_editor.rs             # Édition header/footer
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, BondingBrother)

**Kits existants :**
- `miyucontacts` : Création contact (formulaire)
- `miyunotify` : Newsletter, notifications
- `miyuclock` : Dates (publication planifiée)
- `miyucms` : Contenu (si réutilisation pour blocs)
- Opérateurs métier : MiyuCRM, MiyuForum/Helpdesk, MiyuProject, etc. (formulaires)

---

## 2. Schémas de Données

### 2.1 Modèle Page

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsitePage {
    pub id: PageId,
    pub website_id: WebsiteId,
    pub title: String,
    pub url: String,
    pub content_ref: Option<ViewId>,  // Référence structure (blocs) ou vue QWeb équivalent

    pub in_menu: bool,
    pub is_homepage: bool,
    pub published: bool,
    pub publishing_date: Option<DateTime>,
    pub indexed: bool,
    pub visibility: PageVisibility,

    pub restricted_group_ids: Vec<GroupId>,  // Si Restricted Group
    pub password_hash: Option<String>,       // Si With Password (hash uniquement)

    pub is_template: bool,  // Bloc personnalisé (Custom)

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageVisibility {
    Public,
    SignedIn,
    RestrictedGroup,
    WithPassword,
}
```

### 2.2 Modèle Menu

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteMenu {
    pub id: MenuId,
    pub website_id: WebsiteId,
    pub parent_id: Option<MenuId>,
    pub sequence: u32,

    pub name: String,
    pub page_id: Option<PageId>,
    pub url: Option<String>,  // URL externe si pas de page_id

    pub menu_type: MenuType,  // Header, Footer, Other
    pub active: bool,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.3 Modèle Redirect

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteRedirect {
    pub id: RedirectId,
    pub website_id: Option<WebsiteId>,
    pub url_from: String,
    pub url_to: String,
    pub redirect_type: RedirectType,
    pub sequence: u32,
    pub active: bool,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RedirectType {
    NotFound404,
    MovedPermanently301,
    MovedTemporarily302,
    RedirectRewrite308,
}
```

### 2.4 Modèle Form (config formulaire)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteForm {
    pub id: FormId,
    pub page_id: PageId,
    pub name: String,

    pub action: FormAction,
    pub on_success: FormOnSuccess,  // Redirect(url), Nothing, ShowMessage(text)
    pub redirect_url: Option<String>,
    pub success_message: Option<String>,

    pub field_ids: Vec<FormFieldId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormAction {
    SendEmail,
    CreateCustomer,      // MiyuContacts
    CreateOpportunity,  // MiyuCRM
    CreateTicket,       // MiyuForum/Helpdesk
    ApplyForJob,       // MiyuHR/Recruitment
    SubscribeNewsletter, // MiyuNotify
    CreateTask,        // MiyuProject
}
```

---

## 3. API et Contrats

### 3.1 WebsitePageOperator

- `create_page(intent, mandate) -> Result<WebsitePage>`
- `update_page(page_id, intent, mandate) -> Result<WebsitePage>`
- `publish_page(intent, mandate) -> Result<()>`
- `duplicate_page(page_id, new_title, mandate) -> Result<WebsitePage>`
- `delete_page(page_id, mandate) -> Result<()>`
- `get_page_by_url(website_id, url) -> Result<Option<WebsitePage>>`
- `list_pages(website_id, filter) -> Result<Vec<WebsitePage>>`

### 3.2 WebsiteMenuOperator

- `create_menu(intent, mandate) -> Result<WebsiteMenu>`
- `update_menu(menu_id, intent, mandate) -> Result<WebsiteMenu>`
- `reorder_menus(website_id, parent_id, order, mandate) -> Result<()>`
- `add_page_to_menu(page_id, mandate) -> Result<WebsiteMenu>`
- `list_menus(website_id, menu_type) -> Result<Vec<WebsiteMenu>>`

### 3.3 WebsiteBlockOperator

- `list_categories() -> Result<Vec<BlockCategory>>`
- `list_blocks(category_id) -> Result<Vec<BlockTemplate>>`
- `render_block(block_id, context) -> Result<RenderedBlock>`
- `save_custom_block(name, block_data, mandate) -> Result<BlockTemplateId>`

### 3.4 WebsiteRedirectOperator

- `create_redirect(intent, mandate) -> Result<WebsiteRedirect>`
- `update_redirect(redirect_id, intent, mandate) -> Result<WebsiteRedirect>`
- `resolve_redirect(website_id, url) -> Result<Option<RedirectResult>>`
- `list_redirects(website_id) -> Result<Vec<WebsiteRedirect>>`

### 3.5 WebsiteFormOperator

- `submit_form(intent, mandate) -> Result<FormSubmitResult>`
- `get_form_config(form_id) -> Result<WebsiteForm>`
- `validate_form_fields(form_id, fields) -> Result<()>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Bornage minimal)

**Objectif :** Site vitrine avec pages statiques, menu, publication, visibilité Public.

**Livrables :**
- Crates : `miyuweb` (page, menu, pas encore redirect)
- Modèles : WebsitePage, WebsiteMenu (champs essentiels)
- API : create_page, update_page, publish_page, list_pages, create_menu, update_menu, list_menus
- WebsiteUI : consultation (rendu pages), pas encore éditeur visuel complet
- Visibilité : Public uniquement
- Pas de building blocks avancés : structure de page simple (titre, contenu texte ou référence vue)
- Pas de formulaires métier
- Pas de redirections

**Durée estimée :** 2–3 semaines

### Phase 2 — Éditeur et blocs

**Objectif :** Éditeur visuel (mode Edit), building blocks (catégories de base), propriétés de page.

**Livrables :**
- Crate : `miyuweb-blocks` (catalogue, rendu blocs Basic, About, etc.)
- WebsiteUI : mode Edit, panneau Blocks (drag & drop), panneau Customize (fond, layout)
- Propriétés de page : URL, In Menu, Is Homepage, Published, Indexed, Visibility (SignedIn, Restricted, Password)
- Duplication et suppression de page avec avertissement liens
- Thème minimal (couleurs, polices)

**Durée estimée :** 3–4 semaines

### Phase 3 — Formulaires et redirections

**Objectif :** Formulaires (contact, actions métier), redirections URL.

**Livrables :**
- Crate : `miyuweb-forms` (config formulaire, validation, délégation)
- Intégration BondingBrother avec MiyuContacts, MiyuCRM, MiyuForum, MiyuProject, MiyuNotify (selon apps installées)
- Actions : SendEmail, CreateCustomer, CreateOpportunity, CreateTicket, ApplyForJob, SubscribeNewsletter, CreateTask
- Crate `miyuweb` : WebsiteRedirectOperator (create_redirect, resolve_redirect, list_redirects)
- Configuration : Website ‣ Configuration ‣ Redirects (équivalent)
- Gestion des liens référents à la suppression de page + création redirection

**Durée estimée :** 3–4 semaines

### Phase 4 — Complet

**Objectif :** Multi-website, publication planifiée, blocs personnalisés, SEO avancé, thèmes.

**Livrables :**
- Multi-website : website_id sur Page, Menu, Redirect ; sélection site courant
- Publication planifiée : publishing_date, job planifié pour publier/dépublier
- Blocs personnalisés : save_custom_block, catégorie Custom
- SEO : meta title, meta description, sitemap (optionnel)
- Thèmes : options par site (couleurs, polices, header/footer variants)
- Tests de charge et sécurité (visibilité, formulaires, redirections)

**Durée estimée :** 2–3 semaines

---

## 5. Bornage Fonctionnel (MVP → Complet)

| Fonctionnalité | MVP | Phase 2 | Phase 3 | Complet |
|----------------|-----|---------|---------|---------|
| Pages statiques | ✅ | ✅ | ✅ | ✅ |
| Menus (header/footer) | ✅ | ✅ | ✅ | ✅ |
| Publication / dépublication | ✅ | ✅ | ✅ | ✅ |
| Visibilité Public | ✅ | ✅ | ✅ | ✅ |
| Visibilité SignedIn / Restricted / Password | ❌ | ✅ | ✅ | ✅ |
| Éditeur visuel (Edit) | ❌ | ✅ | ✅ | ✅ |
| Building blocks (catégories) | ❌ | ✅ | ✅ | ✅ |
| Blocs personnalisés (Custom) | ❌ | ❌ | ❌ | ✅ |
| Propriétés (URL, menu, homepage, SEO) | Partiel | ✅ | ✅ | ✅ |
| Formulaires (contact, CRM, etc.) | ❌ | ❌ | ✅ | ✅ |
| Redirections URL | ❌ | ❌ | ✅ | ✅ |
| Multi-website | ❌ | ❌ | ❌ | ✅ |
| Publication planifiée | ❌ | ❌ | ❌ | ✅ |
| Thèmes | ❌ | Minimal | ✅ | ✅ |

---

## 6. Correspondance Miyukini

**Service proposé :** `MiyukiniWeb` ou `MiyuWeb`

**Opérateurs :**
- WebsitePageOperator (miyuweb)
- WebsiteMenuOperator (miyuweb)
- WebsiteRedirectOperator (miyuweb)
- WebsiteBlockOperator (miyuweb-blocks)
- WebsiteFormOperator (miyuweb-forms)
- WebsiteUI (miyuweb-ui)

**Équipe d'Opérateurs :** WebsiteService

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
