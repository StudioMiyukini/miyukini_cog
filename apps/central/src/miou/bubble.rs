//! Composant UI bulle Miou et structures associées.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Type d'action d'une bulle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// Ouvre un service (payload = service_id).
    OuvrirService,
    /// Déclenche une pause.
    Pause,
    /// Ferme simplement la bulle.
    Dismiss,
    /// Action personnalisée (payload = action_id).
    Custom,
}

/// Action disponible dans une bulle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulleAction {
    /// Label affiché sur le bouton.
    pub label: String,
    /// Type d'action.
    pub action_type: ActionType,
    /// Payload optionnel (service_id, etc.).
    pub payload: Option<String>,
}

#[allow(dead_code)]
impl BulleAction {
    pub fn dismiss() -> Self {
        Self {
            label: "OK".to_string(),
            action_type: ActionType::Dismiss,
            payload: None,
        }
    }

    pub fn pause() -> Self {
        Self {
            label: "Je prends une pause".to_string(),
            action_type: ActionType::Pause,
            payload: None,
        }
    }

    pub fn open_service(service_id: &str, label: &str) -> Self {
        Self {
            label: label.to_string(),
            action_type: ActionType::OuvrirService,
            payload: Some(service_id.to_string()),
        }
    }
}

/// Sortie du moteur : bulle à afficher.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BulleOutput {
    /// Texte principal de la bulle.
    pub texte: String,
    /// Catégorie (ex: "accueil_matin", "pause_sante").
    pub categorie: String,
    /// ID de la variante utilisée.
    pub variante_id: String,
    /// Actions disponibles.
    pub actions: Vec<BulleAction>,
}

impl BulleOutput {
    pub fn new(
        texte: String,
        categorie: &str,
        variante_id: &str,
        actions: Vec<BulleAction>,
    ) -> Self {
        Self {
            texte,
            categorie: categorie.to_string(),
            variante_id: variante_id.to_string(),
            actions,
        }
    }
}

/// Style CSS pour l'overlay bulle Miou.
fn bubble_overlay_style() -> String {
    r#"
        position: fixed;
        bottom: 16px;
        right: 16px;
        z-index: 1000;
        animation: miou-slide-up 200ms ease-out;
    "#
    .to_string()
}

/// Style CSS pour la bulle elle-même.
fn bubble_style() -> String {
    r#"
        max-width: 360px;
        min-width: 280px;
        padding: 16px;
        background: linear-gradient(135deg, #2d3748 0%, #1a202c 100%);
        border: 1px solid #4a5568;
        border-radius: 12px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
        color: #f7fafc;
        font-family: 'Segoe UI', sans-serif;
    "#
    .to_string()
}

fn bubble_header_style() -> String {
    r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 12px;
        padding-bottom: 8px;
        border-bottom: 1px solid #4a5568;
    "#
    .to_string()
}

fn bubble_title_style() -> String {
    r#"
        display: flex;
        align-items: center;
        gap: 8px;
        font-weight: 600;
        font-size: 14px;
        color: #ffd4e5;
    "#
    .to_string()
}

fn bubble_close_style() -> String {
    r#"
        background: transparent;
        border: none;
        color: #a0aec0;
        cursor: pointer;
        padding: 4px 8px;
        border-radius: 4px;
        font-size: 16px;
        transition: all 0.15s ease;
    "#
    .to_string()
}

fn bubble_text_style() -> String {
    r#"
        font-size: 14px;
        line-height: 1.5;
        color: #e2e8f0;
        margin-bottom: 12px;
    "#
    .to_string()
}

fn bubble_actions_style() -> String {
    r#"
        display: flex;
        gap: 8px;
        flex-wrap: wrap;
        justify-content: flex-end;
    "#
    .to_string()
}

fn action_button_style(is_primary: bool) -> String {
    if is_primary {
        r#"
            padding: 8px 16px;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: 500;
            background: linear-gradient(135deg, #e91e63 0%, #c2185b 100%);
            color: white;
            transition: all 0.15s ease;
        "#
        .to_string()
    } else {
        r#"
            padding: 8px 16px;
            border: 1px solid #4a5568;
            border-radius: 6px;
            cursor: pointer;
            font-size: 13px;
            font-weight: 500;
            background: transparent;
            color: #a0aec0;
            transition: all 0.15s ease;
        "#
        .to_string()
    }
}

/// CSS global pour les animations Miou (à injecter une fois).
pub const MIOU_CSS: &str = r#"
@keyframes miou-slide-up {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.miou-close-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #f7fafc;
}

.miou-action-btn:hover {
    filter: brightness(1.1);
    transform: translateY(-1px);
}
"#;

/// Overlay de la bulle Miou.
/// S'affiche en position fixe en bas à droite de l'écran.
#[component]
pub fn MiouBubbleOverlay(
    bubble: Signal<Option<BulleOutput>>,
    on_dismiss: EventHandler<()>,
    on_action: EventHandler<BulleAction>,
) -> Element {
    let bubble_read = bubble.read();
    let Some(b) = bubble_read.as_ref() else {
        return rsx! {};
    };

    rsx! {
        div {
            style: "{bubble_overlay_style()}",
            class: "miou-bubble-overlay",

            div {
                style: "{bubble_style()}",
                class: "miou-bubble",

                // En-tête
                div {
                    style: "{bubble_header_style()}",

                    div {
                        style: "{bubble_title_style()}",
                        span { "🌸" }
                        span { "Miou" }
                    }

                    button {
                        style: "{bubble_close_style()}",
                        class: "miou-close-btn",
                        onclick: move |_| on_dismiss.call(()),
                        title: "Fermer",
                        "✕"
                    }
                }

                // Texte
                p {
                    style: "{bubble_text_style()}",
                    "{b.texte}"
                }

                // Actions
                if !b.actions.is_empty() {
                    div {
                        style: "{bubble_actions_style()}",

                        for (idx, action) in b.actions.iter().enumerate() {
                            {
                                let action_clone = action.clone();
                                let is_primary = idx == 0;
                                rsx! {
                                    button {
                                        key: "{idx}",
                                        style: "{action_button_style(is_primary)}",
                                        class: "miou-action-btn",
                                        onclick: move |_| on_action.call(action_clone.clone()),
                                        "{action.label}"
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
