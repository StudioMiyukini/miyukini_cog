//! XP-E08 - Previsualisation vitrine.

use crate::data::{JayXposeDb, PageBuilderBlock, PageBuilderDocument};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use serde_json::Value;
use std::cell::RefCell;
use std::sync::Arc;

/// Ecran previsualisation vitrine.
pub fn exp_e08_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    let tab_labels = ["Accueil", "Catalogue", "Presentation", "Contact"];
    let tab_types = ["accueil", "catalogue", "presentation", "contact"];

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("< Parametres vitrine").clicked() {
                nav_request.replace(Some(ScreenId::ExpVitrineParams));
            }
            ui.heading("Previsualisation vitrine");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Retour builder").clicked() {
                    nav_request.replace(Some(ScreenId::ExpVitrinePresentation));
                }
            });
        });
        ui.add_space(theme.item_spacing());

        ui.horizontal(|ui| {
            for (i, label) in tab_labels.iter().enumerate() {
                if ui
                    .selectable_label(state.vitrine_preview_tab == i, *label)
                    .clicked()
                {
                    state.vitrine_preview_tab = i;
                }
            }
        });
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            let current_type = tab_types
                .get(state.vitrine_preview_tab)
                .copied()
                .unwrap_or("accueil");
            let page = state
                .vitrine_pages
                .iter()
                .find(|p| p.page_type.as_deref() == Some(current_type));

            egui::Frame::new()
                .fill(theme.section_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(16))
                .show(ui, |ui| {
                    if let Some(p) = page {
                        if !p.is_visible.unwrap_or(true) {
                            ui.label("Cette page est masquee.");
                            return;
                        }
                    }

                    if current_type == "presentation" {
                        render_presentation(ui, theme, state);
                    } else if current_type == "catalogue" {
                        render_catalogue(ui, theme, state);
                    } else if current_type == "accueil" {
                        render_accueil(ui, theme, state);
                    } else {
                        let txt = page
                            .and_then(|p| p.content.as_deref())
                            .unwrap_or("Aucun contenu.");
                        ui.label(txt);
                    }
                });

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if ui.button("Publier la vitrine").clicked() {
                    state.fiche_form.vitrine_status = Some("publiee".to_string());
                    match db.exposant_upsert(&state.fiche_form) {
                        Ok(()) => state.status_message = Some("Vitrine publiee avec succes.".to_string()),
                        Err(e) => state.status_message = Some(format!("Erreur publication : {e}")),
                    }
                }
            });

            if let Some(msg) = &state.status_message {
                ui.add_space(8.0);
                ui.colored_label(theme.accent(), msg);
            }
        });
    });
}

fn render_presentation(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &ExpState) {
    let doc = serde_json::from_str::<PageBuilderDocument>(&state.presentation_content)
        .ok()
        .or_else(|| {
            if state.vitrine_pagebuilder_blocks.is_empty() {
                None
            } else {
                Some(PageBuilderDocument {
                    version: "1.1.0".to_string(),
                    page_id: "presentation".to_string(),
                    title: "Presentation".to_string(),
                    slug: "presentation".to_string(),
                    settings: serde_json::json!({}),
                    blocks: state.vitrine_pagebuilder_blocks.clone(),
                })
            }
        });

    let Some(doc) = doc else {
        ui.label("Aucun bloc a previsualiser.");
        return;
    };

    let section_blocks: Vec<&PageBuilderBlock> = doc
        .blocks
        .iter()
        .filter(|b| b.block_type == "section")
        .collect();

    if section_blocks.is_empty() {
        for block in &doc.blocks {
            if block.block_type != "column" {
                render_widget_block(ui, theme, state, block);
                ui.add_space(10.0);
            }
        }
        return;
    }

    for section in section_blocks {
        let section_id = section.id.as_str();
        let sec_pad = prop_i32(&section.props, "padding", 12) as f32;
        egui::Frame::new()
            .fill(color_from_hex(&prop_string(&section.props, "bg_color", "#0F172A")))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(57, 84, 131)))
            .corner_radius(theme.border_radius())
            .inner_margin(egui::Margin::same(sec_pad as i8))
            .show(ui, |ui| {
                let sec_title = prop_string(&section.props, "title", "");
                if !sec_title.is_empty() {
                    ui.label(egui::RichText::new(sec_title).strong().color(theme.accent()));
                    ui.add_space(6.0);
                }

                let columns: Vec<&PageBuilderBlock> = doc
                    .blocks
                    .iter()
                    .filter(|b| b.block_type == "column" && parent_id(&b.props).as_deref() == Some(section_id))
                    .collect();

                if columns.is_empty() {
                    ui.label(egui::RichText::new("Section vide").color(theme.text_secondary()));
                    return;
                }

                ui.columns(columns.len(), |cols| {
                    for (i, col) in columns.iter().enumerate() {
                        let widgets: Vec<&PageBuilderBlock> = doc
                            .blocks
                            .iter()
                            .filter(|b| {
                                b.block_type != "section"
                                    && b.block_type != "column"
                                    && parent_id(&b.props).as_deref() == Some(col.id.as_str())
                            })
                            .collect();

                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(14, 28, 48))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 85, 130)))
                            .corner_radius(theme.border_radius())
                            .inner_margin(egui::Margin::same(8))
                            .show(&mut cols[i], |ui| {
                                if widgets.is_empty() {
                                    ui.label(egui::RichText::new("Colonne vide").size(11.0).color(theme.text_secondary()));
                                } else {
                                    for w in widgets {
                                        render_widget_block(ui, theme, state, w);
                                        ui.add_space(8.0);
                                    }
                                }
                            });
                    }
                });
            });
        ui.add_space(10.0);
    }
}

