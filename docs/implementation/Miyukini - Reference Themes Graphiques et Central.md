# Miyukini - Reference Themes Graphiques et Central

## Contexte

Miyukini Central est le **Hub de gestion des Services** de l'ecosysteme Miyukini COG (Core-Orchestrated Governance). C'est le point d'entree unique pour l'utilisateur du COG : tous les Services s'affichent dans des onglets au sein de Central, jamais en standalone.

Ce document est la **reference d'implementation** pour tout developpeur travaillant sur l'UI des Services integres dans Central. Il decrit l'architecture complete, le systeme de themes, les patterns UI, la couche donnees, et les conventions a respecter.

## Portee / Scope

- Architecture de l'application `apps/central` (Dioxus Desktop natif)
- Systeme de themes graphiques (`ThemePalette`, `styles`, espacement)
- Couche de donnees partagee (`ServiceConnections`, bases KindMother)
- Etat global (`AppState`, `AppContext`)
- Patterns UI recurents (sidebar, composants partages, navigation)
- Guide pour ajouter un nouveau Service dans Central
- Lien profil Central et Services

---

## 1. Architecture Globale

### 1.1 Deux crates distincts

| Crate | Chemin | Role |
|-------|--------|------|
| `miyukini-central` | `crates/miyukini-central/` | **Bibliotheque** : auth, catalog, config, services (types). Pas d'UI. |
| `miyukini-central-native` | `apps/central/` | **Application Dioxus Desktop** : UI complete, rendu natif WGPU. |

L'application native (`apps/central`) depend de la bibliotheque (`crates/miyukini-central`) pour l'authentification et le catalogue.

### 1.2 Stack technique

| Composant | Technologie |
|-----------|-------------|
| Framework UI | **Dioxus 0.6** (Desktop, rendu natif) |
| Langage | **Rust** |
| Rendu | Desktop natif WGPU (pas de webview) |
| Styling | CSS inline via fonctions Rust (`theme::styles::*`) |
| Persistance | **SQLite** via `rusqlite` (mode `legacy-sqlite`) |
| Audio | **rodio** (voix Miou MP3) |
| Async | **tokio** (runtime multi-thread) |
| Logging | **tracing** + `tracing-subscriber` |

### 1.3 Arborescence de l'application

```
apps/central/src/
├── main.rs              # Point d'entree, config fenetre Dioxus Desktop
├── app.rs               # Composant racine App, providers, CSS global
├── theme.rs             # Themes visuels (palette, styles, spacing)
├── state.rs             # Etat global (AppState, AppContext, types)
├── data.rs              # Connexions DB partagees (ServiceConnections)
├── audio.rs             # Lecture audio (voix Miou, rodio)
├── components/          # Composants UI reutilisables Central
│   ├── mod.rs
│   ├── header.rs        # Header principal Steam-like
│   ├── tab_bar.rs       # Barre d'onglets des services ouverts
│   ├── service_card.rs  # Carte de service (grille magasin)
│   └── service_grid.rs  # Grille avec filtres (Tous/Installes/Favoris)
├── screens/             # Ecrans plein-ecran (auth)
│   ├── mod.rs
│   ├── rite_entree.rs   # Premier compte (COG vierge)
│   ├── connexion.rs     # Connexion utilisateur existant
│   └── profile_window.rs # Modal profil (overlay)
└── services/            # Vues des services integres
    ├── mod.rs           # ActiveServiceView (routeur d'onglets)
    ├── home.rs          # Onglet Accueil
    ├── service_view.rs  # Vue generique (placeholder)
    ├── jayxpose_view.rs # JayXpose
    ├── jayfestival/     # JayFestival (multi-fichiers)
    ├── jaykonta/        # JayKonta (multi-fichiers)
    ├── jaykoa_view.rs   # JayKoa (vue unique)
    ├── game_view.rs     # MiyuClicker (jeu)
    └── survivor_embed.rs # Lord of the Castle (jeu)
```

---

## 2. Systeme de Themes Graphiques

### 2.1 Architecture du theme

Le theme est defini dans `apps/central/src/theme.rs`. Tous les elements UI **doivent** utiliser les couleurs et styles du theme, jamais de couleurs en dur.

```rust
// Enum des themes disponibles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Gaming,  // Style sombre type Steam (actuel)
}
```

