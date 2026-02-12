//! Simulation idle — tick, production, transformation, construction, croissance.
//! Toolkit MiyuClickerIdleSim — pas de décision, exécution uniquement.
//!
//! @id: miyuclicker_idlesim_module
//! @role: mutator
//! @layer: app
//! @do: idle_simulation_tick_production_transformation_construction
//! @human: Rythme jour (86 400 s jeu), boucles : production, transformation,
//!         construction, croissance naturelle, moral, game over.

use crate::state::{
    Allocation, AllocationBatisseurs, BuildingType, ClickTarget, GameState, ProductionRates,
};

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes — Temps
// ═══════════════════════════════════════════════════════════════════════════════

/// Secondes de jeu par jour (pour production/consommation par jour).
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// Vitesses de jeu : secondes in-game par seconde IRL.
/// @id: miyuclicker_idlesim_game_speeds
/// @do: define_game_speed_multipliers
/// @role: data
/// @human: Pause=0 ; 1=1h/s ; 2=3h/s ; 3=6h/s ; 4=1j/s ; 5=2j/s.
pub const GAME_SPEED_PAUSE: f64 = 0.0;
pub const GAME_SPEED_1: f64 = 3600.0;
pub const GAME_SPEED_2: f64 = 10800.0;
pub const GAME_SPEED_3: f64 = 21600.0;
pub const GAME_SPEED_4: f64 = 86400.0;
pub const GAME_SPEED_5: f64 = 172800.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes — Production (par ouvrier et par jour in-game)
// ═══════════════════════════════════════════════════════════════════════════════

/// Ferme : 2 nourriture / ouvrier / jour.
const PROD_NOURRITURE_PAR_OUVRIER_PAR_JOUR: f64 = 2.0;
/// Scierie : 1 bois / ouvrier / jour.
const PROD_BOIS_PAR_OUVRIER_PAR_JOUR: f64 = 1.0;
/// Carrière : 1 pierre / ouvrier / jour.
const PROD_PIERRE_PAR_OUVRIER_PAR_JOUR: f64 = 1.0;
/// Mine : 0.5 métal / ouvrier / jour.
const PROD_METAL_PAR_OUVRIER_PAR_JOUR: f64 = 0.5;
/// Atelier : 0.5 outils / ouvrier / jour.
const PROD_OUTILS_PAR_OUVRIER_PAR_JOUR: f64 = 0.5;
/// Forge : 0.2 armes / ouvrier / jour.
const PROD_ARMES_PAR_OUVRIER_PAR_JOUR: f64 = 0.2;

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes — Consommation
// ═══════════════════════════════════════════════════════════════════════════════

/// Consommation nourriture : 1 par personne (ouvrier+bâtisseur+soldat) par jour.
const CONSO_NOURRITURE_PAR_PERS_PAR_JOUR: f64 = 1.0;

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes — Construction
// ═══════════════════════════════════════════════════════════════════════════════

/// Points de construction par bâtisseur et par jour (in-game).
/// @id: miyuclicker_idlesim_construction_pts_per_batisseur_per_day
const CONSTRUCTION_PTS_PER_BATISSEUR_PER_DAY: f64 = 1.0;

/// Maison : 30 pts base (+1% par niveau existant).
const MAISON_PTS_BASE: f64 = 30.0;
const MAISON_PTS_SCALE_PER_LVL: f64 = 0.01;

/// Caserne : 100 pts base (+5% par niveau existant).
const CASERNE_PTS_BASE: f64 = 100.0;
const CASERNE_PTS_SCALE_PER_LVL: f64 = 0.05;

/// Guilde des Maçons : 60 pts base (+5% par niveau existant).
const GUILDE_PTS_BASE: f64 = 60.0;
const GUILDE_PTS_SCALE_PER_LVL: f64 = 0.05;

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes — Moral et Game Over
// ═══════════════════════════════════════════════════════════════════════════════

/// Seuil (jours simulés) à 0 nourriture au-delà duquel Game Over.
/// @id: miyuclicker_idlesim_jours_game_over
pub const JOURS_NOURRITURE_ZERO_GAME_OVER: f64 = 7.0;

