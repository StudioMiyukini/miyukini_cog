// @id: MGE-AnimPack-Naming @do: naming-convention @role: back-end @layer: 6 @human: francois
//! Frame naming convention: `{entity}_{action}_{dir}_{frame:03d}.png`.
//!
//! All animation frame filenames must follow this convention to be
//! processed by the packer. The direction component uses compass
//! abbreviations: `s`, `sw`, `w`, `nw`, `n`, `ne`, `e`, `se`.

use crate::errors::AnimPackError;

/// Valid direction abbreviations for 8-direction sprites.
pub const VALID_DIRECTIONS: &[&str] = &["s", "sw", "w", "nw", "n", "ne", "e", "se"];

/// Format a frame filename following the convention.
///
/// Returns `{entity}_{action}_{dir}_{frame:03d}.png`.
pub fn format_frame_name(entity: &str, action: &str, dir: &str, frame: u32) -> String {
    format!("{entity}_{action}_{dir}_{frame:03}.png")
}

/// Parse a frame filename into its components.
///
/// Expects `{entity}_{action}_{dir}_{frame:03d}.png`.
/// Returns `(entity, action, dir, frame_index)`.
pub fn parse_frame_name(filename: &str) -> Result<(String, String, String, u32), AnimPackError> {
    let stem = filename.strip_suffix(".png").ok_or_else(|| {
        AnimPackError::InvalidFrameName(format!("missing .png extension: '{filename}'"))
    })?;

    let parts: Vec<&str> = stem.split('_').collect();
    if parts.len() < 4 {
        return Err(AnimPackError::InvalidFrameName(format!(
            "expected at least 4 underscore-separated parts: '{filename}'"
        )));
    }

    // Last part is the frame index, second-to-last is direction.
    // Everything before that is entity + action.
    // Convention: entity_action_dir_frame — but entity and action could contain
    // underscores internally. We take the last two parts as dir and frame,
    // the second-to-last-but-one as action, and everything else as entity.
    let frame_str = parts[parts.len() - 1];
    let dir = parts[parts.len() - 2];
    let action = parts[parts.len() - 3];
    let entity_parts = &parts[..parts.len() - 3];

    if entity_parts.is_empty() {
        return Err(AnimPackError::InvalidFrameName(format!(
            "empty entity in filename: '{filename}'"
        )));
    }

    let entity = entity_parts.join("_");

    // Validate direction.
    if !VALID_DIRECTIONS.contains(&dir) {
        return Err(AnimPackError::InvalidFrameName(format!(
            "invalid direction '{dir}' in filename: '{filename}'"
        )));
    }

    // Parse frame index.
    let frame: u32 = frame_str.parse().map_err(|_| {
        AnimPackError::InvalidFrameName(format!(
            "invalid frame index '{frame_str}' in filename: '{filename}'"
        ))
    })?;

    Ok((entity, action.to_string(), dir.to_string(), frame))
}

/// Validate a list of filenames against the naming convention.
///
/// Returns a list of error messages for invalid names.
/// An empty return means all names are valid.
pub fn validate_naming(filenames: &[&str]) -> Vec<String> {
    filenames
        .iter()
        .filter_map(|name| match parse_frame_name(name) {
            Ok(_) => None,
            Err(e) => Some(format!("{name}: {e}")),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_frame_name_basic() {
        let name = format_frame_name("fallen", "idle", "s", 0);
        assert_eq!(name, "fallen_idle_s_000.png");
    }

    #[test]
    fn format_frame_name_multi_digit() {
        let name = format_frame_name("skeleton", "walk", "nw", 12);
        assert_eq!(name, "skeleton_walk_nw_012.png");
    }

    #[test]
    fn parse_frame_name_valid() {
        let (entity, action, dir, frame) = parse_frame_name("fallen_idle_s_000.png").unwrap();
        assert_eq!(entity, "fallen");
        assert_eq!(action, "idle");
        assert_eq!(dir, "s");
        assert_eq!(frame, 0);
    }

    #[test]
    fn parse_frame_name_compound_entity() {
        let (entity, action, dir, frame) =
            parse_frame_name("dark_elder_attack_ne_005.png").unwrap();
        assert_eq!(entity, "dark_elder");
        assert_eq!(action, "attack");
        assert_eq!(dir, "ne");
        assert_eq!(frame, 5);
    }

    #[test]
    fn parse_frame_name_invalid_no_extension() {
        let result = parse_frame_name("bad");
        assert!(result.is_err());
    }

    #[test]
    fn parse_frame_name_invalid_too_few_parts() {
        let result = parse_frame_name("bad.png");
        assert!(result.is_err());
    }

    #[test]
    fn parse_frame_name_invalid_direction() {
        let result = parse_frame_name("fallen_idle_x_000.png");
        assert!(result.is_err());
    }

    #[test]
    fn parse_frame_name_invalid_frame_index() {
        let result = parse_frame_name("fallen_idle_s_abc.png");
        assert!(result.is_err());
    }

    #[test]
    fn validate_naming_all_valid() {
        let names = &[
            "fallen_idle_s_000.png",
            "fallen_idle_sw_001.png",
            "fallen_walk_w_002.png",
            "fallen_attack_nw_003.png",
        ];
        let errors = validate_naming(names);
        assert!(errors.is_empty(), "Expected 0 errors, got: {errors:?}");
    }

    #[test]
    fn validate_naming_with_errors() {
        let names = &[
            "fallen_idle_s_000.png",
            "bad.png",
            "also_bad",
            "fallen_walk_w_002.png",
        ];
        let errors = validate_naming(names);
        assert_eq!(errors.len(), 2, "Expected 2 errors, got: {errors:?}");
    }

    #[test]
    fn roundtrip_format_parse() {
        let name = format_frame_name("zombie", "die", "se", 7);
        let (entity, action, dir, frame) = parse_frame_name(&name).unwrap();
        assert_eq!(entity, "zombie");
        assert_eq!(action, "die");
        assert_eq!(dir, "se");
        assert_eq!(frame, 7);
    }
}