Le theme actif est stocke dans `AppState.current_theme` et accessible partout via :
```rust
let theme = state.read().current_theme;
let c = theme.palette();  // -> ThemePalette
```

### 2.2 ThemePalette — Toutes les couleurs

La palette definit **toutes les couleurs** utilisables dans l'UI. Voici le theme Gaming (Steam) :

| Champ | Valeur hex | Role |
|-------|-----------|------|
| `bg_main` | `#171a21` | Fond principal de l'application |
| `bg_header` | `#1b2838` | Fond du header et footer |
| `bg_card` | `#1e2329` | Fond des cartes, panneaux, modals |
| `bg_hover` | `#2a3f5f` | Fond au survol, items actifs sidebar |
| `bg_active` | `#1a9fff` | Fond element actif (accent) |
| `bg_secondary` | `#232f3e` | Fond secondaire (sidebars, KPI cards) |
| `text_primary` | `#c6d4df` | Texte principal |
| `text_secondary` | `#8f98a0` | Texte secondaire / labels |
| `text_muted` | `#5c6873` | Texte attenue / dates, IDs |
| `text_link` | `#66c0f4` | Liens et labels accent |
| `text_white` | `#ffffff` | Texte fort / titres actifs |
| `accent_blue` | `#1a9fff` | Boutons primaires, onglets actifs, liens |
| `accent_blue_hover` | `#66c0f4` | Bleu survol |
| `accent_green` | `#5ba32b` | Badge "Installe", montants positifs |
| `accent_orange` | `#ff6b00` | Alertes, statut "en preparation" |
| `accent_red` | `#c83737` | Erreurs, montants negatifs |
| `border` | `#2a3f5f` | Bordures par defaut |
| `border_hover` | `#66c0f4` | Bordures au survol |

### 2.3 Espacements (`spacing`)

| Constante | Valeur | Usage |
|-----------|--------|-------|
| `HEADER_HEIGHT` | `40px` | Hauteur du header principal |
| `NAV_HEIGHT` | `36px` | Hauteur barre de navigation secondaire |
| `PADDING` | `16px` | Padding standard |
| `PADDING_SM` | `8px` | Padding petit |
| `PADDING_LG` | `24px` | Padding large |
| `RADIUS` | `4px` | Border-radius standard |
| `RADIUS_LG` | `8px` | Border-radius large (cartes, modals) |

### 2.4 Fonctions de style (`styles::*`)

Chaque composant utilise des fonctions de style qui retournent une `String` CSS inline. Elles prennent toutes le `Theme` en parametre :

#### Layout global
| Fonction | Usage |
|----------|-------|
| `main_container(theme)` | Conteneur racine (flexbox vertical, min-height 100vh) |
| `header(theme)` | Header principal (flexbox, height 40px) |
| `content_area(theme)` | Zone de contenu principale |
| `content_panel(theme)` | Panneau de contenu (fond card, padding) |
| `fullscreen_container(theme)` | Ecran plein (auth : Rite, Connexion) |

#### Navigation
| Fonction | Usage |
|----------|-------|
| `nav_tab(theme, is_active)` | Onglet de navigation header (MAGASIN, BIBLIOTHEQUE...) |
| `tab_bar(theme)` | Barre d'onglets services ouverts |
| `service_tab(theme, is_active)` | Onglet service individuel |
| `tab_close_btn(theme)` | Bouton fermer onglet |

#### Composants
| Fonction | Usage |
|----------|-------|
| `service_card(theme)` | Carte de service (grille magasin) |
| `service_icon_large(theme)` | Zone icone de la carte (gradient) |
| `service_card_content(theme)` | Contenu textuel de la carte |
| `service_title(theme)` | Titre de la carte |
| `price_badge(theme, is_free)` | Badge prix/installe |
| `type_badge(theme, service_type)` | Badge type de service |
| `search_input(theme)` | Champ recherche du header |
| `user_profile(theme)` | Zone utilisateur du header |
| `avatar(theme)` | Avatar utilisateur |
| `services_grid()` | Grille responsive des services |
| `section_title(theme)` | Titre de section |

