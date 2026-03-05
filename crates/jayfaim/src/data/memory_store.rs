//! Store en mémoire pour JayFaim (tables, créneaux, réservations, convives).

use std::collections::HashMap;
use std::sync::RwLock;

use crate::data::types::{Guest, Reservation, ReservationSlot, ReservationStatus, Table};

/// Erreur du store.
#[derive(Debug, thiserror::Error)]
#[error("JayFaim store error: {0}")]
pub struct DbError(pub String);

/// Store en mémoire.
pub struct JayFaimStore {
    tables: RwLock<HashMap<String, Table>>,
    slots: RwLock<HashMap<String, ReservationSlot>>,
    reservations: RwLock<HashMap<String, Reservation>>,
    guests: RwLock<HashMap<String, Guest>>,
}

impl Default for JayFaimStore {
    fn default() -> Self {
        Self::new()
    }
}

impl JayFaimStore {
    /// Crée un store vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tables: RwLock::new(HashMap::new()),
            slots: RwLock::new(HashMap::new()),
            reservations: RwLock::new(HashMap::new()),
            guests: RwLock::new(HashMap::new()),
        }
    }

    // --- Tables
    pub fn table_insert(&self, table: Table) -> Result<(), DbError> {
        let id = table.id.clone();
        self.tables
            .write()
            .map_err(|e| DbError(e.to_string()))?
            .insert(id, table);
        Ok(())
    }

    pub fn table_by_id(&self, id: &str) -> Result<Option<Table>, DbError> {
        let guard = self.tables.read().map_err(|e| DbError(e.to_string()))?;
        Ok(guard.get(id).cloned())
    }

    pub fn table_list(
        &self,
        venue_id: Option<&str>,
        active_only: bool,
    ) -> Result<Vec<Table>, DbError> {
        let guard = self.tables.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<Table> = guard
            .values()
            .filter(|t| {
                venue_id.map_or(true, |v| t.venue_id.as_deref() == Some(v))
                    && (!active_only || t.active)
            })
            .cloned()
            .collect();
        Ok(out)
    }

    // --- Slots
    pub fn slot_insert(&self, slot: ReservationSlot) -> Result<(), DbError> {
        let id = slot.id.clone();
        self.slots
            .write()
            .map_err(|e| DbError(e.to_string()))?
            .insert(id, slot);
        Ok(())
    }

    pub fn slot_by_id(&self, id: &str) -> Result<Option<ReservationSlot>, DbError> {
        let guard = self.slots.read().map_err(|e| DbError(e.to_string()))?;
        Ok(guard.get(id).cloned())
    }

    pub fn slot_list(
        &self,
        venue_id: Option<&str>,
        table_id: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<ReservationSlot>, DbError> {
        let guard = self.slots.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<ReservationSlot> = guard
            .values()
            .filter(|s| {
                venue_id.map_or(true, |v| s.venue_id == v)
                    && table_id.map_or(true, |t| s.table_id.as_deref() == Some(t))
                    && date.map_or(true, |d| s.date == d)
            })
            .cloned()
            .collect();
        Ok(out)
    }

    // --- Reservations
    pub fn reservation_insert(&self, r: Reservation) -> Result<(), DbError> {
        let id = r.id.clone();
        self.reservations
            .write()
            .map_err(|e| DbError(e.to_string()))?
            .insert(id, r);
        Ok(())
    }

    pub fn reservation_by_id(&self, id: &str) -> Result<Option<Reservation>, DbError> {
        let guard = self
            .reservations
            .read()
            .map_err(|e| DbError(e.to_string()))?;
        Ok(guard.get(id).cloned())
    }

    pub fn reservation_list(
        &self,
        venue_id: Option<&str>,
        table_id: Option<&str>,
        date: Option<&str>,
        status_filter: Option<ReservationStatus>,
    ) -> Result<Vec<Reservation>, DbError> {
        let guard = self
            .reservations
            .read()
            .map_err(|e| DbError(e.to_string()))?;
        let out: Vec<Reservation> = guard
            .values()
            .filter(|r| {
                venue_id.map_or(true, |v| r.venue_id == v)
                    && table_id.map_or(true, |t| r.table_id == t)
                    && date.map_or(true, |d| r.date == d)
                    && status_filter.map_or(true, |s| r.status == s)
            })
            .cloned()
            .collect();
        Ok(out)
    }

    pub fn reservation_set_status(
        &self,
        id: &str,
        status: ReservationStatus,
    ) -> Result<(), DbError> {
        let mut guard = self
            .reservations
            .write()
            .map_err(|e| DbError(e.to_string()))?;
        let r = guard
            .get_mut(id)
            .ok_or_else(|| DbError(format!("reservation not found: {id}")))?;
        r.status = status;
        r.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(())
    }

    // --- Guests
    pub fn guest_insert(&self, guest: Guest) -> Result<(), DbError> {
        let id = guest.id.clone();
        self.guests
            .write()
            .map_err(|e| DbError(e.to_string()))?
            .insert(id, guest);
        Ok(())
    }

    pub fn guest_list_by_reservation(&self, reservation_id: &str) -> Result<Vec<Guest>, DbError> {
        let guard = self.guests.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<Guest> = guard
            .values()
            .filter(|g| g.reservation_id == reservation_id)
            .cloned()
            .collect();
        Ok(out)
    }
}
