//! Rite d'Entrée — Création du premier compte (COG vierge). Miou accompagne à la voix.
//! Après le mot de passe : écran infos complémentaires (facultatif, par catégorie).
//! Voir docs/services/MiyukiniCentral/Miyukini Central - Rite de Premiere Connexion.md

use dioxus::prelude::*;
use miyukini_central::auth::{CentralProfile, PASSWORD_MIN_LEN};
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

// ── Helpers de validation ───────────────────────────────────────────────

/// Vérifie un email de manière basique (contient @ et un point après).
fn is_valid_email(email: &str) -> bool {
    let trimmed = email.trim();
    if trimmed.len() < 5 { return false; }
    if let Some(at_pos) = trimmed.find('@') {
        let domain = &trimmed[at_pos + 1..];
        domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
    } else {
        false
    }
}

/// État de validation du mot de passe pour l'indicateur temps réel.
struct PasswordChecks {
    has_min_len: bool,
    has_upper: bool,
    has_lower: bool,
    has_digit: bool,
    has_special: bool,
}

impl PasswordChecks {
    fn from_password(password: &str) -> Self {
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        let mut has_special = false;
        for c in password.chars() {
            if c.is_ascii_uppercase() { has_upper = true; }
            else if c.is_ascii_lowercase() { has_lower = true; }
            else if c.is_ascii_digit() { has_digit = true; }
            else { has_special = true; }
        }
        Self {
            has_min_len: password.len() >= PASSWORD_MIN_LEN,
            has_upper,
            has_lower,
            has_digit,
            has_special,
        }
    }

    fn all_valid(&self) -> bool {
        self.has_min_len && self.has_upper && self.has_lower && self.has_digit && self.has_special
    }

    fn score(&self) -> usize {
        [self.has_min_len, self.has_upper, self.has_lower, self.has_digit, self.has_special]
            .iter().filter(|&&b| b).count()
    }
}

// ── Sous-composants ────────────────────────────────────────────────────

/// Indicateur temps réel de la force du mot de passe.
#[component]
fn PasswordStrengthIndicator(password: String, theme: Theme) -> Element {
    let c = theme.palette();
    let checks = PasswordChecks::from_password(&password);
    let score = checks.score();
    let (bar_color, label) = match score {
        0 => (c.text_muted, ""),
        1 => (c.accent_red, "Très faible"),
        2 => (c.accent_red, "Faible"),
        3 => (c.accent_orange, "Moyen"),
        4 => (c.accent_orange, "Correct"),
        _ => (c.accent_green, "Fort"),
    };
    let pct = score * 20; // 0 à 100

    let check_icon = |ok: bool| if ok { "✓" } else { "○" };
    let check_color = |ok: bool| if ok { c.accent_green } else { c.text_muted };

    if password.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            style: "margin-top: 8px; display: flex; flex-direction: column; gap: 6px;",

            // Barre de progression
            div {
                style: "display: flex; align-items: center; gap: 8px;",
                div {
                    style: "flex: 1; height: 4px; background: {c.bg_hover}; border-radius: 2px; overflow: hidden;",
                    div {
                        style: "width: {pct}%; height: 100%; background: {bar_color}; border-radius: 2px; transition: all 0.3s;",
                    }
                }
                if !label.is_empty() {
                    span {
                        style: "font-size: 11px; color: {bar_color}; font-weight: 500; white-space: nowrap;",
                        "{label}"
                    }
                }
            }

            // Critères détaillés
            div {
                style: "display: flex; flex-wrap: wrap; gap: 4px 12px;",

                span {
                    style: "font-size: 11px; color: {check_color(checks.has_min_len)};",
                    "{check_icon(checks.has_min_len)} 8 caractères"
                }
                span {
                    style: "font-size: 11px; color: {check_color(checks.has_upper)};",
                    "{check_icon(checks.has_upper)} Majuscule"
                }
                span {
                    style: "font-size: 11px; color: {check_color(checks.has_lower)};",
                    "{check_icon(checks.has_lower)} Minuscule"
                }
                span {
                    style: "font-size: 11px; color: {check_color(checks.has_digit)};",
                    "{check_icon(checks.has_digit)} Chiffre"
                }
                span {
                    style: "font-size: 11px; color: {check_color(checks.has_special)};",
                    "{check_icon(checks.has_special)} Symbole"
                }
            }
        }
    }
}

