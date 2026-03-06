//! API JSON CardDAV -- liste et creation de carnets d'adresses et contacts.
//!
//! @id: miyucloud_carddav_json_api
//! @do: expose_carddav_data_as_json
//! @role: api
//! @layer: app
//!
//! Routes protegees par `X-COG-Token` (comme toutes les routes /api/*).
//! Les types de reponse JSON correspondent exactement aux types `AddressBookEntry`
//! et `ContactEntry` de `apps/central/src/services/miyucloud/state.rs`.

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

// ── Types de reponse JSON (compatibles avec AddressBookEntry/ContactEntry) ────

/// Compatible avec `AddressBookEntry` dans Central/state.rs.
/// Le champ `name` contient l'UUID du carnet (identifiant stable).
#[derive(Serialize)]
pub struct AddressBookJson {
    owner_id: String,
    name: String,
    display_name: String,
    ctag: String,
    created_at: String,
}

/// Compatible avec `ContactEntry` dans Central/state.rs.
#[derive(Serialize)]
pub struct ContactJson {
    uid: String,
    /// UUID du carnet parent (utilise comme book_name dans Central).
    book_name: String,
    owner_id: String,
    full_name: String,
    email: Option<String>,
    phone: Option<String>,
    etag: String,
    updated_at: String,
}

/// Corps de la requete de creation de carnet d'adresses.
#[derive(Deserialize)]
pub struct CreateAddressBookRequest {
    pub display_name: String,
}

// ── Handlers ─────────────────────────────────────────────────────────────────

/// GET /api/contacts/books -- liste les carnets d'adresses du proprietaire.
pub async fn list_addressbooks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AddressBookJson>>, (StatusCode, String)> {
    let owner_id = state.config.owner_id.clone();
    let books = state
        .carddav_svc
        .list_addressbooks(&owner_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let json = books
        .into_iter()
        .map(|b| AddressBookJson {
            owner_id: b.owner_id,
            name: b.id.to_string(),
            display_name: b.display_name,
            ctag: b.ctag,
            created_at: b.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(json))
}

/// POST /api/contacts/books -- cree un nouveau carnet d'adresses.
pub async fn create_addressbook(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateAddressBookRequest>,
) -> Result<(StatusCode, Json<AddressBookJson>), (StatusCode, String)> {
    let owner_id = state.config.owner_id.clone();
    let book = state
        .carddav_svc
        .create_addressbook(&owner_id, &body.display_name)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let json = AddressBookJson {
        owner_id: book.owner_id,
        name: book.id.to_string(),
        display_name: book.display_name,
        ctag: book.ctag,
        created_at: book.created_at.to_rfc3339(),
    };

    Ok((StatusCode::CREATED, Json(json)))
}

/// GET /api/contacts/books/{id}/contacts -- liste les contacts d'un carnet.
pub async fn list_contacts(
    State(state): State<Arc<AppState>>,
    Path(addressbook_id): Path<String>,
) -> Result<Json<Vec<ContactJson>>, (StatusCode, String)> {
    let owner_id = state.config.owner_id.clone();
    let contacts = state
        .carddav_svc
        .list_contacts(&addressbook_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let json = contacts
        .into_iter()
        .map(|c| ContactJson {
            uid: c.uid,
            book_name: c.addressbook_id.to_string(),
            owner_id: owner_id.clone(),
            full_name: c.full_name.unwrap_or_default(),
            email: c.email,
            phone: None,
            etag: c.etag,
            updated_at: c.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(json))
}
