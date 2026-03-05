//! Logique métier JayFaim : tables, créneaux, réservations, convives.

use chrono::Utc;
use uuid::Uuid;

use crate::data::{DbError, JayFaimStore};
use crate::{Guest, Reservation, ReservationSlot, ReservationStatus, Table};

/// Crée une table.
pub fn table_create(
    store: &JayFaimStore,
    name: &str,
    capacity: u32,
    venue_id: Option<&str>,
) -> Result<Table, DbError> {
    let id = Uuid::new_v4().to_string();
    let table = Table {
        id: id.clone(),
        name: name.to_string(),
        capacity,
        venue_id: venue_id.map(String::from),
        active: true,
    };
    store.table_insert(table.clone())?;
    Ok(table)
}

/// Crée un créneau de réservation.
pub fn slot_create(
    store: &JayFaimStore,
    venue_id: &str,
    table_id: Option<&str>,
    date: &str,
    start_time: &str,
    end_time: &str,
    max_guests: u32,
) -> Result<ReservationSlot, DbError> {
    let id = Uuid::new_v4().to_string();
    let slot = ReservationSlot {
        id: id.clone(),
        table_id: table_id.map(String::from),
        venue_id: venue_id.to_string(),
        date: date.to_string(),
        start_time: start_time.to_string(),
        end_time: end_time.to_string(),
        max_guests,
    };
    store.slot_insert(slot.clone())?;
    Ok(slot)
}

/// Crée une réservation (pending).
pub fn reservation_create(
    store: &JayFaimStore,
    table_id: &str,
    venue_id: &str,
    date: &str,
    start_time: &str,
    end_time: &str,
    guest_count: u32,
    client_id: &str,
    note: Option<&str>,
) -> Result<Reservation, DbError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let reservation = Reservation {
        id: id.clone(),
        table_id: table_id.to_string(),
        venue_id: venue_id.to_string(),
        date: date.to_string(),
        start_time: start_time.to_string(),
        end_time: end_time.to_string(),
        guest_count,
        client_id: client_id.to_string(),
        status: ReservationStatus::Pending,
        note: note.map(String::from),
        created_at: now.clone(),
        updated_at: now,
    };
    store.reservation_insert(reservation.clone())?;
    Ok(reservation)
}

/// Confirme une réservation.
pub fn reservation_confirm(store: &JayFaimStore, reservation_id: &str) -> Result<(), DbError> {
    store.reservation_set_status(reservation_id, ReservationStatus::Confirmed)
}

/// Annule une réservation.
pub fn reservation_cancel(store: &JayFaimStore, reservation_id: &str) -> Result<(), DbError> {
    store.reservation_set_status(reservation_id, ReservationStatus::Cancelled)
}

/// Ajoute un convive à une réservation.
pub fn guest_add(
    store: &JayFaimStore,
    reservation_id: &str,
    name: &str,
    email: Option<&str>,
) -> Result<Guest, DbError> {
    let id = Uuid::new_v4().to_string();
    let guest = Guest {
        id: id.clone(),
        reservation_id: reservation_id.to_string(),
        name: name.to_string(),
        email: email.map(String::from),
    };
    store.guest_insert(guest.clone())?;
    Ok(guest)
}
