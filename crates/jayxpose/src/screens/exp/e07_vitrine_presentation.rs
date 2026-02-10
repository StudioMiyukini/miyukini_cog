//! XP-E07 - Page builder vitrine (refonte orientee Elementor).

use crate::data::{JayXposeDb, PageBuilderBlock, PageBuilderDocument, VitrineBlock, VitrinePage};
use crate::governance::{govern_non_sql_create, GovernanceContext};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use serde_json::{json, Map, Value};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

/// (block_type, label, description, category)
const BLOCK_LIBRARY: [(&str, &str, &str, &str); 18] = [
    // --- Mise en page ---
    ("section", "Section", "Container de mise en page", "layout"),
    ("spacer", "Spacer", "Espacement vertical", "layout"),
    ("divider", "Divider", "Separateur horizontal", "layout"),
    // --- Contenu ---
    ("hero", "Hero", "Banniere principale", "content"),
    ("heading", "Heading", "Titre H1/H2/H3", "content"),
    ("text", "Text", "Texte riche", "content"),
    ("image", "Image", "Image + alt", "content"),
    ("video", "Video", "Video integree (YouTube/Vimeo)", "content"),
    ("gallery", "Gallery", "Galerie d'images", "content"),
    ("icon", "Icon", "Icone avec texte", "content"),
    // --- Interactif ---
    ("button", "Button", "CTA avec lien", "interactive"),
    ("form", "Form", "Formulaire de contact", "interactive"),
    ("accordion", "Accordion", "Accordeon / FAQ", "interactive"),
    ("tabs", "Tabs", "Contenu a onglets", "interactive"),
    // --- Catalogue ---
    ("product_grid", "Product Grid", "Grille catalogue", "catalogue"),
    ("features", "Features", "Liste points forts", "catalogue"),
    // --- Contact ---
    ("contact_info", "Contact Info", "Coordonnees entreprise", "contact"),
    ("social_links", "Social Links", "Reseaux sociaux", "contact"),
];

const TEMPLATE_LIBRARY: [(&str, &str, &str); 6] = [
    ("mini-site", "Mini Site", "Hero + Story + CTA"),
    ("landing", "Landing Produit", "Hero + Benefits + CTA"),
    ("storytelling", "Storytelling", "Heading + Texte + Image"),
    ("catalog-focus", "Catalogue Focus", "Hero + Product Grid"),
    ("e-shop", "E-Shop", "Hero + Grille produits + Formulaire"),
    ("service-shop", "Service Shop", "Hero + Services + Contact"),
];

/// Affiche l'ecran XP-E07.
pub fn exp_e07_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    ensure_pagebuilder_globals(state);
    if state.presentation_content.is_empty() {
        hydrate_presentation_state(state, db);
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("< Dashboard").clicked() {
                nav_request.replace(Some(ScreenId::ExpDashboard));
            }
            ui.heading("Page Builder");
            ui.label(
                egui::RichText::new("Canvas + Navigator + Content/Style/Advanced")
                    .size(12.0)
                    .color(theme.text_secondary()),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Previsualiser").clicked() {
                    nav_request.replace(Some(ScreenId::ExpVitrinePreview));
                }
                if ui.button("Enregistrer").clicked() {
                    save_presentation(state, db);
                }
            });
        });

        ui.add_space(8.0);
        toolbar(ui, state, db);
        ui.add_space(8.0);

        ui.columns(3, |cols| {
            library_panel(&mut cols[0], theme, state);
            canvas_panel(&mut cols[1], theme, state, db);
            properties_panel(&mut cols[2], theme, state);
        });
        navigator_window(ctx, theme, state, db);
        site_settings_window(ctx, theme, state);
        template_library_window(ctx, theme, state, db);

        state.presentation_content = serialize_document(&build_document(state));

        if let Some(msg) = &state.status_message {
            ui.add_space(8.0);
            ui.colored_label(theme.accent(), msg);
        }
    });
}

fn toolbar(ui: &mut egui::Ui, state: &mut ExpState, db: &Arc<JayXposeDb>) {
    ui.horizontal(|ui| {
        ui.label("Device:");
        ui.selectable_value(&mut state.pagebuilder_device_preview, 0, "Desktop");
        ui.selectable_value(&mut state.pagebuilder_device_preview, 1, "Tablet");
        ui.selectable_value(&mut state.pagebuilder_device_preview, 2, "Mobile");
        ui.separator();
        if ui.button("Undo").clicked() {
            undo_builder(state);
        }
        if ui.button("Redo").clicked() {
            redo_builder(state);
        }
        ui.separator();
        if ui.button("Ajouter widget").clicked() {
            append_selected_block(state, db);
        }
        if ui.button("Navigator").clicked() {
            state.pagebuilder_navigator_open = !state.pagebuilder_navigator_open;
        }
        if ui.button("Reglages site").clicked() {
            state.pagebuilder_settings_open = !state.pagebuilder_settings_open;
        }
        if ui.button("Templates").clicked() {
            state.pagebuilder_templates_open = !state.pagebuilder_templates_open;
        }
        if ui.button("+ Section 1 col").clicked() {
            add_section_with_columns(state, db, 1);
        }
        if ui.button("+ Section 2 col").clicked() {
            add_section_with_columns(state, db, 2);
        }
        if ui.button("+ Section 3 col").clicked() {
            add_section_with_columns(state, db, 3);
        }
        if ui.button("Template mini-site").clicked() {
            apply_default_template(state, db);
        }
        if ui.button("Dupliquer bloc").clicked() {
            duplicate_selected_block(state, db);
        }
        if ui.button("Supprimer bloc").clicked() {
            remove_selected_block(state, db);
        }
    });
}

fn library_panel(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &mut ExpState) {
    egui::Frame::new()
        .fill(theme.card_background())
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 73, 104)))
        .corner_radius(theme.border_radius())
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Widgets").size(15.0).strong());
            ui.add_space(6.0);
            ui.text_edit_singleline(&mut state.pagebuilder_search);
            ui.add_space(6.0);

            let query = state.pagebuilder_search.to_lowercase();
            let categories = [
                ("layout", "Mise en page"),
                ("content", "Contenu"),
                ("interactive", "Interactif"),
                ("catalogue", "Catalogue"),
                ("contact", "Contact"),
            ];
            for (cat_key, cat_label) in categories {
                let items: Vec<(usize, &str, &str)> = BLOCK_LIBRARY
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, _, _, cat))| *cat == cat_key)
                    .filter(|(_, (_, name, desc, _))| {
                        if query.is_empty() {
                            return true;
                        }
                        let hay = format!("{name} {desc}").to_lowercase();
                        hay.contains(&query)
                    })
                    .map(|(idx, (_, name, desc, _))| (idx, *name, *desc))
                    .collect();
                if items.is_empty() {
                    continue;
                }
                ui.label(
                    egui::RichText::new(cat_label.to_uppercase())
                        .size(11.0)
                        .strong()
                        .color(theme.accent()),
                );
                for (idx, name, desc) in items {
                    let selected = state.pagebuilder_selected_block_idx == idx;
                    if ui
                        .selectable_label(selected, format!("{name} - {desc}"))
                        .clicked()
                    {
                        state.pagebuilder_selected_block_idx = idx;
                    }
                }
                ui.add_space(4.0);
            }
        });
}

