//! Ecran de selection Purse / Account.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use super::{JayKontaSpace, JayKontaState};

#[component]
pub fn EntryPointSelector(state: Signal<JayKontaState>) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 32px; padding: 40px;",

            // Titre
            div {
                style: "text-align: center;",

                h1 {
                    style: "font-size: 32px; font-weight: 700; color: {p.text_high}; margin-bottom: 8px;",
                    "JayKonta"
                }
                p {
                    style: "font-size: 16px; color: {p.text_secondary};",
                    "Comptabilite multi-echelle"
                }
                p {
                    style: "font-size: 14px; color: {p.text_muted}; margin-top: 16px;",
                    "Choisissez votre espace :"
                }
            }

            // Cartes de selection
            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px; max-width: 700px; width: 100%;",

                // Purse
                button {
                    style: "background: {p.bg_secondary}; border: 2px solid {p.border_default}; border-radius: 12px; padding: 32px; text-align: left; cursor: pointer; transition: all 0.2s;",
                    onclick: move |_| {
                        state.write().space = JayKontaSpace::Purse;
                    },

                    div {
                        style: "font-size: 40px; margin-bottom: 16px;",
                        "💰"
                    }
                    h2 {
                        style: "font-size: 20px; font-weight: 600; color: {p.text_high}; margin-bottom: 4px;",
                        "JayBudget"
                    }
                    p {
                        style: "font-size: 14px; color: {p.accent_primary}; margin-bottom: 16px;",
                        "(Purse)"
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 6px; margin-bottom: 20px;",

                        FeatureLine { text: "Budget personnel" }
                        FeatureLine { text: "Depenses quotidiennes" }
                        FeatureLine { text: "Objectifs d'epargne" }
                        FeatureLine { text: "Budgets occasionnels" }
                    }
                    div {
                        style: "padding: 10px 20px; background: {p.accent_primary}; color: white; border-radius: 6px; text-align: center; font-size: 14px; font-weight: 500;",
                        "Acceder"
                    }
                }

                // Account
                button {
                    style: "background: {p.bg_secondary}; border: 2px solid {p.border_default}; border-radius: 12px; padding: 32px; text-align: left; cursor: pointer; transition: all 0.2s;",
                    onclick: move |_| {
                        state.write().space = JayKontaSpace::Account;
                    },

                    div {
                        style: "font-size: 40px; margin-bottom: 16px;",
                        "📊"
                    }
                    h2 {
                        style: "font-size: 20px; font-weight: 600; color: {p.text_high}; margin-bottom: 4px;",
                        "JayKonta"
                    }
                    p {
                        style: "font-size: 14px; color: {p.accent_primary}; margin-bottom: 16px;",
                        "(Account)"
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 6px; margin-bottom: 20px;",

                        FeatureLine { text: "Comptabilite entreprise" }
                        FeatureLine { text: "Devis et factures" }
                        FeatureLine { text: "Rapports legaux" }
                        FeatureLine { text: "Integrations services" }
                    }
                    div {
                        style: "padding: 10px 20px; background: {p.accent_primary}; color: white; border-radius: 6px; text-align: center; font-size: 14px; font-weight: 500;",
                        "Acceder"
                    }
                }
            }

            // Note
            p {
                style: "font-size: 12px; color: {p.text_muted};",
                "Vous pouvez changer d'espace a tout moment via le menu."
            }
        }
    }
}

#[component]
fn FeatureLine(text: &'static str) -> Element {
    let p = use_palette();
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {p.text_secondary};",
            span { style: "color: {p.success};", "✓" }
            span { "{text}" }
        }
    }
}
