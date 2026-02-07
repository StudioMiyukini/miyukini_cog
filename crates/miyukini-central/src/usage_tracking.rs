//! Suivi d'utilisation Central et des Services.
//!
//! Enregistre les sessions (HUB vs Service), persiste en storage, et calcule
//! les statistiques : temps total, première utilisation, service le plus utilisé,
//! pics d'utilisation dans la journée.
//!
//! @id: miyukini_central_usage_tracking_module
//! @do: track_and_aggregate_usage_sessions
//! @role: data
//! @layer: domain
//! @human: Suivi d'utilisation : sessions, agrégats, persistance.

use crate::catalog::ServiceId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Contexte d'une session (Central HUB ou un Service).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageContext {
    /// Utilisation du HUB Central (menu, catalogue, paramètres).
    Central,
    /// Utilisation d'un Service identifié par sa clé de stockage.
    Service(String),
}

impl UsageContext {
    /// Clé string pour persistance.
    #[must_use] 
    pub fn key(&self) -> String {
        match self {
            UsageContext::Central => "central".to_string(),
            UsageContext::Service(s) => s.clone(),
        }
    }

    /// Crée un contexte à partir de la clé (central ou storage_key du service).
    #[must_use] 
    pub fn from_key(key: &str) -> Self {
        if key == "central" {
            UsageContext::Central
        } else {
            UsageContext::Service(key.to_string())
        }
    }
}

/// Une session d'utilisation (début, fin, contexte).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageSession {
    /// Timestamp de début (secondes depuis une époque).
    pub start_sec: f64,
    /// Timestamp de fin.
    pub end_sec: f64,
    /// Contexte (central ou service).
    #[serde(rename = "context")]
    pub context_key: String,
}

impl UsageSession {
    /// Durée en secondes.
    #[must_use] 
    pub fn duration_sec(&self) -> f64 {
        (self.end_sec - self.start_sec).max(0.0)
    }
}

/// Store persistant : liste des sessions (tronquée dans le temps pour limiter la taille).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageStore {
    /// Sessions enregistrées (les plus récentes en fin de liste).
    pub sessions: Vec<UsageSession>,
}

/// Nombre de secondes dans un jour ; pour tronquer les sessions au-delà.
const SECONDS_PER_DAY: f64 = 86400.0;
/// Conserver les sessions des N derniers jours.
const RETAIN_DAYS: f64 = 365.0;
/// Période « cette semaine » (7 jours glissants).
const SECONDS_PER_WEEK: f64 = 7.0 * SECONDS_PER_DAY;
/// Période « ce mois » (30 jours glissants).
const SECONDS_PER_MONTH: f64 = 30.0 * SECONDS_PER_DAY;

impl UsageStore {
    /// Enregistre une session et met à jour le store.
    pub fn record(&mut self, start_sec: f64, end_sec: f64, context: &UsageContext) {
        let context_key = context.key();
        if end_sec <= start_sec {
            return;
        }
        self.sessions.push(UsageSession {
            start_sec,
            end_sec,
            context_key,
        });
        self.trim_old_sessions(end_sec);
    }

    /// Supprime les sessions au-delà de RETAIN_DAYS.
    fn trim_old_sessions(&mut self, now_sec: f64) {
        let cutoff = now_sec - RETAIN_DAYS * SECONDS_PER_DAY;
        self.sessions.retain(|s| s.end_sec >= cutoff);
    }

    /// Calcule les statistiques agrégées à partir des sessions.
    /// `now_sec` sert aux tendances (cette semaine / ce mois, comparaisons).
    #[must_use] 
    pub fn compute_stats(&self, now_sec: f64) -> UsageStats {
        let mut first_use_sec: Option<f64> = None;
        let mut total_sec_central: f64 = 0.0;
        let mut total_sec_by_service: HashMap<String, f64> = HashMap::new();
        let mut hourly: [f64; 24] = [0.0; 24];
        let mut longest_session_sec: f64 = 0.0;
        let mut longest_session_context_key: Option<String> = None;

        for s in &self.sessions {
            if first_use_sec.is_none_or(|t| s.start_sec < t) {
                first_use_sec = Some(s.start_sec);
            }
            let dur = s.duration_sec();
            if dur > longest_session_sec {
                longest_session_sec = dur;
                longest_session_context_key = Some(s.context_key.clone());
            }
            if s.context_key == "central" {
                total_sec_central += dur;
            } else {
                *total_sec_by_service.entry(s.context_key.clone()).or_insert(0.0) += dur;
            }
            // Pics : heure de la journée (UTC 0–23) où la session a commencé.
            let hour = ((s.start_sec as i64).rem_euclid(86400) / 3600) as usize;
            let hour = hour.min(23);
            hourly[hour] += dur;
        }

        let total_sec_all = total_sec_central + total_sec_by_service.values().sum::<f64>();
        let session_count = self.sessions.len();
        let avg_session_sec = if session_count > 0 {
            total_sec_all / session_count as f64
        } else {
            0.0
        };

        let central_pct = if total_sec_all > 0.0 {
            100.0 * total_sec_central / total_sec_all
        } else {
            0.0
        };

        let mut top3_services: Vec<(String, f64)> = total_sec_by_service
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        top3_services.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        top3_services.truncate(3);

        let most_used_service_key = total_sec_by_service
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(k, _)| k.clone());