fn canvas_panel(
    ui: &mut egui::Ui,
    theme: &JayXposeTheme,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(9, 18, 34))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 67, 105)))
        .corner_radius(theme.border_radius())
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Canvas / Navigator").size(15.0).strong());
            ui.label(
                egui::RichText::new(device_hint(state.pagebuilder_device_preview))
                    .size(11.0)
                    .color(theme.text_secondary()),
            );
            ui.add_space(8.0);

            let mut move_up: Option<usize> = None;
            let mut move_down: Option<usize> = None;
            let mut delete_idx: Option<usize> = None;
            let mut duplicate_widget_idx: Option<usize> = None;
            let mut delete_widget_idx: Option<usize> = None;
            let mut copy_style_idx: Option<usize> = None;
            let mut paste_style_idx: Option<usize> = None;
            let mut move_widget_up_idx: Option<usize> = None;
            let mut move_widget_down_idx: Option<usize> = None;
            let mut copy_widget_idx: Option<usize> = None;
            let mut paste_widget_after_idx: Option<usize> = None;

            // --- Drag-and-drop natif (UX Elementor) ---
            let mut dnd_candidate: Option<(String, usize)> = None; // (target_col_id, insert_pos)
            let dragging_widget_id = state.pagebuilder_dnd_widget_id.clone();
            let pointer_pos = ui.input(|i| i.pointer.interact_pos());
            let is_dragging = dragging_widget_id.is_some();
            let primary_released = ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary));

            let section_indices: Vec<usize> = state
                .vitrine_pagebuilder_blocks
                .iter()
                .enumerate()
                .filter(|(_, b)| b.block_type == "section")
                .map(|(i, _)| i)
                .collect();

            if section_indices.is_empty() {
                ui.label(egui::RichText::new("Aucune section. Utilisez + Section 2 col.").color(theme.text_secondary()));
            }

            for sec_idx in section_indices {
                let sec_id = state.vitrine_pagebuilder_blocks[sec_idx].id.clone();
                let sec_selected = state.pagebuilder_selected_canvas_idx == Some(sec_idx);
                egui::Frame::new()
                    .fill(if sec_selected {
                        egui::Color32::from_rgb(34, 64, 120)
                    } else {
                        egui::Color32::from_rgb(20, 35, 62)
                    })
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(75, 100, 142)))
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_label(sec_selected, format!("Section - {}", sec_id))
                                .clicked()
                            {
                                state.pagebuilder_selected_canvas_idx = Some(sec_idx);
                            }
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("Suppr").clicked() {
                                    delete_idx = Some(sec_idx);
                                }
                                if ui.small_button("v").clicked() {
                                    move_down = Some(sec_idx);
                                }
                                if ui.small_button("^").clicked() {
                                    move_up = Some(sec_idx);
                                }
                            });
                        });

                        let col_indices: Vec<usize> = state
                            .vitrine_pagebuilder_blocks
                            .iter()
                            .enumerate()
                            .filter(|(_, b)| b.block_type == "column" && parent_id(&b.props).as_deref() == Some(sec_id.as_str()))
                            .map(|(i, _)| i)
                            .collect();

                        ui.columns(col_indices.len().max(1), |cols| {
                            for (k, col_idx) in col_indices.iter().enumerate() {
                                let col = &state.vitrine_pagebuilder_blocks[*col_idx];
                                let col_selected = state.pagebuilder_selected_canvas_idx == Some(*col_idx);
                                let inner = egui::Frame::new()
                                    .fill(if col_selected {
                                        egui::Color32::from_rgb(30, 60, 112)
                                    } else {
                                        egui::Color32::from_rgb(14, 26, 48)
                                    })
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(56, 85, 130)))
                                    .corner_radius(theme.border_radius())
                                    .inner_margin(egui::Margin::same(6))
                                    .show(&mut cols[k], |ui| {
                                        if ui
                                            .selectable_label(col_selected, format!("Colonne {}", k + 1))
                                            .clicked()
                                        {
                                            state.pagebuilder_selected_canvas_idx = Some(*col_idx);
                                        }

                                        let widgets: Vec<(usize, String)> = state
                                            .vitrine_pagebuilder_blocks
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, b)| {
                                                b.block_type != "section"
                                                    && b.block_type != "column"
                                                    && parent_id(&b.props).as_deref() == Some(col.id.as_str())
                                            })
                                            .map(|(i, b)| (i, block_label(b)))
                                            .collect();
                                        if widgets.is_empty() {
                                            let drop_text = if is_dragging {
                                                "Relachez pour deposer ici"
                                            } else {
                                                "Glissez-deposez un widget ici"
                                            };
                                            let drop_resp = ui
                                                .add(
                                                    egui::Label::new(
                                                        egui::RichText::new(drop_text)
                                                            .size(11.0)
                                                            .color(theme.text_secondary()),
                                                    )
                                                    .sense(egui::Sense::hover()),
                                                );
                                            if is_dragging {
                                                if let (Some(p), Some(drag_id)) = (pointer_pos, dragging_widget_id.clone()) {
                                                    if drop_resp.rect.contains(p) {
                                                        dnd_candidate = Some((col.id.clone(), 0));
                                                        ui.painter().rect_stroke(
                                                            drop_resp.rect.expand(4.0),
                                                            theme.border_radius(),
                                                            egui::Stroke::new(2.0, theme.accent()),
                                                            egui::StrokeKind::Inside,
                                                        );
                                                        ui.ctx().output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                                                        // Keep drag_id "used" to avoid unused warnings if the branch changes later
                                                        let _ = drag_id;
                                                    }
                                                }
                                            }
                                        } else {
                                            for (pos, (wi, label)) in widgets.into_iter().enumerate() {
                                                let wsel = state.pagebuilder_selected_canvas_idx == Some(wi);
                                                let response = ui.add(
                                                    egui::Button::new(label.clone())
                                                        .selected(wsel)
                                                        .sense(egui::Sense::click_and_drag()),
                                                );
                                                if response.clicked() {
                                                    state.pagebuilder_selected_canvas_idx = Some(wi);
                                                }
                                                if response.drag_started() {
                                                    // Démarre un drag natif (souris) sur ce widget.
                                                    state.pagebuilder_dnd_widget_id =
                                                        Some(state.vitrine_pagebuilder_blocks[wi].id.clone());
                                                    state.pagebuilder_dnd_widget_label =
                                                        Some(label.clone());
                                                    state.pagebuilder_dnd_target_col_id = None;
                                                    state.pagebuilder_dnd_target_pos = None;
                                                }

                                                // Détection des cibles de drop + rendu indicateur d'insertion.
                                                if let (Some(p), Some(_drag_id)) = (pointer_pos, dragging_widget_id.clone()) {
                                                    if response.rect.contains(p) {
                                                        let before = p.y < response.rect.center().y;
                                                        let insert_pos = if before { pos } else { pos + 1 };
                                                        dnd_candidate = Some((col.id.clone(), insert_pos));
                                                        let y = if before { response.rect.top() } else { response.rect.bottom() };
                                                        ui.painter().line_segment(
                                                            [
                                                                egui::pos2(response.rect.left(), y),
                                                                egui::pos2(response.rect.right(), y),
                                                            ],
                                                            egui::Stroke::new(2.0, theme.accent()),
                                                        );
                                                        ui.ctx().output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                                                    }
                                                }
                                                if let Some(block) = state.vitrine_pagebuilder_blocks.get(wi) {
                                                    ui.label(
                                                        egui::RichText::new(widget_preview_line(block))
                                                            .size(11.0)
                                                            .color(theme.text_secondary()),
                                                    );
                                                }
                                                response.context_menu(|ui| {
                                                    if ui.button("Dupliquer").clicked() {
                                                        duplicate_widget_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Supprimer").clicked() {
                                                        delete_widget_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Copier style").clicked() {
                                                        copy_style_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Coller style").clicked() {
                                                        paste_style_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Monter").clicked() {
                                                        move_widget_up_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Descendre").clicked() {
                                                        move_widget_down_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Copier widget").clicked() {
                                                        copy_widget_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                    if ui.button("Coller widget apres").clicked() {
                                                        paste_widget_after_idx = Some(wi);
                                                        ui.close();
                                                    }
                                                });
                                            }
                                        }
                                    });

                                // Drop "append" si on survole la colonne (mais pas un widget précis)
                                if is_dragging {
                                    if let (Some(p), Some(_drag_id)) = (pointer_pos, dragging_widget_id.clone()) {
                                        let col_rect = inner.response.rect;
                                        if col_rect.contains(p) && dnd_candidate.is_none() {
                                            // Append par défaut : à la fin des widgets de la colonne.
                                            let len_widgets = state
                                                .vitrine_pagebuilder_blocks
                                                .iter()
                                                .filter(|b| {
                                                    b.block_type != "section"
                                                        && b.block_type != "column"
                                                        && parent_id(&b.props).as_deref() == Some(col.id.as_str())
                                                })
                                                .count();
                                            dnd_candidate = Some((col.id.clone(), len_widgets));
                                            cols[k].painter().rect_stroke(
                                                col_rect.expand(2.0),
                                                theme.border_radius(),
                                                egui::Stroke::new(2.0, theme.accent()),
                                                            egui::StrokeKind::Inside,
                                            );
                                            cols[k].ctx().output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                                        }
                                    }
                                }
                            }
                        });
                    });
                ui.add_space(6.0);
            }

            // Rendu "ghost" sous la souris pendant le drag.
            if is_dragging {
                if let (Some(p), Some(lbl)) = (pointer_pos, state.pagebuilder_dnd_widget_label.clone()) {
                    egui::Area::new(egui::Id::new("pagebuilder_dnd_ghost"))
                        .order(egui::Order::Foreground)
                        .fixed_pos(p + egui::vec2(14.0, 14.0))
                        .show(ui.ctx(), |ui| {
                            egui::Frame::new()
                                .fill(egui::Color32::from_rgb(17, 28, 49))
                                .stroke(egui::Stroke::new(1.0, theme.accent()))
                                .corner_radius(theme.border_radius())
                                .inner_margin(egui::Margin::same(8))
                                .show(ui, |ui| {
                                    ui.label(egui::RichText::new(lbl).color(theme.text_secondary()));
                                });
                        });
                }
            }

            // Applique un drop si le bouton souris est relâché pendant un drag.
            if primary_released {
                if let (Some(widget_id), Some((target_col_id, insert_pos))) =
                    (state.pagebuilder_dnd_widget_id.clone(), dnd_candidate.clone())
                {
                    if let Some(src_col_id) = state
                        .vitrine_pagebuilder_blocks
                        .iter()
                        .find(|b| b.id == widget_id)
                        .and_then(|b| parent_id(&b.props))
                    {
                        // Ajuste la position si on reorder dans la même colonne (suppression puis insertion).
                        let mut adjusted_pos = insert_pos;
                        if src_col_id == target_col_id {
                            let siblings: Vec<String> = state
                                .vitrine_pagebuilder_blocks
                                .iter()
                                .filter(|b| {
                                    b.block_type != "section"
                                        && b.block_type != "column"
                                        && parent_id(&b.props).as_deref() == Some(src_col_id.as_str())
                                })
                                .map(|b| b.id.clone())
                                .collect();
                            if let Some(orig_pos) = siblings.iter().position(|id| *id == widget_id) {
                                // No-op si on retombe "sur soi".
                                if adjusted_pos == orig_pos || adjusted_pos == orig_pos + 1 {
                                    state.pagebuilder_dnd_widget_id = None;
                                    state.pagebuilder_dnd_widget_label = None;
                                    state.pagebuilder_dnd_target_col_id = None;
                                    state.pagebuilder_dnd_target_pos = None;
                                } else {
                                    if orig_pos < adjusted_pos && adjusted_pos > 0 {
                                        adjusted_pos -= 1;
                                    }
                                    if govern_builder_non_sql_action(
                                        state,
                                        db,
                                        "move_widget",
                                        json!({
                                            "widget_id": widget_id,
                                            "source_col": src_col_id,
                                            "target_col": target_col_id,
                                            "insert_pos": adjusted_pos
                                        }),
                                    )
                                    .is_ok()
                                    {
                                        record_snapshot(state);
                                        if let Some(new_idx) = move_widget_to_column_at_pos(
                                            &mut state.vitrine_pagebuilder_blocks,
                                            &widget_id,
                                            &target_col_id,
                                            adjusted_pos,
                                        ) {
                                            state.pagebuilder_selected_canvas_idx = Some(new_idx);
                                            state.status_message =
                                                Some("Widget deplace (drag-and-drop).".to_string());
                                        }
                                    } else {
                                        state.status_message =
                                            Some("Gouvernance: deplacement refuse.".to_string());
                                    }
                                    state.pagebuilder_dnd_widget_id = None;
                                    state.pagebuilder_dnd_widget_label = None;
                                    state.pagebuilder_dnd_target_col_id = None;
                                    state.pagebuilder_dnd_target_pos = None;
                                }
                            }
                        } else {
                            if govern_builder_non_sql_action(
                                state,
                                db,
                                "move_widget",
                                json!({
                                    "widget_id": widget_id,
                                    "source_col": src_col_id,
                                    "target_col": target_col_id,
                                    "insert_pos": adjusted_pos
                                }),
                            )
                            .is_ok()
                            {
                                record_snapshot(state);
                                if let Some(new_idx) = move_widget_to_column_at_pos(
                                    &mut state.vitrine_pagebuilder_blocks,
                                    &widget_id,
                                    &target_col_id,
                                    adjusted_pos,
                                ) {
                                    state.pagebuilder_selected_canvas_idx = Some(new_idx);
                                    state.status_message =
                                        Some("Widget deplace (drag-and-drop).".to_string());
                                }
                            } else {
                                state.status_message =
                                    Some("Gouvernance: deplacement refuse.".to_string());
                            }
                            state.pagebuilder_dnd_widget_id = None;
                            state.pagebuilder_dnd_widget_label = None;
                            state.pagebuilder_dnd_target_col_id = None;
                            state.pagebuilder_dnd_target_pos = None;
                        }
                    } else {
                        // Widget introuvable : nettoie l'état.
                        state.pagebuilder_dnd_widget_id = None;
                        state.pagebuilder_dnd_widget_label = None;
                        state.pagebuilder_dnd_target_col_id = None;
                        state.pagebuilder_dnd_target_pos = None;
                    }
                } else if is_dragging {
                    // Release sans cible valide -> annulation.
                    state.pagebuilder_dnd_widget_id = None;
                    state.pagebuilder_dnd_widget_label = None;
                    state.pagebuilder_dnd_target_col_id = None;
                    state.pagebuilder_dnd_target_pos = None;
                }
            } else {
                // Stocke la cible courante (debug/état) pendant le drag.
                if let Some((col_id, pos)) = dnd_candidate.clone() {
                    state.pagebuilder_dnd_target_col_id = Some(col_id);
                    state.pagebuilder_dnd_target_pos = Some(pos);
                } else if is_dragging {
                    state.pagebuilder_dnd_target_col_id = None;
                    state.pagebuilder_dnd_target_pos = None;
                }
            }

            if let Some(i) = move_up {
                if i > 0 {
                    record_snapshot(state);
                    state.vitrine_pagebuilder_blocks.swap(i, i - 1);
                    state.pagebuilder_selected_canvas_idx = Some(i - 1);
                }
            }
            if let Some(i) = move_down {
                if i + 1 < state.vitrine_pagebuilder_blocks.len() {
                    record_snapshot(state);
                    state.vitrine_pagebuilder_blocks.swap(i, i + 1);
                    state.pagebuilder_selected_canvas_idx = Some(i + 1);
                }
            }
            if let Some(i) = delete_idx {
                if govern_builder_non_sql_action(state, db, "remove_widget_tree", json!({})).is_ok() {
                    record_snapshot(state);
                    let deleted_id = state.vitrine_pagebuilder_blocks[i].id.clone();
                    remove_block_subtree(&mut state.vitrine_pagebuilder_blocks, &deleted_id);
                    state.pagebuilder_selected_canvas_idx = None;
                } else {
                    state.status_message = Some("Gouvernance: suppression refusee.".to_string());
                }
            }
            if let Some(wi) = duplicate_widget_idx {
                if wi < state.vitrine_pagebuilder_blocks.len() {
                    if govern_builder_non_sql_action(state, db, "duplicate_widget", json!({})).is_ok() {
                        record_snapshot(state);
                        let mut cloned = state.vitrine_pagebuilder_blocks[wi].clone();
                        cloned.id = format!("blk-{}", uuid::Uuid::new_v4().simple());
                        state.vitrine_pagebuilder_blocks.insert(wi + 1, cloned);
                        state.pagebuilder_selected_canvas_idx = Some(wi + 1);
                    } else {
                        state.status_message = Some("Gouvernance: duplication refusee.".to_string());
                    }
                }
            }
            if let Some(wi) = delete_widget_idx {
                if wi < state.vitrine_pagebuilder_blocks.len() {
                    if govern_builder_non_sql_action(state, db, "remove_widget_tree", json!({})).is_ok() {
                        record_snapshot(state);
                        let deleted_id = state.vitrine_pagebuilder_blocks[wi].id.clone();
                        remove_block_subtree(&mut state.vitrine_pagebuilder_blocks, &deleted_id);
                        state.pagebuilder_selected_canvas_idx = None;
                    } else {
                        state.status_message = Some("Gouvernance: suppression refusee.".to_string());
                    }
                }
            }
            if let Some(wi) = copy_style_idx {
                if wi < state.vitrine_pagebuilder_blocks.len() {
                    state.pagebuilder_style_clipboard = Some(extract_style_json(
                        &state.vitrine_pagebuilder_blocks[wi].props,
                    ));
                    state.status_message = Some("Style copie.".to_string());
                }
            }
            if let Some(wi) = paste_style_idx {
                if wi < state.vitrine_pagebuilder_blocks.len() {
                    if let Some(style) = state.pagebuilder_style_clipboard.clone() {
                        record_snapshot(state);
                        if let Some(target) = state.vitrine_pagebuilder_blocks.get_mut(wi) {
                            apply_style_json(&mut target.props, &style);
                        }
                        state.status_message = Some("Style colle.".to_string());
                    }
                }
            }
            if let Some(wi) = move_widget_up_idx {
                if govern_builder_non_sql_action(state, db, "reorder_widget", json!({"direction":"up"}))
                    .is_ok()
                {
                    record_snapshot(state);
                    if move_widget_in_column(&mut state.vitrine_pagebuilder_blocks, wi, -1) {
                        state.status_message = Some("Widget remonte.".to_string());
                    }
                }
            }
            if let Some(wi) = move_widget_down_idx {
                if govern_builder_non_sql_action(state, db, "reorder_widget", json!({"direction":"down"}))
                    .is_ok()
                {
                    record_snapshot(state);
                    if move_widget_in_column(&mut state.vitrine_pagebuilder_blocks, wi, 1) {
                        state.status_message = Some("Widget descend.".to_string());
                    }
                }
            }
            if let Some(wi) = copy_widget_idx {
                if wi < state.vitrine_pagebuilder_blocks.len() {
                    state.pagebuilder_block_clipboard =
                        Some(state.vitrine_pagebuilder_blocks[wi].clone());
                    state.status_message = Some("Widget copie.".to_string());
                }
            }
            if let Some(wi) = paste_widget_after_idx {
                if wi < state.vitrine_pagebuilder_blocks.len() {
                    if let Some(mut cloned) = state.pagebuilder_block_clipboard.clone() {
                        if govern_builder_non_sql_action(state, db, "paste_widget", json!({}))
                            .is_ok()
                        {
                            record_snapshot(state);
                            cloned.id = format!("blk-{}", uuid::Uuid::new_v4().simple());
                            let parent = parent_id(&state.vitrine_pagebuilder_blocks[wi].props);
                            if let Some(pid) = parent {
                                set_prop_string(&mut cloned.props, "parent_id", pid);
                            }
                            state.vitrine_pagebuilder_blocks.insert(wi + 1, cloned);
                            state.pagebuilder_selected_canvas_idx = Some(wi + 1);
                        }
                    }
                }
            }

            ui.separator();
            ui.label(egui::RichText::new("JSON document (debug)").size(12.0));
            ui.add(
                egui::TextEdit::multiline(&mut state.presentation_content)
                    .desired_width(ui.available_width())
                    .desired_rows(10),
            );
        });
}

