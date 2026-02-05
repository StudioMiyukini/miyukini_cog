//! Organism CTASection — section d'appels à l'action avec CTACards.
//!
//! Conforme [Specification UI] § 2. Dépend de la molecule CTACard.

use crate::theme::JayFestivalTheme;
use crate::ui::molecules::cta_card_render;
use eframe::egui::{Response, Ui};

/// Élément CTA (titre, description optionnelle, libellé bouton).
#[derive(Clone)]
pub struct CtaCardItem {
    /// Titre de la carte CTA.
    pub title: String,
    /// Description optionnelle sous le titre.
    pub description: Option<String>,
    /// Libellé du bouton d'action.
    pub button_label: String,
}

/// @id: organism_cta_section
/// @do: define_cta_section_component
/// @layer: ui
/// @human: Section regroupant un ou plusieurs appels à l'action (CTACard).

/// @id: cta_section_render
/// @do: render_cta_section_with_cards
/// @layer: ui
/// Affiche une section avec une ou plusieurs CTACards ; retourne les réponses des boutons (même ordre que items).
pub fn cta_section_render(
    theme: &JayFestivalTheme,
    ui: &mut Ui,
    items: &[CtaCardItem],
) -> Vec<Response> {
    let mut responses = Vec::with_capacity(items.len());
    ui.horizontal_wrapped(|ui| {
        for item in items {
            let r = cta_card_render(
                theme,
                ui,
                &item.title,
                item.description.as_deref(),
                &item.button_label,
            );
            responses.push(r);
        }
    });
    responses
}
