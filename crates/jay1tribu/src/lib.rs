//! Jay1Tribu — Service de messagerie P2P (tribus, salons, amis).
//!
//! @id: jay1tribu_lib
//! @do: expose_public_modules_jay1tribu
//! @role: service
//! @layer: infra
//!
//! Les fonctionnalités de **chat** (envoi/réception en temps réel) et de **tribu**
//! (sync avec les membres, présence) ne fonctionnent pleinement que si le COG est
//! **connecté au Webway** (MWS). En mode déconnecté : lecture locale uniquement,
//! messages et tribus créés localement ; livraison différée à la reconnexion.

pub mod data;
pub mod domain;
pub mod transport;

pub use transport::{DispatchError, MwsTransportSender, set_mws_transport_sender, has_mws_transport_sender};

pub use data::{
    Friend, FriendWithPresence, Jay1TribuDb, Message, Salon, SalonMember, SalonType, Tribe,
    TribeMember, TribeRole,
};
pub use domain::{
    accept_tribe_invitation, check_can_transfer_file, create_salon, create_tribe,
    find_direct_salon_between, get_friends_list, get_friends_with_presence,
    get_or_create_direct_salon, get_online_friends, invite_to_tribe, is_webway_connected,
    list_pending_invitations, process_pending_deliveries, refuse_tribe_invitation, send_file,
    send_message, set_webway_connected, Jay1TribuDomainError, Jay1TribuResult, PendingInvitation,
};
