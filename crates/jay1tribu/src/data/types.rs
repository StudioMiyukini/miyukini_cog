//! Types domaine Jay1Tribu — Amis, tribus, salons, messages.
//!
//! Conformité : archives locales uniquement (C-1), persistance via KindMother (C-4).
//! Présence et envoi en temps réel requièrent une connexion au Webway (MWS).

use serde::{Deserialize, Serialize};

/// Lien d'amitié entre le profil local et un COG ami.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Friend {
    pub id: String,
    pub profile_id: String,
    pub friend_cog_id: String,
    pub friend_pseudo: Option<String>,
    pub created_at: String,
}

/// Tribu (groupe partageant salons et membres).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tribe {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub creator_cog_id: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Rôle au sein d'une tribu (Chef, Admin, Membre, ou personnalisé).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TribeRole {
    pub id: String,
    pub tribe_id: String,
    pub name: String,
    /// Permissions sérialisées (JSON ou bitmask selon implémentation).
    pub permissions_json: Option<String>,
    pub created_at: String,
}

/// Membre d'une tribu (liaison cog_id ↔ tribe_id + role_id).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TribeMember {
    pub id: String,
    pub tribe_id: String,
    pub cog_id: String,
    pub role_id: String,
    pub joined_at: String,
}

/// Type de salon : direct (2 participants) ou collectif.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SalonType {
    Direct,
    Collective,
}

impl SalonType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Collective => "collective",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "collective" => Self::Collective,
            _ => Self::Direct,
        }
    }
}

/// Salon de discussion (direct ou au sein d'une tribu).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Salon {
    pub id: String,
    /// None = conversation directe (DM), Some = salon d'une tribu.
    pub tribe_id: Option<String>,
    pub name: String,
    pub salon_type: SalonType,
    pub created_at: String,
}

/// Membre d'un salon (participant à la conversation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SalonMember {
    pub salon_id: String,
    pub cog_id: String,
    pub joined_at: String,
}

/// Message dans un salon (archivé localement ; transit crypté via MWS quand connecté).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub salon_id: String,
    pub sender_cog_id: String,
    pub content: String,
    pub created_at: String,
}

/// Ami avec indicateur de présence (fourni par le MWS quand connecté au Webway).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendWithPresence {
    pub friend: Friend,
    /// true = en ligne (MWS), false = hors ligne ou Webway non connecté.
    pub online: bool,
}