#### Modals / Overlays
| Fonction | Usage |
|----------|-------|
| `overlay_backdrop(theme)` | Fond assombri (z-index 1000) |
| `modal_card(theme)` | Carte modale |
| `modal_title(theme)` | Titre modal |
| `modal_body_text(theme)` | Corps modal |
| `modal_label(theme)` | Label accent modal |
| `modal_muted_small(theme)` | Texte muted petit |

#### Boutons
| Fonction | Usage |
|----------|-------|
| `btn_primary(theme)` | Bouton principal (fond accent_blue) |
| `btn_secondary(theme)` | Bouton secondaire (fond hover, bordure) |

#### Formulaires (auth)
| Fonction | Usage |
|----------|-------|
| `form_card(theme)` | Carte formulaire centree |
| `form_title(theme)` | Titre formulaire |
| `form_hint(theme)` | Sous-texte / hint |
| `form_input(theme)` | Champ input |
| `form_btn_primary(theme)` | Bouton principal formulaire |
| `form_error(theme)` | Message d'erreur |

### 2.5 CSS Global

Defini dans `app.rs` via `GLOBAL_CSS`, il etablit :
- Reset CSS (`* { margin: 0; padding: 0; box-sizing: border-box; }`)
- Font : `'Segoe UI', -apple-system, BlinkMacSystemFont, 'Roboto', sans-serif`
- Scrollbars style Steam (fond `#1b2838`, thumb `#3d4f5f`)
- Focus visible : `outline: 2px solid #1a9fff`
- Selection : fond `#1a9fff`, texte blanc
- Transitions globales : `background-color 0.15s, color 0.15s, border-color 0.15s, transform 0.15s`
- Hover boutons : `filter: brightness(1.1)`
- Active boutons : `transform: scale(0.98)`

### 2.6 Couleurs des types de Services

Les types de Service ont des couleurs propres pour les badges :

| Type | Valeur enum | Label | Couleur hex |
|------|------------|-------|-------------|
| Type 1 | `ServiceType::InterneCog` | "Interne COG" | `#3b82f6` (Blue) |
| Type 2 | `ServiceType::SurfaceWeb` | "Surface Web" | `#10b981` (Emerald) |
| Type 3 | `ServiceType::InterCog` | "Inter-COG" | `#8b5cf6` (Violet) |

### 2.7 Comment utiliser le theme dans un composant

Pattern standard :

```rust
use dioxus::prelude::*;
use crate::state::use_app_state;

#[component]
pub fn MonComposant() -> Element {
    let c = use_app_state().read().current_theme.palette();
    
    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",
            
            h2 {
                style: "color: {c.text_white}; font-size: 16px;",
                "Titre"
            }
            p {
                style: "color: {c.text_secondary}; font-size: 13px;",
                "Description"
            }
        }
    }
}
```

Pour les fonctions de style predefinies :

```rust
use crate::theme::styles;

#[component]
pub fn MonEcran() -> Element {
    let state = use_app_state();
    let theme = state.read().current_theme;
    
    rsx! {
        div {
            style: "{styles::fullscreen_container(theme)}",
            div {
                style: "{styles::form_card(theme)}",
                // ...
            }
        }
    }
}
```

---

## 3. Etat Global et Contexte

### 3.1 AppContext

Le contexte est fourni une seule fois a la racine de l'application (`App`) :

```rust
#[derive(Clone)]
pub struct AppContext {
    pub connections: Signal<Arc<ServiceConnections>>,
    pub state: Signal<AppState>,
}
```

Accessible partout via :
```rust
let ctx = use_context::<AppContext>();
// ou les raccourcis :
let state = use_app_state();           // -> Signal<AppState>
let conns = use_service_connections(); // -> Signal<Arc<ServiceConnections>>
```

### 3.2 AppState

```rust
pub struct AppState {
    pub main_tab: MainTab,          // MAGASIN | BIBLIOTHEQUE | COMMUNAUTE | MIYUKINI
    pub open_tabs: Vec<OpenTab>,    // Onglets ouverts (services)
    pub active_tab_index: usize,    // Index onglet actif
    pub services: Vec<ServiceInfo>, // Services disponibles
    pub search_query: String,       // Recherche en cours
    pub current_user: Option<CentralProfile>,  // Profil connecte
    pub is_cog_virgin: bool,        // True si aucun compte cree
    pub show_profile_window: bool,  // Modal profil ouverte
    pub current_theme: Theme,       // Theme actif
    pub last_login_email: String,   // Email dernier profil (pre-remplissage)
    pub last_login_pseudo: String,  // Pseudo dernier profil (accueil)
}
```

