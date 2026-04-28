//! Rappels et échéances — suivi automatique des dates clés.
//!
//! Génère des rappels à partir des dates d'expiration des documents,
//! de renouvellement des contrats, des vaccinations, ordonnances, etc.
//! L'exécution des notifications est déléguée à MiyuNotify.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Priorité d'un rappel.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReminderPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

/// Statut d'un rappel.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReminderStatus {
    #[default]
    Pending,
    Acknowledged,
    Snoozed,
    Completed,
    Expired,
}

/// Source automatique d'un rappel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReminderSource {
    /// Rappel créé manuellement.
    Manual,
    /// Expiration d'un document officiel.
    DocumentExpiry { document_id: String },
    /// Renouvellement / résiliation d'un contrat.
    ContractDeadline { contract_id: String },
    /// Rappel vaccination.
    VaccinationBooster { vaccination_id: String },
    /// Validité d'une ordonnance.
    PrescriptionExpiry { prescription_id: String },
    /// Expiration d'un moyen de paiement.
    PaymentMethodExpiry { payment_method_id: String },
    /// Expiration d'une certification professionnelle.
    CertificationExpiry { certification_id: String },
}

/// Entrée rappel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderEntry {
    pub id: String,
    /// Titre court.
    pub title: String,
    /// Description détaillée.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Date du rappel (ISO 8601 : YYYY-MM-DD).
    pub due_date: String,
    /// Jours d'avance pour le rappel anticipé (ex. 30 = rappel 30j avant).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advance_days: Option<u32>,
    pub priority: ReminderPriority,
    pub status: ReminderStatus,
    /// Source automatique ou manuelle.
    pub source: ReminderSource,
    /// Récurrence (vide = one-shot).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    /// Tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

static REMINDERS_STORE: std::sync::OnceLock<Mutex<HashMap<String, Vec<ReminderEntry>>>> =
    std::sync::OnceLock::new();

fn reminders_store() -> &'static Mutex<HashMap<String, Vec<ReminderEntry>>> {
    REMINDERS_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// tool.profile.reminders.add — Ajoute un rappel.
pub fn add(
    ctx: &GovernedContext,
    user_id: &str,
    entry: ReminderEntry,
) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let id = entry.id.clone();
    let mut guard = reminders_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("reminders lock".into()))?;
    guard.entry(user_id.to_string()).or_default().push(entry);
    Ok(id)
}

/// tool.profile.reminders.list — Liste les rappels d'un utilisateur.
pub fn list(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<Vec<ReminderEntry>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = reminders_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("reminders lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_default())
}

/// tool.profile.reminders.pending — Rappels en attente, triés par date.
pub fn pending(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<Vec<ReminderEntry>, MiyuprofileError> {
    let all = list(ctx, user_id)?;
    let mut pending: Vec<_> = all
        .into_iter()
        .filter(|r| r.status == ReminderStatus::Pending)
        .collect();
    pending.sort_by(|a, b| a.due_date.cmp(&b.due_date));
    Ok(pending)
}

/// tool.profile.reminders.update_status — Met à jour le statut d'un rappel.
pub fn update_status(
    ctx: &GovernedContext,
    user_id: &str,
    reminder_id: &str,
    status: ReminderStatus,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = reminders_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("reminders lock".into()))?;
    let entries = guard
        .get_mut(user_id)
        .ok_or_else(|| MiyuprofileError::NotFound("reminders user".into()))?;
    let entry = entries
        .iter_mut()
        .find(|r| r.id == reminder_id)
        .ok_or_else(|| MiyuprofileError::NotFound(format!("reminder:{reminder_id}")))?;
    entry.status = status;
    Ok(())
}

/// tool.profile.reminders.delete — Supprime un rappel.
pub fn delete(
    ctx: &GovernedContext,
    user_id: &str,
    reminder_id: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = reminders_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("reminders lock".into()))?;
    let entries = guard
        .get_mut(user_id)
        .ok_or_else(|| MiyuprofileError::NotFound("reminders user".into()))?;
    let len_before = entries.len();
    entries.retain(|r| r.id != reminder_id);
    if entries.len() == len_before {
        return Err(MiyuprofileError::NotFound(format!("reminder:{reminder_id}")));
    }
    Ok(())
}

/// tool.profile.reminders.overdue — Rappels en retard (due_date < reference_date).
pub fn overdue(
    ctx: &GovernedContext,
    user_id: &str,
    reference_date: &str,
) -> Result<Vec<ReminderEntry>, MiyuprofileError> {
    let all = pending(ctx, user_id)?;
    Ok(all.into_iter().filter(|r| r.due_date.as_str() < reference_date).collect())
}
