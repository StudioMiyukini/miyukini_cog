// @id: MUID-Divider @do: divider-atom @role: component @layer: 6 @human: miyuk

//! Divider atom for visual separation.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Divider { orientation: Orientation::Horizontal }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Divider orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    /// Horizontal line.
    #[default]
    Horizontal,
    /// Vertical line.
    Vertical,
}

/// Divider line style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerStyle {
    /// Solid line.
    #[default]
    Solid,
    /// Dashed line.
    Dashed,
}

/// Divider component for separating content sections.
#[component]
pub fn Divider(
    /// Line orientation.
    #[props(default)]
    orientation: Orientation,
    /// Line style.
    #[props(default)]
    style_variant: DividerStyle,
) -> Element {
    let theme = use_theme();
    let border_color = theme.palette.border_subtle.to_css();

    let border_style = match style_variant {
        DividerStyle::Solid => "solid",
        DividerStyle::Dashed => "dashed",
    };

    let style_str = match orientation {
        Orientation::Horizontal => format!(
            "width: 100%; height: 0; border: none; \
             border-top: 1px {border_style} {border_color}; \
             margin: 0; flex-shrink: 0;"
        ),
        Orientation::Vertical => format!(
            "height: 100%; width: 0; border: none; \
             border-left: 1px {border_style} {border_color}; \
             margin: 0; flex-shrink: 0; align-self: stretch;"
        ),
    };

    rsx! {
        hr {
            style: "{style_str}",
            role: "separator",
            aria_orientation: if matches!(orientation, Orientation::Vertical) { "vertical" } else { "horizontal" },
        }
    }
}
