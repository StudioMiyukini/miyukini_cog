#![allow(missing_docs)]
//! # MiyuStore — toolkit.commerce.store
//!
//! Kit d'outils commerce (produits, panier, checkout, paiement, livraison, commandes).
//! Toute écriture = WriteIntent KindMother ; décision (checkout, paiement) = StrongFather.
//! Alignement MIP : domaine `commerce`, layer tool/toolkit.

pub mod admin_cell;
pub mod cart;
pub mod checkout;
pub mod context;
pub mod errors;
pub mod order;
pub mod payment;
pub mod product;
pub mod shipping;

pub use admin_cell::{
    miyustore_admin_cell, MiyustoreAdminCell, MiyustoreIdentification, MiyustoreIntegrity,
    MiyustoreTestManifest, TOOLKIT_ID,
};
pub use cart::{add as cart_add, compute as cart_compute, remove as cart_remove, update as cart_update};
pub use checkout::{submit as checkout_submit, validate as checkout_validate};
pub use context::GovernedContext;
pub use errors::MiyustoreError;
pub use order::{create as order_create, list as order_list, status as order_status, update as order_update};
pub use payment::{capture as payment_capture, refund as payment_refund, status as payment_status};
pub use product::{create as product_create, list as product_list, resolve as product_resolve, update as product_update, variations as product_variations};
pub use shipping::{rate as shipping_rate, zones_resolve as shipping_zones_resolve};