fn properties_panel(ui: &mut egui::Ui, theme: &JayXposeTheme, state: &mut ExpState) {
    egui::Frame::new()
        .fill(theme.card_background())
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(52, 73, 104)))
        .corner_radius(theme.border_radius())
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Proprietes").size(15.0).strong());
            ui.horizontal(|ui| {
                ui.selectable_value(&mut state.pagebuilder_active_tab, 0, "Content");
                ui.selectable_value(&mut state.pagebuilder_active_tab, 1, "Style");
                ui.selectable_value(&mut state.pagebuilder_active_tab, 2, "Advanced");
            });
            ui.separator();

            let Some(idx) = state.pagebuilder_selected_canvas_idx else {
                ui.label("Selectionnez un bloc dans le canvas.");
                return;
            };
            if idx >= state.vitrine_pagebuilder_blocks.len() {
                ui.label("Bloc introuvable.");
                return;
            }

            let block_type = state.vitrine_pagebuilder_blocks[idx].block_type.clone();
            ui.label(
                egui::RichText::new(format!("Bloc: {}", block_type))
                    .strong()
                    .color(theme.accent()),
            );
            ui.horizontal(|ui| {
                if ui.small_button("Copier style").clicked() {
                    let props = state.vitrine_pagebuilder_blocks[idx].props.clone();
                    state.pagebuilder_style_clipboard = Some(extract_style_json(&props));
                }
                let can_paste = state.pagebuilder_style_clipboard.is_some();
                if ui
                    .add_enabled(can_paste, egui::Button::new("Coller style"))
                    .clicked()
                {
                    if let Some(s) = state.pagebuilder_style_clipboard.clone() {
                        record_snapshot(state);
                        if let Some(target) = state.vitrine_pagebuilder_blocks.get_mut(idx) {
                            apply_style_json(&mut target.props, &s);
                        }
                    }
                }
            });
            ui.add_space(6.0);

            let block = &mut state.vitrine_pagebuilder_blocks[idx];
            match state.pagebuilder_active_tab {
                0 => edit_content_tab(ui, block),
                1 => edit_style_tab(ui, block, state.pagebuilder_device_preview),
                _ => edit_advanced_tab(ui, block),
            }
        });
}

