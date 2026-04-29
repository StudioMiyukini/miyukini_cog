//! Jay Réunion — visioconférence collaborative.
//!
//! MVP : interface de salle (lobby + vue pre-call). La vidéo WebRTC sera
//! ajoutée dans une phase ultérieure — ce binaire fournit déjà la structure
//! de l'app et le modèle de données (crate jaybureau-core::Meeting).

use dioxus::prelude::*;
use jaybureau_core::Meeting;

fn main() {
    tracing_subscriber::fmt().init();
    let cfg = dioxus::desktop::Config::new().with_window(
        dioxus::desktop::WindowBuilder::new()
            .with_title("Jay Réunion")
            .with_inner_size(dioxus::desktop::LogicalSize::new(1200.0, 800.0)),
    );
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Lobby,
    InMeeting,
}

#[component]
fn App() -> Element {
    let mut screen = use_signal(|| Screen::Lobby);
    let mut meeting = use_signal(|| Meeting::new("Nouvelle réunion", "me".to_string()));
    let mut display_name = use_signal(String::new);

    let current = screen.read().clone();

    rsx! {
        style { "
            * {{ margin: 0; padding: 0; box-sizing: border-box; }}
            body, html {{ height: 100%; font-family: 'Segoe UI', sans-serif; background: #202124; color: white; }}
            .meet-app {{ display: flex; flex-direction: column; height: 100vh; }}
            .meet-lobby {{ flex: 1; display: flex; align-items: center; justify-content: center; }}
            .lobby-card {{ background: #2d2d44; border-radius: 16px; padding: 40px; max-width: 480px; width: 90%; text-align: center; }}
            .lobby-card h1 {{ font-size: 28px; margin-bottom: 12px; color: #22c55e; }}
            .lobby-preview {{ width: 100%; aspect-ratio: 16/9; background: #16162a; border-radius: 12px; margin: 20px 0; display: flex; align-items: center; justify-content: center; color: #666; font-size: 14px; }}
            .lobby-input {{ width: 100%; padding: 12px 16px; background: #16162a; border: 1px solid #3d3d55; color: white; border-radius: 8px; font-size: 15px; margin-bottom: 16px; }}
            .btn-primary {{ width: 100%; padding: 14px; background: #22c55e; color: white; border: none; border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer; }}
            .meet-stage {{ flex: 1; display: grid; grid-template-columns: 1fr 1fr; gap: 8px; padding: 8px; background: #121212; }}
            .participant {{ background: #2d2d44; border-radius: 12px; aspect-ratio: 16/9; display: flex; align-items: center; justify-content: center; color: #888; font-size: 14px; position: relative; }}
            .participant-label {{ position: absolute; bottom: 8px; left: 8px; background: rgba(0,0,0,0.5); padding: 4px 10px; border-radius: 4px; font-size: 12px; }}
            .meet-controls {{ padding: 16px; background: #202124; display: flex; justify-content: center; gap: 12px; }}
            .control-btn {{ width: 48px; height: 48px; border-radius: 50%; border: none; background: #3d3d55; color: white; font-size: 20px; cursor: pointer; }}
            .control-btn.danger {{ background: #dc2626; }}
            .room-code {{ padding: 8px 12px; background: #16162a; border-radius: 6px; font-family: monospace; color: #22c55e; }}
        " }
        div {
            class: "meet-app",
            match current {
                Screen::Lobby => rsx! {
                    div {
                        class: "meet-lobby",
                        div {
                            class: "lobby-card",
                            h1 { "🎥 Jay Réunion" }
                            p { style: "color: #aaa; margin-bottom: 20px;",
                                "Salle: "
                                span { class: "room-code", "{meeting.read().room_code}" }
                            }
                            div {
                                class: "lobby-preview",
                                "Caméra désactivée"
                            }
                            input {
                                class: "lobby-input",
                                r#type: "text",
                                placeholder: "Votre nom",
                                value: "{display_name}",
                                oninput: move |evt| display_name.set(evt.value()),
                            }
                            button {
                                class: "btn-primary",
                                onclick: move |_| {
                                    meeting.write().start();
                                    screen.set(Screen::InMeeting);
                                },
                                "Rejoindre la réunion"
                            }
                        }
                    }
                },
                Screen::InMeeting => rsx! {
                    div {
                        class: "meet-stage",
                        div {
                            class: "participant",
                            "(Vidéo non implémentée)"
                            div { class: "participant-label", "Vous" }
                        }
                        div {
                            class: "participant",
                            "En attente d'autres participants..."
                            div { class: "participant-label", "…" }
                        }
                    }
                    div {
                        class: "meet-controls",
                        button { class: "control-btn", title: "Micro", "🎤" }
                        button { class: "control-btn", title: "Caméra", "📷" }
                        button { class: "control-btn", title: "Partager écran", "🖥" }
                        button { class: "control-btn", title: "Chat", "💬" }
                        button {
                            class: "control-btn danger",
                            title: "Quitter",
                            onclick: move |_| {
                                meeting.write().end();
                                screen.set(Screen::Lobby);
                            },
                            "📞"
                        }
                    }
                },
            }
        }
    }
}
