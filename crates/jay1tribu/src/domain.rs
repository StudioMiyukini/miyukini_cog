//! Logique métier Jay1Tribu : envoi de messages et sync tribu requièrent le Webway.
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

/// Vérifie que l'émetteur peut transférer un fichier vers les destinataires.
/// Règle : les transferts de fichier ne peuvent se faire qu'entre amis.
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

/// Enregistre un message localement. L'envoi en temps réel vers les autres COGs
/// requiert le Webway ; si non connecté, le message est uniquement stocké localement
/// (livraison différée à la reconnexion pour les tribus, selon la spec).
pub fn send_message(
    db: &Jay1TribuDb,
    salon_id: &str,
    sender_cog_id: &str,
    content: &str,
) -> Jay1TribuResult<Message> {
    let msg = db.message_create(salon_id, sender_cog_id, content)?;
    if is_webway_connected() {
        // TODO: déclencher l'envoi via MWS vers les autres participants du salon
    }
    Ok(msg)
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
