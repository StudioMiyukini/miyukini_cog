//! Section finance — coordonnées bancaires, moyens de paiement.
//!
//! Données nécessaires pour remplir des formulaires de prélèvement,
//! de virement, ou de souscription.

use serde::{Deserialize, Serialize};

/// Type de compte bancaire.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    #[default]
    Current,
    Savings,
    Joint,
    Business,
    Other,
}

/// Compte bancaire.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankAccountEntry {
    pub id: String,
    /// Nom descriptif (ex. "Compte courant LCL").
    pub label: String,
    pub bank_name: String,
    pub account_type: AccountType,
    /// Titulaire du compte.
    pub holder_name: String,
    /// IBAN.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// BIC / SWIFT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    /// Numéro de compte (hors IBAN, pour certains pays).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Code agence / guichet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    /// Compte principal (pour prélèvements par défaut) ?
    #[serde(default)]
    pub primary: bool,
    /// Devise (ISO 4217).
    #[serde(default = "default_currency")]
    pub currency: String,
    /// Scans RIB dans le Vault.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vault_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Type de moyen de paiement.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    #[default]
    CreditCard,
    DebitCard,
    Prepaid,
    Cheque,
    PayPal,
    Other,
}

/// Moyen de paiement (carte, PayPal, etc.).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethodEntry {
    pub id: String,
    pub label: String,
    pub method_type: PaymentMethodType,
    /// 4 derniers chiffres (pour les cartes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
    /// Date d'expiration (MM/YY).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Titulaire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<String>,
    /// Émetteur (Visa, Mastercard…).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Compte bancaire associé.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_id: Option<String>,
    #[serde(default)]
    pub primary: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

fn default_currency() -> String {
    "EUR".to_string()
}

/// Section finance complète.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinanceSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bank_accounts: Vec<BankAccountEntry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_methods: Vec<PaymentMethodEntry>,
}
