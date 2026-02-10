//! Fenêtre (overlay) Profil : infos du compte, thème, option réinitialisation COG (tests).

use dioxus::prelude::*;
use crate::data::{use_service_connections, profile_display_name};
use crate::state::use_app_state;
use crate::theme::{Theme, styles};

#[component]
pub fn ProfileWindow() -> Element {
    let mut state = use_app_state();
    let connections = use_service_connections();
    let theme = state.read().current_theme;
    let c = theme.palette();

    let profile = state.read().current_user.clone();
    let Some(ref p) = profile else {
        return rsx! { div { style: "display: none;" } }.into();
    };

    let display_name = profile_display_name(p);
    let on_close = move |_| {
        state.write().show_profile_window = false;
    };

    let on_reset_cog = move |_| {
        let auth_db = connections.read().auth_db.clone();
        if let Err(e) = auth_db.reset_cog_to_virgin() {
            tracing::error!("reset_cog_to_virgin: {}", e);
            return;
        }
        let mut s = state.write();
        s.current_user = None;
        s.is_cog_virgin = true;
        s.show_profile_window = false;
    };

    rsx! {
        div {
            style: "{styles::overlay_backdrop(theme)}",
            onclick: on_close,

            div {
                style: "{styles::modal_card(theme)}",
                onclick: move |evt| evt.stop_propagation(),

                h2 {
                    style: "{styles::modal_title(theme)}",
                    "Profil"
                }

                div {
                    style: "{styles::modal_body_text(theme)}",
                    div {
                        span { style: "{styles::modal_label(theme)}", "Pseudo :" }
                        span { "{display_name}" }
                    }
                    div {
                        span { style: "{styles::modal_label(theme)}", "E-mail :" }
                        span { "{p.email}" }
                    }
                    div {
                        span { style: "{styles::modal_label(theme)}", "Rôle :" }
                        span { if p.is_admin { "Admin (MiyukiniAdmin)" } else { "Utilisateur" } }
                    }
                    div {
                        style: "{styles::modal_muted_small(theme)}",
                        span { "ID : " }
                        span { "{p.id}" }
                    }
                }

                // Sélecteur de thème
                div {
                    style: "margin-bottom: 20px;",
                    div {
                        style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 8px;",
                        "Thème"
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 8px;",
                        for (t, btn_style) in Theme::all().iter().map(|t| {
                            let is_selected = *t == theme;
                            let style = format!(
                                "padding: 6px 12px; border-radius: 4px; border: 1px solid {}; font-size: 12px; cursor: pointer; transition: all 0.2s; background: {}; color: {};",
                                if is_selected { c.accent_blue } else { c.border },
                                if is_selected { c.accent_blue } else { "transparent" },
                                if is_selected { c.text_white } else { c.text_secondary }
                            );
                            (*t, style)
                        }) {
                            button {
                                style: "{btn_style}",
                                onclick: move |_| {
                                    state.write().current_theme = t;
                                },
                                "{t.label()}"
                            }
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 12px; justify-content: flex-end; flex-wrap: wrap;",
                    button {
                        style: "{styles::btn_secondary(theme)}",
                        onclick: on_reset_cog,
                        "Réinitialiser le COG (tests)"
                    }
                    button {
                        style: "{styles::btn_primary(theme)}",
                        onclick: on_close,
                        "Fermer"
                    }
                }
            }
        }
    }
}
