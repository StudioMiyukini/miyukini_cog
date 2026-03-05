//! Modèle carte : cités, routes, déplacements, résolution arrivée.
//! Toolkit MiyuClickerCarte.
//!
//! @role: mutator
//! @layer: domain
//! @do: manage_map_movements_and_combat_arrival

use crate::combat;
use crate::state::{Cite, Deplacement, GameState, Proprietaire, Route};
use std::collections::HashMap;

/// Enregistre un déplacement de troupes (from → to) et déduit les troupes de la cité source.
/// @id: miyuclicker_carte_move_troops
/// @do: register_troop_movement_and_deduct_from_source
/// @role: mutator
pub fn move_troops(
    state: &mut GameState,
    from_cite_id: &str,
    to_cite_id: &str,
    count: i64,
) -> Result<(), String> {
    if count <= 0 {
        return Err("Nombre de troupes invalide".to_string());
    }
    let from_idx = state
        .cites
        .iter()
        .position(|c| c.id == from_cite_id)
        .ok_or_else(|| "Cité source introuvable".to_string())?;
    let to_exists = state.cites.iter().any(|c| c.id == to_cite_id);
    if !to_exists {
        return Err("Cité cible introuvable".to_string());
    }
    let from_troupes = state.cites[from_idx].troupes;
    if from_troupes < count {
        return Err(format!("Pas assez de troupes ({from_troupes} disponibles)"));
    }
    state.cites[from_idx].troupes -= count;
    let to_cite = state.cites.iter().find(|c| c.id == to_cite_id).unwrap();
    let attaquant = matches!(to_cite.proprietaire, Proprietaire::Adverse(_));
    let _duree = state
        .routes
        .iter()
        .find(|r| {
            (r.cite_a == from_cite_id && r.cite_b == to_cite_id)
                || (r.cite_a == to_cite_id && r.cite_b == from_cite_id)
        })
        .map_or(10.0, |r| r.duree_s);
    state.deplacements.push(Deplacement {
        from_cite_id: from_cite_id.to_string(),
        to_cite_id: to_cite_id.to_string(),
        troupes: count,
        progress: 0.0,
        attaquant,
    });
    Ok(())
}

/// Avance les déplacements et résout les arrivées (combat ou renfort).
/// @id: miyuclicker_carte_model_update
/// @do: advance_movements_and_resolve_arrivals
/// @role: mutator
pub fn model_update(state: &mut GameState, delta_s: f64) {
    if state.route_duree_by_pair.is_none() {
        state.route_duree_by_pair = Some(build_route_duree_map(&state.routes));
    }
    let routes_map = state.route_duree_by_pair.as_ref().unwrap();

    let mut to_remove = Vec::new();
    for (i, dep) in state.deplacements.iter_mut().enumerate() {
        let key = (dep.from_cite_id.clone(), dep.to_cite_id.clone());
        let duree = routes_map.get(&key).copied().unwrap_or(10.0);
        dep.progress += delta_s / duree;
        if dep.progress >= 1.0 {
            to_remove.push(i);
        }
    }
    for i in to_remove.into_iter().rev() {
        resolve_arrival(state, i);
    }
}

/// Construit le map (cite_a, cite_b) -> duree_s (sens A→B et B→A).
fn build_route_duree_map(routes: &[Route]) -> HashMap<(String, String), f64> {
    routes
        .iter()
        .flat_map(|r| {
            [
                ((r.cite_a.clone(), r.cite_b.clone()), r.duree_s),
                ((r.cite_b.clone(), r.cite_a.clone()), r.duree_s),
            ]
        })
        .collect()
}

/// Traite l'arrivée d'un déplacement (combat si cité adverse, sinon renfort).
/// @id: miyuclicker_carte_resolve_arrival
/// @do: resolve_movement_arrival_combat_or_reinforce
/// @role: mutator
fn resolve_arrival(state: &mut GameState, deplacement_index: usize) {
    let dep = state.deplacements.remove(deplacement_index);
    let Some(to_idx) = state.cites.iter().position(|c| c.id == dep.to_cite_id) else {
        return;
    };
    let mut to_cite = state.cites[to_idx].clone();
    if dep.attaquant {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let result = combat::resolve(dep.troupes, to_cite.troupes, 1.0, 1.0, seed);
        to_cite.troupes = result.troupes_def_restantes;
        match result.vainqueur {
            Proprietaire::Joueur => {
                to_cite.proprietaire = Proprietaire::Joueur;
                to_cite.troupes = result.troupes_att_restantes;
                if let Some(c) = state.cites.get_mut(to_idx) {
                    *c = to_cite;
                }
            }
            Proprietaire::Adverse(_) => {
                if let Some(c) = state.cites.get_mut(to_idx) {
                    c.troupes = result.troupes_def_restantes;
                }
            }
        }
    } else {
        to_cite.troupes += dep.troupes;
        if let Some(c) = state.cites.get_mut(to_idx) {
            c.troupes = to_cite.troupes;
        }
    }
}

/// Génère une carte MVP minimale : 1 cité joueur + 2 cités adverses + routes.
/// @id: miyuclicker_carte_generate_mvp
/// @do: generate_minimal_map_cities_routes
/// @role: mutator
pub fn generate_carte_mvp(state: &mut GameState) {
    if !state.cites.is_empty() {
        return;
    }
    state.cites = vec![
        Cite {
            id: "ma_cite".to_string(),
            nom: "Ma cité".to_string(),
            x: 0.3,
            y: 0.5,
            proprietaire: Proprietaire::Joueur,
            troupes: 0,
        },
        Cite {
            id: "adverse_1".to_string(),
            nom: "Cité du Nord".to_string(),
            x: 0.6,
            y: 0.3,
            proprietaire: Proprietaire::Adverse(1),
            troupes: 5,
        },
        Cite {
            id: "adverse_2".to_string(),
            nom: "Cité de l'Est".to_string(),
            x: 0.7,
            y: 0.6,
            proprietaire: Proprietaire::Adverse(2),
            troupes: 8,
        },
    ];
    state.routes = vec![
        Route {
            cite_a: "ma_cite".to_string(),
            cite_b: "adverse_1".to_string(),
            duree_s: 8.0,
        },
        Route {
            cite_a: "ma_cite".to_string(),
            cite_b: "adverse_2".to_string(),
            duree_s: 12.0,
        },
    ];
    state.route_duree_by_pair = None;
}
