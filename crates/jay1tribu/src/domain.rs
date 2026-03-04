//! Logique métier Jay1Tribu : envoi de messages et sync tribu requièrent le Webway.
//!
//! @id: jay1tribu_domain
//! @do: orchestrate_messaging_tribes_friends_governed_by_webway
//! @role: domain
//! @layer: domain
//!
//! Quand le COG n'est pas connecté au MWS :
//! - Lecture locale (amis, tribus, salons, messages) fonctionne.
//! - Envoi en temps réel et présence retournent une erreur ou un état dégradé.

use crate::data::{DbError, Friend, FriendWithPresence, Jay1TribuDb, Message, Salon};
use std::sync::atomic::{AtomicBool, Ordering};

/// Indicateur global : le COG est-il connecté au Webway (MWS) ?
/// Central doit le mettre à jour selon `MwsService::state() == Online`.
static WEBWAY_CONNECTED: AtomicBool = AtomicBool::new(false);

/// Définit si le COG est connecté au Webway (à appeler par Central depuis MwsService).
pub fn set_webway_connected(connected: bool) {
    WEBWAY_CONNECTED.store(connected, Ordering::SeqCst);
}

/// Retourne true si le COG est connecté au Webway.
pub fn is_webway_connected() -> bool {
    WEBWAY_CONNECTED.load(Ordering::SeqCst)
}

/// Erreur lorsque l'opération requiert une connexion au Webway.
#[derive(Debug)]
pub struct WebwayRequiredError;

impl std::fmt::Display for WebwayRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connexion au Webway requise pour cette opération")
    }
}

impl std::error::Error for WebwayRequiredError {}

/// Résultat d'une opération pouvant exiger le Webway.
pub type Jay1TribuResult<T> = Result<T, Jay1TribuDomainError>;

#[derive(Debug)]
pub enum Jay1TribuDomainError {
    Db(DbError),
    WebwayRequired,
    /// Transfert de fichier refusé : l'émetteur et le ou les destinataires doivent être amis.
    TransfertFichierReserveAmi,
}

impl std::fmt::Display for Jay1TribuDomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(e) => write!(f, "{}", e),
            Self::WebwayRequired => write!(f, "Connexion au Webway requise"),
            Self::TransfertFichierReserveAmi => {
                write!(f, "Les transferts de fichier ne peuvent se faire qu'entre amis")
            }
        }
    }
}

impl std::error::Error for Jay1TribuDomainError {}

impl From<DbError> for Jay1TribuDomainError {
    fn from(e: DbError) -> Self {
        Jay1TribuDomainError::Db(e)
    }
}

/// Liste des amis pour un profil (toujours disponible en local).
pub fn get_friends_list(db: &Jay1TribuDb, profile_id: &str) -> Result<Vec<Friend>, Jay1TribuDomainError> {
    db.friend_list(profile_id).map_err(Jay1TribuDomainError::from)
}

/// Liste des amis avec indicateur de présence. Si le Webway n'est pas connecté,
/// tous les amis sont renvoyés avec `online: false`.
pub fn get_friends_with_presence(
    db: &Jay1TribuDb,
    profile_id: &str,
    online_cog_ids: &[String],
) -> Result<Vec<FriendWithPresence>, Jay1TribuDomainError> {
    let friends = db.friend_list(profile_id)?;
    let online_set: std::collections::HashSet<_> = online_cog_ids.iter().cloned().collect();
    Ok(friends
        .into_iter()
        .map(|friend| FriendWithPresence {
            online: online_set.contains(&friend.friend_cog_id),
            friend,
        })
        .collect())
}

/// Amis actuellement en ligne (cog_ids). À fournir par le MWS quand connecté ;
/// sinon retourne une liste vide (dégradation gracieuse).
pub fn get_online_friends(
    db: &Jay1TribuDb,
    profile_id: &str,
    online_cog_ids: &[String],
) -> Result<Vec<Friend>, Jay1TribuDomainError> {
    let with_presence = get_friends_with_presence(db, profile_id, online_cog_ids)?;
    Ok(with_presence
        .into_iter()
        .filter(|fwp| fwp.online)
        .map(|fwp| fwp.friend)
        .collect())
}

