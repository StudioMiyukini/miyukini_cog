//! Jay Slides — presentations collaboratives.

use dioxus::prelude::*;
use jaybureau_core::SlideDeck;

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Slides")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1280.0, 800.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut deck = use_signal(SlideDeck::new);
    let mut current = use_signal(|| 0usize);

    let slide_count = deck.read().slides.len();
    let current_idx = *current.read();

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; background: #1a1a2e; }}
            .slides-app {{ display: flex; flex-direction: column; height: 100vh; color: #e0e0e0; }}
            .slides-header {{ padding: 10px 16px; background: #f97316; color: white; font-weight: 600; }}
            .slides-main {{ display: flex; flex: 1; min-height: 0; }}
            .slides-list {{ width: 200px; overflow-y: auto; padding: 10px; border-right: 1px solid #2d2d44; background: #13132a; }}
            .slide-thumb {{ width: 100%; aspect-ratio: 16/9; background: white; border: 2px solid transparent; margin-bottom: 8px; border-radius: 4px; cursor: pointer; display: flex; align-items: center; justify-content: center; color: #333; font-size: 12px; }}
            .slide-thumb.active {{ border-color: #f97316; }}
            .slide-stage {{ flex: 1; display: flex; align-items: center; justify-content: center; padding: 40px; }}
            .slide-canvas {{ background: white; width: 100%; max-width: 960px; aspect-ratio: 16/9; border-radius: 8px; box-shadow: 0 4px 20px rgba(0,0,0,0.3); color: #202124; padding: 40px; }}
            .slides-toolbar {{ padding: 8px 16px; background: #13132a; border-top: 1px solid #2d2d44; display: flex; gap: 8px; }}
            .btn {{ padding: 6px 14px; background: #f97316; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; }}
            .btn-ghost {{ background: transparent; border: 1px solid #3d3d55; color: #ccc; }}
        " }
        div {
            class: "slides-app",
            div { class: "slides-header", "🎞 Jay Slides — Présentation sans titre" }
            div {
                class: "slides-main",
                div {
                    class: "slides-list",
                    for (i, _slide) in deck.read().slides.iter().enumerate() {
                        div {
                            key: "{i}",
                            class: if i == current_idx { "slide-thumb active" } else { "slide-thumb" },
                            onclick: move |_| current.set(i),
                            "Slide {i + 1}"
                        }
                    }
                }
                div {
                    class: "slide-stage",
                    div {
                        class: "slide-canvas",
                        h1 { style: "font-size: 48px; color: #202124; margin-bottom: 20px;", "Votre titre ici" }
                        p { style: "font-size: 24px; color: #5f6368;", "Cliquez pour ajouter un sous-titre" }
                    }
                }
            }
            div {
                class: "slides-toolbar",
                button {
                    class: "btn",
                    onclick: move |_| {
                        deck.write().add_slide();
                    },
                    "+ Nouvelle slide"
                }
                button {
                    class: "btn btn-ghost",
                    "Mode présentation"
                }
                span { style: "margin-left: auto; color: #888; font-size: 13px;", "{current_idx + 1} / {slide_count}" }
            }
        }
    }
}
