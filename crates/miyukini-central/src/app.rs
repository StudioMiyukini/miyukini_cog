//! Application principale du Hub Miyukini Central.
//!
//! Gère l'interface utilisateur, le header, les onglets, le menu services,
//! et l'affichage du contenu selon l'onglet actif.

use crate::catalog::{CategoryId, ServiceId};
use crate::pixel_theme::{chrome_dimensions, chrome_shapes, PixelChromeTheme};
use crate::services::{CalculatorService, GameService, NotesService, ServiceUi, TextEditorService};
use eframe::egui;
use eframe::App;

/// Application principale du Hub.
pub struct MiyukiniCentralApp {
    /// État du header (onglets, menu, etc.).
    pub header_state: HeaderState,
    /// Services ouverts (onglets actifs).
    pub open_services: Vec<Box<dyn ServiceUi>>,
    /// Catalogue des services disponibles.
    pub catalog: Vec<ServiceId>,
    /// Vue actuelle dans l'onglet HUB.
    pub hub_view: HubView,
    /// Filtre actif dans le menu services.
    pub menu_filter: Option<CategoryId>,
}

/// État du header (onglets, menu).
#[derive(Default)]
pub struct HeaderState {
    /// Onglets ouverts (premier = HUB toujours présent).
    pub tabs: Vec<TabInfo>,
    /// Index de l'onglet actif.
    pub active_tab_index: usize,
    /// Menu services ouvert ou fermé.
    pub menu_open: bool,
}

/// Information sur un onglet.
#[derive(Clone)]
pub struct TabInfo {
    /// ID du service (None pour HUB et Profile).
    pub service_id: Option<ServiceId>,
    /// Titre affiché dans l'onglet.
    pub title: String,
    /// Type d'onglet.
    pub tab_type: TabType,
}

/// Type d'onglet.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TabType {
    /// Onglet système HUB.
    Hub,
    /// Onglet service.
    Service,
    /// Onglet profil utilisateur.
    Profile,
}

/// Vue dans l'onglet HUB.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HubView {
    /// Vue d'accueil.
    Home,
    /// Vue catalogue.
    Catalog,
    /// Vue mes services.
    MyServices,
    /// Vue paramètres.
    Settings,
}

/// Résultat d'un clic sur un onglet.
#[derive(Default)]
pub struct TabClickResult {
    /// L'onglet doit être activé.
    pub activate: bool,
    /// L'onglet doit être fermé.
    pub close: bool,
}

impl MiyukiniCentralApp {
    /// Crée une nouvelle instance de l'application.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut header_state = HeaderState::default();
        // Ajouter l'onglet HUB par défaut.
        header_state.tabs.push(TabInfo {
            service_id: None,
            title: "HUB".to_string(),
            tab_type: TabType::Hub,
        });
        header_state.active_tab_index = 0;

        Self {
            header_state,
            open_services: Vec::new(),
            catalog: vec![
                ServiceId::Calculator,
                ServiceId::Game,
                ServiceId::TextEditor,
                ServiceId::Notes,
            ],
            hub_view: HubView::Home,
            menu_filter: None,
        }
    }
}

impl App for MiyukiniCentralApp {
    /// Point d'entrée principal de l'UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Appliquer le thème Chrome pixel art.
        let pixel_theme = PixelChromeTheme::new(ctx.style().visuals.dark_mode);
        pixel_theme.apply(ctx);

        // Header en 2 lignes.
        self.ui_header(ctx);

        // Contenu principal selon l'onglet actif.
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_content(ui);
        });
    }
}

