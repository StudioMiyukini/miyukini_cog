//! État du jeu MiyuClicker — ressources, population, bâtiments, carte.
//! Aligné sur le schéma GUI de référence (MiyukiniClicker_GUI.jpg).
//!
//! @id: miyuclicker_state_module
//! @role: data
//! @layer: domain
//! @do: represent_game_state_and_entities
//! @human: Ouvriers = base. Bâtisseurs (coût outils). Soldats (coût armes). 6 ressources, 3 bâtiments.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// Enums
// ═══════════════════════════════════════════════════════════════════════════════

/// Cible du clic manuel (production directe +1 ressource).
/// @id: miyuclicker_state_click_target
/// @do: represent_manual_click_target
/// @role: data
/// @human: Clic Ferme=+1 nourriture, Scierie=+1 bois, Carrière=+1 pierre, Mine=+1 métal, Atelier=+1 outils, Forge=+1 armes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickTarget {
    /// Ferme → +1 nourriture
    Ferme,
    /// Scierie → +1 bois
    Scierie,
    /// Carrière → +1 pierre
    Carriere,
    /// Mine → +1 métal
    Mine,
    /// Atelier → +1 outils
    Atelier,
    /// Forge → +1 armes
    Forge,
}

/// Type de bâtiment constructible (GUI panneau droit).
/// @id: miyuclicker_state_building_type
/// @do: represent_building_type_for_construction
/// @role: data
/// @human: Maison=+pop, Caserne=+soldats max, Guilde=+bâtisseurs max.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingType {
    /// Maisons : augmente la population maximale (+4/niveau).
    Maison,
    /// Casernes : augmente le cap soldats (+5/niveau).
    Caserne,
    /// Guilde des Maçons : augmente le cap bâtisseurs (+3/niveau).
    GuildeDesMacons,
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

// ═══════════════════════════════════════════════════════════════════════════════
// Carte
// ═══════════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════════
// Allocations
// ═══════════════════════════════════════════════════════════════════════════════

/// Allocation des ouvriers aux bâtiments de production.
/// GUI panneau gauche : Ferme, Scierie, Carrière, Mine, Atelier, Forge.
/// Chaque [+] / [-] ajoute ou retire 1 ouvrier du bâtiment.
/// @id: miyuclicker_state_allocation
/// @do: represent_worker_allocation_per_production_building
/// @role: data
/// @human: Ferme=nourriture, Scierie=bois, Carrière=pierre, Mine=métal, Atelier=outils, Forge=armes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Allocation {
    pub ferme: i64,
    pub scierie: i64,
    pub carriere: i64,
    pub mine: i64,
    pub atelier: i64,
    pub forge: i64,
}

impl Allocation {
    /// Somme totale des ouvriers affectés.
    #[must_use]
    pub fn total(&self) -> i64 {
        self.ferme + self.scierie + self.carriere + self.mine + self.atelier + self.forge
    }
}

/// Allocation des bâtisseurs aux chantiers de construction.
/// GUI panneau droit : Maisons, Casernes, Guilde des Maçons.
/// Chaque [+] / [-] ajoute ou retire 1 bâtisseur du chantier.
/// @id: miyuclicker_state_allocation_batisseurs
/// @do: represent_builder_allocation_per_construction_site
/// @role: data
/// @human: Chaque bâtisseur alloué = 1 pt construction/jour (in-game) sur le chantier.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllocationBatisseurs {
    pub maison: i64,
    pub caserne: i64,
    pub guilde_des_macons: i64,
}

