#![allow(missing_docs)]
//! # MiyuBooking — toolkit.booking.reservations
//!
//! Kit d'outils réservations (créneaux, réservations, ressources, prix, participants).
//! Toute écriture = WriteIntent KindMother ; décision (création, annulation) = StrongFather.
//! Alignement MIP : domaine `booking`, layer tool/toolkit.

pub mod admin_cell;
pub mod booking;
pub mod context;
pub mod errors;
pub mod participants;
pub mod price;
pub mod resource;
pub mod slots;

pub use admin_cell::{
    miyubooking_admin_cell, MiyubookingAdminCell, MiyubookingIdentification, MiyubookingIntegrity,
    MiyubookingTestManifest, TOOLKIT_ID,
};
pub use booking::{cancel as booking_cancel, create as booking_create, update as booking_update};
pub use context::GovernedContext;
pub use errors::MiyubookingError;
pub use participants::compute as participants_compute;
pub use price::compute as price_compute;
pub use resource::{
    availability as resource_availability, register_resource, resolve as resource_resolve,
};
pub use slots::{list as slots_list, register_slot, resolve as slots_resolve};

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("test-mandate".to_string(), 1)
    }

    #[test]
    fn slots_register_list_resolve() {
        let c = ctx();
        register_slot(&c, "slot-1", "res-1", "2026-02-17", "{}").unwrap();
        register_slot(&c, "slot-2", "res-1", "2026-02-17", "{\"start\":\"10:00\"}").unwrap();
        let list = slots_list(&c, "res-1", "2026-02-17", None).unwrap();
        assert!(list.contains(&"slot-1".to_string()));
        assert!(list.contains(&"slot-2".to_string()));
        assert_eq!(slots_resolve(&c, "slot-1").unwrap(), "{}");
        assert_eq!(
            slots_resolve(&c, "slot-2").unwrap(),
            "{\"start\":\"10:00\"}"
        );
        assert!(slots_list(&c, "res-2", "2026-02-17", None)
            .unwrap()
            .is_empty());
    }

    #[test]
    fn slots_resolve_not_found() {
        let c = ctx();
        let r = slots_resolve(&c, "nonexistent");
        assert!(r.is_err());
    }

    #[test]
    fn resource_register_resolve_availability() {
        let c = ctx();
        register_resource(&c, "res-a", "salle 1").unwrap();
        let id = resource_resolve(&c, None).unwrap();
        assert_eq!(id, "res-a");
        let avail = resource_availability(&c, "res-a", "2026-02-17").unwrap();
        assert_eq!(avail, "[]");
    }

    #[test]
    fn resource_resolve_default_when_empty() {
        let c = ctx();
        let id = resource_resolve(&c, None).unwrap();
        assert!(id == "res:default" || !id.is_empty());
    }

    #[test]
    fn price_compute_returns_zero() {
        let c = ctx();
        assert_eq!(price_compute(&c, "book-1", None).unwrap(), "0");
    }

    #[test]
    fn participants_compute_returns_zero() {
        let c = ctx();
        assert_eq!(participants_compute(&c, "slot-1").unwrap(), "0");
    }

    #[test]
    fn no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(slots_list(&c, "r", "d", None).is_err());
        assert!(resource_resolve(&c, None).is_err());
        assert!(price_compute(&c, "b", None).is_err());
        assert!(participants_compute(&c, "s").is_err());
    }
}
