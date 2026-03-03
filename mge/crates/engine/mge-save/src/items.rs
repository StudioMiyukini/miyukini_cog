// @id: MGE-Save-Items @do: inventory-persistence @role: back-end @layer: 2 @human: miyuk
//! DAL pour les items (strategie JSON blob).
//!
//! Les items sont stockes avec leurs donnees variables (affixes, sockets, qualite)
//! dans une colonne JSON `item_data`. Cela permet de supporter des schemas d'items
//! arbitrairement complexes sans modifier le schema SQL.

use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult};

/// Representation d'un affix sur un item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemAffix {
    /// Identifiant de l'affix (ref TOML).
    pub affix_id: String,
    /// Valeur numerique de l'affix.
    pub value: f32,
}

/// Item complet tel que stocke (JSON blob dans `item_data`).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemData {
    /// Identifiant de base de l'item (ref TOML, ex: "long_bow").
    pub base_item_id: String,
    /// Qualite : "normal", "magic", "rare", "unique", "rune_word".
    pub quality: String,
    /// Quantite pour les stackables (runes, potions).
    pub quantity: u32,
    /// Durabilite courante.
    pub durability_cur: u32,
    /// Durabilite maximum.
    pub durability_max: u32,
    /// Liste des affixes.
    pub affixes: Vec<ItemAffix>,
    /// IDs des items socketes (runes/gems inserees).
    pub socketed: Vec<String>,
    /// L'item a-t-il ete identifie.
    pub is_identified: bool,
    /// Niveau de l'item (item level).
    pub item_level: u32,
}

/// Representation d'une ligne item en base (metadata + `ItemData`).
#[derive(Debug, Clone)]
pub struct ItemRow {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Proprietaire (character_id ou stash_id).
    pub owner_id: String,
    /// Type de proprietaire : "character_inventory", "character_equipped", "stash".
    pub owner_type: String,
    /// Slot d'equipement (si equipe) : "head", "chest", "main_hand", etc.
    pub slot: Option<String>,
    /// Position X dans la grille d'inventaire.
    pub grid_x: Option<i32>,
    /// Position Y dans la grille d'inventaire.
    pub grid_y: Option<i32>,
    /// Donnees de l'item (deserialise du JSON).
    pub data: ItemData,
    /// Date de creation (ISO 8601).
    pub created_at: String,
}

/// DAL pour les items.
pub struct ItemDal<'a>(pub &'a DbPool);

impl ItemDal<'_> {
    /// Insere un nouvel item en base avec les donnees fournies.
    ///
    /// Genere un UUID v4 et serialise `data` en JSON.
    pub fn insert(
        &self,
        owner_id: &str,
        owner_type: &str,
        data: &ItemData,
    ) -> PersistResult<ItemRow> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let json = serde_json::to_string(data)?;
        let row = ItemRow {
            id: id.clone(),
            owner_id: owner_id.to_string(),
            owner_type: owner_type.to_string(),
            slot: None,
            grid_x: None,
            grid_y: None,
            data: data.clone(),
            created_at: now.clone(),
        };
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO items (id, owner_id, owner_type, slot, grid_x, grid_y, item_data, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    id,
                    owner_id,
                    owner_type,
                    Option::<String>::None,
                    Option::<i32>::None,
                    Option::<i32>::None,
                    json,
                    now
                ],
            )?;
            Ok(row)
        })
    }

    /// Liste tous les items d'un proprietaire et d'un type donnes.
    pub fn list_for_owner(
        &self,
        owner_id: &str,
        owner_type: &str,
    ) -> PersistResult<Vec<ItemRow>> {
        self.0.with(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, owner_id, owner_type, slot, grid_x, grid_y, item_data, created_at
                 FROM items WHERE owner_id = ?1 AND owner_type = ?2",
            )?;
            let rows = stmt
                .query_map(params![owner_id, owner_type], |row| {
                    let json: String = row.get(6)?;
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<i32>>(4)?,
                        row.get::<_, Option<i32>>(5)?,
                        json,
                        row.get::<_, String>(7)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            rows.into_iter()
                .map(
                    |(id, owner_id, owner_type, slot, gx, gy, json, created_at)| {
                        let data: ItemData = serde_json::from_str(&json)?;
                        Ok(ItemRow {
                            id,
                            owner_id,
                            owner_type,
                            slot,
                            grid_x: gx,
                            grid_y: gy,
                            data,
                            created_at,
                        })
                    },
                )
                .collect()
        })
    }

    /// Met a jour l'emplacement d'un item (type de proprietaire, slot, position grille).
    pub fn update_location(
        &self,
        item_id: &str,
        owner_type: &str,
        slot: Option<&str>,
        grid_x: Option<i32>,
        grid_y: Option<i32>,
    ) -> PersistResult<()> {
        self.0.with(|conn| {
            conn.execute(
                "UPDATE items SET owner_type=?1, slot=?2, grid_x=?3, grid_y=?4 WHERE id=?5",
                params![owner_type, slot, grid_x, grid_y, item_id],
            )?;
            Ok(())
        })
    }

    /// Supprime un item par son identifiant.
    pub fn delete(&self, item_id: &str) -> PersistResult<()> {
        self.0.with(|conn| {
            conn.execute("DELETE FROM items WHERE id=?1", params![item_id])?;
            Ok(())
        })
    }
}

