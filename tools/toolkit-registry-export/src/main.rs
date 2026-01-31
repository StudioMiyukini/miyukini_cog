//! Exporte le registre des admin_cells des 49 Toolkits vers mscm_index/toolkit_registry.json.
//! MiyukiniAdmin charge ce fichier au démarrage pour découvrir et interroger les admin_cells.

use miyukini_admin::admin_cell::{AdminCell, AdminCellRef, ModuleInfo, ModuleType};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const VERSION: &str = "0.1.0";
const FINGERPRINT: &str = "toolkit-registry-export";

fn to_admin_cell<T: serde::Serialize>(cell: &T) -> Result<AdminCell, Box<dyn std::error::Error>> {
    let value = serde_json::to_value(cell)?;
    let admin_cell: AdminCell = serde_json::from_value(value)?;
    Ok(admin_cell)
}

#[derive(Debug, Serialize, Deserialize)]
struct RegistryEntry {
    module_info: ModuleInfo,
    admin_cell: AdminCell,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToolkitRegistry {
    entries: Vec<RegistryEntry>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = Vec::new();

    /// Enregistre un toolkit : appelle xxx_admin_cell, convertit en AdminCell, construit ModuleInfo.
    fn push_toolkit(
        entries: &mut Vec<RegistryEntry>,
        toolkit_id: &str,
        origin: &str,
        cell: &impl serde::Serialize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let admin_cell = to_admin_cell(cell)?;
        let module_info = ModuleInfo {
            id: toolkit_id.to_string(),
            version: VERSION.to_string(),
            module_type: ModuleType::Toolkit,
            module_origin: origin.to_string(),
            admin_cell_ref: AdminCellRef {
                module_id: toolkit_id.to_string(),
                ref_path: format!("crate:{}", toolkit_id),
            },
            locked: false,
        };
        entries.push(RegistryEntry {
            module_info,
            admin_cell,
        });
        Ok(())
    }

    push_toolkit(&mut entries, miyucalc::TOOLKIT_ID, "miyukini-miyucalc", &miyucalc::admin_cell::miyucalc_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyusql::TOOLKIT_ID, "miyukini-miyusql", &miyusql::admin_cell::miyusql_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyutext::TOOLKIT_ID, "miyukini-miyutext", &miyutext::admin_cell::miyutext_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuvalidate::TOOLKIT_ID, "miyukini-miyuvalidate", &miyuvalidate::admin_cell::miyuvalidate_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyulocale::TOOLKIT_ID, "miyukini-miyulocale", &miyulocale::admin_cell::miyulocale_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuexport::TOOLKIT_ID, "miyukini-miyuexport", &miyuexport::admin_cell::miyuexport_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyujobs::TOOLKIT_ID, "miyukini-miyujobs", &miyujobs::admin_cell::miyujobs_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyauth::TOOLKIT_ID, "miyukini-miyauth", &miyauth::admin_cell::miyauth_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuweb::TOOLKIT_ID, "miyukini-miyuweb", &miyuweb::admin_cell::miyuweb_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuclock::TOOLKIT_ID, "miyukini-miyuclock", &miyuclock::admin_cell::miyuclock_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuforum::TOOLKIT_ID, "miyukini-miyuforum", &miyuforum::admin_cell::miyuforum_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyupm::TOOLKIT_ID, "miyukini-miyupm", &miyupm::admin_cell::miyupm_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyunotify::TOOLKIT_ID, "miyukini-miyunotify", &miyunotify::admin_cell::miyunotify_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyusearch::TOOLKIT_ID, "miyukini-miyusearch", &miyusearch::admin_cell::miyusearch_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuwebway_participant::TOOLKIT_ID, "miyukini-miyuwebway-participant", &miyuwebway_participant::admin_cell::miyuwebway_participant_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuwebway_tracker::TOOLKIT_ID, "miyukini-miyuwebway-tracker", &miyuwebway_tracker::admin_cell::miyuwebway_tracker_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyubilling::TOOLKIT_ID, "miyukini-miyubilling", &miyubilling::admin_cell::miyubilling_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyubooking::TOOLKIT_ID, "miyukini-miyubooking", &miyubooking::admin_cell::miyubooking_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyucms::TOOLKIT_ID, "miyukini-miyucms", &miyucms::admin_cell::miyucms_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyumedia::TOOLKIT_ID, "miyukini-miyumedia", &miyumedia::admin_cell::miyumedia_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyushipping::TOOLKIT_ID, "miyukini-miyushipping", &miyushipping::admin_cell::miyushipping_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyustore::TOOLKIT_ID, "miyukini-miyustore", &miyustore::admin_cell::miyustore_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuwidgets::TOOLKIT_ID, "miyukini-miyuwidgets", &miyuwidgets::admin_cell::miyuwidgets_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuinvoice::TOOLKIT_ID, "miyukini-miyuinvoice", &miyuinvoice::admin_cell::miyuinvoice_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyucptaledger::TOOLKIT_ID, "miyukini-miyucptaledger", &miyucptaledger::admin_cell::miyucptaledger_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuexpense::TOOLKIT_ID, "miyukini-miyuexpense", &miyuexpense::admin_cell::miyuexpense_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyutreasury::TOOLKIT_ID, "miyukini-miyutreasury", &miyutreasury::admin_cell::miyutreasury_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuantispam::TOOLKIT_ID, "miyukini-miyuantispam", &miyuantispam::admin_cell::miyuantispam_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyubookmarks::TOOLKIT_ID, "miyukini-miyubookmarks", &miyubookmarks::admin_cell::miyubookmarks_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyucomptareports::TOOLKIT_ID, "miyukini-miyucomptareports", &miyucomptareports::admin_cell::miyucomptareports_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyucontacts::TOOLKIT_ID, "miyukini-miyucontacts", &miyucontacts::admin_cell::miyucontacts_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyudeclarations::TOOLKIT_ID, "miyukini-miyudeclarations", &miyudeclarations::admin_cell::miyudeclarations_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyudiscovery::TOOLKIT_ID, "miyukini-miyudiscovery", &miyudiscovery::admin_cell::miyudiscovery_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyufeeds::TOOLKIT_ID, "miyukini-miyufeeds", &miyufeeds::admin_cell::miyufeeds_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuhr::TOOLKIT_ID, "miyukini-miyuhr", &miyuhr::admin_cell::miyuhr_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyumoderationforum::TOOLKIT_ID, "miyukini-miyumoderationforum", &miyumoderationforum::admin_cell::miyumoderationforum_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyupolls::TOOLKIT_ID, "miyukini-miyupolls", &miyupolls::admin_cell::miyupolls_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuposanalytics::TOOLKIT_ID, "miyukini-miyuposanalytics", &miyuposanalytics::admin_cell::miyuposanalytics_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuposinventory::TOOLKIT_ID, "miyukini-miyuposinventory", &miyuposinventory::admin_cell::miyuposinventory_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuposkitchen::TOOLKIT_ID, "miyukini-miyuposkitchen", &miyuposkitchen::admin_cell::miyuposkitchen_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuposloyalty::TOOLKIT_ID, "miyukini-miyuposloyalty", &miyuposloyalty::admin_cell::miyuposloyalty_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyupospayment::TOOLKIT_ID, "miyukini-miyupospayment", &miyupospayment::admin_cell::miyupospayment_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyupossales::TOOLKIT_ID, "miyukini-miyupossales", &miyupossales::admin_cell::miyupossales_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyuprofile::TOOLKIT_ID, "miyukini-miyuprofile", &miyuprofile::admin_cell::miyuprofile_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyusocialfeed::TOOLKIT_ID, "miyukini-miyusocialfeed", &miyusocialfeed::admin_cell::miyusocialfeed_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyusocialmessaging::TOOLKIT_ID, "miyukini-miyusocialmessaging", &miyusocialmessaging::admin_cell::miyusocialmessaging_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyusocialmoderation::TOOLKIT_ID, "miyukini-miyusocialmoderation", &miyusocialmoderation::admin_cell::miyusocialmoderation_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyusocialprofile::TOOLKIT_ID, "miyukini-miyusocialprofile", &miyusocialprofile::admin_cell::miyusocialprofile_admin_cell(VERSION, FINGERPRINT))?;
    push_toolkit(&mut entries, miyustory::TOOLKIT_ID, "miyukini-miyustory", &miyustory::admin_cell::miyustory_admin_cell(VERSION, FINGERPRINT))?;

    let registry = ToolkitRegistry { entries };
    let out_dir = Path::new("mscm_index");
    fs::create_dir_all(out_dir)?;
    let path = out_dir.join("toolkit_registry.json");
    let json = serde_json::to_string_pretty(&registry)?;
    fs::write(&path, json)?;
    println!("Registre des 49 Toolkits exporté vers {:?}", path);
    Ok(())
}
