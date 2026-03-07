//! UNC-E11/E12/E13/E14 — Auth facade publique (CTA modal, Connexion, Inscription, Mentions).
//!
//! Gestion de l'authentification et modals CTA pour utilisateurs non connectes.
//!
//! @id: jf_unc_auth @do: render_unc_auth
//! @role: ui @layer: service
//! @human: Ecrans UNC-E11/E14 JayFestival: authentification et modals CTA (connexion, inscription, mentions).

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::{UncSection, JayFestivalRole, JayFestivalState};
use super::components::ActionButton;

#[allow(dead_code)]
fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// UNC-E11 — Modal CTA contextuel.
#[component]
pub fn UncCtaModal(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let action = state.read().cta_action.clone().unwrap_or_default();
    let (title, message, target_role) = match action.as_str() {
        "candidater" => (
            "Candidater en tant qu'exposant",
            "Pour deposer une candidature, vous devez etre connecte en tant qu'exposant.",
            "exposant",
        ),
        "reserver" => (
            "Reserver une place",
            "Pour effectuer une reservation, vous devez etre connecte en tant que visiteur.",
            "visiteur",
        ),
        "organiser" => (
            "Creer un evenement",
            "Pour creer et gerer des evenements, vous devez etre connecte en tant qu'organisateur.",
            "organisateur",
        ),
        _ => (
            "Action requiert une connexion",
            "Connectez-vous ou inscrivez-vous pour continuer.",
            "",
        ),
    };

    rsx! {
        div {
            style: "position: fixed; inset: 0; background: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; z-index: 1000;",
            onclick: move |_| {
                state.write().unc_section = UncSection::Landing;
            },

            div {
                style: "background: {c.bg_main}; border-radius: 12px; padding: 32px; max-width: 400px; width: 90%;",
                onclick: move |evt| evt.stop_propagation(),

                h2 {
                    style: "font-size: 20px; color: {c.text_white}; margin-bottom: 12px;",
                    "{title}"
                }
                p {
                    style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 24px;",
                    "{message}"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    ActionButton {
                        label: "Se connecter".to_string(),
                        icon: "🔑".to_string(),
                        accent: true,
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Connexion;
                        },
                    }

                    ActionButton {
                        label: "S'inscrire".to_string(),
                        icon: "✨".to_string(),
                        accent: false,
                        onclick: {
                            let target = target_role.to_string();
                            move |_| {
                                state.write().inscription_type = if target.is_empty() { None } else { Some(target.clone()) };
                                state.write().unc_section = UncSection::Inscription;
                            }
                        },
                    }

                    button {
                        style: "padding: 10px; background: transparent; border: none; color: {c.text_muted}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            state.write().unc_section = UncSection::Landing;
                        },
                        "Annuler"
                    }
                }
            }
        }
    }
}

