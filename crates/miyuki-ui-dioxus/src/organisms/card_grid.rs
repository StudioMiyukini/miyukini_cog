// @id: MUID-CardGrid @do: card-grid-organism @role: component @layer: 6 @human: miyuk

//! Card grid organism for responsive grid layouts.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     CardGrid { columns: 3, gap: Some(16.0),
//!         Card { title: Some("Card 1".into()), "Content 1" }
//!         Card { title: Some("Card 2".into()), "Content 2" }
//!         Card { title: Some("Card 3".into()), "Content 3" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// Card grid organism.
#[component]
pub fn CardGrid(
    /// Number of columns.
    #[props(default = 3)]
    columns: u32,
    /// Optional gap in pixels.
    #[props(default)]
    gap: Option<f32>,
    /// Grid children (cards).
    children: Element,
) -> Element {
    let gap_val = gap.unwrap_or(16.0);

    let style_str = format!(
        "display: grid; grid-template-columns: repeat({columns}, 1fr); \
         gap: {gap_val}px; width: 100%;"
    );

    rsx! {
        div {
            style: "{style_str}",
            {children}
        }
    }
}
