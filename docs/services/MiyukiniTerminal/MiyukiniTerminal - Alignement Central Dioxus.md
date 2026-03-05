# MiyukiniTerminal â€” Alignement Central Dioxus

## Contexte

Ce document dÃ©crit le **mapping des patterns Miyukini Central** vers MiyukiniTerminal : AppContext, hooks, thÃ¨me, navigation, composants, et les **diffÃ©rences imposÃ©es par le mobile** (Ã©cran tactile, taille, gestures).

**RÃ©fÃ©rences :**

- [Architecture Technique](./MiyukiniTerminal%20-%20Architecture%20Technique.md)
- [Skill miyukini-dioxus-ui](_index.md)
- Code Central : `apps/central/`

---

## PortÃ©e / Scope

- Patterns Central : AppContext, use_app_state, theme
- Navigation : MainTab vs mobile (bottom nav)
- Composants : Props, Signal, RSX
- DiffÃ©rences mobile : touch, taille, gestures
- RÃ©utilisation thÃ¨me Gaming

---

## 1. Gestion d'Ã©tat : AppContext

### 1.1 Central (rÃ©fÃ©rence)

```rust
pub struct AppContext {
    pub connections: Signal<Arc<ServiceConnections>>,
    pub state: Signal<AppState>,
}

// Provider racine
use_context_provider(|| { ... });

// Hooks
pub fn use_app_state() -> Signal<AppState> {
    use_context::<AppContext>().state
}
```

### 1.2 Terminal (adaptation)

| Ã‰lÃ©ment | Central | Terminal |
|---------|---------|----------|
| **ServiceConnections** | Oui (bases locales) | AdaptÃ© : connexions rÃ©duites (cache, queue, MWS) |
| **AppState** | current_user, main_tab, theme, etc. | current_user (identique), main_tab (adaptÃ©), theme, connection_state |
| **Provider** | use_context_provider | Idem |
| **Hooks** | use_app_state, use_service_connections | Idem ; ajouter use_connection_state |

### 1.3 AppState Terminal (champs additionnels)

```rust
pub struct AppState {
    // HÃ©ritÃ©s de Central
    pub current_user: Option<User>,
    pub current_theme: Theme,
    // SpÃ©cifiques Terminal
    pub parent_cog_id: Option<String>,
    pub cog_id: Option<String>,
    pub connection_state: ConnectionState, // Online, Offline, Degrading
    pub main_tab: MainTabTerminal,
}
```

---

## 2. ThÃ¨me : ThemePalette

### 2.1 RÃ©utilisation

Le thÃ¨me **Gaming** (palette type Steam) est **rÃ©utilisÃ© tel quel** pour la cohÃ©rence visuelle entre Central et Terminal.

```rust
// Copier ou partager depuis apps/central/src/theme.rs
pub enum Theme { Gaming }

pub struct ThemePalette {
    pub bg_main: &'static str,
    pub bg_header: &'static str,
    pub bg_card: &'static str,
    // ... (identique Central)
}
```

### 2.2 Adaptations mobile

| Aspect | Central (desktop) | Terminal (mobile) |
|--------|-------------------|-------------------|
| **UnitÃ©** | px | dp (density-independent) ou px |
| **Touch** | Clic souris | Zone tactile min 44Ã—44 pt |
| **Contraste** | Idem | Renforcer si usage extÃ©rieur |
| **Espacement** | Standard | Augmenter pour touch |

### 2.3 Styles

Les fonctions `styles::xxx(theme)` de Central peuvent Ãªtre rÃ©utilisÃ©es ; ajouter des variantes mobile :

```rust
pub mod styles {
    pub fn button(theme: Theme) -> String { ... }
    pub fn button_mobile(theme: Theme) -> String {
        // Min-height: 44px, padding accru
    }
}
```

---

## 3. Navigation

### 3.1 Central

```rust
pub enum MainTab {
    Salon,
    Bibliotheque,
    Communaute,
    Miyukini,
}
```

Conditionnel dans App : `match state.read().main_tab { ... }`

### 3.2 Terminal

```rust
pub enum MainTabTerminal {
    Salon,      // Services du parent
    Parametres,
    Profil,
}
```

**Pas de :** BibliothÃ¨que (services complets) ; CommunautÃ© (simplifiÃ© ou futur).

### 3.3 Bottom Navigation

Sur mobile : **barre de navigation en bas** (standard Android/iOS).

