//! Service MiyukiniLoyalty — vitrine du système de récompenses et fidélisation.
//!
//! Interface graphique pour consulter les achievements, la boutique MIYU, l'inventaire,
//! les missions, et le système de gacha.

use crate::catalog::ServiceId;
use crate::loyalty::{
    Achievement, AchievementStatus, GachaSystem, ItemRarity, LoyaltyState, Mission, VirtualItem,
};
use crate::services::ServiceUi;
use eframe::egui;

/// Onglet actif dans l'interface Loyalty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoyaltyTab {
    /// Achievements (succès).
    Achievements,
    /// Boutique MIYU.
    Shop,
    /// Inventaire.
    Inventory,
    /// Missions.
    Missions,
    /// Gacha.
    Gacha,
}

/// Service MiyukiniLoyalty.
pub struct LoyaltyService {
    /// Onglet actif.
    active_tab: LoyaltyTab,
    /// Système de gacha.
    gacha_system: GachaSystem,
    /// Items récemment obtenus via gacha (affichage temporaire).
    recent_gacha_pulls: Vec<VirtualItem>,
}

impl LoyaltyService {
    /// Crée une nouvelle instance.
    pub fn new() -> Self {
        let mut gacha_system = GachaSystem::new();

        // Remplir le pool de gacha avec des items d'exemple
        gacha_system.available_items = Self::create_example_items();

        Self {
            active_tab: LoyaltyTab::Achievements,
            gacha_system,
            recent_gacha_pulls: Vec::new(),
        }
    }

    /// Crée des items d'exemple pour le gacha.
    fn create_example_items() -> Vec<VirtualItem> {
        vec![
            VirtualItem {
                id: "avatar_cat".to_string(),
                name: "Avatar Chat".to_string(),
                description: "Un adorable avatar de chat".to_string(),
                icon: "🐱".to_string(),
                item_type: crate::loyalty::ItemType::Cosmetic,
                rarity: ItemRarity::Common,
                source_service: None,
                price_miyu: Some(50),
                gacha_only: false,
            },
            VirtualItem {
                id: "avatar_dragon".to_string(),
                name: "Avatar Dragon".to_string(),
                description: "Avatar dragon majestueux".to_string(),
                icon: "🐉".to_string(),
                item_type: crate::loyalty::ItemType::Cosmetic,
                rarity: ItemRarity::Rare,
                source_service: None,
                price_miyu: None,
                gacha_only: true,
            },
            VirtualItem {
                id: "theme_dark_blue".to_string(),
                name: "Thème Bleu Nuit".to_string(),
                description: "Thème sombre aux tons bleutés".to_string(),
                icon: "🌙".to_string(),
                item_type: crate::loyalty::ItemType::Skin,
                rarity: ItemRarity::Epic,
                source_service: None,
                price_miyu: Some(500),
                gacha_only: false,
            },
            VirtualItem {
                id: "badge_pioneer".to_string(),
                name: "Badge Pionnier".to_string(),
                description: "Pour les premiers utilisateurs".to_string(),
                icon: "🏆".to_string(),
                item_type: crate::loyalty::ItemType::Badge,
                rarity: ItemRarity::Legendary,
                source_service: None,
                price_miyu: None,
                gacha_only: true,
            },
            VirtualItem {
                id: "particle_sparkle".to_string(),
                name: "Particules Étincelles".to_string(),
                description: "Effet de particules scintillantes".to_string(),
                icon: "✨".to_string(),
                item_type: crate::loyalty::ItemType::ParticleEffect,
                rarity: ItemRarity::Rare,
                source_service: None,
                price_miyu: Some(200),
                gacha_only: false,
            },
        ]
    }

