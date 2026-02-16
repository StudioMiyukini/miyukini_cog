//! MiyukiniSales — Service devis et ventes du COG.
//!
//! Domaine : devis, commandes, lignes, statuts.
//! Intégration prévue : MiyuStore, MiyuInvoice, JayKonta.

pub mod data;
pub mod domain;

pub use data::{
    DbError, MiyukiniSalesStore, Order, OrderLine, OrderStatus, Quote, QuoteStatus,
};
pub use domain::{
    order_confirm, order_create, order_create_from_quote, quote_accept, quote_create, quote_send,
};
