//! Gestion des promotions JayManga — reductions, campagnes promotionnelles.

use dioxus::prelude::*;
use jaymanga::data::Promotion;
use miyukini_service_ui::use_palette;
use crate::use_db;
use super::components::{PageHeader, EmptyState, ActionButton, Badge, StatCard, FormField, FormSection};
use super::JayMangaState;

#[component]
pub fn Promotions(state: Signal<JayMangaState>) -> Element {
    let c = use_palette();
    let db = use_db();

    let promotions = db.promotion_list_active().unwrap_or_default();

    let mut show_form = use_signal(|| false);

    // Formulaire
    let mut promo_name = use_signal(String::new);
    let mut discount_type = use_signal(|| "percent".to_string());
    let mut discount_value = use_signal(|| "10".to_string());
    let mut target_scope = use_signal(|| "catalog".to_string());
    let mut start_date = use_signal(String::new);
    let mut end_date = use_signal(String::new);

    // Stats
    let active_count = promotions.iter()
        .filter(|p| p.active.unwrap_or(false))
        .count();

    let db_create = db.clone();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            PageHeader {
                title: "\u{1F381} Promotions".to_string(),
                subtitle: "Reductions et campagnes promotionnelles".to_string(),
                count: Some(promotions.len()),
                action_label: "Nouvelle promotion".to_string(),
                action_icon: "\u{2795}".to_string(),
                on_action: move |_| {
                    promo_name.set(String::new());
                    discount_type.set("percent".to_string());
                    discount_value.set("10".to_string());
                    target_scope.set("catalog".to_string());
                    start_date.set(String::new());
                    end_date.set(String::new());
                    show_form.set(true);
                },
            }

            // Stats
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                StatCard { label: "Promotions actives".to_string(), value: active_count.to_string(), icon: "\u{1F381}".to_string(), color: c.accent_green.to_string() }
                StatCard { label: "Total promotions".to_string(), value: promotions.len().to_string(), icon: "\u{1F3F7}\u{FE0F}".to_string(), color: c.accent_blue.to_string() }
            }

            // Formulaire
            if *show_form.read() {
                FormSection { title: "Nouvelle promotion".to_string(),
                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr; gap: 16px;",

                        FormField {
                            label: "Nom de la promotion".to_string(),
                            value: promo_name.read().clone(),
                            placeholder: "Promo ete 2026".to_string(),
                            oninput: move |evt: FormEvent| { promo_name.set(evt.value()); },
                        }
                        FormField {
                            label: "Type de reduction".to_string(),
                            value: discount_type.read().clone(),
                            placeholder: "percent / fixed_amount / free".to_string(),
                            oninput: move |evt: FormEvent| { discount_type.set(evt.value()); },
                        }
                    }

                    div {
                        style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 16px;",

                        FormField {
                            label: "Valeur (% ou centimes)".to_string(),
                            value: discount_value.read().clone(),
                            placeholder: "10".to_string(),
                            field_type: "number".to_string(),
                            oninput: move |evt: FormEvent| { discount_value.set(evt.value()); },
                        }
                        FormField {
                            label: "Cible".to_string(),
                            value: target_scope.read().clone(),
                            placeholder: "catalog / work / chapter / series".to_string(),
                            oninput: move |evt: FormEvent| { target_scope.set(evt.value()); },
                        }
                        FormField {
                            label: "Date de fin".to_string(),
                            value: end_date.read().clone(),
                            placeholder: "2026-12-31".to_string(),
                            oninput: move |evt: FormEvent| { end_date.set(evt.value()); },
                        }
                    }

                    div {
                        style: "display: flex; gap: 12px; justify-content: flex-end;",

                        ActionButton {
                            label: "Annuler".to_string(),
                            icon: "\u{2715}".to_string(),
                            onclick: move |_| { show_form.set(false); },
                        }
                        ActionButton {
                            label: "Creer la promotion".to_string(),
                            icon: "\u{1F381}".to_string(),
                            accent: true,
                            onclick: move |_| {
                                let now = chrono::Utc::now().to_rfc3339();

                                let promo = Promotion {
                                    id: Some(uuid::Uuid::new_v4().to_string()),
                                    name: Some(promo_name.read().clone()),
                                    discount_type: Some(discount_type.read().clone()),
                                    discount_value: discount_value.read().parse::<i64>().ok(),
                                    target_scope: Some(target_scope.read().clone()),
                                    target_ids: None,
                                    start_date: Some(now),
                                    end_date: if end_date.read().is_empty() { None } else { Some(end_date.read().clone()) },
                                    active: Some(true),
                                };
                                let _ = db_create.promotion_create(&promo);
                                show_form.set(false);
                            },
                        }
                    }
                }
            }

            // Liste des promotions
            if promotions.is_empty() && !*show_form.read() {
                EmptyState {
                    title: "Aucune promotion".to_string(),
                    message: "Creez des promotions pour attirer de nouveaux lecteurs.".to_string(),
                    icon: "\u{1F381}".to_string(),
                    action_label: "Creer une promotion".to_string(),
                    on_action: move |_| { show_form.set(true); },
                }
            } else {
                // En-tete tableau
                div {
                    style: "display: grid; grid-template-columns: 2fr 1fr 1fr 1fr 1fr; gap: 12px; padding: 8px 16px; font-size: 11px; color: {c.text_muted}; text-transform: uppercase; letter-spacing: 0.5px;",
                    span { "Nom" }
                    span { "Reduction" }
                    span { "Cible" }
                    span { "Statut" }
                    span { "Expire" }
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 6px;",

                    for promo in promotions.iter() {
                        {
                            let name = promo.name.clone().unwrap_or_else(|| "\u{2014}".to_string());
                            let dtype = promo.discount_type.clone().unwrap_or_else(|| "percent".to_string());
                            let dvalue = promo.discount_value.unwrap_or(0);
                            let target = promo.target_scope.clone().unwrap_or_else(|| "catalog".to_string());
                            let is_active = promo.active.unwrap_or(false);
                            let expires = promo.end_date.clone()
                                .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d })
                                .unwrap_or_else(|| "Illimite".to_string());

                            let discount_display = match dtype.as_str() {
                                "percent" => format!("-{}%", dvalue),
                                "fixed_amount" => format!("-{},{:02} \u{20AC}", dvalue / 100, (dvalue % 100).abs()),
                                "free" => "Gratuit".to_string(),
                                _ => format!("{dvalue}"),
                            };

                            let (status_color, status_label) = if is_active {
                                (c.accent_green, "Active")
                            } else {
                                (c.text_muted, "Inactive")
                            };

                            rsx! {
                                div {
                                    style: "display: grid; grid-template-columns: 2fr 1fr 1fr 1fr 1fr; gap: 12px; padding: 12px 16px; background: {c.bg_secondary}; border-radius: 4px; align-items: center;",

                                    span {
                                        style: "font-size: 14px; color: {c.text_white}; font-weight: 600;",
                                        "{name}"
                                    }
                                    span {
                                        style: "font-size: 14px; color: {c.accent_green}; font-weight: 500;",
                                        "{discount_display}"
                                    }
                                    Badge { text: target, color: c.accent_blue.to_string() }
                                    Badge { text: status_label.to_string(), color: status_color.to_string() }
                                    span {
                                        style: "font-size: 12px; color: {c.text_muted};",
                                        "{expires}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
