//! État du jeu MiyuClicker (ressources, allocation, carte).
//! Aligné sur le Guide d'implémentation MVP.
//!
//! @role: data
//! @layer: domain
//! @do: represent_game_state_and_entities

use serde::{Deserialize, Serialize};

/// Cible du clic manuel (Champs, ressources, Château, Village).
/// @id: miyuclicker_state_click_target
/// @do: represent_click_target
/// @role: data
/// @human: Clic Champ = 1 nourriture ; Village = +1 pop -30 nourriture ; Château = +1 soldat -50 nourriture -5 armes ; ressource = +1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickTarget {
    /// Champs → +1 nourriture
    Champs,
    /// Ateliers → +1 outils
    Ateliers,
    /// Bois → +1 bois
    Bois,
    /// Pierre → +1 pierre
    Pierre,
    /// Fer → +1 fer
    Fer,
    /// Château → +1 soldat, -50 nourriture, -5 armes
    Chateau,
    /// Village → +1 pop, -30 nourriture
    Village,
}

/// Type de bâtiment constructible (pour allocation maçons et cartes UI).
/// @id: miyuclicker_state_building_type
/// @do: represent_building_type_for_construction
/// @role: data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    Maison,
    Caserne,
    Grenier,
    Depot,
    Entrepot,
}

/// Propriétaire d'une cité (joueur ou adverse).
/// @id: miyuclicker_state_proprietaire
/// @do: represent_cite_owner
/// @role: data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Proprietaire {
    /// Cité du joueur
    Joueur,
    /// Cité adverse (id pour distinguer plusieurs adverses)
    Adverse(u8),
}

/// Une cité sur la carte.
/// @id: miyuclicker_state_cite
/// @do: represent_city_on_map
/// @role: data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cite {
    pub id: String,
    pub nom: String,
    pub x: f32,
    pub y: f32,
    pub proprietaire: Proprietaire,
    pub troupes: i64,
}

/// Route entre deux cités (durée en secondes simulées).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub cite_a: String,
    pub cite_b: String,
    pub duree_s: f64,
}

/// Déplacement de troupes en cours.
/// @id: miyuclicker_state_deplacement
/// @do: represent_troop_movement_in_progress
/// @role: data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deplacement {
    pub from_cite_id: String,
    pub to_cite_id: String,
    pub troupes: i64,
    /// Progression 0..1
    pub progress: f64,
    /// true si attaque (cité cible adverse)
    pub attaquant: bool,
}

/// Affectation des gens par lieu (production automatique).
/// @id: miyuclicker_state_allocation
/// @do: represent_worker_allocation_per_zone
/// @human: Ferme=champs, Scierie, Carrière, Mine, Atelier=outils, Forge=armes, Recherche.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Allocation {
    pub champs: i64,
    pub ateliers: i64,
    pub scierie: i64,
    pub carriere: i64,
    pub mine: i64,
    #[serde(default)]
    pub forge: i64,
    pub recherche: i64,
}

impl Allocation {
    /// Somme totale des gens affectés.
    pub fn total(&self) -> i64 {
        self.champs + self.ateliers + self.scierie + self.carriere + self.mine + self.forge + self.recherche
    }
}

/// Affectation des maçons par type de bâtiment (construction).
/// @id: miyuclicker_state_allocation_macons
/// @do: represent_mason_allocation_per_building
/// @role: data
/// @human: Chaque maçon alloué = 1 pt construction/jour sur le bâtiment.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllocationMacons {
    pub maison: i64,
    pub caserne: i64,
    pub grenier: i64,
    pub depot: i64,
    pub entrepot: i64,
}

impl AllocationMacons {
    pub fn total(&self) -> i64 {
        self.maison + self.caserne + self.grenier + self.depot + self.entrepot
    }
}

