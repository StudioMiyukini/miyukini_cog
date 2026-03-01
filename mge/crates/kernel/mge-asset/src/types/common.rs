//! Types partages entre les differents domaines de donnees.
//!
//! <!-- @id: sd-data-common @do: define @role: arpg @layer: 3 @human: miyuk -->

use serde::{Deserialize, Serialize};

/// Valeur de statistique generique, utilisee dans les affixes, uniques, sets, etc.
///
/// Un `StatValue` peut representer :
/// - une valeur fixe (`value = Some(x)`)
/// - une plage aleatoire (`min = Some(a), max = Some(b)`)
/// - les deux (bonus fixe avec plage variable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatValue {
    /// Nom de la statistique (ex: `"enhanced_damage"`, `"life"`, `"fire_resist"`).
    pub stat: String,
    /// Valeur fixe (si applicable).
    #[serde(default)]
    pub value: Option<i32>,
    /// Borne minimale de la plage aleatoire.
    #[serde(default)]
    pub min: Option<i32>,
    /// Borne maximale de la plage aleatoire.
    #[serde(default)]
    pub max: Option<i32>,
}
