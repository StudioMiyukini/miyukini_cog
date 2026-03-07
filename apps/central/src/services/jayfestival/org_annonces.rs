//! ORG-E23 — Annonces et notifications aux exposants/visiteurs.
//!
//! @id: jf_org_annonces @do: render_org_annonces
//! @role: ui @layer: service
//! @human: Ecran ORG-E23 JayFestival: envoi annonces et notifications organisateur.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use super::components::{ActionButton, Badge, EmptyState};

/// Gestion des annonces d'une edition.
#[component]
pub fn OrgAnnonces(edition_id: String) -> Element {
    let p = use_palette();

    let mut show_form = use_signal(|| false);

    // Données de démonstration (à remplacer par DB)
    let annonces = vec![
        ("Important : Changement d'horaire", "Exposants", "2026-01-15", "publie"),
        ("Rappel : Deadline candidatures", "Tous", "2026-01-10", "publie"),
        ("Preparation evenement", "Exposants", "2025-12-20", "brouillon"),
    ];

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {p.text_high};",
                    "Annonces et communications"
                }

                ActionButton {
                    label: "Nouvelle annonce".to_string(),
                    icon: "📢".to_string(),
                    accent: true,
                    onclick: move |_| {
                        show_form.set(true);
                    },
                }
            }

            // Formulaire nouvelle annonce
            if *show_form.read() {
                div {
                    style: "background: {p.bg_secondary}; border-radius: 8px; padding: 20px;",

                    h4 {
                        style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                        "Creer une annonce"
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 16px;",

                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 6px;",
                                "Titre"
                            }
                            input {
                                r#type: "text",
                                style: "width: 100%; padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px;",
                                placeholder: "Titre de l'annonce",
                            }
                        }

                        div {
                            label {
                                style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 6px;",
                                "Contenu"
                            }
                            textarea {
                                style: "width: 100%; padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px; min-height: 100px;",
                                placeholder: "Contenu de l'annonce...",
                            }
                        }

                        div {
                            style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                            div {
                                label {
                                    style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 6px;",
                                    "Destinataires"
                                }
                                select {
                                    style: "width: 100%; padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px;",

                                    option { value: "all", "Tous (exposants + visiteurs)" }
                                    option { value: "exposants", "Exposants uniquement" }
                                    option { value: "visiteurs", "Visiteurs uniquement" }
                                }
                            }

                            div {
                                label {
                                    style: "display: block; font-size: 12px; color: {p.text_secondary}; margin-bottom: 6px;",
                                    "Canal"
                                }
                                select {
                                    style: "width: 100%; padding: 10px 12px; background: {p.bg_base}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 14px;",

                                    option { value: "app", "Application uniquement" }
                                    option { value: "email", "Email" }
                                    option { value: "both", "Application + Email" }
                                }
                            }
                        }

                        div {
                            style: "display: flex; gap: 12px;",

                            ActionButton {
                                label: "Enregistrer brouillon".to_string(),
                                icon: "💾".to_string(),
                                accent: false,
                                onclick: move |_| {
                                    show_form.set(false);
                                },
                            }
                            ActionButton {
                                label: "Publier".to_string(),
                                icon: "📤".to_string(),
                                accent: true,
                                onclick: move |_| {
                                    show_form.set(false);
                                },
                            }
                            button {
                                style: "padding: 10px 16px; background: transparent; border: none; color: {p.text_muted}; cursor: pointer; font-size: 13px;",
                                onclick: move |_| show_form.set(false),
                                "Annuler"
                            }
                        }
                    }
                }
            }

            // Liste des annonces
            if annonces.is_empty() {
                EmptyState {
                    title: "Aucune annonce".to_string(),
                    message: "Creez votre premiere annonce pour communiquer avec les participants".to_string(),
                    icon: "📢".to_string(),
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for (title, dest, date, status) in annonces {
                        div {
                            style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

                            // Icône
                            div {
                                style: "width: 48px; height: 48px; background: {p.bg_overlay}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                                "📢"
                            }

                            // Infos
                            div {
                                style: "flex: 1;",
                                p {
                                    style: "font-size: 14px; color: {p.text_high};",
                                    "{title}"
                                }
                                div {
                                    style: "display: flex; gap: 12px; font-size: 12px; color: {p.text_muted};",
                                    span { "👥 {dest}" }
                                    span { "📅 {date}" }
                                }
                            }

                            // Badge statut
                            Badge {
                                text: status.to_string(),
                                color: if status == "publie" { p.success.to_string() } else { p.warning.to_string() },
                            }

                            // Actions
                            div {
                                style: "display: flex; gap: 8px;",

                                button {
                                    style: "padding: 8px 12px; background: {p.bg_overlay}; border: none; border-radius: 4px; color: {p.text_primary}; cursor: pointer; font-size: 12px;",
                                    "Modifier"
                                }
                                if status != "publie" {
                                    button {
                                        style: "padding: 8px 12px; background: {p.accent_primary}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                                        "Publier"
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
