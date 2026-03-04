// @id: MUID-Tooltip @do: tooltip-atom @role: component @layer: 6 @human: miyuk

//! Tooltip atom displaying contextual information on hover.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Tooltip {
//!         content: "More information here".to_string(),
//!         position: TooltipPosition::Top,
//!         Icon { name: "?".to_string() }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Tooltip display position relative to the trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    /// Above the trigger.
    #[default]
    Top,
    /// Below the trigger.
    Bottom,
    /// Left of the trigger.
    Left,
    /// Right of the trigger.
    Right,
}

/// Tooltip component wrapping its children with a hover popup.
#[component]
pub fn Tooltip(
    /// Tooltip text content.
    content: String,
    /// Position relative to trigger.
    #[props(default)]
    position: TooltipPosition,
    /// The trigger element.
    children: Element,
) -> Element {
    let mut visible = use_signal(|| false);
    let theme = use_theme();
    let p = &theme.palette;
    let z = theme.z_index.tooltip;

    let bg_color = p.bg_elevated.to_css();
    let text_color = p.text_primary.to_css();
    let border_color = p.border_default.to_css();
    let radius = theme.radius.md;
    let font_px = theme.typography.sm.px;

    let is_visible = *visible.read();
    let display = if is_visible { "block" } else { "none" };

    let (pos_css, transform_css) = match position {
        TooltipPosition::Top => (
            "bottom: 100%; left: 50%; margin-bottom: 6px;",
            "transform: translateX(-50%);",
        ),
        TooltipPosition::Bottom => (
            "top: 100%; left: 50%; margin-top: 6px;",
            "transform: translateX(-50%);",
        ),
        TooltipPosition::Left => (
            "right: 100%; top: 50%; margin-right: 6px;",
            "transform: translateY(-50%);",
        ),
        TooltipPosition::Right => (
            "left: 100%; top: 50%; margin-left: 6px;",
            "transform: translateY(-50%);",
        ),
    };

    let tooltip_style = format!(
        "position: absolute; {pos_css} {transform_css} \
         display: {display}; \
         background: {bg_color}; color: {text_color}; \
         border: 1px solid {border_color}; border-radius: {radius}px; \
         padding: 4px 8px; font-size: {font_px}px; \
         white-space: nowrap; z-index: {z}; \
         pointer-events: none;"
    );

    rsx! {
        div {
            style: "position: relative; display: inline-flex;",
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            {children}
            div {
                style: "{tooltip_style}",
                role: "tooltip",
                "{content}"
            }
        }
    }
}
