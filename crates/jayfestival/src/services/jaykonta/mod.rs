//! Intégration JayKonta / Miyuinvoice (devis, factures, budget par édition).
//!
//! JayFestival consomme JayKonta pour budget édition et facturation exposants.
//! Décision P0 : Miyuinvoice en façade, JayKonta en backend.

pub mod adapter;

pub use adapter::{
    jaykonta_create_quote,
    jaykonta_emit_invoice,
    miyuinvoice_facade_quote_create,
    JayKontaError,
};
