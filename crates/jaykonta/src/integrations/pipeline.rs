//! Pipeline d'integration CK-INT-01/02/03 + audit.

use crate::data::{AuditRecord, InvoiceRecord, JayKontaDb, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord};
use crate::integrations::contracts::{
    IntegrationError, JayFestivalEvent, JayKoaReminderEvent, JayRDVEvent,
};
use serde::Serialize;
use std::sync::Arc;

/// Service de pipeline integration.
pub struct IntegrationPipeline {
    db: Arc<JayKontaDb>,
}

impl IntegrationPipeline {
    /// Cree le pipeline pour une DB JayKonta.
    pub fn new(db: Arc<JayKontaDb>) -> Self {
        Self { db }
    }

    /// Applique un flux CK-INT-01 (JayFestival -> JayKonta).
    pub fn apply_jayfestival(&self, event: JayFestivalEvent) -> Result<(), IntegrationError> {
        match event {
            JayFestivalEvent::QuoteCreate(meta, payload) => {
                self.db
                    .insert_quote(&QuoteRecord {
                        id: payload.quote_id.clone(),
                        scope: payload.scope.clone(),
                        context_ref: payload.context_ref.clone(),
                        counterparty_ref: payload.counterparty_ref.clone(),
                        total: payload.total,
                        currency: payload.currency.clone(),
                        status: "sent".to_string(),
                        created_at: meta.occurred_at.clone(),
                    })
                    .map_err(|e| IntegrationError(e.to_string()))?;

                self.audit(&meta, "quote.create", &payload.scope, &payload.quote_id, "ok", &payload)
            }
            JayFestivalEvent::InvoiceEmit(meta, payload) => {
                self.db
                    .insert_invoice(&InvoiceRecord {
                        id: payload.invoice_id.clone(),
                        scope: payload.scope.clone(),
                        context_ref: payload.context_ref.clone(),
                        counterparty_ref: payload.counterparty_ref.clone(),
                        quote_id: payload.quote_id.clone(),
                        total: payload.total,
                        paid_amount: 0.0,
                        currency: payload.currency.clone(),
                        status: "issued".to_string(),
                        issued_at: meta.occurred_at.clone(),
                        due_at: payload.due_at.clone(),
                    })
                    .map_err(|e| IntegrationError(e.to_string()))?;

                self.audit(
                    &meta,
                    "invoice.emit",
                    &payload.scope,
                    &payload.invoice_id,
                    "ok",
                    &payload,
                )
            }
            JayFestivalEvent::BudgetMovementRecord(meta, payload) => {
                self.db
                    .insert_movement(&MovementRecord {
                        id: payload.movement_id.clone(),
                        scope: payload.scope.clone(),
                        context_ref: payload.context_ref.clone(),
                        category: payload.category.clone(),
                        amount: payload.amount,
                        currency: payload.currency.clone(),
                        movement_date: payload.movement_date.clone(),
                        source_service: "jayfestival".to_string(),
                    })
                    .map_err(|e| IntegrationError(e.to_string()))?;

                self.audit(
                    &meta,
                    "budget.movements.record",
                    &payload.scope,
                    &payload.movement_id,
                    "ok",
                    &payload,
                )
            }
            JayFestivalEvent::ReportByEdition(meta, payload) => self.audit(
                &meta,
                "report.by_edition",
                &payload.scope,
                &payload.edition_ref,
                "ok",
                &payload,
            ),
        }
    }

