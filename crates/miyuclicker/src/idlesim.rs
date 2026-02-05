//! Simulation idle (tick, clic, affectation).
//! Toolkit MiyuClickerIdleSim — pas de décision, exécution uniquement.
//!
//! @id: miyuclicker_idlesim_module
//! @role: mutator
//! @layer: domain
//! @do: idle_simulation_tick_and_production
//! @human: Rythme jour (86400 s jeu), consommation 1/pers/j, production par lieu/jour.

use crate::state::{Allocation, AllocationMacons, BuildingType, ClickTarget, GameState};

/// Secondes de jeu par jour (pour production/consommation par jour).
pub const SECONDS_PER_DAY: f64 = 86400.0;

/// Vitesses de jeu : secondes in-game par seconde IRL.
/// @id: miyuclicker_idlesim_game_speeds
/// @do: define_game_speed_multipliers
/// @role: data
/// @human: Pause=0 ; 1=1h/s ; 2=3h/s ; 3=6h/s ; 4=1j/s ; 5=2j/s.
pub const GAME_SPEED_PAUSE: f64 = 0.0;
/// 1 h in-game par seconde IRL (index 1).
pub const GAME_SPEED_1: f64 = 3600.0;
/// 3 h in-game par seconde IRL (index 2).
pub const GAME_SPEED_2: f64 = 10800.0;
/// 6 h in-game par seconde IRL (index 3).
pub const GAME_SPEED_3: f64 = 21600.0;
/// 1 jour in-game par seconde IRL (index 4).
pub const GAME_SPEED_4: f64 = 86400.0;
/// 2 jours in-game par seconde IRL (index 5).
pub const GAME_SPEED_5: f64 = 172800.0;

/// Coûts / gains par clic (voir MiyuClicker - Batiments Macons et Construction).
/// @id: miyuclicker_idlesim_click_costs
const CLIC_VILLAGE_COUT_NOURRITURE: f64 = 30.0;
const CLIC_CHATEAU_COUT_NOURRITURE: f64 = 50.0;
const CLIC_CHATEAU_COUT_ARMES: i64 = 5;
const CLIC_CHAMPS_GAIN: f64 = 1.0;
const CLIC_RESSOURCE_GAIN: f64 = 1.0;

/// Production par personne et par jour (base utilisateur).
const PROD_NOURRITURE_PAR_PERS_PAR_JOUR: f64 = 2.0;
const PROD_BOIS_PAR_PERS_PAR_JOUR: f64 = 1.0;
const PROD_PIERRE_PAR_PERS_PAR_JOUR: f64 = 1.0;
const PROD_FER_PAR_PERS_PAR_JOUR: f64 = 0.5;
const PROD_OUTILS_PAR_PERS_PAR_JOUR: f64 = 0.5;
const PROD_RECHERCHE_PAR_PERS_PAR_JOUR: f64 = 0.1;
/// Production armes par personne à la forge et par jour.
const PROD_ARMES_PAR_FORGE_PAR_JOUR: f64 = 0.2;

/// Consommation nourriture : 1 / personne / jour.
const CONSO_NOURRITURE_PAR_PERS_PAR_JOUR: f64 = 1.0;

/// Pts construction par maçon et par jour (in-game).
/// @id: miyuclicker_idlesim_construction_pts_per_macon_per_day
const CONSTRUCTION_PTS_PER_MACON_PER_DAY: f64 = 1.0;

/// Coûts et pts construction (voir MiyuClicker - Batiments Macons et Construction).
/// Maison: 30b 20p 5f | 30 pts (+1% par maison)
const MAISON_COUT_BOIS: i64 = 30;
const MAISON_COUT_PIERRE: i64 = 20;
const MAISON_COUT_FER: i64 = 5;
const MAISON_PTS_BASE: f64 = 30.0;
const MAISON_PTS_PCT_PAR_MAISON: f64 = 0.01;

/// Caserne: 50b 100p 20f | 100 pts (+5% par niveau)
const CASERNE_COUT_BOIS: i64 = 50;
const CASERNE_COUT_PIERRE: i64 = 100;
const CASERNE_COUT_FER: i64 = 20;
const CASERNE_PTS_BASE: f64 = 100.0;
const CASERNE_PTS_PCT_PAR_NIVEAU: f64 = 0.05;

/// Grenier: 50b 50p 10f | 50 pts (+10% par niveau)
const GRENIER_COUT_BOIS: i64 = 50;
const GRENIER_COUT_PIERRE: i64 = 50;
const GRENIER_COUT_FER: i64 = 10;
const GRENIER_PTS_BASE: f64 = 50.0;
const GRENIER_PTS_PCT_PAR_NIVEAU: f64 = 0.10;

