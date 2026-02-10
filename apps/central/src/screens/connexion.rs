//! Connexion — Retour d'un habitant connu. Miou accueille avec la voix (son + phrase).
//! Pré-remplit l'email du dernier profil connecté.

use dioxus::prelude::*;
use crate::audio;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use crate::theme::styles;

/// Étapes du rite de retour : Accueil → Clé.
const STEP_ACCUEIL: u8 = 0;
const STEP_CLE: u8 = 1;

/// Phrases d'accueil de Miou (une jouée aléatoirement avec le son correspondant).
const PHRASES_MIOU: [&str; 3] = [
    "Quelle bonne surprise. Entre donc avec ta clé et rejoins moi à l'intérieur.",
    "Te voilà de retour. Entre donc avec ta clé et rejoins moi à l'intérieur.",
    "J'étais si impatiente de te revoir. Entre donc avec ta clé et rejoins moi à l'intérieur.",
];

#[component]
pub fn Connexion() -> Element {
    let connections = use_service_connections();
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();

    // Pré-remplir l'email avec le dernier profil connecté
    let saved_email = state.read().last_login_email.clone();
    let saved_pseudo = state.read().last_login_pseudo.clone();
    let has_saved = !saved_email.is_empty();

    let mut step = use_signal(|| if has_saved { STEP_ACCUEIL } else { STEP_CLE });
    let initial_email = saved_email.clone();
    let mut email = use_signal(move || initial_email.clone());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    // Index de la phrase (et du son) Miou choisi au montage (0=a, 1=b, 2=c).
    let mut miou_phrase_index = use_signal(|| None::<usize>);

    // À l'affichage de l'écran connexion avec compte connu : jouer un des 3 sons Miou et retenir la phrase.
    use_effect(move || {
        if !has_saved || miou_phrase_index().is_some() {
            return;
        }
        let base = connections.read().miyuclicker_data_dir.clone();
        let suffixes = ['a', 'b', 'c'];
        let idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() % 3) as usize;
        let filename = format!("login_retour_{}.mp3", suffixes[idx]);
        audio::play_voice_background(&base, &filename);
        miou_phrase_index.set(Some(idx));
    });

    let on_enter = move |_| {
        let em = email.read().trim().to_string();
        let pass = password.read().clone();
        error.write().clear();
        if em.is_empty() {
            error.write().push_str("Indique ton adresse e-mail.");
            return;
        }
        let auth_db = connections.read().auth_db.clone();
        match auth_db.sign_in(&em, &pass) {
            Ok(Some(profile)) => {
                let _ = auth_db.set_current_profile_id(Some(profile.id.as_str()));
                state.write().current_user = Some(profile);
            }
            Ok(None) => {
                error.write().push_str("Clé incorrecte ou compte inconnu.");
            }
            Err(e) => {
                error.write().push_str(&e.to_string());
            }
        }
    };

    // Nom affiché pour l'accueil
    let display_name = if saved_pseudo.is_empty() {
        saved_email.split('@').next().unwrap_or("habitant").to_string()
    } else {
        saved_pseudo.clone()
    };

    rsx! {
        div {
            style: "{styles::fullscreen_container(theme)}",
            div {
                style: "max-width: 420px; width: 100%;",
                div {
                    style: "{styles::form_card(theme)}",

                    {if step() == STEP_ACCUEIL && has_saved {
                        // Étape 1 : accueil par Miou (voix + phrase)
                        let phrase = miou_phrase_index()
                            .and_then(|i| PHRASES_MIOU.get(i).copied())
                            .unwrap_or(PHRASES_MIOU[0]);
                        rsx! {
                            div {
                                style: "text-align: center; margin-bottom: 24px;",
                                span {
                                    style: "font-size: 48px; display: block; margin-bottom: 12px;",
                                    "🌸"
                                }
                            }
                            p {
                                style: "{styles::form_title(theme)}",
                                "— Miou"
                            }
                            p {
                                style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 24px; line-height: 1.5;",
                                "{phrase}"
                            }
                            div {
                                style: "display: flex; align-items: center; gap: 12px; padding: 12px 16px; background: {c.bg_hover}; border-radius: 8px; margin-bottom: 20px;",
                                div {
                                    style: "width: 40px; height: 40px; border-radius: 50%; background: {c.accent_blue}; display: flex; align-items: center; justify-content: center; font-size: 18px; color: white; font-weight: 600;",
                                    "{display_name.chars().next().unwrap_or('?').to_uppercase().next().unwrap_or('?')}"
                                }
                                div {
                                    p {
                                        style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                                        "{display_name}"
                                    }
                                    p {
                                        style: "font-size: 12px; color: {c.text_muted};",
                                        "{saved_email}"
                                    }
                                }
                            }
                            button {
                                style: "{styles::form_btn_primary(theme)}",
                                onclick: move |_| step.set(STEP_CLE),
                                "Continuer"
                            }
                            button {
                                style: "width: 100%; padding: 10px; background: transparent; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 12px; margin-top: 8px;",
                                onclick: move |_| {
                                    email.set(String::new());
                                    step.set(STEP_CLE);
                                },
                                "Utiliser un autre compte"
                            }
                        }
                    } else {
                        // Étape 2 (ou directe) : saisie email + clé
                        rsx! {
                            p {
                                style: "{styles::form_title(theme)}",
                                if has_saved {
                                    "Bienvenue à toi. Peux-tu entrer la clé ici, s'il te plaît ?"
                                } else {
                                    "Bienvenue, habitant. Identifie-toi pour entrer."
                                }
                            }
                            if !has_saved || email().is_empty() {
                                // Email modifiable si pas de profil sauvé ou « autre compte »
                                input {
                                    style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                    r#type: "email",
                                    placeholder: "Adresse e-mail",
                                    value: "{email()}",
                                    oninput: move |evt| email.set(evt.value()),
                                }
                            } else {
                                // Rappel du profil sélectionné (non modifiable, clic pour changer)
                                div {
                                    style: "display: flex; align-items: center; gap: 10px; padding: 10px 14px; background: {c.bg_hover}; border-radius: 6px; margin-bottom: 12px; cursor: pointer;",
                                    onclick: move |_| {
                                        step.set(STEP_ACCUEIL);
                                    },
                                    span {
                                        style: "font-size: 13px; color: {c.text_white}; flex: 1;",
                                        "{email()}"
                                    }
                                    span {
                                        style: "font-size: 11px; color: {c.text_muted};",
                                        "changer"
                                    }
                                }
                            }
                            input {
                                style: "{styles::form_input(theme)} margin-bottom: 8px;",
                                r#type: "password",
                                placeholder: "Mot de passe",
                                value: "{password()}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                            if !error().is_empty() {
                                p { style: "{styles::form_error(theme)}", "{error()}" }
                            }
                            button {
                                style: "{styles::form_btn_primary(theme)}",
                                onclick: on_enter,
                                "🚪 Entrer"
                            }
                        }
                    }}
                }
            }
        }
    }
}
