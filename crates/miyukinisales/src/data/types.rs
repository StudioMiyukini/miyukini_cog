//! Types domaine MiyukiniSales : devis, commandes, lignes, statuts.
//!
//! Modèle métier : devis (quote) → conversion en commande (order) ; lignes de commande.

use serde::{Deserialize, Serialize};

/// Statut d'un devis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    /// Brouillon.
    Draft,
    /// Envoyé au client.
    Sent,
    /// Accepté (peut être converti en commande).
    Accepted,
    /// Refusé.
    Rejected,
    /// Expiré.
    Expired,
}

impl Default for QuoteStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// Statut d'une commande.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// En attente (créée, non validée).
    Pending,
    /// Confirmée.
    Confirmed,
    /// En préparation / expédition.
    InProgress,
    /// Livrée / terminée.
    Completed,
    /// Annulée.
    Cancelled,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Devis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Identifiant unique.
    pub id: String,
    /// Référence métier (ex. DEV-2026-001).
    pub reference: String,
    /// Identifiant client (mandat ou tiers).
    pub client_id: String,
    /// Statut du devis.
    pub status: QuoteStatus,
    /// Montant total TTC (centimes ou unité minimale).
    pub total_cents: i64,
    /// Date de création (ISO 8601).
    pub created_at: String,
    /// Date de validité (ISO date).
    pub valid_until: String,
}

/// Ligne de devis ou de commande.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLine {
    /// Identifiant unique.
    pub id: String,
    /// Référence produit ou description.
    pub label: String,
    /// Quantité.
    pub quantity: u32,
    /// Prix unitaire (centimes).
    pub unit_price_cents: i64,
    /// Montant ligne (quantity * unit_price_cents).
    pub line_total_cents: i64,
}

/// Commande (issue d'un devis ou créée directement).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Identifiant unique.
    pub id: String,
    /// Référence métier (ex. CMD-2026-001).
    pub reference: String,
    /// Identifiant devis source (optionnel).
    pub quote_id: Option<String>,
    /// Identifiant client.
    pub client_id: String,
    /// Statut de la commande.
    pub status: OrderStatus,
    /// Lignes de la commande.
    pub lines: Vec<OrderLine>,
    /// Montant total TTC (centimes).
    pub total_cents: i64,
    /// Date de création (ISO 8601).
    pub created_at: String,
    /// Date de mise à jour (ISO 8601).
    pub updated_at: String,
}
