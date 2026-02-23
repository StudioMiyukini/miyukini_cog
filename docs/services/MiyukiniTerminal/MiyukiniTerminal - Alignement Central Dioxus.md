# MiyukiniTerminal — Alignement Central Dioxus

## Contexte

Ce document décrit le **mapping des patterns Miyukini Central** vers MiyukiniTerminal : AppContext, hooks, thème, navigation, composants, et les **différences imposées par le mobile** (écran tactile, taille, gestures).

**Références :**

- [Architecture Technique](./MiyukiniTerminal%20-%20Architecture%20Technique.md)
- [Skill miyukini-dioxus-ui](.cursor/skills/miyukini-dioxus-ui/SKILL.md)
- Code Central : `apps/central/`

---

## Portée / Scope

- Patterns Central : AppContext, use_app_state, theme
- Navigation : MainTab vs mobile (bottom nav)
- Composants : Props, Signal, RSX
- Différences mobile : touch, taille, gestures
- Réutilisation thème Gaming

---

## 1. Gestion d'état : AppContext

### 1.1 Central (référence)

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

| Élément | Central | Terminal |
|---------|---------|----------|
| **ServiceConnections** | Oui (bases locales) | Adapté : connexions réduites (cache, queue, MWS) |
| **AppState** | current_user, main_tab, theme, etc. | current_user (identique), main_tab (adapté), theme, connection_state |
| **Provider** | use_context_provider | Idem |
| **Hooks** | use_app_state, use_service_connections | Idem ; ajouter use_connection_state |

### 1.3 AppState Terminal (champs additionnels)

```rust
pub struct AppState {
    // Hérités de Central
    pub current_user: Option<User>,
    pub current_theme: Theme,
    // Spécifiques Terminal
    pub parent_cog_id: Option<String>,
    pub cog_id: Option<String>,
    pub connection_state: ConnectionState, // Online, Offline, Degrading
    pub main_tab: MainTabTerminal,
}
```

---

## 2. Thème : ThemePalette

### 2.1 Réutilisation

Le thème **Gaming** (palette type Steam) est **réutilisé tel quel** pour la cohérence visuelle entre Central et Terminal.

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
| **Unité** | px | dp (density-independent) ou px |
| **Touch** | Clic souris | Zone tactile min 44×44 pt |
| **Contraste** | Idem | Renforcer si usage extérieur |
| **Espacement** | Standard | Augmenter pour touch |

### 2.3 Styles

Les fonctions `styles::xxx(theme)` de Central peuvent être réutilisées ; ajouter des variantes mobile :

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

**Pas de :** Bibliothèque (services complets) ; Communauté (simplifié ou futur).

### 3.3 Bottom Navigation

Sur mobile : **barre de navigation en bas** (standard Android/iOS).

```rust
// Structure
rsx! {
    main { /* contenu écran actif */ }
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

## 5. Différences mobile

### 5.1 Écran tactile

| Règle | Description |
|-------|-------------|
| **Taille cible** | Min 44×44 pt (ou dp) pour boutons/liens |
| **Espacement** | Padding accru entre éléments cliquables |
| **Pas de hover** | Remplacer par : focus, état actif |
| **Long press** | Possible pour actions contextuelles |

### 5.2 Gestures

| Gesture | Usage possible |
|---------|----------------|
| **Tap** | Sélection, navigation |
| **Swipe** | Refresh, navigation latérale |
| **Pinch** | Zoom (si applicable) |
| **Pull-to-refresh** | Synchronisation |

### 5.3 Taille écran

- **Width** : 360–480 dp typique (phone)
- **Orientation** : Portrait prioritaire ; paysage optionnel
- **Safe area** : Notch, barre système ; prévoir margins

### 5.4 Performance

- **WebView** : Limiter re-renders ; éviter listes très longues sans virtualisation
- **Batterie** : Sync par batch ; pas de polling continu

---

## 6. Mapping écrans Central → Terminal

| Central | Terminal | Correspondance |
|---------|----------|----------------|
| RiteEntree | — | Non (Terminal = enfant existant) |
| Connexion | Liaison | Token / QR au lieu de login |
| Salon | Salon | Liste services (réduite) |
| Bibliothèque | — | Non (ou simplifié) |
| Communauté | — | Futur : découverte COGs |
| Miyukini (paramètres) | Paramètres | Adapté |
| ProfileWindow | Profil | Adapté |

---

## 7. Réutilisation concrète

### 7.1 Fichiers à réutiliser / adapter

| Fichier Central | Action Terminal |
|-----------------|----------------|
| `theme.rs` | Copier ; ajouter variantes mobile |
| `state.rs` (partie) | Adapter AppState ; garder structure |
| Composants (cards, buttons) | Réutiliser ; augmenter zone touch |
| `styles` | Réutiliser ; ajouter `_mobile` si besoin |

### 7.2 Pas de dépendance directe

Terminal et Central sont des **applications séparées**. Pas de `path = "../../apps/central"` dans Cargo.toml. La réutilisation se fait par **copie** ou extraction en crate partagé (ex. `miyukini-ui-common`) si pertinent.

---

## 8. Références

- [Skill miyukini-dioxus-ui](.cursor/skills/miyukini-dioxus-ui/SKILL.md)
- [Spec Design System Mobile](./MiyukiniTerminal%20-%20Spec%20Design%20System%20Mobile.md)
- [Spec Ecrans et Navigation](./MiyukiniTerminal%20-%20Spec%20Ecrans%20et%20Navigation.md)
- Code : `apps/central/src/`
