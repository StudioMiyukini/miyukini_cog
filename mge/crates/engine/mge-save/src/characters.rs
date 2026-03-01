// @id: MGE-Save-Characters @do: dal-characters @role: back-end @layer: 3 @human: denis
//! DAL pour les personnages joueurs.
//!
//! CRUD : creation, listing par compte, recherche par id, sauvegarde des stats.

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
