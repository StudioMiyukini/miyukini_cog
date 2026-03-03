// @id: MGE-Save-Characters @do: character-persistence @role: back-end @layer: 2 @human: miyuk
//! Persistance des personnages joueurs.
//!
//! Fournit deux couches :
//! - `CharacterSave` / `CharacterSummary` — structs de serialisation JSON portables
//! - `CharacterDal` — DAL SQLite CRUD (creation, listing, recherche, sauvegarde des stats)
//!
//! Les fonctions `save_character` / `load_character` permettent un export/import JSON
//! independant de la base (fichiers de sauvegarde portables, backup, etc.).

use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult, PersistenceError};

/// DAL pour les personnages joueurs.
pub struct CharacterDal<'a>(pub &'a DbPool);

/// Representation d'un personnage en base.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CharacterRow {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Compte proprietaire.
    pub account_id: String,
    /// Nom du personnage.
    pub name: String,
    /// Classe du personnage.
    pub class: String,
    /// Niveau courant.
    pub level: i32,
    /// Experience accumulee.
    pub experience: i64,
    /// Force.
    pub strength: i32,
    /// Dexterite.
    pub dexterity: i32,
    /// Vitalite.
    pub vitality: i32,
    /// Energie.
    pub energy: i32,
    /// Points de stats non depenses.
    pub unspent_stats: i32,
    /// Points de vie courants.
    pub current_life: i32,
    /// Points de vie maximum.
    pub max_life: i32,
    /// Points de mana courants.
    pub current_mana: i32,
    /// Points de mana maximum.
    pub max_mana: i32,
    /// Or en possession.
    pub gold: i64,
    /// Zone courante.
    pub zone_id: String,
    /// Position X dans la zone.
    pub pos_x: f32,
    /// Position Y dans la zone.
    pub pos_y: f32,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Derniere date de jeu (ISO 8601) ou `None`.
    pub last_played: Option<String>,
}

