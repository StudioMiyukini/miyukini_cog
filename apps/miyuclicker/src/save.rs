//! Sauvegarde / chargement des 3 slots (JSON).
//! Toolkit MiyuClickerSave — format v2.0 aligné sur le nouveau GameState.
//!
//! @id: miyuclicker_save_module
//! @role: infrastructure
//! @layer: app
//! @do: persist_and_load_game_state_slots
//! @human: 3 slots, JSON pretty-printed. Version 2.0 : ouvriers/bâtisseurs/soldats, 6 ressources, 3 bâtiments.

use crate::state::{
    Allocation, AllocationBatisseurs, Cite, Deplacement, GameState, Route,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════════════════
// Structures de sauvegarde (sérialisables)
// ═══════════════════════════════════════════════════════════════════════════════

/// Structure racine sérialisable pour le fichier JSON.
/// @id: miyuclicker_save_save_state
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub version: String,
    pub pseudo: String,
    pub resources: SaveResources,
    pub population: SavePopulation,
    pub buildings: SaveBuildings,
    pub allocation: Allocation,
    pub allocation_batisseurs: AllocationBatisseurs,
    pub moral: SaveMoral,
    pub carte: SaveCarte,
    pub meta: SaveMeta,
}

/// Ressources sérialisées (6 types, toutes f64).
/// @id: miyuclicker_save_save_resources
/// @do: represent_serialized_resources
/// @role: data
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveResources {
    pub nourriture: f64,
    pub bois: f64,
    pub pierre: f64,
    pub metal: f64,
    pub outils: f64,
    pub armes: f64,
}

/// Population sérialisée (3 rôles).
/// @id: miyuclicker_save_save_population
/// @do: represent_serialized_population
/// @role: data
#[derive(Debug, Serialize, Deserialize)]
pub struct SavePopulation {
    pub ouvriers: i64,
    pub batisseurs: i64,
    pub soldats: i64,
}

/// Bâtiments sérialisés (niveaux + construction en cours).
/// @id: miyuclicker_save_save_buildings
/// @do: represent_serialized_buildings
/// @role: data
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveBuildings {
    pub maisons_lvl: i64,
    pub caserne_lvl: i64,
    pub guilde_lvl: i64,
    #[serde(default)]
    pub construction_maison: f64,
    #[serde(default)]
    pub construction_maison_paid: bool,
    #[serde(default)]
    pub construction_caserne: f64,
    #[serde(default)]
    pub construction_caserne_paid: bool,
    #[serde(default)]
    pub construction_guilde: f64,
    #[serde(default)]
    pub construction_guilde_paid: bool,
}

/// Moral / bonheur / croissance sérialisés.
/// @id: miyuclicker_save_save_moral
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMoral {
    pub moral: f64,
    pub fecondite: f64,
    #[serde(default)]
    pub growth_accumulator: f64,
    #[serde(default)]
    pub jours_nourriture_zero: f64,
    #[serde(default)]
    pub game_over: bool,
}

/// Carte sérialisée (cités, routes, déplacements).
/// @id: miyuclicker_save_save_carte
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveCarte {
    pub cites: Vec<Cite>,
    pub routes: Vec<Route>,
    pub deplacements: Vec<Deplacement>,
}

/// Métadonnées de sauvegarde.
/// @id: miyuclicker_save_save_meta
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMeta {
    pub temps_simule_s: f64,
    pub slot_id: u8,
    pub version_sauvegarde: String,
}

