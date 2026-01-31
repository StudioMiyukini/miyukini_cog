//! Application principale du Hub Miyukini Central (MVP).

use crate::catalog::{mock_catalog, CategoryId, ServiceId, ServiceMeta};
use crate::services::{CalculatorService, GameService, NotesService, ServiceUi, TextEditorService};
use crate::theme;
use eframe::egui;

/// Vue courante du Hub (navigation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum View {
    /// Accueil / tableau de bord.
    #[default]
    Home,
    /// Catalogue des Services.
    Catalogue,
    /// Mes Services (déjà activés).
    MyServices,
    /// Fiche détail d'un Service (sélectionné).
    ServiceDetail(ServiceId),
    /// Paramètres du Hub.
    Settings,
}

/// État de l'application Hub (MVP).
pub struct MiyukiniCentralApp {
    /// Vue courante.
    pub view: View,
    /// Service actuellement ouvert (affiché dans le panneau central).
    pub open_service: Option<ServiceId>,
    /// Services "activés" (ajoutés à Mes Services) — pour le MVP, on les ajoute au premier Ouvrir.
    pub my_services: Vec<ServiceId>,
    /// Service sélectionné pour la fiche détail (catalogue ou mes services).
    pub selected_service: Option<ServiceId>,
    /// Filtre de catégorie (catalogue).
    pub filter_category: Option<CategoryId>,
    /// Recherche texte (catalogue).
    pub search_query: String,
    /// Thème : true = sombre.
    pub dark_theme: bool,
    /// Instances des Services (état persistant de chaque démo).
    /// Service démo : Calculatrice.
    pub calculator: CalculatorService,
    /// Service démo : Jeu (clics rapides).
    pub game: GameService,
    /// Service démo : Traitement de texte.
    pub text_editor: TextEditorService,
    /// Service démo : Notes rapides.
    pub notes: NotesService,
}

impl Default for MiyukiniCentralApp {
    fn default() -> Self {
        Self {
            view: View::Home,
            open_service: None,
            my_services: Vec::new(),
            selected_service: None,
            filter_category: None,
            search_query: String::new(),
            dark_theme: true,
            calculator: CalculatorService::default(),
            game: GameService::default(),
            text_editor: TextEditorService::default(),
            notes: NotesService::default(),
        }
    }
}