impl CharacterDal<'_> {
    /// Cree un nouveau personnage niveau 1 avec les stats par defaut.
    pub fn create(
        &self,
        account_id: &str,
        name: &str,
        class: &str,
    ) -> PersistResult<CharacterRow> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let row = CharacterRow {
            id: id.clone(),
            account_id: account_id.to_string(),
            name: name.to_string(),
            class: class.to_string(),
            level: 1,
            experience: 0,
            strength: 10,
            dexterity: 10,
            vitality: 10,
            energy: 10,
            unspent_stats: 0,
            current_life: 80,
            max_life: 80,
            current_mana: 20,
            max_mana: 20,
            gold: 0,
            zone_id: "rogue_encampment".to_string(),
            pos_x: 0.0,
            pos_y: 0.0,
            created_at: now.clone(),
            last_played: None,
        };
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO characters
                 (id, account_id, name, class, level, experience,
                  strength, dexterity, vitality, energy, unspent_stats,
                  current_life, max_life, current_mana, max_mana,
                  gold, zone_id, pos_x, pos_y, created_at)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20)",
                params![
                    row.id, row.account_id, row.name, row.class, row.level, row.experience,
                    row.strength, row.dexterity, row.vitality, row.energy, row.unspent_stats,
                    row.current_life, row.max_life, row.current_mana, row.max_mana,
                    row.gold, row.zone_id, row.pos_x, row.pos_y, row.created_at
                ],
            )?;
            Ok(row)
        })
    }

    /// Liste tous les personnages d'un compte, tries par dernier jeu decroissant.
    pub fn list_for_account(&self, account_id: &str) -> PersistResult<Vec<CharacterRow>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, account_id, name, class, level, experience,
                        strength, dexterity, vitality, energy, unspent_stats,
                        current_life, max_life, current_mana, max_mana,
                        gold, zone_id, pos_x, pos_y, created_at, last_played
                 FROM characters WHERE account_id = ?1 ORDER BY last_played DESC",
            )?;
            let rows = stmt
                .query_map(params![account_id], Self::map_row)?
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
    }

    /// Recherche un personnage par son identifiant.
    ///
    /// Retourne `PersistenceError::NotFound` si aucun personnage ne correspond.
    pub fn find(&self, character_id: &str) -> PersistResult<CharacterRow> {
        self.0.with(|conn| {
            conn.query_row(
                "SELECT id, account_id, name, class, level, experience,
                        strength, dexterity, vitality, energy, unspent_stats,
                        current_life, max_life, current_mana, max_mana,
                        gold, zone_id, pos_x, pos_y, created_at, last_played
                 FROM characters WHERE id = ?1",
                params![character_id],
                Self::map_row,
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => PersistenceError::NotFound {
                    entity: "Character",
                    id: character_id.to_string(),
                },
                other => other.into(),
            })
        })
    }

    /// Sauvegarde les stats mutables d'un personnage existant.
    ///
    /// Met a jour `last_played` a l'instant courant.
    pub fn save(&self, ch: &CharacterRow) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "UPDATE characters SET
                    level=?1, experience=?2, strength=?3, dexterity=?4,
                    vitality=?5, energy=?6, unspent_stats=?7,
                    current_life=?8, max_life=?9, current_mana=?10, max_mana=?11,
                    gold=?12, zone_id=?13, pos_x=?14, pos_y=?15, last_played=?16
                 WHERE id=?17",
                params![
                    ch.level, ch.experience, ch.strength, ch.dexterity,
                    ch.vitality, ch.energy, ch.unspent_stats,
                    ch.current_life, ch.max_life, ch.current_mana, ch.max_mana,
                    ch.gold, ch.zone_id, ch.pos_x, ch.pos_y, now,
                    ch.id
                ],
            )?;
            Ok(())
        })
    }

    /// Mappe une ligne SQLite vers un `CharacterRow`.
    fn map_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<CharacterRow> {
        Ok(CharacterRow {
            id: row.get(0)?,
            account_id: row.get(1)?,
            name: row.get(2)?,
            class: row.get(3)?,
            level: row.get(4)?,
            experience: row.get(5)?,
            strength: row.get(6)?,
            dexterity: row.get(7)?,
            vitality: row.get(8)?,
            energy: row.get(9)?,
            unspent_stats: row.get(10)?,
            current_life: row.get(11)?,
            max_life: row.get(12)?,
            current_mana: row.get(13)?,
            max_mana: row.get(14)?,
            gold: row.get(15)?,
            zone_id: row.get(16)?,
            pos_x: row.get(17)?,
            pos_y: row.get(18)?,
            created_at: row.get(19)?,
            last_played: row.get(20)?,
        })
    }
}

// =========================================================================
// CharacterSave — serialisation JSON portable (fichiers de sauvegarde)
// =========================================================================

/// Sauvegarde portable d'un personnage, serialisable en JSON.
///
/// Represente un instantane complet des stats et de la position d'un personnage,
/// destine au stockage fichier ou a l'export/import entre sessions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterSave {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Identifiant du compte proprietaire.
    pub account_id: String,
    /// Nom de la classe du personnage (ex: "Chasseresse", "Barbare").
    pub class: String,
    /// Niveau courant (1-99).
    pub level: u32,
    /// Experience accumulee.
    pub experience: u64,
    /// Caracteristique Force.
    pub strength: u32,
    /// Caracteristique Dexterite.
    pub dexterity: u32,
    /// Caracteristique Vitalite.
    pub vitality: u32,
    /// Caracteristique Energie.
    pub energy: u32,
    /// Points de vie courants.
    pub hp_current: i32,
    /// Points de mana courants.
    pub mana_current: i32,
    /// Position X dans la zone courante.
    pub pos_x: f32,
    /// Position Y dans la zone courante.
    pub pos_y: f32,
    /// Identifiant de la zone courante.
    pub zone_id: String,
    /// Or en possession (0 a 2 500 000 000).
    pub gold: i64,
}

/// Resume d'un personnage pour les listings (ecran de selection).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterSummary {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Nom de la classe du personnage.
    pub class: String,
    /// Niveau courant.
    pub level: u32,
}

