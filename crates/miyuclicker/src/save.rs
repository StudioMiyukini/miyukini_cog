//! Sauvegarde / chargement des 3 slots (JSON).
//! Toolkit MiyuClickerSave.
//!
//! @id: miyuclicker_save_module
//! @role: infrastructure
//! @layer: domain
//! @do: persist_and_load_game_state_slots

use crate::state::{Allocation, AllocationMacons, Cite, Deplacement, GameState, Route};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Structure sérialisable pour le fichier JSON (alignée Guide MVP).
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub version: String,
    pub resources: SaveResources,
    pub cap_moral: SaveCapMoral,
    #[serde(default)]
    pub buildings: SaveBuildings,
    pub allocation: Allocation,
    #[serde(default)]
    pub allocation_macons: AllocationMacons,
    pub carte: SaveCarte,
    pub meta: SaveMeta,
}

/// Ressources brutes sérialisées (or, gens, soldats, nourriture, matières, outils, armes).
/// @id: miyuclicker_save_save_resources
/// @do: represent_serialized_resources
/// @role: data
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveResources {
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
}

/// Cap, bonheur (moral), fécondité, Game Over.
/// @id: miyuclicker_save_cap_moral
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveCapMoral {
    pub cap_gens: i64,
    #[serde(default, alias = "habitations")]
    pub maisons: i64,
    pub moral: f64,
    pub fecondite: f64,
    #[serde(default)]
    pub jours_nourriture_zero: f64,
    #[serde(default)]
    pub game_over: bool,
}

/// Bâtiments, maçons, progression construction.
/// @id: miyuclicker_save_buildings
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SaveBuildings {
    #[serde(default)]
    pub maisons: i64,
    #[serde(default)]
    pub caserne_lvl: i64,
    #[serde(default)]
    pub grenier_lvl: i64,
    #[serde(default)]
    pub depot_lvl: i64,
    #[serde(default)]
    pub entrepot_lvl: i64,
    #[serde(default)]
    pub macons: i64,
    #[serde(default)]
    pub construction_maison: f64,
    #[serde(default)]
    pub construction_maison_paid: bool,
    #[serde(default)]
    pub construction_caserne: f64,
    #[serde(default)]
    pub construction_grenier: f64,
    #[serde(default)]
    pub construction_depot: f64,
    #[serde(default)]
    pub construction_entrepot: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveCarte {
    pub cites: Vec<Cite>,
    pub routes: Vec<Route>,
    pub deplacements: Vec<Deplacement>,
}

/// Métadonnées de sauvegarde (temps simulé, slot_id, version).
/// @id: miyuclicker_save_save_meta
/// @do: represent_serialized_meta
/// @role: data
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

/// Convertit GameState en SaveState pour sérialisation.
/// @id: miyuclicker_save_game_state_to_save
/// @do: convert_game_state_to_serializable
/// @role: mutator
fn game_state_to_save(state: &GameState) -> SaveState {
    SaveState {
        version: "1.0".to_string(),
        resources: SaveResources {
            or: state.or,
            gens: state.gens,
            soldats: state.soldats,
            recherche: state.recherche,
            nourriture: state.nourriture,
            bois: state.bois,
            pierre: state.pierre,
            fer: state.fer,
            outils: state.outils,
            armes: state.armes,
        },
        cap_moral: SaveCapMoral {
            cap_gens: state.cap_gens,
            maisons: state.maisons,
            moral: state.moral,
            fecondite: state.fecondite,
            jours_nourriture_zero: state.jours_nourriture_zero,
            game_over: state.game_over,
        },
        buildings: SaveBuildings {
            maisons: state.maisons,
            caserne_lvl: state.caserne_lvl,
            grenier_lvl: state.grenier_lvl,
            depot_lvl: state.depot_lvl,
            entrepot_lvl: state.entrepot_lvl,
            macons: state.macons,
            construction_maison: state.construction_maison,
            construction_maison_paid: state.construction_maison_paid,
            construction_caserne: state.construction_caserne,
            construction_grenier: state.construction_grenier,
            construction_depot: state.construction_depot,
            construction_entrepot: state.construction_entrepot,
        },
        allocation: state.allocation.clone(),
        allocation_macons: state.allocation_macons.clone(),
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

/// Reconstruit GameState depuis SaveState (rétrocompatibilité maisons/habitations).
/// @id: miyuclicker_save_save_to_game_state
/// @do: deserialize_save_to_game_state
/// @role: mutator
fn save_to_game_state(save: SaveState) -> GameState {
    let maisons = if save.buildings.maisons > 0 {
        save.buildings.maisons
    } else {
        save.cap_moral.maisons.max(1)
    };
    let caserne_lvl = save.buildings.caserne_lvl.max(1);
    let grenier_lvl = save.buildings.grenier_lvl.max(1);
    let depot_lvl = save.buildings.depot_lvl.max(1);
    let entrepot_lvl = save.buildings.entrepot_lvl.max(1);
    GameState {
        or: save.resources.or,
        gens: save.resources.gens,
        soldats: save.resources.soldats,
        recherche: save.resources.recherche,
        nourriture: save.resources.nourriture,
        bois: save.resources.bois,
        pierre: save.resources.pierre,
        fer: save.resources.fer,
        outils: save.resources.outils,
        armes: save.resources.armes,
        cap_gens: maisons * 4,
        maisons,
        caserne_lvl,
        grenier_lvl,
        depot_lvl,
        entrepot_lvl,
        macons: save.buildings.macons,
        construction_maison: save.buildings.construction_maison,
        construction_maison_paid: save.buildings.construction_maison_paid,
        construction_caserne: save.buildings.construction_caserne,
        construction_grenier: save.buildings.construction_grenier,
        construction_depot: save.buildings.construction_depot,
        construction_entrepot: save.buildings.construction_entrepot,
        allocation_macons: save.allocation_macons.clone(),
        moral: save.cap_moral.moral,
        fecondite: save.cap_moral.fecondite,
        jours_nourriture_zero: save.cap_moral.jours_nourriture_zero,
        game_over: save.cap_moral.game_over,
        allocation: save.allocation,
        cites: save.carte.cites,
        routes: save.carte.routes,
        route_duree_by_pair: None,
        deplacements: save.carte.deplacements,
        temps_simule_s: save.meta.temps_simule_s,
        slot_id: save.meta.slot_id,
        version_sauvegarde: save.meta.version_sauvegarde,
    }
}

/// Retourne le chemin du fichier pour un slot (dans le répertoire data fourni).
/// @id: miyuclicker_save_slot_path
/// @do: resolve_slot_file_path
/// @role: reader
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
                .map(|save| format!("Gens: {}, Or: {}", save.resources.gens, save.resources.or));
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
