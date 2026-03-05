//! Configuration boutique JayManga — parametres vendeur et optimisation images.

use super::components::{ActionButton, FormField, FormSection, FormTextarea, PageHeader};
use super::JayMangaState;
use crate::use_db;
use dioxus::prelude::*;
use jaymanga::data::{OptimizationConfig, SellerConfig};
use miyukini_service_ui::use_palette;

#[component]
pub fn Boutique(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let config = db.seller_config_get().ok().flatten().unwrap_or_default();
    let opt_config = db
        .optimization_config_get()
        .unwrap_or_else(|_| jaymanga::data::OptimizationConfig::defaults());

    // Champs vendeur
    let mut shop_name = use_signal(|| config.shop_name.clone().unwrap_or_default());
    let mut shop_description = use_signal(|| config.shop_description.clone().unwrap_or_default());
    let mut default_demo_pages = use_signal(|| config.default_demo_pages.unwrap_or(5).to_string());
    let mut currency = use_signal(|| config.currency.clone().unwrap_or_else(|| "EUR".to_string()));
    let mut reading_direction = use_signal(|| {
        config
            .reading_direction
            .clone()
            .unwrap_or_else(|| "rtl".to_string())
    });
    let mut allow_aggregation = use_signal(|| config.allow_aggregation.unwrap_or(true));

    // Champs optimisation
    let mut quality_hd = use_signal(|| opt_config.quality_hd.unwrap_or(85).to_string());
    let mut quality_sd = use_signal(|| opt_config.quality_sd.unwrap_or(80).to_string());
    let mut quality_mobile = use_signal(|| opt_config.quality_mobile.unwrap_or(75).to_string());
    let mut output_format = use_signal(|| {
        opt_config
            .output_format
            .clone()
            .unwrap_or_else(|| "webp".to_string())
    });
    let mut max_jobs = use_signal(|| opt_config.max_concurrent_jobs.unwrap_or(2).to_string());

    let config_id = config.id.clone();
    let db_save = db.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            PageHeader {
                title: "\u{1F3EA} Boutique".to_string(),
                subtitle: "Configuration de votre boutique manga".to_string(),
            }

            // Parametres boutique
            FormSection { title: "Informations boutique".to_string(),
                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",

                    FormField {
                        label: "Nom de la boutique".to_string(),
                        value: shop_name.read().clone(),
                        placeholder: "Ma Boutique Manga".to_string(),
                        oninput: move |evt: FormEvent| { shop_name.set(evt.value()); },
                    }
                    FormField {
                        label: "Devise".to_string(),
                        value: currency.read().clone(),
                        placeholder: "EUR".to_string(),
                        oninput: move |evt: FormEvent| { currency.set(evt.value()); },
                    }
                }

                FormTextarea {
                    label: "Description".to_string(),
                    value: shop_description.read().clone(),
                    placeholder: "Description de votre boutique...".to_string(),
                    rows: 3,
                    optional: true,
                    oninput: move |evt: FormEvent| { shop_description.set(evt.value()); },
                }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;",

                    FormField {
                        label: "Pages demo (defaut)".to_string(),
                        value: default_demo_pages.read().clone(),
                        field_type: "number".to_string(),
                        oninput: move |evt: FormEvent| { default_demo_pages.set(evt.value()); },
                    }
                    FormField {
                        label: "Direction de lecture".to_string(),
                        value: reading_direction.read().clone(),
                        placeholder: "rtl / ltr".to_string(),
                        oninput: move |evt: FormEvent| { reading_direction.set(evt.value()); },
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 4px;",
                        label {
                            style: "font-size: 12px; color: {c.text_secondary}; font-weight: 500;",
                            "Agregation autorisee"
                        }
                        button {
                            style: "padding: 10px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px; cursor: pointer; text-align: left;",
                            onclick: move |_| {
                                let current = *allow_aggregation.read();
                                allow_aggregation.set(!current);
                            },
                            if *allow_aggregation.read() { "\u{2705} Oui" } else { "\u{274C} Non" }
                        }
                    }
                }
            }

            // Optimisation images
            FormSection { title: "Optimisation des images".to_string(),
                div {
                    style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                    FormField {
                        label: "Qualite HD".to_string(),
                        value: quality_hd.read().clone(),
                        field_type: "number".to_string(),
                        oninput: move |evt: FormEvent| { quality_hd.set(evt.value()); },
                    }
                    FormField {
                        label: "Qualite SD".to_string(),
                        value: quality_sd.read().clone(),
                        field_type: "number".to_string(),
                        oninput: move |evt: FormEvent| { quality_sd.set(evt.value()); },
                    }
                    FormField {
                        label: "Qualite mobile".to_string(),
                        value: quality_mobile.read().clone(),
                        field_type: "number".to_string(),
                        oninput: move |evt: FormEvent| { quality_mobile.set(evt.value()); },
                    }
                    FormField {
                        label: "Jobs paralleles".to_string(),
                        value: max_jobs.read().clone(),
                        field_type: "number".to_string(),
                        oninput: move |evt: FormEvent| { max_jobs.set(evt.value()); },
                    }
                }

                FormField {
                    label: "Format de sortie".to_string(),
                    value: output_format.read().clone(),
                    placeholder: "webp / avif / jpeg".to_string(),
                    oninput: move |evt: FormEvent| { output_format.set(evt.value()); },
                }
            }

            // Bouton sauvegarder
            div {
                style: "display: flex; justify-content: flex-end;",

                ActionButton {
                    label: "Enregistrer".to_string(),
                    icon: "\u{1F4BE}".to_string(),
                    accent: true,
                    onclick: move |_| {
                        let now = chrono::Utc::now().to_rfc3339();

                        // Sauvegarder config vendeur
                        let seller = SellerConfig {
                            id: Some(config_id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string())),
                            shop_name: Some(shop_name.read().clone()),
                            shop_description: if shop_description.read().is_empty() { None } else { Some(shop_description.read().clone()) },
                            default_demo_pages: default_demo_pages.read().parse::<i32>().ok(),
                            currency: Some(currency.read().clone()),
                            reading_direction: Some(reading_direction.read().clone()),
                            allow_aggregation: Some(*allow_aggregation.read()),
                            updated_at: Some(now),
                            ..Default::default()
                        };
                        let _ = db_save.seller_config_upsert(&seller);

                        // Sauvegarder config optimisation
                        let opt = OptimizationConfig {
                            quality_hd: quality_hd.read().parse::<i32>().ok(),
                            quality_sd: quality_sd.read().parse::<i32>().ok(),
                            quality_mobile: quality_mobile.read().parse::<i32>().ok(),
                            output_format: Some(output_format.read().clone()),
                            max_concurrent_jobs: max_jobs.read().parse::<i32>().ok(),
                            ..Default::default()
                        };
                        let _ = db_save.optimization_config_upsert(&opt);
                    },
                }
            }
        }
    }
}
