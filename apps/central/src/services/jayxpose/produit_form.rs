//! Fiche produit — creation / modification (XP-E04).

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{FormField, FormTextarea, FormSection};
use super::{JayXposeSection, JayXposeState};

#[component]
pub fn ProduitForm(state: Signal<JayXposeState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();
    let editing_id = state.read().editing_product_id.clone();
    let editing_id_for_save = editing_id.clone();
    let editing_id_for_delete = editing_id.clone();
    let is_edit = editing_id.is_some();

    // Charger le produit existant si modification
    let existing = if let Some(ref pid) = editing_id {
        let db = &conns.read().jayxpose;
        db.produit_by_id(pid).ok().flatten().unwrap_or_default()
    } else {
        jayxpose::data::ProduitCatalogue::default()
    };

    // Charger les categories
    let categories = {
        let db = &conns.read().jayxpose;
        db.categories_by_exposant(&exposant_id).unwrap_or_default()
    };

    let mut name = use_signal(|| existing.name.clone().unwrap_or_default());
    let mut description = use_signal(|| existing.description.clone().unwrap_or_default());
    let mut price = use_signal(|| existing.price.map(|p| p.to_string()).unwrap_or_default());
    let mut category_id = use_signal(|| existing.category_id.clone().unwrap_or_default());
    let mut availability = use_signal(|| existing.availability.clone().unwrap_or_else(|| "disponible".to_string()));
    let mut is_featured = use_signal(|| existing.is_featured.unwrap_or(false));
    let mut save_message = use_signal(String::new);

    let on_save = move |_| {
        let p_name = name.read().trim().to_string();
        if p_name.is_empty() {
            save_message.set("Le nom du produit est requis.".to_string());
            return;
        }

        let p_price = price.read().parse::<f64>().ok();
        let product = jayxpose::data::ProduitCatalogue {
            id: editing_id_for_save.clone().or_else(|| Some(uuid::Uuid::new_v4().to_string())),
            exposant_id: Some(exposant_id.clone()),
            name: Some(p_name),
            description: Some(description.read().clone()),
            price: p_price,
            category_id: Some(category_id.read().clone()).filter(|s| !s.is_empty()),
            availability: Some(availability.read().clone()),
            is_featured: Some(*is_featured.read()),
            ..Default::default()
        };

        let db = &conns.read().jayxpose;
        let result = if is_edit {
            db.produit_update(&product)
        } else {
            db.produit_insert(&product)
        };

        match result {
            Ok(()) => {
                state.write().section = JayXposeSection::Catalogue;
            }
            Err(e) => {
                save_message.set(format!("Erreur: {e}"));
            }
        }
    };

    let title = if is_edit { "✏️ Modifier le produit" } else { "➕ Nouveau produit" };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px; max-width: 700px;",

            // Header avec retour
            div {
                style: "display: flex; align-items: center; gap: 12px;",

                button {
                    style: "padding: 8px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { state.write().section = JayXposeSection::Catalogue; },
                    "← Retour"
                }
                h2 {
                    style: "font-size: 20px; color: {c.text_white};",
                    "{title}"
                }
            }

            // Message
            if !save_message.read().is_empty() {
                div {
                    style: "padding: 10px 16px; background: {c.accent_red}15; border: 1px solid {c.accent_red}40; border-radius: 4px; font-size: 13px; color: {c.accent_red};",
                    "{save_message}"
                }
            }

            // Formulaire
            FormSection {
                title: "Informations produit".to_string(),

                FormField { label: "Nom du produit".to_string(), value: name.read().clone(), placeholder: "Ex: Savon artisanal lavande".to_string(), oninput: move |evt: FormEvent| name.set(evt.value()) }
                FormTextarea { label: "Description".to_string(), value: description.read().clone(), placeholder: "Description detaillee du produit".to_string(), rows: 4, oninput: move |evt: FormEvent| description.set(evt.value()) }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",

                    FormField { label: "Prix (EUR)".to_string(), value: price.read().clone(), placeholder: "Laisser vide = sur demande".to_string(), field_type: "number".to_string(), optional: true, oninput: move |evt: FormEvent| price.set(evt.value()) }

                    // Categorie
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",

                        label {
                            style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500;",
                            "Categorie"
                        }
                        select {
                            style: "padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                            onchange: move |evt| category_id.set(evt.value()),
                            value: "{category_id}",

                            option { value: "", "Aucune categorie" }
                            for cat in categories.iter() {
                                option {
                                    value: "{cat.id.as_deref().unwrap_or(\"\")}",
                                    selected: category_id.read().as_str() == cat.id.as_deref().unwrap_or(""),
                                    "{cat.name.as_deref().unwrap_or(\"Sans nom\")}"
                                }
                            }
                        }
                    }
                }

                // Disponibilite
                div {
                    style: "display: flex; flex-direction: column; gap: 4px;",

                    label {
                        style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500;",
                        "Disponibilite"
                    }
                    select {
                        style: "padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                        onchange: move |evt| availability.set(evt.value()),
                        value: "{availability}",

                        option { value: "disponible", selected: availability.read().as_str() == "disponible", "Disponible" }
                        option { value: "rupture", selected: availability.read().as_str() == "rupture", "En rupture" }
                        option { value: "sur_commande", selected: availability.read().as_str() == "sur_commande", "Sur commande" }
                    }
                }

                // Vedette
                div {
                    style: "display: flex; align-items: center; gap: 8px;",

                    input {
                        r#type: "checkbox",
                        checked: *is_featured.read(),
                        onchange: move |evt| is_featured.set(evt.checked()),
                    }
                    label {
                        style: "font-size: 13px; color: {c.text_primary};",
                        "⭐ Produit vedette (mis en avant sur la vitrine)"
                    }
                }
            }

            // Actions
            div {
                style: "display: flex; justify-content: space-between; padding-top: 8px;",

                button {
                    style: "padding: 10px 20px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 14px;",
                    onclick: move |_| { state.write().section = JayXposeSection::Catalogue; },
                    "Annuler"
                }

                div {
                    style: "display: flex; gap: 8px;",

                    if is_edit {
                        {
                            let del_id = editing_id_for_delete.clone().unwrap_or_default();
                            rsx! {
                                button {
                                    style: "padding: 10px 20px; background: {c.accent_red}15; border: 1px solid {c.accent_red}40; border-radius: 4px; color: {c.accent_red}; cursor: pointer; font-size: 14px;",
                                    onclick: move |_| {
                                        let db = &conns.read().jayxpose;
                                        let _ = db.produit_delete(&del_id);
                                        state.write().section = JayXposeSection::Catalogue;
                                    },
                                    "🗑️ Supprimer"
                                }
                            }
                        }
                    }

                    button {
                        style: "padding: 10px 24px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; font-weight: 500;",
                        onclick: on_save,
                        "💾 Enregistrer"
                    }
                }
            }
        }
    }
}
