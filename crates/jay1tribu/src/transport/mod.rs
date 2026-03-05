//! Abstraction transport MWS pour Jay1Tribu.
//!
//! @id: jay1tribu_transport
//! @do: dispatch_messages_via_mws_to_recipients
//! @role: transport
//! @layer: infra
//!
//! Envoi des messages vers les COGs destinataires via le MWS. L'appelant (ex. Central)
//! enregistre un `MwsTransportSender` (ex. délégation à miyuwebway_participant::transport::send).
//! Transit crypté (C-2).

use crate::data::{Jay1TribuDb, Message};

/// Trait d'envoi MWS : envoie un payload vers un COG (adresse = cog_id ou adresse réseau).
/// Central enregistre une implémentation qui délègue à miyuwebway_participant quand le MWS est connecté.
pub trait MwsTransportSender: Send + Sync {
    /// Envoie `payload` vers le COG `to`. Retourne une erreur si l'envoi échoue ou si le transport n'est pas disponible.
    fn send(&self, to: &str, payload: &[u8]) -> Result<(), DispatchError>;
}

static REGISTERED_SENDER: std::sync::OnceLock<
    std::sync::Arc<dyn MwsTransportSender + Send + Sync>,
> = std::sync::OnceLock::new();

/// Enregistre le sender MWS (à appeler par Central quand le Webway est connecté).
/// Remplace tout sender précédent si appelé plusieurs fois (via OnceLock, seul le premier reste ; pour remplacer, utiliser un wrapper).
pub fn set_mws_transport_sender(sender: std::sync::Arc<dyn MwsTransportSender + Send + Sync>) {
    let _ = REGISTERED_SENDER.set(sender);
}

/// Retourne true si un sender MWS a été enregistré.
pub fn has_mws_transport_sender() -> bool {
    REGISTERED_SENDER.get().is_some()
}

/// Dispatch un message vers un destinataire unique (livraison différée).
/// Si un `MwsTransportSender` est enregistré, le message est sérialisé (JSON) et envoyé via MWS.
pub fn dispatch_to_recipient(
    _db: &Jay1TribuDb,
    msg: &Message,
    _sender_cog_id: &str,
    recipient_cog_id: &str,
) -> Result<(), DispatchError> {
    if let Some(sender) = REGISTERED_SENDER.get() {
        let payload =
            serde_json::to_vec(msg).map_err(|e| DispatchError::Transport(e.to_string()))?;
        return sender.send(recipient_cog_id, &payload);
    }
    Ok(())
}

/// Dispatch un message vers les destinataires d'un salon (hors expéditeur).
pub fn dispatch_to_recipients(
    db: &Jay1TribuDb,
    msg: &Message,
    sender_cog_id: &str,
) -> Result<(), DispatchError> {
    let recipient_ids = db
        .salon_member_cog_ids(&msg.salon_id)
        .map_err(DispatchError::Db)?;
    for cog_id in recipient_ids {
        if cog_id != sender_cog_id {
            let _ = dispatch_to_recipient(db, msg, sender_cog_id, &cog_id);
        }
    }
    Ok(())
}

/// Erreur de dispatch transport.
#[derive(Debug)]
pub enum DispatchError {
    /// Erreur base de données (ex. lecture des membres du salon).
    Db(crate::data::DbError),
    /// Erreur d'envoi MWS (transport non disponible, réseau, etc.).
    Transport(String),
}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(e) => write!(f, "Dispatch: {}", e),
            Self::Transport(e) => write!(f, "Transport MWS: {}", e),
        }
    }
}

impl std::error::Error for DispatchError {}
