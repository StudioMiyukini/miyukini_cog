//! Organism FeaturesGrid — grille de FeatureCards (Specification UI § 2).
//!
//! Dépend de la molecule FeatureCard.

use crate::theme::JayFestivalTheme;
use crate::ui::molecules::feature_card_render;
use crate::ui::molecules::FeatureCardVariant;
use eframe::egui::Ui;

/// Élément de grille pour FeatureCard (titre, description, icône, variante).
#[derive(Clone)]
pub struct FeatureCardItem {
    /// Titre de la carte.
    pub title: String,
    /// Description courte.
    pub description: String,
    /// Caractère icône (emoji ou unicode).
    pub icon: String,
    /// Variante visuelle (couleur accent).
    pub variant: FeatureCardVariant,
}

/// @id: organism_features_grid
/// @do: define_features_grid_component
/// @layer: ui
/// @human: Grille de cartes fonctionnalités (FeatureCard).

/// @id: features_grid_render
/// @do: render_grid_of_feature_cards
/// @layer: ui
/// Affiche une grille responsive de FeatureCards (colonnes selon largeur disponible).
pub fn features_grid_render(
    theme: &JayFestivalTheme,
    ui: &mut Ui,
    items: &[FeatureCardItem],
) {
    let columns = (ui.available_width() / 280.0).max(1.0) as usize;
    let columns = columns.min(items.len()).max(1);

    for chunk in items.chunks(columns) {
        ui.horizontal(|ui| {
            for item in chunk {
                ui.vertical(|ui| {
                    feature_card_render(
                        theme,
                        ui,
                        &item.title,
                        &item.description,
                        &item.icon,
                        item.variant,
                    );
                });
            }
        });
    }
}
