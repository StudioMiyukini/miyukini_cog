// @id: MUID-Button @do: button-atom @role: component @layer: 6 @human: miyuk

//! Button atom with multiple variants and sizes.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Button {
//!         variant: ButtonVariant::Primary,
//!         size: ButtonSize::Md,
//!         on_click: move |_| { /* handle click */ },
//!         "Click me"
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Button visual variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Primary action button (accent background).
    #[default]
    Primary,
    /// Secondary button (elevated background, border).
    Secondary,
    /// Ghost button (transparent background).
    Ghost,
    /// Danger/destructive action (error color).
    Danger,
    /// Icon-only button (transparent, secondary text).
    IconOnly,
}

/// Button size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Small button.
    Sm,
    /// Medium (default) button.
    #[default]
    Md,
    /// Large button.
    Lg,
}

/// Button component with multiple variants and sizes.
#[component]
pub fn Button(
    /// Visual variant.
    #[props(default)]
    variant: ButtonVariant,
    /// Size variant.
    #[props(default)]
    size: ButtonSize,
    /// Disabled state.
    #[props(default)]
    disabled: bool,
    /// Loading state (shows spinner, disables interaction).
    #[props(default)]
    loading: bool,
    /// Click handler.
    #[props(default)]
    on_click: EventHandler<MouseEvent>,
    /// Button content (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;
    let rad = &theme.radius;

    let (bg_color, text_color, border_css) = match variant {
        ButtonVariant::Primary => (
            p.accent_primary.to_css(),
            p.text_high.to_css(),
            "none".to_string(),
        ),
        ButtonVariant::Secondary => (
            p.bg_overlay.to_css(),
            p.text_primary.to_css(),
            format!("1px solid {}", p.border_default.to_css()),
        ),
        ButtonVariant::Ghost => (
            "transparent".to_string(),
            p.text_primary.to_css(),
            "none".to_string(),
        ),
        ButtonVariant::Danger => (
            p.error.to_css(),
            p.text_high.to_css(),
            "none".to_string(),
        ),
        ButtonVariant::IconOnly => (
            "transparent".to_string(),
            p.text_secondary.to_css(),
            "none".to_string(),
        ),
    };

    let (pad_v, pad_h, font_px) = match size {
        ButtonSize::Sm => (sp.space_1, sp.space_2, theme.typography.sm.px),
        ButtonSize::Md => (sp.space_2, sp.space_4, theme.typography.body.px),
        ButtonSize::Lg => (sp.space_3, sp.space_6, theme.typography.lg.px),
    };

    let is_disabled = disabled || loading;
    let opacity = if is_disabled { "0.5" } else { "1" };
    let cursor = if is_disabled {
        "not-allowed"
    } else {
        "pointer"
    };
    let radius_val = rad.md;
    let gap_val = sp.space_2;

    let style_str = format!(
        "display: inline-flex; align-items: center; justify-content: center; \
         gap: {gap_val}px; padding: {pad_v}px {pad_h}px; \
         background: {bg_color}; color: {text_color}; border: {border_css}; \
         border-radius: {radius_val}px; font-size: {font_px}px; font-weight: 500; \
         cursor: {cursor}; opacity: {opacity}; transition: all 200ms; \
         outline: none; user-select: none;"
    );

    rsx! {
        button {
            style: "{style_str}",
            disabled: is_disabled,
            onclick: move |evt| {
                if !is_disabled {
                    on_click.call(evt);
                }
            },
            {children}
        }
    }
}
