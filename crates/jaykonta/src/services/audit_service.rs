//! AuditService — journalisation des ecritures (CK-AUD-01).
//!
//! Toute operation d'ecriture (mouvement, devis, facture, paiement)
//! est journalisee dans la table `audit` avec :
//! - Le contrat concerne (CK-xxx)
//! - L'acteur (user ou service)
//! - L'operation effectuee
//! - Le scope (purse/account)
//! - L'objet cible
//! - Le resultat (ok/error/denied)
//! - Le payload serialise JSON

use crate::data::{AuditRecord, DbError, JayKontaDb};
use chrono::Local;
use std::sync::Arc;

/// Service d'audit JayKonta.
pub struct AuditService {
    db: Arc<JayKontaDb>,
}

/// Resultat d'une operation auditee.
#[derive(Debug, Clone, Copy)]
pub enum AuditResult {
    /// Succes.
    Ok,
    /// Erreur technique.
    Error,
    /// Refuse (permission).
    Denied,
}

impl AuditResult {
    /// Chaine pour la DB.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Error => "error",
            Self::Denied => "denied",
        }
    }
}

impl AuditService {
    /// Cree le service d'audit.
    pub fn new(db: Arc<JayKontaDb>) -> Self {
        Self { db }
    }

    /// Journalise une operation (CK-AUD-01).
    pub fn log(
        &self,
        contract_id: &str,
        actor_ref: &str,
        operation: &str,
        scope: &str,
        object_ref: &str,
        result: AuditResult,
        payload: Option<&str>,
    ) -> Result<(), DbError> {
        let record = AuditRecord {
            id: uuid::Uuid::new_v4().to_string(),
            contract_id: contract_id.to_string(),
            actor_ref: actor_ref.to_string(),
            operation: operation.to_string(),
            scope: scope.to_string(),
            object_ref: object_ref.to_string(),
            result: result.as_str().to_string(),
            payload_json: payload.unwrap_or("{}").to_string(),
            created_at: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        };
        self.db.insert_audit(&record)
    }

    /// Raccourci : log un mouvement cree.
    pub fn log_movement_created(&self, actor: &str, movement_id: &str) -> Result<(), DbError> {
        self.log(
            "CK-AUD-01",
            actor,
            "movement.create",
            "purse",
            movement_id,
            AuditResult::Ok,
            None,
        )
    }

    /// Raccourci : log un budget cree.
    pub fn log_budget_created(&self, actor: &str, budget_id: &str) -> Result<(), DbError> {
        self.log(
            "CK-AUD-01",
            actor,
            "budget.create",
            "purse",
            budget_id,
            AuditResult::Ok,
            None,
        )
    }

    /// Raccourci : log un objectif cree.
    pub fn log_goal_created(&self, actor: &str, goal_id: &str) -> Result<(), DbError> {
        self.log(
            "CK-AUD-01",
            actor,
            "goal.create",
            "purse",
            goal_id,
            AuditResult::Ok,
            None,
        )
    }

    /// Raccourci : log un devis cree (Account).
    pub fn log_quote_created(&self, actor: &str, quote_id: &str) -> Result<(), DbError> {
        self.log(
            "CK-AUD-01",
            actor,
            "quote.create",
            "account",
            quote_id,
            AuditResult::Ok,
            None,
        )
    }

    /// Raccourci : log une facture emise (Account).
    pub fn log_invoice_issued(&self, actor: &str, invoice_id: &str) -> Result<(), DbError> {
        self.log(
            "CK-AUD-01",
            actor,
            "invoice.emit",
            "account",
            invoice_id,
            AuditResult::Ok,
            None,
        )
    }

    /// Raccourci : log un paiement enregistre (Account).
    pub fn log_payment_recorded(&self, actor: &str, payment_id: &str) -> Result<(), DbError> {
        self.log(
            "CK-AUD-01",
            actor,
            "payment.record",
            "account",
            payment_id,
            AuditResult::Ok,
            None,
        )
    }

    /// Nombre total d'entrees d'audit.
    pub fn count(&self) -> Result<i64, DbError> {
        self.db.audit_count()
    }
}
