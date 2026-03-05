//! Adapter JayFestival → JayKonta / Miyuinvoice (devis, factures, budget).
//!
//! @id: jaykonta_create_quote
//! @do: crée un devis pour un exposant (façade Miyuinvoice → JayKonta)
//! @layer: domain
//!
//! @id: jaykonta_emit_invoice
//! @do: émet une facture (conversion devis ou création directe ; façade Miyuinvoice)
//! @layer: domain
//!
//! @id: miyuinvoice_facade
//! @do: façade d'accès aux outils Miyuinvoice (quote, invoice) pour JayFestival
//! @layer: domain
//!
//! Décision P0 : Miyuinvoice en façade, JayKonta en backend ; alpha = appels Miyuinvoice.

use miyuinvoice::context::GovernedContext;
use miyuinvoice::errors::MiyuinvoiceError;
use miyuinvoice::{invoice, quote};
use std::fmt;

/// Contexte gouverné alpha pour les appels Miyuinvoice depuis JayFestival.
fn jayfestival_invoice_ctx() -> GovernedContext {
    GovernedContext::new("jayfestival_alpha".to_string(), 1)
}

/// Erreur d'intégration JayKonta / Miyuinvoice.
#[derive(Debug)]
pub enum JayKontaError {
    /// Erreur remontée depuis Miyuinvoice.
    Invoice(MiyuinvoiceError),
}

impl fmt::Display for JayKontaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JayKontaError::Invoice(e) => write!(f, "invoice: {e}"),
        }
    }
}

impl std::error::Error for JayKontaError {}

impl From<MiyuinvoiceError> for JayKontaError {
    fn from(e: MiyuinvoiceError) -> Self {
        JayKontaError::Invoice(e)
    }
}

/// Crée un devis pour un exposant (façade Miyuinvoice ; backend JayKonta en phase 2).
///
/// @id: jaykonta_create_quote
/// @do: crée un devis pour un exposant (façade Miyuinvoice → JayKonta)
/// @layer: domain
pub fn jaykonta_create_quote(payload: &str) -> Result<String, JayKontaError> {
    let ctx = jayfestival_invoice_ctx();
    quote::create(&ctx, payload).map_err(JayKontaError::Invoice)
}

/// Émet une facture (conversion devis ou création directe ; façade Miyuinvoice).
///
/// @id: jaykonta_emit_invoice
/// @do: émet une facture (conversion devis ou création directe ; façade Miyuinvoice)
/// @layer: domain
pub fn jaykonta_emit_invoice(
    quote_id_or_payload: &str,
    from_quote: bool,
) -> Result<String, JayKontaError> {
    let ctx = jayfestival_invoice_ctx();
    if from_quote {
        quote::to_invoice(&ctx, quote_id_or_payload).map_err(JayKontaError::Invoice)
    } else {
        invoice::create(&ctx, quote_id_or_payload).map_err(JayKontaError::Invoice)
    }
}

/// Façade : crée un devis via Miyuinvoice (outil quote.create).
///
/// @id: miyuinvoice_facade
/// @do: façade d'accès aux outils Miyuinvoice (quote, invoice) pour JayFestival
/// @layer: domain
pub fn miyuinvoice_facade_quote_create(payload: &str) -> Result<String, JayKontaError> {
    jaykonta_create_quote(payload)
}
