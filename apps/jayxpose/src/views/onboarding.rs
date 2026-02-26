//! Onboarding JayXpose — Parcours guidé par Miou pour la création du profil exposant.
//!
//! Didacticiel en 6 étapes avec Miou qui explique chaque bloc d'information :
//! 0. Bienvenue — Miou explique JayXpose
//! 1. Vie privée — Miou rassure sur la confidentialité locale
//! 2. Identité — Nom de l'entreprise, forme juridique, slogan
//! 3. Description — Résumé et présentation détaillée
//! 4. Contact — Email, téléphone, adresse
//! 5. Félicitations — Profil créé, redirection vers le dashboard

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use super::components::{FormField, FormTextarea};
use super::{JayXposeState, JayXposeSection};

/// Nombre total d'étapes de l'onboarding.
const STEP_COUNT: u8 = 6;

/// Étapes de l'onboarding.
const STEP_BIENVENUE: u8 = 0;
const STEP_VIE_PRIVEE: u8 = 1;
const STEP_IDENTITE: u8 = 2;
const STEP_DESCRIPTION: u8 = 3;
const STEP_CONTACT: u8 = 4;
const STEP_FELICITATIONS: u8 = 5;

// ── Miou messages ────────────────────────────────────────────────────

/// Texte Miou pour chaque étape (titre, message, tip).
struct MiouMessage {
    titre: &'static str,
    message: &'static str,
    tip: &'static str,
}

fn miou_message(step: u8) -> MiouMessage {
    match step {
        STEP_BIENVENUE => MiouMessage {
            titre: "Bienvenue dans JayXpose !",
            message: "JayXpose est ton espace professionnel. Il te permet de créer un \
                      profil exposant complet, de gérer ton catalogue de produits, de \
                      construire ta vitrine en ligne et de stocker tes documents \
                      professionnels dans un coffre-fort sécurisé. C'est ton stand \
                      numérique pour te présenter aux organisateurs d'événements et \
                      aux visiteurs.",
            tip: "Tu pourras toujours modifier ces informations plus tard depuis la fiche entreprise.",
        },
        STEP_VIE_PRIVEE => MiouMessage {
            titre: "Tes données restent chez toi",
            message: "Je tiens à te rassurer : toutes les informations que tu vas saisir \
                      sont stockées uniquement sur ton ordinateur, dans ta base de données \
                      locale. Rien n'est envoyé sur Internet sans ton accord explicite. \
                      Tu gardes le contrôle total de tes données. Quand tu décideras de \
                      publier ta vitrine, tu choisiras toi-même ce qui sera visible.",
            tip: "Le coffre-fort documentaire chiffre tes fichiers localement. Même tes documents sensibles sont protégés.",
        },
        STEP_IDENTITE => MiouMessage {
            titre: "Commençons par l'essentiel",
            message: "Pour créer ton profil, j'ai besoin de connaître le nom de ton \
                      entreprise. Les autres champs sont optionnels pour l'instant, \
                      mais un slogan accrocheur peut faire la différence !",
            tip: "Le nom de l'entreprise sera affiché en gros sur ta vitrine et dans l'annuaire des exposants.",
        },
        STEP_DESCRIPTION => MiouMessage {
            titre: "Présente ton activité",
            message: "La description courte apparaîtra dans les résultats de recherche \
                      et l'annuaire. C'est ta carte de visite en 2-3 lignes. \
                      La description longue sera affichée sur ta page vitrine pour \
                      les visiteurs qui veulent en savoir plus.",
            tip: "Pas d'inspiration ? Tu peux simplement décrire ce que tu fais, pour qui, et ce qui te rend unique.",
        },
        STEP_CONTACT => MiouMessage {
            titre: "Comment te contacter ?",
            message: "Ces informations permettront aux organisateurs et aux visiteurs \
                      intéressés de te joindre. L'email et le téléphone sont les \
                      plus importants. L'adresse est utile pour la logistique \
                      des événements.",
            tip: "Tu pourras configurer la visibilité de chaque champ dans les paramètres de confidentialité.",
        },
        STEP_FELICITATIONS => MiouMessage {
            titre: "Ton profil est prêt !",
            message: "Félicitations ! Ton profil exposant est maintenant créé. \
                      Tu peux dès à présent enrichir ta fiche entreprise avec \
                      des informations juridiques, ajouter des produits à ton \
                      catalogue, et configurer ta vitrine. Tout se fait depuis \
                      le menu à gauche.",
            tip: "Mon conseil : commence par ajouter quelques produits à ton catalogue pour donner vie à ton stand.",
        },
        _ => MiouMessage {
            titre: "Miou",
            message: "",
            tip: "",
        },
    }
}

