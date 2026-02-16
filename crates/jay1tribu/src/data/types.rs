//! Types domaine Jay1Tribu — Amis, tribus, salons, messages.
//!
//! @id: jay1tribu_domain_types
//! @do: define_jay1tribu_domain_model
//! @role: data
//! @layer: domain
//!
//! Conformité : archives locales uniquement (C-1), persistance via KindMother (C-4).
//! Présence et envoi en temps réel requièrent une connexion au Webway (MWS).

use serde::{Deserialize, Serialize};

/// Lien d'amitié entre le profil local et un COG ami.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Friend {
    /// Identifiant unique du lien.
    pub id: String,
    /// Profil local propriétaire de la liste d'amis.
    pub profile_id: String,
    /// COG de l'ami.
    pub friend_cog_id: String,
    /// Pseudo affiché (résolu localement).
    pub friend_pseudo: Option<String>,
    /// Date de création du lien (RFC3339).
    pub created_at: String,
}

/// Tribu (groupe partageant salons et membres).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tribe {
    /// Identifiant unique de la tribu.
    pub id: String,
    /// Nom affiché.
    pub name: String,
    /// Description optionnelle.
    pub description: Option<String>,
    /// COG du créateur (Chef de tribu).
    pub creator_cog_id: String,
    /// Date de création (RFC3339).
    pub created_at: String,
    /// Dernière mise à jour (RFC3339).
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
    /// Identifiant unique du salon.
    pub id: String,
    /// None = conversation directe (DM), Some = salon d'une tribu.
    pub tribe_id: Option<String>,
    /// Nom affiché du salon.
    pub name: String,
    /// Type : direct (2 participants) ou collectif.
    pub salon_type: SalonType,
    /// Date de création (RFC3339).
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
    /// Identifiant unique du message.
    pub id: String,
    /// Salon concerné.
    pub salon_id: String,
    /// COG de l'expéditeur.
    pub sender_cog_id: String,
    /// Contenu texte (ou référence à une pièce jointe).
    pub content: String,
    /// Date d'envoi (RFC3339).
    pub created_at: String,
}

/// Ami avec indicateur de présence (fourni par le MWS quand connecté au Webway).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendWithPresence {
    pub friend: Friend,
    /// true = en ligne (MWS), false = hors ligne ou Webway non connecté.
    pub online: bool,
}