/// Métadonnées d'un slot (pour l'écran Slots).
/// @id: miyuclicker_save_slot_metadata
/// @do: represent_slot_display_metadata
/// @role: data
#[derive(Debug, Clone)]
pub struct SlotMetadata {
    pub slot_id: u8,
    pub occupied: bool,
    pub saved_at: Option<String>,
    pub summary: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Conversions GameState ↔ SaveState
// ═══════════════════════════════════════════════════════════════════════════════

/// Convertit GameState en SaveState pour sérialisation.
/// @id: miyuclicker_save_game_state_to_save
/// @do: convert_game_state_to_serializable
/// @role: mutator
fn game_state_to_save(state: &GameState) -> SaveState {
    SaveState {
        version: "2.0".to_string(),
        pseudo: state.pseudo.clone(),
        resources: SaveResources {
            nourriture: state.nourriture,
            bois: state.bois,
            pierre: state.pierre,
            metal: state.metal,
            outils: state.outils,
            armes: state.armes,
        },
        population: SavePopulation {
            ouvriers: state.ouvriers,
            batisseurs: state.batisseurs,
            soldats: state.soldats,
        },
        buildings: SaveBuildings {
            maisons_lvl: state.maisons_lvl,
            caserne_lvl: state.caserne_lvl,
            guilde_lvl: state.guilde_lvl,
            construction_maison: state.construction_maison,
            construction_maison_paid: state.construction_maison_paid,
            construction_caserne: state.construction_caserne,
            construction_caserne_paid: state.construction_caserne_paid,
            construction_guilde: state.construction_guilde,
            construction_guilde_paid: state.construction_guilde_paid,
        },
        allocation: state.allocation.clone(),
        allocation_batisseurs: state.allocation_batisseurs.clone(),
        moral: SaveMoral {
            moral: state.moral,
            fecondite: state.fecondite,
            growth_accumulator: state.growth_accumulator,
            jours_nourriture_zero: state.jours_nourriture_zero,
            game_over: state.game_over,
        },
        carte: SaveCarte {
            cites: state.cites.clone(),
            routes: state.routes.clone(),
            deplacements: state.deplacements.clone(),
        },
        meta: SaveMeta {
            temps_simule_s: state.temps_simule_s,
            slot_id: state.slot_id,
            version_sauvegarde: state.version_sauvegarde.clone(),
        },
    }
}

/// Reconstruit GameState depuis SaveState.
/// @id: miyuclicker_save_save_to_game_state
/// @do: deserialize_save_to_game_state
/// @role: mutator
fn save_to_game_state(save: SaveState) -> GameState {
    GameState {
        nourriture: save.resources.nourriture,
        bois: save.resources.bois,
        pierre: save.resources.pierre,
        metal: save.resources.metal,
        outils: save.resources.outils,
        armes: save.resources.armes,

        ouvriers: save.population.ouvriers,
        batisseurs: save.population.batisseurs,
        soldats: save.population.soldats,

        maisons_lvl: save.buildings.maisons_lvl.max(1),
        caserne_lvl: save.buildings.caserne_lvl.max(1),
        guilde_lvl: save.buildings.guilde_lvl.max(1),

        construction_maison: save.buildings.construction_maison,
        construction_maison_paid: save.buildings.construction_maison_paid,
        construction_caserne: save.buildings.construction_caserne,
        construction_caserne_paid: save.buildings.construction_caserne_paid,
        construction_guilde: save.buildings.construction_guilde,
        construction_guilde_paid: save.buildings.construction_guilde_paid,

        allocation: save.allocation,
        allocation_batisseurs: save.allocation_batisseurs,

        moral: save.moral.moral,
        fecondite: save.moral.fecondite,
        growth_accumulator: save.moral.growth_accumulator,
        jours_nourriture_zero: save.moral.jours_nourriture_zero,
        game_over: save.moral.game_over,

        cites: save.carte.cites,
        routes: save.carte.routes,
        route_duree_by_pair: None,
        deplacements: save.carte.deplacements,

        pseudo: save.pseudo,
        temps_simule_s: save.meta.temps_simule_s,
        slot_id: save.meta.slot_id,
        version_sauvegarde: save.meta.version_sauvegarde,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// API publique
// ═══════════════════════════════════════════════════════════════════════════════

/// Retourne le chemin du fichier pour un slot (dans le répertoire data fourni).
/// @id: miyuclicker_save_slot_path
fn slot_path(data_dir: &Path, slot_id: u8) -> std::path::PathBuf {
    data_dir.join(format!("miyuclicker_slot_{slot_id}.json"))
}

/// Écrit l'état dans le slot (1, 2 ou 3).
/// @id: miyuclicker_save_slot_write
/// @do: serialize_and_write_game_state_to_slot_file
/// @role: mutator
pub fn slot_write(data_dir: &Path, slot_id: u8, state: &GameState) -> Result<(), String> {
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
    }
    let path = slot_path(data_dir, slot_id);
    let save = game_state_to_save(state);
    let json = serde_json::to_string_pretty(&save).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Lit l'état du slot (1, 2 ou 3).
/// @id: miyuclicker_save_slot_read
/// @do: read_and_deserialize_slot_file_to_game_state
/// @role: reader
pub fn slot_read(data_dir: &Path, slot_id: u8) -> Result<GameState, String> {
    let path = slot_path(data_dir, slot_id);
    if !path.exists() {
        return Err("Slot vide".to_string());
    }
    let json = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let save: SaveState = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(save_to_game_state(save))
}

/// Liste les métadonnées des 3 slots.
/// @id: miyuclicker_save_slot_list
/// @do: list_slot_metadata_for_ui
/// @role: reader
#[must_use]
pub fn slot_list(data_dir: &Path) -> Vec<SlotMetadata> {
    let mut out = Vec::with_capacity(3);
    for slot_id in 1..=3 {
        let path = slot_path(data_dir, slot_id);
        let (occupied, saved_at, summary) = if path.exists() {
            let meta = fs::metadata(&path).ok();
            let modified = meta.and_then(|m| m.modified().ok());
            let saved_at = modified.map(|t| format!("{t:?}"));
            let summary = fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str::<SaveState>(&s).ok())
                .map(|save| {
                    format!(
                        "Pop: {}/{}, Ouvriers: {}, Bâtisseurs: {}, Soldats: {}",
                        save.population.ouvriers + save.population.batisseurs + save.population.soldats,
                        save.buildings.maisons_lvl * 4,
                        save.population.ouvriers,
                        save.population.batisseurs,
                        save.population.soldats,
                    )
                });
            (true, saved_at, summary)
        } else {
            (false, None, None)
        };
        out.push(SlotMetadata {
            slot_id,
            occupied,
            saved_at,
            summary,
        });
    }
    out
}