/// UNC-E12 — Page de connexion.
#[component]
pub fn UncConnexion(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let handle_submit = move |evt: FormEvent| {
        evt.prevent_default();
        loading.set(true);
        error.set(None);

        let db = &conns.read().jayfestival;
        match db.profile_by_email_password(&email.read(), &password.read()) {
            Ok(Some(profile)) => {
                // Connexion réussie - basculer vers l'espace approprié
                let role = match profile.user_type.as_deref() {
                    Some("organisateur") => JayFestivalRole::Organisateur,
                    Some("exposant") => JayFestivalRole::Exposant,
                    _ => JayFestivalRole::Visiteur,
                };
                state.write().role = role;
                state.write().is_connected = true;
                state.write().current_user_id = profile.id;
            }
            Ok(None) => {
                error.set(Some("Email ou mot de passe incorrect".to_string()));
            }
            Err(e) => {
                error.set(Some(format!("Erreur: {e}")));
            }
        }
        loading.set(false);
    };

    rsx! {
        div {
            style: "max-width: 400px; margin: 60px auto;",

            // Header
            div {
                style: "text-align: center; margin-bottom: 32px;",

                button {
                    style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px; margin-bottom: 16px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Landing;
                    },
                    "← Retour"
                }

                span {
                    style: "font-size: 48px; display: block; margin-bottom: 16px;",
                    "🔐"
                }
                h1 {
                    style: "font-size: 24px; color: {c.text_white}; margin-bottom: 8px;",
                    "Connexion"
                }
                p {
                    style: "font-size: 14px; color: {c.text_secondary};",
                    "Connectez-vous a votre compte JayFestival"
                }
            }

            // Formulaire
            form {
                style: "display: flex; flex-direction: column; gap: 16px;",
                onsubmit: handle_submit,

                // Email
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Email"
                    }
                    input {
                        r#type: "email",
                        style: "width: 100%; padding: 12px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                        placeholder: "votre@email.com",
                        value: "{email}",
                        oninput: move |evt| email.set(evt.value()),
                        required: true,
                    }
                }

                // Mot de passe
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Mot de passe"
                    }
                    input {
                        r#type: "password",
                        style: "width: 100%; padding: 12px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                        placeholder: "••••••••",
                        value: "{password}",
                        oninput: move |evt| password.set(evt.value()),
                        required: true,
                    }
                }

                // Erreur
                if let Some(err) = error.read().as_ref() {
                    div {
                        style: "padding: 12px; background: {c.accent_red}20; border-radius: 6px; color: {c.accent_red}; font-size: 13px;",
                        "{err}"
                    }
                }

                // Bouton submit
                button {
                    r#type: "submit",
                    style: "width: 100%; padding: 14px; background: {c.accent_blue}; border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 14px; font-weight: 500;",
                    disabled: *loading.read(),
                    if *loading.read() { "Connexion..." } else { "Se connecter" }
                }
            }

            // Lien inscription
            div {
                style: "text-align: center; margin-top: 24px;",

                span {
                    style: "color: {c.text_secondary}; font-size: 14px;",
                    "Pas encore de compte ? "
                }
                button {
                    style: "background: none; border: none; color: {c.accent_blue}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Inscription;
                    },
                    "S'inscrire"
                }
            }
        }
    }
}

/// UNC-E13 — Page d'inscription (choix type puis formulaire).
#[component]
pub fn UncInscription(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let inscription_type = state.read().inscription_type.clone();

    // Si pas de type choisi, afficher le choix
    if inscription_type.is_none() {
        return rsx! {
            UncInscriptionChoice { state: state }
        };
    }

    let user_type = inscription_type.unwrap();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut password_confirm = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);

    let handle_submit = {
        let user_type = user_type.clone();
        move |evt: FormEvent| {
            evt.prevent_default();
            // Validation
            if password.read().len() < 6 {
                error.set(Some("Le mot de passe doit contenir au moins 6 caracteres".to_string()));
                return;
            }
            if *password.read() != *password_confirm.read() {
                error.set(Some("Les mots de passe ne correspondent pas".to_string()));
                return;
            }

            loading.set(true);
            error.set(None);

            let db = &conns.read().jayfestival;
            match db.profile_create(&email.read(), &password.read(), &user_type) {
                Ok(id) => {
                    // Inscription réussie - connecter automatiquement
                    let role = match user_type.as_str() {
                        "organisateur" => JayFestivalRole::Organisateur,
                        "exposant" => JayFestivalRole::Exposant,
                        _ => JayFestivalRole::Visiteur,
                    };
                    state.write().role = role;
                    state.write().is_connected = true;
                    state.write().current_user_id = Some(id);
                }
                Err(e) => {
                    error.set(Some(format!("Erreur: {e}")));
                }
            }
            loading.set(false);
        }
    };

    let type_label = match user_type.as_str() {
        "organisateur" => "Organisateur",
        "exposant" => "Exposant",
        "visiteur" => "Visiteur",
        _ => "Utilisateur",
    };

    let type_icon = match user_type.as_str() {
        "organisateur" => "🏢",
        "exposant" => "🧑‍💼",
        _ => "👤",
    };

    rsx! {
        div {
            style: "max-width: 400px; margin: 60px auto;",

            // Header
            div {
                style: "text-align: center; margin-bottom: 32px;",

                button {
                    style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px; margin-bottom: 16px;",
                    onclick: move |_| {
                        state.write().inscription_type = None;
                    },
                    "← Changer de type"
                }

                span {
                    style: "font-size: 48px; display: block; margin-bottom: 16px;",
                    "{type_icon}"
                }
                h1 {
                    style: "font-size: 24px; color: {c.text_white}; margin-bottom: 8px;",
                    "Inscription {type_label}"
                }
                p {
                    style: "font-size: 14px; color: {c.text_secondary};",
                    "Creez votre compte JayFestival"
                }
            }

            // Formulaire
            form {
                style: "display: flex; flex-direction: column; gap: 16px;",
                onsubmit: handle_submit,

                // Email
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Email"
                    }
                    input {
                        r#type: "email",
                        style: "width: 100%; padding: 12px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                        placeholder: "votre@email.com",
                        value: "{email}",
                        oninput: move |evt| email.set(evt.value()),
                        required: true,
                    }
                }

                // Mot de passe
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Mot de passe"
                    }
                    input {
                        r#type: "password",
                        style: "width: 100%; padding: 12px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                        placeholder: "Minimum 6 caracteres",
                        value: "{password}",
                        oninput: move |evt| password.set(evt.value()),
                        required: true,
                    }
                }

                // Confirmation mot de passe
                div {
                    label {
                        style: "display: block; font-size: 12px; color: {c.text_secondary}; margin-bottom: 6px;",
                        "Confirmer le mot de passe"
                    }
                    input {
                        r#type: "password",
                        style: "width: 100%; padding: 12px 16px; background: {c.bg_secondary}; border: 1px solid {c.border}; border-radius: 8px; color: {c.text_primary}; font-size: 14px;",
                        placeholder: "••••••••",
                        value: "{password_confirm}",
                        oninput: move |evt| password_confirm.set(evt.value()),
                        required: true,
                    }
                }

                // Erreur
                if let Some(err) = error.read().as_ref() {
                    div {
                        style: "padding: 12px; background: {c.accent_red}20; border-radius: 6px; color: {c.accent_red}; font-size: 13px;",
                        "{err}"
                    }
                }

                // Bouton submit
                button {
                    r#type: "submit",
                    style: "width: 100%; padding: 14px; background: {c.accent_blue}; border: none; border-radius: 8px; color: white; cursor: pointer; font-size: 14px; font-weight: 500;",
                    disabled: *loading.read(),
                    if *loading.read() { "Inscription..." } else { "Creer mon compte" }
                }
            }

            // Lien connexion
            div {
                style: "text-align: center; margin-top: 24px;",

                span {
                    style: "color: {c.text_secondary}; font-size: 14px;",
                    "Deja un compte ? "
                }
                button {
                    style: "background: none; border: none; color: {c.accent_blue}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Connexion;
                    },
                    "Se connecter"
                }
            }
        }
    }
}