fn edit_content_tab(ui: &mut egui::Ui, block: &mut PageBuilderBlock) {
    match block.block_type.as_str() {
        "section" => {
            edit_text_prop(ui, &mut block.props, "title", "Nom section", "Section");
        }
        "column" => {
            edit_i32_prop(ui, &mut block.props, "width_pct", "Largeur (%)", 50, 10, 100);
        }
        "hero" => {
            edit_text_prop(ui, &mut block.props, "title", "Titre", "Bienvenue");
            edit_text_prop(
                ui,
                &mut block.props,
                "subtitle",
                "Sous-titre",
                "Decouvrez notre univers",
            );
            edit_text_prop(ui, &mut block.props, "cta_text", "Texte CTA", "Voir le catalogue");
            edit_text_prop(ui, &mut block.props, "cta_link", "Lien CTA", "/catalogue");
        }
        "heading" => {
            edit_text_prop(ui, &mut block.props, "text", "Texte", "Titre principal");
            edit_text_prop(ui, &mut block.props, "tag", "Tag", "h2");
        }
        "text" => {
            edit_multiline_prop(ui, &mut block.props, "content", "Contenu", "Votre texte...");
        }
        "button" => {
            edit_text_prop(ui, &mut block.props, "text", "Texte", "Cliquez ici");
            edit_text_prop(ui, &mut block.props, "link", "Lien", "#");
            edit_text_prop(ui, &mut block.props, "size", "Taille", "md");
        }
        "image" => {
            edit_text_prop(ui, &mut block.props, "url", "URL image", "https://");
            edit_text_prop(ui, &mut block.props, "alt", "Alt", "Image description");
        }
        "product_grid" => {
            edit_i32_prop(ui, &mut block.props, "limit", "Nb produits", 6, 1, 24);
            edit_i32_prop(ui, &mut block.props, "columns", "Colonnes", 3, 1, 4);
            edit_bool_prop(ui, &mut block.props, "show_price", "Afficher prix", true);
            edit_bool_prop(ui, &mut block.props, "show_button", "Afficher bouton", true);
        }
        "features" => {
            edit_multiline_prop(
                ui,
                &mut block.props,
                "items_text",
                "Items (1 ligne/item)",
                "Livraison locale\nFait main\nService premium",
            );
        }
        "faq" => {
            edit_multiline_prop(
                ui,
                &mut block.props,
                "qa_text",
                "FAQ (Q:... / R:...)",
                "Q: Delai ?\nR: 48h",
            );
        }
        "spacer" => {
            edit_i32_prop(ui, &mut block.props, "height", "Hauteur (px)", 40, 4, 200);
        }
        "divider" => {
            edit_text_prop(ui, &mut block.props, "style", "Style (solid/dashed/dotted)", "solid");
            edit_text_prop(ui, &mut block.props, "color", "Couleur", "#374151");
            edit_i32_prop(ui, &mut block.props, "width", "Epaisseur (px)", 1, 1, 10);
        }
        "video" => {
            edit_text_prop(ui, &mut block.props, "provider", "Fournisseur (youtube/vimeo)", "youtube");
            edit_text_prop(ui, &mut block.props, "embed_id", "ID video (ex: dQw4w9WgXcQ)", "");
            edit_bool_prop(ui, &mut block.props, "autoplay", "Lecture auto", false);
        }
        "gallery" => {
            edit_multiline_prop(
                ui,
                &mut block.props,
                "images_text",
                "URLs images (1 par ligne)",
                "https://exemple.com/img1.jpg\nhttps://exemple.com/img2.jpg",
            );
            edit_i32_prop(ui, &mut block.props, "columns", "Colonnes", 3, 1, 6);
            edit_i32_prop(ui, &mut block.props, "gap", "Espacement (px)", 8, 0, 32);
        }
        "icon" => {
            edit_text_prop(ui, &mut block.props, "icon", "Icone (star/heart/check/info/warning/mail/phone/map)", "star");
            edit_text_prop(ui, &mut block.props, "label", "Texte associe", "");
            edit_i32_prop(ui, &mut block.props, "size", "Taille (px)", 24, 12, 96);
            edit_text_prop(ui, &mut block.props, "color", "Couleur", "#22D3EE");
        }
        "form" => {
            edit_multiline_prop(
                ui,
                &mut block.props,
                "fields_text",
                "Champs (1 par ligne, prefixe *=obligatoire)",
                "*Nom\n*Email\nTelephone\n*Message",
            );
            edit_text_prop(ui, &mut block.props, "submit_text", "Texte bouton", "Envoyer");
            edit_text_prop(ui, &mut block.props, "redirect", "URL redirection apres envoi", "");
        }
        "accordion" => {
            ui.label("Items (JSON array [{title, content}]):");
            let mut val = prop_string(&block.props, "items_json", "[]");
            if ui
                .add(egui::TextEdit::multiline(&mut val).desired_rows(6))
                .changed()
            {
                set_prop_string(&mut block.props, "items_json", val);
            }
        }
        "tabs" => {
            ui.label("Onglets (JSON array [{title, content}]):");
            let mut val = prop_string(&block.props, "tabs_json", "[]");
            if ui
                .add(egui::TextEdit::multiline(&mut val).desired_rows(6))
                .changed()
            {
                set_prop_string(&mut block.props, "tabs_json", val);
            }
        }
        "contact_info" => {
            edit_text_prop(ui, &mut block.props, "email", "Email", "");
            edit_text_prop(ui, &mut block.props, "phone", "Telephone", "");
            edit_multiline_prop(ui, &mut block.props, "address", "Adresse", "");
            edit_bool_prop(ui, &mut block.props, "show_email", "Afficher email", true);
            edit_bool_prop(ui, &mut block.props, "show_phone", "Afficher telephone", true);
            edit_bool_prop(ui, &mut block.props, "show_address", "Afficher adresse", true);
        }
        "social_links" => {
            edit_text_prop(ui, &mut block.props, "facebook", "Facebook URL", "");
            edit_text_prop(ui, &mut block.props, "instagram", "Instagram URL", "");
            edit_text_prop(ui, &mut block.props, "linkedin", "LinkedIn URL", "");
            edit_text_prop(ui, &mut block.props, "tiktok", "TikTok URL", "");
            edit_text_prop(ui, &mut block.props, "youtube", "YouTube URL", "");
            edit_text_prop(ui, &mut block.props, "x", "X (Twitter) URL", "");
            edit_text_prop(ui, &mut block.props, "style", "Style (icon/text/both)", "icon");
        }
        _ => {
            edit_multiline_prop(ui, &mut block.props, "content", "Contenu", "...");
        }
    }
}

fn edit_style_tab(ui: &mut egui::Ui, block: &mut PageBuilderBlock, device_preview: usize) {
    ui.label(
        egui::RichText::new(format!("Device cible: {}", device_hint(device_preview))).size(11.0),
    );
    edit_text_prop(ui, &mut block.props, "align", "Align", "left");
    edit_text_prop(ui, &mut block.props, "color", "Couleur texte", "#E5E7EB");
    edit_text_prop(ui, &mut block.props, "bg_color", "Couleur fond", "#1F2937");
    edit_responsive_i32_prop(
        ui,
        &mut block.props,
        "font_size",
        "Font size",
        16,
        10,
        72,
        device_preview,
    );
    edit_responsive_i32_prop(
        ui,
        &mut block.props,
        "padding",
        "Padding",
        12,
        0,
        64,
        device_preview,
    );
}

fn edit_advanced_tab(ui: &mut egui::Ui, block: &mut PageBuilderBlock) {
    edit_text_prop(ui, &mut block.props, "css_class", "CSS Class", "");
    edit_text_prop(ui, &mut block.props, "anchor", "Anchor", "");
    edit_i32_prop(ui, &mut block.props, "margin_top", "Margin top", 0, 0, 128);
    edit_i32_prop(ui, &mut block.props, "margin_bottom", "Margin bottom", 0, 0, 128);
}

fn edit_text_prop(ui: &mut egui::Ui, props: &mut Value, key: &str, label: &str, default: &str) {
    ui.label(label);
    let mut val = prop_string(props, key, default);
    if ui.text_edit_singleline(&mut val).changed() {
        set_prop_string(props, key, val);
    }
}

fn edit_multiline_prop(
    ui: &mut egui::Ui,
    props: &mut Value,
    key: &str,
    label: &str,
    default: &str,
) {
    ui.label(label);
    let mut val = prop_string(props, key, default);
    if ui
        .add(egui::TextEdit::multiline(&mut val).desired_rows(4))
        .changed()
    {
        set_prop_string(props, key, val);
    }
}

fn edit_i32_prop(
    ui: &mut egui::Ui,
    props: &mut Value,
    key: &str,
    label: &str,
    default: i32,
    min: i32,
    max: i32,
) {
    let mut val = prop_i32(props, key, default);
    ui.horizontal(|ui| {
        ui.label(label);
        if ui.add(egui::Slider::new(&mut val, min..=max)).changed() {
            set_prop_i32(props, key, val);
        }
    });
}

fn edit_bool_prop(ui: &mut egui::Ui, props: &mut Value, key: &str, label: &str, default: bool) {
    let mut val = prop_bool(props, key, default);
    if ui.checkbox(&mut val, label).changed() {
        set_prop_bool(props, key, val);
    }
}

fn responsive_key(base: &str, device_preview: usize) -> String {
    match device_preview {
        1 => format!("{base}_tablet"),
        2 => format!("{base}_mobile"),
        _ => base.to_string(),
    }
}

fn edit_responsive_i32_prop(
    ui: &mut egui::Ui,
    props: &mut Value,
    base_key: &str,
    label: &str,
    default: i32,
    min: i32,
    max: i32,
    device_preview: usize,
) {
    let key = responsive_key(base_key, device_preview);
    let fallback = prop_i32(props, base_key, default);
    let mut val = prop_i32(props, &key, fallback);
    ui.horizontal(|ui| {
        ui.label(label);
        if ui.add(egui::Slider::new(&mut val, min..=max)).changed() {
            set_prop_i32(props, &key, val);
        }
        if device_preview != 0 && ui.small_button("Reset").clicked() {
            ensure_props_object(props).remove(&key);
        }
    });
}

fn append_selected_block(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    if let Err(e) = govern_builder_non_sql_action(state, db, "append_widget", json!({})) {
        state.status_message = Some(e);
        return;
    }
    record_snapshot(state);
    let (block_type, _, _, _) = BLOCK_LIBRARY
        .get(state.pagebuilder_selected_block_idx)
        .copied()
        .unwrap_or(("text", "Text", "", "content"));
    if block_type == "section" {
        add_section_with_columns(state, db, 1);
        return;
    }

    let id = format!("blk-{}", uuid::Uuid::new_v4().simple());
    let mut props = default_props(block_type);
    if let Some(sel_idx) = state.pagebuilder_selected_canvas_idx {
        if let Some(sel) = state.vitrine_pagebuilder_blocks.get(sel_idx) {
            if sel.block_type == "column" {
                set_prop_string(&mut props, "parent_id", sel.id.clone());
            } else if let Some(pid) = parent_id(&sel.props) {
                set_prop_string(&mut props, "parent_id", pid);
            }
        }
    }
    state.vitrine_pagebuilder_blocks.push(PageBuilderBlock {
        id,
        block_type: block_type.to_string(),
        props,
    });
    state.pagebuilder_selected_canvas_idx = Some(state.vitrine_pagebuilder_blocks.len() - 1);
}