### 3.3 MainTab (navigation header)

```rust
pub enum MainTab {
    Magasin,       // "MAGASIN" — grille de tous les services
    Bibliotheque,  // "BIBLIOTHÈQUE" — services installes
    Communaute,    // "COMMUNAUTÉ" — forum, guides, workshop
    Miyukini,      // "MIYUKINI" — parametres du COG
}
```

### 3.4 ServiceInfo

```rust
pub struct ServiceInfo {
    pub id: String,                 // "jayxpose", "jayfestival", etc.
    pub name: String,               // Nom affiche
    pub description: String,        // Description courte
    pub icon: String,               // Emoji (ex: "🏪")
    pub service_type: ServiceType,  // InterneCog | SurfaceWeb | InterCog
    pub is_installed: bool,         // Installe dans le COG
    pub is_favorite: bool,          // Favori utilisateur
    pub version: String,            // Version du service
    pub developer: String,          // Developeur (ex: "Miyukini")
}
```

### 3.5 OpenTab (onglet ouvert)

```rust
pub struct OpenTab {
    pub id: String,                    // Identifiant unique
    pub title: String,                 // Titre affiche
    pub service_id: Option<String>,    // None = onglet Accueil
    pub closable: bool,                // L'onglet Accueil n'est pas fermable
}
```

### 3.6 Services enregistres (par defaut)

| `id` | `name` | `service_type` | `icon` |
|------|--------|----------------|--------|
| `jayxpose` | JayXpose | SurfaceWeb | 🏪 |
| `jayfestival` | JayFestival | SurfaceWeb | 📅 |
| `jaykoa` | JayKoa | InterneCog | 📆 |
| `jaykonta` | JayKonta | InterneCog | 🧮 |
| `miyuclicker` | Lord of the Click | InterCog | 🎮 |
| `lord_of_the_castle` | Miyukini Survivor | InterCog | 🏰 |

---

## 4. Couche Donnees Partagee

### 4.1 ServiceConnections

Toutes les bases de donnees sont ouvertes au demarrage et partagees en `Arc` :

```rust
pub struct ServiceConnections {
    pub auth_db: Arc<CentralAuthDb>,       // Profils, session, COG vierge
    pub jayxpose: Arc<JayXposeDb>,         // Exposants, catalogue, vitrine
    pub jaykonta: Arc<JayKontaDb>,         // Comptabilite Purse/Account
    pub jayfestival: Arc<JayFestivalDb>,   // Editions, organisateurs, exposants
    pub jaykoa: Arc<JayKoaDb>,             // Calendrier universel, agendas
    pub miyuclicker_data_dir: PathBuf,     // Repertoire donnees MiyuClicker
}
```

Ouverture depuis la racine workspace :
```rust
let connections = ServiceConnections::open(&base_path)?;
// Cree : central.db, jayxpose.db, jaykonta.db, jayfestival.db, jaykoa.db
```

### 4.2 Pattern d'acces aux donnees dans un composant

```rust
use crate::data::use_service_connections;

#[component]
pub fn MonServiceView() -> Element {
    let conns = use_service_connections();
    
    // Lecture des donnees
    let editions = {
        let db = &conns.read().jayfestival;
        db.editions_list().unwrap_or_default()
    };
    
    rsx! { /* ... */ }
}
```

### 4.3 Feature flags des crates service

Chaque crate service supporte deux modes de persistance :

| Feature | Mode | Usage |
|---------|------|-------|
| `legacy-sqlite` | Acces SQLite direct via `rusqlite` | Migration progressive (actuel) |
| `kindmother-only` | Delegation exclusive via KindMother Service | Production (cible) |

Dans `apps/central/Cargo.toml`, tous les services sont importes avec le feature par defaut (`legacy-sqlite`).

### 4.4 Pattern de la couche data dans un crate service

Chaque crate service (`jayfestival`, `jayxpose`, `jaykonta`, `jaykoa`) suit cette structure :

