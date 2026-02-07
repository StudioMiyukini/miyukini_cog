//! Payloads types pour CK-INT-01/02/03.

use serde::{Deserialize, Serialize};

/// Erreur d'integration.
#[derive(Debug)]
pub struct IntegrationError(pub String);

impl std::fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Integration error: {}", self.0)
    }
}

impl std::error::Error for IntegrationError {}

/// Entete commun d'un evenement d'integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMeta {
    /// Contrat (CK-INT-01/02/03).
    pub contract_id: String,
    /// Acteur appelant.
    pub actor_ref: String,
    /// Horodatage ISO.
    pub occurred_at: String,
}

/// Payload quote.create.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteCreatePayload {
    /// Identifiant quote.
    pub quote_id: String,
    /// Scope account/purse.
    pub scope: String,
    /// Reference contexte metier.
    pub context_ref: String,
    /// Reference contrepartie.
    pub counterparty_ref: String,
    /// Montant total.
    pub total: f64,
    /// Devise.
    pub currency: String,
}

/// Payload invoice.emit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceEmitPayload {
    /// Identifiant facture.
    pub invoice_id: String,
    /// Scope account/purse.
    pub scope: String,
    /// Reference contexte metier.
    pub context_ref: String,
    /// Reference contrepartie.
    pub counterparty_ref: String,
    /// Devis source optionnel.
    pub quote_id: Option<String>,
    /// Montant total.
    pub total: f64,
    /// Devise.
    pub currency: String,
    /// Date echeance ISO.
    pub due_at: Option<String>,
}

/// Payload budget.movements.record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetMovementPayload {
    /// Identifiant mouvement.
    pub movement_id: String,
    /// Scope account/purse.
    pub scope: String,
    /// Reference contexte metier.
    pub context_ref: String,
    /// Categorie.
    pub category: String,
    /// Montant.
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Date ISO.
    pub movement_date: String,
}

/// Payload payment.record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRecordPayload {
    /// Identifiant paiement.
    pub payment_id: String,
    /// Facture cible.
    pub invoice_id: String,
    /// Montant.
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Methode.
    pub method: String,
    /// Reference opaque tokenisee.
    pub reference_opaque: String,
    /// Date ISO.
    pub paid_at: String,
}

/// Payload report.by_edition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportByEditionPayload {
    /// Reference edition.
    pub edition_ref: String,
    /// Scope.
    pub scope: String,
}

/// Payload report.by_professional.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportByProfessionalPayload {
    /// Reference professionnel.
    pub professional_ref: String,
    /// Scope.
    pub scope: String,
}

/// Payload deadline.reminder.publish.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineReminderPayload {
    /// Reference echeance.
    pub deadline_ref: String,
    /// Date d'echeance.
    pub due_at: String,
    /// Label.
    pub label: String,
    /// Contexte.
    pub context_ref: String,
}

/// Evenement JayFestival -> JayKonta (CK-INT-01).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JayFestivalEvent {
    /// quote.create
    QuoteCreate(IntegrationMeta, QuoteCreatePayload),
    /// invoice.emit
    InvoiceEmit(IntegrationMeta, InvoiceEmitPayload),
    /// budget.movements.record
    BudgetMovementRecord(IntegrationMeta, BudgetMovementPayload),
    /// report.by_edition
    ReportByEdition(IntegrationMeta, ReportByEditionPayload),
}

/// Evenement JayRDV -> JayKonta (CK-INT-02).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JayRDVEvent {
    /// quote.create
    QuoteCreate(IntegrationMeta, QuoteCreatePayload),
    /// invoice.emit
    InvoiceEmit(IntegrationMeta, InvoiceEmitPayload),
    /// payment.record
    PaymentRecord(IntegrationMeta, PaymentRecordPayload),
    /// report.by_professional
    ReportByProfessional(IntegrationMeta, ReportByProfessionalPayload),
}

/// Evenement JayKoa -> JayKonta (CK-INT-03).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JayKoaReminderEvent {
    /// Entete commun.
    pub meta: IntegrationMeta,
    /// Payload reminder.
    pub payload: DeadlineReminderPayload,
}