fn duplicate_selected_block(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    if let Err(e) = govern_builder_non_sql_action(state, db, "duplicate_widget", json!({})) {
        state.status_message = Some(e);
        return;
    }
    record_snapshot(state);
    let Some(i) = state.pagebuilder_selected_canvas_idx else {
        return;
    };
    if i >= state.vitrine_pagebuilder_blocks.len() {
        return;
    }
    let mut cloned = state.vitrine_pagebuilder_blocks[i].clone();
    cloned.id = format!("blk-{}", uuid::Uuid::new_v4().simple());
    state.vitrine_pagebuilder_blocks.insert(i + 1, cloned);
    state.pagebuilder_selected_canvas_idx = Some(i + 1);
}

fn remove_selected_block(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    if let Err(e) = govern_builder_non_sql_action(state, db, "remove_widget", json!({})) {
        state.status_message = Some(e);
        return;
    }
    record_snapshot(state);
    let Some(i) = state.pagebuilder_selected_canvas_idx else {
        return;
    };
    if i < state.vitrine_pagebuilder_blocks.len() {
        let deleted_id = state.vitrine_pagebuilder_blocks[i].id.clone();
        remove_block_subtree(&mut state.vitrine_pagebuilder_blocks, &deleted_id);
        state.pagebuilder_selected_canvas_idx = None;
    }
}

fn apply_default_template(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    if let Err(e) = govern_builder_non_sql_action(state, db, "apply_template", json!({"template":"mini-site"})) {
        state.status_message = Some(e);
        return;
    }
    record_snapshot(state);
    let sec = "sec-default".to_string();
    let col1 = "col-default-1".to_string();
    let col2 = "col-default-2".to_string();
    let mut sec_props = default_props("section");
    set_prop_string(&mut sec_props, "title", "Section Hero + Story".to_string());
    let mut col1_props = default_props("column");
    set_prop_string(&mut col1_props, "parent_id", sec.clone());
    let mut col2_props = default_props("column");
    set_prop_string(&mut col2_props, "parent_id", sec.clone());
    let mut hero_props = default_props("hero");
    set_prop_string(&mut hero_props, "parent_id", col1.clone());
    let mut text_props = default_props("text");
    set_prop_string(&mut text_props, "parent_id", col2.clone());
    let mut cta_props = default_props("button");
    set_prop_string(&mut cta_props, "parent_id", col2.clone());

    state.vitrine_pagebuilder_blocks = vec![
        PageBuilderBlock { id: sec, block_type: "section".to_string(), props: sec_props },
        PageBuilderBlock { id: col1, block_type: "column".to_string(), props: col1_props },
        PageBuilderBlock { id: col2, block_type: "column".to_string(), props: col2_props },
        PageBuilderBlock { id: "hero-default".to_string(), block_type: "hero".to_string(), props: hero_props },
        PageBuilderBlock { id: "text-default".to_string(), block_type: "text".to_string(), props: text_props },
        PageBuilderBlock { id: "cta-default".to_string(), block_type: "button".to_string(), props: cta_props },
    ];
    state.pagebuilder_selected_canvas_idx = Some(1);
}

fn default_props(block_type: &str) -> Value {
    match block_type {
        "section" => {
            json!({"title":"Section","bg_color":"#0F172A","padding":16})
        }
        "column" => {
            json!({"width_pct":50,"padding":8})
        }
        "hero" => {
            json!({"title":"Bienvenue","subtitle":"Decouvrez notre univers","cta_text":"Voir le catalogue","cta_link":"/catalogue","align":"left","color":"#E5E7EB","bg_color":"#111827","font_size":32,"padding":24})
        }
        "heading" => {
            json!({"text":"Qui sommes-nous ?","tag":"h2","align":"left","color":"#E5E7EB","font_size":36,"padding":8})
        }
        "text" => {
            json!({"content":"Ajoutez votre description ici.","align":"left","color":"#D1D5DB","font_size":16,"padding":8})
        }
        "button" => {
            json!({"text":"Demander un devis","link":"/contact","size":"md","align":"left","color":"#111827","bg_color":"#22D3EE","font_size":14,"padding":10})
        }
        "image" => {
            json!({"url":"","alt":"","align":"center","padding":8})
        }
        "product_grid" => {
            json!({"limit":6,"columns":3,"show_price":true,"show_button":true,"padding":8})
        }
        "features" => {
            json!({"items_text":"Fait main\nLivraison locale\nService premium","padding":8})
        }
        "faq" => {
            json!({"qa_text":"Q: Delai ?\nR: 48h","padding":8})
        }
        "spacer" => {
            json!({"height":40})
        }
        "divider" => {
            json!({"style":"solid","color":"#374151","width":1,"padding":8})
        }
        "video" => {
            json!({"provider":"youtube","embed_id":"","autoplay":false,"align":"center","padding":12})
        }
        "gallery" => {
            json!({"images_text":"","columns":3,"gap":8,"padding":8})
        }
        "icon" => {
            json!({"icon":"star","label":"","size":24,"color":"#22D3EE","align":"left","padding":8})
        }
        "form" => {
            json!({"fields_text":"Nom\nEmail\nMessage","submit_text":"Envoyer","redirect":"","align":"left","bg_color":"#1F2937","color":"#E5E7EB","padding":16})
        }
        "accordion" => {
            json!({"items_json":"[{\"title\":\"Question 1\",\"content\":\"Reponse 1\"},{\"title\":\"Question 2\",\"content\":\"Reponse 2\"}]","padding":8,"color":"#E5E7EB","bg_color":"#111827"})
        }
        "tabs" => {
            json!({"tabs_json":"[{\"title\":\"Onglet 1\",\"content\":\"Contenu onglet 1\"},{\"title\":\"Onglet 2\",\"content\":\"Contenu onglet 2\"}]","padding":8,"color":"#E5E7EB","bg_color":"#111827"})
        }
        "contact_info" => {
            json!({"show_email":true,"show_phone":true,"show_address":true,"email":"","phone":"","address":"","padding":12,"color":"#E5E7EB"})
        }
        "social_links" => {
            json!({"facebook":"","instagram":"","linkedin":"","tiktok":"","youtube":"","x":"","style":"icon","size":20,"padding":8})
        }
        _ => json!({"content":""}),
    }
}

fn add_section_with_columns(state: &mut ExpState, db: &Arc<JayXposeDb>, cols: i32) {
    if let Err(e) = govern_builder_non_sql_action(
        state,
        db,
        "add_section_with_columns",
        json!({ "columns": cols }),
    ) {
        state.status_message = Some(e);
        return;
    }
    record_snapshot(state);
    let sec_id = format!("sec-{}", uuid::Uuid::new_v4().simple());
    let mut sec_props = default_props("section");
    set_prop_string(&mut sec_props, "title", format!("Section {}", state.vitrine_pagebuilder_blocks.iter().filter(|b| b.block_type=="section").count()+1));
    state.vitrine_pagebuilder_blocks.push(PageBuilderBlock {
        id: sec_id.clone(),
        block_type: "section".to_string(),
        props: sec_props,
    });

    for i in 0..cols.max(1) {
        let col_id = format!("col-{}-{}", uuid::Uuid::new_v4().simple(), i + 1);
        let mut col_props = default_props("column");
        set_prop_string(&mut col_props, "parent_id", sec_id.clone());
        set_prop_i32(&mut col_props, "width_pct", 100 / cols.max(1));
        state.vitrine_pagebuilder_blocks.push(PageBuilderBlock {
            id: col_id,
            block_type: "column".to_string(),
            props: col_props,
        });
    }
    state.pagebuilder_selected_canvas_idx = state
        .vitrine_pagebuilder_blocks
        .iter()
        .enumerate()
        .find(|(_, b)| b.block_type == "column" && parent_id(&b.props).as_deref() == Some(sec_id.as_str()))
        .map(|(i, _)| i);
}

fn hydrate_presentation_state(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    let presentation_page = state
        .vitrine_pages
        .iter()
        .find(|p| p.page_type.as_deref() == Some("presentation"))
        .cloned();

    if state.vitrine_templates.is_empty() {
        if let Ok(templates) = db.vitrine_templates_list() {
            state.vitrine_templates = templates;
        }
    }

    if let Some(page) = presentation_page {
        if let Some(page_id) = page.id.as_deref() {
            if let Ok(blocks) = db.vitrine_blocks_by_page(page_id) {
                if !blocks.is_empty() {
                    state.vitrine_pagebuilder_blocks = blocks
                        .into_iter()
                        .map(|b| PageBuilderBlock {
                            id: b
                                .block_key
                                .unwrap_or_else(|| format!("blk-{}", uuid::Uuid::new_v4().simple())),
                            block_type: b.block_type.unwrap_or_else(|| "text".to_string()),
                            props: serde_json::from_str(b.props_json.as_deref().unwrap_or("{}"))
                                .unwrap_or_else(|_| json!({})),
                        })
                        .collect();
                    state.pagebuilder_selected_canvas_idx = Some(0);
                    state.presentation_content = serialize_document(&build_document(state));
                    return;
                }
            }
        }
        if let Some(content) = page.content {
            if let Ok(doc) = serde_json::from_str::<PageBuilderDocument>(&content) {
                hydrate_globals_from_settings(state, &doc.settings);
                state.vitrine_pagebuilder_blocks = doc.blocks;
                state.pagebuilder_selected_canvas_idx = Some(0);
                state.presentation_content = serialize_document(&build_document(state));
                return;
            }
        }
    }

    apply_default_template(state, db);
    state.presentation_content = serialize_document(&build_document(state));
}

fn ensure_pagebuilder_globals(state: &mut ExpState) {
    if state.pagebuilder_global_primary.is_empty() {
        state.pagebuilder_global_primary = "#22D3EE".to_string();
    }
    if state.pagebuilder_global_text.is_empty() {
        state.pagebuilder_global_text = "#E5E7EB".to_string();
    }
    if state.pagebuilder_global_bg.is_empty() {
        state.pagebuilder_global_bg = "#0B1220".to_string();
    }
    if state.pagebuilder_global_heading_size <= 0 {
        state.pagebuilder_global_heading_size = 36;
    }
    if state.pagebuilder_global_body_size <= 0 {
        state.pagebuilder_global_body_size = 16;
    }
}