/// Envoi d'un fichier ou d'une image dans un salon (restriction amis, spec §2.4).
/// Crée un message avec contenu de référence, enregistre la pièce jointe, puis
/// dispatch ou file d'attente selon présence Webway et destinataires en ligne.
///
/// # Deprecation
/// Cette fonction sera remplacee par MiyuCloud (transfert chiffre via lien de partage).
/// Voir `miyucloud::domain::FileOps::upload_file` + `ShareLink` pour le nouveau flux.
#[deprecated(
    since = "0.2.0",
    note = "Utiliser MiyuCloud pour le transfert de fichiers. send_file sera supprime dans la v0.3.0."
)]
#[allow(deprecated)]
pub fn send_file(
    db: &Jay1TribuDb,
    salon_id: &str,
    sender_cog_id: &str,
    sender_profile_id: &str,
    local_path: &str,
    kind: &str,
    online_cog_ids: Option<&[String]>,
) -> Jay1TribuResult<Message> {
    let recipients = db.salon_member_cog_ids(salon_id)?;
    let recipients_only: Vec<String> = recipients.iter().filter(|c| *c != sender_cog_id).cloned().collect();
    check_can_transfer_file(db, sender_profile_id, &recipients_only)?;
    let content = if kind.eq_ignore_ascii_case("image") {
        "[image]".to_string()
    } else {
        format!("[fichier: {}]", local_path)
    };
    let msg = db.message_create(salon_id, sender_cog_id, &content)?;
    db.message_attachment_create(&msg.id, kind, local_path)?;
    if !is_webway_connected() {
        for cog_id in &recipients {
            if *cog_id != sender_cog_id {
                db.pending_delivery_enqueue(&msg.id, cog_id)?;
            }
        }
        return Ok(msg);
    }
    let online_set = online_cog_ids.map(|ids| ids.iter().cloned().collect::<std::collections::HashSet<_>>());
    for cog_id in &recipients {
        if *cog_id == sender_cog_id {
            continue;
        }
        let is_online = online_set.as_ref().map_or(true, |s| s.contains(cog_id));
        if is_online {
            if crate::transport::dispatch_to_recipient(db, &msg, sender_cog_id, cog_id).is_err() {
                let _ = db.pending_delivery_enqueue(&msg.id, cog_id);
            }
        } else {
            let _ = db.pending_delivery_enqueue(&msg.id, cog_id);
        }
    }
    Ok(msg)
}

/// Vérifie que l'émetteur peut transférer un fichier vers les destinataires.
/// Règle : les transferts de fichier ne peuvent se faire qu'entre amis.
///
/// # Deprecation
/// Cette fonction sera remplacee par le systeme de permissions MiyuCloud (`SharePermission`).
#[deprecated(
    since = "0.2.0",
    note = "Utiliser MiyuCloud SharePermission pour le controle d'acces fichiers. Sera supprime dans la v0.3.0."
)]
pub fn check_can_transfer_file(
    db: &Jay1TribuDb,
    sender_profile_id: &str,
    recipient_cog_ids: &[String],
) -> Jay1TribuResult<()> {
    for cog_id in recipient_cog_ids {
        if !db.is_friend(sender_profile_id, cog_id)? {
            return Err(Jay1TribuDomainError::TransfertFichierReserveAmi);
        }
    }
    Ok(())
}

/// Enregistre un message localement. Si le Webway est connecté, envoi en temps réel
/// aux destinataires en ligne ; les destinataires hors ligne sont mis en file
/// (livraison différée à la reconnexion). `online_cog_ids` : COGs actuellement
/// en ligne (fourni par Central depuis le MWS) ; si `None`, tente l'envoi à tous.
pub fn send_message(
    db: &Jay1TribuDb,
    salon_id: &str,
    sender_cog_id: &str,
    content: &str,
    online_cog_ids: Option<&[String]>,
) -> Jay1TribuResult<Message> {
    let msg = db.message_create(salon_id, sender_cog_id, content)?;
    if !is_webway_connected() {
        let recipients = db.salon_member_cog_ids(salon_id)?;
        for cog_id in &recipients {
            if *cog_id != sender_cog_id {
                db.pending_delivery_enqueue(&msg.id, cog_id)?;
            }
        }
        return Ok(msg);
    }
    let recipients = db.salon_member_cog_ids(salon_id)?;
    let online_set = online_cog_ids.map(|ids| ids.iter().cloned().collect::<std::collections::HashSet<_>>());
    for cog_id in &recipients {
        if *cog_id == sender_cog_id {
            continue;
        }
        let is_online = online_set.as_ref().map_or(true, |s| s.contains(cog_id));
        if is_online {
            if crate::transport::dispatch_to_recipient(db, &msg, sender_cog_id, cog_id).is_err() {
                let _ = db.pending_delivery_enqueue(&msg.id, cog_id);
            }
        } else {
            let _ = db.pending_delivery_enqueue(&msg.id, cog_id);
        }
    }
    Ok(msg)
}