        let most_used_service = most_used_service_key.and_then(|k| ServiceId::from_storage_key(&k));

        // Périodes pour tendances (jours glissants).
        let this_week_start = now_sec - SECONDS_PER_WEEK;
        let this_month_start = now_sec - SECONDS_PER_MONTH;
        let last_week_start = now_sec - 2.0 * SECONDS_PER_WEEK;
        let last_month_start = now_sec - 2.0 * SECONDS_PER_MONTH;

        let mut this_week_sec = 0.0;
        let mut this_month_sec = 0.0;
        let mut last_week_sec = 0.0;
        let mut last_month_sec = 0.0;
        let mut active_days_this_week = HashSet::new();
        let mut active_days_this_month = HashSet::new();
        let mut active_days_total = HashSet::new();

        for s in &self.sessions {
            let day_idx = (s.start_sec / SECONDS_PER_DAY).floor() as i64;
            active_days_total.insert(day_idx);

            let overlap_this_week =
                (s.end_sec.min(now_sec) - s.start_sec.max(this_week_start)).max(0.0);
            if overlap_this_week > 0.0 {
                this_week_sec += overlap_this_week;
                active_days_this_week.insert(day_idx);
            }
            let overlap_this_month =
                (s.end_sec.min(now_sec) - s.start_sec.max(this_month_start)).max(0.0);
            if overlap_this_month > 0.0 {
                this_month_sec += overlap_this_month;
                active_days_this_month.insert(day_idx);
            }
            let overlap_last_week = (s.end_sec.min(this_week_start) - s.start_sec.max(last_week_start))
                .max(0.0);
            last_week_sec += overlap_last_week;
            let overlap_last_month =
                (s.end_sec.min(this_month_start) - s.start_sec.max(last_month_start)).max(0.0);
            last_month_sec += overlap_last_month;
        }

        UsageStats {
            first_use_sec,
            total_sec_central,
            total_sec_by_service,
            total_sec_all,
            hourly_distribution: hourly,
            most_used_service,
            session_count,
            avg_session_sec,
            longest_session_sec,
            longest_session_context_key,
            central_pct,
            top3_services,
            this_week_sec,
            this_month_sec,
            last_week_sec,
            last_month_sec,
            active_days_this_week: active_days_this_week.len(),
            active_days_this_month: active_days_this_month.len(),
            active_days_total: active_days_total.len(),
        }
    }
}

/// Statistiques d'utilisation calculées (lecture seule pour affichage).
#[derive(Debug, Clone)]
pub struct UsageStats {
    /// Date de première utilisation (timestamp sec).
    pub first_use_sec: Option<f64>,
    /// Temps total passé sur le HUB Central (secondes).
    pub total_sec_central: f64,
    /// Temps total par service (clé = storage_key).
    pub total_sec_by_service: HashMap<String, f64>,
    /// Temps total tout contexte confondu.
    pub total_sec_all: f64,
    /// Répartition par heure de la journée (0..24), en secondes.
    pub hourly_distribution: [f64; 24],
    /// Service avec le plus de temps d'utilisation.
    pub most_used_service: Option<ServiceId>,
    /// Nombre total de sessions enregistrées.
    pub session_count: usize,
    /// Durée moyenne par session (secondes).
    pub avg_session_sec: f64,
    /// Durée de la plus longue session (secondes).
    pub longest_session_sec: f64,
    /// Contexte de la plus longue session (central ou clé service).
    pub longest_session_context_key: Option<String>,
    /// Part du temps sur Central (0..100 %).
    pub central_pct: f64,
    /// Top 3 services par temps (clé, secondes).
    pub top3_services: Vec<(String, f64)>,
    /// Temps dans la dernière semaine (7 jours glissants), secondes.
    pub this_week_sec: f64,
    /// Temps dans le dernier mois (30 jours glissants), secondes.
    pub this_month_sec: f64,
    /// Temps dans la semaine précédente (7 jours avant), secondes.
    pub last_week_sec: f64,
    /// Temps dans le mois précédent (30 jours avant), secondes.
    pub last_month_sec: f64,
    /// Nombre de jours distincts avec au moins une session dans la dernière semaine.
    pub active_days_this_week: usize,
    /// Nombre de jours distincts avec au moins une session dans le dernier mois.
    pub active_days_this_month: usize,
    /// Nombre total de jours distincts avec au moins une session.
    pub active_days_total: usize,
}