fn hydrate_globals_from_settings(state: &mut ExpState, settings: &Value) {
    if let Some(colors) = settings.get("global_colors") {
        if let Some(v) = colors.get("primary").and_then(Value::as_str) {
            state.pagebuilder_global_primary = v.to_string();
        }
        if let Some(v) = colors.get("text").and_then(Value::as_str) {
            state.pagebuilder_global_text = v.to_string();
        }
        if let Some(v) = colors.get("bg").and_then(Value::as_str) {
            state.pagebuilder_global_bg = v.to_string();
        }
    }
    if let Some(typo) = settings.get("global_typography") {
        if let Some(v) = typo.get("heading_size").and_then(Value::as_i64) {
            state.pagebuilder_global_heading_size = v as i32;
        }
        if let Some(v) = typo.get("body_size").and_then(Value::as_i64) {
            state.pagebuilder_global_body_size = v as i32;
        }
    }
    ensure_pagebuilder_globals(state);
}

fn build_document(state: &ExpState) -> PageBuilderDocument {
    PageBuilderDocument {
        version: "1.1.0".to_string(),
        page_id: state
            .vitrine_pages
            .iter()
            .find(|p| p.page_type.as_deref() == Some("presentation"))
            .and_then(|p| p.id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        title: "Presentation".to_string(),
        slug: "presentation".to_string(),
        settings: json!({
            "device_preview": state.pagebuilder_device_preview,
            "global_colors": {
                "primary": state.pagebuilder_global_primary.clone(),
                "text": state.pagebuilder_global_text.clone(),
                "bg": state.pagebuilder_global_bg.clone()
            },
            "global_typography": {
                "heading_size": state.pagebuilder_global_heading_size,
                "body_size": state.pagebuilder_global_body_size
            }
        }),
        blocks: state.vitrine_pagebuilder_blocks.clone(),
    }
}

fn serialize_document(doc: &PageBuilderDocument) -> String {
    serde_json::to_string_pretty(doc).unwrap_or_else(|_| "{}".to_string())
}

fn save_presentation(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    let parsed_doc = serde_json::from_str::<PageBuilderDocument>(&state.presentation_content)
        .unwrap_or_else(|_| build_document(state));

    let mut page = if let Some(existing) = state
        .vitrine_pages
        .iter_mut()
        .find(|p| p.page_type.as_deref() == Some("presentation"))
    {
        existing.content = Some(serialize_document(&parsed_doc));
        existing.clone()
    } else {
        let new_page = VitrinePage {
            id: Some(parsed_doc.page_id.clone()),
            exposant_id: state.current_exposant_id.clone(),
            page_type: Some("presentation".to_string()),
            content: Some(serialize_document(&parsed_doc)),
            is_visible: Some(true),
            sort_order: Some(2),
            updated_at: None,
        };
        state.vitrine_pages.push(new_page.clone());
        new_page
    };
    page.content = Some(serialize_document(&parsed_doc));

    match db.vitrine_page_upsert_return_id(&page) {
        Ok(page_id) => {
            let blocks: Vec<VitrineBlock> = parsed_doc
                .blocks
                .iter()
                .enumerate()
                .map(|(idx, b)| VitrineBlock {
                    id: None,
                    page_id: Some(page_id.clone()),
                    block_key: Some(b.id.clone()),
                    block_type: Some(b.block_type.clone()),
                    props_json: Some(
                        serde_json::to_string(&b.props).unwrap_or_else(|_| "{}".to_string()),
                    ),
                    position: Some(idx as i32),
                    created_at: None,
                    updated_at: None,
                })
                .collect();

            if db.vitrine_blocks_replace(&page_id, &blocks).is_err() {
                state.status_message = Some("Page sauvee mais blocs non persistes.".to_string());
                return;
            }

            state.vitrine_pagebuilder_blocks = parsed_doc.blocks.clone();
            state.presentation_content = serialize_document(&parsed_doc);
            if let Some(existing) = state
                .vitrine_pages
                .iter_mut()
                .find(|p| p.page_type.as_deref() == Some("presentation"))
            {
                existing.id = Some(page_id);
                existing.content = Some(state.presentation_content.clone());
            }
            state.status_message = Some("Page builder enregistre avec personnalisation complete.".to_string());
        }
        Err(e) => state.status_message = Some(format!("Erreur enregistrement : {e}")),
    }
}

fn navigator_window(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    if !state.pagebuilder_navigator_open {
        return;
    }
    let mut open = state.pagebuilder_navigator_open;
    egui::Window::new("Navigator")
        .open(&mut open)
        .vscroll(true)
        .default_width(340.0)
        .show(ctx, |ui| {
            let section_indices: Vec<usize> = state
                .vitrine_pagebuilder_blocks
                .iter()
                .enumerate()
                .filter(|(_, b)| b.block_type == "section")
                .map(|(i, _)| i)
                .collect();
            if section_indices.is_empty() {
                ui.label(egui::RichText::new("Aucune section").color(theme.text_secondary()));
            }

            for sec_idx in section_indices {
                let sec = &state.vitrine_pagebuilder_blocks[sec_idx];
                let sec_id = sec.id.clone();
                let sec_title = prop_string(&sec.props, "title", "Section");
                ui.collapsing(format!("Section: {sec_title}"), |ui| {
                    if ui
                        .selectable_label(
                            state.pagebuilder_selected_canvas_idx == Some(sec_idx),
                            format!("id={sec_id}"),
                        )
                        .clicked()
                    {
                        state.pagebuilder_selected_canvas_idx = Some(sec_idx);
                    }
                    let col_indices: Vec<usize> = state
                        .vitrine_pagebuilder_blocks
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| {
                            b.block_type == "column"
                                && parent_id(&b.props).as_deref() == Some(sec_id.as_str())
                        })
                        .map(|(i, _)| i)
                        .collect();
                    for col_idx in col_indices {
                        let col = &state.vitrine_pagebuilder_blocks[col_idx];
                        ui.indent(format!("col-{}", col.id), |ui| {
                            if ui
                                .selectable_label(
                                    state.pagebuilder_selected_canvas_idx == Some(col_idx),
                                    format!(
                                        "Colonne {} ({})",
                                        prop_i32(&col.props, "width_pct", 50),
                                        col.id
                                    ),
                                )
                                .clicked()
                            {
                                state.pagebuilder_selected_canvas_idx = Some(col_idx);
                            }
                            let widgets: Vec<(usize, String)> = state
                                .vitrine_pagebuilder_blocks
                                .iter()
                                .enumerate()
                                .filter(|(_, b)| {
                                    b.block_type != "section"
                                        && b.block_type != "column"
                                        && parent_id(&b.props).as_deref() == Some(col.id.as_str())
                                })
                                .map(|(i, b)| (i, block_label(b)))
                                .collect();
                            for (wi, label) in widgets {
                                if ui
                                    .selectable_label(
                                        state.pagebuilder_selected_canvas_idx == Some(wi),
                                        format!("- {label}"),
                                    )
                                    .clicked()
                                {
                                    state.pagebuilder_selected_canvas_idx = Some(wi);
                                }
                            }
                        });
                    }
                });
            }
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Monter").clicked() {
                    if let Some(sel) = state.pagebuilder_selected_canvas_idx {
                        record_snapshot(state);
                        if !move_widget_in_column(&mut state.vitrine_pagebuilder_blocks, sel, -1) {
                            if sel > 0 {
                                state.vitrine_pagebuilder_blocks.swap(sel, sel - 1);
                                state.pagebuilder_selected_canvas_idx = Some(sel - 1);
                            }
                        }
                    }
                }
                if ui.button("Descendre").clicked() {
                    if let Some(sel) = state.pagebuilder_selected_canvas_idx {
                        record_snapshot(state);
                        if !move_widget_in_column(&mut state.vitrine_pagebuilder_blocks, sel, 1) {
                            if sel + 1 < state.vitrine_pagebuilder_blocks.len() {
                                state.vitrine_pagebuilder_blocks.swap(sel, sel + 1);
                                state.pagebuilder_selected_canvas_idx = Some(sel + 1);
                            }
                        }
                    }
                }
                if ui.button("Dupliquer").clicked() {
                    duplicate_selected_block(state, db);
                }
                if ui.button("Supprimer").clicked() {
                    remove_selected_block(state, db);
                }
            });
        });
    state.pagebuilder_navigator_open = open;
}

fn site_settings_window(ctx: &egui::Context, theme: &JayXposeTheme, state: &mut ExpState) {
    if !state.pagebuilder_settings_open {
        return;
    }
    let mut open = state.pagebuilder_settings_open;
    egui::Window::new("Reglages du site")
        .open(&mut open)
        .default_width(360.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Couleurs globales").strong());
            ui.text_edit_singleline(&mut state.pagebuilder_global_primary);
            ui.text_edit_singleline(&mut state.pagebuilder_global_text);
            ui.text_edit_singleline(&mut state.pagebuilder_global_bg);
            ui.label(egui::RichText::new("Typographie globale").strong());
            ui.add(egui::Slider::new(
                &mut state.pagebuilder_global_heading_size,
                16..=72,
            ).text("Heading size"));
            ui.add(egui::Slider::new(
                &mut state.pagebuilder_global_body_size,
                10..=28,
            ).text("Body size"));
            if ui.button("Appliquer aux blocs existants").clicked() {
                record_snapshot(state);
                apply_global_settings_to_blocks(state);
                state.status_message = Some("Reglages globaux appliques.".to_string());
            }
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(
                    "Astuce: Enregistrez ensuite pour persister les global settings dans le document.",
                )
                .size(11.0)
                .color(theme.text_secondary()),
            );
        });
    state.pagebuilder_settings_open = open;
}

fn template_library_window(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    if !state.pagebuilder_templates_open {
        return;
    }
    let mut open = state.pagebuilder_templates_open;
    egui::Window::new("Template Library")
        .open(&mut open)
        .vscroll(true)
        .default_width(520.0)
        .show(ctx, |ui| {
            ui.label(egui::RichText::new("Pages").strong());
            ui.add_space(6.0);
            for (key, title, desc) in TEMPLATE_LIBRARY {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(17, 28, 49))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(54, 79, 120)))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(egui::RichText::new(title).strong());
                                ui.label(
                                    egui::RichText::new(desc)
                                        .size(11.0)
                                        .color(theme.text_secondary()),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("Inserer").clicked() {
                                        apply_template_by_key(state, db, key);
                                    }
                                },
                            );
                        });
                    });
                ui.add_space(6.0);
            }
        });
    state.pagebuilder_templates_open = open;
}

fn device_hint(device: usize) -> &'static str {
    match device {
        1 => "Preview tablette ~ 820px",
        2 => "Preview mobile ~ 390px",
        _ => "Preview desktop ~ 1280px",
    }
}

