//! Store en mémoire pour MiyukiniSales (devis et commandes).

use std::collections::HashMap;
use std::sync::RwLock;

use crate::data::types::{Order, OrderStatus, Quote, QuoteStatus};

/// Erreur du store.
#[derive(Debug, thiserror::Error)]
#[error("MiyukiniSales store error: {0}")]
pub struct DbError(pub String);

/// Store en mémoire : devis et commandes.
pub struct MiyukiniSalesStore {
    quotes: RwLock<HashMap<String, Quote>>,
    orders: RwLock<HashMap<String, Order>>,
}

impl Default for MiyukiniSalesStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MiyukiniSalesStore {
    /// Crée un store vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            quotes: RwLock::new(HashMap::new()),
            orders: RwLock::new(HashMap::new()),
        }
    }

    /// Insère un devis (écrase si même id).
    pub fn quote_insert(&self, quote: Quote) -> Result<(), DbError> {
        let id = quote.id.clone();
        self.quotes
            .write()
            .map_err(|e| DbError(e.to_string()))?
            .insert(id, quote);
        Ok(())
    }

    /// Récupère un devis par id.
    pub fn quote_by_id(&self, id: &str) -> Result<Option<Quote>, DbError> {
        let guard = self.quotes.read().map_err(|e| DbError(e.to_string()))?;
        Ok(guard.get(id).cloned())
    }

    /// Liste les devis (optionnel : par statut, par client).
    pub fn quote_list(
        &self,
        status_filter: Option<QuoteStatus>,
        client_id_filter: Option<&str>,
    ) -> Result<Vec<Quote>, DbError> {
        let guard = self.quotes.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<Quote> = guard
            .values()
            .filter(|q| {
                status_filter.map_or(true, |s| q.status == s)
                    && client_id_filter.map_or(true, |c| q.client_id == c)
            })
            .cloned()
            .collect();
        Ok(out)
    }

    /// Met à jour le statut d'un devis.
    pub fn quote_set_status(&self, id: &str, status: QuoteStatus) -> Result<(), DbError> {
        let mut guard = self.quotes.write().map_err(|e| DbError(e.to_string()))?;
        let q = guard
            .get_mut(id)
            .ok_or_else(|| DbError(format!("quote not found: {id}")))?;
        q.status = status;
        Ok(())
    }

    /// Insère une commande.
    pub fn order_insert(&self, order: Order) -> Result<(), DbError> {
        let id = order.id.clone();
        self.orders
            .write()
            .map_err(|e| DbError(e.to_string()))?
            .insert(id, order);
        Ok(())
    }

    /// Récupère une commande par id.
    pub fn order_by_id(&self, id: &str) -> Result<Option<Order>, DbError> {
        let guard = self.orders.read().map_err(|e| DbError(e.to_string()))?;
        Ok(guard.get(id).cloned())
    }

    /// Liste les commandes (optionnel : par statut, par client).
    pub fn order_list(
        &self,
        status_filter: Option<OrderStatus>,
        client_id_filter: Option<&str>,
    ) -> Result<Vec<Order>, DbError> {
        let guard = self.orders.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<Order> = guard
            .values()
            .filter(|o| {
                status_filter.map_or(true, |s| o.status == s)
                    && client_id_filter.map_or(true, |c| o.client_id == c)
            })
            .cloned()
            .collect();
        Ok(out)
    }

    /// Met à jour le statut d'une commande.
    pub fn order_set_status(&self, id: &str, status: OrderStatus) -> Result<(), DbError> {
        let mut guard = self.orders.write().map_err(|e| DbError(e.to_string()))?;
        let o = guard
            .get_mut(id)
            .ok_or_else(|| DbError(format!("order not found: {id}")))?;
        o.status = status;
        o.updated_at = chrono::Utc::now().to_rfc3339();
        Ok(())
    }
}