    /// Affiche la barre d'onglets.
    fn show_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.active_tab, LoyaltyTab::Achievements, "🏆 Achievements");
            ui.selectable_value(&mut self.active_tab, LoyaltyTab::Shop, "🛒 Boutique");
            ui.selectable_value(&mut self.active_tab, LoyaltyTab::Inventory, "🎒 Inventaire");
            ui.selectable_value(&mut self.active_tab, LoyaltyTab::Missions, "🎯 Missions");
            ui.selectable_value(&mut self.active_tab, LoyaltyTab::Gacha, "🎲 Gacha");
        });
        ui.separator();
    }

    /// Affiche l'onglet Achievements.
    fn show_achievements_tab(&self, ui: &mut egui::Ui, loyalty_state: &mut LoyaltyState) {
        ui.heading("🏆 Achievements");
        ui.add_space(10.0);

        if loyalty_state.achievements.is_empty() {
            ui.label("Aucun achievement pour le moment. Explorez les services pour en débloquer !");
            return;
        }

        let list: Vec<(String, Achievement)> = loyalty_state
            .achievements
            .iter()
            .map(|(id, a)| (id.clone(), a.clone()))
            .collect();
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (id, achievement) in &list {
                self.show_achievement_card(ui, id, achievement, loyalty_state);
            }
        });
    }

    /// Affiche une carte d'achievement.
    fn show_achievement_card(
        &self,
        ui: &mut egui::Ui,
        id: &str,
        achievement: &Achievement,
        loyalty_state: &mut LoyaltyState,
    ) {
        let bg_color = match achievement.status {
            AchievementStatus::Locked => egui::Color32::from_rgb(60, 60, 60),
            AchievementStatus::Unlocked => egui::Color32::from_rgb(50, 100, 50),
            AchievementStatus::Claimed => egui::Color32::from_rgb(40, 80, 120),
        };

        egui::Frame::NONE
            .fill(bg_color)
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&achievement.icon).size(32.0));
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&achievement.name).strong());
                        ui.label(&achievement.description);
                        ui.label(format!("💎 {} MIYU", achievement.reward_miyu));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        match achievement.status {
                            AchievementStatus::Locked => {
                                ui.label("🔒 Verrouillé");
                            }
                            AchievementStatus::Unlocked => {
                                if ui.button("✅ Réclamer").clicked() {
                                    loyalty_state.claim_achievement(id);
                                }
                            }
                            AchievementStatus::Claimed => {
                                ui.label("✓ Réclamé");
                            }
                        }
                    });
                });
            });
        ui.add_space(8.0);
    }

    /// Affiche l'onglet Boutique.
    fn show_shop_tab(&self, ui: &mut egui::Ui, loyalty_state: &mut LoyaltyState) {
        ui.heading("🛒 Boutique MIYU");
        ui.label(format!("Solde : 💎 {} MIYU", loyalty_state.miyu_points.get()));
        ui.add_space(10.0);

        let purchasable_items: Vec<_> = self
            .gacha_system
            .available_items
            .iter()
            .filter(|item| item.price_miyu.is_some())
            .collect();

        if purchasable_items.is_empty() {
            ui.label("La boutique est vide pour le moment.");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for item in purchasable_items {
                self.show_shop_item_card(ui, item, loyalty_state);
            }
        });
    }

    /// Affiche une carte d'item dans la boutique.
    fn show_shop_item_card(
        &self,
        ui: &mut egui::Ui,
        item: &VirtualItem,
        loyalty_state: &mut LoyaltyState,
    ) {
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(40, 40, 50))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&item.icon).size(32.0));
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&item.name)
                                .strong()
                                .color(item.rarity.color()),
                        );
                        ui.label(&item.description);
                        ui.label(format!("Rareté : {}", item.rarity.label()));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(price) = item.price_miyu {
                            if ui.button(format!("💎 {} MIYU", price)).clicked() {
                                if loyalty_state.spend_miyu(price, format!("Achat : {}", item.name))
                                {
                                    loyalty_state.add_item(item.id.clone(), 1);
                                }
                            }
                        }
                    });
                });
            });
        ui.add_space(8.0);
    }

    /// Affiche l'onglet Inventaire.
    fn show_inventory_tab(&self, ui: &mut egui::Ui, loyalty_state: &LoyaltyState) {
        ui.heading("🎒 Inventaire");
        ui.add_space(10.0);

        if loyalty_state.items.is_empty() {
            ui.label("Votre inventaire est vide. Achetez des items ou tentez le gacha !");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (item_id, quantity) in &loyalty_state.items {
                if let Some(item) = self
                    .gacha_system
                    .available_items
                    .iter()
                    .find(|i| &i.id == item_id)
                {
                    self.show_inventory_item_card(ui, item, *quantity);
                }
            }
        });
    }

    /// Affiche une carte d'item dans l'inventaire.
    fn show_inventory_item_card(&self, ui: &mut egui::Ui, item: &VirtualItem, quantity: u32) {
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(40, 40, 50))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&item.icon).size(32.0));
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&item.name)
                                .strong()
                                .color(item.rarity.color()),
                        );
                        ui.label(&item.description);
                        ui.label(format!("Quantité : {}", quantity));
                    });
                });
            });
        ui.add_space(8.0);
    }

    /// Affiche l'onglet Missions.
    fn show_missions_tab(&self, ui: &mut egui::Ui, loyalty_state: &LoyaltyState) {
        ui.heading("🎯 Missions");
        ui.add_space(10.0);

        if loyalty_state.missions.is_empty() {
            ui.label("Aucune mission active. Les services créeront des missions pour vous !");
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (_id, mission) in &loyalty_state.missions {
                self.show_mission_card(ui, mission);
            }
        });
    }

    /// Affiche une carte de mission.
    fn show_mission_card(&self, ui: &mut egui::Ui, mission: &Mission) {
        let bg_color = if mission.completed {
            egui::Color32::from_rgb(40, 80, 40)
        } else {
            egui::Color32::from_rgb(50, 50, 60)
        };

        egui::Frame::NONE
            .fill(bg_color)
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(&mission.name).strong());
                    ui.label(&mission.description);

                    let progress_fraction = mission.progress as f32 / mission.goal as f32;
                    let progress_bar = egui::ProgressBar::new(progress_fraction)
                        .text(format!("{}/{}", mission.progress, mission.goal));
                    ui.add(progress_bar);

                    ui.horizontal(|ui| {
                        ui.label(format!("💎 {} MIYU", mission.reward_miyu));
                        if mission.completed {
                            ui.label("✓ Complétée");
                        }
                    });
                });
            });
        ui.add_space(8.0);
    }

    /// Affiche l'onglet Gacha.
    fn show_gacha_tab(&mut self, ui: &mut egui::Ui, loyalty_state: &mut LoyaltyState) {
        ui.heading("🎲 Gacha");
        ui.label(format!("Solde : 💎 {} MIYU", loyalty_state.miyu_points.get()));
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui
                .button(format!(
                    "🎲 Tirage simple (💎 {} MIYU)",
                    self.gacha_system.single_pull_cost
                ))
                .clicked()
            {
                if loyalty_state.spend_miyu(
                    self.gacha_system.single_pull_cost,
                    "Gacha : tirage simple".to_string(),
                ) {
                    if let Some(item) = self.gacha_system.single_pull() {
                        loyalty_state.add_item(item.id.clone(), 1);
                        self.recent_gacha_pulls = vec![item];
                    }
                }
            }

            if ui
                .button(format!(
                    "🎲🎲 Tirage x10 (💎 {} MIYU)",
                    self.gacha_system.multi_pull_cost
                ))
                .clicked()
            {
                if loyalty_state.spend_miyu(
                    self.gacha_system.multi_pull_cost,
                    "Gacha : tirage x10".to_string(),
                ) {
                    let items = self.gacha_system.multi_pull();
                    for item in &items {
                        loyalty_state.add_item(item.id.clone(), 1);
                    }
                    self.recent_gacha_pulls = items;
                }
            }
        });

        ui.add_space(20.0);

        if !self.recent_gacha_pulls.is_empty() {
            ui.heading("🎁 Résultats du tirage :");
            ui.add_space(10.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                for item in &self.recent_gacha_pulls {
                    self.show_gacha_result_card(ui, item);
                }
            });
        } else {
            ui.label("Aucun tirage récent. Tentez votre chance !");
        }
    }

    /// Affiche une carte de résultat de gacha.
    fn show_gacha_result_card(&self, ui: &mut egui::Ui, item: &VirtualItem) {
        egui::Frame::NONE
            .fill(item.rarity.color().linear_multiply(0.3))
            .stroke(egui::Stroke::new(2.0, item.rarity.color()))
            .corner_radius(egui::CornerRadius::same(8))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&item.icon).size(40.0));
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&item.name)
                                .strong()
                                .size(18.0)
                                .color(item.rarity.color()),
                        );
                        ui.label(format!("✨ {} ✨", item.rarity.label()));
                    });
                });
            });
        ui.add_space(8.0);
    }
}

impl Default for LoyaltyService {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUi for LoyaltyService {
    fn id(&self) -> ServiceId {
        ServiceId::MiyukiniLoyalty
    }

    fn title(&self) -> &'static str {
        "Miyukini Récompenses"
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        // NOTE : Pour l'instant on utilise un état local temporaire
        // Dans une vraie implémentation, cet état serait partagé avec l'app principale
        let mut loyalty_state = LoyaltyState::new();

        // Ajouter des points et achievements d'exemple pour la démo
        loyalty_state.miyu_points.add(1000);

        self.show_tabs(ui);
        ui.add_space(10.0);

        match self.active_tab {
            LoyaltyTab::Achievements => self.show_achievements_tab(ui, &mut loyalty_state),
            LoyaltyTab::Shop => self.show_shop_tab(ui, &mut loyalty_state),
            LoyaltyTab::Inventory => self.show_inventory_tab(ui, &loyalty_state),
            LoyaltyTab::Missions => self.show_missions_tab(ui, &loyalty_state),
            LoyaltyTab::Gacha => self.show_gacha_tab(ui, &mut loyalty_state),
        }
    }
}