fn block_label(block: &PageBuilderBlock) -> String {
    let mut label = block.block_type.clone();
    if let Some(t) = block.props.get("text").and_then(Value::as_str) {
        if !t.is_empty() {
            label = format!("{label}: {t}");
        }
    }
    if let Some(t) = block.props.get("title").and_then(Value::as_str) {
        if !t.is_empty() {
            label = format!("{label}: {t}");
        }
    }
    label
}

fn widget_preview_line(block: &PageBuilderBlock) -> String {
    match block.block_type.as_str() {
        "hero" => format!(
            "{} | {}",
            prop_string(&block.props, "title", "Bienvenue"),
            prop_string(&block.props, "subtitle", "")
        ),
        "heading" => prop_string(&block.props, "text", "Titre"),
        "text" => {
            let t = prop_string(&block.props, "content", "");
            if t.chars().count() > 72 {
                format!("{}...", t.chars().take(72).collect::<String>())
            } else {
                t
            }
        }
        "button" => format!(
            "CTA: {} -> {}",
            prop_string(&block.props, "text", "Bouton"),
            prop_string(&block.props, "link", "#")
        ),
        "image" => format!("Image: {}", prop_string(&block.props, "url", "(vide)")),
        "product_grid" => format!(
            "Catalogue: {} produits / {} colonnes",
            prop_i32(&block.props, "limit", 6),
            prop_i32(&block.props, "columns", 3)
        ),
        "features" => "Liste d'avantages".to_string(),
        "faq" => "Bloc FAQ".to_string(),
        "spacer" => format!("Espace: {}px", prop_i32(&block.props, "height", 40)),
        "divider" => format!(
            "Separateur {} {}px",
            prop_string(&block.props, "style", "solid"),
            prop_i32(&block.props, "width", 1)
        ),
        "video" => format!(
            "Video: {} ({})",
            prop_string(&block.props, "provider", "youtube"),
            prop_string(&block.props, "embed_id", "(vide)")
        ),
        "gallery" => {
            let urls = prop_string(&block.props, "images_text", "");
            let count = urls.lines().filter(|l| !l.trim().is_empty()).count();
            format!("Galerie: {} images / {} col", count, prop_i32(&block.props, "columns", 3))
        }
        "icon" => format!(
            "Icone: {} {}",
            prop_string(&block.props, "icon", "star"),
            prop_string(&block.props, "label", "")
        ),
        "form" => format!(
            "Formulaire: {} champs",
            prop_string(&block.props, "fields_text", "")
                .lines()
                .filter(|l| !l.trim().is_empty())
                .count()
        ),
        "accordion" => "Accordeon".to_string(),
        "tabs" => "Onglets".to_string(),
        "contact_info" => "Coordonnees".to_string(),
        "social_links" => "Reseaux sociaux".to_string(),
        _ => block.block_type.clone(),
    }
}

fn apply_global_settings_to_blocks(state: &mut ExpState) {
    for block in &mut state.vitrine_pagebuilder_blocks {
        match block.block_type.as_str() {
            "section" => {
                set_prop_string(
                    &mut block.props,
                    "bg_color",
                    state.pagebuilder_global_bg.clone(),
                );
            }
            "hero" | "heading" => {
                set_prop_string(
                    &mut block.props,
                    "color",
                    state.pagebuilder_global_text.clone(),
                );
                set_prop_i32(
                    &mut block.props,
                    "font_size",
                    state.pagebuilder_global_heading_size,
                );
            }
            "text" | "faq" | "features" => {
                set_prop_string(
                    &mut block.props,
                    "color",
                    state.pagebuilder_global_text.clone(),
                );
                set_prop_i32(
                    &mut block.props,
                    "font_size",
                    state.pagebuilder_global_body_size,
                );
            }
            "button" => {
                set_prop_string(
                    &mut block.props,
                    "bg_color",
                    state.pagebuilder_global_primary.clone(),
                );
                set_prop_i32(
                    &mut block.props,
                    "font_size",
                    state.pagebuilder_global_body_size,
                );
            }
            _ => {}
        }
    }
}

fn apply_template_by_key(state: &mut ExpState, db: &Arc<JayXposeDb>, key: &str) {
    if key == "mini-site" {
        apply_default_template(state, db);
        return;
    }
    if let Err(e) = govern_builder_non_sql_action(state, db, "apply_template", json!({"template":key})) {
        state.status_message = Some(e);
        return;
    }
    record_snapshot(state);
    let sec_id = format!("sec-{}", uuid::Uuid::new_v4().simple());
    let col_a = format!("col-{}-1", uuid::Uuid::new_v4().simple());
    let col_b = format!("col-{}-2", uuid::Uuid::new_v4().simple());
    let mut blocks = vec![];

    let mut sec_props = default_props("section");
    let mut col_a_props = default_props("column");
    let mut col_b_props = default_props("column");
    set_prop_string(&mut col_a_props, "parent_id", sec_id.clone());
    set_prop_string(&mut col_b_props, "parent_id", sec_id.clone());
    blocks.push(PageBuilderBlock {
        id: sec_id.clone(),
        block_type: "section".to_string(),
        props: sec_props.clone(),
    });
    blocks.push(PageBuilderBlock {
        id: col_a.clone(),
        block_type: "column".to_string(),
        props: col_a_props,
    });
    blocks.push(PageBuilderBlock {
        id: col_b.clone(),
        block_type: "column".to_string(),
        props: col_b_props,
    });

    match key {
        "landing" => {
            set_prop_string(&mut sec_props, "title", "Landing Produit".to_string());
            let mut hero = default_props("hero");
            set_prop_string(&mut hero, "parent_id", col_a.clone());
            set_prop_string(&mut hero, "title", "Lancez votre offre".to_string());
            set_prop_string(&mut hero, "cta_text", "Commander".to_string());
            let mut features = default_props("features");
            set_prop_string(&mut features, "parent_id", col_b.clone());
            set_prop_string(
                &mut features,
                "items_text",
                "Livraison 24h\nPaiement securise\nSupport dedie".to_string(),
            );
            let mut cta = default_props("button");
            set_prop_string(&mut cta, "parent_id", col_b.clone());
            set_prop_string(&mut cta, "text", "Je me lance".to_string());
            blocks[0].props = sec_props;
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "hero".to_string(), props: hero });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "features".to_string(), props: features });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "button".to_string(), props: cta });
        }
        "storytelling" => {
            set_prop_string(&mut sec_props, "title", "Notre Histoire".to_string());
            let mut heading = default_props("heading");
            set_prop_string(&mut heading, "parent_id", col_a.clone());
            set_prop_string(&mut heading, "text", "Un savoir-faire artisanal".to_string());
            let mut text = default_props("text");
            set_prop_string(&mut text, "parent_id", col_a.clone());
            set_prop_string(&mut text, "content", "Racontez ici la vision, l'origine, les engagements et la promesse de marque.".to_string());
            let mut image = default_props("image");
            set_prop_string(&mut image, "parent_id", col_b.clone());
            blocks[0].props = sec_props;
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "heading".to_string(), props: heading });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "text".to_string(), props: text });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "image".to_string(), props: image });
        }
        "catalog-focus" => {
            set_prop_string(&mut sec_props, "title", "Catalogue".to_string());
            let mut hero = default_props("hero");
            set_prop_string(&mut hero, "parent_id", col_a.clone());
            set_prop_string(&mut hero, "title", "Decouvrez nos best-sellers".to_string());
            let mut grid = default_props("product_grid");
            set_prop_string(&mut grid, "parent_id", col_b.clone());
            set_prop_i32(&mut grid, "limit", 9);
            set_prop_i32(&mut grid, "columns", 3);
            blocks[0].props = sec_props;
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "hero".to_string(), props: hero });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "product_grid".to_string(), props: grid });
        }
        "e-shop" => {
            set_prop_string(&mut sec_props, "title", "E-Shop".to_string());
            let mut hero = default_props("hero");
            set_prop_string(&mut hero, "parent_id", col_a.clone());
            set_prop_string(&mut hero, "title", "Bienvenue dans notre boutique".to_string());
            set_prop_string(&mut hero, "subtitle", "Decouvrez nos creations artisanales".to_string());
            set_prop_string(&mut hero, "cta_text", "Voir le catalogue".to_string());
            let mut grid = default_props("product_grid");
            set_prop_string(&mut grid, "parent_id", col_b.clone());
            set_prop_i32(&mut grid, "limit", 9);
            set_prop_i32(&mut grid, "columns", 3);
            blocks[0].props = sec_props;
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "hero".to_string(), props: hero });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "product_grid".to_string(), props: grid });

            // Section 2 : Avantages + Contact
            let sec2_id = format!("sec-{}", uuid::Uuid::new_v4().simple());
            let col2_a = format!("col-{}-1", uuid::Uuid::new_v4().simple());
            let col2_b = format!("col-{}-2", uuid::Uuid::new_v4().simple());
            let mut sec2_props = default_props("section");
            set_prop_string(&mut sec2_props, "title", "Avantages & Contact".to_string());
            let mut col2_a_props = default_props("column");
            set_prop_string(&mut col2_a_props, "parent_id", sec2_id.clone());
            let mut col2_b_props = default_props("column");
            set_prop_string(&mut col2_b_props, "parent_id", sec2_id.clone());
            blocks.push(PageBuilderBlock { id: sec2_id.clone(), block_type: "section".to_string(), props: sec2_props });
            blocks.push(PageBuilderBlock { id: col2_a.clone(), block_type: "column".to_string(), props: col2_a_props });
            blocks.push(PageBuilderBlock { id: col2_b.clone(), block_type: "column".to_string(), props: col2_b_props });

            let mut features = default_props("features");
            set_prop_string(&mut features, "parent_id", col2_a.clone());
            set_prop_string(&mut features, "items_text", "Livraison rapide\nPaiement securise\nFait main\nSatisfait ou rembourse".to_string());
            let mut form = default_props("form");
            set_prop_string(&mut form, "parent_id", col2_b.clone());
            set_prop_string(&mut form, "fields_text", "*Nom\n*Email\nMessage".to_string());
            set_prop_string(&mut form, "submit_text", "Nous contacter".to_string());
            let mut social = default_props("social_links");
            set_prop_string(&mut social, "parent_id", col2_b.clone());
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "features".to_string(), props: features });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "form".to_string(), props: form });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "social_links".to_string(), props: social });
        }
        "service-shop" => {
            set_prop_string(&mut sec_props, "title", "Nos Services".to_string());
            let mut hero = default_props("hero");
            set_prop_string(&mut hero, "parent_id", col_a.clone());
            set_prop_string(&mut hero, "title", "Nos Services".to_string());
            set_prop_string(&mut hero, "subtitle", "Expertise et savoir-faire a votre service".to_string());
            set_prop_string(&mut hero, "cta_text", "Prendre RDV".to_string());
            set_prop_string(&mut hero, "cta_link", "/contact".to_string());
            let mut acc = default_props("accordion");
            set_prop_string(&mut acc, "parent_id", col_b.clone());
            set_prop_string(&mut acc, "items_json", r#"[{"title":"Consultation","content":"Diagnostic personnalise et recommandations sur mesure."},{"title":"Creation","content":"Conception et realisation de votre projet."},{"title":"Suivi","content":"Accompagnement continu et support."}]"#.to_string());
            blocks[0].props = sec_props;
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "hero".to_string(), props: hero });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "accordion".to_string(), props: acc });

            // Section 2 : Contact + Infos
            let sec2_id = format!("sec-{}", uuid::Uuid::new_v4().simple());
            let col2_a = format!("col-{}-1", uuid::Uuid::new_v4().simple());
            let col2_b = format!("col-{}-2", uuid::Uuid::new_v4().simple());
            let mut sec2_props = default_props("section");
            set_prop_string(&mut sec2_props, "title", "Contactez-nous".to_string());
            let mut col2_a_props = default_props("column");
            set_prop_string(&mut col2_a_props, "parent_id", sec2_id.clone());
            let mut col2_b_props = default_props("column");
            set_prop_string(&mut col2_b_props, "parent_id", sec2_id.clone());
            blocks.push(PageBuilderBlock { id: sec2_id.clone(), block_type: "section".to_string(), props: sec2_props });
            blocks.push(PageBuilderBlock { id: col2_a.clone(), block_type: "column".to_string(), props: col2_a_props });
            blocks.push(PageBuilderBlock { id: col2_b.clone(), block_type: "column".to_string(), props: col2_b_props });

            let mut contact = default_props("contact_info");
            set_prop_string(&mut contact, "parent_id", col2_a.clone());
            let mut form = default_props("form");
            set_prop_string(&mut form, "parent_id", col2_b.clone());
            set_prop_string(&mut form, "fields_text", "*Nom\n*Email\n*Service souhaite\nMessage".to_string());
            set_prop_string(&mut form, "submit_text", "Demander un devis".to_string());
            let mut social = default_props("social_links");
            set_prop_string(&mut social, "parent_id", col2_a.clone());
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "contact_info".to_string(), props: contact });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "form".to_string(), props: form });
            blocks.push(PageBuilderBlock { id: format!("blk-{}", uuid::Uuid::new_v4().simple()), block_type: "social_links".to_string(), props: social });
        }
        _ => {
            return;
        }
    }
    state.vitrine_pagebuilder_blocks = blocks;
    state.pagebuilder_selected_canvas_idx = Some(1);
    state.status_message = Some(format!("Template '{key}' applique."));
}