impl MiyukiniCentralApp {
    /// Affiche le header en 2 lignes.
    fn ui_header(&mut self, ctx: &egui::Context) {
        const LINE1_HEIGHT: f32 = 36.0;
        const LINE2_HEIGHT: f32 = 36.0;

        // Ligne 1 : Logo, Menu Services, Profil.
        egui::TopBottomPanel::top("header_line1")
            .resizable(false)
            .frame(egui::Frame::default().inner_margin(egui::Margin::same(0)))
            .show(ctx, |ui| {
                ui.set_min_height(LINE1_HEIGHT);
                ui.set_max_height(LINE1_HEIGHT);
                ui.horizontal(|ui| {
                    self.ui_header_logo(ui);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.ui_header_profile(ui);
                        self.ui_header_menu_services_button(ui);
                    });
                });
            });

        // Ligne 2 : Onglets.
        egui::TopBottomPanel::top("header_line2")
            .resizable(false)
            .frame(egui::Frame::default().inner_margin(egui::Margin::same(0)))
            .show(ctx, |ui| {
                ui.set_min_height(LINE2_HEIGHT);
                ui.set_max_height(LINE2_HEIGHT);
                let pixel_theme = PixelChromeTheme::new(ctx.style().visuals.dark_mode);
                let tab_bar_bg = pixel_theme.tab_bar_bg();
                let painter = ui.painter();
                painter.rect_filled(ui.max_rect(), 0.0, tab_bar_bg);
                self.ui_header_tabs(ui, ctx);
            });

        // Menu services dropdown (si ouvert).
        if self.header_state.menu_open {
            self.ui_menu_services_dropdown(ctx);
        }
    }

    /// Affiche le logo Miyukini COG dans le header.
    fn ui_header_logo(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("⚙️").size(20.0));
            if ui.button(egui::RichText::new("Miyukini COG").size(13.0)).clicked() {
                // Activer l'onglet HUB.
                if let Some(hub_index) = self.header_state.tabs.iter().position(|t| t.tab_type == TabType::Hub) {
                    self.header_state.active_tab_index = hub_index;
                }
            }
            ui.label(egui::RichText::new("v1.0.0").size(11.0).weak());
            ui.separator();
        });
    }

    /// Affiche le bouton "Menu Services" dans le header.
    fn ui_header_menu_services_button(&mut self, ui: &mut egui::Ui) {
        let button_text = if self.header_state.menu_open {
            "▼ Menu Services"
        } else {
            "▶ Menu Services"
        };

        let button_response = ui.button(egui::RichText::new(button_text).size(12.0));
        
        // Stocker la position du bouton pour éviter la fermeture lors du clic
        if button_response.clicked() {
            let button_rect = button_response.rect;
            ui.ctx().data_mut(|d| {
                d.insert_temp(egui::Id::new("menu_button_rect"), button_rect);
            });
            
            self.header_state.menu_open = !self.header_state.menu_open;
            // Marquer que le menu vient de s'ouvrir pour éviter la fermeture immédiate.
            ui.ctx().data_mut(|d| {
                d.insert_temp(egui::Id::new("menu_just_opened"), true);
                d.insert_temp(egui::Id::new("menu_ready"), false);
            });
        }
    }

    /// Affiche le profil utilisateur dans le header.
    fn ui_header_profile(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("👤").size(20.0));
            if ui.button(egui::RichText::new("Utilisateur").size(12.0)).clicked() {
                self.open_profile_tab();
            }
            if ui.button(egui::RichText::new("⚙").size(16.0)).clicked() {
                // Ouvrir paramètres.
            }
        });
    }

    /// Affiche les onglets dans la ligne 2 du header.
    fn ui_header_tabs(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        use chrome_dimensions::*;

        let tabs_data: Vec<(usize, TabInfo)> = self
            .header_state
            .tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| (i, tab.clone()))
            .collect();

        // Utiliser un layout horizontal pour positionner les onglets
        ui.horizontal(|ui| {
            let available_rect = ui.max_rect();
            let tab_bar_top = available_rect.min.y;
            let mut current_x = available_rect.min.x;

            for (index, tab_info) in tabs_data {
                let is_active = index == self.header_state.active_tab_index;
                let tab_height = if is_active {
                    TAB_HEIGHT_ACTIVE
                } else {
                    TAB_HEIGHT_INACTIVE
                };
                
                // Position verticale : onglets inactifs légèrement décalés vers le bas
                let tab_y = if is_active {
                    tab_bar_top
                } else {
                    tab_bar_top + TAB_INACTIVE_OFFSET_Y
                };

                // Rectangle de l'onglet
                let tab_rect = egui::Rect::from_min_max(
                    egui::pos2(current_x, tab_y),
                    egui::pos2(current_x + TAB_WIDTH, tab_y + tab_height),
                );

                // Allouer l'espace UI pour l'onglet (pour les interactions)
                let _response = ui.allocate_space(tab_rect.size());
                
                // Dessiner l'onglet directement avec le painter et gérer les interactions
                let result = Self::ui_tab_static(ui, &tab_info, is_active, ctx, tab_rect);

                if result.activate {
                    self.header_state.active_tab_index = index;
                }
                if result.close {
                    self.close_tab(index);
                    // Sortir de la boucle car l'index a changé
                    break;
                }

                // Avancer pour le prochain onglet
                current_x += TAB_WIDTH;
            }
        });
    }

    /// Affiche un onglet statique (sans mutation d'état).
    fn ui_tab_static(
        ui: &mut egui::Ui,
        tab_info: &TabInfo,
        is_active: bool,
        ctx: &egui::Context,
        tab_rect: egui::Rect,
    ) -> TabClickResult {
        use chrome_dimensions::*;
        use chrome_shapes::*;

        let pixel_theme = PixelChromeTheme::new(ctx.style().visuals.dark_mode);
        let painter = ui.painter();

        // Couleurs selon l'état.
        let bg_color = if is_active {
            pixel_theme.tab_active_bg()
        } else {
            pixel_theme.tab_inactive_bg()
        };
        let text_color = if is_active {
            pixel_theme.tab_active_text()
        } else {
            pixel_theme.tab_inactive_text()
        };

        // Dessiner la forme de l'onglet.
        if is_active {
            let shapes = create_active_tab_shape(tab_rect, bg_color);
            for shape in shapes {
                painter.add(shape);
            }
        } else {
            let shape = create_inactive_tab_shape(tab_rect, bg_color);
            painter.add(shape);
            // Séparateur vertical à droite.
            painter.line_segment(
                [egui::pos2(tab_rect.max.x, tab_rect.min.y), egui::pos2(tab_rect.max.x, tab_rect.max.y)],
                egui::Stroke::new(1.0, pixel_theme.tab_separator()),
            );
        }

        // Zone de contenu (texte + bouton fermer).
        let content_rect = egui::Rect::from_min_max(
            egui::pos2(tab_rect.min.x + TAB_PADDING_X, tab_rect.min.y + TAB_PADDING_Y),
            egui::pos2(tab_rect.max.x - TAB_PADDING_X, tab_rect.max.y - TAB_PADDING_Y),
        );

        // Bouton fermer (aligné à droite).
        let close_button_size = CLOSE_BUTTON_SIZE;
        let close_button_rect = egui::Rect::from_min_max(
            egui::pos2(
                content_rect.max.x - close_button_size - CLOSE_BUTTON_PADDING,
                content_rect.center().y - close_button_size / 2.0,
            ),
            egui::pos2(
                content_rect.max.x - CLOSE_BUTTON_PADDING,
                content_rect.center().y + close_button_size / 2.0,
            ),
        );

        // Zone cliquable de l'onglet (excluant le bouton fermer).
        let clickable_rect = egui::Rect::from_min_max(
            tab_rect.min,
            egui::pos2(close_button_rect.min.x - 4.0, tab_rect.max.y),
        );

        let mut result = TabClickResult::default();

        // Gestion des clics.
        let response = ui.interact(clickable_rect, ui.id().with("tab"), egui::Sense::click());
        if response.clicked() {
            result.activate = true;
        }

        // Bouton fermer (zone cliquable élargie pour faciliter le clic).
        let close_id = ui.id().with("close").with(tab_info.title.clone());
        let close_response = ui.interact(
            close_button_rect.expand(4.0),
            close_id,
            egui::Sense::click(),
        );

        let close_hovered = close_response.hovered();
        let dark_mode = pixel_theme.dark_mode;
        use crate::pixel_theme::chrome_colors;
        let close_color = if close_hovered {
            if dark_mode {
                chrome_colors::CLOSE_BUTTON_HOVER_DARK
            } else {
                chrome_colors::CLOSE_BUTTON_HOVER_LIGHT
            }
        } else {
            if dark_mode {
                chrome_colors::CLOSE_BUTTON_DARK
            } else {
                chrome_colors::CLOSE_BUTTON_LIGHT
            }
        };

        if close_hovered {
            // Cercle de fond au survol.
            let hover_bg = if dark_mode {
                chrome_colors::CLOSE_BUTTON_BG_HOVER_DARK
            } else {
                chrome_colors::CLOSE_BUTTON_BG_HOVER_LIGHT
            };
            painter.circle_filled(close_button_rect.center(), close_button_size / 2.0, hover_bg);
        }

        // Dessiner la croix de fermeture.
        let cross_size = CLOSE_CROSS_SIZE;
        let cross_stroke = CLOSE_CROSS_STROKE;
        let center = close_button_rect.center();
        let half_size = cross_size / 2.0;

        painter.line_segment(
            [
                egui::pos2(center.x - half_size, center.y - half_size),
                egui::pos2(center.x + half_size, center.y + half_size),
            ],
            egui::Stroke::new(cross_stroke, close_color),
        );
        painter.line_segment(
            [
                egui::pos2(center.x - half_size, center.y + half_size),
                egui::pos2(center.x + half_size, center.y - half_size),
            ],
            egui::Stroke::new(cross_stroke, close_color),
        );

        if close_response.clicked() {
            result.close = true;
            return result;
        }

        // Texte de l'onglet (dessiné directement avec le painter).
        let text_rect = egui::Rect::from_min_max(
            content_rect.min,
            egui::pos2(close_button_rect.min.x - 8.0, content_rect.max.y),
        );
        
        // Utiliser painter.text pour dessiner le texte avec troncature automatique
        let font_id = egui::FontId::proportional(TAB_TEXT_SIZE);
        let available_width = text_rect.width();
        
        // Créer le texte avec troncature si nécessaire
        let text_to_display = {
            let full_text = &tab_info.title;
            // Estimer la largeur approximative du texte
            let estimated_width = full_text.len() as f32 * (TAB_TEXT_SIZE * 0.6); // Approximation
            if estimated_width > available_width {
                // Tronquer avec ellipses
                let max_chars = ((available_width / (TAB_TEXT_SIZE * 0.6)) as usize).saturating_sub(3);
                if max_chars > 0 && full_text.len() > max_chars {
                    format!("{}...", &full_text[..max_chars])
                } else {
                    full_text.clone()
                }
            } else {
                full_text.clone()
            }
        };
        
        // Dessiner le texte avec painter.text qui gère automatiquement le layout
        painter.text(
            egui::pos2(
                text_rect.min.x,
                text_rect.center().y,
            ),
            egui::Align2::LEFT_CENTER,
            text_to_display,
            font_id,
            text_color,
        );

        result
    }

    /// Ferme un onglet.
    fn close_tab(&mut self, index: usize) {
        if index >= self.header_state.tabs.len() {
            return;
        }

        // Ne pas fermer le dernier onglet (doit rester au moins HUB).
        if self.header_state.tabs.len() <= 1 {
            return;
        }

        // Si l'onglet fermé était actif, activer un autre onglet.
        let was_active = index == self.header_state.active_tab_index;
        
        // Supprimer le service associé si c'est un onglet service.
        if let Some(TabInfo { service_id: Some(service_id), .. }) = self.header_state.tabs.get(index) {
            self.open_services.retain(|s| s.id() != *service_id);
        }

        // Supprimer l'onglet.
        self.header_state.tabs.remove(index);

        // Réactiver un onglet si nécessaire.
        if was_active {
            let new_index = if index > 0 { index - 1 } else { 0 };
            self.header_state.active_tab_index = new_index.min(self.header_state.tabs.len() - 1);
        } else if index < self.header_state.active_tab_index {
            self.header_state.active_tab_index -= 1;
        }
    }

    /// Affiche le menu services dropdown.
    fn ui_menu_services_dropdown(&mut self, ctx: &egui::Context) {
        const LINE1_HEIGHT: f32 = 36.0;
        const LINE2_HEIGHT: f32 = 36.0;
        const HEADER_TOTAL_HEIGHT: f32 = LINE1_HEIGHT + LINE2_HEIGHT;

        let viewport_rect = ctx.viewport_rect();
        let menu_rect = egui::Rect::from_min_max(
            egui::pos2(0.0, HEADER_TOTAL_HEIGHT),
            egui::pos2(300.0, viewport_rect.max.y),
        );

        egui::Window::new("menu_services")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .fixed_rect(menu_rect)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Services disponibles");

                    // Filtres.
                    ui.horizontal(|ui| {
                        if ui.button("Tous").clicked() {
                            ctx.data_mut(|d| {
                                d.insert_temp(egui::Id::new("menu_filter_clicked"), true);
                            });
                            self.menu_filter = None;
                        }
                        for &category in &[CategoryId::Utilitaires, CategoryId::Loisirs, CategoryId::Productivite] {
                            if ui.button(format!("{:?}", category)).clicked() {
                                ctx.data_mut(|d| {
                                    d.insert_temp(egui::Id::new("menu_filter_clicked"), true);
                                });
                                self.menu_filter = Some(category);
                            }
                        }
                    });

                    ui.separator();

                    // Liste des services.
                    let catalog_copy = self.catalog.clone();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for service_id in &catalog_copy {
                            self.ui_service_card_in_menu(ui, *service_id);
                        }
                    });
                });
            });

        // Fermer le menu si clic en dehors (sauf si filtre vient d'être cliqué).
        let filter_clicked = ctx.data(|d| d.get_temp::<bool>(egui::Id::new("menu_filter_clicked")).unwrap_or(false));
        ctx.data_mut(|d| {
            d.insert_temp(egui::Id::new("menu_filter_clicked"), false);
        });

        if !filter_clicked {
            let menu_just_opened = ctx.data(|d| d.get_temp::<bool>(egui::Id::new("menu_just_opened")).unwrap_or(false));
            if menu_just_opened {
                ctx.data_mut(|d| {
                    d.insert_temp(egui::Id::new("menu_just_opened"), false);
                    d.insert_temp(egui::Id::new("menu_ready"), true);
                });
            } else {
                let menu_ready = ctx.data(|d| d.get_temp::<bool>(egui::Id::new("menu_ready")).unwrap_or(false));
                if menu_ready {
                    if ctx.input(|i| i.pointer.primary_clicked()) {
                        let click_pos = ctx.input(|i| i.pointer.interact_pos());
                        if let Some(pos) = click_pos {
                            if !menu_rect.contains(pos) {
                                self.header_state.menu_open = false;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Affiche une carte de service dans le menu.
    fn ui_service_card_in_menu(&mut self, ui: &mut egui::Ui, service_id: ServiceId) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(service_id.icon()).size(24.0));
            ui.label(service_id.as_str());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Ouvrir").clicked() {
                    self.open_service(service_id);
                    self.header_state.menu_open = false;
                }
            });
        });
        ui.separator();
    }

    /// Ouvre un service (crée un nouvel onglet).
    fn open_service(&mut self, service_id: ServiceId) {
        // Vérifier si le service est déjà ouvert.
        if let Some(index) = self.header_state.tabs.iter().position(|t| {
            if let Some(id) = t.service_id {
                id == service_id
            } else {
                false
            }
        }) {
            self.header_state.active_tab_index = index;
            return;
        }

        // Créer le service.
        let service: Box<dyn ServiceUi> = match service_id {
            ServiceId::Calculator => Box::new(CalculatorService::default()),
            ServiceId::Game => Box::new(GameService::default()),
            ServiceId::TextEditor => Box::new(TextEditorService::default()),
            ServiceId::Notes => Box::new(NotesService::default()),
        };

        // Ajouter l'onglet.
        self.header_state.tabs.push(TabInfo {
            service_id: Some(service_id),
            title: service.title().to_string(),
            tab_type: TabType::Service,
        });
        self.open_services.push(service);
        self.header_state.active_tab_index = self.header_state.tabs.len() - 1;
    }

    /// Ouvre l'onglet profil.
    fn open_profile_tab(&mut self) {
        // Vérifier si l'onglet profil existe déjà.
        if let Some(index) = self.header_state.tabs.iter().position(|t| t.tab_type == TabType::Profile) {
            self.header_state.active_tab_index = index;
            return;
        }

        // Créer l'onglet profil.
        self.header_state.tabs.push(TabInfo {
            service_id: None,
            title: "Profile".to_string(),
            tab_type: TabType::Profile,
        });
        self.header_state.active_tab_index = self.header_state.tabs.len() - 1;
    }

    /// Affiche le contenu principal selon l'onglet actif.
    fn ui_content(&mut self, ui: &mut egui::Ui) {
        if let Some(tab) = self.header_state.tabs.get(self.header_state.active_tab_index) {
            match tab.tab_type {
                TabType::Hub => self.ui_hub_content(ui),
                TabType::Service => {
                    if let Some(service_id) = tab.service_id {
                        self.ui_service_content(ui, service_id);
                    }
                }
                TabType::Profile => self.ui_profile_content(ui),
            }
        }
    }

    /// Affiche le contenu de l'onglet HUB.
    fn ui_hub_content(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Miyukini Central — Hub Services");
            ui.separator();

            match self.hub_view {
                HubView::Home => {
                    ui.label("Bienvenue sur Miyukini Central");
                    ui.label("Hub de gestion des Services");
                }
                HubView::Catalog => {
                    ui.heading("Catalogue");
                    let catalog_copy = self.catalog.clone();
                    for service_id in &catalog_copy {
                        ui.horizontal(|ui| {
                            ui.label(service_id.icon());
                            ui.label(service_id.as_str());
                            if ui.button("Ouvrir").clicked() {
                                self.open_service(*service_id);
                            }
                        });
                    }
                }
                HubView::MyServices => {
                    ui.heading("Mes Services");
                    for tab in &self.header_state.tabs {
                        if tab.tab_type == TabType::Service {
                            ui.label(&tab.title);
                        }
                    }
                }
                HubView::Settings => {
                    ui.heading("Paramètres");
                    ui.label("Configuration de l'interface");
                }
            }
        });
    }

    /// Affiche le contenu d'un service.
    fn ui_service_content(&mut self, ui: &mut egui::Ui, service_id: ServiceId) {
        if let Some(service) = self.open_services.iter_mut().find(|s| s.id() == service_id) {
            service.show(ui);
        }
    }

    /// Affiche le contenu de l'onglet profil.
    fn ui_profile_content(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Profil utilisateur");
            ui.label("Informations du profil");
        });
    }
}