```rust
// Structure
rsx! {
    main { /* contenu Ã©cran actif */ }
    nav { style: "position: fixed; bottom: 0; ...",
        TabButton { tab: MainTabTerminal::Salon, ... }
        TabButton { tab: MainTabTerminal::Parametres, ... }
        TabButton { tab: MainTabTerminal::Profil, ... }
    }
}
```

---

## 4. Composants : Props, Signal, RSX

### 4.1 Pattern identique

```rust
#[derive(Props, Clone, PartialEq)]
pub struct ServiceCardProps {
    pub title: String,
    pub subtitle: Option<String>,
    pub on_click: EventHandler<()>,
}

#[component]
pub fn ServiceCard(props: ServiceCardProps) -> Element {
    let theme = use_app_state().read().current_theme;
    let c = theme.palette();
    rsx! {
        div {
            style: "background: {c.bg_card}; ...",
            onclick: move |_| props.on_click.call(()),
            h3 { "{props.title}" }
            if let Some(s) = &props.subtitle {
                p { "{s}" }
            }
        }
    }
}
```

### 4.2 Signaux locaux

```rust
let mut value = use_signal(|| String::new());
let mut expanded = use_signal(|| false);
```

### 4.3 Handlers async

```rust
onclick: move |_| {
    spawn(async move {
        let result = service.fetch().await;
        state.write().cache = result;
    });
}
```

---

## 5. DiffÃ©rences mobile

### 5.1 Ã‰cran tactile

| RÃ¨gle | Description |
|-------|-------------|
| **Taille cible** | Min 44Ã—44 pt (ou dp) pour boutons/liens |
| **Espacement** | Padding accru entre Ã©lÃ©ments cliquables |
| **Pas de hover** | Remplacer par : focus, Ã©tat actif |
| **Long press** | Possible pour actions contextuelles |

### 5.2 Gestures

| Gesture | Usage possible |
|---------|----------------|
| **Tap** | SÃ©lection, navigation |
| **Swipe** | Refresh, navigation latÃ©rale |
| **Pinch** | Zoom (si applicable) |
| **Pull-to-refresh** | Synchronisation |

### 5.3 Taille Ã©cran

- **Width** : 360â€“480 dp typique (phone)
- **Orientation** : Portrait prioritaire ; paysage optionnel
- **Safe area** : Notch, barre systÃ¨me ; prÃ©voir margins

### 5.4 Performance

- **WebView** : Limiter re-renders ; Ã©viter listes trÃ¨s longues sans virtualisation
- **Batterie** : Sync par batch ; pas de polling continu

---

## 6. Mapping Ã©crans Central â†’ Terminal

| Central | Terminal | Correspondance |
|---------|----------|----------------|
| RiteEntree | â€” | Non (Terminal = enfant existant) |
| Connexion | Liaison | Token / QR au lieu de login |
| Salon | Salon | Liste services (rÃ©duite) |
| BibliothÃ¨que | â€” | Non (ou simplifiÃ©) |
| CommunautÃ© | â€” | Futur : dÃ©couverte COGs |
| Miyukini (paramÃ¨tres) | ParamÃ¨tres | AdaptÃ© |
| ProfileWindow | Profil | AdaptÃ© |

---

## 7. RÃ©utilisation concrÃ¨te

### 7.1 Fichiers Ã  rÃ©utiliser / adapter

| Fichier Central | Action Terminal |
|-----------------|----------------|
| `theme.rs` | Copier ; ajouter variantes mobile |
| `state.rs` (partie) | Adapter AppState ; garder structure |
| Composants (cards, buttons) | RÃ©utiliser ; augmenter zone touch |
| `styles` | RÃ©utiliser ; ajouter `_mobile` si besoin |

### 7.2 Pas de dÃ©pendance directe

Terminal et Central sont des **applications sÃ©parÃ©es**. Pas de `path = "../../apps/central"` dans Cargo.toml. La rÃ©utilisation se fait par **copie** ou extraction en crate partagÃ© (ex. `miyukini-ui-common`) si pertinent.

---

## 8. RÃ©fÃ©rences

- [Skill miyukini-dioxus-ui](_index.md)
- [Spec Design System Mobile](./MiyukiniTerminal%20-%20Spec%20Design%20System%20Mobile.md)
- [Spec Ecrans et Navigation](./MiyukiniTerminal%20-%20Spec%20Ecrans%20et%20Navigation.md)
- Code : `apps/central/src/`