/// Dépôt: 30b 30p 10f | 50 pts (+5% par niveau)
const DEPOT_COUT_BOIS: i64 = 30;
const DEPOT_COUT_PIERRE: i64 = 30;
const DEPOT_COUT_FER: i64 = 10;
const DEPOT_PTS_BASE: f64 = 50.0;
const DEPOT_PTS_PCT_PAR_NIVEAU: f64 = 0.05;

/// Entrepôt: 40b 40p 20f | 100 pts (+5% par niveau)
const ENTREPOT_COUT_BOIS: i64 = 40;
const ENTREPOT_COUT_PIERRE: i64 = 40;
const ENTREPOT_COUT_FER: i64 = 20;
const ENTREPOT_PTS_BASE: f64 = 100.0;
const ENTREPOT_PTS_PCT_PAR_NIVEAU: f64 = 0.05;

/// Seuil (jours simulés) à 0 nourriture au-delà duquel Game Over (mort du seigneur).
/// @id: miyuclicker_idlesim_jours_game_over
/// @do: define_game_over_threshold_days
/// @role: data
/// @human: 0 nourriture pendant plus de 7 jours → Game Over.
pub const JOURS_NOURRITURE_ZERO_GAME_OVER: f64 = 7.0;

/// Baisse moral (bonheur) par jour jeu : nourriture < population.
/// @id: miyuclicker_idlesim_moral_baisse_1pct
const MORAL_BAISSE_1_PCT_PAR_JOUR: f64 = 0.01;

/// Baisse moral (bonheur) par jour jeu : nourriture ≤ population/2.
/// @id: miyuclicker_idlesim_moral_baisse_5pct
const MORAL_BAISSE_5_PCT_PAR_JOUR: f64 = 0.05;

/// Remontée moral (bonheur) par jour jeu quand nourriture suffisante.
const MORAL_REMONTEE_PAR_JOUR: f64 = 0.02;

/// Ajuste les valeurs pour qu'elles ne soient pas négatives.
/// @id: miyuclicker_idlesim_clamp_resources
/// @do: clamp_all_resources_non_negative
/// @role: mutator
fn clamp_resources(state: &mut GameState) {
    state.nourriture = state.nourriture.max(0.0);
    state.bois = state.bois.max(0.0);
    state.pierre = state.pierre.max(0.0);
    state.fer = state.fer.max(0.0);
    state.outils = state.outils.max(0.0);
    state.or = state.or.max(0);
    state.recherche = state.recherche.max(0);
    state.armes = state.armes.max(0);
    state.gens = state.gens.max(0);
    state.soldats = state.soldats.max(0);
    state.macons = state.macons.max(0);
}

/// Applique un gain au clic (Champs +1 nourriture, Village +1 pop -30 nourriture, Château +1 soldat -50 nourriture -5 armes, ressource +1).
/// @id: miyuclicker_idlesim_apply_click
/// @do: apply_click_gain_to_state
/// @role: mutator
pub fn apply_click(state: &mut GameState, target: ClickTarget) {
    match target {
        ClickTarget::Champs => {
            state.nourriture += CLIC_CHAMPS_GAIN;
        }
        ClickTarget::Bois => state.bois += CLIC_RESSOURCE_GAIN,
        ClickTarget::Pierre => state.pierre += CLIC_RESSOURCE_GAIN,
        ClickTarget::Fer => state.fer += CLIC_RESSOURCE_GAIN,
        ClickTarget::Ateliers => state.outils += CLIC_RESSOURCE_GAIN,
        ClickTarget::Chateau => {
            let cap_soldats = state.cap_soldats();
            if state.soldats < cap_soldats
                && state.nourriture >= CLIC_CHATEAU_COUT_NOURRITURE
                && state.armes >= CLIC_CHATEAU_COUT_ARMES
            {
                state.nourriture -= CLIC_CHATEAU_COUT_NOURRITURE;
                state.armes -= CLIC_CHATEAU_COUT_ARMES;
                state.soldats += 1;
            }
        }
        ClickTarget::Village => {
            if state.gens < state.cap_gens && state.nourriture >= CLIC_VILLAGE_COUT_NOURRITURE {
                state.nourriture -= CLIC_VILLAGE_COUT_NOURRITURE;
                state.gens += 1;
            }
        }
    }
    clamp_resources(state);
}

