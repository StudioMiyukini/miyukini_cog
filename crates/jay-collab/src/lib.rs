//! Jay Collab — moteur de collaboration temps réel.
//!
//! Fournit :
//! - Un état CRDT (Yrs) partagé entre participants
//! - Un client WebSocket pour synchroniser avec le serveur collab
//! - Un gestionnaire de présence (cursors, sélections)
//!
//! ## Modèle
//!
//! ```text
//!   Participant A          Serveur Collab          Participant B
//!      │                        │                       │
//!      │─── Join room ─────────▶│                       │
//!      │◀── Initial state ──────│                       │
//!      │                        │◀─── Join room ────────│
//!      │                        │──── Initial state ───▶│
//!      │── Local edit ──▶│      │                       │
//!      │     encode update      │                       │
//!      │── Update ─────────────▶│                       │
//!      │                        │── Broadcast update ──▶│
//!      │                        │                       │  apply
//!      │                        │                       │
//! ```

pub mod client;
pub mod presence;
pub mod room;

pub use client::{CollabClient, CollabError};
pub use presence::PresenceManager;
pub use room::{CollabRoom, RoomStats};

use yrs::{Doc, ReadTxn, Transact};

/// Initialise un nouveau document Yrs vide.
pub fn new_doc() -> Doc {
    Doc::new()
}

/// Encode un état complet (snapshot) du document.
pub fn encode_state(doc: &Doc) -> Vec<u8> {
    use yrs::updates::encoder::Encode;
    let txn = doc.transact();
    txn.state_vector().encode_v1()
}

/// Encode un update diff depuis un state vector donné.
pub fn encode_update_since(doc: &Doc, state_vector: &[u8]) -> Result<Vec<u8>, CollabError> {
    use yrs::updates::decoder::Decode;
    let sv = yrs::StateVector::decode_v1(state_vector)
        .map_err(|e| CollabError::Crdt(format!("decode state vector: {e}")))?;
    let txn = doc.transact();
    Ok(txn.encode_state_as_update_v1(&sv))
}

/// Applique un update binaire au document.
pub fn apply_update(doc: &Doc, update: &[u8]) -> Result<(), CollabError> {
    use yrs::updates::decoder::Decode;
    let update = yrs::Update::decode_v1(update)
        .map_err(|e| CollabError::Crdt(format!("decode update: {e}")))?;
    let mut txn = doc.transact_mut();
    txn.apply_update(update)
        .map_err(|e| CollabError::Crdt(format!("apply update: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use yrs::{GetString, Text, Transact};

    #[test]
    fn two_docs_converge_after_sync() {
        // Document A
        let doc_a = new_doc();
        let text_a = doc_a.get_or_insert_text("content");
        {
            let mut txn = doc_a.transact_mut();
            text_a.insert(&mut txn, 0, "Bonjour");
        }

        // Document B
        let doc_b = new_doc();

        // A envoie son update complet à B
        let sv_b = encode_state(&doc_b);
        let update = encode_update_since(&doc_a, &sv_b).unwrap();
        apply_update(&doc_b, &update).unwrap();

        // B doit maintenant voir "Bonjour"
        let text_b = doc_b.get_or_insert_text("content");
        let txn_b = doc_b.transact();
        assert_eq!(text_b.get_string(&txn_b), "Bonjour");
    }

    #[test]
    fn concurrent_edits_merge() {
        let doc_a = new_doc();
        let doc_b = new_doc();

        let text_a = doc_a.get_or_insert_text("content");
        let text_b = doc_b.get_or_insert_text("content");

        // Synchroniser l'état initial
        let sv = encode_state(&doc_b);
        let update = encode_update_since(&doc_a, &sv).unwrap();
        apply_update(&doc_b, &update).unwrap();

        // Edits concurrents
        {
            let mut txn = doc_a.transact_mut();
            text_a.insert(&mut txn, 0, "Hello ");
        }
        {
            let mut txn = doc_b.transact_mut();
            text_b.insert(&mut txn, 0, "Bonjour ");
        }

        // Échanger les updates
        let sv_b = encode_state(&doc_b);
        let update_a_to_b = encode_update_since(&doc_a, &sv_b).unwrap();
        apply_update(&doc_b, &update_a_to_b).unwrap();

        let sv_a = encode_state(&doc_a);
        let update_b_to_a = encode_update_since(&doc_b, &sv_a).unwrap();
        apply_update(&doc_a, &update_b_to_a).unwrap();

        // Les deux doivent être identiques
        let txn_a = doc_a.transact();
        let txn_b = doc_b.transact();
        assert_eq!(text_a.get_string(&txn_a), text_b.get_string(&txn_b));
    }
}