/// État complet du jeu (source de vérité).
/// @id: miyuclicker_state_game_state
/// @do: represent_full_game_state
/// @role: data
/// @human: Ressources, cap, moral, allocation, carte, temps simulé.
#[derive(Debug, Clone)]
pub struct GameState {
    // Ressources
    pub or: i64,
    pub gens: i64,
    pub soldats: i64,
    pub recherche: i64,
    pub nourriture: f64,
    pub bois: f64,
    pub pierre: f64,
    pub fer: f64,
    pub outils: f64,
    pub armes: i64,
    // Cap et bonheur (moral)
    pub cap_gens: i64,
    /// Nombre de maisons ; cap_gens = maisons * 4.
    /// @id: miyuclicker_state_maisons
    pub maisons: i64,
    /// Niveau Caserne ; cap_soldats = caserne_lvl * 10.
    pub caserne_lvl: i64,
    /// Niveau Grenier ; cap_nourriture = grenier_lvl * 100.
    pub grenier_lvl: i64,
    /// Niveau Dépôt ; cap matières premières = depot_lvl * 100.
    pub depot_lvl: i64,
    /// Niveau Entrepôt ; cap produits manufacturés = entrepot_lvl * 50.
    pub entrepot_lvl: i64,
    /// Maçons (transformés depuis la pop via Guilde des Maçons) ; 1 pt construction/jour par maçon alloué.
    /// @id: miyuclicker_state_macons
    pub macons: i64,
    /// Progression construction (pts) vers le prochain niveau / prochaine maison.
    pub construction_maison: f64,
    /// true si le coût de la construction maison en cours a été payé au clic « Construire ».
    pub construction_maison_paid: bool,
    pub construction_caserne: f64,
    pub construction_grenier: f64,
    pub construction_depot: f64,
    pub construction_entrepot: f64,
    /// Allocation des maçons par type de bâtiment.
    pub allocation_macons: AllocationMacons,
    /// Bonheur (moral) 0..1 ; affiché en % (0–100). Métrique dynamique selon nourriture.
    /// @id: miyuclicker_state_moral_bonheur
    /// @do: represent_happiness_moral_metric
    /// @role: data
    /// @human: Bonheur = moral × 100 % ; baisse si nourriture < population (1 %/j ou 5 %/j).
    pub moral: f64,
    pub fecondite: f64,
    /// Jours simulés consécutifs à 0 nourriture ; > 7 → Game Over (mort du seigneur).
    /// @id: miyuclicker_state_jours_nourriture_zero
    /// @do: track_days_at_zero_food_for_game_over
    /// @role: data
    pub jours_nourriture_zero: f64,
    /// true si 0 nourriture pendant plus de 7 jours → mort du seigneur, fin de partie.
    /// @id: miyuclicker_state_game_over
    /// @do: represent_game_over_state
    /// @role: data
    pub game_over: bool,
    // Affectation (gens et maçons)
    pub allocation: Allocation,
    // Carte
    pub cites: Vec<Cite>,
    pub routes: Vec<Route>,
    /// Cache durée par paire (cite_a, cite_b) ; None = à recalculer (chargement ou routes modifiées).
    pub route_duree_by_pair: Option<std::collections::HashMap<(String, String), f64>>,
    pub deplacements: Vec<Deplacement>,
    // Meta
    pub temps_simule_s: f64,
    pub slot_id: u8,
    pub version_sauvegarde: String,
}

/// Valeurs par défaut (état initial 10 gens, 20 nourriture, 3 maisons, tous bâts niveau 1).
/// @id: miyuclicker_state_default
/// @do: default_game_state
/// @role: data
impl Default for GameState {
    fn default() -> Self {
        Self {
            or: 0,
            gens: 10,
            soldats: 0,
            recherche: 0,
            nourriture: 20.0,
            bois: 50.0,
            pierre: 50.0,
            fer: 0.0,
            outils: 20.0,
            armes: 0,
            cap_gens: 12,
            maisons: 3,
            caserne_lvl: 1,
            grenier_lvl: 1,
            depot_lvl: 1,
            entrepot_lvl: 1,
            macons: 0,
            construction_maison: 0.0,
            construction_maison_paid: false,
            construction_caserne: 0.0,
            construction_grenier: 0.0,
            construction_depot: 0.0,
            construction_entrepot: 0.0,
            allocation_macons: AllocationMacons::default(),
            moral: 1.0,
            fecondite: 1.0,
            jours_nourriture_zero: 0.0,
            game_over: false,
            allocation: Allocation::default(),
            cites: Vec::new(),
            routes: Vec::new(),
            route_duree_by_pair: None,
            deplacements: Vec::new(),
            temps_simule_s: 0.0,
            slot_id: 1,
            version_sauvegarde: "1.0".to_string(),
        }
    }
}

