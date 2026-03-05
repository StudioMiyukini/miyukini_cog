//! Ecran de selection Purse / Account.

use super::{JayKontaSpace, JayKontaState};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

#[component]
pub fn EntryPointSelector(state: Signal<JayKontaState>) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 32px; padding: 40px;",

            div {
                style: "text-align: center;",
                h1 { style: "font-size: 32px; font-weight: 700; color: {c.text_white}; margin-bottom: 8px;", "JayKonta" }
                p { style: "font-size: 16px; color: {c.text_secondary};", "Comptabilite multi-echelle" }
                p { style: "font-size: 14px; color: {c.text_muted}; margin-top: 16px;", "Choisissez votre espace :" }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 24px; max-width: 700px; width: 100%;",

                button {
                    style: "background: {c.bg_secondary}; border: 2px solid {c.border}; border-radius: 12px; padding: 32px; text-align: left; cursor: pointer; transition: all 0.2s;",
                    onclick: move |_| { state.write().space = JayKontaSpace::Purse; },
                    div { style: "font-size: 40px; margin-bottom: 16px;", "\u{1F4B0}" }
                    h2 { style: "font-size: 20px; font-weight: 600; color: {c.text_white}; margin-bottom: 4px;", "JayBudget" }
                    p { style: "font-size: 14px; color: {c.accent_blue}; margin-bottom: 16px;", "(Purse)" }
                    div {
                        style: "display: flex; flex-direction: column; gap: 6px; margin-bottom: 20px;",
                        FeatureLine { text: "Budget personnel" }
                        FeatureLine { text: "Depenses quotidiennes" }
                        FeatureLine { text: "Objectifs d'epargne" }
                        FeatureLine { text: "Budgets occasionnels" }
                    }
                    div { style: "padding: 10px 20px; background: {c.accent_blue}; color: white; border-radius: 6px; text-align: center; font-size: 14px; font-weight: 500;", "Acceder" }
                }

                button {
                    style: "background: {c.bg_secondary}; border: 2px solid {c.border}; border-radius: 12px; padding: 32px; text-align: left; cursor: pointer; transition: all 0.2s;",
                    onclick: move |_| { state.write().space = JayKontaSpace::Account; },
                    div { style: "font-size: 40px; margin-bottom: 16px;", "\u{1F4CA}" }
                    h2 { style: "font-size: 20px; font-weight: 600; color: {c.text_white}; margin-bottom: 4px;", "JayKonta" }
                    p { style: "font-size: 14px; color: {c.accent_blue}; margin-bottom: 16px;", "(Account)" }
                    div {
                        style: "display: flex; flex-direction: column; gap: 6px; margin-bottom: 20px;",
                        FeatureLine { text: "Comptabilite entreprise" }
                        FeatureLine { text: "Devis et factures" }
                        FeatureLine { text: "Rapports legaux" }
                        FeatureLine { text: "Integrations services" }
                    }
                    div { style: "padding: 10px 20px; background: {c.accent_blue}; color: white; border-radius: 6px; text-align: center; font-size: 14px; font-weight: 500;", "Acceder" }
                }
            }

            p { style: "font-size: 12px; color: {c.text_muted};", "Vous pouvez changer d'espace a tout moment via le menu." }
        }
    }
}

#[component]
fn FeatureLine(text: &'static str) -> Element {
    let c = use_palette();
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; font-size: 13px; color: {c.text_secondary};",
            span { style: "color: {c.accent_green};", "\u{2713}" }
            span { "{text}" }
        }
    }
}