impl MiyukiniCentralApp {
    /// Crée l'app (pour eframe).
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_store_theme(&cc.egui_ctx, true);
        Self::default()
    }

    /// Retourne le catalogue (mock).
    fn catalog(&self) -> Vec<ServiceMeta> {
        mock_catalog()
    }

    /// Filtre le catalogue par catégorie et recherche.
    fn filtered_catalog(&self) -> Vec<ServiceMeta> {
        self.catalog()
            .into_iter()
            .filter(|s| {
                let cat_ok = self
                    .filter_category
                    .map(|c| s.category == c)
                    .unwrap_or(true);
                let search_ok = self.search_query.is_empty()
                    || s.name.to_lowercase().contains(&self.search_query.to_lowercase())
                    || s.description.to_lowercase().contains(&self.search_query.to_lowercase());
                cat_ok && search_ok
            })
            .collect()
    }

    /// Vérifie si un Service est dans "Mes Services" (réservé pour usage futur).
    #[allow(dead_code)]
    fn is_in_my_services(&self, id: ServiceId) -> bool {
        self.my_services.contains(&id)
    }

    /// Ajoute un Service à Mes Services (au premier Ouvrir).
    fn add_to_my_services(&mut self, id: ServiceId) {
        if !self.my_services.contains(&id) {
            self.my_services.push(id);
        }
    }

    /// Ouvre un Service (affiche son UI dans le panneau central).
    fn open_service(&mut self, id: ServiceId) {
        self.add_to_my_services(id);
        self.open_service = Some(id);
    }

    /// Ferme le Service ouvert.
    fn close_service(&mut self) {
        self.open_service = None;
    }

    /// Dessine la barre latérale (navigation) — style store.
    fn ui_sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("hub_sidebar")
            .resizable(false)
            .default_width(220.0)
            .exact_width(220.0)
            .show(ctx, |ui| {
                ui.add_space(12.0);
                ui.heading(egui::RichText::new("Miyukini Central").size(18.0));
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(12.0);

                let nav_items = [
                    (View::Home, "🏠", "Accueil"),
                    (View::Catalogue, "📦", "Catalogue"),
                    (View::MyServices, "📌", "Mes Services"),
                ];
                for (v, icon, label) in nav_items {
                    let selected = self.view == v && !matches!(self.view, View::ServiceDetail(_) | View::Settings);
                    let r = ui.selectable_label(selected, format!("{}  {}", icon, label));
                    if r.clicked() {
                        self.view = v;
                        self.selected_service = None;
                    }
                    ui.add_space(4.0);
                }
                ui.add_space(16.0);
                if ui.selectable_label(self.view == View::Settings, "⚙️  Paramètres").clicked() {
                    self.view = View::Settings;
                }
                ui.add_space(24.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Hub MVP").color(ui.visuals().weak_text_color()));
            });
    }

    /// Dessine le panneau central (contenu selon la vue ou le Service ouvert).
    fn ui_central_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Si un Service est ouvert, afficher son UI et un bouton "Fermer".
            if let Some(id) = self.open_service {
                egui::Frame::group(ui.style())
                    .inner_margin(12.0)
                    .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
                    .fill(theme::card_bg_color(self.dark_theme))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(id.icon()).size(22.0));
                            ui.heading(self.service_title(id));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("✕  Fermer").clicked() {
                                    self.close_service();
                                }
                            });
                        });
                    });
                ui.add_space(12.0);
                self.show_service_ui(ui, id);
                return;
            }

            // Sinon, afficher la vue courante.
            match self.view {
                View::Home => self.ui_home(ui),
                View::Catalogue => self.ui_catalogue(ui),
                View::MyServices => self.ui_my_services(ui),
                View::ServiceDetail(id) => self.ui_service_detail(ui, id),
                View::Settings => self.ui_settings(ui, frame),
            }
        });
    }

    fn service_title(&self, id: ServiceId) -> &'static str {
        match id {
            ServiceId::Calculator => self.calculator.title(),
            ServiceId::Game => self.game.title(),
            ServiceId::TextEditor => self.text_editor.title(),
            ServiceId::Notes => self.notes.title(),
        }
    }

    fn show_service_ui(&mut self, ui: &mut egui::Ui, id: ServiceId) {
        match id {
            ServiceId::Calculator => self.calculator.show(ui),
            ServiceId::Game => self.game.show(ui),
            ServiceId::TextEditor => self.text_editor.show(ui),
            ServiceId::Notes => self.notes.show(ui),
        }
    }

    /// Dessine une carte Service (style store) — catalogue ou accueil.
    fn ui_service_card(
        &mut self,
        ui: &mut egui::Ui,
        meta: &ServiceMeta,
        compact: bool,
        width: f32,
        min_height: f32,
    ) {
        let desc_trunc = if meta.description.len() > 48 {
            format!("{}…", meta.description.chars().take(48).collect::<String>())
        } else {
            meta.description.clone()
        };
        let frame = egui::Frame::new()
            .inner_margin(12.0)
            .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
            .fill(theme::card_bg_color(self.dark_theme))
            .stroke(ui.visuals().widgets.noninteractive.bg_stroke);
        frame.show(ui, |ui| {
            ui.set_min_width(width);
            ui.set_min_height(min_height);
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(meta.id.icon()).size(if compact { 28.0 } else { 40.0 }));
                });
                ui.add_space(6.0);
                ui.label(egui::RichText::new(&meta.name).size(if compact { 13.0 } else { 15.0 }));
                if !compact {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(desc_trunc).size(12.0).color(ui.visuals().weak_text_color()));
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.small_button("Détails").clicked() {
                            self.view = View::ServiceDetail(meta.id);
                            self.selected_service = Some(meta.id);
                        }
                        if ui.button("Ouvrir").clicked() {
                            self.open_service(meta.id);
                        }
                    });
                } else if ui.button("Ouvrir").clicked() {
                    self.open_service(meta.id);
                }
            });
        });
    }

    fn ui_home(&mut self, ui: &mut egui::Ui) {
        // Hero / bannière d'accueil
        egui::Frame::group(ui.style())
            .inner_margin(24.0)
            .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
            .fill(theme::card_bg_color(self.dark_theme))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Bienvenue sur Miyukini Central").size(22.0));
                    ui.add_space(4.0);
                    ui.label("Hub de gestion des Services — découvrez, activez et lancez vos Services.");
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        if ui.button("📦  Voir le catalogue").clicked() {
                            self.view = View::Catalogue;
                        }
                        ui.add_space(12.0);
                        if ui.button("📌  Mes Services").clicked() {
                            self.view = View::MyServices;
                        }
                    });
                    ui.add_space(8.0);
                });
            });
        ui.add_space(24.0);

        if !self.my_services.is_empty() {
            ui.label(egui::RichText::new("Services récents").size(16.0));
            ui.add_space(12.0);
            let recent: Vec<ServiceId> = self.my_services.iter().take(6).copied().collect();
            let card_w = 160.0;
            let n_cols = (ui.available_width() / card_w).floor().max(1.0) as usize;
            for chunk in recent.chunks(n_cols) {
                ui.horizontal(|ui| {
                    for id in chunk {
                        let meta = self.catalog().into_iter().find(|m| m.id == *id);
                        if let Some(meta) = meta {
                            self.ui_service_card(ui, &meta, true, card_w, 100.0);
                        }
                    }
                });
                ui.add_space(8.0);
            }
        }
    }

    fn ui_catalogue(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Catalogue des Services").size(20.0));
        ui.add_space(12.0);

        // Barre de recherche (style store)
        egui::Frame::group(ui.style())
            .inner_margin(10.0)
            .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.search_query));
                });
            });
        ui.add_space(12.0);

        // Pills catégories
        ui.horizontal(|ui| {
            ui.label("Catégorie :");
            ui.add_space(8.0);
            if ui.selectable_label(self.filter_category.is_none(), "Toutes").clicked() {
                self.filter_category = None;
            }
            for cat in [CategoryId::Utilitaires, CategoryId::Loisirs, CategoryId::Productivite] {
                if ui
                    .selectable_label(self.filter_category == Some(cat), format!("{} {}", cat.icon(), cat.label()))
                    .clicked()
                {
                    self.filter_category = Some(cat);
                }
            }
        });
        ui.add_space(20.0);

        // Grille de cartes
        let catalog = self.filtered_catalog();
        let card_width = 220.0;
        let card_height = 180.0;
        let n_cols = (ui.available_width() / card_width).floor().max(1.0) as usize;
        for chunk in catalog.chunks(n_cols) {
            ui.horizontal(|ui| {
                for meta in chunk {
                    ui.add_space(8.0);
                    self.ui_service_card(ui, meta, false, card_width, card_height);
                }
            });
            ui.add_space(12.0);
        }
    }

    fn ui_my_services(&mut self, ui: &mut egui::Ui) {
        ui.heading(egui::RichText::new("Mes Services").size(20.0));
        ui.add_space(12.0);
        if self.my_services.is_empty() {
            egui::Frame::group(ui.style())
                .inner_margin(24.0)
                .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
                .fill(theme::card_bg_color(self.dark_theme))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("Aucun Service activé").size(16.0));
                        ui.add_space(8.0);
                        ui.label("Parcourez le Catalogue et cliquez sur « Ouvrir » pour en ajouter.");
                        ui.add_space(16.0);
                        if ui.button("📦  Voir le catalogue").clicked() {
                            self.view = View::Catalogue;
                        }
                    });
                });
            return;
        }
        let catalog = self.catalog();
        let card_width = 220.0;
        let card_height = 180.0;
        let n_cols = (ui.available_width() / card_width).floor().max(1.0) as usize;
        let my_meta: Vec<ServiceMeta> = self
            .my_services
            .iter()
            .filter_map(|id| catalog.iter().find(|m| m.id == *id).cloned())
            .collect();
        for chunk in my_meta.chunks(n_cols) {
            ui.horizontal(|ui| {
                for meta in chunk {
                    ui.add_space(8.0);
                    self.ui_service_card(ui, meta, false, card_width, card_height);
                }
            });
            ui.add_space(12.0);
        }
    }

    fn ui_service_detail(&mut self, ui: &mut egui::Ui, id: ServiceId) {
        let catalog = self.catalog();
        let meta = catalog.iter().find(|m| m.id == id);
        if let Some(meta) = meta {
            egui::Frame::group(ui.style())
                .inner_margin(24.0)
                .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
                .fill(theme::card_bg_color(self.dark_theme))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(meta.id.icon()).size(48.0));
                        ui.vertical(|ui| {
                            ui.heading(egui::RichText::new(&meta.name).size(22.0));
                            ui.add_space(4.0);
                            ui.label(&meta.description);
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new(format!("{}  {} · v{}", meta.category.icon(), meta.category.label(), meta.version))
                                .color(ui.visuals().weak_text_color()));
                        });
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        if ui.button("Ouvrir ce Service").clicked() {
                            self.open_service(meta.id);
                        }
                        if ui.small_button("← Catalogue").clicked() {
                            self.view = View::Catalogue;
                            self.selected_service = None;
                        }
                        if ui.small_button("← Mes Services").clicked() {
                            self.view = View::MyServices;
                            self.selected_service = None;
                        }
                    });
                });
        } else {
            ui.label("Service introuvable.");
            if ui.button("Retour").clicked() {
                self.view = View::Catalogue;
                self.selected_service = None;
            }
        }
    }

    fn ui_settings(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading(egui::RichText::new("Paramètres").size(20.0));
        ui.add_space(12.0);
        egui::Frame::group(ui.style())
            .inner_margin(16.0)
            .corner_radius(egui::CornerRadius::same(theme::card_corner_radius()))
            .fill(theme::card_bg_color(self.dark_theme))
            .show(ui, |ui| {
                if ui.checkbox(&mut self.dark_theme, "Thème sombre").changed() {
                    theme::apply_store_theme(ui.ctx(), self.dark_theme);
                }
            });
        ui.add_space(16.0);
        ui.label("Miyukini Central — MVP Hub");
        ui.label("Version démo — Services factices (Calculatrice, Jeu, Traitement de texte, Notes).");
    }
}

impl eframe::App for MiyukiniCentralApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui_sidebar(ctx);
        self.ui_central_panel(ctx, frame);
    }
}
