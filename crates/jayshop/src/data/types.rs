//! Types domaine JayShop (tickets, paiements, sessions, config, événements).
//!
//! Alignés sur le schéma de persistance ; persistance via KindMother Client.
//!
//! @id: jayshop_data_types
//! @do: define_domain_types_jayshop
//! @layer: domain

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// Source du ticket (PoS ou en ligne).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum TicketSource {
    /// Vente en point de vente.
    #[default]
    Pos,
    /// Vente en ligne.
    Online,
}

impl TicketSource {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pos => "pos",
            Self::Online => "online",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "online" => Self::Online,
            _ => Self::Pos,
        }
    }
}

/// Statut du ticket.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum TicketStatus {
    /// Brouillon.
    #[default]
    Draft,
    /// Ouvert (en cours).
    Open,
    /// Payé et clôturé.
    Paid,
    /// Remboursé.
    Refunded,
    /// Annulé.
    Cancelled,
}

impl TicketStatus {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Draft => "draft",
            Self::Open => "open",
            Self::Paid => "paid",
            Self::Refunded => "refunded",
            Self::Cancelled => "cancelled",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "open" => Self::Open,
            "paid" => Self::Paid,
            "refunded" => Self::Refunded,
            "cancelled" => Self::Cancelled,
            _ => Self::Draft,
        }
    }
}

/// Type de remise.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum DiscountType {
    /// Pourcentage.
    #[default]
    Percent,
    /// Montant fixe.
    Amount,
}

impl DiscountType {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Percent => "percent",
            Self::Amount => "amount",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "amount" => Self::Amount,
            _ => Self::Percent,
        }
    }
}

/// Type de taxe.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum TaxType {
    /// Taxe incluse dans le prix (TTC).
    #[default]
    Included,
    /// Taxe ajoutée au prix (HT + taxe).
    Added,
}

impl TaxType {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Included => "included",
            Self::Added => "added",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "added" => Self::Added,
            _ => Self::Included,
        }
    }
}

/// Type de mouvement de caisse.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum MovementType {
    /// Entrée d'espèces.
    #[default]
    In,
    /// Sortie d'espèces.
    Out,
}

impl MovementType {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::In => "in",
            Self::Out => "out",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "out" => Self::Out,
            _ => Self::In,
        }
    }
}

/// Statut d'un événement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum EventStatus {
    /// Brouillon.
    #[default]
    Draft,
    /// Confirmé.
    Confirmed,
    /// En cours.
    Ongoing,
    /// Clôturé.
    Closed,
    /// Annulé.
    Cancelled,
}

impl EventStatus {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Draft => "draft",
            Self::Confirmed => "confirmed",
            Self::Ongoing => "ongoing",
            Self::Closed => "closed",
            Self::Cancelled => "cancelled",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "confirmed" => Self::Confirmed,
            "ongoing" => Self::Ongoing,
            "closed" => Self::Closed,
            "cancelled" => Self::Cancelled,
            _ => Self::Draft,
        }
    }
}

/// Catégorie de coût événement.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum EventCostCategory {
    /// Stand.
    #[default]
    Stand,
    /// Transport.
    Transport,
    /// Logement.
    Lodging,
    /// Nourriture.
    Food,
    /// Autre.
    Other,
}

impl EventCostCategory {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Stand => "stand",
            Self::Transport => "transport",
            Self::Lodging => "lodging",
            Self::Food => "food",
            Self::Other => "other",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "transport" => Self::Transport,
            "lodging" => Self::Lodging,
            "food" => Self::Food,
            "other" => Self::Other,
            _ => Self::Stand,
        }
    }
}

/// Statut de synchronisation JayFestival.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum JayFestivalSyncStatus {
    /// Synchronisé.
    #[default]
    Synced,
    /// En attente.
    Pending,
    /// Erreur.
    Error,
}

impl JayFestivalSyncStatus {
    /// Convertit en chaîne pour la DB.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Synced => "synced",
            Self::Pending => "pending",
            Self::Error => "error",
        }
    }
    /// Parse depuis une chaîne DB.
    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => Self::Pending,
            "error" => Self::Error,
            _ => Self::Synced,
        }
    }
}

// ---------------------------------------------------------------------------
// Types de domaine principaux
// ---------------------------------------------------------------------------