/// Choix du type d'inscription.
#[component]
fn UncInscriptionChoice(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "max-width: 600px; margin: 60px auto;",

            // Header
            div {
                style: "text-align: center; margin-bottom: 32px;",

                button {
                    style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px; margin-bottom: 16px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Landing;
                    },
                    "← Retour"
                }

                h1 {
                    style: "font-size: 24px; color: {c.text_white}; margin-bottom: 8px;",
                    "Rejoindre JayFestival"
                }
                p {
                    style: "font-size: 14px; color: {c.text_secondary};",
                    "Choisissez votre type de compte"
                }
            }

            // Options
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                InscriptionTypeCard {
                    icon: "🏢",
                    title: "Organisateur",
                    description: "Creez et gerez vos evenements, festivals et salons",
                    onclick: move |_| {
                        state.write().inscription_type = Some("organisateur".to_string());
                    },
                }

                InscriptionTypeCard {
                    icon: "🧑‍💼",
                    title: "Exposant",
                    description: "Participez a des evenements et presentez votre activite",
                    onclick: move |_| {
                        state.write().inscription_type = Some("exposant".to_string());
                    },
                }

                InscriptionTypeCard {
                    icon: "👤",
                    title: "Visiteur",
                    description: "Decouvrez les evenements et reservez vos places",
                    onclick: move |_| {
                        state.write().inscription_type = Some("visiteur".to_string());
                    },
                }
            }

            // Lien connexion
            div {
                style: "text-align: center; margin-top: 32px;",

                span {
                    style: "color: {c.text_secondary}; font-size: 14px;",
                    "Deja un compte ? "
                }
                button {
                    style: "background: none; border: none; color: {c.accent_blue}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Connexion;
                    },
                    "Se connecter"
                }
            }
        }
    }
}

