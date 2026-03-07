//! EXP-E15 — Notifications exposant.
//!
//! @id: jf_exp_notifications @do: render_exp_notifications
//! @role: ui @layer: service
//! @human: Ecran EXP-E15 JayFestival: centre de notifications exposant.

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Centre de notifications exposant.
#[component]
pub fn ExpNotifications() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let mut filter = use_signal(|| "toutes".to_string());

    let current_filter = filter.read().clone();
    let filter_toutes_bg = if current_filter == "toutes" { c.accent_blue } else { c.bg_secondary };
    let filter_toutes_color = if current_filter == "toutes" { "white" } else { c.text_primary };
    let filter_nonlues_bg = if current_filter == "non_lues" { c.accent_blue } else { c.bg_secondary };
    let filter_nonlues_color = if current_filter == "non_lues" { "white" } else { c.text_primary };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "Notifications"
                }

                div {
                    style: "display: flex; gap: 8px;",

                    button {
                        style: "padding: 8px 16px; background: {filter_toutes_bg}; border: none; border-radius: 6px; color: {filter_toutes_color}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| { filter.set("toutes".to_string()); },
                        "Toutes"
                    }
                    button {
                        style: "padding: 8px 16px; background: {filter_nonlues_bg}; border: none; border-radius: 6px; color: {filter_nonlues_color}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| { filter.set("non_lues".to_string()); },
                        "Non lues (3)"
                    }

                    button {
                        style: "margin-left: 8px; padding: 8px 12px; background: {c.bg_secondary}; border: none; border-radius: 6px; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                        "Tout marquer comme lu"
                    }
                }
            }

            // Liste des notifications
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                NotificationItem {
                    notif_type: "candidature",
                    title: "Candidature acceptee !",
                    message: "Votre candidature pour le Salon Printemps 2026 a ete acceptee.",
                    time: "Il y a 2 heures",
                    is_read: false,
                }
                NotificationItem {
                    notif_type: "facture",
                    title: "Nouvelle facture",
                    message: "Une facture de 400.00 EUR est disponible pour le Salon Printemps 2026.",
                    time: "Il y a 5 heures",
                    is_read: false,
                }
                NotificationItem {
                    notif_type: "message",
                    title: "Nouveau message",
                    message: "L'organisateur Events Pro vous a envoye un message.",
                    time: "Hier a 14:30",
                    is_read: false,
                }
                NotificationItem {
                    notif_type: "rappel",
                    title: "Rappel: Documents manquants",
                    message: "N'oubliez pas de fournir votre extrait Kbis avant le 20/02/2026.",
                    time: "Il y a 2 jours",
                    is_read: true,
                }
                NotificationItem {
                    notif_type: "info",
                    title: "Mise a jour du programme",
                    message: "Le programme du Salon Printemps 2026 a ete mis a jour.",
                    time: "Il y a 3 jours",
                    is_read: true,
                }
                NotificationItem {
                    notif_type: "candidature",
                    title: "Candidature envoyee",
                    message: "Votre candidature pour le Salon Printemps 2026 a bien ete envoyee.",
                    time: "Il y a 1 semaine",
                    is_read: true,
                }
            }

            // Paramètres de notification
            NotificationSettings {}
        }
    }
}

#[component]
fn NotificationItem(
    notif_type: &'static str,
    title: &'static str,
    message: &'static str,
    time: &'static str,
    is_read: bool,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (icon, icon_color) = match notif_type {
        "candidature" => ("🎫", c.accent_green),
        "facture" => ("💰", c.accent_orange),
        "message" => ("✉️", c.accent_blue),
        "rappel" => ("⏰", c.accent_orange),
        "info" => ("ℹ️", c.text_muted),
        _ => ("🔔", c.text_muted),
    };

    let bg = if is_read { c.bg_secondary.to_string() } else { format!("{}15", c.accent_blue) };
    let border = if is_read { "none".to_string() } else { format!("1px solid {}40", c.accent_blue) };
    let font_weight = if is_read { "400" } else { "600" };

    rsx! {
        div {
            style: "display: flex; align-items: flex-start; gap: 12px; background: {bg}; border: {border}; border-radius: 8px; padding: 16px; cursor: pointer;",

            // Icône
            div {
                style: "width: 40px; height: 40px; background: {icon_color}20; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                "{icon}"
            }

            // Contenu
            div {
                style: "flex: 1;",

                div {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 4px;",

                    h4 {
                        style: "font-size: 14px; color: {c.text_white}; font-weight: {font_weight};",
                        "{title}"
                    }

                    if !is_read {
                        div {
                            style: "width: 8px; height: 8px; background: {c.accent_blue}; border-radius: 50%;",
                        }
                    }
                }

                p {
                    style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 4px;",
                    "{message}"
                }

                p {
                    style: "font-size: 11px; color: {c.text_muted};",
                    "{time}"
                }
            }

            // Actions
            button {
                style: "padding: 4px 8px; background: transparent; border: none; color: {c.text_muted}; cursor: pointer; font-size: 16px;",
                "⋯"
            }
        }
    }
}

#[component]
fn NotificationSettings() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        section {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; margin-top: 16px;",

            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                "Parametres de notification"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                NotificationSetting {
                    title: "Candidatures",
                    description: "Acceptation, refus, demande de documents",
                    email: true,
                    push: true,
                }
                NotificationSetting {
                    title: "Factures et paiements",
                    description: "Nouvelles factures, rappels de paiement",
                    email: true,
                    push: false,
                }
                NotificationSetting {
                    title: "Messages",
                    description: "Messages des organisateurs",
                    email: true,
                    push: true,
                }
                NotificationSetting {
                    title: "Rappels",
                    description: "Echeances, documents manquants",
                    email: true,
                    push: true,
                }
                NotificationSetting {
                    title: "Actualites",
                    description: "Mises a jour des evenements, nouveautes",
                    email: false,
                    push: false,
                }
            }
        }
    }
}

#[component]
fn NotificationSetting(title: &'static str, description: &'static str, email: bool, push: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let email_bg = if email { c.accent_blue } else { c.bg_hover };
    let email_pos = if email { "right: 2px" } else { "left: 2px" };
    let push_bg = if push { c.accent_blue } else { c.bg_hover };
    let push_pos = if push { "right: 2px" } else { "left: 2px" };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; padding: 12px; background: {c.bg_main}; border-radius: 6px;",

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 2px;",
                    "{title}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{description}"
                }
            }

            // Email toggle
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 12px; color: {c.text_muted};", "Email" }
                div {
                    style: "width: 36px; height: 20px; background: {email_bg}; border-radius: 10px; position: relative; cursor: pointer;",
                    div {
                        style: "width: 16px; height: 16px; background: white; border-radius: 50%; position: absolute; top: 2px; {email_pos};",
                    }
                }
            }

            // Push toggle
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 12px; color: {c.text_muted};", "Push" }
                div {
                    style: "width: 36px; height: 20px; background: {push_bg}; border-radius: 10px; position: relative; cursor: pointer;",
                    div {
                        style: "width: 16px; height: 16px; background: white; border-radius: 50%; position: absolute; top: 2px; {push_pos};",
                    }
                }
            }
        }
    }
}