fn render_widget_block(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &ExpState, block: &PageBuilderBlock) {
    let padding = prop_i32_responsive(&block.props, "padding", 8, state.pagebuilder_device_preview)
        as f32;
    egui::Frame::new()
        .fill(color_from_hex(&prop_string(&block.props, "bg_color", "#111827")))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 73, 104)))
        .corner_radius(theme.border_radius())
        .inner_margin(egui::Margin::same(padding as i8))
        .show(ui, |ui| match block.block_type.as_str() {
            "hero" => {
                ui.label(
                    egui::RichText::new(prop_string(&block.props, "title", "Bienvenue"))
                        .size(
                            prop_i32_responsive(
                                &block.props,
                                "font_size",
                                32,
                                state.pagebuilder_device_preview,
                            ) as f32,
                        )
                        .strong()
                        .color(color_from_hex(&prop_string(&block.props, "color", "#E5E7EB"))),
                );
                ui.label(prop_string(
                    &block.props,
                    "subtitle",
                    "Decouvrez notre univers",
                ));
                ui.add_space(6.0);
                ui.add(egui::Button::new(prop_string(
                    &block.props,
                    "cta_text",
                    "Voir le catalogue",
                )));
            }
            "heading" => {
                ui.label(
                    egui::RichText::new(prop_string(&block.props, "text", "Titre"))
                        .size(
                            prop_i32_responsive(
                                &block.props,
                                "font_size",
                                36,
                                state.pagebuilder_device_preview,
                            ) as f32,
                        )
                        .strong()
                        .color(color_from_hex(&prop_string(&block.props, "color", "#E5E7EB"))),
                );
            }
            "text" => {
                ui.label(
                    egui::RichText::new(prop_string(&block.props, "content", ""))
                        .size(
                            prop_i32_responsive(
                                &block.props,
                                "font_size",
                                16,
                                state.pagebuilder_device_preview,
                            ) as f32,
                        )
                        .color(color_from_hex(&prop_string(&block.props, "color", "#D1D5DB"))),
                );
            }
            "button" => {
                ui.add(egui::Button::new(prop_string(
                    &block.props,
                    "text",
                    "Cliquez ici",
                )));
                ui.label(
                    egui::RichText::new(format!(
                        "Lien: {}",
                        prop_string(&block.props, "link", "#")
                    ))
                    .size(11.0)
                    .color(theme.text_secondary()),
                );
            }
            "image" => {
                ui.label(
                    egui::RichText::new("[Image]")
                        .strong()
                        .color(theme.text_secondary()),
                );
                ui.label(prop_string(&block.props, "url", ""));
                let alt = prop_string(&block.props, "alt", "");
                if !alt.is_empty() {
                    ui.label(format!("alt: {alt}"));
                }
            }
            "product_grid" => {
                ui.label(egui::RichText::new("Produits").strong());
                let limit = prop_i32(&block.props, "limit", 6) as usize;
                for prod in state.produits.iter().take(limit) {
                    let n = prod.name.as_deref().unwrap_or("Produit");
                    let p = prod
                        .price
                        .map(|v| format!("{v:.2} EUR"))
                        .unwrap_or_else(|| "Sur demande".to_string());
                    ui.label(format!("- {n} ({p})"));
                }
            }
            "features" => {
                ui.label(egui::RichText::new("Points forts").strong());
                let items = prop_string(&block.props, "items_text", "");
                for line in items.lines() {
                    if !line.trim().is_empty() {
                        ui.label(format!("- {}", line.trim()));
                    }
                }
            }
            "faq" => {
                ui.label(egui::RichText::new("FAQ").strong());
                let txt = prop_string(&block.props, "qa_text", "");
                for line in txt.lines() {
                    ui.label(line);
                }
            }
            "spacer" => {
                let h = prop_i32(&block.props, "height", 40) as f32;
                ui.add_space(h);
            }
            "divider" => {
                let color = color_from_hex(&prop_string(&block.props, "color", "#374151"));
                let w = prop_i32(&block.props, "width", 1) as f32;
                ui.add(egui::Separator::default().spacing(w));
                // Overlay stroke
                let rect = ui.available_rect_before_wrap();
                if rect.width() > 0.0 {
                    let y = rect.top();
                    ui.painter().line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        egui::Stroke::new(w, color),
                    );
                }
            }
            "video" => {
                let provider = prop_string(&block.props, "provider", "youtube");
                let embed_id = prop_string(&block.props, "embed_id", "");
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(0, 0, 0))
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(24))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!("[Video {provider}]"))
                                .size(18.0)
                                .strong()
                                .color(egui::Color32::WHITE),
                        );
                        if embed_id.is_empty() {
                            ui.label(
                                egui::RichText::new("Aucun ID video configure")
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(160, 160, 160)),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new(format!("ID: {embed_id}"))
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(200, 200, 200)),
                            );
                            ui.label(
                                egui::RichText::new("Apercu en mode publication")
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(120, 120, 120)),
                            );
                        }
                    });
            }
            "gallery" => {
                let urls = prop_string(&block.props, "images_text", "");
                let cols = prop_i32(&block.props, "columns", 3).max(1) as usize;
                let images: Vec<&str> = urls.lines().filter(|l| !l.trim().is_empty()).collect();
                if images.is_empty() {
                    ui.label(
                        egui::RichText::new("[Galerie vide]")
                            .color(theme.text_secondary()),
                    );
                } else {
                    ui.label(egui::RichText::new("Galerie").strong());
                    ui.columns(cols.min(images.len()), |column_uis| {
                        for (i, url) in images.iter().enumerate() {
                            let col = i % column_uis.len();
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(30, 40, 60))
                                .corner_radius(theme.border_radius())
                                .inner_margin(egui::Margin::same(6))
                                .show(&mut column_uis[col], |ui| {
                                    ui.label(
                                        egui::RichText::new("[IMG]")
                                            .strong()
                                            .color(theme.text_secondary()),
                                    );
                                    let short = if url.len() > 30 {
                                        format!("{}...", &url[..30])
                                    } else {
                                        url.to_string()
                                    };
                                    ui.label(
                                        egui::RichText::new(short)
                                            .size(10.0)
                                            .color(theme.text_secondary()),
                                    );
                                });
                        }
                    });
                }
            }
            "icon" => {
                let icon_name = prop_string(&block.props, "icon", "star");
                let label = prop_string(&block.props, "label", "");
                let size = prop_i32(&block.props, "size", 24) as f32;
                let color = color_from_hex(&prop_string(&block.props, "color", "#22D3EE"));
                let icon_char = match icon_name.as_str() {
                    "heart" => "♥",
                    "check" => "✓",
                    "info" => "ℹ",
                    "warning" => "⚠",
                    "mail" => "✉",
                    "phone" => "☎",
                    "map" => "⌖",
                    _ => "★",
                };
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(icon_char).size(size).color(color));
                    if !label.is_empty() {
                        ui.label(
                            egui::RichText::new(label)
                                .size(size * 0.6)
                                .color(color_from_hex(&prop_string(&block.props, "color", "#E5E7EB"))),
                        );
                    }
                });
            }
            "form" => {
                let fields = prop_string(&block.props, "fields_text", "Nom\nEmail\nMessage");
                let submit = prop_string(&block.props, "submit_text", "Envoyer");
                let text_color = color_from_hex(&prop_string(&block.props, "color", "#E5E7EB"));
                ui.label(egui::RichText::new("Formulaire de contact").strong().color(text_color));
                ui.add_space(4.0);
                for line in fields.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let required = trimmed.starts_with('*');
                    let field_label = if required { &trimmed[1..] } else { trimmed };
                    let label_text = if required {
                        format!("{field_label} *")
                    } else {
                        field_label.to_string()
                    };
                    ui.label(egui::RichText::new(label_text).size(12.0).color(text_color));
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgb(31, 41, 55))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(75, 85, 99)))
                        .corner_radius(4.0)
                        .inner_margin(egui::Margin::same(6))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("Saisir {}", field_label.to_lowercase()))
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(107, 114, 128)),
                            );
                        });
                    ui.add_space(4.0);
                }
                ui.add_space(4.0);
                ui.add(egui::Button::new(submit));
            }
            "accordion" => {
                let items_raw = prop_string(&block.props, "items_json", "[]");
                let text_color = color_from_hex(&prop_string(&block.props, "color", "#E5E7EB"));
                if let Ok(items) = serde_json::from_str::<Vec<Value>>(&items_raw) {
                    for item in &items {
                        let title = item.get("title").and_then(Value::as_str).unwrap_or("Item");
                        let content = item.get("content").and_then(Value::as_str).unwrap_or("");
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgb(20, 30, 50))
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(55, 65, 81)))
                            .corner_radius(4.0)
                            .inner_margin(egui::Margin::same(8))
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(format!("▸ {title}")).strong().color(text_color));
                                ui.label(
                                    egui::RichText::new(content)
                                        .size(13.0)
                                        .color(egui::Color32::from_rgb(180, 180, 190)),
                                );
                            });
                        ui.add_space(4.0);
                    }
                } else {
                    ui.label(egui::RichText::new("Accordeon (JSON invalide)").color(theme.text_secondary()));
                }
            }
            "tabs" => {
                let tabs_raw = prop_string(&block.props, "tabs_json", "[]");
                let text_color = color_from_hex(&prop_string(&block.props, "color", "#E5E7EB"));
                if let Ok(tabs) = serde_json::from_str::<Vec<Value>>(&tabs_raw) {
                    if !tabs.is_empty() {
                        ui.horizontal(|ui| {
                            for (i, tab) in tabs.iter().enumerate() {
                                let title = tab.get("title").and_then(Value::as_str).unwrap_or("Onglet");
                                let selected = i == 0; // preview: 1er onglet actif
                                if selected {
                                    ui.label(egui::RichText::new(title).strong().underline().color(theme.accent()));
                                } else {
                                    ui.label(egui::RichText::new(title).color(text_color));
                                }
                            }
                        });
                        // Rendu du 1er onglet
                        if let Some(first) = tabs.first() {
                            let content = first.get("content").and_then(Value::as_str).unwrap_or("");
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(20, 30, 50))
                                .corner_radius(4.0)
                                .inner_margin(egui::Margin::same(8))
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new(content).color(text_color));
                                });
                        }
                    }
                } else {
                    ui.label(egui::RichText::new("Onglets (JSON invalide)").color(theme.text_secondary()));
                }
            }
            "contact_info" => {
                ui.label(egui::RichText::new("Coordonnees").strong().color(
                    color_from_hex(&prop_string(&block.props, "color", "#E5E7EB")),
                ));
                let show_email = prop_bool(&block.props, "show_email", true);
                let show_phone = prop_bool(&block.props, "show_phone", true);
                let show_address = prop_bool(&block.props, "show_address", true);
                // Tente d'utiliser les infos de l'exposant si les champs sont vides
                let email = prop_string(&block.props, "email", "");
                let phone = prop_string(&block.props, "phone", "");
                let address = prop_string(&block.props, "address", "");
                let email_display = if email.is_empty() {
                    state.fiche_form.contact_email.as_deref().unwrap_or("email@exemple.com").to_string()
                } else {
                    email
                };
                let phone_display = if phone.is_empty() {
                    state.fiche_form.contact_phone.as_deref().unwrap_or("+33 1 23 45 67 89").to_string()
                } else {
                    phone
                };
                if show_email {
                    ui.label(format!("✉ {email_display}"));
                }
                if show_phone {
                    ui.label(format!("☎ {phone_display}"));
                }
                if show_address && !address.is_empty() {
                    ui.label(format!("⌖ {address}"));
                }
            }
            "social_links" => {
                ui.label(egui::RichText::new("Reseaux sociaux").strong());
                let networks = [
                    ("facebook", "Facebook"),
                    ("instagram", "Instagram"),
                    ("linkedin", "LinkedIn"),
                    ("tiktok", "TikTok"),
                    ("youtube", "YouTube"),
                    ("x", "X"),
                ];
                ui.horizontal_wrapped(|ui| {
                    for (key, label) in networks {
                        let url = prop_string(&block.props, key, "");
                        // Fallback: essaie les champs exposant
                        let display_url = if url.is_empty() {
                            match key {
                                "facebook" => state.fiche_form.social_facebook.clone().unwrap_or_default(),
                                "instagram" => state.fiche_form.social_instagram.clone().unwrap_or_default(),
                                "linkedin" => state.fiche_form.social_linkedin.clone().unwrap_or_default(),
                                "tiktok" => state.fiche_form.social_tiktok.clone().unwrap_or_default(),
                                "youtube" => state.fiche_form.social_youtube.clone().unwrap_or_default(),
                                "x" => state.fiche_form.social_x.clone().unwrap_or_default(),
                                _ => String::new(),
                            }
                        } else {
                            url
                        };
                        if !display_url.is_empty() {
                            ui.add(egui::Button::new(label).small());
                        }
                    }
                });
            }
            "section" | "column" => {}
            _ => {
                ui.label(format!("Bloc {}", block.block_type));
            }
        });
}

