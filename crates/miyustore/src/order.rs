//! Tools MiyuStore — tool.commerce.order.* (create, update, status, list).
//! Store en mémoire : order_id -> (payload, status).

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

struct OrderRecord {
    payload: String,
    status: String,
}

static ORDERS: std::sync::OnceLock<Mutex<HashMap<String, OrderRecord>>> =
    std::sync::OnceLock::new();

fn orders() -> &'static Mutex<HashMap<String, OrderRecord>> {
    ORDERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyustore_tool_commerce_order_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une commande ; WriteIntent KindMother (souvent via checkout.submit).
/// @do: commerce_order_create_under_governance
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let id = format!("ord:{}", UuidIdGenerator.generate());
    let mut guard = orders()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    guard.insert(
        id.clone(),
        OrderRecord {
            payload: payload.to_string(),
            status: "pending".to_string(),
        },
    );
    Ok(id)
}

/// @id: miyustore_tool_commerce_order_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une commande (statut, champs) ; WriteIntent KindMother.
/// @do: commerce_order_update_under_governance
pub fn update(ctx: &GovernedContext, order_id: &str, payload: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let mut guard = orders()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    if let Some(rec) = guard.get_mut(order_id) {
        rec.payload = payload.to_string();
    }
    Ok(())
}

/// @id: miyustore_tool_commerce_order_status
/// @role: accessor
/// @layer: tool
/// @human: Retourne le statut d'une commande ; lecture.
/// @do: commerce_order_status_under_governance
pub fn status(ctx: &GovernedContext, order_id: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let guard = orders()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    Ok(guard
        .get(order_id)
        .map(|r| r.status.as_str())
        .unwrap_or("unknown")
        .to_string())
}

/// @id: miyustore_tool_commerce_order_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les commandes ; filtres fournis.
/// @do: commerce_order_list_under_governance
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let guard = orders()
        .lock()
        .map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    Ok(guard.keys().cloned().collect())
}
