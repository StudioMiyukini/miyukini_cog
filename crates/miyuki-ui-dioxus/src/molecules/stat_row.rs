// @id: MUID-StatRow @do: stat-row-molecule @role: component @layer: 6 @human: miyuk

//! Stat row molecule displaying a label-value pair with optional trend indicator.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     StatRow {
//!         label: "Revenue".to_string(),
//!         value: "$12,340".to_string(),
//!         indicator: Some(StatIndicator::Up),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Trend direction indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatIndicator {
    /// Positive/upward trend.
    Up,
    /// Negative/downward trend.
    Down,
    /// Neutral/unchanged.
    Neutral,
}

/// Stat row molecule.
#[component]
pub fn StatRow(
    /// Label text.
    label: String,
    /// Value text.
    value: String,
    /// Optional trend indicator.
    #[props(default)]
    indicator: Option<StatIndicator>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let label_color = p.text_secondary.to_css();
    let value_color = p.text_high.to_css();

    let (indicator_char, indicator_color) = match indicator {
        Some(StatIndicator::Up) => ("^", p.success.to_css()),
        Some(StatIndicator::Down) => ("v", p.error.to_css()),
        Some(StatIndicator::Neutral) => ("-", p.text_muted.to_css()),
        None => ("", String::new()),
    };
    let has_indicator = indicator.is_some();

    let row_style = "display: flex; align-items: center; justify-content: space-between; \
                     padding: 8px 0;";

    rsx! {
        div { style: "{row_style}",
            span { style: "font-size: 14px; color: {label_color};", "{label}" }
            div { style: "display: flex; align-items: center; gap: 6px;",
                span { style: "font-size: 16px; font-weight: 600; color: {value_color};", "{value}" }
                if has_indicator {
                    span { style: "font-size: 14px; color: {indicator_color}; font-weight: 600;", "{indicator_char}" }
                }
            }
        }
    }
}
