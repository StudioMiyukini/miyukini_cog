//! Organism Header Landing — barre d'accueil : Accueil (gauche), titre (centre), connexion + conf (droite).
//!
//! Conforme wireframe Page accueil JayFestival.

use crate::theme::JayFestivalTheme;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui::Ui;

/// Réponses des boutons du header landing : [Accueil, Connexion, Conf].
pub struct LandingHeaderResponses {
    /// Clic sur Accueil (déjà sur la page).
    pub accueil_clicked: bool,
    /// Clic sur Connexion.
    pub connexion_clicked: bool,
    /// Clic sur Conf (paramètres).
    pub conf_clicked: bool,
}

/// Dessine le header de la page d'accueil : Accueil (gauche, sélectionné), titre au centre, Connexion et Conf à droite.
pub fn header_landing_render(
    ui: &mut Ui,
    theme: &JayFestivalTheme,
    title: &str,
) -> LandingHeaderResponses {
    let mut accueil_clicked = false;
    let mut connexion_clicked = false;
    let mut conf_clicked = false;

    ui.horizontal(|ui| {
        let r_accueil = button(ui, theme, "Accueil", ButtonVariant::NavSelected, Default::default());
        if r_accueil.clicked() {
            accueil_clicked = true;
        }
        ui.add_space(theme.item_spacing() * 2.0);

        ui.with_layout(
            eframe::egui::Layout::top_down(eframe::egui::Align::Center).with_main_justify(true),
            |ui| {
                ui.add_space(theme.item_spacing());
                label(ui, theme, title, LabelLevel::Heading);
            },
        );

        ui.with_layout(
            eframe::egui::Layout::right_to_left(eframe::egui::Align::Center),
            |ui| {
                let r_conf = button(ui, theme, "conf", ButtonVariant::Ghost, Default::default());
                if r_conf.clicked() {
                    conf_clicked = true;
                }
                ui.add_space(theme.item_spacing());
                let r_connexion = button(ui, theme, "connexion", ButtonVariant::Outline, Default::default());
                if r_connexion.clicked() {
                    connexion_clicked = true;
                }
            },
        );
    });

    LandingHeaderResponses {
        accueil_clicked,
        connexion_clicked,
        conf_clicked,
    }
}