fn ensure_props_object(props: &mut Value) -> &mut Map<String, Value> {
    if !props.is_object() {
        *props = json!({});
    }
    props.as_object_mut().expect("props should be object")
}

fn prop_string(props: &Value, key: &str, default: &str) -> String {
    props
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or(default)
        .to_string()
}

fn set_prop_string(props: &mut Value, key: &str, value: String) {
    ensure_props_object(props).insert(key.to_string(), Value::String(value));
}

fn prop_i32(props: &Value, key: &str, default: i32) -> i32 {
    props
        .get(key)
        .and_then(Value::as_i64)
        .map(|v| v as i32)
        .unwrap_or(default)
}

fn set_prop_i32(props: &mut Value, key: &str, value: i32) {
    ensure_props_object(props).insert(key.to_string(), Value::Number(value.into()));
}

fn prop_bool(props: &Value, key: &str, default: bool) -> bool {
    props.get(key).and_then(Value::as_bool).unwrap_or(default)
}

fn set_prop_bool(props: &mut Value, key: &str, value: bool) {
    ensure_props_object(props).insert(key.to_string(), Value::Bool(value));
}

fn parent_id(props: &Value) -> Option<String> {
    props
        .get("parent_id")
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn remove_block_subtree(blocks: &mut Vec<PageBuilderBlock>, root_id: &str) {
    let mut to_remove: HashSet<String> = HashSet::new();
    to_remove.insert(root_id.to_string());
    loop {
        let before = to_remove.len();
        for block in blocks.iter() {
            if let Some(pid) = parent_id(&block.props) {
                if to_remove.contains(&pid) {
                    to_remove.insert(block.id.clone());
                }
            }
        }
        if to_remove.len() == before {
            break;
        }
    }
    blocks.retain(|b| !to_remove.contains(&b.id));
}

fn move_widget_in_column(blocks: &mut [PageBuilderBlock], widget_idx: usize, delta: i32) -> bool {
    if widget_idx >= blocks.len() {
        return false;
    }
    if blocks[widget_idx].block_type == "section" || blocks[widget_idx].block_type == "column" {
        return false;
    }
    let Some(parent) = parent_id(&blocks[widget_idx].props) else {
        return false;
    };
    let siblings: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| {
            b.block_type != "section"
                && b.block_type != "column"
                && parent_id(&b.props).as_deref() == Some(parent.as_str())
        })
        .map(|(i, _)| i)
        .collect();
    let Some(pos) = siblings.iter().position(|i| *i == widget_idx) else {
        return false;
    };
    if delta < 0 {
        if pos == 0 {
            return false;
        }
        let a = siblings[pos - 1];
        let b = siblings[pos];
        blocks.swap(a, b);
        return true;
    }
    if delta > 0 {
        if pos + 1 >= siblings.len() {
            return false;
        }
        let a = siblings[pos];
        let b = siblings[pos + 1];
        blocks.swap(a, b);
        return true;
    }
    false
}

/// Déplace un widget (non section/column) vers une colonne et une position d'insertion donnée.
/// Retourne le nouvel index global du widget si succès.
fn move_widget_to_column_at_pos(
    blocks: &mut Vec<PageBuilderBlock>,
    widget_id: &str,
    target_col_id: &str,
    insert_pos: usize,
) -> Option<usize> {
    let widx = blocks.iter().position(|b| b.id == widget_id)?;
    if blocks[widx].block_type == "section" || blocks[widx].block_type == "column" {
        return None;
    }
    let mut block = blocks.remove(widx);
    set_prop_string(&mut block.props, "parent_id", target_col_id.to_string());

    // Siblings dans la colonne cible (après removal).
    let siblings: Vec<usize> = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| {
            b.block_type != "section"
                && b.block_type != "column"
                && parent_id(&b.props).as_deref() == Some(target_col_id)
        })
        .map(|(i, _)| i)
        .collect();

    let insert_pos = insert_pos.min(siblings.len());
    let mut insert_at = if siblings.is_empty() {
        // Colonne vide : insère juste après le bloc colonne si possible.
        blocks
            .iter()
            .position(|b| b.block_type == "column" && b.id == target_col_id)
            .map(|i| i + 1)
            .unwrap_or(blocks.len())
    } else if insert_pos >= siblings.len() {
        siblings.last().copied().unwrap_or(blocks.len()).saturating_add(1)
    } else {
        siblings[insert_pos]
    };

    if insert_at > blocks.len() {
        insert_at = blocks.len();
    }
    blocks.insert(insert_at, block);
    Some(insert_at)
}

fn record_snapshot(state: &mut ExpState) {
    let snap = serialize_document(&build_document(state));
    state.pagebuilder_history.push(snap);
    if state.pagebuilder_history.len() > 100 {
        let overflow = state.pagebuilder_history.len() - 100;
        state.pagebuilder_history.drain(0..overflow);
    }
    state.pagebuilder_future.clear();
}

fn undo_builder(state: &mut ExpState) {
    let Some(prev) = state.pagebuilder_history.pop() else {
        state.status_message = Some("Undo: aucun historique.".to_string());
        return;
    };
    let current = serialize_document(&build_document(state));
    state.pagebuilder_future.push(current);
    restore_from_snapshot(state, &prev);
    state.status_message = Some("Undo applique.".to_string());
}

fn redo_builder(state: &mut ExpState) {
    let Some(next) = state.pagebuilder_future.pop() else {
        state.status_message = Some("Redo: aucun historique futur.".to_string());
        return;
    };
    let current = serialize_document(&build_document(state));
    state.pagebuilder_history.push(current);
    restore_from_snapshot(state, &next);
    state.status_message = Some("Redo applique.".to_string());
}

fn restore_from_snapshot(state: &mut ExpState, snapshot: &str) {
    if let Ok(doc) = serde_json::from_str::<PageBuilderDocument>(snapshot) {
        state.vitrine_pagebuilder_blocks = doc.blocks;
        state.presentation_content = snapshot.to_string();
        state.pagebuilder_selected_canvas_idx = if state.vitrine_pagebuilder_blocks.is_empty() {
            None
        } else {
            Some(0)
        };
    }
}

fn extract_style_json(props: &Value) -> String {
    let style_keys = [
        "align",
        "color",
        "bg_color",
        "font_size",
        "padding",
        "css_class",
        "anchor",
        "margin_top",
        "margin_bottom",
    ];
    let mut out = Map::new();
    for k in style_keys {
        if let Some(v) = props.get(k) {
            out.insert(k.to_string(), v.clone());
        }
    }
    serde_json::to_string(&Value::Object(out)).unwrap_or_else(|_| "{}".to_string())
}

fn apply_style_json(props: &mut Value, style_json: &str) {
    let Ok(style) = serde_json::from_str::<Value>(style_json) else {
        return;
    };
    let Some(obj) = style.as_object() else {
        return;
    };
    let map = ensure_props_object(props);
    for (k, v) in obj {
        map.insert(k.clone(), v.clone());
    }
}

fn govern_builder_non_sql_action(
    state: &ExpState,
    db: &Arc<JayXposeDb>,
    action: &str,
    payload: Value,
) -> Result<(), String> {
    let ctx = GovernanceContext {
        mandate_id: state.governance_mandate_id.clone(),
        security_level: state.governance_security_level,
    };
    govern_non_sql_create(
        &ctx,
        db,
        state.current_exposant_id.as_deref(),
        action,
        payload,
    )
    .map_err(|e| format!("Gouvernance: {e}"))
}