    /// Applique un flux CK-INT-02 (JayRDV -> JayKonta).
    pub fn apply_jayrdv(&self, event: JayRDVEvent) -> Result<(), IntegrationError> {
        match event {
            JayRDVEvent::QuoteCreate(meta, payload) => {
                self.db
                    .insert_quote(&QuoteRecord {
                        id: payload.quote_id.clone(),
                        scope: payload.scope.clone(),
                        context_ref: payload.context_ref.clone(),
                        counterparty_ref: payload.counterparty_ref.clone(),
                        total: payload.total,
                        currency: payload.currency.clone(),
                        status: "sent".to_string(),
                        created_at: meta.occurred_at.clone(),
                    })
                    .map_err(|e| IntegrationError(e.to_string()))?;

                self.audit(&meta, "quote.create", &payload.scope, &payload.quote_id, "ok", &payload)
            }
            JayRDVEvent::InvoiceEmit(meta, payload) => {
                self.db
                    .insert_invoice(&InvoiceRecord {
                        id: payload.invoice_id.clone(),
                        scope: payload.scope.clone(),
                        context_ref: payload.context_ref.clone(),
                        counterparty_ref: payload.counterparty_ref.clone(),
                        quote_id: payload.quote_id.clone(),
                        total: payload.total,
                        paid_amount: 0.0,
                        currency: payload.currency.clone(),
                        status: "issued".to_string(),
                        issued_at: meta.occurred_at.clone(),
                        due_at: payload.due_at.clone(),
                    })
                    .map_err(|e| IntegrationError(e.to_string()))?;

                self.audit(
                    &meta,
                    "invoice.emit",
                    &payload.scope,
                    &payload.invoice_id,
                    "ok",
                    &payload,
                )
            }
            JayRDVEvent::PaymentRecord(meta, payload) => {
                self.db
                    .insert_payment_and_update_invoice(&PaymentRecord {
                        id: payload.payment_id.clone(),
                        invoice_id: payload.invoice_id.clone(),
                        amount: payload.amount,
                        currency: payload.currency.clone(),
                        paid_at: payload.paid_at.clone(),
                        method: payload.method.clone(),
                        reference_opaque: payload.reference_opaque.clone(),
                    })
                    .map_err(|e| IntegrationError(e.to_string()))?;

                self.audit(
                    &meta,
                    "payment.record",
                    "account",
                    &payload.payment_id,
                    "ok",
                    &payload,
                )
            }
            JayRDVEvent::ReportByProfessional(meta, payload) => self.audit(
                &meta,
                "report.by_professional",
                &payload.scope,
                &payload.professional_ref,
                "ok",
                &payload,
            ),
        }
    }

    /// Applique un flux CK-INT-03 (JayKoa reminder publish).
    pub fn apply_jaykoa(&self, event: JayKoaReminderEvent) -> Result<(), IntegrationError> {
        self.db
            .insert_reminder(&ReminderRecord {
                id: uuid::Uuid::new_v4().to_string(),
                deadline_ref: event.payload.deadline_ref.clone(),
                due_at: event.payload.due_at.clone(),
                label: event.payload.label.clone(),
                context_ref: event.payload.context_ref.clone(),
                source_service: "jaykoa".to_string(),
            })
            .map_err(|e| IntegrationError(e.to_string()))?;

        self.audit(
            &event.meta,
            "deadline.reminder.publish",
            "purse",
            &event.payload.deadline_ref,
            "ok",
            &event.payload,
        )
    }

    fn audit<T: Serialize>(
        &self,
        meta: &crate::integrations::contracts::IntegrationMeta,
        operation: &str,
        scope: &str,
        object_ref: &str,
        result: &str,
        payload: &T,
    ) -> Result<(), IntegrationError> {
        let payload_json =
            serde_json::to_string(payload).map_err(|e| IntegrationError(e.to_string()))?;
        self.db
            .insert_audit(&AuditRecord {
                id: uuid::Uuid::new_v4().to_string(),
                contract_id: meta.contract_id.clone(),
                actor_ref: meta.actor_ref.clone(),
                operation: operation.to_string(),
                scope: scope.to_string(),
                object_ref: object_ref.to_string(),
                result: result.to_string(),
                payload_json,
                created_at: meta.occurred_at.clone(),
            })
            .map_err(|e| IntegrationError(e.to_string()))
    }
}