fn render_catalogue(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &ExpState) {
    ui.label(egui::RichText::new("Catalogue public").size(24.0).strong());
    ui.label(
        egui::RichText::new(format!("{} produit(s)", state.produits.len()))
            .color(theme.text_secondary()),
    );
    ui.add_space(8.0);
    for prod in &state.produits {
        let name = prod.name.as_deref().unwrap_or("Produit");
        let price = prod
            .price
            .map(|v| format!("{v:.2} EUR"))
            .unwrap_or_else(|| "Sur demande".to_string());
        ui.label(format!("- {name} | {price}"));
    }
}

fn render_accueil(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &ExpState) {
    let company = state
        .fiche_form
        .company_name
        .as_deref()
        .unwrap_or("Mon entreprise");
    ui.label(
        egui::RichText::new(company)
            .size(32.0)
            .strong()
            .color(theme.accent()),
    );
    if let Some(slogan) = &state.fiche_form.slogan {
        if !slogan.is_empty() {
            ui.label(slogan);
        }
    }
}

fn prop_string(props: &Value, key: &str, default: &str) -> String {
    props
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or(default)
        .to_string()
}

fn prop_i32(props: &Value, key: &str, default: i32) -> i32 {
    props
        .get(key)
        .and_then(Value::as_i64)
        .map(|v| v as i32)
        .unwrap_or(default)
}

fn prop_i32_responsive(props: &Value, key: &str, default: i32, device_preview: usize) -> i32 {
    let device_key = match device_preview {
        1 => format!("{key}_tablet"),
        2 => format!("{key}_mobile"),
        _ => key.to_string(),
    };
    let desktop_val = prop_i32(props, key, default);
    prop_i32(props, &device_key, desktop_val)
}

fn prop_bool(props: &Value, key: &str, default: bool) -> bool {
    props.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn parent_id(props: &Value) -> Option<String> {
    props
        .get("parent_id")
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn color_from_hex(hex: &str) -> egui::Color32 {
    let clean = hex.trim_start_matches('#');
    if clean.len() != 6 {
        return egui::Color32::from_rgb(17, 24, 39);
    }
    let r = u8::from_str_radix(&clean[0..2], 16).ok();
    let g = u8::from_str_radix(&clean[2..4], 16).ok();
    let b = u8::from_str_radix(&clean[4..6], 16).ok();
    match (r, g, b) {
        (Some(r), Some(g), Some(b)) => egui::Color32::from_rgb(r, g, b),
        _ => egui::Color32::from_rgb(17, 24, 39),
    }
}