/// Configuration boutique (table `shop_config`).
///
/// @id: jayshop_shop_config_type
/// @do: represent_shop_configuration
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ShopConfig {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur propriétaire.
    pub seller_id: Option<String>,
    /// Nom de la boutique.
    pub shop_name: Option<String>,
    /// Slug URL.
    pub slug: Option<String>,
    /// Devise (EUR).
    pub currency: Option<String>,
    /// Conditions générales de vente.
    pub legal_cgv: Option<String>,
    /// Mentions légales.
    pub legal_mentions: Option<String>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Ticket de vente (table `tickets`).
///
/// @id: jayshop_ticket_type
/// @do: represent_sale_ticket
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Ticket {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Numéro séquentiel (ex: JSHOP-2026-00042).
    pub ticket_number: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Session de caisse associée.
    pub session_id: Option<String>,
    /// Événement associé (vente sur événement).
    pub event_id: Option<String>,
    /// Source (pos/online).
    pub source: Option<String>,
    /// Statut.
    pub status: Option<String>,
    /// Client associé.
    pub customer_id: Option<String>,
    /// Sous-total HT (centimes).
    pub subtotal: Option<i64>,
    /// Total taxes (centimes).
    pub tax_total: Option<i64>,
    /// Total remises (centimes).
    pub discount_total: Option<i64>,
    /// Total TTC (centimes).
    pub total: Option<i64>,
    /// Devise.
    pub currency: Option<String>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de clôture.
    pub closed_at: Option<String>,
    /// Ticket original (si remboursement).
    pub refund_of: Option<String>,
}

/// Ligne de ticket (table `ticket_lines`).
///
/// @id: jayshop_ticket_line_type
/// @do: represent_ticket_line
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TicketLine {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Ticket parent.
    pub ticket_id: Option<String>,
    /// Produit (référence JayXpose).
    pub product_id: Option<String>,
    /// Nom du produit (snapshot).
    pub product_name: Option<String>,
    /// Quantité.
    pub quantity: Option<i32>,
    /// Prix unitaire (centimes).
    pub unit_price: Option<i64>,
    /// Remise appliquée.
    pub discount_id: Option<String>,
    /// Montant remise (centimes).
    pub discount_amount: Option<i64>,
    /// Taux de taxe appliqué.
    pub tax_rate: Option<f64>,
    /// Montant taxe (centimes).
    pub tax_amount: Option<i64>,
    /// Total ligne TTC (centimes).
    pub line_total: Option<i64>,
    /// Ordre de tri.
    pub sort_order: Option<i32>,
}

/// Paiement (table `payments`).
///
/// @id: jayshop_payment_type
/// @do: represent_payment
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Payment {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Ticket associé.
    pub ticket_id: Option<String>,
    /// Mode de paiement.
    pub method_id: Option<String>,
    /// Type de mode (cash/card/check/transfer/other).
    pub method_type: Option<String>,
    /// Montant (centimes).
    pub amount: Option<i64>,
    /// Montant donné (espèces, centimes).
    pub given_amount: Option<i64>,
    /// Rendu monnaie (centimes).
    pub change_amount: Option<i64>,
    /// Date du paiement.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Session de caisse (table `cash_sessions`).
///
/// @id: jayshop_cash_session_type
/// @do: represent_cash_session
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CashSession {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Ouverture.
    pub opened_at: Option<String>,
    /// Clôture.
    pub closed_at: Option<String>,
    /// Fond de caisse initial (centimes).
    pub opening_cash: Option<i64>,
    /// Espèces attendues à la clôture (centimes).
    pub closing_cash_expected: Option<i64>,
    /// Espèces comptées (centimes).
    pub closing_cash_counted: Option<i64>,
    /// Écart (centimes).
    pub cash_difference: Option<i64>,
    /// Total ventes (centimes).
    pub total_sales: Option<i64>,
    /// Total remboursements (centimes).
    pub total_refunds: Option<i64>,
    /// Note.
    pub note: Option<String>,
}

/// Mouvement de caisse (table `cash_movements`).
///
/// @id: jayshop_cash_movement_type
/// @do: represent_cash_movement
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CashMovement {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Session parent.
    pub session_id: Option<String>,
    /// Type (in/out).
    pub movement_type: Option<String>,
    /// Montant (centimes).
    pub amount: Option<i64>,
    /// Motif.
    pub reason: Option<String>,
    /// Date.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Remise pré-définie (table `discounts`).
///
/// @id: jayshop_discount_type
/// @do: represent_predefined_discount
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Discount {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Nom.
    pub name: Option<String>,
    /// Type (percent/amount).
    pub discount_type: Option<String>,
    /// Valeur.
    pub value: Option<f64>,
    /// Accès restreint (admin seulement).
    pub restricted_access: Option<bool>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Taux de taxe (table `tax_rates`).
///
/// @id: jayshop_tax_rate_type
/// @do: represent_tax_rate
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaxRate {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Nom (ex: TVA 20%).
    pub name: Option<String>,
    /// Taux (ex: 20.0).
    pub rate: Option<f64>,
    /// Type (included/added).
    pub tax_type: Option<String>,
    /// Appliquer par défaut aux nouveaux articles.
    pub apply_to_new_items: Option<bool>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Configuration PoS (table `pos_config`).
///
/// @id: jayshop_pos_config_type
/// @do: represent_pos_configuration
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PosConfig {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Onglets (JSON).
    pub tabs_json: Option<String>,
    /// Boutons (JSON).
    pub buttons_json: Option<String>,
    /// Modes de paiement (JSON).
    pub payment_methods_json: Option<String>,
    /// Logo reçu.
    pub receipt_logo_url: Option<String>,
    /// En-tête reçu.
    pub receipt_header: Option<String>,
    /// Pied reçu.
    pub receipt_footer: Option<String>,
    /// Date de mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Mode de paiement (table `payment_methods`).
///
/// @id: jayshop_payment_method_type
/// @do: represent_payment_method
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaymentMethod {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Nom (ex: Espèces, CB).
    pub name: Option<String>,
    /// Type (cash/card/check/transfer/other).
    pub method_type: Option<String>,
    /// Ordre d'affichage.
    pub sort_order: Option<i32>,
    /// Actif.
    pub is_active: Option<bool>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Client (table `customers`).
///
/// @id: jayshop_customer_type
/// @do: represent_customer
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Customer {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Nom.
    pub name: Option<String>,
    /// Email.
    pub email: Option<String>,
    /// Téléphone.
    pub phone: Option<String>,
    /// Adresse.
    pub address: Option<String>,
    /// Ville.
    pub city: Option<String>,
    /// Code postal.
    pub postal_code: Option<String>,
    /// Pays.
    pub country: Option<String>,
    /// Notes.
    pub notes: Option<String>,
    /// Nombre d'achats.
    pub total_purchases: Option<i64>,
    /// Montant total dépensé (centimes).
    pub total_spent: Option<i64>,
    /// Points fidélité.
    pub loyalty_points: Option<i64>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Journal de synchronisation (table `sync_logs`).
///
/// @id: jayshop_sync_log_type
/// @do: represent_sync_log_entry
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SyncLogEntry {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Source (jayshop/jayxpose/jaykonta/jayfestival).
    pub sync_source: Option<String>,
    /// Type de sync.
    pub sync_type: Option<String>,
    /// Statut (ok/partial/error/denied).
    pub status: Option<String>,
    /// Payload JSON.
    pub payload_json: Option<String>,
    /// Message d'erreur.
    pub error_message: Option<String>,
    /// Date.
    #[serde(default)]
    pub created_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Types événements
// ---------------------------------------------------------------------------

/// Fiche événement / participation (table `events`).
///
/// @id: jayshop_event_type
/// @do: represent_event_participation
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Vendeur.
    pub seller_id: Option<String>,
    /// Nom de l'événement.
    pub name: Option<String>,
    /// Date de début (ISO).
    pub start_date: Option<String>,
    /// Date de fin (ISO).
    pub end_date: Option<String>,
    /// Lieu.
    pub location: Option<String>,
    /// Informations stand.
    pub stand_info: Option<String>,
    /// Statut.
    pub status: Option<String>,
    /// Notes internes.
    pub notes: Option<String>,
    /// Liaison JayFestival — édition.
    pub jayfestival_edition_id: Option<String>,
    /// Liaison JayFestival — candidature.
    pub jayfestival_candidature_id: Option<String>,
    /// Statut de synchronisation JayFestival.
    pub jayfestival_sync_status: Option<String>,
    /// CA total (centimes).
    pub total_revenue: Option<i64>,
    /// Coûts totaux (centimes).
    pub total_costs: Option<i64>,
    /// Bénéfice brut (centimes).
    pub gross_profit: Option<i64>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Date de clôture.
    pub closed_at: Option<String>,
}

/// Coût de participation événement (table `event_costs`).
///
/// @id: jayshop_event_cost_type
/// @do: represent_event_cost
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventCost {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Événement parent.
    pub event_id: Option<String>,
    /// Catégorie (stand/transport/lodging/food/other).
    pub category: Option<String>,
    /// Libellé.
    pub label: Option<String>,
    /// Montant (centimes).
    pub amount: Option<i64>,
    /// Devise.
    pub currency: Option<String>,
    /// Date du coût.
    pub cost_date: Option<String>,
    /// URL justificatif.
    pub receipt_url: Option<String>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Stock temporaire événement (table `event_stock`).
///
/// @id: jayshop_event_stock_type
/// @do: represent_event_stock_allocation
/// @layer: domain
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventStock {
    /// Identifiant unique.
    pub id: Option<String>,
    /// Événement parent.
    pub event_id: Option<String>,
    /// Produit (référence JayXpose).
    pub product_id: Option<String>,
    /// Nom du produit (snapshot).
    pub product_name: Option<String>,
    /// Quantité allouée.
    pub allocated_qty: Option<i32>,
    /// Quantité vendue.
    pub sold_qty: Option<i32>,
    /// Quantité réintégrée.
    pub returned_qty: Option<i32>,
    /// Coût unitaire d'achat (centimes).
    pub unit_cost: Option<i64>,
    /// Date de création.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Date de mise à jour.
    #[serde(default)]
    pub updated_at: Option<String>,
}
