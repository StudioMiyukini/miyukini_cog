// @id: MUID-DataTable @do: data-table-organism @role: component @layer: 6 @human: miyuk

//! Data table organism for tabular data display.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     DataTable {
//!         columns: vec![
//!             Column { id: "name".into(), label: "Name".into(), width: None },
//!             Column { id: "value".into(), label: "Value".into(), width: Some("100px".into()) },
//!         ],
//!         rows: vec![
//!             Row { id: "1".into(), cells: vec!["Alice".into(), "42".into()] },
//!         ],
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Column definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    /// Unique identifier.
    pub id: String,
    /// Header label.
    pub label: String,
    /// Optional CSS width.
    pub width: Option<String>,
}

/// Row data.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// Unique row identifier.
    pub id: String,
    /// Cell values (must match column count).
    pub cells: Vec<String>,
}

/// Sort event data.
#[derive(Debug, Clone, PartialEq)]
pub struct SortEvent {
    /// Column id that was sorted.
    pub column_id: String,
    /// Sort direction.
    pub ascending: bool,
}

/// Data table organism.
#[component]
pub fn DataTable(
    /// Column definitions.
    columns: Vec<Column>,
    /// Row data.
    #[props(default)]
    rows: Vec<Row>,
    /// Whether columns are sortable.
    #[props(default)]
    sortable: bool,
    /// Sort handler.
    #[props(default)]
    on_sort: Option<EventHandler<SortEvent>>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_surface.to_css();
    let header_bg = p.bg_secondary.to_css();
    let border_color = p.border_subtle.to_css();
    let text_color = p.text_primary.to_css();
    let header_color = p.text_secondary.to_css();
    let radius = theme.radius.lg;

    let table_style = format!(
        "width: 100%; border-collapse: collapse; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {radius}px; overflow: hidden;"
    );

    let header_style = format!("background: {header_bg}; border-bottom: 1px solid {border_color};");

    rsx! {
        table { style: "{table_style}",
            thead {
                tr { style: "{header_style}",
                    for col in columns.iter() {
                        {
                            let width_css = col.width.as_deref().unwrap_or("auto");
                            let cursor = if sortable { "pointer" } else { "default" };
                            let col_id = col.id.clone();
                            let th_style = format!(
                                "padding: 8px 12px; text-align: left; \
                                 font-size: 12px; font-weight: 600; \
                                 color: {header_color}; width: {width_css}; \
                                 cursor: {cursor}; user-select: none;"
                            );
                            rsx! {
                                th {
                                    style: "{th_style}",
                                    onclick: move |_| {
                                        if sortable {
                                            if let Some(ref handler) = on_sort {
                                                handler.call(SortEvent {
                                                    column_id: col_id.clone(),
                                                    ascending: true,
                                                });
                                            }
                                        }
                                    },
                                    "{col.label}"
                                }
                            }
                        }
                    }
                }
            }
            tbody {
                for row in rows.iter() {
                    {
                        let row_style = format!(
                            "border-bottom: 1px solid {border_color}; transition: background 100ms;"
                        );
                        rsx! {
                            tr { style: "{row_style}",
                                for cell in row.cells.iter() {
                                    {
                                        let td_style = format!(
                                            "padding: 8px 12px; font-size: 13px; color: {text_color};"
                                        );
                                        rsx! {
                                            td { style: "{td_style}", "{cell}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