/// Traite les livraisons en attente pour l'émetteur : envoie les messages
/// aux destinataires qui sont maintenant en ligne. À appeler par Central
/// quand le Webway passe à connecté ou quand la liste des COGs en ligne est mise à jour.
pub fn process_pending_deliveries(
    db: &Jay1TribuDb,
    sender_cog_id: &str,
    online_cog_ids: &[String],
) -> Jay1TribuResult<()> {
    if !is_webway_connected() {
        return Ok(());
    }
    let online_set: std::collections::HashSet<_> = online_cog_ids.iter().cloned().collect();
    let pending = db.pending_deliveries_for_sender(sender_cog_id)?;
    for (pending_id, message_id, recipient_cog_id) in pending {
        if !online_set.contains(&recipient_cog_id) {
            continue;
        }
        let Some(msg) = db.message_by_id(&message_id)? else {
            continue;
        };
        if crate::transport::dispatch_to_recipient(db, &msg, sender_cog_id, &recipient_cog_id).is_ok() {
            db.pending_delivery_delete(&pending_id)?;
        }
    }
    Ok(())
}

/// Crée un salon (direct ou dans une tribu). Toujours possible en local.
pub fn create_salon(
    db: &Jay1TribuDb,
    tribe_id: Option<&str>,
    name: &str,
    is_direct: bool,
) -> Jay1TribuResult<Salon> {
    let salon_type = if is_direct {
        crate::data::SalonType::Direct
    } else {
        crate::data::SalonType::Collective
    };
    db.salon_create(tribe_id, name, salon_type).map_err(Into::into)
}

/// Retourne le salon direct entre deux COG s'il existe, sinon None (DAT-01).
pub fn find_direct_salon_between(
    db: &Jay1TribuDb,
    cog_id_1: &str,
    cog_id_2: &str,
) -> Jay1TribuResult<Option<Salon>> {
    db.salon_find_direct_between(cog_id_1, cog_id_2).map_err(Into::into)
}

/// Trouve ou crée le salon direct entre deux COG (au plus un par paire, DAT-01).
pub fn get_or_create_direct_salon(
    db: &Jay1TribuDb,
    cog_id_1: &str,
    cog_id_2: &str,
    name: &str,
) -> Jay1TribuResult<Salon> {
    if let Some(salon) = db.salon_find_direct_between(cog_id_1, cog_id_2)? {
        return Ok(salon);
    }
    let salon = db.salon_create(None, name, crate::data::SalonType::Direct)?;
    db.salon_add_member(&salon.id, cog_id_1)?;
    db.salon_add_member(&salon.id, cog_id_2)?;
    Ok(salon)
}

/// Crée une tribu. L'invitation des membres en temps réel requiert le Webway ;
/// la création locale est toujours possible.
pub fn create_tribe(
    db: &Jay1TribuDb,
    name: &str,
    description: Option<&str>,
    creator_cog_id: &str,
) -> Jay1TribuResult<crate::data::Tribe> {
    db.tribe_create(name, description, creator_cog_id).map_err(Into::into)
}

// ---------- Invitations tribu (M4) ----------

/// Invitation en attente pour un COG (id, tribe_id, inviter_cog_id).
pub type PendingInvitation = (String, String, String);

/// Invite un COG dans une tribu (création invitation status = pending).
pub fn invite_to_tribe(
    db: &Jay1TribuDb,
    tribe_id: &str,
    inviter_cog_id: &str,
    invitee_cog_id: &str,
) -> Jay1TribuResult<String> {
    db.tribe_invitation_create(tribe_id, inviter_cog_id, invitee_cog_id)
        .map_err(Into::into)
}

/// Accepte une invitation à une tribu (ajoute le membre avec rôle Membre).
pub fn accept_tribe_invitation(db: &Jay1TribuDb, invitation_id: &str) -> Jay1TribuResult<()> {
    db.tribe_invitation_accept(invitation_id).map_err(Into::into)
}

/// Refuse une invitation à une tribu.
pub fn refuse_tribe_invitation(db: &Jay1TribuDb, invitation_id: &str) -> Jay1TribuResult<()> {
    db.tribe_invitation_refuse(invitation_id).map_err(Into::into)
}

/// Liste les invitations en attente pour un COG (invité).
pub fn list_pending_invitations(
    db: &Jay1TribuDb,
    invitee_cog_id: &str,
) -> Jay1TribuResult<Vec<PendingInvitation>> {
    db.tribe_invitations_pending_for(invitee_cog_id)
        .map_err(Into::into)
}