// ── Sous-composants ──────────────────────────────────────────────────

/// Indicateur de progression (cercles étapes).
#[component]
fn OnboardingProgress(current: u8, total: u8) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: center; gap: 6px; margin-bottom: 24px;",

            for i in 0..total {
                {
                    let is_done = i < current;
                    let is_current = i == current;
                    let bg = if is_done || is_current { c.accent_blue } else { c.bg_hover };
                    let w = if is_current { "32px" } else { "24px" };
                    let opacity = if is_done { "0.6" } else { "1" };
                    rsx! {
                        div {
                            key: "{i}",
                            style: "width: {w}; height: 4px; background: {bg}; border-radius: 2px; transition: all 0.3s; opacity: {opacity};",
                        }
                    }
                }
            }
        }
    }
}

/// Bulle Miou inline (style didacticiel, intégrée au flux onboarding).
#[component]
fn MiouBubbleInline(step: u8) -> Element {
    let c = use_palette();
    let msg = miou_message(step);

    rsx! {
        div {
            style: "background: linear-gradient(135deg, #2d3748 0%, #1a202c 100%); \
                    border: 1px solid #4a5568; border-radius: 12px; padding: 20px; \
                    margin-bottom: 24px; box-shadow: 0 4px 16px rgba(0, 0, 0, 0.3);",

            // En-tête Miou
            div {
                style: "display: flex; align-items: center; gap: 10px; margin-bottom: 14px; \
                        padding-bottom: 10px; border-bottom: 1px solid #4a5568;",

                // Avatar Miou
                div {
                    style: "width: 36px; height: 36px; border-radius: 50%; \
                            background: linear-gradient(135deg, #e91e63, #c2185b); \
                            display: flex; align-items: center; justify-content: center; \
                            font-size: 18px; flex-shrink: 0;",
                    "🌸"
                }

                div {
                    span {
                        style: "font-weight: 600; font-size: 14px; color: #ffd4e5;",
                        "Miou"
                    }
                    span {
                        style: "font-size: 11px; color: {c.text_muted}; margin-left: 8px;",
                        "— {msg.titre}"
                    }
                }
            }

            // Message principal
            p {
                style: "font-size: 14px; line-height: 1.6; color: #e2e8f0; margin-bottom: 12px;",
                "{msg.message}"
            }

            // Tip
            if !msg.tip.is_empty() {
                div {
                    style: "display: flex; align-items: flex-start; gap: 8px; padding: 10px 12px; \
                            background: rgba(26, 159, 255, 0.08); border-radius: 6px; \
                            border-left: 3px solid {c.accent_blue};",

                    span {
                        style: "font-size: 14px; flex-shrink: 0; margin-top: 1px;",
                        "💡"
                    }
                    p {
                        style: "font-size: 12px; line-height: 1.5; color: {c.text_secondary};",
                        "{msg.tip}"
                    }
                }
            }
        }
    }
}

// ── Composant principal ──────────────────────────────────────────────

