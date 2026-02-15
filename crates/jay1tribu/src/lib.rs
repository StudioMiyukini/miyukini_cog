//! Jay1Tribu — Service de messagerie P2P (tribus, salons, amis).
#![allow(missing_docs)]
//!
//! Les fonctionnalités de **chat** (envoi/réception en temps réel) et de **tribu**
//! (sync avec les membres, présence) ne fonctionnent pleinement que si le COG est
//! **connecté au Webway** (MWS). En mode déconnecté : lecture locale uniquement,
//! messages et tribus créés localement ; livraison différée à la reconnexion.

pub mod data;
pub mod domain;

pub use data::{
    Friend, FriendWithPresence, Jay1TribuDb, Message, Salon, SalonMember, SalonType, Tribe,
    TribeMember, TribeRole,
};
pub use domain::{
    check_can_transfer_file, create_salon, create_tribe, get_friends_list, get_friends_with_presence,
    get_online_friends, is_webway_connected, send_message, set_webway_connected,
    Jay1TribuDomainError, Jay1TribuResult,
};
