//! Jay Sheets — tableur collaboratif.

use dioxus::prelude::*;
use jaybureau_core::{Cell, CellValue, SheetData};

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Sheets")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1200.0, 800.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

const ROWS: u32 = 50;
const COLS: u32 = 20;

#[component]
fn App() -> Element {
    let mut sheet = use_signal(|| SheetData::new("Feuille1", ROWS, COLS));
    let mut selected = use_signal(|| (0u32, 0u32));
    let mut editing_value = use_signal(String::new);

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; }}
            .sheet-app {{ display: flex; flex-direction: column; height: 100vh; }}
            .sheet-header {{ padding: 10px 16px; background: #16a34a; color: white; font-weight: 600; }}
            .sheet-toolbar {{ display: flex; gap: 8px; padding: 8px 16px; background: white; border-bottom: 1px solid #e0e0e8; }}
            .sheet-toolbar input {{ padding: 6px 10px; border: 1px solid #e0e0e8; border-radius: 4px; flex: 1; max-width: 400px; font-family: monospace; }}
            .sheet-grid-wrap {{ flex: 1; overflow: auto; background: white; }}
            table.sheet-grid {{ border-collapse: collapse; }}
            .sheet-grid th, .sheet-grid td {{ border: 1px solid #e0e0e8; padding: 4px 8px; font-size: 13px; min-width: 80px; text-align: left; }}
            .sheet-grid th {{ background: #f5f5fa; font-weight: 600; position: sticky; top: 0; color: #666; }}
            .sheet-grid td.selected {{ background: #e0f2fe; outline: 2px solid #16a34a; }}
            .sheet-grid td.header-col {{ background: #f5f5fa; font-weight: 600; color: #666; text-align: center; position: sticky; left: 0; }}
        " }
        div {
            class: "sheet-app",
            div { class: "sheet-header", "📊 Jay Sheets — Feuille sans titre" }
            div {
                class: "sheet-toolbar",
                span {
                    style: "padding: 6px 12px; background: #f5f5fa; border-radius: 4px; font-family: monospace; font-weight: 600;",
                    "{SheetData::col_name(selected.read().1)}{selected.read().0 + 1}"
                }
                input {
                    r#type: "text",
                    placeholder: "Entrez une valeur ou une formule (=SOMME(A1:A10))",
                    value: "{editing_value}",
                    oninput: move |evt| editing_value.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == dioxus::prelude::Key::Enter {
                            let val = editing_value.read().clone();
                            let (row, col) = *selected.read();
                            let parsed = if val.starts_with('=') {
                                CellValue::Text(val.clone())
                            } else if let Ok(n) = val.parse::<f64>() {
                                CellValue::Number(n)
                            } else if val.is_empty() {
                                CellValue::Empty
                            } else {
                                CellValue::Text(val.clone())
                            };
                            let cell = Cell {
                                value: parsed,
                                formula: if val.starts_with('=') { Some(val) } else { None },
                                ..Default::default()
                            };
                            sheet.write().set(row, col, cell);
                            editing_value.set(String::new());
                        }
                    },
                }
            }
            div {
                class: "sheet-grid-wrap",
                table {
                    class: "sheet-grid",
                    thead {
                        tr {
                            th { "" }
                            for c in 0..COLS {
                                th { "{SheetData::col_name(c)}" }
                            }
                        }
                    }
                    tbody {
                        for r in 0..ROWS {
                            tr {
                                td { class: "header-col", "{r + 1}" }
                                for c in 0..COLS {
                                    {
                                        let is_selected = *selected.read() == (r, c);
                                        let cell_value = sheet.read().get(r, c);
                                        let display = match &cell_value.value {
                                            CellValue::Empty => String::new(),
                                            CellValue::Text(t) => t.clone(),
                                            CellValue::Number(n) => format!("{n}"),
                                            CellValue::Bool(b) => b.to_string(),
                                            CellValue::Date(d) => d.format("%Y-%m-%d").to_string(),
                                            CellValue::Error(e) => format!("#ERR: {e}"),
                                        };
                                        rsx! {
                                            td {
                                                class: if is_selected { "selected" } else { "" },
                                                onclick: move |_| {
                                                    selected.set((r, c));
                                                    let v = sheet.read().get(r, c);
                                                    let val_str = v.formula.clone().unwrap_or_else(|| match &v.value {
                                                        CellValue::Empty => String::new(),
                                                        CellValue::Text(t) => t.clone(),
                                                        CellValue::Number(n) => format!("{n}"),
                                                        _ => String::new(),
                                                    });
                                                    editing_value.set(val_str);
                                                },
                                                "{display}"
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
}