```
crates/{service}/src/data/
├── mod.rs              # Exports conditionnels (feature flags)
├── types.rs            # Types domaine (structs, enums)
├── kindmother_db.rs    # Implementation SQLite directe (#[cfg(feature = "legacy-sqlite")])
└── kindmother_client_db.rs  # Implementation KindMother (#[cfg(feature = "kindmother-only")])
```

Le `mod.rs` exporte :
- Les types domaine (toujours)
- L'implementation DB selon le feature flag (`JayXposeDb`, `JayFestivalDb`, etc.)
- Un type `DbError` unifie

### 4.5 Types domaine par service

#### JayFestival
- `Profile`, `UserType` (admin, manager, exhibitor, volunteer, visitor)
- `Edition` (evenement), `Organisateur`, `Exposant`
- `EditionExposant` (participation exposant x edition, candidature)
- `Animation` (programme), `BudgetEntry`, `BudgetSummary`

#### JayXpose
- `ExposantProfile` (profil enrichi, contacts, reseaux sociaux, SEO)
- `ProduitCatalogue`, `CategorieProduit`, `ProduitVisuel`
- `DocumentProfessionnel`, `DocumentVersion`, `DocumentPartage`
- `VitrinePage`, `VitrineBlock`, `VitrineTemplate`
- `CmsArticle`, `CmsCategory`
- `ConfidentialiteProfil`, `SyncLog`, `PosStockLink`
- Enums : `DocType`, `DocStatus`, `VitrineStatus`, `Availability`, `Visibility`, `ArticleType`, `ArticleStatus`

#### JayKonta
- `MovementRecord`, `ReminderRecord`, `AuditRecord`
- `InvoiceRecord`, `PaymentRecord`, `QuoteRecord`
- `PurseStats`, `AccountStats` (via DB)

#### JayKoa
- `Agenda`, `TemporalEntry`, `TemporalConflict`
- `CalendarType`, `EntryType`, `EventSource`
- `TemporalStatus`, `UserSettings`

---

## 5. Authentification Central

### 5.1 CentralAuthDb (profils, session, COG vierge)

La base `central.db` gere :

| Table | Contenu |
|-------|---------|
| `central_profiles` | Profils utilisateur (id, email, password_hash, champs enrichis) |
| `cog_meta` | Metadonnees COG (cog_virgin, current_profile_id) |
| `profile_service_refs` | Liens profil -> service (profil_id, service_key, ref_id) |
| `central_profile_saves` | Sauvegardes liees au profil (profil_id, service_key, slot, data BLOB) |

### 5.2 CentralProfile

```rust
pub struct CentralProfile {
    pub id: String,
    pub email: String,
    pub pseudonyme: Option<String>,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub date_naissance: Option<String>,
    pub telephone: Option<String>,
    pub numero_voie: Option<String>,
    pub rue: Option<String>,
    pub code_postal: Option<String>,
    pub ville: Option<String>,
    pub is_admin: bool,  // Premier compte = admin COG
}
```

### 5.3 Liens profil-service

Chaque service peut lier le profil Central a sa table dediee :
```rust
// Lire
let ref_id = auth_db.get_profile_service_ref(profile_id, "lord_of_the_castle")?;
// Ecrire
auth_db.set_profile_service_ref(profile_id, "lord_of_the_castle", Some(&save_id))?;
```

Les sauvegardes sont stockees par (profile_id, service_key, slot) :
```rust
let save_id = auth_db.insert_profile_save(profile_id, "lord_of_the_castle", 0, &data)?;
let save = auth_db.get_profile_save_by_slot(profile_id, "lord_of_the_castle", 0)?;
auth_db.update_profile_save(&save_id, &new_data)?;
```

### 5.4 Mot de passe

Regles de complexite (module `auth::password`) :
- 8 caracteres minimum
- Au moins une lettre (A-Z ou a-z)
- Au moins un chiffre (0-9)
- Au moins un caractere special
- Au moins une majuscule (A-Z)
- Au moins une minuscule (a-z)

### 5.5 Flux d'authentification

1. **COG vierge** (`is_cog_virgin = true`) : ecran Rite d'Entree (3 etapes : Nom -> Email -> Cle)
2. **Profil sauve** : ecran Connexion avec accueil vocal par Miou + pre-remplissage email
3. **Pas de profil sauve** : ecran Connexion classique (email + mot de passe)
4. **Connecte** : affichage du Hub (header + onglets + services)

