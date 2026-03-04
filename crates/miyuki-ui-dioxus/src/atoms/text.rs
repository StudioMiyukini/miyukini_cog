// @id: MUID-Text @do: text-atom @role: component @layer: 6 @human: miyuk

//! Text atom with semantic variants.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Text { variant: TextVariant::H1, "Hello World" }
//!     Text { "Body text content" }
//! }
//! ```

use dioxus::prelude::*;
use miyuki_ui_tokens::FontWeight;

use crate::context::use_theme;

/// Semantic text level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextVariant {
    /// Main page title (32px bold).
    H1,
    /// Section title (24px bold).
    H2,
    /// Subsection title (18px semibold).
    H3,
    /// Group title (16px semibold).
    H4,
    /// Small heading (14px semibold).
    H5,
    /// Overline heading (12px semibold).
    H6,
    /// Body text (14px regular).
    #[default]
    Body,
    /// Caption text (12px, secondary color).
    Caption,
    /// Overline text (10px, muted, uppercase).
    Overline,
    /// Code/monospace text (12px, mono font).
    Code,
}

/// Text component with semantic variants and configurable weight.
#[component]
pub fn Text(
    /// Semantic variant controlling size, weight, and color.
    #[props(default)]
    variant: TextVariant,
    /// Override the default font weight.
    #[props(default)]
    weight: Option<FontWeight>,
    /// Text content.
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let t = &theme.typography;

    let (size, default_weight, color, tag) = match variant {
        TextVariant::H1 => (t.xxxl.px, FontWeight::Bold, p.text_high.to_css(), "h1"),
        TextVariant::H2 => (t.xxl.px, FontWeight::Bold, p.text_high.to_css(), "h2"),
        TextVariant::H3 => (t.xl.px, FontWeight::SemiBold, p.text_high.to_css(), "h3"),
        TextVariant::H4 => (t.lg.px, FontWeight::SemiBold, p.text_primary.to_css(), "h4"),
        TextVariant::H5 => (
            t.body.px,
            FontWeight::SemiBold,
            p.text_primary.to_css(),
            "h5",
        ),
        TextVariant::H6 => (
            t.sm.px,
            FontWeight::SemiBold,
            p.text_secondary.to_css(),
            "h6",
        ),
        TextVariant::Body => (
            t.body.px,
            FontWeight::Regular,
            p.text_primary.to_css(),
            "p",
        ),
        TextVariant::Caption => (
            t.sm.px,
            FontWeight::Regular,
            p.text_secondary.to_css(),
            "span",
        ),
        TextVariant::Overline => (
            t.xs.px,
            FontWeight::Medium,
            p.text_muted.to_css(),
            "span",
        ),
        TextVariant::Code => (
            t.sm.px,
            FontWeight::Regular,
            p.text_primary.to_css(),
            "code",
        ),
    };

    let w = weight.unwrap_or(default_weight);
    let weight_val = w.value();
    let family = if matches!(variant, TextVariant::Code) {
        t.family_mono.to_css()
    } else {
        t.family_ui.to_css()
    };

    let extra_css = if matches!(variant, TextVariant::Overline) {
        "text-transform: uppercase; letter-spacing: 1px;"
    } else {
        ""
    };

    let style_str = format!(
        "font-size: {size}px; font-weight: {weight_val}; color: {color}; \
         font-family: {family}; margin: 0; {extra_css}"
    );

    // Dioxus 0.6: dynamic tags are not supported, use div with role.
    rsx! {
        div {
            style: "{style_str}",
            role: "{tag}",
            {children}
        }
    }
}
