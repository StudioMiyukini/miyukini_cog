//! Types de données MiyukiniWatch.
//!
//! Structures de données pour la collecte de métriques, préférences et audit.
//! Les champs sont auto-explicatifs par leur nom.

#![allow(missing_docs)]

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Tranche horaire (06-12 matin, 12-18 après-midi, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeSlot {
    /// 06:00 – 11:59
    Morning,
    /// 12:00 – 17:59
    Afternoon,
    /// 18:00 – 22:59
    Evening,
    /// 23:00 – 05:59
    Night,
}

impl TimeSlot {
    /// Déduit la tranche depuis une heure locale.
    #[must_use]
    pub fn from_hour(hour: u32) -> Self {
        match hour {
            6..=11 => Self::Morning,
            12..=17 => Self::Afternoon,
            18..=22 => Self::Evening,
            _ => Self::Night,
        }
    }
}

/// Préférences de collecte et rétention.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prefs {
    pub profile_id: String,
    pub collect_enabled: bool,
    pub collect_sessions: bool,
    pub collect_services: bool,
    pub collect_friends: bool,
    pub collect_activity: bool,
    pub retention_raw_days: u32,
    pub retention_daily_days: u32,
    pub retention_weekly_days: u32,
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            profile_id: String::new(),
            collect_enabled: true,
            collect_sessions: true,
            collect_services: true,
            collect_friends: true,
            collect_activity: true,
            retention_raw_days: 30,
            retention_daily_days: 90,
            retention_weekly_days: 365,
        }
    }
}

/// Enregistrement d'une métrique brute.
#[derive(Debug, Clone)]
pub struct MetricRecord {
    pub record_id: String,
    pub profile_id: String,
    pub metric_id: String,
    pub session_id: String,
    pub timestamp: DateTime<Local>,
    pub service_id: Option<String>,
    pub friend_id: Option<String>,
    pub value_int: Option<i64>,
    pub value_str: Option<String>,
}

/// Événement d'audit (collecte on/off, effacement, purge, etc.).
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: i64,
    pub profile_id: String,
    pub timestamp: DateTime<Local>,
    pub event_type: String,
    pub details: Option<String>,
    pub records_affected: Option<i64>,
}