// =========================================================================
// InventorySave — serialisation JSON portable de l'inventaire
// =========================================================================

/// Sauvegarde portable de l'inventaire d'un personnage, serialisable en JSON.
///
/// Chaque slot est soit `None` (vide) soit une `String` contenant le JSON
/// d'un `ItemInstance`. Cette representation permet un roundtrip sans perte
/// independamment du schema SQL, pour les exports/imports et les sauvegardes
/// fichier portables.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InventorySave {
    /// Identifiant du personnage proprietaire (UUID v4).
    pub character_id: String,
    /// Slots de l'inventaire : `None` = vide, `Some(json)` = item serialise.
    pub slots: Vec<Option<String>>,
}

/// Cree un `InventorySave` et le serialise en octets JSON.
///
/// # Arguments
///
/// * `character_id` — UUID v4 du personnage proprietaire.
/// * `items` — Tranche de slots : `None` pour un slot vide, `Some(json)` pour un item.
///
/// # Erreurs
///
/// Retourne `PersistenceError::Json` si la serialisation echoue.
pub fn save_inventory(
    character_id: &str,
    items: &[Option<String>],
) -> Result<Vec<u8>, crate::PersistenceError> {
    let save = InventorySave {
        character_id: character_id.to_string(),
        slots: items.to_vec(),
    };
    let bytes = serde_json::to_vec(&save)?;
    Ok(bytes)
}

/// Deserialise des octets JSON en `InventorySave`.
///
/// # Erreurs
///
/// Retourne `PersistenceError::Json` si la deserialisation echoue.
pub fn load_inventory(data: &[u8]) -> Result<InventorySave, crate::PersistenceError> {
    let save = serde_json::from_slice(data)?;
    Ok(save)
}

// =========================================================================
// Tests — InventorySave
// =========================================================================

#[cfg(test)]
mod tests {
    use super::{load_inventory, save_inventory};

    /// Construit 40 slots : slots pairs avec un JSON item factice, slots impairs vides.
    fn make_slots(count: usize) -> Vec<Option<String>> {
        (0..count)
            .map(|i| {
                if i % 2 == 0 {
                    Some(format!(r#"{{"base_id":"item_{i}","slot":{i}}}"#))
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn inventory_save_load_roundtrip() {
        let character_id = uuid::Uuid::new_v4().to_string();
        let slots = make_slots(40);

        let bytes = save_inventory(&character_id, &slots).expect("save_inventory should succeed");
        let loaded = load_inventory(&bytes).expect("load_inventory should succeed");

        assert_eq!(loaded.character_id, character_id);
        assert_eq!(loaded.slots.len(), 40);

        for (i, (original, restored)) in slots.iter().zip(loaded.slots.iter()).enumerate() {
            assert_eq!(
                original, restored,
                "slot {i} mismatch after roundtrip"
            );
        }
    }

    #[test]
    fn inventory_empty_slot() {
        let character_id = uuid::Uuid::new_v4().to_string();
        // Inventaire vide : tous les slots a None
        let slots: Vec<Option<String>> = vec![None; 40];

        let bytes = save_inventory(&character_id, &slots).expect("save_inventory should succeed");
        let loaded = load_inventory(&bytes).expect("load_inventory should succeed");

        assert_eq!(loaded.slots.len(), 40);
        for (i, slot) in loaded.slots.iter().enumerate() {
            assert!(slot.is_none(), "slot {i} should be None after roundtrip");
        }
    }
}