impl AllocationBatisseurs {
    /// Somme totale des bâtisseurs affectés.
    #[must_use]
    pub fn total(&self) -> i64 {
        self.maison + self.caserne + self.guilde_des_macons
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Données auxiliaires
// ═══════════════════════════════════════════════════════════════════════════════

/// Coût en ressources pour construire un bâtiment.
/// GUI : B:xx P:xx M:xx (Bois, Pierre, Métal).
/// @id: miyuclicker_state_building_cost
/// @do: represent_building_resource_cost
/// @role: data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuildingCost {
    pub bois: f64,
    pub pierre: f64,
    pub metal: f64,
}

/// Taux de production effectif par seconde IRL (pour affichage UI).
/// GUI panneau gauche : "xxx Food/sec", "xxx Bois/sec", etc.
/// @id: miyuclicker_state_production_rates
/// @do: represent_per_second_production_rates_for_ui
/// @role: data
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ProductionRates {
    /// Nourriture nette (production − consommation)
    pub nourriture: f64,
    pub bois: f64,
    pub pierre: f64,
    pub metal: f64,
    pub outils: f64,
    pub armes: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// GameState
// ═══════════════════════════════════════════════════════════════════════════════

/// État complet du jeu (source de vérité unique).
/// @id: miyuclicker_state_game_state
/// @do: represent_full_game_state
/// @role: data
/// @human: 6 ressources, 3 types de population (ouvrier/bâtisseur/soldat),
///         3 bâtiments (Maisons/Casernes/Guilde), carte stratégique, moral.
#[derive(Debug, Clone)]
pub struct GameState {
    // ═══ Ressources (6 types — GUI barre supérieure) ═══════════════════════
    pub nourriture: f64,
    pub bois: f64,
    pub pierre: f64,
    pub metal: f64,
    pub outils: f64,
    pub armes: f64,

    // ═══ Population (3 rôles — GUI seconde barre) ══════════════════════════
    /// Ouvriers : unité de base, assignable à la production.
    /// Peuvent être transformés en bâtisseurs ou soldats.
    pub ouvriers: i64,
    /// Bâtisseurs : transformés depuis ouvriers (coût : 20 outils + 1/bâtisseur existant).
    /// Assignables aux chantiers de construction.
    pub batisseurs: i64,
    /// Soldats : transformés depuis ouvriers (coût : 10 armes + 1/soldat existant).
    /// Déployables sur la carte pour combat/conquête.
    pub soldats: i64,

    // ═══ Bâtiments — niveaux (GUI panneau droit) ═══════════════════════════
    /// Maisons : chaque niveau = +4 population maximale.
    /// @id: miyuclicker_state_maisons_lvl
    pub maisons_lvl: i64,
    /// Casernes : chaque niveau = +5 soldats max.
    /// @id: miyuclicker_state_caserne_lvl
    pub caserne_lvl: i64,
    /// Guilde des Maçons : chaque niveau = +3 bâtisseurs max.
    /// @id: miyuclicker_state_guilde_lvl
    pub guilde_lvl: i64,

    // ═══ Construction (progression + coût payé) ════════════════════════════
    /// Points de construction accumulés pour la prochaine Maison.
    pub construction_maison: f64,
    /// true si le coût « Construire » a été payé (Maison en cours).
    pub construction_maison_paid: bool,
    /// Points de construction accumulés pour la prochaine Caserne.
    pub construction_caserne: f64,
    /// true si le coût « Construire » a été payé (Caserne en cours).
    pub construction_caserne_paid: bool,
    /// Points de construction accumulés pour la prochaine Guilde.
    pub construction_guilde: f64,
    /// true si le coût « Construire » a été payé (Guilde en cours).
    pub construction_guilde_paid: bool,

    // ═══ Allocations ═══════════════════════════════════════════════════════
    /// Affectation des ouvriers aux bâtiments de production.
    pub allocation: Allocation,
    /// Affectation des bâtisseurs aux chantiers de construction.
    pub allocation_batisseurs: AllocationBatisseurs,

    // ═══ Bonheur / Moral ═══════════════════════════════════════════════════
    /// Moral (bonheur) 0.0..1.0 ; affiché en % (0–100). Baisse si nourriture insuffisante.
    /// @id: miyuclicker_state_moral
    pub moral: f64,
    /// Fécondité 0.0..1.0 ; taux de croissance naturelle de la population.
    pub fecondite: f64,
    /// Accumulateur fractionnaire pour la croissance naturelle (< 1.0 = pas encore +1 ouvrier).
    pub growth_accumulator: f64,
    /// Jours simulés consécutifs à 0 nourriture ; > 7 → Game Over.
    /// @id: miyuclicker_state_jours_nourriture_zero
    pub jours_nourriture_zero: f64,
    /// true si 0 nourriture pendant plus de 7 jours → mort du seigneur, fin de partie.
    /// @id: miyuclicker_state_game_over
    pub game_over: bool,

    // ═══ Carte ═════════════════════════════════════════════════════════════
    pub cites: Vec<Cite>,
    pub routes: Vec<Route>,
    /// Cache durée par paire (cite_a, cite_b) ; None = à recalculer.
    pub route_duree_by_pair: Option<std::collections::HashMap<(String, String), f64>>,
    pub deplacements: Vec<Deplacement>,

    // ═══ Méta ══════════════════════════════════════════════════════════════
    /// Pseudo du joueur (GUI barre supérieure, haut-gauche).
    pub pseudo: String,
    pub temps_simule_s: f64,
    pub slot_id: u8,
    pub version_sauvegarde: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Default (état initial nouvelle partie)
// ═══════════════════════════════════════════════════════════════════════════════

/// Valeurs par défaut : 10 ouvriers, 50 nourriture, bâtiments niveau de départ.
/// @id: miyuclicker_state_default
/// @do: default_game_state_initial_values
/// @role: data
impl Default for GameState {
    fn default() -> Self {
        Self {
            // Ressources initiales
            nourriture: 50.0,
            bois: 30.0,
            pierre: 30.0,
            metal: 10.0,
            outils: 25.0,
            armes: 0.0,

            // Population initiale : 10 ouvriers, aucun bâtisseur ni soldat
            ouvriers: 10,
            batisseurs: 0,
            soldats: 0,

            // Bâtiments initiaux
            maisons_lvl: 3, // pop max = 3×4 = 12
            caserne_lvl: 1, // soldats max = 1×5 = 5
            guilde_lvl: 1,  // bâtisseurs max = 1×3 = 3

            // Pas de construction en cours
            construction_maison: 0.0,
            construction_maison_paid: false,
            construction_caserne: 0.0,
            construction_caserne_paid: false,
            construction_guilde: 0.0,
            construction_guilde_paid: false,

            // Allocations vides
            allocation: Allocation::default(),
            allocation_batisseurs: AllocationBatisseurs::default(),

            // Moral plein, fécondité normale
            moral: 1.0,
            fecondite: 1.0,
            growth_accumulator: 0.0,
            jours_nourriture_zero: 0.0,
            game_over: false,

            // Carte vide (peuplée par carte::generate_carte_mvp)
            cites: Vec::new(),
            routes: Vec::new(),
            route_duree_by_pair: None,
            deplacements: Vec::new(),

            // Méta
            pseudo: "Seigneur".to_string(),
            temps_simule_s: 0.0,
            slot_id: 1,
            version_sauvegarde: "2.0".to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Méthodes GameState
// ═══════════════════════════════════════════════════════════════════════════════

impl GameState {
    // ─── Population ────────────────────────────────────────────────────────

    /// Population totale = ouvriers + bâtisseurs + soldats.
    /// @id: miyuclicker_state_pop_totale
    /// @do: compute_total_population
    /// @role: reader
    #[inline]
    #[must_use]
    pub fn pop_totale(&self) -> i64 {
        self.ouvriers + self.batisseurs + self.soldats
    }

    /// Population maximale = maisons_lvl × 4.
    /// @id: miyuclicker_state_pop_max
    /// @do: compute_max_population_from_houses
    /// @role: reader
    #[inline]
    #[must_use]
    pub fn pop_max(&self) -> i64 {
        self.maisons_lvl * 4
    }

    /// Ouvriers non assignés à la production (libres).
    /// @id: miyuclicker_state_ouvriers_libres
    #[inline]
    #[must_use]
    pub fn ouvriers_libres(&self) -> i64 {
        self.ouvriers - self.allocation.total()
    }

    /// Bâtisseurs non assignés aux chantiers (libres).
    /// @id: miyuclicker_state_batisseurs_libres
    #[inline]
    #[must_use]
    pub fn batisseurs_libres(&self) -> i64 {
        self.batisseurs - self.allocation_batisseurs.total()
    }

    // ─── Caps population ───────────────────────────────────────────────────

    /// Cap soldats = caserne_lvl × 5.
    /// @id: miyuclicker_state_cap_soldats
    /// @do: compute_soldier_cap_from_barracks
    /// @role: reader
    #[inline]
    #[must_use]
    pub fn cap_soldats(&self) -> i64 {
        self.caserne_lvl * 5
    }

    /// Cap bâtisseurs = guilde_lvl × 3.
    /// @id: miyuclicker_state_cap_batisseurs
    /// @do: compute_builder_cap_from_guild
    /// @role: reader
    #[inline]
    #[must_use]
    pub fn cap_batisseurs(&self) -> i64 {
        self.guilde_lvl * 3
    }

    // ─── Caps ressources ───────────────────────────────────────────────────

    /// Cap nourriture = 200 + maisons_lvl × 100.
    /// @id: miyuclicker_state_cap_nourriture
    #[inline]
    #[must_use]
    pub fn cap_nourriture(&self) -> f64 {
        (200 + self.maisons_lvl * 100) as f64
    }

    /// Cap bois = 200 + guilde_lvl × 50.
    /// @id: miyuclicker_state_cap_bois
    #[inline]
    #[must_use]
    pub fn cap_bois(&self) -> f64 {
        (200 + self.guilde_lvl * 50) as f64
    }

    /// Cap pierre = 200 + guilde_lvl × 50.
    /// @id: miyuclicker_state_cap_pierre
    #[inline]
    #[must_use]
    pub fn cap_pierre(&self) -> f64 {
        (200 + self.guilde_lvl * 50) as f64
    }

    /// Cap métal = 100 + guilde_lvl × 50.
    /// @id: miyuclicker_state_cap_metal
    #[inline]
    #[must_use]
    pub fn cap_metal(&self) -> f64 {
        (100 + self.guilde_lvl * 50) as f64
    }

    /// Cap outils = 100 + guilde_lvl × 25.
    /// @id: miyuclicker_state_cap_outils
    #[inline]
    #[must_use]
    pub fn cap_outils(&self) -> f64 {
        (100 + self.guilde_lvl * 25) as f64
    }

    /// Cap armes = 100 + caserne_lvl × 25.
    /// @id: miyuclicker_state_cap_armes
    #[inline]
    #[must_use]
    pub fn cap_armes(&self) -> f64 {
        (100 + self.caserne_lvl * 25) as f64
    }

    // ─── Coûts de transformation ───────────────────────────────────────────

    /// Coût pour transformer 1 ouvrier en bâtisseur : 20 outils + 1 par bâtisseur existant.
    /// @id: miyuclicker_state_cout_batisseur
    /// @do: compute_builder_transformation_cost
    /// @role: reader
    /// @human: Escalade : plus tu as de bâtisseurs, plus le suivant coûte cher.
    #[inline]
    #[must_use]
    pub fn cout_batisseur(&self) -> f64 {
        20.0 + self.batisseurs as f64
    }

    /// Coût pour transformer 1 ouvrier en soldat : 10 armes + 1 par soldat existant.
    /// @id: miyuclicker_state_cout_soldat
    /// @do: compute_soldier_transformation_cost
    /// @role: reader
    /// @human: Escalade : plus tu as de soldats, plus le suivant coûte cher.
    #[inline]
    #[must_use]
    pub fn cout_soldat(&self) -> f64 {
        10.0 + self.soldats as f64
    }

    // ─── Coûts de construction ─────────────────────────────────────────────

    /// Coût en ressources pour construire le prochain niveau d'un bâtiment.
    /// GUI : B:xx P:xx M:xx (Bois, Pierre, Métal). Escalade +10–15% par niveau.
    /// @id: miyuclicker_state_cout_construction
    /// @do: compute_building_resource_cost_by_type
    /// @role: reader
    #[must_use]
    pub fn cout_construction(&self, bt: BuildingType) -> BuildingCost {
        match bt {
            BuildingType::Maison => {
                let scale = 1.0 + 0.10 * self.maisons_lvl as f64;
                BuildingCost {
                    bois: (30.0 * scale).round(),
                    pierre: (20.0 * scale).round(),
                    metal: (5.0 * scale).round(),
                }
            }
            BuildingType::Caserne => {
                let scale = 1.0 + 0.15 * self.caserne_lvl as f64;
                BuildingCost {
                    bois: (50.0 * scale).round(),
                    pierre: (100.0 * scale).round(),
                    metal: (20.0 * scale).round(),
                }
            }
            BuildingType::GuildeDesMacons => {
                let scale = 1.0 + 0.10 * self.guilde_lvl as f64;
                BuildingCost {
                    bois: (40.0 * scale).round(),
                    pierre: (40.0 * scale).round(),
                    metal: (10.0 * scale).round(),
                }
            }
        }
    }

    // ─── Vérifications (pour UI : enable/disable boutons) ──────────────────

    /// Peut-on transformer 1 ouvrier en bâtisseur ?
    /// @id: miyuclicker_state_can_convert_to_batisseur
    #[must_use]
    pub fn can_convert_to_batisseur(&self) -> bool {
        self.ouvriers_libres() > 0
            && self.outils >= self.cout_batisseur()
            && self.batisseurs < self.cap_batisseurs()
    }

    /// Peut-on transformer 1 ouvrier en soldat ?
    /// @id: miyuclicker_state_can_convert_to_soldat
    #[must_use]
    pub fn can_convert_to_soldat(&self) -> bool {
        self.ouvriers_libres() > 0
            && self.armes >= self.cout_soldat()
            && self.soldats < self.cap_soldats()
    }

    /// Peut-on lancer la construction d'un bâtiment ?
    /// @id: miyuclicker_state_can_start_construction
    #[must_use]
    pub fn can_start_construction(&self, bt: BuildingType) -> bool {
        let paid = match bt {
            BuildingType::Maison => self.construction_maison_paid,
            BuildingType::Caserne => self.construction_caserne_paid,
            BuildingType::GuildeDesMacons => self.construction_guilde_paid,
        };
        if paid {
            return false; // déjà en cours
        }
        let cost = self.cout_construction(bt);
        self.bois >= cost.bois && self.pierre >= cost.pierre && self.metal >= cost.metal
    }

    // ─── Affichage ─────────────────────────────────────────────────────────

    /// Bonheur en pourcentage (0–100) pour affichage UI.
    /// @id: miyuclicker_state_bonheur_pourcent
    /// @do: compute_happiness_percentage_for_display
    /// @role: reader
    #[inline]
    #[must_use]
    pub fn bonheur_pourcent(&self) -> i64 {
        (self.moral.clamp(0.0, 1.0) * 100.0).round() as i64
    }

    /// Formate le temps simulé en "hh:mm jj/mm/yyyy" (calendrier 360j = 12×30).
    /// @id: miyuclicker_state_format_clock
    /// @do: format_game_time_as_hh_mm_ddmmyyyy
    /// @role: formatter
    #[must_use]
    pub fn format_clock(temps_simule_s: f64) -> String {
        let total_s = temps_simule_s.max(0.0) as u64;
        let hour = (total_s / 3600) % 24;
        let minute = (total_s / 60) % 60;
        let total_days = total_s / 86400;
        let year = total_days / 360;
        let day_of_year = total_days % 360;
        let month = (day_of_year / 30) + 1;
        let day = (day_of_year % 30) + 1;
        format!("{hour:02}:{minute:02} {day:02}/{month:02}/{year:04}")
    }

    // ─── Construction d'état ───────────────────────────────────────────────

    /// Nouvel état initial pour une partie (avec une cité joueur de base).
    /// @id: miyuclicker_state_new_game
    /// @do: create_initial_game_state_with_one_city
    /// @role: mutator
    #[must_use]
    pub fn new_game(slot_id: u8) -> Self {
        let mut state = Self {
            slot_id,
            ..Default::default()
        };
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