#[component]
pub fn JayXposeOnboarding(state: Signal<JayXposeState>) -> Element {
    let c = use_palette();
    let db = crate::use_db();
    let mut step = use_signal(|| STEP_BIENVENUE);

    // Champs du profil
    let mut company_name = use_signal(String::new);
    let mut legal_form = use_signal(String::new);
    let mut slogan = use_signal(String::new);
    let mut description_short = use_signal(String::new);
    let mut description_long = use_signal(String::new);
    let mut contact_email = use_signal(String::new);
    let mut contact_phone = use_signal(String::new);
    let mut adresse_siege = use_signal(String::new);

    // Validation
    let has_company_name = !company_name.read().trim().is_empty();

    // Style bouton désactivé
    let disabled_btn = format!(
        "flex: 1; padding: 12px; border: none; border-radius: 6px; font-size: 14px; \
         cursor: not-allowed; background: {}; color: {}; opacity: 0.4; font-weight: 500;",
        c.bg_hover, c.text_muted
    );

    // Style bouton suivant
    let next_btn = format!(
        "flex: 1; padding: 12px; background: {}; color: white; border: none; \
         border-radius: 6px; cursor: pointer; font-size: 14px; font-weight: 500; \
         transition: all 0.2s;",
        c.accent_blue
    );

    // Style bouton retour
    let back_btn = format!(
        "flex: 1; padding: 12px; background: transparent; border: 1px solid {}; \
         border-radius: 6px; color: {}; cursor: pointer; font-size: 14px; \
         transition: all 0.2s;",
        c.border, c.text_secondary
    );

    // Style bouton final
    let final_btn = format!(
        "flex: 1; padding: 14px; background: {}; color: white; border: none; \
         border-radius: 6px; cursor: pointer; font-size: 15px; font-weight: 600; \
         transition: all 0.2s;",
        c.accent_green
    );

    rsx! {
        div {
            style: "display: flex; align-items: flex-start; justify-content: center; \
                    min-height: 100%; padding: 32px 24px;",

            div {
                style: "max-width: 560px; width: 100%;",

                // Indicateur de progression
                OnboardingProgress { current: step(), total: STEP_COUNT }

                // Bulle Miou contextuelle
                MiouBubbleInline { step: step() }

                // ── Contenu par étape ────────────────────────────────

                {match step() {
                    STEP_BIENVENUE => rsx! {
                        // Illustration des fonctionnalités
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 24px;",

                            // Carte Profil
                            div {
                                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; \
                                        border: 1px solid {c.border}; text-align: center;",
                                span { style: "font-size: 28px; display: block; margin-bottom: 8px;", "🏢" }
                                p { style: "font-size: 13px; font-weight: 500; color: {c.text_white}; margin-bottom: 4px;", "Profil entreprise" }
                                p { style: "font-size: 11px; color: {c.text_muted};", "Ta carte d'identité pro" }
                            }
                            // Carte Catalogue
                            div {
                                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; \
                                        border: 1px solid {c.border}; text-align: center;",
                                span { style: "font-size: 28px; display: block; margin-bottom: 8px;", "📦" }
                                p { style: "font-size: 13px; font-weight: 500; color: {c.text_white}; margin-bottom: 4px;", "Catalogue produits" }
                                p { style: "font-size: 11px; color: {c.text_muted};", "Tes produits et services" }
                            }
                            // Carte Vitrine
                            div {
                                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; \
                                        border: 1px solid {c.border}; text-align: center;",
                                span { style: "font-size: 28px; display: block; margin-bottom: 8px;", "🪟" }
                                p { style: "font-size: 13px; font-weight: 500; color: {c.text_white}; margin-bottom: 4px;", "Vitrine en ligne" }
                                p { style: "font-size: 11px; color: {c.text_muted};", "Ton stand numérique" }
                            }
                            // Carte Documents
                            div {
                                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; \
                                        border: 1px solid {c.border}; text-align: center;",
                                span { style: "font-size: 28px; display: block; margin-bottom: 8px;", "🔒" }
                                p { style: "font-size: 13px; font-weight: 500; color: {c.text_white}; margin-bottom: 4px;", "Coffre-fort docs" }
                                p { style: "font-size: 11px; color: {c.text_muted};", "Tes documents sécurisés" }
                            }
                        }

                        // Bouton suivant
                        div {
                            style: "display: flex; gap: 12px;",
                            button {
                                style: "{next_btn}",
                                onclick: move |_| step.set(STEP_VIE_PRIVEE),
                                "Commencer →"
                            }
                        }
                    },

                    STEP_VIE_PRIVEE => rsx! {
                        // Illustration sécurité
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; \
                                    margin-bottom: 24px; border: 1px solid {c.border};",

                            div {
                                style: "display: flex; align-items: center; gap: 16px; margin-bottom: 16px;",
                                span { style: "font-size: 36px;", "🛡️" }
                                div {
                                    p { style: "font-size: 14px; font-weight: 600; color: {c.text_white};", "Stockage 100% local" }
                                    p { style: "font-size: 12px; color: {c.text_secondary}; margin-top: 2px;", "Tes données restent sur cet ordinateur" }
                                }
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 10px;",

                                // Point 1
                                div {
                                    style: "display: flex; align-items: center; gap: 10px;",
                                    span { style: "color: {c.accent_green}; font-size: 14px; flex-shrink: 0;", "✓" }
                                    p { style: "font-size: 13px; color: {c.text_primary};", "Base de données locale chiffrée" }
                                }
                                // Point 2
                                div {
                                    style: "display: flex; align-items: center; gap: 10px;",
                                    span { style: "color: {c.accent_green}; font-size: 14px; flex-shrink: 0;", "✓" }
                                    p { style: "font-size: 13px; color: {c.text_primary};", "Aucun envoi sur Internet sans ton accord" }
                                }
                                // Point 3
                                div {
                                    style: "display: flex; align-items: center; gap: 10px;",
                                    span { style: "color: {c.accent_green}; font-size: 14px; flex-shrink: 0;", "✓" }
                                    p { style: "font-size: 13px; color: {c.text_primary};", "Tu choisis ce que tu publies" }
                                }
                                // Point 4
                                div {
                                    style: "display: flex; align-items: center; gap: 10px;",
                                    span { style: "color: {c.accent_green}; font-size: 14px; flex-shrink: 0;", "✓" }
                                    p { style: "font-size: 13px; color: {c.text_primary};", "Documents protégés dans un coffre-fort" }
                                }
                            }
                        }

                        // Boutons
                        div {
                            style: "display: flex; gap: 12px;",
                            button {
                                style: "{back_btn}",
                                onclick: move |_| step.set(STEP_BIENVENUE),
                                "← Retour"
                            }
                            button {
                                style: "{next_btn}",
                                onclick: move |_| step.set(STEP_IDENTITE),
                                "J'ai compris, on continue →"
                            }
                        }
                    },

                    STEP_IDENTITE => rsx! {
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; \
                                    margin-bottom: 24px; display: flex; flex-direction: column; gap: 16px;",

                            h3 {
                                style: "font-size: 14px; color: {c.text_white}; font-weight: 600; \
                                        padding-bottom: 8px; border-bottom: 1px solid {c.border};",
                                "Identité de l'entreprise"
                            }

                            FormField {
                                label: "Raison sociale *".to_string(),
                                value: company_name.read().clone(),
                                placeholder: "Nom de ton entreprise".to_string(),
                                oninput: move |evt: FormEvent| company_name.set(evt.value()),
                            }

                            FormField {
                                label: "Forme juridique".to_string(),
                                value: legal_form.read().clone(),
                                placeholder: "SARL, SAS, Auto-entrepreneur...".to_string(),
                                optional: true,
                                oninput: move |evt: FormEvent| legal_form.set(evt.value()),
                            }

                            FormField {
                                label: "Slogan".to_string(),
                                value: slogan.read().clone(),
                                placeholder: "Ton accroche en une phrase".to_string(),
                                optional: true,
                                oninput: move |evt: FormEvent| slogan.set(evt.value()),
                            }
                        }

                        // Boutons
                        div {
                            style: "display: flex; gap: 12px;",
                            button {
                                style: "{back_btn}",
                                onclick: move |_| step.set(STEP_VIE_PRIVEE),
                                "← Retour"
                            }
                            if has_company_name {
                                button {
                                    style: "{next_btn}",
                                    onclick: move |_| step.set(STEP_DESCRIPTION),
                                    "Suivant →"
                                }
                            } else {
                                button {
                                    style: "{disabled_btn}",
                                    disabled: true,
                                    "Suivant →"
                                }
                            }
                        }
                    },

                    STEP_DESCRIPTION => rsx! {
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; \
                                    margin-bottom: 24px; display: flex; flex-direction: column; gap: 16px;",

                            h3 {
                                style: "font-size: 14px; color: {c.text_white}; font-weight: 600; \
                                        padding-bottom: 8px; border-bottom: 1px solid {c.border};",
                                "Présentation"
                            }

                            FormTextarea {
                                label: "Description courte".to_string(),
                                value: description_short.read().clone(),
                                placeholder: "Résumé de ton activité en 2-3 lignes (annuaire, recherche)".to_string(),
                                rows: 3,
                                oninput: move |evt: FormEvent| description_short.set(evt.value()),
                            }

                            FormTextarea {
                                label: "Description longue".to_string(),
                                value: description_long.read().clone(),
                                placeholder: "Présentation détaillée pour ta page vitrine".to_string(),
                                rows: 6,
                                optional: true,
                                oninput: move |evt: FormEvent| description_long.set(evt.value()),
                            }
                        }

                        // Boutons
                        div {
                            style: "display: flex; gap: 12px;",
                            button {
                                style: "{back_btn}",
                                onclick: move |_| step.set(STEP_IDENTITE),
                                "← Retour"
                            }
                            button {
                                style: "{next_btn}",
                                onclick: move |_| step.set(STEP_CONTACT),
                                "Suivant →"
                            }
                        }
                    },

                    STEP_CONTACT => rsx! {
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; \
                                    margin-bottom: 24px; display: flex; flex-direction: column; gap: 16px;",

                            h3 {
                                style: "font-size: 14px; color: {c.text_white}; font-weight: 600; \
                                        padding-bottom: 8px; border-bottom: 1px solid {c.border};",
                                "Coordonnées"
                            }

                            div {
                                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",

                                FormField {
                                    label: "Email professionnel".to_string(),
                                    value: contact_email.read().clone(),
                                    placeholder: "contact@entreprise.fr".to_string(),
                                    field_type: "email".to_string(),
                                    oninput: move |evt: FormEvent| contact_email.set(evt.value()),
                                }

                                FormField {
                                    label: "Téléphone".to_string(),
                                    value: contact_phone.read().clone(),
                                    placeholder: "+33 1 23 45 67 89".to_string(),
                                    field_type: "tel".to_string(),
                                    optional: true,
                                    oninput: move |evt: FormEvent| contact_phone.set(evt.value()),
                                }
                            }

                            FormTextarea {
                                label: "Adresse du siège".to_string(),
                                value: adresse_siege.read().clone(),
                                placeholder: "Adresse complète".to_string(),
                                rows: 2,
                                optional: true,
                                oninput: move |evt: FormEvent| adresse_siege.set(evt.value()),
                            }
                        }

                        // Boutons
                        div {
                            style: "display: flex; gap: 12px;",
                            button {
                                style: "{back_btn}",
                                onclick: move |_| step.set(STEP_DESCRIPTION),
                                "← Retour"
                            }
                            button {
                                style: "{next_btn}",
                                onclick: {
                                    let db_create = db.clone();
                                    move |_| {
                                        // Créer le profil avant d'afficher les félicitations
                                        let new_id = uuid::Uuid::new_v4().to_string();
                                        let profile = jayxpose::data::ExposantProfile {
                                            id: Some(new_id.clone()),
                                            company_name: Some(company_name.read().trim().to_string()),
                                            legal_form: {
                                                let v = legal_form.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            slogan: {
                                                let v = slogan.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            description_short: {
                                                let v = description_short.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            description_long: {
                                                let v = description_long.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            contact_email: {
                                                let v = contact_email.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            contact_phone: {
                                                let v = contact_phone.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            adresse_siege: {
                                                let v = adresse_siege.read().trim().to_string();
                                                if v.is_empty() { None } else { Some(v) }
                                            },
                                            ..Default::default()
                                        };
                                        if db_create.exposant_upsert(&profile).is_ok() {
                                            state.write().exposant_id = Some(new_id);
                                            step.set(STEP_FELICITATIONS);
                                        }
                                    }
                                },
                                "Créer mon profil →"
                            }
                        }
                    },

                    // STEP_FELICITATIONS
                    _ => rsx! {
                        // Récapitulatif
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; \
                                    margin-bottom: 24px; border: 1px solid {c.border};",

                            h3 {
                                style: "font-size: 14px; color: {c.text_white}; font-weight: 600; \
                                        margin-bottom: 16px; padding-bottom: 8px; border-bottom: 1px solid {c.border};",
                                "Récapitulatif"
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 10px;",

                                // Nom entreprise
                                div {
                                    style: "display: flex; gap: 8px;",
                                    span { style: "font-size: 12px; color: {c.text_muted}; min-width: 100px;", "Entreprise" }
                                    span { style: "font-size: 13px; color: {c.text_white}; font-weight: 500;", "{company_name}" }
                                }

                                if !slogan.read().is_empty() {
                                    div {
                                        style: "display: flex; gap: 8px;",
                                        span { style: "font-size: 12px; color: {c.text_muted}; min-width: 100px;", "Slogan" }
                                        span { style: "font-size: 13px; color: {c.text_primary};", "{slogan}" }
                                    }
                                }

                                if !contact_email.read().is_empty() {
                                    div {
                                        style: "display: flex; gap: 8px;",
                                        span { style: "font-size: 12px; color: {c.text_muted}; min-width: 100px;", "Email" }
                                        span { style: "font-size: 13px; color: {c.text_primary};", "{contact_email}" }
                                    }
                                }

                                if !contact_phone.read().is_empty() {
                                    div {
                                        style: "display: flex; gap: 8px;",
                                        span { style: "font-size: 12px; color: {c.text_muted}; min-width: 100px;", "Téléphone" }
                                        span { style: "font-size: 13px; color: {c.text_primary};", "{contact_phone}" }
                                    }
                                }
                            }
                        }

                        // Prochaines étapes
                        div {
                            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; \
                                    margin-bottom: 24px; border: 1px solid {c.border};",

                            h3 {
                                style: "font-size: 14px; color: {c.text_white}; font-weight: 600; \
                                        margin-bottom: 16px; padding-bottom: 8px; border-bottom: 1px solid {c.border};",
                                "Prochaines étapes"
                            }

                            div {
                                style: "display: flex; flex-direction: column; gap: 12px;",

                                div {
                                    style: "display: flex; align-items: center; gap: 12px;",
                                    span { style: "font-size: 20px;", "📦" }
                                    div {
                                        p { style: "font-size: 13px; color: {c.text_white}; font-weight: 500;", "Ajouter des produits" }
                                        p { style: "font-size: 11px; color: {c.text_muted};", "Enrichis ton catalogue avec tes produits et services" }
                                    }
                                }
                                div {
                                    style: "display: flex; align-items: center; gap: 12px;",
                                    span { style: "font-size: 20px;", "🏢" }
                                    div {
                                        p { style: "font-size: 13px; color: {c.text_white}; font-weight: 500;", "Compléter la fiche entreprise" }
                                        p { style: "font-size: 11px; color: {c.text_muted};", "Informations juridiques, contacts, réseaux sociaux" }
                                    }
                                }
                                div {
                                    style: "display: flex; align-items: center; gap: 12px;",
                                    span { style: "font-size: 20px;", "🪟" }
                                    div {
                                        p { style: "font-size: 13px; color: {c.text_white}; font-weight: 500;", "Configurer ta vitrine" }
                                        p { style: "font-size: 11px; color: {c.text_muted};", "Personnalise ta page publique et choisis ce que tu montres" }
                                    }
                                }
                            }
                        }

                        // Bouton final
                        div {
                            style: "display: flex; gap: 12px;",
                            button {
                                style: "{final_btn}",
                                onclick: move |_| {
                                    state.write().section = JayXposeSection::Dashboard;
                                },
                                "✨ Accéder à mon espace JayXpose"
                            }
                        }
                    },
                }}
            }
        }
    }
}
