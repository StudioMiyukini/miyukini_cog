//! Types de domaine Account (comptabilite entreprise).
//!
//! Couvre les entites : contrepartie, devis, lignes devis, facture,
//! lignes facture, paiement, et vues dashboard Account.

use serde::{Deserialize, Serialize};

// ─── Contrepartie ────────────────────────────────────────────

/// Contrepartie (client ou fournisseur).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterparty {
    /// Identifiant unique.
    pub id: String,
    /// Nom ou raison sociale.
    pub name: String,
    /// Email de contact.
    pub email: Option<String>,
    /// Adresse.
    pub address: Option<String>,
    /// Numero SIRET.
    pub siret: Option<String>,
    /// Numero de TVA intracommunautaire.
    pub tva_number: Option<String>,
    /// Telephone.
    pub phone: Option<String>,
    /// Notes libres.
    pub notes: Option<String>,
}

/// Formulaire de creation de contrepartie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCounterparty {
    /// Nom ou raison sociale.
    pub name: String,
    /// Email.
    pub email: Option<String>,
    /// Adresse.
    pub address: Option<String>,
    /// SIRET.
    pub siret: Option<String>,
    /// TVA.
    pub tva_number: Option<String>,
    /// Telephone.
    pub phone: Option<String>,
}

// ─── Devis ───────────────────────────────────────────────────

/// Devis (quote).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Identifiant unique.
    pub id: String,
    /// Numero lisible (DEV-2026-001).
    pub number: String,
    /// Contrepartie.
    pub counterparty: Counterparty,
    /// Lignes du devis.
    pub lines: Vec<QuoteLine>,
    /// Total hors taxes.
    pub total_ht: f64,
    /// Total TVA.
    pub total_tva: f64,
    /// Total toutes taxes comprises.
    pub total_ttc: f64,
    /// Devise.
    pub currency: String,
    /// Statut du devis.
    pub status: QuoteStatus,
    /// Duree de validite en jours.
    pub validity_days: u32,
    /// Notes / conditions.
    pub notes: Option<String>,
    /// Reference contexte (ex: edition JayFestival).
    pub context_ref: Option<String>,
    /// Service source.
    pub source_service: Option<String>,
    /// Date de creation.
    pub created_at: String,
}

/// Ligne de devis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteLine {
    /// Identifiant unique.
    pub id: String,
    /// Description de la prestation.
    pub description: String,
    /// Quantite.
    pub quantity: f64,
    /// Prix unitaire HT.
    pub unit_price: f64,
    /// Taux TVA (%).
    pub tva_rate: f64,
    /// Total HT de la ligne.
    pub total_ht: f64,
    /// Ordre d'affichage.
    pub sort_order: i32,
}

/// Statut d'un devis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuoteStatus {
    /// Brouillon.
    Draft,
    /// Envoye au client.
    Sent,
    /// Accepte.
    Accepted,
    /// Refuse.
    Rejected,
    /// Converti en facture.
    Converted,
}

impl QuoteStatus {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "sent" => Self::Sent,
            "accepted" => Self::Accepted,
            "rejected" => Self::Rejected,
            "converted" => Self::Converted,
            _ => Self::Draft,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Sent => "sent",
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Converted => "converted",
        }
    }

    /// Label lisible en UI.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Draft => "Brouillon",
            Self::Sent => "Envoye",
            Self::Accepted => "Accepte",
            Self::Rejected => "Refuse",
            Self::Converted => "Converti",
        }
    }
}

/// Formulaire de creation de devis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuote {
    /// Contrepartie cible.
    pub counterparty_id: String,
    /// Reference contexte optionnelle.
    pub context_ref: Option<String>,
    /// Lignes du devis.
    pub lines: Vec<CreateQuoteLine>,
    /// Duree de validite en jours.
    pub validity_days: u32,
    /// Notes.
    pub notes: Option<String>,
}

/// Ligne de creation de devis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuoteLine {
    /// Description.
    pub description: String,
    /// Quantite.
    pub quantity: f64,
    /// Prix unitaire HT.
    pub unit_price: f64,
    /// Taux TVA (%).
    pub tva_rate: f64,
}

// ─── Facture ─────────────────────────────────────────────────

/// Facture (invoice).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    /// Identifiant unique.
    pub id: String,
    /// Numero lisible (FAC-2026-001).
    pub number: String,
    /// Contrepartie.
    pub counterparty: Counterparty,
    /// Devis source optionnel.
    pub quote_id: Option<String>,
    /// Lignes de la facture.
    pub lines: Vec<InvoiceLine>,
    /// Total HT.
    pub total_ht: f64,
    /// Total TVA.
    pub total_tva: f64,
    /// Total TTC.
    pub total_ttc: f64,
    /// Montant deja paye.
    pub paid_amount: f64,
    /// Restant du (calcule : total_ttc - paid_amount).
    pub remaining: f64,
    /// Progression paiement (calcule : paid / ttc * 100).
    pub payment_progress: f64,
    /// Devise.
    pub currency: String,
    /// Statut de la facture.
    pub status: InvoiceStatus,
    /// Date d'emission.
    pub issued_at: String,
    /// Date d'echeance.
    pub due_at: Option<String>,
    /// Reference contexte.
    pub context_ref: Option<String>,
    /// Service source.
    pub source_service: Option<String>,
    /// Paiements associes.
    pub payments: Vec<Payment>,
    /// Notes.
    pub notes: Option<String>,
}

