//! Adapter JayFestival → Miyubooking (créneaux, réservations).
//!
//! @id: miyubooking_list_slots
//! @do: liste les créneaux disponibles pour une ressource / édition (Miyubooking slots.list)
//! @layer: domain
//!
//! @id: miyubooking_create_booking
//! @do: crée une réservation (atelier, billet, pass) via Miyubooking (booking.create)
//! @layer: domain
//!
//! Alpha : appelle Miyubooking (slots_list, booking_create).

use miyubooking::context::GovernedContext;
use miyubooking::slots;
use miyubooking::booking;
use miyubooking::errors::MiyubookingError;
use std::fmt;

/// Contexte gouverné alpha pour les appels Miyubooking depuis JayFestival.
fn jayfestival_booking_ctx() -> GovernedContext {
    GovernedContext::new("jayfestival_alpha".to_string(), 1)
}

/// Erreur d'intégration Miyubooking.
#[derive(Debug)]
pub struct MiyubookingAdapterError {
    /// Message d'erreur (réseau, payload, créneaux).
    pub message: String,
}

impl fmt::Display for MiyubookingAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MiyubookingAdapterError {}

impl From<MiyubookingError> for MiyubookingAdapterError {
    fn from(e: MiyubookingError) -> Self {
        MiyubookingAdapterError {
            message: format!("{e:?}"),
        }
    }
}

/// Liste les créneaux disponibles pour une ressource / édition.
///
/// @id: miyubooking_list_slots
/// @do: liste les créneaux disponibles pour une ressource / édition (Miyubooking slots.list)
/// @layer: domain
pub fn miyubooking_list_slots(
    resource_ref: &str,
    date_range: &str,
    duration_minutes: Option<u32>,
) -> Result<Vec<String>, MiyubookingAdapterError> {
    let ctx = jayfestival_booking_ctx();
    slots::list(&ctx, resource_ref, date_range, duration_minutes).map_err(MiyubookingAdapterError::from)
}

/// Crée une réservation (atelier, billet, pass) via Miyubooking.
///
/// @id: miyubooking_create_booking
/// @do: crée une réservation (atelier, billet, pass) via Miyubooking (booking.create)
/// @layer: domain
pub fn miyubooking_create_booking(payload: &str) -> Result<String, MiyubookingAdapterError> {
    let ctx = jayfestival_booking_ctx();
    booking::create(&ctx, payload).map_err(MiyubookingAdapterError::from)
}