/// Transforme 1 pop (gens) en 1 maçon (Guilde des Maçons). Renvoie true si effectué.
/// @id: miyuclicker_idlesim_convert_pop_to_macon
/// @do: convert_one_pop_to_one_macon
/// @role: mutator
pub fn convert_pop_to_macon(state: &mut GameState) -> bool {
    if state.gens > 0 {
        state.gens -= 1;
        state.macons += 1;
        clamp_resources(state);
        true
    } else {
        false
    }
}

/// Transforme 1 maçon en 1 pop. Renvoie true si effectué.
/// @id: miyuclicker_idlesim_convert_macon_to_pop
/// @do: convert_one_macon_to_one_pop
/// @role: mutator
pub fn convert_macon_to_pop(state: &mut GameState) -> bool {
    if state.macons > 0 {
        state.macons -= 1;
        state.gens += 1;
        clamp_resources(state);
        true
    } else {
        false
    }
}

/// Vérifie que la somme des affectations ne dépasse pas le nombre de gens ; applique l'allocation.
/// @id: miyuclicker_idlesim_apply_allocation
/// @do: apply_worker_allocation_to_state
/// @role: mutator
pub fn apply_allocation(state: &mut GameState, allocation: &Allocation) -> Result<(), String> {
    let total = allocation.total();
    if total > state.gens {
        return Err(format!(
            "Affectation {} > gens {}",
            total,
            state.gens
        ));
    }
    state.allocation = allocation.clone();
    Ok(())
}

/// Applique l'allocation des maçons (somme <= macons).
/// @id: miyuclicker_idlesim_apply_allocation_macons
/// @do: apply_mason_allocation_to_state
/// @role: mutator
pub fn apply_allocation_macons(state: &mut GameState, allocation: &AllocationMacons) -> Result<(), String> {
    let total = allocation.total();
    if total > state.macons {
        return Err(format!("Allocation maçons {} > macons {}", total, state.macons));
    }
    state.allocation_macons = allocation.clone();
    Ok(())
}

/// Pts requis pour la prochaine maison (pour affichage UI).
pub fn pts_required_maison_pub(state: &GameState) -> f64 {
    pts_required_for_building(state, BuildingType::Maison)
}

/// Pts requis pour le prochain niveau caserne (pour affichage UI).
/// @id: miyuclicker_idlesim_pts_required_caserne_pub
/// @do: compute_construction_pts_required_for_next_barracks
/// @role: reader
pub fn pts_required_caserne_pub(state: &GameState) -> f64 {
    pts_required_for_building(state, BuildingType::Caserne)
}

/// Pts requis pour le prochain niveau grenier (pour affichage UI).
/// @id: miyuclicker_idlesim_pts_required_grenier_pub
/// @do: compute_construction_pts_required_for_next_granary
/// @role: reader
pub fn pts_required_grenier_pub(state: &GameState) -> f64 {
    pts_required_for_building(state, BuildingType::Grenier)
}

/// Pts requis pour le prochain niveau dépôt (pour affichage UI).
pub fn pts_required_depot_pub(state: &GameState) -> f64 {
    DEPOT_PTS_BASE * (1.0 + DEPOT_PTS_PCT_PAR_NIVEAU * (state.depot_lvl as f64))
}

/// Pts requis pour le prochain niveau entrepôt (pour affichage UI).
/// @id: miyuclicker_idlesim_pts_required_entrepot_pub
/// @do: compute_construction_pts_required_for_next_warehouse
/// @role: reader
pub fn pts_required_entrepot_pub(state: &GameState) -> f64 {
    pts_required_for_building(state, BuildingType::Entrepot)
}

