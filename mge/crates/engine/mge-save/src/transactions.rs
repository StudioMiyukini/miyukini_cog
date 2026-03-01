// @id: MGE-Save-Transactions @do: atomic-save @role: back-end @layer: 3 @human: denis
//! Transactions atomiques pour la sauvegarde complete d'un personnage.
//!
//! La fonction `save_character_full()` persiste en une seule transaction SQLite :
//! personnage, items equipes, inventaire, competences, waypoints et quest flags.
//! Si une seule ecriture echoue, la totalite est annulee par ROLLBACK.

use rusqlite::params;
use chrono::Utc;
use crate::{DbPool, PersistResult};
use crate::characters::CharacterRow;
use crate::items::ItemData;

/// Snapshot complet d'un personnage a sauvegarder atomiquement.
///
/// Regroupe toutes les donnees mutables d'un personnage : stats, equipement,
/// inventaire, competences, waypoints et flags de quete. Le tout est persiste
/// en une seule transaction SQLite pour garantir la coherence.
pub struct CharacterSnapshot {
    /// Stats et position du personnage.
    pub character: CharacterRow,
    /// Items equipes : (slot, `ItemData`).
    pub equipped_items: Vec<(String, ItemData)>,
    /// Items en inventaire : (grid_x, grid_y, `ItemData`).
    pub inventory_items: Vec<(i32, i32, ItemData)>,
    /// Competences : (skill_id, points).
    pub skill_points: Vec<(String, i32)>,
    /// Waypoints debloques : (act, waypoint_id).
    pub waypoints: Vec<(i32, String)>,
    /// Flags de quete : (quest_id, state comme chaine).
    pub quest_flags: Vec<(String, String)>,
}

/// Persiste un snapshot complet de personnage en une seule transaction atomique.
///
/// # Strategie
///
/// - `BEGIN IMMEDIATE` verrouille la base en ecriture des le debut,
///   evitant les race conditions en mode WAL.
/// - Les items equipes et l'inventaire sont entierement remplaces
///   (delete + insert) car la grille peut changer arbitrairement.
/// - Les competences et quest flags utilisent UPSERT (additifs).
/// - Les waypoints utilisent INSERT OR IGNORE (idempotent).
///
/// # Erreurs
///
/// Retourne `PersistenceError` si une ecriture echoue. En cas d'erreur,
/// la transaction est annulee par ROLLBACK et aucune donnee n'est modifiee.
pub fn save_character_full(pool: &DbPool, snap: &CharacterSnapshot) -> PersistResult<()> {
    pool.with(|conn| {
        let now = Utc::now().to_rfc3339();

        // Transaction explicite -- IMMEDIATE pour eviter les deadlocks en WAL
        conn.execute_batch("BEGIN IMMEDIATE")?;

        let result = (|| -> PersistResult<()> {
            // 1. Mise a jour du personnage
            conn.execute(
                "UPDATE characters SET
                    level=?1, experience=?2, strength=?3, dexterity=?4,
                    vitality=?5, energy=?6, unspent_stats=?7,
                    current_life=?8, max_life=?9, current_mana=?10, max_mana=?11,
                    gold=?12, zone_id=?13, pos_x=?14, pos_y=?15, last_played=?16
                 WHERE id=?17",
                params![
                    snap.character.level, snap.character.experience,
                    snap.character.strength, snap.character.dexterity,
                    snap.character.vitality, snap.character.energy,
                    snap.character.unspent_stats,
                    snap.character.current_life, snap.character.max_life,
                    snap.character.current_mana, snap.character.max_mana,
                    snap.character.gold, snap.character.zone_id,
                    snap.character.pos_x, snap.character.pos_y,
                    now, snap.character.id
                ],
            )?;

            // 2. Items equipes -- on remplace tout (delete + insert)
            conn.execute(
                "DELETE FROM items WHERE owner_id=?1 AND owner_type='character_equipped'",
                params![snap.character.id],
            )?;
            for (slot, item_data) in &snap.equipped_items {
                let json = serde_json::to_string(item_data)?;
                let item_id = uuid::Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO items (id, owner_id, owner_type, slot, item_data, created_at)
                     VALUES (?1, ?2, 'character_equipped', ?3, ?4, ?5)",
                    params![item_id, snap.character.id, slot, json, now],
                )?;
            }

            // 3. Inventaire -- on remplace tout (delete + insert)
            conn.execute(
                "DELETE FROM items WHERE owner_id=?1 AND owner_type='character_inventory'",
                params![snap.character.id],
            )?;
            for (gx, gy, item_data) in &snap.inventory_items {
                let json = serde_json::to_string(item_data)?;
                let item_id = uuid::Uuid::new_v4().to_string();
                conn.execute(
                    "INSERT INTO items (id, owner_id, owner_type, grid_x, grid_y, item_data, created_at)
                     VALUES (?1, ?2, 'character_inventory', ?3, ?4, ?5, ?6)",
                    params![item_id, snap.character.id, gx, gy, json, now],
                )?;
            }

            // 4. Competences -- UPSERT (insert ou mise a jour si deja present)
            for (skill_id, points) in &snap.skill_points {
                conn.execute(
                    "INSERT INTO character_skills (character_id, skill_id, points)
                     VALUES (?1, ?2, ?3)
                     ON CONFLICT(character_id, skill_id)
                     DO UPDATE SET points=excluded.points",
                    params![snap.character.id, skill_id, points],
                )?;
            }

            // 5. Waypoints -- INSERT OR IGNORE (idempotent)
            for (act, wp_id) in &snap.waypoints {
                conn.execute(
                    "INSERT OR IGNORE INTO waypoints
                        (character_id, act, waypoint_id, unlocked_at)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![snap.character.id, act, wp_id, now],
                )?;
            }

            // 6. Quest flags -- UPSERT
            for (quest_id, state) in &snap.quest_flags {
                conn.execute(
                    "INSERT INTO quest_flags
                        (character_id, quest_id, state, updated_at)
                     VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(character_id, quest_id)
                     DO UPDATE SET state=excluded.state, updated_at=excluded.updated_at",
                    params![snap.character.id, quest_id, state, now],
                )?;
            }

            Ok(())
        })();

        match result {
            Ok(()) => {
                conn.execute_batch("COMMIT")?;
                Ok(())
            }
            Err(e) => {
                let _ = conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    })
}