/// Ligne de facture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    /// Identifiant unique.
    pub id: String,
    /// Description de la prestation.
    pub description: String,
    /// Quantite.
    pub quantity: f64,
    /// Prix unitaire HT.
    pub unit_price: f64,
    /// Taux TVA (%).
    pub tva_rate: f64,
    /// Total HT de la ligne.
    pub total_ht: f64,
    /// Ordre d'affichage.
    pub sort_order: i32,
}

/// Statut d'une facture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvoiceStatus {
    /// Emise.
    Issued,
    /// Envoyee.
    Sent,
    /// Partiellement payee.
    Partial,
    /// Integralement payee.
    Paid,
    /// En retard.
    Overdue,
    /// Annulee.
    Cancelled,
}

impl InvoiceStatus {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "sent" => Self::Sent,
            "partial" => Self::Partial,
            "paid" => Self::Paid,
            "overdue" => Self::Overdue,
            "cancelled" => Self::Cancelled,
            _ => Self::Issued,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Issued => "issued",
            Self::Sent => "sent",
            Self::Partial => "partial",
            Self::Paid => "paid",
            Self::Overdue => "overdue",
            Self::Cancelled => "cancelled",
        }
    }

    /// Label lisible en UI.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Issued => "Emise",
            Self::Sent => "Envoyee",
            Self::Partial => "Partielle",
            Self::Paid => "Payee",
            Self::Overdue => "En retard",
            Self::Cancelled => "Annulee",
        }
    }
}

/// Formulaire de creation de facture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoice {
    /// Contrepartie.
    pub counterparty_id: String,
    /// Devis source optionnel.
    pub quote_id: Option<String>,
    /// Reference contexte.
    pub context_ref: Option<String>,
    /// Lignes de la facture.
    pub lines: Vec<CreateInvoiceLine>,
    /// Date d'echeance.
    pub due_at: Option<String>,
    /// Notes.
    pub notes: Option<String>,
}

/// Ligne de creation de facture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceLine {
    /// Description.
    pub description: String,
    /// Quantite.
    pub quantity: f64,
    /// Prix unitaire HT.
    pub unit_price: f64,
    /// Taux TVA (%).
    pub tva_rate: f64,
}

// ─── Paiement ────────────────────────────────────────────────

/// Paiement enregistre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    /// Identifiant unique.
    pub id: String,
    /// Montant paye.
    pub amount: f64,
    /// Devise.
    pub currency: String,
    /// Methode de paiement.
    pub method: PaymentMethod,
    /// Reference opaque (tokenisee, jamais en clair).
    pub reference_opaque: Option<String>,
    /// Date du paiement.
    pub paid_at: String,
    /// Notes.
    pub notes: Option<String>,
}

/// Methode de paiement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    /// Virement bancaire.
    Virement,
    /// Carte bancaire.
    Cb,
    /// Cheque.
    Cheque,
    /// Especes.
    Especes,
    /// Autre moyen.
    Autre,
}

impl PaymentMethod {
    /// Convertit depuis chaine SQL.
    pub fn from_str(s: &str) -> Self {
        match s {
            "cb" => Self::Cb,
            "cheque" => Self::Cheque,
            "especes" => Self::Especes,
            "autre" => Self::Autre,
            _ => Self::Virement,
        }
    }

    /// Convertit en chaine SQL.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Virement => "virement",
            Self::Cb => "cb",
            Self::Cheque => "cheque",
            Self::Especes => "especes",
            Self::Autre => "autre",
        }
    }

    /// Label lisible en UI.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Virement => "Virement",
            Self::Cb => "CB",
            Self::Cheque => "Cheque",
            Self::Especes => "Especes",
            Self::Autre => "Autre",
        }
    }
}

/// Formulaire d'enregistrement de paiement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayment {
    /// Facture cible.
    pub invoice_id: String,
    /// Montant paye.
    pub amount: f64,
    /// Date du paiement.
    pub paid_at: String,
    /// Methode de paiement.
    pub method: PaymentMethod,
    /// Reference opaque (tokenisee).
    pub reference_opaque: Option<String>,
    /// Notes.
    pub notes: Option<String>,
}

// ─── Vue Dashboard Account ───────────────────────────────────

/// Modele de vue du dashboard Account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDashboardView {
    /// Chiffre d'affaires ce mois.
    pub ca_mensuel: f64,
    /// Montant impayes.
    pub impayes: f64,
    /// Taux de paiement (%).
    pub taux_paiement: f64,
    /// Devise.
    pub currency: String,
    /// Devis recents (3 derniers).
    pub recent_quotes: Vec<QuoteSummary>,
    /// Factures en attente (3 dernieres).
    pub pending_invoices: Vec<InvoiceSummary>,
    /// Statut des integrations.
    pub integrations_status: Vec<IntegrationStatus>,
}

/// Resume de devis pour le dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteSummary {
    /// Identifiant.
    pub id: String,
    /// Numero.
    pub number: String,
    /// Nom client.
    pub counterparty_name: String,
    /// Montant TTC.
    pub total_ttc: f64,
    /// Statut.
    pub status: QuoteStatus,
}

/// Resume de facture pour le dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSummary {
    /// Identifiant.
    pub id: String,
    /// Numero.
    pub number: String,
    /// Nom client.
    pub counterparty_name: String,
    /// Montant TTC.
    pub total_ttc: f64,
    /// Statut.
    pub status: InvoiceStatus,
    /// Date echeance.
    pub due_at: Option<String>,
}

/// Statut d'une integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatus {
    /// Nom du service.
    pub service_name: String,
    /// Contrat.
    pub contract_id: String,
    /// Est connecte.
    pub is_connected: bool,
    /// Operations ce mois.
    pub ops_this_month: u32,
}
