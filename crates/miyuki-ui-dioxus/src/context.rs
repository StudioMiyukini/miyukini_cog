// @id: MUID-Context @do: theme-context @role: provider @layer: 6 @human: miyuk

//! Theme context and hooks for Dioxus components.
//!
//! Provides a signal-based theme system. Install the provider once in the root
//! component, then read the theme in any child via [`use_theme`] or [`use_palette`].
//!
//! # Example
//!
//! ```rust,ignore
//! use miyuki_ui_dioxus::context::provide_theme;
//! use miyuki_ui_tokens::COG_THEME;
//!
//! fn App() -> Element {
//!     provide_theme(COG_THEME.clone());
//!     rsx! { /* child components can call use_theme() */ }
//! }
//! ```

use dioxus::prelude::*;
use miyuki_ui_tokens::{Palette, UiTheme};

/// Signal holding the active theme.
///
/// Installed at the app root with [`use_context_provider`].
/// All child components access it via [`use_theme`].
#[derive(Clone, Copy)]
pub struct ThemeSignal(pub Signal<UiTheme>);

/// Install the theme provider at the root of the app.
///
/// Call this once in the root component:
/// ```rust,ignore
/// fn App() -> Element {
///     provide_theme(COG_THEME.clone());
///     rsx! { /* ... */ }
/// }
/// ```
pub fn provide_theme(initial: UiTheme) {
    let sig = use_signal(|| initial);
    use_context_provider(|| ThemeSignal(sig));
}

/// Read the current theme from context.
pub fn use_theme() -> UiTheme {
    let ctx = use_context::<ThemeSignal>();
    let theme = ctx.0.read().clone();
    theme
}

/// Read the current palette from context (shortcut).
pub fn use_palette() -> Palette {
    use_theme().palette
}

/// Update the active theme at runtime.
pub fn set_theme(theme: UiTheme) {
    let mut ctx = use_context::<ThemeSignal>();
    ctx.0.set(theme);
}
