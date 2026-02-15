//! Rite d'Entrée — Création du premier compte (COG vierge). Miou accompagne à la voix.
//! Après le mot de passe : écran infos complémentaires (facultatif, par catégorie).
//! Voir docs/services/MiyukiniCentral/Miyukini Central - Rite de Premiere Connexion.md

use dioxus::prelude::*;
use miyukini_central::auth::CentralProfile;
use crate::audio;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use crate::theme::{Theme, styles};

/// Étapes du Rite : Nom → Email → Clé → Infos complémentaires.
const STEP_NOM: u8 = 0;
const STEP_EMAIL: u8 = 1;
const STEP_CLE: u8 = 2;
const STEP_INFOS: u8 = 3;

/// Catégories des infos complémentaires (0..4).
#[allow(dead_code)]
const CAT_IDENTITE: u8 = 0;
#[allow(dead_code)]
const CAT_LOCALISATION: u8 = 1;
#[allow(dead_code)]
const CAT_SITUATION: u8 = 2;
#[allow(dead_code)]
const CAT_CONTEXTE: u8 = 3;
const CAT_COUNT: u8 = 4;


#[component]
fn RiteCatSituation(
    theme: Theme,
    statut_marital: Signal<String>,
    partenaire_genre: Signal<String>,
    partenaire_nom: Signal<String>,
    enfants_oui: Signal<Option<bool>>,
    enfants_nombre: Signal<String>,
    enfants_noms: Signal<String>,
) -> Element {
    let c = theme.palette();
    let show_partenaire = !statut_marital().is_empty() && statut_marital() != "Célibataire";
    let show_enfants = enfants_oui() == Some(true);
    let bg_oui = if enfants_oui() == Some(true) { c.accent_blue } else { c.bg_hover };
    let bg_non = if enfants_oui() == Some(false) { c.accent_blue } else { c.bg_hover };
    rsx! {
        p { style: "font-size: 13px; font-weight: 600; color: {c.text_white}; margin-bottom: 8px;", "Situation personnelle" }
        p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Statut relationnel" }
        select {
            style: "{styles::form_input(theme)} margin-bottom: 12px;",
            value: "{statut_marital()}",
            oninput: move |evt| statut_marital.set(evt.value()),
            option { value: "", "Choisir" }
            option { value: "Célibataire", "Célibataire" }
            option { value: "En couple", "En couple" }
            option { value: "Marié-e", "Marié-e" }
        }
        {if show_partenaire {
            rsx! {
            p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Genre de ton/sa partenaire" }
            select {
                style: "{styles::form_input(theme)} margin-bottom: 8px;",
                value: "{partenaire_genre()}",
                oninput: move |evt| partenaire_genre.set(evt.value()),
                option { value: "", "Choisir" }
                option { value: "Homme", "Homme" }
                option { value: "Femme", "Femme" }
                option { value: "Autre", "Autre" }
            }
            p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Prénom du ou de la partenaire" }
            input {
                style: "{styles::form_input(theme)} margin-bottom: 12px;",
                r#type: "text",
                placeholder: "Prénom du ou de la partenaire",
                value: "{partenaire_nom()}",
                oninput: move |evt| partenaire_nom.set(evt.value()),
            }
            }
        } else {
            rsx! { }
        }}
        p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "As-tu des enfants" }
        div {
            style: "display: flex; gap: 12px; margin-bottom: 12px;",
            button {
                style: "padding: 8px 16px; background: {bg_oui}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                onclick: move |_| enfants_oui.set(Some(true)),
                "Oui"
            }
            button {
                style: "padding: 8px 16px; background: {bg_non}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                onclick: move |_| enfants_oui.set(Some(false)),
                "Non"
            }
        }
        {if show_enfants {
            rsx! {
            input {
                style: "{styles::form_input(theme)} margin-bottom: 8px;",
                r#type: "number",
                placeholder: "Combien",
                min: "1",
                max: "20",
                value: "{enfants_nombre()}",
                oninput: move |evt| enfants_nombre.set(evt.value()),
            }
            input {
                style: "{styles::form_input(theme)} margin-bottom: 16px;",
                r#type: "text",
                placeholder: "Prénoms (séparés par des virgules)",
                value: "{enfants_noms()}",
                oninput: move |evt| enfants_noms.set(evt.value()),
            }
            }
        } else {
            rsx! { }
        }}
    }
}

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
    let c = theme.palette();
    let mut step = use_signal(|| STEP_NOM);
    let mut pseudonyme = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(String::new);
    let mut last_played_step = use_signal(|| None::<u8>);
    // Profil créé (pour mise à jour avec infos complémentaires)
    let mut profile_created = use_signal(|| None::<CentralProfile>);
    // Catégorie courante des infos
    let mut cat_idx = use_signal(|| 0u8);
    // Infos complémentaires (signaux)
    let mut genre = use_signal(String::new);
    let mut date_naissance = use_signal(String::new);
    let mut ville = use_signal(String::new);
    let mut numero_voie = use_signal(String::new);
    let mut rue = use_signal(String::new);
    let mut code_postal = use_signal(String::new);
    let statut_marital = use_signal(String::new);
    let partenaire_genre = use_signal(String::new);
    let partenaire_nom = use_signal(String::new);
    let enfants_oui = use_signal(|| None::<bool>);
    let enfants_nombre = use_signal(String::new);
    let enfants_noms = use_signal(String::new);
    let mut profession = use_signal(String::new);
    let mut langue_maternelle = use_signal(String::new);
    let mut preference_rappel = use_signal(String::new);

    // Jouer la voix Miou de l'étape courante (une fois par étape).
    use_effect(move || {
        let s = step();
        if s != STEP_INFOS && last_played_step() == Some(s) {
            return;
        }
        if s == STEP_NOM || s == STEP_EMAIL || s == STEP_CLE {
            let base = connections.read().miyuclicker_data_dir.clone();
            audio::play_voice_background(&base, VOIX_RITE[s as usize]);
            last_played_step.set(Some(s));
        }
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
                profile_created.set(Some(profile.clone()));
                let mut s = state.write();
                s.current_user = Some(profile);
                s.is_cog_virgin = false;
                s.rite_infos_pending = true;
                step.set(STEP_INFOS);
                cat_idx.set(0);
            }
            Err(e) => {
                error.write().push_str(&e.to_string());
            }
        }
    };

    let mut on_infos_done = move |_| {
        if let Some(mut profile) = profile_created.take() {
            let genre_val = genre.read().trim().to_string();
            let date_val = date_naissance.read().trim().to_string();
            let ville_val = ville.read().trim().to_string();
            let num_val = numero_voie.read().trim().to_string();
            let rue_val = rue.read().trim().to_string();
            let cp_val = code_postal.read().trim().to_string();
            let statut_val = statut_marital.read().trim().to_string();
            let part_genre_val = partenaire_genre.read().trim().to_string();
            let part_nom_val = partenaire_nom.read().trim().to_string();
            let enf_nb_str = enfants_nombre.read().trim().to_string();
            let enf_noms_val = enfants_noms.read().trim().to_string();
            let profession_val = profession.read().trim().to_string();
            let langue_val = langue_maternelle.read().trim().to_string();

            if !genre_val.is_empty() {
                profile.genre = Some(genre_val);
            }
            if !date_val.is_empty() {
                profile.date_naissance = Some(date_val);
            }
            if !ville_val.is_empty() {
                profile.ville = Some(ville_val);
            }
            if !num_val.is_empty() {
                profile.numero_voie = Some(num_val);
            }
            if !rue_val.is_empty() {
                profile.rue = Some(rue_val);
            }
            if !cp_val.is_empty() {
                profile.code_postal = Some(cp_val);
            }
            if !statut_val.is_empty() {
                profile.statut_marital = Some(statut_val);
            }
            if !part_genre_val.is_empty() {
                profile.partenaire_genre = Some(part_genre_val);
            }
            if !part_nom_val.is_empty() {
                profile.partenaire_nom = Some(part_nom_val);
            }
            if let Ok(n) = enf_nb_str.parse::<i32>() {
                if n > 0 {
                    profile.enfants_nombre = Some(n);
                }
            }
            if !enf_noms_val.is_empty() {
                profile.enfants_noms = Some(enf_noms_val);
            }
            if !profession_val.is_empty() {
                profile.profession = Some(profession_val);
            }
            if !langue_val.is_empty() {
                profile.langue_maternelle = Some(langue_val);
            }

            if let Err(e) = connections.read().auth_db.update_profile(&profile) {
                tracing::warn!("update_profile infos rite: {}", e);
            }
            let mut s = state.write();
            s.current_user = Some(profile);
            s.rite_infos_pending = false;
        }
    };

    let on_pass_cat = move |_| {
        if cat_idx() + 1 >= CAT_COUNT {
            on_infos_done(());
        } else {
            cat_idx.set(cat_idx() + 1);
        }
    };

    let on_next_cat = move |_| {
        if cat_idx() + 1 >= CAT_COUNT {
            on_infos_done(());
        } else {
            cat_idx.set(cat_idx() + 1);
        }
    };

    rsx! {
        div {
            style: "{styles::fullscreen_container(theme)}",
            div {
                style: "max-width: 440px; width: 100%;",
                div {
                    style: "{styles::form_card(theme)}",

                    {if step() == STEP_NOM {
                        rsx! {
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {c.text_secondary}; line-height: 1.5;", "Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ?" }
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
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {c.text_secondary}; line-height: 1.5;", "Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ?" }
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
                    } else if step() == STEP_CLE {
                        rsx! {
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {c.text_secondary}; line-height: 1.5;", "Pour finir, peux-tu me donner une clé pour protéger l'entrée ? Suis les instructions ci-dessous." }
                            ul {
                                style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 16px; padding-left: 20px;",
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
                    } else {
                        // STEP_INFOS — infos complémentaires par catégorie
                        rsx! {
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 20px; color: {c.text_secondary}; line-height: 1.5;", "Pour que je te connaisse un peu mieux, peux-tu me répondre à quelques questions ? Tu peux passer celles que tu ne veux pas partager." }

                            {match cat_idx() {
                                0 => rsx! {
                                    // Catégorie A — Identité
                                    p { style: "font-size: 13px; font-weight: 600; color: {c.text_white}; margin-bottom: 8px;", "Identité" }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Comment te sens-tu le mieux défini ?" }
                                    select {
                                        style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                        value: "{genre()}",
                                        oninput: move |evt| genre.set(evt.value()),
                                        option { value: "", "— Choisir —" }
                                        option { value: "Masculin", "Masculin" }
                                        option { value: "Féminin", "Féminin" }
                                        option { value: "Neutre", "Neutre" }
                                    }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Date de naissance (JJ/MM/AAAA)" }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 16px;",
                                        r#type: "text",
                                        placeholder: "ex. 15/03/1990",
                                        value: "{date_naissance()}",
                                        oninput: move |evt| date_naissance.set(evt.value()),
                                    }
                                },
                                1 => rsx! {
                                    // Catégorie B — Localisation
                                    p { style: "font-size: 13px; font-weight: 600; color: {c.text_white}; margin-bottom: 8px;", "Localisation" }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Où habites-tu ?" }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                        r#type: "text",
                                        placeholder: "Ville",
                                        value: "{ville()}",
                                        oninput: move |evt| ville.set(evt.value()),
                                    }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Adresse postale (optionnel)" }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 8px;",
                                        r#type: "text",
                                        placeholder: "N° de voie",
                                        value: "{numero_voie()}",
                                        oninput: move |evt| numero_voie.set(evt.value()),
                                    }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 8px;",
                                        r#type: "text",
                                        placeholder: "Nom de la rue",
                                        value: "{rue()}",
                                        oninput: move |evt| rue.set(evt.value()),
                                    }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 16px;",
                                        r#type: "text",
                                        placeholder: "Code postal",
                                        value: "{code_postal()}",
                                        oninput: move |evt| code_postal.set(evt.value()),
                                    }
                                },
                                2 => rsx! {
                                    RiteCatSituation {
                                        theme,
                                        statut_marital,
                                        partenaire_genre,
                                        partenaire_nom,
                                        enfants_oui,
                                        enfants_nombre,
                                        enfants_noms,
                                    }
                                },
                                _ => rsx! {
                                    // Catégorie D — Contexte
                                    p { style: "font-size: 13px; font-weight: 600; color: {c.text_white}; margin-bottom: 8px;", "Contexte de vie" }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Que fais-tu dans la vie ?" }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                        r#type: "text",
                                        placeholder: "Profession, études...",
                                        value: "{profession()}",
                                        oninput: move |evt| profession.set(evt.value()),
                                    }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Langue maternelle ?" }
                                    select {
                                        style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                        value: "{langue_maternelle()}",
                                        oninput: move |evt| langue_maternelle.set(evt.value()),
                                        option { value: "", "— Choisir —" }
                                        option { value: "Français", "Français" }
                                        option { value: "Anglais", "Anglais" }
                                        option { value: "Espagnol", "Espagnol" }
                                        option { value: "Allemand", "Allemand" }
                                        option { value: "Italien", "Italien" }
                                        option { value: "Autre", "Autre" }
                                    }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Tu préfères le matin ou le soir pour mes rappels ?" }
                                    select {
                                        style: "{styles::form_input(theme)} margin-bottom: 16px;",
                                        value: "{preference_rappel()}",
                                        oninput: move |evt| preference_rappel.set(evt.value()),
                                        option { value: "", "— Choisir —" }
                                        option { value: "Matin", "Matin" }
                                        option { value: "Soir", "Soir" }
                                        option { value: "Peu importe", "Peu importe" }
                                    }
                                },
                            }}

                            div {
                                style: "display: flex; flex-direction: column; gap: 8px; margin-top: 20px;",
                                button {
                                    style: "{styles::form_btn_primary(theme)}",
                                    onclick: on_next_cat,
                                    if cat_idx() + 1 >= CAT_COUNT { "Terminer et entrer" } else { "Continuer" }
                                }
                                button {
                                    style: "padding: 10px; background: transparent; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                                    onclick: on_pass_cat,
                                    "Passer cette catégorie"
                                }
                            }
                        }
                    }}
                }
            }
        }
    }
}
