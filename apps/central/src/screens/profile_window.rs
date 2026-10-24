//! @id     central_screens_profile_window
//! @do     profile_window
//! @role   ui
//! @layer  app
//! @human  central — module « screens profile window ».

//! Fenêtre (overlay) Profil : infos du compte, thème, option réinitialisation COG (tests).

use crate::data::{profile_display_name, use_service_connections};
use crate::state::use_app_state;
use crate::theme::{styles, Theme};
use dioxus::prelude::*;

#[component]
pub fn ProfileWindow() -> Element {
    let mut state = use_app_state();
    let connections = use_service_connections();
    let theme = state.read().current_theme;
    let c = theme.palette();

    let profile = state.read().current_user.clone();
    let Some(ref p) = profile else {
        return rsx! { div { style: "display: none;" } };
    };

    let display_name = profile_display_name(p);
    let on_close = move |_| {
        state.write().show_profile_window = false;
    };

    // Réinitialisation COG : réservée à un profil admin, avec ré-authentification.
    let is_admin = p.is_admin;
    let admin_email = p.email.clone();
    let mut confirm_open = use_signal(|| false);
    let mut reset_password = use_signal(String::new);
    let mut reset_error = use_signal(|| Option::<String>::None);

    let on_reset_cog = move |_| {
        if !is_admin {
            return;
        }
        let auth_db = connections.read().auth_db.clone();
        let pw = reset_password.read().clone();
        match auth_db.reset_cog_to_virgin(&admin_email, &pw) {
            Ok(()) => {
                reset_password.set(String::new());
                let mut s = state.write();
                s.current_user = None;
                s.is_cog_virgin = true;
                s.show_profile_window = false;
            }
            Err(e) => {
                tracing::error!("reset_cog_to_virgin: {}", e);
                reset_password.set(String::new());
                reset_error.set(Some(e.to_string()));
            }
        }
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

                // Zone dangereuse : visible uniquement pour un admin.
                if is_admin && *confirm_open.read() {
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px; margin-bottom: 16px;",
                        div {
                            style: "font-size: 12px; color: {c.accent_red};",
                            "Action destructrice : tous les comptes seront supprimés. Confirmez avec votre mot de passe."
                        }
                        input {
                            r#type: "password",
                            autocomplete: "current-password",
                            placeholder: "Mot de passe administrateur",
                            value: "{reset_password}",
                            oninput: move |evt| reset_password.set(evt.value()),
                            style: "padding: 8px; border-radius: 4px; border: 1px solid {c.border}; background: transparent; color: {c.text_secondary};",
                        }
                        if let Some(err) = reset_error.read().clone() {
                            div { style: "font-size: 12px; color: {c.accent_red};", "{err}" }
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 12px; justify-content: flex-end; flex-wrap: wrap;",
                    if is_admin {
                        if *confirm_open.read() {
                            button {
                                style: "{styles::btn_secondary(theme)}",
                                disabled: reset_password.read().is_empty(),
                                onclick: on_reset_cog,
                                "Confirmer la réinitialisation"
                            }
                        } else {
                            button {
                                style: "{styles::btn_secondary(theme)}",
                                onclick: move |_| {
                                    reset_error.set(None);
                                    confirm_open.set(true);
                                },
                                "Réinitialiser le COG (tests)"
                            }
                        }
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
