//! Types pour Jay Réunion (conférences audio/vidéo collaboratives).

use crate::UserId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Statut d'une réunion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeetingStatus {
    /// Planifiée dans le futur.
    Scheduled,
    /// En cours.
    Live,
    /// Terminée.
    Ended,
    /// Annulée.
    Cancelled,
}

/// Réunion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meeting {
    pub id: String,
    pub title: String,
    pub description: String,
    pub host_id: UserId,
    pub status: MeetingStatus,
    pub scheduled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Participants invités.
    pub invited: Vec<UserId>,
    /// Participants réellement connectés.
    pub participants: Vec<MeetingParticipant>,
    /// Code de salle (pour rejoindre via lien court).
    pub room_code: String,
    /// Enregistrement activé.
    pub recording_enabled: bool,
    /// URL de l'enregistrement (après la réunion).
    pub recording_url: Option<String>,
    /// Transcription live.
    pub transcription_enabled: bool,
    /// Liens avec autres docs Jay Bureau (ordre du jour, notes).
    pub linked_docs: Vec<String>,
}

/// Participant d'une réunion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingParticipant {
    pub user_id: UserId,
    pub display_name: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub left_at: Option<chrono::DateTime<chrono::Utc>>,
    pub video_on: bool,
    pub audio_on: bool,
    pub screen_sharing: bool,
    /// Main levée (raise hand).
    pub hand_raised: bool,
}

impl Meeting {
    pub fn new(title: impl Into<String>, host_id: UserId) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            description: String::new(),
            host_id,
            status: MeetingStatus::Scheduled,
            scheduled_at: None,
            started_at: None,
            ended_at: None,
            invited: Vec::new(),
            participants: Vec::new(),
            room_code: generate_room_code(),
            recording_enabled: false,
            recording_url: None,
            transcription_enabled: false,
            linked_docs: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.status = MeetingStatus::Live;
        self.started_at = Some(chrono::Utc::now());
    }

    pub fn end(&mut self) {
        self.status = MeetingStatus::Ended;
        self.ended_at = Some(chrono::Utc::now());
    }

    pub fn join(&mut self, participant: MeetingParticipant) {
        // Si l'user est déjà dans la liste (rejoint à nouveau), on met à jour
        self.participants.retain(|p| p.user_id != participant.user_id);
        self.participants.push(participant);
    }

    pub fn leave(&mut self, user_id: &str) {
        if let Some(p) = self.participants.iter_mut().find(|p| p.user_id == user_id) {
            p.left_at = Some(chrono::Utc::now());
        }
    }

    /// Nombre de participants actifs (non partis).
    pub fn active_count(&self) -> usize {
        self.participants.iter().filter(|p| p.left_at.is_none()).count()
    }
}

/// Génère un code de salle aléatoire (3 groupes de 3 lettres, ex: "abc-def-ghi").
fn generate_room_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut group = || -> String {
        (0..3)
            .map(|_| (b'a' + rng.gen_range(0..26)) as char)
            .collect()
    };
    format!("{}-{}-{}", group(), group(), group())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meeting_lifecycle() {
        let mut m = Meeting::new("Standup", "alice".into());
        assert_eq!(m.status, MeetingStatus::Scheduled);

        m.start();
        assert_eq!(m.status, MeetingStatus::Live);
        assert!(m.started_at.is_some());

        m.join(MeetingParticipant {
            user_id: "bob".into(),
            display_name: "Bob".into(),
            joined_at: chrono::Utc::now(),
            left_at: None,
            video_on: true,
            audio_on: true,
            screen_sharing: false,
            hand_raised: false,
        });
        assert_eq!(m.active_count(), 1);

        m.leave("bob");
        assert_eq!(m.active_count(), 0);

        m.end();
        assert_eq!(m.status, MeetingStatus::Ended);
    }

    #[test]
    fn room_code_format() {
        let code = generate_room_code();
        assert_eq!(code.len(), 11); // 3 + 1 + 3 + 1 + 3
        assert_eq!(code.chars().filter(|&c| c == '-').count(), 2);
    }
}