impl UsageStats {
    /// Temps total sur Central formaté (minutes / heures / jours).
    #[must_use] 
    pub fn central_duration_human(&self) -> String {
        format_duration_sec(self.total_sec_central)
    }

    /// Temps total tout contexte formaté.
    #[must_use] 
    pub fn total_duration_human(&self) -> String {
        format_duration_sec(self.total_sec_all)
    }

    /// Texte "depuis quand" à partir de la première utilisation et du temps actuel.
    #[must_use] 
    pub fn first_use_relative_human(&self, now_sec: f64) -> Option<String> {
        self.first_use_sec.map(|sec| {
            let elapsed_sec = (now_sec - sec).max(0.0);
            if elapsed_sec < 60.0 {
                "Depuis à l'instant".to_string()
            } else if elapsed_sec < 3600.0 {
                format!("Depuis il y a {:.0} min", elapsed_sec / 60.0)
            } else if elapsed_sec < 86400.0 {
                format!("Depuis il y a {:.1} h", elapsed_sec / 3600.0)
            } else if elapsed_sec < 30.0 * 86400.0 {
                format!("Depuis il y a {:.0} j", elapsed_sec / 86400.0)
            } else if elapsed_sec < 365.0 * 86400.0 {
                format!("Depuis il y a {:.0} mois", elapsed_sec / (30.0 * 86400.0))
            } else {
                format!("Depuis il y a {:.0} an(s)", elapsed_sec / (365.0 * 86400.0))
            }
        })
    }

    /// Indices des heures avec le plus d'utilisation (pics, triés par ordre décroissant).
    #[must_use] 
    pub fn peak_hours(&self) -> Vec<(usize, f64)> {
        let mut v: Vec<(usize, f64)> = (0..24).map(|h| (h, self.hourly_distribution[h])).collect();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    /// Part des services (100 - central_pct), pour affichage.
    #[must_use] 
    pub fn services_pct(&self) -> f64 {
        (100.0 - self.central_pct).max(0.0)
    }

    /// Comparaison cette semaine vs semaine précédente (ex. "+25 %" ou "-10 %" ou "=").
    #[must_use] 
    pub fn week_comparison_human(&self) -> Option<String> {
        if self.last_week_sec <= 0.0 {
            return None;
        }
        let diff_pct = 100.0 * (self.this_week_sec - self.last_week_sec) / self.last_week_sec;
        Some(if diff_pct.abs() < 1.0 {
            "≈ identique".to_string()
        } else if diff_pct > 0.0 {
            format!("+{diff_pct:.0} % vs sem. précédente")
        } else {
            format!("{diff_pct:.0} % vs sem. précédente")
        })
    }

    /// Comparaison ce mois vs mois précédent (ex. "+15 %" ou "-20 %").
    #[must_use] 
    pub fn month_comparison_human(&self) -> Option<String> {
        if self.last_month_sec <= 0.0 {
            return None;
        }
        let diff_pct = 100.0 * (self.this_month_sec - self.last_month_sec) / self.last_month_sec;
        Some(if diff_pct.abs() < 1.0 {
            "≈ identique".to_string()
        } else if diff_pct > 0.0 {
            format!("+{diff_pct:.0} % vs mois précédent")
        } else {
            format!("{diff_pct:.0} % vs mois précédent")
        })
    }
}

/// Formate une durée en secondes en chaîne lisible (min, h, j, etc.).
#[must_use] 
pub fn format_duration_sec(sec: f64) -> String {
    if sec < 60.0 {
        return format!("{sec:.0} s");
    }
    let min = sec / 60.0;
    if min < 60.0 {
        return format!("{min:.1} min");
    }
    let h = min / 60.0;
    if h < 24.0 {
        return format!("{h:.1} h");
    }
    let d = h / 24.0;
    if d < 30.0 {
        return format!("{d:.1} j");
    }
    let m = d / 30.0;
    if m < 12.0 {
        return format!("{m:.1} mois");
    }
    let y = d / 365.0;
    format!("{y:.1} an(s)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_stats() {
        let mut store = UsageStore::default();
        store.record(1000.0, 1100.0, &UsageContext::Central);
        store.record(1100.0, 1200.0, &UsageContext::Service("calculator".to_string()));
        let stats = store.compute_stats(1200.0);
        assert!((stats.total_sec_central - 100.0).abs() < 0.1);
        assert!((stats.total_sec_by_service.get("calculator").copied().unwrap_or(0.0) - 100.0).abs() < 0.1);
        assert_eq!(stats.first_use_sec, Some(1000.0));
        assert_eq!(stats.session_count, 2);
        assert!((stats.avg_session_sec - 100.0).abs() < 0.1);
        assert!((stats.longest_session_sec - 100.0).abs() < 0.1);
    }
}
