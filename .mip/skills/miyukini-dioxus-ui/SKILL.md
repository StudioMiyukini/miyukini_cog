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

## Pieges RSX Dioxus 0.6

### INTERDIT : expressions avec accolades dans les format strings RSX

Le parseur RSX de Dioxus 0.6 ne distingue pas les accolades d'un `if/else` ou d'un `format!()` des accolades de fin d'interpolation `{...}`. Cela provoque des erreurs `Expected Ident or Expression` sur tout le bloc `rsx!`.

```rust
// INTERDIT — nested braces dans string RSX
style: "width: {if active { 24 } else { 8 }}px;"
style: "border: {if ok { \"none\" } else { format!(\"1px solid {}\", c.border) }};"

// CORRECT — extraire en variable AVANT le rsx!
let w = if active { 24 } else { 8 };
let border = if ok { "none".to_string() } else { format!("1px solid {}", c.border) };
rsx! {
    div { style: "width: {w}px;" }
    div { style: "border: {border};" }
}
```

### INTERDIT : named format args dans les text nodes RSX

```rust
// INTERDIT — Dioxus RSX n'est pas format!()
p { "Total : {count}", count = items.len() }

// CORRECT — variable locale
let count = items.len();
rsx! { p { "Total : {count}" } }
```

### INTERDIT : read + set sur le meme signal dans une seule expression

```rust
// INTERDIT — borrow conflict (immutable borrow still alive)
counter.set(*counter.read() + 1);

// CORRECT — lire d'abord, puis muter
let prev = *counter.read();
counter.set(prev + 1);
```

## Regles

1. **Provider unique** a la racine : `AppContext` via `use_context_provider`
2. **Hooks** : `use_app_state()` et `use_service_connections()` partout
3. **Styles** : fonctions `styles::xxx(theme)` ou inline avec `c.palette()`
4. **Pas de CSS externe** : tout en inline via fonctions Rust
5. **Audio non-bloquant** : thread dedie, erreurs loggees
6. **Props** : `#[derive(Props, Clone, PartialEq)]`
7. **Async** : `spawn()` dans handlers, `use_effect` pour effets de bord
8. **RSX format strings** : jamais d'expressions avec accolades (`if`, `match`, `format!`) dans les `"..."` RSX — toujours extraire en variable avant le `rsx!`
9. **Signaux** : jamais `signal.set(*signal.read() + x)` en une ligne — lire d'abord dans un `let`

## References

- **Application** : `apps/central/`
- **Theme** : `apps/central/src/theme.rs`
- **Etat** : `apps/central/src/state.rs`
- **Audio** : `apps/central/src/audio.rs`
