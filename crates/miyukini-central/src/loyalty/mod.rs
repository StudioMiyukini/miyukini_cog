//! Système de gamification et fidélisation MiyukiniLoyalty.
//!
//! Gère les points MIYU (monnaie virtuelle non convertible), achievements,
//! récompenses, objets virtuels, missions, et système de gacha.
//!
//! @id: miyukini_loyalty_module
//! @do: provide_gamification_and_loyalty_system
//! @role: domain
//! @layer: application
//! @human: Système de récompenses et gamification intégré au Central.

use crate::catalog::ServiceId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Monnaie virtuelle MIYU - points de fidélité non convertibles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MiyuPoints(pub u64);

impl MiyuPoints {
    /// Crée une nouvelle instance avec 0 points.
    pub fn new() -> Self {
        Self(0)
    }

    /// Ajoute des points.
    pub fn add(&mut self, amount: u64) {
        self.0 = self.0.saturating_add(amount);
    }

    /// Retire des points (retourne false si insuffisant).
    pub fn spend(&mut self, amount: u64) -> bool {
        if self.0 >= amount {
            self.0 -= amount;
            true
        } else {
            false
        }
    }

    /// Retourne le nombre de points.
    pub fn get(&self) -> u64 {
        self.0
    }
}

impl Default for MiyuPoints {
    fn default() -> Self {
        Self::new()
    }
}

/// Type d'achievement.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementType {
    /// Premier lancement du Central.
    FirstLaunch,
    /// Première connexion.
    FirstLogin,
    /// Utilisation quotidienne (streak).
    DailyStreak {
        /// Nombre de jours consécutifs.
        days: u32,
    },
    /// Utilisation d'un service.
    ServiceUsage {
        /// Service concerné.
        service: ServiceId,
        /// Nombre d'utilisations.
        count: u32,
    },
    /// Temps passé sur un service (minutes).
    ServiceTimeSpent {
        /// Service concerné.
        service: ServiceId,
        /// Minutes passées.
        minutes: u32,
    },
    /// Achievement personnalisé d'un service.
    ServiceCustom {
        /// Service source.
        service: ServiceId,
        /// Identifiant de l'achievement côté service.
        achievement_id: String,
    },
    /// Collection d'objets.
    CollectionComplete {
        /// Nom de la collection.
        collection_name: String,
    },
    /// Participation à un événement.
    EventParticipation {
        /// Identifiant de l'événement.
        event_id: String,
    },
}

/// Statut d'un achievement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementStatus {
    /// Pas encore débloqué.
    Locked,
    /// Débloqué mais pas encore réclamé.
    Unlocked,
    /// Réclamé (récompense obtenue).
    Claimed,
}

/// Achievement (succès) avec ses métadonnées.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Type d'achievement.
    pub achievement_type: AchievementType,
    /// Nom affiché.
    pub name: String,
    /// Description.
    pub description: String,
    /// Icône (emoji ou path).
    pub icon: String,
    /// Récompense en points MIYU.
    pub reward_miyu: u64,
    /// Statut actuel.
    pub status: AchievementStatus,
    /// Timestamp de déblocage (None si pas encore débloqué).
    pub unlocked_at: Option<f64>,
    /// Timestamp de réclamation (None si pas encore réclamé).
    pub claimed_at: Option<f64>,
    /// Service source (None pour achievements globaux).
    pub source_service: Option<ServiceId>,
}

/// Type d'item virtuel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    /// Cosmétique (avatar, cadre, etc.).
    Cosmetic,
    /// Skin (thème visuel).
    Skin,
    /// Badge.
    Badge,
    /// Effet de particules.
    ParticleEffect,
    /// Objet de collection.
    Collectible,
}

/// Rareté d'un item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemRarity {
    /// Commun.
    Common,
    /// Rare.
    Rare,
    /// Épique.
    Epic,
    /// Légendaire.
    Legendary,
}