---

## 6. Patterns UI des Services

### 6.1 Pattern Sidebar + Contenu (standard)

La majorite des services suivent ce layout :

```
┌─────────────────────────────────────────┐
│ [Central Header]                         │
│ [Tab Bar: Accueil | Service1 | ...]     │
├──────────┬──────────────────────────────┤
│ Sidebar  │  Contenu principal           │
│ 200-220px│  flex: 1, padding: 24px      │
│          │  overflow-y: auto            │
│ nav items│                              │
│ avec     │                              │
│ icones   │                              │
└──────────┴──────────────────────────────┘
```

CSS de base :
```rust
// Conteneur
div { style: "display: flex; height: 100%;" }

// Sidebar
aside {
    style: "width: 200px; background: {c.bg_secondary}; border-right: 1px solid {c.border}; padding: 16px 0;",
}

// Contenu
main {
    style: "flex: 1; padding: 24px; overflow-y: auto;",
}
```

### 6.2 Composant SidebarItem (reutilisable)

Le pattern exact du SidebarItem (identique dans JayXpose, JayKonta, JayFestival) :

```rust
#[component]
fn SidebarItem(
    icon: &'static str,
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let bg = if is_active { c.bg_hover } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border = if is_active {
        format!("2px solid {}", c.accent_blue)
    } else {
        "2px solid transparent".to_string()
    };

    rsx! {
        button {
            style: "display: flex; align-items: center; gap: 12px; padding: 10px 16px; background: {bg}; color: {color}; border: none; border-left: {border}; cursor: pointer; font-size: 13px; text-align: left; width: 100%; transition: all 0.2s;",
            onclick: move |evt| onclick.call(evt),
            span { "{icon}" }
            span { "{label}" }
        }
    }
}
```

### 6.3 Composants partages

Les composants suivants sont dupliques dans chaque service. Voici leur specification :

#### StatCard (carte KPI)
```
Background: c.bg_secondary
Border-radius: 8px
Padding: 16px (ou 20px)
Border-left: 3px solid {color}
Label: 12-13px, c.text_secondary
Valeur: 24-28px, font-weight 600, c.text_white
Icon: 20px, opacity 0.6
```

#### Badge
```
Padding: 4px 10px
Background: {color}20 (couleur avec 20% opacite)
Color: {color}
Border-radius: 4px
Font-size: 11px, font-weight 500
```

#### ActionButton
```
Display: flex, align-items center, gap 8px
Padding: 10-12px 16px
Background: accent (c.accent_blue) ou normal (c.bg_hover)
Border: accent (none) ou normal (1px solid c.border)
Border-radius: 4px
Font-size: 13px
Transition: all 0.2s
```

#### PlaceholderSection
```
Display: flex, flex-direction column, center center
Height: 100%
Icon: 64px, margin-bottom 16px, opacity 0.3
Title: 20px, c.text_secondary
Description: 14px, c.text_muted
```

#### EmptyState
```
Identique a PlaceholderSection mais avec height: 300px
Utilise quand aucune donnee en DB
```

### 6.4 Pattern d'etat local d'un service

Chaque service gere son propre etat local via `use_signal` :

```rust
/// Etat local du service.
#[derive(Debug, Clone, Default)]
pub struct MonServiceState {
    pub section: MonServiceSection,
    pub selected_item_id: Option<String>,
    // ...
}

#[component]
pub fn MonServiceView() -> Element {
    let _app_state = use_app_state();
    let state = use_signal(MonServiceState::default);
    
    rsx! {
        div {
            style: "display: flex; height: 100%;",
            MonServiceSidebar { state: state }
            main {
                style: "flex: 1; padding: 24px; overflow-y: auto;",
                match state.read().section {
                    MonServiceSection::Dashboard => rsx! { Dashboard { state: state } },
                    MonServiceSection::Liste => rsx! { Liste { state: state } },
                    // ...
                }
            }
        }
    }
}
```

### 6.5 Pattern multi-role (JayFestival)

