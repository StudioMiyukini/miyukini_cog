//! Validation cross-reference du registre de donnees.
//!
//! <!-- @id: sd-data-validation @do: define @role: arpg @layer: 3 @human: miyuk -->
//!
//! Verifie la coherence des references croisees entre les differents
//! domaines de donnees (skills, monstres, zones, items, quetes).

use crate::registry::GameDataRegistry;
use crate::DataLoadError;

/// Valide la coherence des references croisees dans le registre.
///
/// Appele une fois apres le chargement complet de toutes les donnees.
/// Retourne une erreur contenant la liste de toutes les incoherences trouvees.
pub fn validate_registry(registry: &GameDataRegistry) -> Result<(), DataLoadError> {
    let mut errors = Vec::new();

    validate_skills(registry, &mut errors);
    validate_monsters(registry, &mut errors);
    validate_zones(registry, &mut errors);
    validate_uniques(registry, &mut errors);
    validate_runewords(registry, &mut errors);
    validate_quests(registry, &mut errors);
    validate_super_uniques(registry, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(DataLoadError::ValidationErrors { errors })
    }
}

/// Valide les references des skills (synergies et prerequisites).
fn validate_skills(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    for (skill_id, skill_def) in &registry.skills {
        // Verifier que chaque synergie reference un skill existant.
        for synergy in &skill_def.synergies {
            if !registry.skills.contains_key(&synergy.skill_id) {
                errors.push(format!(
                    "Skill '{skill_id}': synergy '{}' references unknown skill",
                    synergy.skill_id
                ));
            }
        }

        // Verifier les prerequisites.
        for prereq in &skill_def.prerequisites.skills_required {
            if !registry.skills.contains_key(prereq) {
                errors.push(format!(
                    "Skill '{skill_id}': prerequisite '{prereq}' references unknown skill"
                ));
            }
        }
    }
}

/// Valide les references des monstres (loot tables).
fn validate_monsters(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    for (monster_id, monster_def) in &registry.monsters {
        if !registry
            .treasure_classes
            .contains_key(&monster_def.drops.loot_table)
        {
            errors.push(format!(
                "Monster '{monster_id}': loot_table '{}' references unknown treasure class",
                monster_def.drops.loot_table
            ));
        }
    }
}

/// Valide les references des zones (monstres, connexions).
fn validate_zones(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    for (zone_id, zone_def) in &registry.zones {
        // Verifier les spawn groups.
        for spawn in &zone_def.spawn_groups {
            if !registry.monsters.contains_key(&spawn.monster_id) {
                errors.push(format!(
                    "Zone '{zone_id}': spawn group monster '{}' references unknown monster",
                    spawn.monster_id
                ));
            }
        }

        // Verifier les connexions directionnelles.
        let connections = [
            ("north", &zone_def.connections.north),
            ("south", &zone_def.connections.south),
            ("east", &zone_def.connections.east),
            ("west", &zone_def.connections.west),
            ("cave", &zone_def.connections.cave),
            ("dungeon", &zone_def.connections.dungeon),
        ];

        for (dir, conn) in &connections {
            if let Some(target_id) = conn {
                if !registry.zones.contains_key(target_id) {
                    errors.push(format!(
                        "Zone '{zone_id}': {dir} connection to '{target_id}' references unknown zone"
                    ));
                }
            }
        }
    }
}

/// Valide les references des items uniques (base_type).
fn validate_uniques(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    for (unique_id, unique_def) in &registry.unique_items {
        if !unique_def.base_type.is_empty()
            && !registry.item_bases.contains_key(&unique_def.base_type)
        {
            errors.push(format!(
                "Unique '{unique_id}': base_type '{}' references unknown item base",
                unique_def.base_type
            ));
        }
    }
}

/// Valide les references des runewords (runes).
fn validate_runewords(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    for runeword in &registry.runewords {
        for rune_name in &runeword.rune_sequence {
            let rune_id = rune_name.to_lowercase();
            if !registry.runes.contains_key(&rune_id) {
                errors.push(format!(
                    "Runeword '{}': rune '{}' not found in rune definitions",
                    runeword.id, rune_name
                ));
            }
        }
    }
}

/// Valide les references des quetes (prerequisites).
fn validate_quests(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    // Note : les quetes n'ont pas de prerequisite_quests dans le schema actuel
    // (SD-Tech-Data-Schemas). La validation est prete pour quand elles en auront.
    let _ = registry;
    let _ = errors;
}

/// Valide les references des super-uniques (monstre de base, zone).
fn validate_super_uniques(registry: &GameDataRegistry, errors: &mut Vec<String>) {
    for (su_id, su_def) in &registry.super_uniques {
        if !registry.monsters.contains_key(&su_def.base_monster) {
            errors.push(format!(
                "SuperUnique '{su_id}': base_monster '{}' references unknown monster",
                su_def.base_monster
            ));
        }
        if !registry.zones.contains_key(&su_def.zone) {
            errors.push(format!(
                "SuperUnique '{su_id}': zone '{}' references unknown zone",
                su_def.zone
            ));
        }
    }
}