impl GameState {
    /// Cap population = maisons × 4.
    /// @id: miyuclicker_state_cap_gens_from_maisons
    #[inline]
    pub fn cap_gens_from_maisons(&self) -> i64 {
        self.maisons * 4
    }

    /// Cap soldats = caserne_lvl × 10.
    /// @id: miyuclicker_state_cap_soldats
    /// @do: compute_soldier_cap_from_barracks
    /// @role: reader
    #[inline]
    pub fn cap_soldats(&self) -> i64 {
        self.caserne_lvl * 10
    }

    /// Cap nourriture = grenier_lvl × 100.
    /// @id: miyuclicker_state_cap_nourriture
    /// @do: compute_food_cap_from_granary
    /// @role: reader
    #[inline]
    pub fn cap_nourriture(&self) -> f64 {
        (self.grenier_lvl * 100) as f64
    }

    /// Cap matières premières (bois + pierre + fer) = depot_lvl × 100.
    /// @id: miyuclicker_state_cap_matières
    /// @do: compute_raw_materials_cap_from_depot
    /// @role: reader
    #[inline]
    pub fn cap_matières(&self) -> i64 {
        self.depot_lvl * 100
    }

    /// Cap produits manufacturés (outils + armes) = entrepot_lvl × 50.
    /// @id: miyuclicker_state_cap_manufacturés
    /// @do: compute_manufactured_cap_from_warehouse
    /// @role: reader
    #[inline]
    pub fn cap_manufacturés(&self) -> i64 {
        self.entrepot_lvl * 50
    }

    /// Retourne le bonheur en pourcentage (0–100) pour affichage UI. Métrique dynamique.
    /// @id: miyuclicker_state_bonheur_pourcent
    /// @do: compute_happiness_percentage_for_display
    /// @role: reader
    /// @human: Bonheur = moral × 100, entier pour affichage.
    #[inline]
    pub fn bonheur_pourcent(&self) -> i64 {
        (self.moral.clamp(0.0, 1.0) * 100.0).round() as i64
    }

    /// Formate le temps simulé en "hh:mm jjmmyyyy" (départ 00:00 01/01/0000).
    /// @id: miyuclicker_state_format_clock
    /// @do: format_game_time_as_hh_mm_ddmmyyyy
    /// @role: formatter
    /// @human: Calendrier simplifié 360 jours/an (12×30).
    pub fn format_clock(temps_simule_s: f64) -> String {
        let total_s = temps_simule_s.max(0.0) as u64;
        let hour = (total_s / 3600) % 24;
        let minute = (total_s / 60) % 60;
        let total_days = total_s / 86400;
        let year = total_days / 360;
        let day_of_year = total_days % 360;
        let month = (day_of_year / 30) + 1;
        let day = (day_of_year % 30) + 1;
        format!(
            "{:02}:{:02} {:02}/{:02}/{:04}",
            hour,
            minute,
            day,
            month,
            year
        )
    }

    /// État initial pour une nouvelle partie (avec une cité joueur de base).
    /// @id: miyuclicker_state_new_game
    /// @do: create_initial_game_state_with_one_city
    /// @role: mutator
    pub fn new_game(slot_id: u8) -> Self {
        let mut state = Self::default();
        state.slot_id = slot_id;
        state.cites.push(Cite {
            id: "ma_cite".to_string(),
            nom: "Ma cité".to_string(),
            x: 0.5,
            y: 0.5,
            proprietaire: Proprietaire::Joueur,
            troupes: 0,
        });
        state
    }
}
