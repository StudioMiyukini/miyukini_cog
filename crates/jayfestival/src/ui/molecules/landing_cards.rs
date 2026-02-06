//! Cartes pour la page d'accueil : carte événement (logo, nom, type, date, lieu) et carte news (image, article, type, description).

use crate::theme::JayFestivalTheme;
use eframe::egui;

/// Données d'une carte événement (landing).
pub struct EventCardLandingItem {
    /// Nom de l'événement.
    pub nom: String,
    /// Type d'événement (festival, salon, etc.).
    pub type_event: String,
    /// Date affichée.
    pub date: String,
    /// Lieu.
    pub lieu: String,
}

const EVENT_CARD_WIDTH: f32 = 128.0;
const EVENT_CARD_HEIGHT: f32 = 148.0;
/// Padding réduit pour les cartes landing (proportions compactes).
const LANDING_CARD_PADDING: f32 = 8.0;

/// Dessine une carte événement : zone logo (placeholder), nom, type, date, lieu.
pub fn event_card_landing_render(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    item: &EventCardLandingItem,
) -> egui::Response {
    let frame = egui::Frame::default()
        .fill(theme.section_card_background())
        .stroke(egui::Stroke::new(1.0, theme.section_border()))
        .corner_radius(egui::CornerRadius::same(theme.radius_small() as u8))
        .inner_margin(egui::Margin::same(LANDING_CARD_PADDING as i8));

    let inner = ui.allocate_ui(
        egui::vec2(EVENT_CARD_WIDTH, EVENT_CARD_HEIGHT),
        |ui| {
            frame.show(ui, |ui| {
                ui.set_min_width(EVENT_CARD_WIDTH - LANDING_CARD_PADDING * 2.0);
                ui.set_max_width(EVENT_CARD_WIDTH - LANDING_CARD_PADDING * 2.0);

                let logo_size = 36.0;
                let (logo_rect, _) = ui.allocate_exact_size(
                    egui::vec2(logo_size, logo_size),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(
                    logo_rect,
                    egui::CornerRadius::same(theme.radius_small() as u8),
                    theme.section_background(),
                );
                ui.painter().text(
                    logo_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Logo",
                    egui::FontId::new(theme.font_size_sm(), egui::FontFamily::Proportional),
                    theme.section_description(),
                );
                ui.allocate_rect(logo_rect, egui::Sense::hover());
                ui.add_space(theme.item_spacing());

                ui.label(
                    egui::RichText::new(&item.nom)
                        .color(theme.text_primary())
                        .size(theme.font_size_md()),
                );
                ui.label(
                    egui::RichText::new(&item.type_event)
                        .color(theme.section_description())
                        .size(theme.font_size_sm()),
                );
                ui.label(
                    egui::RichText::new(&item.date)
                        .color(theme.section_description())
                        .size(theme.font_size_sm()),
                );
                ui.label(
                    egui::RichText::new(&item.lieu)
                        .color(theme.section_description())
                        .size(theme.font_size_sm()),
                );
            });
        },
    );
    inner.response
}

/// Données d'une carte news (landing).
pub struct NewsCardLandingItem {
    /// Titre de l'article.
    pub titre: String,
    /// Type d'article.
    pub type_article: String,
    /// Courte description.
    pub description: String,
}

const NEWS_CARD_WIDTH: f32 = 160.0;
const NEWS_CARD_HEIGHT: f32 = 168.0;

/// Dessine une carte news : zone image/couleur (placeholder), article (titre), type, short description.
pub fn news_card_landing_render(
    theme: &JayFestivalTheme,
    ui: &mut egui::Ui,
    item: &NewsCardLandingItem,
) -> egui::Response {
    let frame = egui::Frame::default()
        .fill(theme.section_card_background())
        .stroke(egui::Stroke::new(1.0, theme.section_border()))
        .corner_radius(egui::CornerRadius::same(theme.radius_small() as u8))
        .inner_margin(egui::Margin::same(LANDING_CARD_PADDING as i8));

    let inner = ui.allocate_ui(
        egui::vec2(NEWS_CARD_WIDTH, NEWS_CARD_HEIGHT),
        |ui| {
            frame.show(ui, |ui| {
                ui.set_min_width(NEWS_CARD_WIDTH - LANDING_CARD_PADDING * 2.0);
                ui.set_max_width(NEWS_CARD_WIDTH - LANDING_CARD_PADDING * 2.0);

                let image_height = 56.0;
                let (image_rect, _) = ui.allocate_exact_size(
                    egui::vec2(NEWS_CARD_WIDTH - LANDING_CARD_PADDING * 2.0, image_height),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(
                    image_rect,
                    egui::CornerRadius::same(theme.radius_small() as u8),
                    theme.section_background(),
                );
                ui.painter().text(
                    image_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "Image/color",
                    egui::FontId::new(theme.font_size_sm(), egui::FontFamily::Proportional),
                    theme.section_description(),
                );
                ui.allocate_rect(image_rect, egui::Sense::hover());
                ui.add_space(theme.item_spacing());

                ui.label(
                    egui::RichText::new("Article")
                        .color(theme.section_description())
                        .size(theme.font_size_sm()),
                );
                ui.label(
                    egui::RichText::new(&item.titre)
                        .color(theme.text_primary())
                        .size(theme.font_size_md()),
                );
                ui.label(
                    egui::RichText::new(&item.type_article)
                        .color(theme.section_description())
                        .size(theme.font_size_sm()),
                );
                ui.label(
                    egui::RichText::new(&item.description)
                        .color(theme.section_description())
                        .size(theme.font_size_sm()),
                );
            });
        },
    );
    inner.response
}