/// Met à jour l'état sur un tick (production/consommation par jour, moral, cap).
/// @id: miyuclicker_idlesim_tick
/// @do: advance_game_time_and_apply_production_consumption
/// @role: mutator
/// @human: delta_s = secondes IRL ; game_speed = s in-game/s IRL (0 = pause).
pub fn tick(state: &mut GameState, delta_s: f64, game_speed: f64) {
    if game_speed <= 0.0 || state.game_over {
        return;
    }
    let game_seconds = (delta_s * game_speed).min(SECONDS_PER_DAY * 2.0);
    let a = &state.allocation;

    let factor = game_seconds / SECONDS_PER_DAY;

    state.nourriture += (a.champs as f64) * PROD_NOURRITURE_PAR_PERS_PAR_JOUR * factor;
    state.bois += (a.scierie as f64) * PROD_BOIS_PAR_PERS_PAR_JOUR * factor;
    state.pierre += (a.carriere as f64) * PROD_PIERRE_PAR_PERS_PAR_JOUR * factor;
    state.fer += (a.mine as f64) * PROD_FER_PAR_PERS_PAR_JOUR * factor;
    state.outils += (a.ateliers as f64) * PROD_OUTILS_PAR_PERS_PAR_JOUR * factor;
    state.armes += ((a.forge as f64) * PROD_ARMES_PAR_FORGE_PAR_JOUR * factor) as i64;
    state.recherche += ((a.recherche as f64) * PROD_RECHERCHE_PAR_PERS_PAR_JOUR * factor) as i64;

    let consommation = (state.gens as f64) * CONSO_NOURRITURE_PAR_PERS_PAR_JOUR * factor;
    state.nourriture -= consommation;

    // Évolution bonheur (moral) et Game Over — règles nourriture (voir MiyuClicker - Systeme Bonheur).
    // @id: miyuclicker_idlesim_tick_moral_bonheur
    // @do: update_moral_and_game_over_from_food
    // @role: mutator
    // @human: Nourriture < pop → -1 %/j ; ≤ pop/2 → -5 %/j ; 0 nourriture > 7 j → Game Over.
    if state.game_over {
        // Ne plus modifier moral / jours_nourriture_zero une fois Game Over.
    } else if state.nourriture <= 0.0 {
        state.jours_nourriture_zero += factor;
        if state.jours_nourriture_zero > JOURS_NOURRITURE_ZERO_GAME_OVER {
            state.game_over = true;
        }
        state.moral = (state.moral - MORAL_BAISSE_5_PCT_PAR_JOUR * factor).max(0.0);
        state.fecondite = (state.fecondite - 0.03 * factor).max(0.0);
    } else {
        state.jours_nourriture_zero = 0.0;
        let pop = state.gens as f64;
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

    state.cap_gens = state.cap_gens_from_maisons();
    if state.gens > state.cap_gens {
        state.gens = state.cap_gens;
    }
    let cap_soldats = state.cap_soldats();
    if state.soldats > cap_soldats {
        state.soldats = cap_soldats;
    }
    let cap_nourriture = state.cap_nourriture();
    if state.nourriture > cap_nourriture {
        state.nourriture = cap_nourriture;
    }
    let cap_mat = state.cap_matières();
    let total_mat = state.bois as i64 + state.pierre as i64 + state.fer as i64;
    if total_mat > cap_mat {
        let scale = cap_mat as f64 / total_mat as f64;
        state.bois *= scale;
        state.pierre *= scale;
        state.fer *= scale;
    }
    let cap_manuf = state.cap_manufacturés();
    let total_manuf = state.outils as i64 + state.armes;
    if total_manuf > cap_manuf {
        let scale = cap_manuf as f64 / total_manuf as f64;
        state.outils *= scale;
        state.armes = (state.armes as f64 * scale).round() as i64;
    }

    // Construction : chaque maçon alloué = 1 pt/jour sur le bâtiment.
    // @id: miyuclicker_idlesim_tick_construction_macons
    let am = &state.allocation_macons;
    state.construction_maison += (am.maison as f64) * CONSTRUCTION_PTS_PER_MACON_PER_DAY * factor;
    state.construction_caserne += (am.caserne as f64) * CONSTRUCTION_PTS_PER_MACON_PER_DAY * factor;
    state.construction_grenier += (am.grenier as f64) * CONSTRUCTION_PTS_PER_MACON_PER_DAY * factor;
    state.construction_depot += (am.depot as f64) * CONSTRUCTION_PTS_PER_MACON_PER_DAY * factor;
    state.construction_entrepot += (am.entrepot as f64) * CONSTRUCTION_PTS_PER_MACON_PER_DAY * factor;

    for bt in [
        BuildingType::Maison,
        BuildingType::Caserne,
        BuildingType::Grenier,
        BuildingType::Depot,
        BuildingType::Entrepot,
    ] {
        try_complete_building(state, bt);
    }

    state.temps_simule_s += game_seconds;
    clamp_resources(state);
}

/// Pts construction requis pour le prochain niveau / prochaine maison (par type).
/// @id: miyuclicker_idlesim_pts_required_for_building
/// @do: compute_construction_pts_required_by_type
/// @role: reader
fn pts_required_for_building(state: &GameState, bt: BuildingType) -> f64 {
    match bt {
        BuildingType::Maison => MAISON_PTS_BASE * (1.0 + MAISON_PTS_PCT_PAR_MAISON * (state.maisons as f64)),
        BuildingType::Caserne => CASERNE_PTS_BASE * (1.0 + CASERNE_PTS_PCT_PAR_NIVEAU * (state.caserne_lvl as f64)),
        BuildingType::Grenier => GRENIER_PTS_BASE * (1.0 + GRENIER_PTS_PCT_PAR_NIVEAU * (state.grenier_lvl as f64)),
        BuildingType::Depot => DEPOT_PTS_BASE * (1.0 + DEPOT_PTS_PCT_PAR_NIVEAU * (state.depot_lvl as f64)),
        BuildingType::Entrepot => ENTREPOT_PTS_BASE * (1.0 + ENTREPOT_PTS_PCT_PAR_NIVEAU * (state.entrepot_lvl as f64)),
    }
}

/// Tente de compléter un bâtiment si pts + ressources suffisants (factorisation try_complete_*).
/// @id: miyuclicker_idlesim_try_complete_building
/// @do: try_complete_building_by_type
/// @role: mutator
fn try_complete_building(state: &mut GameState, bt: BuildingType) {
    let required = pts_required_for_building(state, bt);
    let (bois_min, pierre_min, fer_min) = match bt {
        BuildingType::Maison => (MAISON_COUT_BOIS as f64, MAISON_COUT_PIERRE as f64, MAISON_COUT_FER as f64),
        BuildingType::Caserne => (CASERNE_COUT_BOIS as f64, CASERNE_COUT_PIERRE as f64, CASERNE_COUT_FER as f64),
        BuildingType::Grenier => (GRENIER_COUT_BOIS as f64, GRENIER_COUT_PIERRE as f64, GRENIER_COUT_FER as f64),
        BuildingType::Depot => (DEPOT_COUT_BOIS as f64, DEPOT_COUT_PIERRE as f64, DEPOT_COUT_FER as f64),
        BuildingType::Entrepot => (ENTREPOT_COUT_BOIS as f64, ENTREPOT_COUT_PIERRE as f64, ENTREPOT_COUT_FER as f64),
    };
    let (cur_pts, cur_bois, cur_pierre, cur_fer) = match bt {
        BuildingType::Maison => (
            &mut state.construction_maison,
            &mut state.bois,
            &mut state.pierre,
            &mut state.fer,
        ),
        BuildingType::Caserne => (
            &mut state.construction_caserne,
            &mut state.bois,
            &mut state.pierre,
            &mut state.fer,
        ),
        BuildingType::Grenier => (
            &mut state.construction_grenier,
            &mut state.bois,
            &mut state.pierre,
            &mut state.fer,
        ),
        BuildingType::Depot => (
            &mut state.construction_depot,
            &mut state.bois,
            &mut state.pierre,
            &mut state.fer,
        ),
        BuildingType::Entrepot => (
            &mut state.construction_entrepot,
            &mut state.bois,
            &mut state.pierre,
            &mut state.fer,
        ),
    };
    let done = match bt {
        BuildingType::Maison => {
            *cur_pts >= required && state.construction_maison_paid
        }
        _ => *cur_pts >= required && *cur_bois >= bois_min && *cur_pierre >= pierre_min && *cur_fer >= fer_min,
    };
    if done {
        if bt != BuildingType::Maison {
            *cur_bois -= bois_min;
            *cur_pierre -= pierre_min;
            *cur_fer -= fer_min;
        }
        *cur_pts -= required;
        if bt == BuildingType::Maison {
            state.construction_maison_paid = false;
        }
        match bt {
            BuildingType::Maison => state.maisons += 1,
            BuildingType::Caserne => state.caserne_lvl += 1,
            BuildingType::Grenier => state.grenier_lvl += 1,
            BuildingType::Depot => state.depot_lvl += 1,
            BuildingType::Entrepot => state.entrepot_lvl += 1,
        }
    }
}

/// Démarre la construction d'une maison : prélève le coût (30b 20p 5f) et marque comme payé. Renvoie true si effectué.
/// @id: miyuclicker_idlesim_start_construction_maison
/// @do: pay_and_start_house_construction
/// @role: mutator
pub fn start_construction_maison(state: &mut GameState) -> bool {
    if state.construction_maison_paid {
        return false;
    }
    if state.bois >= MAISON_COUT_BOIS as f64
        && state.pierre >= MAISON_COUT_PIERRE as f64
        && state.fer >= MAISON_COUT_FER as f64
    {
        state.bois -= MAISON_COUT_BOIS as f64;
        state.pierre -= MAISON_COUT_PIERRE as f64;
        state.fer -= MAISON_COUT_FER as f64;
        state.construction_maison_paid = true;
        clamp_resources(state);
        true
    } else {
        false
    }
}