/// Baisse moral par jour jeu : nourriture < population.
const MORAL_BAISSE_1_PCT_PAR_JOUR: f64 = 0.01;
/// Baisse moral par jour jeu : nourriture ≤ population/2.
const MORAL_BAISSE_5_PCT_PAR_JOUR: f64 = 0.05;
/// Remontée moral par jour jeu quand nourriture suffisante.
const MORAL_REMONTEE_PAR_JOUR: f64 = 0.02;

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes — Croissance naturelle
// ═══════════════════════════════════════════════════════════════════════════════

/// Taux de croissance de base : 0.1 ouvrier / jour (avant fécondité).
/// @id: miyuclicker_idlesim_growth_rate
/// @human: Croissance = 0.1 × fécondité ouvriers/jour. Nouveaux = ouvriers.
const GROWTH_RATE_BASE_PER_DAY: f64 = 0.1;

// ═══════════════════════════════════════════════════════════════════════════════
// Clic manuel
// ═══════════════════════════════════════════════════════════════════════════════

/// Applique un gain au clic manuel (+1 sur la ressource correspondante).
/// @id: miyuclicker_idlesim_apply_click
/// @do: apply_manual_click_gain_to_state
/// @role: mutator
pub fn apply_click(state: &mut GameState, target: ClickTarget) {
    match target {
        ClickTarget::Ferme => {
            state.nourriture = (state.nourriture + 1.0).min(state.cap_nourriture());
        }
        ClickTarget::Scierie => {
            state.bois = (state.bois + 1.0).min(state.cap_bois());
        }
        ClickTarget::Carriere => {
            state.pierre = (state.pierre + 1.0).min(state.cap_pierre());
        }
        ClickTarget::Mine => {
            state.metal = (state.metal + 1.0).min(state.cap_metal());
        }
        ClickTarget::Atelier => {
            state.outils = (state.outils + 1.0).min(state.cap_outils());
        }
        ClickTarget::Forge => {
            state.armes = (state.armes + 1.0).min(state.cap_armes());
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Transformations (ouvrier ↔ bâtisseur / soldat)
// ═══════════════════════════════════════════════════════════════════════════════

/// Transforme 1 ouvrier en 1 bâtisseur.
/// Coût : 20 outils + 1 par bâtisseur existant (escalade).
/// Renvoie true si effectué.
/// @id: miyuclicker_idlesim_convert_ouvrier_to_batisseur
/// @do: transform_worker_to_builder_with_tool_cost
/// @role: mutator
/// @human: Vérifie ouvrier libre, cap bâtisseurs, et coût outils.
pub fn convert_ouvrier_to_batisseur(state: &mut GameState) -> bool {
    if !state.can_convert_to_batisseur() {
        return false;
    }
    let cout = state.cout_batisseur();
    state.ouvriers -= 1;
    state.outils -= cout;
    state.batisseurs += 1;
    clamp_resources(state);
    true
}

/// Transforme 1 ouvrier en 1 soldat.
/// Coût : 10 armes + 1 par soldat existant (escalade).
/// Renvoie true si effectué.
/// @id: miyuclicker_idlesim_convert_ouvrier_to_soldat
/// @do: transform_worker_to_soldier_with_arms_cost
/// @role: mutator
/// @human: Vérifie ouvrier libre, cap soldats, et coût armes.
pub fn convert_ouvrier_to_soldat(state: &mut GameState) -> bool {
    if !state.can_convert_to_soldat() {
        return false;
    }
    let cout = state.cout_soldat();
    state.ouvriers -= 1;
    state.armes -= cout;
    state.soldats += 1;
    clamp_resources(state);
    true
}

/// Rétrograde 1 bâtisseur en 1 ouvrier (gratuit).
/// Renvoie true si effectué.
/// @id: miyuclicker_idlesim_convert_batisseur_to_ouvrier
/// @do: demote_builder_back_to_worker_free
/// @role: mutator
pub fn convert_batisseur_to_ouvrier(state: &mut GameState) -> bool {
    if state.batisseurs <= 0 || state.batisseurs_libres() <= 0 {
        return false;
    }
    state.batisseurs -= 1;
    state.ouvriers += 1;
    true
}

/// Rétrograde 1 soldat en 1 ouvrier (gratuit).
/// Renvoie true si effectué.
/// @id: miyuclicker_idlesim_convert_soldat_to_ouvrier
/// @do: demote_soldier_back_to_worker_free
/// @role: mutator
pub fn convert_soldat_to_ouvrier(state: &mut GameState) -> bool {
    if state.soldats <= 0 {
        return false;
    }
    state.soldats -= 1;
    state.ouvriers += 1;
    true
}

// ═══════════════════════════════════════════════════════════════════════════════
// Allocations
// ═══════════════════════════════════════════════════════════════════════════════

/// Applique l'allocation des ouvriers (somme ≤ ouvriers).
/// @id: miyuclicker_idlesim_apply_allocation
/// @do: apply_worker_allocation_to_state
/// @role: mutator
pub fn apply_allocation(state: &mut GameState, allocation: &Allocation) -> Result<(), String> {
    let total = allocation.total();
    if total > state.ouvriers {
        return Err(format!(
            "Affectation ouvriers {} > ouvriers disponibles {}",
            total, state.ouvriers
        ));
    }
    state.allocation = allocation.clone();
    Ok(())
}

/// Applique l'allocation des bâtisseurs (somme ≤ bâtisseurs).
/// @id: miyuclicker_idlesim_apply_allocation_batisseurs
/// @do: apply_builder_allocation_to_state
/// @role: mutator
pub fn apply_allocation_batisseurs(
    state: &mut GameState,
    allocation: &AllocationBatisseurs,
) -> Result<(), String> {
    let total = allocation.total();
    if total > state.batisseurs {
        return Err(format!(
            "Affectation bâtisseurs {} > bâtisseurs disponibles {}",
            total, state.batisseurs
        ));
    }
    state.allocation_batisseurs = allocation.clone();
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Construction
// ═══════════════════════════════════════════════════════════════════════════════

/// Démarre la construction d'un bâtiment : prélève le coût B/P/M et marque comme payé.
/// Renvoie true si effectué.
/// @id: miyuclicker_idlesim_start_construction
/// @do: pay_resources_and_start_building_construction
/// @role: mutator
/// @human: Clic « Construire » → paie coût → bâtisseurs assignés font progresser.
pub fn start_construction(state: &mut GameState, bt: BuildingType) -> bool {
    if !state.can_start_construction(bt) {
        return false;
    }
    let cost = state.cout_construction(bt);
    state.bois -= cost.bois;
    state.pierre -= cost.pierre;
    state.metal -= cost.metal;
    match bt {
        BuildingType::Maison => state.construction_maison_paid = true,
        BuildingType::Caserne => state.construction_caserne_paid = true,
        BuildingType::GuildeDesMacons => state.construction_guilde_paid = true,
    }
    clamp_resources(state);
    true
}

/// Pts construction requis pour le prochain niveau (par type).
/// @id: miyuclicker_idlesim_pts_required_for_building
/// @do: compute_construction_pts_required_by_type
/// @role: reader
#[must_use]
pub fn pts_required_for_building(state: &GameState, bt: BuildingType) -> f64 {
    match bt {
        BuildingType::Maison => {
            MAISON_PTS_BASE * (1.0 + MAISON_PTS_SCALE_PER_LVL * state.maisons_lvl as f64)
        }
        BuildingType::Caserne => {
            CASERNE_PTS_BASE * (1.0 + CASERNE_PTS_SCALE_PER_LVL * state.caserne_lvl as f64)
        }
        BuildingType::GuildeDesMacons => {
            GUILDE_PTS_BASE * (1.0 + GUILDE_PTS_SCALE_PER_LVL * state.guilde_lvl as f64)
        }
    }
}

/// Pourcentage de progression de construction (0.0–100.0) pour affichage UI.
/// @id: miyuclicker_idlesim_construction_progress_pct
/// @do: compute_construction_progress_percentage
/// @role: reader
#[must_use]
pub fn construction_progress_pct(state: &GameState, bt: BuildingType) -> f64 {
    let current = match bt {
        BuildingType::Maison => state.construction_maison,
        BuildingType::Caserne => state.construction_caserne,
        BuildingType::GuildeDesMacons => state.construction_guilde,
    };
    let required = pts_required_for_building(state, bt);
    if required <= 0.0 {
        return 0.0;
    }
    (current / required * 100.0).clamp(0.0, 100.0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Déploiement soldats
// ═══════════════════════════════════════════════════════════════════════════════

/// Déploie des soldats du domaine vers une cité joueur (pour ensuite les déplacer sur la carte).
/// @id: miyuclicker_idlesim_deploy_soldats
/// @do: deploy_soldiers_from_domain_to_player_city
/// @role: mutator
/// @human: Pont entre « Mon domaine » et « Carte du monde ».
pub fn deploy_soldats_to_cite(
    state: &mut GameState,
    cite_id: &str,
    count: i64,
) -> Result<(), String> {
    if count <= 0 {
        return Err("Nombre invalide".to_string());
    }
    if state.soldats < count {
        return Err(format!(
            "Pas assez de soldats ({} disponibles, {} demandés)",
            state.soldats, count
        ));
    }
    let cite = state
        .cites
        .iter_mut()
        .find(|c| {
            c.id == cite_id
                && matches!(c.proprietaire, crate::state::Proprietaire::Joueur)
        })
        .ok_or_else(|| "Cité joueur introuvable".to_string())?;
    state.soldats -= count;
    cite.troupes += count;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Taux de production (pour affichage UI)
// ═══════════════════════════════════════════════════════════════════════════════

/// Calcule les taux de production effectifs par seconde IRL (pour affichage GUI).
/// @id: miyuclicker_idlesim_production_rates
/// @do: compute_effective_production_rates_per_real_second
/// @role: reader
/// @human: rate = (ouvriers_affectés × production_par_jour / 86400) × game_speed.
///         nourriture = production − consommation (net).
#[must_use]
pub fn production_rates(state: &GameState, game_speed: f64) -> ProductionRates {
    if game_speed <= 0.0 {
        return ProductionRates::default();
    }
    let factor = game_speed / SECONDS_PER_DAY;
    let a = &state.allocation;
    let pop = state.pop_totale() as f64;
    ProductionRates {
        nourriture: (a.ferme as f64) * PROD_NOURRITURE_PAR_OUVRIER_PAR_JOUR * factor
            - pop * CONSO_NOURRITURE_PAR_PERS_PAR_JOUR * factor,
        bois: (a.scierie as f64) * PROD_BOIS_PAR_OUVRIER_PAR_JOUR * factor,
        pierre: (a.carriere as f64) * PROD_PIERRE_PAR_OUVRIER_PAR_JOUR * factor,
        metal: (a.mine as f64) * PROD_METAL_PAR_OUVRIER_PAR_JOUR * factor,
        outils: (a.atelier as f64) * PROD_OUTILS_PAR_OUVRIER_PAR_JOUR * factor,
        armes: (a.forge as f64) * PROD_ARMES_PAR_OUVRIER_PAR_JOUR * factor,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tick principal
// ═══════════════════════════════════════════════════════════════════════════════

/// Met à jour l'état sur un tick (production, consommation, construction, croissance, moral).
/// @id: miyuclicker_idlesim_tick
/// @do: advance_game_time_and_apply_all_simulation_loops
/// @role: mutator
/// @human: delta_s = secondes IRL ; game_speed = s in-game/s IRL (0 = pause).
///
/// Boucles exécutées :
/// 1. Production (ouvriers assignés × taux/jour × factor)
/// 2. Consommation nourriture (pop_totale × 1/jour)
/// 3. Moral (bonheur) + Game Over
/// 4. Croissance naturelle (nouveaux ouvriers si nourriture ok)
/// 5. Construction (bâtisseurs assignés × 1 pt/jour × factor)
/// 6. Caps (plafonds ressources et population)
pub fn tick(state: &mut GameState, delta_s: f64, game_speed: f64) {
    if game_speed <= 0.0 || state.game_over {
        return;
    }
    let game_seconds = (delta_s * game_speed).min(SECONDS_PER_DAY * 2.0);
    let factor = game_seconds / SECONDS_PER_DAY;

    // ─── 1. Production ─────────────────────────────────────────────────
    let a = &state.allocation;
    state.nourriture += (a.ferme as f64) * PROD_NOURRITURE_PAR_OUVRIER_PAR_JOUR * factor;
    state.bois += (a.scierie as f64) * PROD_BOIS_PAR_OUVRIER_PAR_JOUR * factor;
    state.pierre += (a.carriere as f64) * PROD_PIERRE_PAR_OUVRIER_PAR_JOUR * factor;
    state.metal += (a.mine as f64) * PROD_METAL_PAR_OUVRIER_PAR_JOUR * factor;
    state.outils += (a.atelier as f64) * PROD_OUTILS_PAR_OUVRIER_PAR_JOUR * factor;
    state.armes += (a.forge as f64) * PROD_ARMES_PAR_OUVRIER_PAR_JOUR * factor;

    // ─── 2. Consommation nourriture ────────────────────────────────────
    let pop = state.pop_totale() as f64;
    let consommation = pop * CONSO_NOURRITURE_PAR_PERS_PAR_JOUR * factor;
    state.nourriture -= consommation;

    // ─── 3. Moral (bonheur) + Game Over ────────────────────────────────
    if state.nourriture <= 0.0 {
        state.jours_nourriture_zero += factor;
        if state.jours_nourriture_zero > JOURS_NOURRITURE_ZERO_GAME_OVER {
            state.game_over = true;
        }
        state.moral = (state.moral - MORAL_BAISSE_5_PCT_PAR_JOUR * factor).max(0.0);
        state.fecondite = (state.fecondite - 0.03 * factor).max(0.0);
    } else {
        state.jours_nourriture_zero = 0.0;
        if pop > 0.0 {
            if state.nourriture <= pop / 2.0 {
                state.moral = (state.moral - MORAL_BAISSE_5_PCT_PAR_JOUR * factor).max(0.0);
                state.fecondite = (state.fecondite - 0.03 * factor).max(0.0);
            } else if state.nourriture < pop {
                state.moral = (state.moral - MORAL_BAISSE_1_PCT_PAR_JOUR * factor).max(0.0);
                state.fecondite = (state.fecondite - 0.01 * factor).max(0.0);
            } else {
                state.moral = (state.moral + MORAL_REMONTEE_PAR_JOUR * factor).min(1.0);
                state.fecondite = (state.fecondite + 0.01 * factor).min(1.0);
            }
        } else {
            state.moral = (state.moral + MORAL_REMONTEE_PAR_JOUR * factor).min(1.0);
            state.fecondite = (state.fecondite + 0.01 * factor).min(1.0);
        }
    }

    // ─── 4. Croissance naturelle ───────────────────────────────────────
    if state.nourriture > pop && state.pop_totale() < state.pop_max() {
        let growth = GROWTH_RATE_BASE_PER_DAY * state.fecondite * factor;
        state.growth_accumulator += growth;
        while state.growth_accumulator >= 1.0 && state.pop_totale() < state.pop_max() {
            state.ouvriers += 1;
            state.growth_accumulator -= 1.0;
        }
    }

    // ─── 5. Construction ───────────────────────────────────────────────
    let ab = &state.allocation_batisseurs;
    if state.construction_maison_paid {
        state.construction_maison +=
            (ab.maison as f64) * CONSTRUCTION_PTS_PER_BATISSEUR_PER_DAY * factor;
    }
    if state.construction_caserne_paid {
        state.construction_caserne +=
            (ab.caserne as f64) * CONSTRUCTION_PTS_PER_BATISSEUR_PER_DAY * factor;
    }
    if state.construction_guilde_paid {
        state.construction_guilde +=
            (ab.guilde_des_macons as f64) * CONSTRUCTION_PTS_PER_BATISSEUR_PER_DAY * factor;
    }

    // Tentative de complétion des bâtiments
    for bt in [
        BuildingType::Maison,
        BuildingType::Caserne,
        BuildingType::GuildeDesMacons,
    ] {
        try_complete_building(state, bt);
    }

    // ─── 6. Caps (plafonds) ────────────────────────────────────────────
    apply_caps(state);

    // ─── Avance du temps ───────────────────────────────────────────────
    state.temps_simule_s += game_seconds;
    clamp_resources(state);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Fonctions internes
// ═══════════════════════════════════════════════════════════════════════════════

/// Ajuste les valeurs pour qu'elles ne soient pas négatives.
/// @id: miyuclicker_idlesim_clamp_resources
fn clamp_resources(state: &mut GameState) {
    state.nourriture = state.nourriture.max(0.0);
    state.bois = state.bois.max(0.0);
    state.pierre = state.pierre.max(0.0);
    state.metal = state.metal.max(0.0);
    state.outils = state.outils.max(0.0);
    state.armes = state.armes.max(0.0);
    state.ouvriers = state.ouvriers.max(0);
    state.batisseurs = state.batisseurs.max(0);
    state.soldats = state.soldats.max(0);
}

/// Applique tous les plafonds (ressources et population).
/// @id: miyuclicker_idlesim_apply_caps
fn apply_caps(state: &mut GameState) {
    // Caps ressources
    state.nourriture = state.nourriture.min(state.cap_nourriture());
    state.bois = state.bois.min(state.cap_bois());
    state.pierre = state.pierre.min(state.cap_pierre());
    state.metal = state.metal.min(state.cap_metal());
    state.outils = state.outils.min(state.cap_outils());
    state.armes = state.armes.min(state.cap_armes());

    // Caps population
    let pop_max = state.pop_max();
    while state.pop_totale() > pop_max && state.ouvriers > 0 {
        state.ouvriers -= 1;
    }
    let cap_soldats = state.cap_soldats();
    if state.soldats > cap_soldats {
        let excedent = state.soldats - cap_soldats;
        state.soldats = cap_soldats;
        state.ouvriers += excedent; // rétrogradés en ouvriers
    }
    let cap_batisseurs = state.cap_batisseurs();
    if state.batisseurs > cap_batisseurs {
        let excedent = state.batisseurs - cap_batisseurs;
        state.batisseurs = cap_batisseurs;
        state.ouvriers += excedent; // rétrogradés en ouvriers
    }
}

/// Tente de compléter un bâtiment si les pts sont atteints et la construction est payée.
/// @id: miyuclicker_idlesim_try_complete_building
/// @do: try_complete_building_by_type
/// @role: mutator
fn try_complete_building(state: &mut GameState, bt: BuildingType) {
    let required = pts_required_for_building(state, bt);
    let (progress, paid) = match bt {
        BuildingType::Maison => (state.construction_maison, state.construction_maison_paid),
        BuildingType::Caserne => (state.construction_caserne, state.construction_caserne_paid),
        BuildingType::GuildeDesMacons => {
            (state.construction_guilde, state.construction_guilde_paid)
        }
    };

    if !paid || progress < required {
        return;
    }

    // Complétion : level up + reset
    match bt {
        BuildingType::Maison => {
            state.construction_maison -= required;
            state.construction_maison_paid = false;
            state.maisons_lvl += 1;
        }
        BuildingType::Caserne => {
            state.construction_caserne -= required;
            state.construction_caserne_paid = false;
            state.caserne_lvl += 1;
        }
        BuildingType::GuildeDesMacons => {
            state.construction_guilde -= required;
            state.construction_guilde_paid = false;
            state.guilde_lvl += 1;
        }
    }
}
