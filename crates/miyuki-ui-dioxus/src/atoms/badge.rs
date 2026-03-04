// @id: MUID-Badge @do: badge-atom @role: component @layer: 6 @human: miyuk

//! Badge atom for labels, statuses, and counters.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Badge { label: "Active".to_string(), variant: BadgeVariant::Success }
//!     Badge { label: "3".to_string(), variant: BadgeVariant::Info, size: BadgeSize::Sm }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Badge color variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    /// Default neutral badge.
    #[default]
    Default,
    /// Success/positive badge.
    Success,
    /// Warning badge.
    Warning,
    /// Error/danger badge.
    Error,
    /// Informational badge.
    Info,
    /// Sakura accent badge (Miyukini brand).
    Sakura,
}

/// Badge size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeSize {
    /// Small badge.
    Sm,
    /// Medium (default) badge.
    #[default]
    Md,
}

/// Badge component displaying a short label with semantic coloring.
#[component]
pub fn Badge(
    /// Text content of the badge.
    label: String,
    /// Color variant.
    #[props(default)]
    variant: BadgeVariant,
    /// Size variant.
    #[props(default)]
    size: BadgeSize,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let rad = &theme.radius;

    let (bg_color, text_color) = match variant {
        BadgeVariant::Default => (p.bg_overlay.to_css(), p.text_primary.to_css()),
        BadgeVariant::Success => (p.success_subtle.to_css(), p.success.to_css()),
        BadgeVariant::Warning => (p.warning_subtle.to_css(), p.warning.to_css()),
        BadgeVariant::Error => (p.error_subtle.to_css(), p.error.to_css()),
        BadgeVariant::Info => (p.info_subtle.to_css(), p.info.to_css()),
        BadgeVariant::Sakura => (
            p.accent_secondary_subtle.to_css(),
            p.accent_secondary.to_css(),
        ),
    };

    let (pad_v, pad_h, font_px) = match size {
        BadgeSize::Sm => (2.0, 6.0, theme.typography.xs.px),
        BadgeSize::Md => (4.0, 8.0, theme.typography.sm.px),
    };

    let radius_val = rad.sm;

    let style_str = format!(
        "display: inline-flex; align-items: center; \
         padding: {pad_v}px {pad_h}px; \
         background: {bg_color}; color: {text_color}; \
         border-radius: {radius_val}px; font-size: {font_px}px; \
         font-weight: 500; white-space: nowrap; line-height: 1;"
    );

    rsx! {
        span {
            style: "{style_str}",
            "{label}"
        }
    }
}
