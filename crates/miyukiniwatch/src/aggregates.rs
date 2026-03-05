//! Calcul des agrégats exposés à Miou.
//!
//! Les structures de données d'agrégation ont des champs auto-explicatifs.

#![allow(missing_docs)]
#![allow(unused_imports)]

use crate::data::{MiyukiniWatchDb, TimeSlot};
use chrono::{Local, Timelike};
use serde::{Deserialize, Serialize};

/// Identifiant d'un agrégat (contrat Miou).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateId {
    SessionSummary,
    SessionReturn,
    SessionTime,
    TopServices,
    NeglectedServices,
    FavoriteService,
    FavoriteTab,
    FriendReminders,
    TopFriends,
    SocialActivity,
    ActivityLevel,
    CurrentSession,
    Milestones,
    NewMilestone,
}

impl AggregateId {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SessionSummary => "AGG_SESSION_SUMMARY",
            Self::SessionReturn => "AGG_SESSION_RETURN",
            Self::SessionTime => "AGG_SESSION_TIME",
            Self::TopServices => "AGG_TOP_SERVICES",
            Self::NeglectedServices => "AGG_NEGLECTED_SERVICES",
            Self::FavoriteService => "AGG_FAVORITE_SERVICE",
            Self::FavoriteTab => "AGG_FAVORITE_TAB",
            Self::FriendReminders => "AGG_FRIEND_REMINDERS",
            Self::TopFriends => "AGG_TOP_FRIENDS",
            Self::SocialActivity => "AGG_SOCIAL_ACTIVITY",
            Self::ActivityLevel => "AGG_ACTIVITY_LEVEL",
            Self::CurrentSession => "AGG_CURRENT_SESSION",
            Self::Milestones => "AGG_MILESTONES",
            Self::NewMilestone => "AGG_NEW_MILESTONE",
        }
    }
}

/// Agrégat générique exposé à Miou.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregate {
    pub id: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Résumé de session (AGG_SESSION_SUMMARY).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummaryData {
    pub days_since_last_session: Option<u32>,
    pub avg_duration_seconds: Option<i64>,
    pub usual_time_slot: Option<String>,
    pub total_sessions: Option<i64>,
    pub consecutive_active_days: Option<i64>,
}

/// Indicateur de retour (AGG_SESSION_RETURN).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReturnData {
    pub is_returning: bool,
    pub days_away: Option<u32>,
}

/// Niveau d'activité (AGG_ACTIVITY_LEVEL).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLevelData {
    pub level: String,
    pub sessions_week: i64,
    pub total_duration_week: i64,
}

/// Calcule les agrégats pour un profil.
#[allow(clippy::too_many_lines)]
pub fn compute_aggregates(
    db: &MiyukiniWatchDb,
    profile_id: &str,
) -> Result<Vec<Aggregate>, crate::errors::MiyukiniWatchError> {
    let mut aggregates = Vec::new();
    let now = Local::now();

    // AGG_SESSION_TIME — tranche horaire actuelle
    let slot = TimeSlot::from_hour(now.hour());
    aggregates.push(Aggregate {
        id: "AGG_SESSION_TIME".to_string(),
        data: serde_json::json!({
            "current_time_slot": format!("{:?}", slot),
        }),
    });

    // AGG_SESSION_RETURN / AGG_SESSION_SUMMARY — jours depuis dernière session
    let last_end = db.get_last_session_end(profile_id)?;
    let days_away = last_end.as_ref().and_then(|ts| {
        chrono::DateTime::parse_from_rfc3339(ts).ok().map(|dt| {
            (now - dt.with_timezone(&Local))
                .num_days()
                .unsigned_abs()
                .min(u32::MAX as u64) as u32
        })
    });
    let total_sessions = db.get_global(profile_id, "total_sessions")?;
    aggregates.push(Aggregate {
        id: "AGG_SESSION_RETURN".to_string(),
        data: serde_json::json!({
            "is_returning": days_away.map(|d| d > 1).unwrap_or(false),
            "days_away": days_away,
        }),
    });
    aggregates.push(Aggregate {
        id: "AGG_SESSION_SUMMARY".to_string(),
        data: serde_json::json!({
            "days_since_last_session": days_away,
            "avg_duration_seconds": None::<i64>,
            "usual_time_slot": format!("{:?}", slot),
            "total_sessions": total_sessions,
            "consecutive_active_days": db.get_global(profile_id, "consecutive_active_days")?,
        }),
    });

    // AGG_ACTIVITY_LEVEL — niveau basique
    aggregates.push(Aggregate {
        id: "AGG_ACTIVITY_LEVEL".to_string(),
        data: serde_json::json!({
            "level": "moderate",
            "sessions_week": total_sessions.unwrap_or(0),
            "total_duration_week": 0_i64,
        }),
    });

    Ok(aggregates)
}