/// Indicateur de progression des étapes (0..3).
#[component]
fn StepIndicator(current: u8, total: u8, theme: Theme) -> Element {
    let c = theme.palette();
    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: center; gap: 8px; margin-bottom: 20px;",
            for i in 0..total {
                {
                    let (bg, w) = if i < current {
                        (c.accent_blue, "24px")
                    } else if i == current {
                        (c.accent_blue, "32px")
                    } else {
                        (c.bg_hover, "24px")
                    };
                    rsx! {
                        div {
                            key: "{i}",
                            style: "width: {w}; height: 4px; background: {bg}; border-radius: 2px; transition: all 0.3s;",
                        }
                    }
                }
            }
        }
    }
}


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
    let mut password_confirm = use_signal(String::new);
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
            // Audio playback désactivé — les assets voix seront dans le service Miou à terme.
            last_played_step.set(Some(s));
        }
    });

    // Validations dérivées
    let nom_valid = !pseudonyme.read().trim().is_empty();
    let email_valid = is_valid_email(&email.read());
    let pw = password.read().clone();
    let pw_checks = PasswordChecks::from_password(&pw);
    let pw_valid = pw_checks.all_valid();
    let pw_match = !password_confirm.read().is_empty() && *password.read() == *password_confirm.read();
    let can_sign = pw_valid && pw_match;

    let on_sign_done = move |_| {
        let pseudo = pseudonyme.read().trim().to_string();
        let em = email.read().trim().to_string();
        let pass = password.read().clone();
        let pass_confirm = password_confirm.read().clone();
        error.write().clear();
        if em.is_empty() {
            error.write().push_str("L'adresse e-mail est requise.");
            return;
        }
        if pass != pass_confirm {
            error.write().push_str("Les deux clés ne correspondent pas.");
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

            if !genre_val.is_empty() { profile.genre = Some(genre_val); }
            if !date_val.is_empty() { profile.date_naissance = Some(date_val); }
            if !ville_val.is_empty() { profile.ville = Some(ville_val); }
            if !num_val.is_empty() { profile.numero_voie = Some(num_val); }
            if !rue_val.is_empty() { profile.rue = Some(rue_val); }
            if !cp_val.is_empty() { profile.code_postal = Some(cp_val); }
            if !statut_val.is_empty() { profile.statut_marital = Some(statut_val); }
            if !part_genre_val.is_empty() { profile.partenaire_genre = Some(part_genre_val); }
            if !part_nom_val.is_empty() { profile.partenaire_nom = Some(part_nom_val); }
            if let Ok(n) = enf_nb_str.parse::<i32>() {
                if n > 0 { profile.enfants_nombre = Some(n); }
            }
            if !enf_noms_val.is_empty() { profile.enfants_noms = Some(enf_noms_val); }
            if !profession_val.is_empty() { profile.profession = Some(profession_val); }
            if !langue_val.is_empty() { profile.langue_maternelle = Some(langue_val); }

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

    // Style du bouton désactivé
    let disabled_btn = format!(
        "width: 100%; padding: 12px; border: none; border-radius: 4px; font-size: 14px; cursor: not-allowed; background: {}; color: {}; opacity: 0.5;",
        c.bg_hover, c.text_muted
    );

    rsx! {
        div {
            style: "{styles::fullscreen_container(theme)}",
            div {
                style: "max-width: 440px; width: 100%;",
                div {
                    style: "{styles::form_card(theme)}",

                    {if step() == STEP_NOM {
                        rsx! {
                            StepIndicator { current: 0, total: 3, theme }
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {c.text_secondary}; line-height: 1.5;", "Bienvenue à toi dans ton nouveau Miyukini COG. Avant d'emménager, peux-tu me dire quel est ton nom ?" }
                            input {
                                style: "{styles::form_input(theme)}",
                                r#type: "text",
                                placeholder: "Nom / pseudo",
                                value: "{pseudonyme()}",
                                oninput: move |evt| pseudonyme.set(evt.value()),
                            }
                            if nom_valid {
                                button {
                                    style: "{styles::form_btn_primary(theme)}",
                                    onclick: move |_| step.set(STEP_EMAIL),
                                    "Continuer"
                                }
                            } else {
                                button {
                                    style: "{disabled_btn}",
                                    disabled: true,
                                    "Continuer"
                                }
                            }
                        }
                    } else if step() == STEP_EMAIL {
                        rsx! {
                            StepIndicator { current: 1, total: 3, theme }
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {c.text_secondary}; line-height: 1.5;", "Pour pouvoir t'envoyer du courrier, peux-tu entrer ton adresse e-mail, s'il te plaît ?" }
                            input {
                                style: "{styles::form_input(theme)}",
                                r#type: "email",
                                placeholder: "Adresse e-mail",
                                value: "{email()}",
                                oninput: move |evt| email.set(evt.value()),
                            }
                            // Feedback email
                            if !email.read().is_empty() && !email_valid {
                                p {
                                    style: "font-size: 11px; color: {c.accent_orange}; margin-top: 4px;",
                                    "Format attendu : exemple@domaine.fr"
                                }
                            }
                            if email_valid {
                                button {
                                    style: "{styles::form_btn_primary(theme)}",
                                    onclick: move |_| step.set(STEP_CLE),
                                    "Continuer"
                                }
                            } else {
                                button {
                                    style: "{disabled_btn}",
                                    disabled: true,
                                    "Continuer"
                                }
                            }
                            button {
                                style: "width: 100%; padding: 8px; background: transparent; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px; margin-top: 4px;",
                                onclick: move |_| step.set(STEP_NOM),
                                "← Retour"
                            }
                        }
                    } else if step() == STEP_CLE {
                        rsx! {
                            StepIndicator { current: 2, total: 3, theme }
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 16px; color: {c.text_secondary}; line-height: 1.5;", "Pour finir, peux-tu me donner une clé pour protéger l'entrée ?" }
                            input {
                                style: "{styles::form_input(theme)} margin-bottom: 4px;",
                                r#type: "password",
                                placeholder: "Mot de passe",
                                value: "{password()}",
                                oninput: move |evt| password.set(evt.value()),
                            }
                            // Indicateur force mot de passe en temps réel
                            PasswordStrengthIndicator { password: password.read().clone(), theme }

                            // Confirmation mot de passe (apparaît quand le premier est assez fort)
                            if pw_valid {
                                div {
                                    style: "margin-top: 12px;",
                                    input {
                                        style: "{styles::form_input(theme)}",
                                        r#type: "password",
                                        placeholder: "Confirmer la clé",
                                        value: "{password_confirm()}",
                                        oninput: move |evt| password_confirm.set(evt.value()),
                                    }
                                    if !password_confirm.read().is_empty() && !pw_match {
                                        p {
                                            style: "font-size: 11px; color: {c.accent_red}; margin-top: 4px;",
                                            "Les deux clés ne correspondent pas."
                                        }
                                    }
                                    if pw_match {
                                        p {
                                            style: "font-size: 11px; color: {c.accent_green}; margin-top: 4px;",
                                            "✓ Les clés correspondent"
                                        }
                                    }
                                }
                            }

                            if !error().is_empty() {
                                p { style: "{styles::form_error(theme)}", "{error()}" }
                            }
                            if can_sign {
                                button {
                                    style: "{styles::form_btn_primary(theme)} margin-top: 12px;",
                                    onclick: on_sign_done,
                                    "🖋️ Signer"
                                }
                            } else {
                                button {
                                    style: "{disabled_btn} margin-top: 12px;",
                                    disabled: true,
                                    "🖋️ Signer"
                                }
                            }
                            button {
                                style: "width: 100%; padding: 8px; background: transparent; border: none; color: {c.text_muted}; cursor: pointer; font-size: 12px; margin-top: 4px;",
                                onclick: move |_| step.set(STEP_EMAIL),
                                "← Retour"
                            }
                        }
                    } else {
                        // STEP_INFOS — infos complémentaires par catégorie
                        rsx! {
                            // Progression catégories
                            div {
                                style: "display: flex; align-items: center; justify-content: center; gap: 6px; margin-bottom: 16px;",
                                for i in 0..CAT_COUNT {
                                    {
                                        let bg = if i <= cat_idx() { c.accent_blue } else { c.bg_hover };
                                        rsx! {
                                            div {
                                                key: "{i}",
                                                style: "width: 40px; height: 4px; background: {bg}; border-radius: 2px; transition: all 0.3s;",
                                            }
                                        }
                                    }
                                }
                            }
                            p { style: "{styles::form_title(theme)}", "— Miou" }
                            p { style: "font-size: 14px; margin-bottom: 20px; color: {c.text_secondary}; line-height: 1.5;", "Pour que je te connaisse un peu mieux, peux-tu me répondre à quelques questions ? Tu peux passer celles que tu ne veux pas partager." }

                            {match cat_idx() {
                                0 => rsx! {
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
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Date de naissance" }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 16px;",
                                        r#type: "date",
                                        value: "{date_naissance()}",
                                        oninput: move |evt| date_naissance.set(evt.value()),
                                    }
                                },
                                1 => rsx! {
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
                                    p { style: "font-size: 13px; font-weight: 600; color: {c.text_white}; margin-bottom: 8px;", "Contexte" }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Quelle est ta profession ou activité ?" }
                                    input {
                                        style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                        r#type: "text",
                                        placeholder: "Profession / activité",
                                        value: "{profession()}",
                                        oninput: move |evt| profession.set(evt.value()),
                                    }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Quelle est ta langue maternelle ?" }
                                    select {
                                        style: "{styles::form_input(theme)} margin-bottom: 12px;",
                                        value: "{langue_maternelle()}",
                                        oninput: move |evt| langue_maternelle.set(evt.value()),
                                        option { value: "", "— Choisir —" }
                                        option { value: "Français", "Français" }
                                        option { value: "Anglais", "Anglais" }
                                        option { value: "Espagnol", "Espagnol" }
                                        option { value: "Allemand", "Allemand" }
                                        option { value: "Arabe", "Arabe" }
                                        option { value: "Portugais", "Portugais" }
                                        option { value: "Autre", "Autre" }
                                    }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-bottom: 8px;", "Rappels et notifications ?" }
                                    select {
                                        style: "{styles::form_input(theme)} margin-bottom: 16px;",
                                        value: "{preference_rappel()}",
                                        oninput: move |evt| preference_rappel.set(evt.value()),
                                        option { value: "", "— Choisir —" }
                                        option { value: "Oui, toutes", "Oui, toutes" }
                                        option { value: "Occasionnelles", "Occasionnellement" }
                                        option { value: "Non", "Non merci" }
                                    }
                                },
                            }}

                            // Boutons Passer / Suivant / Terminer
                            div {
                                style: "display: flex; gap: 12px; margin-top: 12px;",

                                button {
                                    style: "flex: 1; padding: 10px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_muted}; cursor: pointer; font-size: 13px;",
                                    onclick: on_pass_cat,
                                    "Passer"
                                }

                                if cat_idx() + 1 >= CAT_COUNT {
                                    button {
                                        style: "flex: 1; padding: 10px; background: {c.accent_green}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px; font-weight: 600;",
                                        onclick: on_next_cat,
                                        "✨ Entrer dans mon COG"
                                    }
                                } else {
                                    button {
                                        style: "flex: 1; padding: 10px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 13px; font-weight: 500;",
                                        onclick: on_next_cat,
                                        "Suivant →"
                                    }
                                }
                            }

                            // Bouton "Tout passer"
                            if cat_idx() == 0 {
                                button {
                                    style: "width: 100%; padding: 8px; background: transparent; border: none; color: {c.text_muted}; cursor: pointer; font-size: 11px; margin-top: 8px;",
                                    onclick: move |_| on_infos_done(()),
                                    "Passer toutes les questions et entrer directement"
                                }
                            }
                        }
                    }}
                }
            }
        }
    }
}
