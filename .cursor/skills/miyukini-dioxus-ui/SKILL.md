---
name: miyukini-dioxus-ui
description: Patterns UI Dioxus pour Miyukini Central (apps/central). Gestion etat (AppContext, use_app_state), navigation (MainTab, ecrans conditionnels), theme (ThemePalette, styles), integration services (ServiceConnections), audio Miou, composants (Props, Signal, Element, RSX). Utiliser quand on cree ou modifie un ecran/composant dans apps/central, quand on travaille sur le theme, la navigation, ou l'integration d'un service dans l'UI.
---

# Dioxus UI — Miyukini Central

## Architecture

```
apps/central/src/
├── main.rs           # Point d'entree Dioxus
├── app.rs            # Composant App racine + context providers
├── state.rs          # AppState, AppContext, hooks
├── audio.rs          # Systeme audio Miou
├── data.rs           # ServiceConnections
├── theme.rs          # Theme, ThemePalette, module styles
├── screens/          # Ecrans plein page
│   ├── connexion.rs    # Retour habitant connu
│   ├── rite_entree.rs  # Creation premier compte
│   └── profile_window.rs # Modal profil
├── services/         # Vues services integrees
│   ├── home.rs         # Page d'accueil
│   ├── jayxpose/       # Service JayXpose
│   └── miyukiniwatch/  # Service MiyukiniWatch
├── components/       # Composants reutilisables
│   └── service_card.rs
└── miou/             # Systeme Miou (bot assistant)
```

## Gestion d'etat

Provider unique a la racine :

```rust
pub struct AppContext {
    pub connections: Signal<Arc<ServiceConnections>>,
    pub state: Signal<AppState>,
}

// Hooks reutilisables
pub fn use_app_state() -> Signal<AppState> {
    use_context::<AppContext>().state
}
pub fn use_service_connections() -> Signal<Arc<ServiceConnections>> {
    use_context::<AppContext>().connections
}
```

Acces dans les composants :

```rust
let mut state = use_app_state();
let theme = state.read().current_theme;
// Modification
state.write().main_tab = MainTab::Salon;
```

## Navigation

Conditionnelle dans `App` :

```rust
rsx! {
    if is_cog_virgin {
        RiteEntree {}
    } else if !has_user {
        Connexion {}
    } else {
        Header {}
        main {
            match state.read().main_tab {
                MainTab::Salon => rsx! { ActiveServiceView {} },
                MainTab::Bibliotheque => rsx! { LibraryView {} },
                MainTab::Communaute => rsx! { MwsNetworkView {} },
                MainTab::Miyukini => rsx! { SettingsView {} },
            }
        }
    }
}
```

Changement d'onglet :

```rust
onclick: move |_| { state.write().main_tab = MainTab::Salon; }
```

## Theme et styles

```rust
pub enum Theme { Gaming }

pub struct ThemePalette {
    pub bg_main: &'static str,
    pub bg_header: &'static str,
    pub text_primary: &'static str,
    pub accent_blue: &'static str,
    // ... 20+ couleurs
}

// Fonctions de style
pub mod styles {
    pub fn main_container(theme: Theme) -> String { ... }
    pub fn header(theme: Theme) -> String { ... }
    pub fn nav_tab(theme: Theme, active: bool) -> String { ... }
}
```

Utilisation :

```rust
let c = theme.palette();
rsx! {
    div { style: "{styles::main_container(theme)}" }
    span { style: "color: {c.text_primary};" }
}
```

## Composants standard

```rust
#[derive(Props, Clone, PartialEq)]
pub struct MyProps {
    pub required: String,
    #[props(default = false)]
    pub optional: bool,
}

#[component]
pub fn MyComponent(props: MyProps) -> Element {
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();

    rsx! {
        div { style: "{styles::card(theme)}",
            "{props.required}"
        }
    }
}
```

## Patterns recurrents

### Signaux locaux
```rust
let mut value = use_signal(String::new);
let mut toggle = use_signal(|| false);
```

### Handlers async
```rust
onclick: move |_| {
    spawn(async move {
        let result = auth_db.sign_in(&email, &password).await;
        state.write().user = Some(result);
    });
}
```

### Effets (use_effect)
```rust
use_effect(move || {
    if condition {
        audio::play_voice_background(&base, "welcome.mp3");
    }
});
```

### Modal overlay
```rust
div {
    style: "{styles::overlay_backdrop(theme)}",
    onclick: on_close,
    div {
        style: "{styles::modal_card(theme)}",
        onclick: move |evt| evt.stop_propagation(),
        // Contenu
    }
}
```

### RSX conditionnel
```rust
rsx! {
    if condition { div { "Oui" } } else { div { "Non" } }
    match value {
        A => rsx! { /* ... */ },
        B => rsx! { /* ... */ },
    }
}
```

## Integration services

```rust
let connections = use_service_connections();
let db = connections.read().jayxpose.clone();

// Async dans handler
spawn(async move {
    let items = db.list_items()?;
    // ...
});
```

## Audio Miou

```rust
// Lecture non-bloquante
audio::play_voice_background(&base_path, "filename.mp3");
audio::play_tts_background("Texte a lire");

// Resolution multi-bases
let path = audio::resolve_voice_path(&base, "subdir/file.mp3");
```

## Regles

1. **Provider unique** a la racine : `AppContext` via `use_context_provider`
2. **Hooks** : `use_app_state()` et `use_service_connections()` partout
3. **Styles** : fonctions `styles::xxx(theme)` ou inline avec `c.palette()`
4. **Pas de CSS externe** : tout en inline via fonctions Rust
5. **Audio non-bloquant** : thread dedie, erreurs loggees
6. **Props** : `#[derive(Props, Clone, PartialEq)]`
7. **Async** : `spawn()` dans handlers, `use_effect` pour effets de bord

## References

- **Application** : `apps/central/`
- **Theme** : `apps/central/src/theme.rs`
- **Etat** : `apps/central/src/state.rs`
- **Audio** : `apps/central/src/audio.rs`
