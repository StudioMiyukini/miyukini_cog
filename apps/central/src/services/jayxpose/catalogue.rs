//! Catalogue produits (XP-E03 + XP-E05) — liste des produits + gestion categories.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{PageHeader, Badge, EmptyState};
use super::{JayXposeSection, JayXposeState};

#[component]
pub fn Catalogue(state: Signal<JayXposeState>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();
    let exposant_id = state.read().exposant_id.clone().unwrap_or_default();
    let mut show_categories = use_signal(|| false);
    let mut search_query = use_signal(|| String::new());
    let mut filter_category = use_signal(|| String::new());
    let new_category_name = use_signal(|| String::new());

    // Charger donnees reelles
    let db = &conns.read().jayxpose;
    let products = db.produits_by_exposant(&exposant_id).unwrap_or_default();
    let categories = db.categories_by_exposant(&exposant_id).unwrap_or_default();

    // Filtrage
    let filtered: Vec<_> = products.iter().filter(|p| {
        let q = search_query.read().to_lowercase();
        let cat_filter = filter_category.read().clone();
        let name_match = q.is_empty() || p.name.as_deref().unwrap_or("").to_lowercase().contains(&q);
        let cat_match = cat_filter.is_empty() || p.category_id.as_deref() == Some(cat_filter.as_str());
        name_match && cat_match
    }).collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            PageHeader {
                title: "📦 Mon catalogue".to_string(),
                count: Some(products.len()),
                action_label: "Ajouter un produit".to_string(),
                action_icon: "➕".to_string(),
                on_action: move |_| {
                    let mut s = state.write();
                    s.editing_product_id = None;
                    s.section = JayXposeSection::NouveauProduit;
                },
            }

            // Barre de filtres
            div {
                style: "display: flex; align-items: center; gap: 12px;",

                input {
                    style: "flex: 1; padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                    r#type: "text",
                    placeholder: "Rechercher un produit...",
                    value: "{search_query}",
                    oninput: move |evt| search_query.set(evt.value()),
                }

                select {
                    style: "padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                    onchange: move |evt| filter_category.set(evt.value()),

                    option { value: "", "Toutes les categories" }
                    for cat in categories.iter() {
                        option {
                            value: "{cat.id.as_deref().unwrap_or(\"\")}",
                            "{cat.name.as_deref().unwrap_or(\"Sans nom\")}"
                        }
                    }
                }

                button {
                    style: "padding: 10px 16px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| show_categories.set(!show_categories()),
                    "🏷️ Categories ({categories.len()})"
                }
            }

            // Panel categories (toggle)
            if *show_categories.read() {
                CategoriesPanel {
                    exposant_id: exposant_id.clone(),
                    categories: categories.clone(),
                    new_category_name: new_category_name,
                }
            }

            // Liste des produits
            if filtered.is_empty() {
                EmptyState {
                    title: "Aucun produit".to_string(),
                    message: "Ajoutez votre premier produit au catalogue.".to_string(),
                    icon: "📦".to_string(),
                    action_label: "Ajouter un produit".to_string(),
                    on_action: move |_| {
                        let mut s = state.write();
                        s.editing_product_id = None;
                        s.section = JayXposeSection::NouveauProduit;
                    },
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    for product in filtered.iter() {
                        ProductRow {
                            key: "{product.id.as_deref().unwrap_or(\"\")}",
                            product: (*product).clone(),
                            categories: categories.clone(),
                            on_edit: move |pid: String| {
                                let mut s = state.write();
                                s.editing_product_id = Some(pid);
                                s.section = JayXposeSection::ModifierProduit;
                            },
                            on_delete: move |pid: String| {
                                let db = &conns.read().jayxpose;
                                let _ = db.produit_delete(&pid);
                            },
                        }
                    }
                }
            }
        }
    }
}

