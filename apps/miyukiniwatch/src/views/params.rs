//! Ecran Parametres et vie privee MiyukiniWatch.

use crate::{use_db, use_profile_id};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use miyukiniwatch::{MiyukiniWatchPresenter, Prefs};

#[component]
pub fn ParamsScreen(prefs: Prefs) -> Element {
    let c = use_palette();
    let db = use_db();
    let profile_id = use_profile_id();
    let mut show_confirm_delete = use_signal(|| false);

    let db_toggle = db.clone();
    let db_delete = db.clone();
    let profile_id_toggle = profile_id.clone();
    let profile_id_delete = profile_id.clone();
    let mut local_prefs = use_signal(|| prefs.clone());

    let on_toggle_collect = move |_| {
        let enabled = !local_prefs().collect_enabled;
        local_prefs.write().collect_enabled = enabled;
        let mut p = local_prefs().clone();
        p.profile_id = profile_id_toggle.clone();
        let presenter = MiyukiniWatchPresenter::new(db_toggle.clone());
        let _ = presenter.set_prefs(&p);
    };

    let on_delete_all = move |_| {
        let presenter = MiyukiniWatchPresenter::new(db_delete.clone());
        let _ = presenter.delete_all(&profile_id_delete);
        show_confirm_delete.set(false);
    };

    let btn_bg = if local_prefs().collect_enabled {
        c.accent_green
    } else {
        c.bg_hover
    };
    let btn_label = if local_prefs().collect_enabled {
        "Desactiver"
    } else {
        "Activer"
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 600px;",

            // Etat collecte
            div {
                style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                h3 { style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 12px;", "Collecte" }
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "color: {c.text_primary};", "Activer la collecte" }
                    button {
                        style: "padding: 6px 16px; background: {btn_bg}; color: {c.text_white}; border: none; border-radius: 4px; cursor: pointer;",
                        onclick: on_toggle_collect,
                        "{btn_label}"
                    }
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted}; margin-top: 8px;",
                    "Quand desactivee, les donnees existantes restent consultables."
                }
            }

            // Effacement
            div {
                style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                h3 { style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 12px;", "Effacement" }
                p {
                    style: "font-size: 13px; color: {c.text_primary}; margin-bottom: 16px;",
                    "Supprimer toutes les donnees MiyukiniWatch. Miou utilisera des messages generiques."
                }
                button {
                    style: "padding: 8px 16px; background: {c.accent_red}; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    onclick: move |_| show_confirm_delete.set(true),
                    "Effacer tout l'historique"
                }
            }
        }

        {if show_confirm_delete() {
            rsx! {
                div {
                    style: "position: fixed; inset: 0; background: rgba(0,0,0,0.6); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                    div {
                        style: "background: {c.bg_card}; padding: 24px; border-radius: 8px; max-width: 400px; border: 1px solid {c.border};",
                        h3 { style: "margin-bottom: 12px; color: {c.text_white};", "Effacer les donnees ?" }
                        p {
                            style: "color: {c.text_secondary}; margin-bottom: 16px; font-size: 14px;",
                            "Cette action est irreversible. Miou utilisera des messages generiques."
                        }
                        div { style: "display: flex; gap: 8px; justify-content: flex-end;",
                            button {
                                style: "padding: 8px 16px; background: {c.bg_hover}; color: {c.text_primary}; border: none; border-radius: 4px; cursor: pointer;",
                                onclick: move |_| show_confirm_delete.set(false),
                                "Annuler"
                            }
                            button {
                                style: "padding: 8px 16px; background: {c.accent_red}; color: white; border: none; border-radius: 4px; cursor: pointer;",
                                onclick: on_delete_all,
                                "Confirmer l'effacement"
                            }
                        }
                    }
                }
            }
        } else {
            rsx! { }
        }}
    }
}