/// Valide les contraintes metier d'un `CharacterSave`.
///
/// Regles appliquees :
/// - `level` : 1 <= level <= 99
/// - statistiques (`strength`, `dexterity`, `vitality`, `energy`) : 0 <= stat <= 1023
/// - `gold` : 0 <= gold <= 2 500 000 000
///
/// # Erreurs
///
/// Retourne `PersistenceError::Duplicate` avec un message descriptif si une
/// contrainte est violee. (Reuse de la variante disponible dans le type d'erreur
/// existant pour les violations de regles metier.)
pub fn validate_character(save: &CharacterSave) -> Result<(), crate::PersistenceError> {
    if !(1..=99).contains(&save.level) {
        return Err(crate::PersistenceError::Duplicate(format!(
            "level {} is out of range [1, 99]",
            save.level
        )));
    }
    for (name, val) in [
        ("strength", save.strength),
        ("dexterity", save.dexterity),
        ("vitality", save.vitality),
        ("energy", save.energy),
    ] {
        if val > 1023 {
            return Err(crate::PersistenceError::Duplicate(format!(
                "stat {name} = {val} exceeds maximum 1023"
            )));
        }
    }
    if !(0..=2_500_000_000_i64).contains(&save.gold) {
        return Err(crate::PersistenceError::Duplicate(format!(
            "gold {} is out of range [0, 2_500_000_000]",
            save.gold
        )));
    }
    Ok(())
}

/// Serialise un `CharacterSave` valide en octets JSON.
///
/// # Erreurs
///
/// Retourne `PersistenceError::Json` si la serialisation echoue.
pub fn save_character(save: &CharacterSave) -> Result<Vec<u8>, crate::PersistenceError> {
    let bytes = serde_json::to_vec(save)?;
    Ok(bytes)
}

/// Deserialise des octets JSON en `CharacterSave`.
///
/// # Erreurs
///
/// Retourne `PersistenceError::Json` si la deserialisation echoue.
pub fn load_character(data: &[u8]) -> Result<CharacterSave, crate::PersistenceError> {
    let save = serde_json::from_slice(data)?;
    Ok(save)
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::{CharacterSave, load_character, save_character, validate_character};

    fn make_save() -> CharacterSave {
        CharacterSave {
            id: uuid::Uuid::new_v4().to_string(),
            account_id: uuid::Uuid::new_v4().to_string(),
            class: "Chasseresse".to_string(),
            level: 42,
            experience: 1_500_000,
            strength: 100,
            dexterity: 200,
            vitality: 150,
            energy: 35,
            hp_current: 480,
            mana_current: 95,
            pos_x: 12.5,
            pos_y: -3.0,
            zone_id: "cold_plains".to_string(),
            gold: 75_000,
        }
    }

    #[test]
    fn save_load_character_roundtrip() {
        let original = make_save();
        let bytes = save_character(&original).expect("save_character should succeed");
        let loaded = load_character(&bytes).expect("load_character should succeed");

        assert_eq!(loaded.id, original.id);
        assert_eq!(loaded.account_id, original.account_id);
        assert_eq!(loaded.class, original.class);
        assert_eq!(loaded.level, original.level);
        assert_eq!(loaded.experience, original.experience);
        assert_eq!(loaded.strength, original.strength);
        assert_eq!(loaded.dexterity, original.dexterity);
        assert_eq!(loaded.vitality, original.vitality);
        assert_eq!(loaded.energy, original.energy);
        assert_eq!(loaded.hp_current, original.hp_current);
        assert_eq!(loaded.mana_current, original.mana_current);
        assert_eq!(loaded.pos_x, original.pos_x);
        assert_eq!(loaded.pos_y, original.pos_y);
        assert_eq!(loaded.zone_id, original.zone_id);
        assert_eq!(loaded.gold, original.gold);
    }

    #[test]
    fn save_invalid_level() {
        let mut save = make_save();
        save.level = 100; // hors plage [1, 99]
        let result = validate_character(&save);
        assert!(
            result.is_err(),
            "level 100 must fail validation"
        );
    }

    #[test]
    fn save_invalid_stats() {
        let mut save = make_save();
        save.strength = 1024; // depasse le max 1023
        let result = validate_character(&save);
        assert!(
            result.is_err(),
            "strength 1024 must fail validation"
        );
    }
}