/// Carte de choix de type d'inscription.
#[component]
fn InscriptionTypeCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 12px; padding: 24px; text-align: center; cursor: pointer; transition: transform 0.2s, border-color 0.2s; border: 2px solid transparent;",
            onclick: move |evt| onclick.call(evt),

            span {
                style: "font-size: 40px; display: block; margin-bottom: 16px;",
                "{icon}"
            }
            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 8px;",
                "{title}"
            }
            p {
                style: "font-size: 13px; color: {c.text_secondary};",
                "{description}"
            }
        }
    }
}

/// UNC-E14 — Mentions legales / CGU / Confidentialite.
#[component]
pub fn UncMentionsLegales(state: Signal<JayFestivalState>) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let active_tab = use_signal(|| "mentions".to_string());
    
    let tab = active_tab.read().clone();
    let is_mentions = tab == "mentions";
    let is_cgu = tab == "cgu";
    let is_confidentialite = tab == "confidentialite";
    let is_accessibilite = tab == "accessibilite";

    rsx! {
        div {
            style: "max-width: 800px; margin: 0 auto;",

            // Header
            div {
                style: "margin-bottom: 24px;",

                button {
                    style: "background: none; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        state.write().unc_section = UncSection::Landing;
                    },
                    "← Retour"
                }
                h1 {
                    style: "font-size: 24px; color: {c.text_white}; margin-top: 8px;",
                    "Informations legales"
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 8px; margin-bottom: 24px; border-bottom: 1px solid {c.border}; padding-bottom: 12px;",

                MentionsTab { label: "Mentions legales", id: "mentions", active_tab: active_tab, is_active: is_mentions }
                MentionsTab { label: "CGU", id: "cgu", active_tab: active_tab, is_active: is_cgu }
                MentionsTab { label: "Confidentialite", id: "confidentialite", active_tab: active_tab, is_active: is_confidentialite }
                MentionsTab { label: "Accessibilite", id: "accessibilite", active_tab: active_tab, is_active: is_accessibilite }
            }

            // Contenu
            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 24px;",

                if is_mentions {
                    MentionsContent {}
                }
                if is_cgu {
                    CguContent {}
                }
                if is_confidentialite {
                    ConfidentialiteContent {}
                }
                if is_accessibilite {
                    AccessibiliteContent {}
                }
            }
        }
    }
}

#[component]
fn MentionsTab(label: &'static str, id: &'static str, active_tab: Signal<String>, is_active: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let border = if is_active { c.accent_blue } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_muted };
    
    rsx! {
        button {
            style: "padding: 8px 16px; background: transparent; border: none; border-bottom: 2px solid {border}; color: {color}; cursor: pointer; font-size: 13px;",
            onclick: move |_| active_tab.set(id.to_string()),
            "{label}"
        }
    }
}

#[component]
fn MentionsContent() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        h2 {
            style: "font-size: 18px; color: {c.text_white}; margin-bottom: 16px;",
            "Mentions legales"
        }
        p {
            style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.8;",
            "JayFestival est un service de gestion d'evenements integre a l'ecosysteme Miyukini COG."
        }
        p {
            style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.8; margin-top: 16px;",
            "Editeur : Miyukini — Contact : contact@miyukini.com"
        }
    }
}

#[component]
fn CguContent() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        h2 {
            style: "font-size: 18px; color: {c.text_white}; margin-bottom: 16px;",
            "Conditions Generales d'Utilisation"
        }
        p {
            style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.8;",
            "En utilisant JayFestival, vous acceptez les presentes conditions d'utilisation."
        }
    }
}

#[component]
fn ConfidentialiteContent() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        h2 {
            style: "font-size: 18px; color: {c.text_white}; margin-bottom: 16px;",
            "Politique de confidentialite"
        }
        p {
            style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.8;",
            "Vos donnees personnelles sont traitees conformement au RGPD. Elles ne sont jamais vendues a des tiers."
        }
    }
}

#[component]
fn AccessibiliteContent() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        h2 {
            style: "font-size: 18px; color: {c.text_white}; margin-bottom: 16px;",
            "Declaration d'accessibilite"
        }
        p {
            style: "font-size: 14px; color: {c.text_secondary}; line-height: 1.8;",
            "JayFestival s'engage a rendre son service accessible a tous. Nous travaillons continuellement a ameliorer l'accessibilite de notre plateforme."
        }
    }
}
