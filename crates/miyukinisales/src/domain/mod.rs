//! Logique métier MiyukiniSales : création devis, conversion en commande, statuts.

use chrono::Utc;
use uuid::Uuid;

use crate::data::{DbError, MiyukiniSalesStore};
use crate::{Order, OrderLine, OrderStatus, Quote, QuoteStatus};

/// Crée un devis (brouillon).
pub fn quote_create(
    store: &MiyukiniSalesStore,
    reference: &str,
    client_id: &str,
    total_cents: i64,
    valid_until: &str,
) -> Result<Quote, DbError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let quote = Quote {
        id: id.clone(),
        reference: reference.to_string(),
        client_id: client_id.to_string(),
        status: QuoteStatus::Draft,
        total_cents,
        created_at: now,
        valid_until: valid_until.to_string(),
    };
    store.quote_insert(quote.clone())?;
    Ok(quote)
}

/// Passe un devis au statut « envoyé ».
pub fn quote_send(store: &MiyukiniSalesStore, quote_id: &str) -> Result<(), DbError> {
    store.quote_set_status(quote_id, QuoteStatus::Sent)
}

/// Accepte un devis (permet la conversion en commande).
pub fn quote_accept(store: &MiyukiniSalesStore, quote_id: &str) -> Result<(), DbError> {
    store.quote_set_status(quote_id, QuoteStatus::Accepted)
}

/// Crée une commande (sans lien devis).
pub fn order_create(
    store: &MiyukiniSalesStore,
    reference: &str,
    client_id: &str,
    lines: Vec<OrderLine>,
) -> Result<Order, DbError> {
    let total_cents: i64 = lines.iter().map(|l| l.line_total_cents).sum();
    let now = Utc::now().to_rfc3339();
    let order = Order {
        id: Uuid::new_v4().to_string(),
        reference: reference.to_string(),
        quote_id: None,
        client_id: client_id.to_string(),
        status: OrderStatus::Pending,
        lines,
        total_cents,
        created_at: now.clone(),
        updated_at: now,
    };
    store.order_insert(order.clone())?;
    Ok(order)
}

/// Crée une commande à partir d'un devis accepté.
pub fn order_create_from_quote(
    store: &MiyukiniSalesStore,
    quote_id: &str,
    reference: &str,
    lines: Vec<OrderLine>,
) -> Result<Order, DbError> {
    let quote = store
        .quote_by_id(quote_id)?
        .ok_or_else(|| DbError(format!("quote not found: {quote_id}")))?;
    if quote.status != QuoteStatus::Accepted {
        return Err(DbError("quote must be accepted to create order".to_string()));
    }
    let total_cents: i64 = lines.iter().map(|l| l.line_total_cents).sum();
    let now = Utc::now().to_rfc3339();
    let order = Order {
        id: Uuid::new_v4().to_string(),
        reference: reference.to_string(),
        quote_id: Some(quote_id.to_string()),
        client_id: quote.client_id,
        status: OrderStatus::Pending,
        lines,
        total_cents,
        created_at: now.clone(),
        updated_at: now,
    };
    store.order_insert(order.clone())?;
    Ok(order)
}

/// Confirme une commande.
pub fn order_confirm(store: &MiyukiniSalesStore, order_id: &str) -> Result<(), DbError> {
    store.order_set_status(order_id, OrderStatus::Confirmed)
}
