//! Section contrats — contrats de travail, assurances, abonnements, baux.
//!
//! Suivi des engagements contractuels, échéances, et procédures
//! de résiliation/renouvellement.

use serde::{Deserialize, Serialize};

/// Type de contrat.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractType {
    /// Contrat de travail (CDI, CDD…).
    Employment,
    /// Assurance (auto, habitation, vie, responsabilité civile…).
    Insurance,
    /// Abonnement (téléphone, internet, streaming, salle de sport…).
    Subscription,
    /// Bail / location.
    Lease,
    /// Prêt / crédit.
    Loan,
    /// Contrat de prestation / freelance.
    ServiceAgreement,
    /// Garantie (électroménager, véhicule…).
    Warranty,
    /// Autre.
    #[default]
    Other,
}

/// Statut du contrat.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    #[default]
    Active,
    /// En cours de résiliation.
    PendingCancellation,
    /// Résilié.
    Cancelled,
    /// Expiré.
    Expired,
    /// Suspendu.
    Suspended,
    /// En attente de signature.
    Draft,
}

/// Fréquence de paiement.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentFrequency {
    OneTime,
    Weekly,
    #[default]
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
}

/// Entrée contrat.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractEntry {
    pub id: String,
    pub contract_type: ContractType,
    /// Nom descriptif (ex. "Assurance auto Macif").
    pub label: String,
    /// Prestataire / fournisseur / employeur.
    pub provider: String,
    /// Référence / numéro de contrat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Date de début (ISO 8601).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Date de fin (vide = sans terme).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Date du prochain renouvellement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewal_date: Option<String>,
    /// Délai de préavis pour résiliation (ex. "2 mois", "30 jours").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_notice_period: Option<String>,
    /// Date limite pour envoyer la résiliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_deadline: Option<String>,
    /// Montant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Devise (EUR, USD…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Fréquence de paiement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_frequency: Option<PaymentFrequency>,
    /// Moyen de paiement (prélèvement, CB, chèque…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Statut actuel.
    pub status: ContractStatus,
    /// Coordonnées du service client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// Espace client URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    /// Documents dans le Vault (contrat signé, conditions…).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Section contrats complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractsSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracts: Vec<ContractEntry>,
}