JayFestival gere 4 perspectives :
1. **UNC** (utilisateur non connecte) : facade publique avec header propre et footer
2. **Organisateur** : sidebar + dashboard, editions, exposants, budget, programme...
3. **Exposant** : sidebar + dashboard, candidatures, participations, agenda...
4. **Visiteur** : sidebar + catalogue, agenda, billets, reservations...

Le role actif est stocke dans `JayFestivalState.role` et determine la sidebar et le contenu.

### 6.6 Pattern dual-espace (JayKonta)

JayKonta propose un selecteur initial (Purse ou Account) puis affiche la sidebar correspondante :
- **Purse** (JayBudget) : budget personnel
- **Account** (JayKonta) : comptabilite entreprise

Un bouton en bas de sidebar permet de basculer entre les deux espaces.

---

## 7. Routage des Services dans Central

### 7.1 ActiveServiceView (routeur d'onglets)

Le composant `ActiveServiceView` dans `services/mod.rs` effectue le routage :

```rust
match tab.service_id.as_deref() {
    None                      => rsx! { HomeView {} },
    Some("jayxpose")          => rsx! { JayXposeView {} },
    Some("jayfestival")       => rsx! { JayFestivalView {} },
    Some("jaykoa")            => rsx! { JayKoaView {} },
    Some("jaykonta")          => rsx! { JayKontaView {} },
    Some("lord_of_the_castle")=> rsx! { SurvivorEmbed {} },
    Some("miyuclicker")       => rsx! { GameView { service_id: ... } },
    Some(id)                  => rsx! { ServiceView { service_id: id.to_string() } },
}
```

### 7.2 Ajouter un nouveau Service

Etapes pour integrer un nouveau service dans Central :

**1. Crate service** (`crates/mon_service/`)
- Structure standard : `src/data/mod.rs`, `src/data/types.rs`, `src/data/kindmother_db.rs`
- Expose un type `MonServiceDb` avec methode `open(path)`
- Exporte les types domaine publiquement

**2. Dependance dans `apps/central/Cargo.toml`**
```toml
mon_service = { path = "../../crates/mon_service" }
```

**3. Connexion DB dans `data.rs`**
```rust
pub struct ServiceConnections {
    // ... existant ...
    pub mon_service: Arc<MonServiceDb>,
}

impl ServiceConnections {
    pub fn open(base_path: &Path) -> Result<Self, String> {
        // ... existant ...
        let mon_service = MonServiceDb::open(base_path.join("mon_service.db"))
            .map_err(|e| format!("MonService DB: {e}"))?;
        // ...
    }
}
```

**4. Catalogue dans `crates/miyukini-central/src/catalog.rs`**
```rust
pub enum ServiceId {
    // ... existant ...
    MonService,
}
```

**5. ServiceInfo dans `state.rs`**
```rust
fn default_services() -> Vec<ServiceInfo> {
    vec![
        // ... existant ...
        ServiceInfo {
            id: "mon_service".to_string(),
            name: "Mon Service".to_string(),
            description: "Description courte".to_string(),
            icon: "📋".to_string(),
            service_type: ServiceType::InterneCog, // ou SurfaceWeb, InterCog
            is_installed: true,
            is_favorite: false,
            version: "0.1.0".to_string(),
            developer: "Miyukini".to_string(),
        },
    ]
}
```

**6. Vue du service dans `services/`**
- Service simple : un fichier `services/mon_service_view.rs`
- Service complexe : un dossier `services/mon_service/` avec `mod.rs`, `sidebar.rs`, `components.rs`, etc.

**7. Routage dans `services/mod.rs`**
```rust
// Dans l'import :
mod mon_service_view;
pub use mon_service_view::MonServiceView;

// Dans ActiveServiceView :
Some("mon_service") => rsx! { MonServiceView {} },
```

---

## 8. Voix Miou (Audio)

### 8.1 Systeme audio

Le module `audio.rs` gere la lecture de fichiers MP3 en arriere-plan :
- Utilise `rodio` pour la lecture audio
- Les fichiers voix sont dans `voices/fr/`
- Fallback Windows via `cmd /C start /min` si rodio echoue
- La resolution de chemin essaie plusieurs bases (workspace, parent, exe dir)

### 8.2 Fichiers voix utilises