impl ItemRarity {
    /// Couleur associée à la rareté.
    pub fn color(&self) -> egui::Color32 {
        match self {
            ItemRarity::Common => egui::Color32::from_rgb(200, 200, 200),
            ItemRarity::Rare => egui::Color32::from_rgb(100, 150, 255),
            ItemRarity::Epic => egui::Color32::from_rgb(200, 100, 255),
            ItemRarity::Legendary => egui::Color32::from_rgb(255, 200, 50),
        }
    }

    /// Label affiché.
    pub fn label(&self) -> &'static str {
        match self {
            ItemRarity::Common => "Commun",
            ItemRarity::Rare => "Rare",
            ItemRarity::Epic => "Épique",
            ItemRarity::Legendary => "Légendaire",
        }
    }
}

/// Item virtuel (objet de collection, cosmétique, skin).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualItem {
    /// ID unique de l'item.
    pub id: String,
    /// Nom affiché.
    pub name: String,
    /// Description.
    pub description: String,
    /// Icône (emoji ou path).
    pub icon: String,
    /// Type d'item.
    pub item_type: ItemType,
    /// Rareté.
    pub rarity: ItemRarity,
    /// Service source (None pour items globaux).
    pub source_service: Option<ServiceId>,
    /// Prix en MIYU (None si pas achetable directement).
    pub price_miyu: Option<u64>,
    /// Obtenu via gacha uniquement.
    pub gacha_only: bool,
}

/// Mission avec progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    /// ID unique de la mission.
    pub id: String,
    /// Nom affiché.
    pub name: String,
    /// Description.
    pub description: String,
    /// Service source (None pour missions globales).
    pub source_service: Option<ServiceId>,
    /// Progression actuelle.
    pub progress: u32,
    /// Objectif à atteindre.
    pub goal: u32,
    /// Récompense en MIYU.
    pub reward_miyu: u64,
    /// Items récompenses (optionnel).
    pub reward_items: Vec<String>,
    /// Complétée ou non.
    pub completed: bool,
    /// Timestamp de complétion (None si pas encore complétée).
    pub completed_at: Option<f64>,
}

/// Type de notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationType {
    /// Achievement débloqué.
    AchievementUnlocked,
    /// Mission complétée.
    MissionCompleted,
    /// Item obtenu.
    ItemObtained,
    /// Points MIYU gagnés.
    MiyuEarned,
}

/// Notification affichée à l'utilisateur.
#[derive(Debug, Clone)]
pub struct LoyaltyNotification {
    /// Type de notification.
    pub notification_type: NotificationType,
    /// Titre.
    pub title: String,
    /// Message.
    pub message: String,
    /// Icône (emoji).
    pub icon: String,
    /// Timestamp de création.
    pub created_at: f64,
    /// Durée d'affichage (secondes).
    pub duration: f32,
}

