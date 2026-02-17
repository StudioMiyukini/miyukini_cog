//! Tools MiyuBooking — tool.booking.slots.list, tool.booking.slots.resolve.
//! Créneaux : liste disponible ; résolution par identifiant ; contexte ressource, date, durée fourni.
//! Store en mémoire (id → resource_id, date_range, payload).

use crate::context::GovernedContext;
use crate::errors::MiyubookingError;
use std::collections::HashMap;
use std::sync::Mutex;

struct SlotRecord {
    resource_id: String,
    date_range: String,
    payload: String,
}

static SLOTS: std::sync::OnceLock<Mutex<HashMap<String, SlotRecord>>> = std::sync::OnceLock::new();

fn slots_store() -> &'static Mutex<HashMap<String, SlotRecord>> {
    SLOTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Enregistre un créneau (pour alimentation par flux / adaptateur JayRDV). Pas de décision métier ; données fournies.
pub fn register_slot(
    ctx: &GovernedContext,
    slot_id: &str,
    resource_id: &str,
    date_range: &str,
    payload: &str,
) -> Result<(), MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let mut guard = slots_store().lock().map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    guard.insert(
        slot_id.to_string(),
        SlotRecord {
            resource_id: resource_id.to_string(),
            date_range: date_range.to_string(),
            payload: payload.to_string(),
        },
    );
    Ok(())
}

/// @id: miyubooking_tool_booking_slots_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les créneaux disponibles ; contexte ressource, date, durée fourni.
/// @do: booking_slots_list_under_governance
pub fn list(
    ctx: &GovernedContext,
    resource_ref: &str,
    date_range: &str,
    _duration_minutes: Option<u32>,
) -> Result<Vec<String>, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let guard = slots_store().lock().map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    let ids: Vec<String> = guard
        .iter()
        .filter(|(_, r)| r.resource_id == resource_ref && r.date_range == date_range)
        .map(|(id, _)| id.clone())
        .collect();
    Ok(ids)
}

/// @id: miyubooking_tool_booking_slots_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un créneau par identifiant ; lecture.
/// @do: booking_slots_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, slot_id: &str) -> Result<String, MiyubookingError> {
    if !ctx.has_mandate() {
        return Err(MiyubookingError::NoMandate);
    }
    let guard = slots_store().lock().map_err(|_| MiyubookingError::InvalidInput("lock".into()))?;
    let payload = guard
        .get(slot_id)
        .map(|r| r.payload.clone())
        .ok_or_else(|| MiyubookingError::InvalidInput("slot not found".into()))?;
    Ok(payload)
}