| Ecran | Fichier | Contenu |
|-------|---------|---------|
| Rite d'Entree (etape 1) | `login_new_ask_name.mp3` | Miou demande le nom |
| Rite d'Entree (etape 2) | `login_new_ask_email.mp3` | Miou demande l'email |
| Rite d'Entree (etape 3) | `login_new_ask_password.mp3` | Miou demande le mot de passe |
| Connexion (retour a) | `login_retour_a.mp3` | Phrase d'accueil variante A |
| Connexion (retour b) | `login_retour_b.mp3` | Phrase d'accueil variante B |
| Connexion (retour c) | `login_retour_c.mp3` | Phrase d'accueil variante C |

### 8.3 Usage dans un composant

```rust
use crate::audio;

// Dans un use_effect :
let base = connections.read().miyuclicker_data_dir.clone();
audio::play_voice_background(&base, "mon_fichier.mp3");
```

---

## 9. Configuration Supabase (DB Mere)

### 9.1 SupabaseConfig

```rust
pub struct SupabaseConfig {
    pub project_id: Option<String>,
    pub anon_key: Option<String>,
    pub service_role_key: Option<String>,
    pub mother_db_url: Option<String>,
}
```

Chargee depuis `supabase-catakana.env` (non commite). Les profils locaux sont valides aupres de la DB mere via `validate_profiles_with_mother()`.

---

## 10. Regles et Conventions

### 10.1 Regles de style

1. **Jamais de couleurs en dur** : toujours utiliser `c.xxx` depuis la palette du theme
2. **Toujours passer `theme`** aux fonctions de style
3. **CSS inline** : pas de fichiers CSS separes, tout est genere par Rust
4. **Font par defaut** : Segoe UI (Windows) / system-ui
5. **Transitions** : 0.15s-0.2s pour les interactions
6. **Border-radius** : 4px (standard), 8px (cartes, modals)

### 10.2 Regles de composant

1. **Etat global** : `use_app_state()` pour theme, utilisateur, navigation principale
2. **Etat local** : `use_signal()` pour l'etat propre au service
3. **Connexions DB** : `use_service_connections()` pour acceder aux bases
4. **Props minimales** : passer le `state: Signal<MonState>` plutot que des valeurs individuelles
5. **Icones** : emojis Unicode (pas de librairie d'icones)

### 10.3 Regles de nommage

| Element | Convention | Exemple |
|---------|-----------|---------|
| Vue service | `{Service}View` | `JayXposeView` |
| Sidebar service | `{Service}Sidebar` | `JayKontaSidebar` |
| Etat local | `{Service}State` | `JayFestivalState` |
| Section enum | `{Role}Section` | `OrgSection`, `ExpSection` |
| Composant partage | PascalCase descriptif | `StatCard`, `ActionButton` |
| ID service | snake_case | `"jayxpose"`, `"jayfestival"` |
| Service key (DB) | snake_case | `"lord_of_the_castle"` |

### 10.4 Terminologie Miyukini (rappel)

- **Service** = Capacite percue par l'utilisateur (JayFestival, JayXpose...)
- **Central** = Hub de gestion des Services (cote COG)
- **Portail** = Hub des surfaces web (cote externe)
- **Operateur** = Entite fonctionnelle gouvernee qui execute
- On ne dit **jamais** "application" ou "app" : on dit "Service" ou "Operateur"

---

## 11. Checklist Nouveau Service

- [ ] Crate service cree dans `crates/` avec `data/mod.rs`, `types.rs`, `kindmother_db.rs`
- [ ] `lib.rs` exporte `pub mod data` et les types
- [ ] Dependance ajoutee dans `apps/central/Cargo.toml`
- [ ] `ServiceId` ajoute dans `crates/miyukini-central/src/catalog.rs`
- [ ] `ServiceInfo` ajoute dans `apps/central/src/state.rs` (`default_services()`)
- [ ] DB ajoutee dans `ServiceConnections` (`apps/central/src/data.rs`)
- [ ] Vue creee dans `apps/central/src/services/`
- [ ] Route ajoutee dans `ActiveServiceView` (`services/mod.rs`)
- [ ] Theme utilise via `use_app_state().read().current_theme.palette()`
- [ ] Pas de couleurs en dur dans le code UI
- [ ] Composants reutilisables utilises (StatCard, Badge, SidebarItem...)