impl LoyaltyNotification {
    /// Crée une notification pour un achievement débloqué.
    pub fn achievement_unlocked(achievement: &Achievement) -> Self {
        Self {
            notification_type: NotificationType::AchievementUnlocked,
            title: "Achievement débloqué !".to_string(),
            message: format!("{} (+{} MIYU)", achievement.name, achievement.reward_miyu),
            icon: achievement.icon.clone(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            duration: 5.0,
        }
    }

    /// Crée une notification pour une mission complétée.
    pub fn mission_completed(mission: &Mission) -> Self {
        Self {
            notification_type: NotificationType::MissionCompleted,
            title: "Mission accomplie !".to_string(),
            message: format!("{} (+{} MIYU)", mission.name, mission.reward_miyu),
            icon: "🎯".to_string(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            duration: 5.0,
        }
    }

    /// Crée une notification pour un item obtenu.
    pub fn item_obtained(item: &VirtualItem) -> Self {
        Self {
            notification_type: NotificationType::ItemObtained,
            title: "Nouvel objet !".to_string(),
            message: format!("{} ({})", item.name, item.rarity.label()),
            icon: item.icon.clone(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            duration: 5.0,
        }
    }

    /// Crée une notification pour des points MIYU gagnés.
    pub fn miyu_earned(amount: u64, reason: &str) -> Self {
        Self {
            notification_type: NotificationType::MiyuEarned,
            title: format!("+{} MIYU", amount),
            message: reason.to_string(),
            icon: "💎".to_string(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
            duration: 3.0,
        }
    }
}

/// État du système de fidélisation pour un profil.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyState {
    /// Points MIYU actuels.
    pub miyu_points: MiyuPoints,
    /// Achievements débloqués/réclamés.
    pub achievements: HashMap<String, Achievement>,
    /// Items possédés (ID -> quantité).
    pub items: HashMap<String, u32>,
    /// Missions actives.
    pub missions: HashMap<String, Mission>,
    /// Historique des transactions MIYU (optionnel, pour stats).
    pub transaction_history: Vec<MiyuTransaction>,
    /// Streak quotidien (nombre de jours consécutifs).
    pub daily_streak: u32,
    /// Dernière connexion (timestamp sec).
    pub last_login: Option<f64>,
}

impl Default for LoyaltyState {
    fn default() -> Self {
        Self::new()
    }
}

impl LoyaltyState {
    /// Crée un nouvel état vide.
    pub fn new() -> Self {
        Self {
            miyu_points: MiyuPoints::new(),
            achievements: HashMap::new(),
            items: HashMap::new(),
            missions: HashMap::new(),
            transaction_history: Vec::new(),
            daily_streak: 0,
            last_login: None,
        }
    }

    /// Ajoute des points MIYU.
    pub fn add_miyu(&mut self, amount: u64, reason: String) {
        self.miyu_points.add(amount);
        self.transaction_history.push(MiyuTransaction {
            amount: amount as i64,
            reason,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        });
    }

    /// Dépense des points MIYU (retourne true si succès).
    pub fn spend_miyu(&mut self, amount: u64, reason: String) -> bool {
        if self.miyu_points.spend(amount) {
            self.transaction_history.push(MiyuTransaction {
                amount: -(amount as i64),
                reason,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            });
            true
        } else {
            false
        }
    }

    /// Ajoute un item à l'inventaire.
    pub fn add_item(&mut self, item_id: String, quantity: u32) {
        *self.items.entry(item_id).or_insert(0) += quantity;
    }

    /// Débloque un achievement.
    pub fn unlock_achievement(&mut self, achievement_id: String, achievement: Achievement) {
        if let Some(existing) = self.achievements.get(&achievement_id) {
            if existing.status != AchievementStatus::Locked {
                return; // Déjà débloqué
            }
        }

        let mut unlocked = achievement;
        unlocked.status = AchievementStatus::Unlocked;
        unlocked.unlocked_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        );
        self.achievements.insert(achievement_id, unlocked);
    }

    /// Réclame la récompense d'un achievement.
    pub fn claim_achievement(&mut self, achievement_id: &str) -> Option<u64> {
        let (reward, name) = {
            if let Some(achievement) = self.achievements.get_mut(achievement_id) {
                if achievement.status == AchievementStatus::Unlocked {
                    achievement.status = AchievementStatus::Claimed;
                    achievement.claimed_at = Some(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64(),
                    );
                    let reward = achievement.reward_miyu;
                    let name = achievement.name.clone();
                    (Some(reward), Some(name))
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            }
        };
        if let (Some(reward), Some(name)) = (reward, name) {
            self.add_miyu(reward, format!("Achievement : {}", name));
            Some(reward)
        } else {
            None
        }
    }

    /// Met à jour le daily streak.
    pub fn update_daily_streak(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        if let Some(last) = self.last_login {
            let elapsed_days = ((now - last) / 86400.0) as u32;
            if elapsed_days == 1 {
                // Jour consécutif
                self.daily_streak += 1;
            } else if elapsed_days > 1 {
                // Streak cassé
                self.daily_streak = 1;
            }
            // Si même jour (elapsed_days == 0), on ne change rien
        } else {
            // Première connexion
            self.daily_streak = 1;
        }

        self.last_login = Some(now);
    }
}

/// Transaction MIYU (historique).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyuTransaction {
    /// Montant (positif = gain, négatif = dépense).
    pub amount: i64,
    /// Raison de la transaction.
    pub reason: String,
    /// Timestamp.
    pub timestamp: f64,
}

/// Système de gacha (tirage aléatoire).
pub struct GachaSystem {
    /// Pool d'items disponibles au gacha.
    pub available_items: Vec<VirtualItem>,
    /// Coût d'un tirage simple en MIYU.
    pub single_pull_cost: u64,
    /// Coût d'un tirage x10 en MIYU.
    pub multi_pull_cost: u64,
}

impl GachaSystem {
    /// Crée un nouveau système de gacha.
    pub fn new() -> Self {
        Self {
            available_items: Vec::new(),
            single_pull_cost: 100,
            multi_pull_cost: 900, // Réduction pour 10 tirages
        }
    }

    /// Effectue un tirage simple.
    pub fn single_pull(&self) -> Option<VirtualItem> {
        if self.available_items.is_empty() {
            return None;
        }

        // Algorithme de tirage pondéré par rareté
        let total_weight = self.available_items.iter().map(|item| {
            match item.rarity {
                ItemRarity::Common => 100,
                ItemRarity::Rare => 20,
                ItemRarity::Epic => 5,
                ItemRarity::Legendary => 1,
            }
        }).sum::<u32>();

        let mut rng = rand::random::<u32>() % total_weight;

        for item in &self.available_items {
            let weight = match item.rarity {
                ItemRarity::Common => 100,
                ItemRarity::Rare => 20,
                ItemRarity::Epic => 5,
                ItemRarity::Legendary => 1,
            };

            if rng < weight {
                return Some(item.clone());
            }
            rng -= weight;
        }

        None
    }

    /// Effectue un tirage x10.
    pub fn multi_pull(&self) -> Vec<VirtualItem> {
        (0..10).filter_map(|_| self.single_pull()).collect()
    }
}

impl Default for GachaSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Crée des achievements d'exemple pour la démonstration.
pub fn create_example_achievements() -> Vec<(String, Achievement)> {
    vec![
        (
            "first_launch".to_string(),
            Achievement {
                achievement_type: AchievementType::FirstLaunch,
                name: "Bienvenue !".to_string(),
                description: "Lancez Miyukini Central pour la première fois".to_string(),
                icon: "🎉".to_string(),
                reward_miyu: 100,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: None,
            },
        ),
        (
            "first_login".to_string(),
            Achievement {
                achievement_type: AchievementType::FirstLogin,
                name: "Connexion établie".to_string(),
                description: "Connectez-vous à votre profil Miyukini".to_string(),
                icon: "🔐".to_string(),
                reward_miyu: 50,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: None,
            },
        ),
        (
            "daily_streak_3".to_string(),
            Achievement {
                achievement_type: AchievementType::DailyStreak { days: 3 },
                name: "Assidu".to_string(),
                description: "Connectez-vous 3 jours d'affilée".to_string(),
                icon: "📅".to_string(),
                reward_miyu: 150,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: None,
            },
        ),
        (
            "daily_streak_7".to_string(),
            Achievement {
                achievement_type: AchievementType::DailyStreak { days: 7 },
                name: "Habitué".to_string(),
                description: "Connectez-vous 7 jours d'affilée".to_string(),
                icon: "🔥".to_string(),
                reward_miyu: 300,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: None,
            },
        ),
        (
            "service_explorer".to_string(),
            Achievement {
                achievement_type: AchievementType::ServiceCustom {
                    service: crate::catalog::ServiceId::Calculator,
                    achievement_id: "service_explorer".to_string(),
                },
                name: "Explorateur".to_string(),
                description: "Ouvrez 5 services différents".to_string(),
                icon: "🧭".to_string(),
                reward_miyu: 200,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: None,
            },
        ),
        (
            "miyu_clicker_10".to_string(),
            Achievement {
                achievement_type: AchievementType::ServiceUsage {
                    service: crate::catalog::ServiceId::MiyuClicker,
                    count: 10,
                },
                name: "Cliqueur amateur".to_string(),
                description: "Lancez Lord of the Click 10 fois".to_string(),
                icon: "🖱️".to_string(),
                reward_miyu: 100,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: Some(crate::catalog::ServiceId::MiyuClicker),
            },
        ),
        (
            "gacha_first_pull".to_string(),
            Achievement {
                achievement_type: AchievementType::ServiceCustom {
                    service: crate::catalog::ServiceId::MiyukiniLoyalty,
                    achievement_id: "gacha_first_pull".to_string(),
                },
                name: "Premier tirage".to_string(),
                description: "Effectuez votre premier tirage gacha".to_string(),
                icon: "🎰".to_string(),
                reward_miyu: 50,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: Some(crate::catalog::ServiceId::MiyukiniLoyalty),
            },
        ),
        (
            "collector".to_string(),
            Achievement {
                achievement_type: AchievementType::CollectionComplete {
                    collection_name: "starter_pack".to_string(),
                },
                name: "Collectionneur".to_string(),
                description: "Obtenez 10 objets différents".to_string(),
                icon: "🎁".to_string(),
                reward_miyu: 500,
                status: AchievementStatus::Locked,
                unlocked_at: None,
                claimed_at: None,
                source_service: Some(crate::catalog::ServiceId::MiyukiniLoyalty),
            },
        ),
    ]
}

/// Initialise le système de fidélisation avec des achievements et points de départ.
pub fn initialize_loyalty_system(state: &mut LoyaltyState) {
    // Ajouter les achievements d'exemple
    for (id, achievement) in create_example_achievements() {
        if !state.achievements.contains_key(&id) {
            state.achievements.insert(id, achievement);
        }
    }

    // Donner des points de départ
    if state.miyu_points.get() == 0 {
        state.add_miyu(500, "Bonus de bienvenue".to_string());
    }
}

/// Vérifie et débloque automatiquement certains achievements selon le contexte.
pub fn check_and_unlock_achievements(
    state: &mut LoyaltyState,
    notifications: &mut Vec<LoyaltyNotification>,
) {
    // Achievement "Premier lancement"
    if let Some(achievement) = state.achievements.get("first_launch") {
        if achievement.status == AchievementStatus::Locked {
            let mut unlocked = achievement.clone();
            unlocked.status = AchievementStatus::Unlocked;
            unlocked.unlocked_at = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            );
            notifications.push(LoyaltyNotification::achievement_unlocked(&unlocked));
            state.achievements.insert("first_launch".to_string(), unlocked);
        }
    }

    // Vérifier les daily streaks
    if state.daily_streak >= 3 {
        if let Some(achievement) = state.achievements.get("daily_streak_3") {
            if achievement.status == AchievementStatus::Locked {
                let mut unlocked = achievement.clone();
                unlocked.status = AchievementStatus::Unlocked;
                unlocked.unlocked_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                );
                notifications.push(LoyaltyNotification::achievement_unlocked(&unlocked));
                state.achievements.insert("daily_streak_3".to_string(), unlocked);
            }
        }
    }

    if state.daily_streak >= 7 {
        if let Some(achievement) = state.achievements.get("daily_streak_7") {
            if achievement.status == AchievementStatus::Locked {
                let mut unlocked = achievement.clone();
                unlocked.status = AchievementStatus::Unlocked;
                unlocked.unlocked_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                );
                notifications.push(LoyaltyNotification::achievement_unlocked(&unlocked));
                state.achievements.insert("daily_streak_7".to_string(), unlocked);
            }
        }
    }

    // Vérifier le collectionneur (10 objets)
    if state.items.len() >= 10 {
        if let Some(achievement) = state.achievements.get("collector") {
            if achievement.status == AchievementStatus::Locked {
                let mut unlocked = achievement.clone();
                unlocked.status = AchievementStatus::Unlocked;
                unlocked.unlocked_at = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64(),
                );
                notifications.push(LoyaltyNotification::achievement_unlocked(&unlocked));
                state.achievements.insert("collector".to_string(), unlocked);
            }
        }
    }
}
