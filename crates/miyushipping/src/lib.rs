#![allow(missing_docs)]
//! # MiyuShipping — toolkit.commerce.shipping
//!
//! Kit d'outils livraison (tarifs, zones, étiquettes, suivi, expéditions).
//! Toute écriture = WriteIntent KindMother ; décision (étiquette, expédition) = StrongFather.
//! Alignement MIP : domaine `commerce` / shipping, layer tool/toolkit.

pub mod admin_cell;
pub mod context;
pub mod errors;
pub mod label;
pub mod rate;
pub mod shipment;
pub mod tracking;
pub mod zones;

pub use admin_cell::{
    miyushipping_admin_cell, MiyushippingAdminCell, MiyushippingIdentification,
    MiyushippingIntegrity, MiyushippingTestManifest, TOOLKIT_ID,
};
pub use context::GovernedContext;
pub use errors::MiyushippingError;
pub use label::{create as label_create, print as label_print};
pub use rate::{rate as shipping_rate, rates_compare as shipping_rates_compare};
pub use shipment::{create as shipment_create, list as shipment_list};
pub use tracking::get as tracking_get;
pub use zones::resolve as zones_resolve;