/// Ligne produit dans la liste.
#[component]
fn ProductRow(
    product: jayxpose::data::ProduitCatalogue,
    categories: Vec<jayxpose::data::CategorieProduit>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let pid = product.id.clone().unwrap_or_default();
    let name = product.name.as_deref().unwrap_or("Sans nom");
    let price = product.price.map(|p| format!("{:.2} EUR", p)).unwrap_or_else(|| "Sur demande".to_string());
    let availability = product.availability.as_deref().unwrap_or("disponible");
    let is_featured = product.is_featured.unwrap_or(false);

    let avail_color = match availability {
        "disponible" => c.accent_green,
        "rupture" => c.accent_red,
        "sur_commande" => c.accent_orange,
        _ => c.text_muted,
    };

    let cat_name = product.category_id.as_ref()
        .and_then(|cid| categories.iter().find(|c| c.id.as_deref() == Some(cid.as_str())))
        .and_then(|c| c.name.clone())
        .unwrap_or_default();

    let pid_edit = pid.clone();
    let pid_delete = pid.clone();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px 16px; background: {c.bg_secondary}; border-radius: 4px; transition: background 0.15s;",

            // Icone / image placeholder
            div {
                style: "width: 48px; height: 48px; background: {c.bg_hover}; border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                "📦"
            }

            // Infos
            div {
                style: "flex: 1;",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    p {
                        style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                        "{name}"
                    }
                    if is_featured {
                        span { style: "font-size: 12px;", "⭐" }
                    }
                    Badge { text: availability.to_string(), color: avail_color.to_string() }
                }
                div {
                    style: "display: flex; gap: 16px; font-size: 11px; color: {c.text_muted}; margin-top: 2px;",
                    if !cat_name.is_empty() {
                        span { "🏷️ {cat_name}" }
                    }
                    span { "💰 {price}" }
                }
            }

            // Actions
            div {
                style: "display: flex; gap: 4px;",
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                    onclick: move |_| on_edit.call(pid_edit.clone()),
                    "✏️ Modifier"
                }
                button {
                    style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.accent_red}; cursor: pointer; font-size: 12px;",
                    onclick: move |_| on_delete.call(pid_delete.clone()),
                    "🗑️"
                }
            }
        }
    }
}

/// Panel de gestion des categories (XP-E05).
#[component]
fn CategoriesPanel(
    exposant_id: String,
    categories: Vec<jayxpose::data::CategorieProduit>,
    new_category_name: Signal<String>,
) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; display: flex; flex-direction: column; gap: 12px;",

            h3 {
                style: "font-size: 14px; color: {c.text_white}; font-weight: 600;",
                "🏷️ Gestion des categories"
            }

            // Ajout nouvelle categorie
            div {
                style: "display: flex; gap: 8px;",
                input {
                    style: "flex: 1; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                    placeholder: "Nom de la nouvelle categorie",
                    value: "{new_category_name}",
                    oninput: move |evt| new_category_name.set(evt.value()),
                }
                button {
                    style: "padding: 8px 16px; background: {c.accent_blue}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 13px;",
                    onclick: move |_| {
                        let name = new_category_name.read().trim().to_string();
                        if !name.is_empty() {
                            let cat = jayxpose::data::CategorieProduit {
                                id: Some(uuid::Uuid::new_v4().to_string()),
                                exposant_id: Some(exposant_id.clone()),
                                name: Some(name),
                                ..Default::default()
                            };
                            let db = &conns.read().jayxpose;
                            let _ = db.categorie_insert(&cat);
                            new_category_name.set(String::new());
                        }
                    },
                    "Ajouter"
                }
            }

            // Liste
            if categories.is_empty() {
                p {
                    style: "font-size: 12px; color: {c.text_muted}; text-align: center; padding: 8px;",
                    "Aucune categorie. Ajoutez-en une ci-dessus."
                }
            } else {
                for cat in categories.iter() {
                    div {
                        key: "{cat.id.as_deref().unwrap_or(\"\")}",
                        style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; background: {c.bg_hover}; border-radius: 4px;",

                        span {
                            style: "font-size: 13px; color: {c.text_primary};",
                            "{cat.name.as_deref().unwrap_or(\"Sans nom\")}"
                        }

                        {
                            let cat_id = cat.id.clone().unwrap_or_default();
                            rsx! {
                                button {
                                    style: "padding: 4px 8px; background: transparent; border: none; color: {c.accent_red}; cursor: pointer; font-size: 11px;",
                                    onclick: move |_| {
                                        let db = &conns.read().jayxpose;
                                        let _ = db.categorie_delete(&cat_id);
                                    },
                                    "Supprimer"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
