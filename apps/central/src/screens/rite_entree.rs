//! Rite d'Entrée — Création du premier compte (COG vierge). Miou accompagne à la voix.
//! Wording et parcours selon le document fondateur.

use dioxus::prelude::*;
use crate::audio;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use crate::theme::styles;

/// Étapes du Rite : Nom → Email → Clé (mot de passe).
const STEP_NOM: u8 = 0;
const STEP_EMAIL: u8 = 1;
const STEP_CLE: u8 = 2;

/// Fichiers voix Miou par étape (nom, email, clé).
const VOIX_RITE: [&str; 3] = [
    "login_new_ask_name.mp3",
    "login_new_ask_email.mp3",
    "login_new_ask_password.mp3",
];

#[component]
pub fn RiteEntree() -> Element {
    let connections = use_service_connections();
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let mut step = use_signal(|| STEP_NOM);
    let mut pseudonyme = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut last_played_step = use_signal(|| None::<u8>);

    // Jouer la voix Miou de l'étape courante (une fois par étape).
    use_effect(move || {
        let s = step();
        if last_played_step() == Some(s) {
            return;
        }
        let base = connections.read().miyuclicker_data_dir.clone();
        audio::play_voice_background(&base, VOIX_RITE[s as usize]);
        last_played_step.set(Some(s));
    });

    let on_sign_done = move |_| {
        let pseudo = pseudonyme.read().trim().to_string();
        let em = email.read().trim().to_string();
        let pass = password.read().clone();
        error.write().clear();
        if em.is_empty() {
            error.write().push_str("L'adresse e-mail est requise.");
            return;
        }
        let auth_db = connections.read().auth_db.clone();
        match auth_db.sign_up(&em, &pass, Some(pseudo.as_str())) {
            Ok(profile) => {
                let _ = auth_db.set_current_profile_id(Some(profile.id.as_str()));
                let mut s = state.write();
                s.current_user = Some(profile.clone());
                s.is_cog_virgin = false;
            }
            Err(e) => {
                error.write().push_str(&e.to_string());
            }
        }
    };

    rsx! {
        div {
            style: "{styles::fullscreen_container(theme)}",
            div {
                style: "max-width: 400px; width: 100%;",
                div {
                    style: "{styles::form_card(theme)}",

                    {if step() == STEP_NOM {
                        rsx! {
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {theme.palette().text_secondary}; line-height: 1.5;", "Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ?" }
                            input {
                                style: "{styles::form_input(theme)}",
                                r#type: "text",
                                placeholder: "Nom / pseudo",
                                value: "{pseudonyme()}",
                                oninput: move |evt| pseudonyme.set(evt.value()),
                            }
                            button {
                                style: "{styles::form_btn_primary(theme)}",
                                onclick: move |_| step.set(STEP_EMAIL),
                                "Continuer"
                            }
                        }
                    } else if step() == STEP_EMAIL {
                        rsx! {
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {theme.palette().text_secondary}; line-height: 1.5;", "Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ?" }
                            input {
                                style: "{styles::form_input(theme)}",
                                r#type: "email",
                                placeholder: "Adresse e-mail",
                                value: "{email()}",
                                oninput: move |evt| email.set(evt.value()),
                            }
                            button {
                                style: "{styles::form_btn_primary(theme)}",
                                onclick: move |_| step.set(STEP_CLE),
                                "Continuer"
                            }
                        }
                    } else {
                        rsx! {
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {theme.palette().text_secondary}; line-height: 1.5;", "Pour finir, peux-tu me donner une clé pour protéger l'entrée ? Suis les instructions ci-dessous." }
                            ul {
                                style: "font-size: 12px; color: {theme.palette().text_secondary}; margin-bottom: 16px; padding-left: 20px;",
                                li { "Longueur minimale 8 caractères" }
                                li { "Une majuscule, une minuscule" }
                                li { "Un chiffre" }
                                li { "Un symbole" }
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
                                onclick: on_sign_done,
                                "🖋️ Signer"
                            }
                        }
                    }}
                }
            }
        }
    }
}
