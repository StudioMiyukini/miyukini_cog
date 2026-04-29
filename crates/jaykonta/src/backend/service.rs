//! Backend JayKonta : lecture depuis persistance + ingestion des flux CK-INT.

use crate::data::{DbError, JayKontaDb};
use crate::domain::model::{
    Capability, ContractItem, DashboardSection, DashboardViewModel, EntryPoint, ImplementationItem,
    IntegrationFlow, KpiMetric, OperatorItem, QuickAction, RiskSignal, ToolkitItem,
};
use crate::integrations::contracts::{
    DeadlineReminderPayload, IntegrationError, IntegrationMeta, InvoiceEmitPayload,
    JayKoaReminderEvent, JayRDVEvent, PaymentRecordPayload, QuoteCreatePayload,
    ReportByProfessionalPayload,
};
use crate::integrations::pipeline::IntegrationPipeline;
use crate::theme::FeatureStatus;
use chrono::Local;
use std::sync::Arc;

/// Backend applicatif JayKonta.
pub struct JayKontaBackend {
    entry_point: EntryPoint,
    section: DashboardSection,
    db: Arc<JayKontaDb>,
}

impl JayKontaBackend {
    /// Cree le backend avec un chemin DB explicite.
    pub fn new_with_db(path: &str) -> Result<Self, DbError> {
        let db = Arc::new(JayKontaDb::open(path)?);
        let backend = Self {
            entry_point: EntryPoint::Account,
            section: DashboardSection::Overview,
            db,
        };
        backend.bootstrap_if_needed()?;
        Ok(backend)
    }

    /// Retourne le point d'entree actif.
    pub const fn entry_point(&self) -> EntryPoint {
        self.entry_point
    }

    /// Retourne la section active.
    pub const fn section(&self) -> DashboardSection {
        self.section
    }

    /// Modifie le point d'entree actif.
    pub fn set_entry_point(&mut self, entry_point: EntryPoint) {
        self.entry_point = entry_point;
    }

    /// Modifie la section active.
    pub fn set_section(&mut self, section: DashboardSection) {
        self.section = section;
    }

    /// Construit le modele de vue courant.
    pub fn view_model(&self) -> DashboardViewModel {
        let audit_count = self.db.audit_count().unwrap_or(0);
        match self.entry_point {
            EntryPoint::Account => account_view_model(self.db.as_ref(), audit_count),
            EntryPoint::Purse => purse_view_model(self.db.as_ref(), audit_count),
        }
    }

    /// Ingestion reelle CK-INT-02 (JayRDV -> JayKonta).
    pub fn ingest_jayrdv_event(&self, event: JayRDVEvent) -> Result<(), IntegrationError> {
        IntegrationPipeline::new(self.db.clone()).apply_jayrdv(event)
    }

    /// Ingestion reelle CK-INT-03 (JayKoa -> JayKonta).
    pub fn ingest_jaykoa_event(&self, event: JayKoaReminderEvent) -> Result<(), IntegrationError> {
        IntegrationPipeline::new(self.db.clone()).apply_jaykoa(event)
    }

    fn bootstrap_if_needed(&self) -> Result<(), DbError> {
        if !self.db.is_bootstrap_needed()? {
            return Ok(());
        }
        let pipeline = IntegrationPipeline::new(self.db.clone());
        bootstrap_int_02(&pipeline);
        bootstrap_int_03(&pipeline);
        Ok(())
    }
}

impl Default for JayKontaBackend {
    fn default() -> Self {
        Self::new_with_db("jaykonta.db").unwrap_or_else(|e| {
            panic!(
                "JayKonta: impossible d'ouvrir la base SQLite (KindMother fille): {}",
                e
            )
        })
    }
}

fn account_view_model(db: &JayKontaDb, audit_count: i64) -> DashboardViewModel {
    let stats = db.account_stats().unwrap_or_default();
    let payment_rate = format!("{:.0}%", stats.payment_rate_pct);

    DashboardViewModel {
        hero_title: "Pilotage financier unifie Account",
        hero_subtitle:
            "Cycle devis -> facture -> paiement, avec comptabilite coeur et reporting legal.",
        health_score: "91 / 100",
        top_alert: "Top urgence : 3 factures en relance J+15",
        kpis: vec![
            KpiMetric {
                label: "CA Mensuel",
                value: format_eur(stats.ca_mensuel),
                delta: "base sur factures emises ce mois".to_string(),
            },
            KpiMetric {
                label: "Impayes",
                value: format_eur(stats.impayes),
                delta: "suivi factures issued/partial/overdue".to_string(),
            },
            KpiMetric {
                label: "Factures emises",
                value: stats.invoices_count.to_string(),
                delta: format!("Taux paiement {}", payment_rate),
            },
            KpiMetric {
                label: "Couverture audit",
                value: format!("{} evenements", audit_count),
                delta: "CK-AUD-01/02/03 journalises".to_string(),
            },
        ],
        quick_actions: vec![
            QuickAction {
                label: "Creer devis",
                state: "Ready",
            },
            QuickAction {
                label: "Convertir en facture",
                state: "Ready",
            },
            QuickAction {
                label: "Enregistrer paiement",
                state: "Ready",
            },
            QuickAction {
                label: "Exporter rapport legal",
                state: "Review",
            },
            QuickAction {
                label: "Declencher relance",
                state: "Ready",
            },
        ],
        capabilities: vec![
            Capability {
                name: "Comptabilite coeur",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Cycle devis -> facture",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Paiements et statuts",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Reporting legal",
                status: FeatureStatus::Build,
            },
            Capability {
                name: "Integrations JayRDV",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Rapprochement bancaire",
                status: FeatureStatus::Locked,
            },
        ],
        integrations: common_integrations(),
        risks: vec![
            RiskSignal {
                label: "Mandats expirants (24h)",
                value: "4",
                severity: "warn",
            },
            RiskSignal {
                label: "Exports hors scope bloques",
                value: "12",
                severity: "ok",
            },
            RiskSignal {
                label: "Echecs paiement token",
                value: "2",
                severity: "risk",
            },
            RiskSignal {
                label: "Niveau securite courant",
                value: "WS-2",
                severity: "ok",
            },
        ],
        contracts: common_contracts(),
        toolkits: account_toolkits(),
        operators: account_operators(),
        implementation: implementation_items(),
    }
}

fn purse_view_model(db: &JayKontaDb, audit_count: i64) -> DashboardViewModel {
    let stats = db.purse_stats().unwrap_or_default();
    DashboardViewModel {
        hero_title: "Pilotage budget personnel Purse",
        hero_subtitle:
            "Mouvements, categories, budgets occasionnels, objectifs et alertes intelligentes.",
        health_score: "88 / 100",
        top_alert: "2 categories depassent le seuil hebdo",
        kpis: vec![
            KpiMetric {
                label: "Depenses mensuelles",
                value: format_eur(stats.monthly_expenses),
                delta: "somme mouvements purse du mois".to_string(),
            },
            KpiMetric {
                label: "Budget utilise",
                value: format!("{:.0}%", stats.budget_usage_pct),
                delta: "base budget_in vs depenses".to_string(),
            },
            KpiMetric {
                label: "Objectifs actifs",
                value: stats.active_goals.to_string(),
                delta: "rappels CK-INT-03 en file".to_string(),
            },
            KpiMetric {
                label: "Mouvements classes",
                value: format!("{:.0}%", stats.categorized_pct),
                delta: format!("{} evenements audit", audit_count),
            },
        ],
        quick_actions: vec![
            QuickAction {
                label: "Ajouter mouvement",
                state: "Ready",
            },
            QuickAction {
                label: "Creer budget occasionnel",
                state: "Ready",
            },
            QuickAction {
                label: "Mettre a jour objectif",
                state: "Ready",
            },
            QuickAction {
                label: "Exporter rapport Purse",
                state: "Review",
            },
            QuickAction {
                label: "Publier rappel JayKoa",
                state: "Optional",
            },
        ],
        capabilities: vec![
            Capability {
                name: "Journal mouvements",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Budgets occasionnels",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Objectifs et alertes",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Rapports et export",
                status: FeatureStatus::Build,
            },
            Capability {
                name: "Rappels JayKoa",
                status: FeatureStatus::Live,
            },
            Capability {
                name: "Open banking perso",
                status: FeatureStatus::Locked,
            },
        ],
        integrations: common_integrations(),
        risks: vec![
            RiskSignal {
                label: "Alertes non lues",
                value: "3",
                severity: "warn",
            },
            RiskSignal {
                label: "Mouvements sans categorie",
                value: "5",
                severity: "warn",
            },
            RiskSignal {
                label: "Export sensible bloque",
                value: "9",
                severity: "ok",
            },
            RiskSignal {
                label: "Niveau securite courant",
                value: "WS-2",
                severity: "ok",
            },
        ],
        contracts: common_contracts(),
        toolkits: purse_toolkits(),
        operators: purse_operators(),
        implementation: implementation_items(),
    }
}

fn bootstrap_int_02(pipeline: &IntegrationPipeline) {
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let invoice_id = uuid::Uuid::new_v4().to_string();
    let meta = IntegrationMeta {
        contract_id: "CK-INT-02".to_string(),
        actor_ref: "jayrdv:billing-bot".to_string(),
        occurred_at: now.clone(),
    };
    let _ = pipeline.apply_jayrdv(JayRDVEvent::QuoteCreate(
        meta.clone(),
        QuoteCreatePayload {
            quote_id: uuid::Uuid::new_v4().to_string(),
            scope: "account".to_string(),
            context_ref: "professional:kinetic-care".to_string(),
            counterparty_ref: "customer:patient-42".to_string(),
            total: 120.0,
            currency: "EUR".to_string(),
        },
    ));
    let _ = pipeline.apply_jayrdv(JayRDVEvent::InvoiceEmit(
        meta.clone(),
        InvoiceEmitPayload {
            invoice_id: invoice_id.clone(),
            scope: "account".to_string(),
            context_ref: "professional:kinetic-care".to_string(),
            counterparty_ref: "customer:patient-42".to_string(),
            quote_id: None,
            total: 120.0,
            currency: "EUR".to_string(),
            due_at: Some("2026-03-15T23:59:59".to_string()),
        },
    ));
    let _ = pipeline.apply_jayrdv(JayRDVEvent::PaymentRecord(
        meta.clone(),
        PaymentRecordPayload {
            payment_id: uuid::Uuid::new_v4().to_string(),
            invoice_id,
            amount: 120.0,
            currency: "EUR".to_string(),
            method: "card_token".to_string(),
            reference_opaque: "tok_7f9a2".to_string(),
            paid_at: now.clone(),
        },
    ));
    let _ = pipeline.apply_jayrdv(JayRDVEvent::ReportByProfessional(
        meta,
        ReportByProfessionalPayload {
            professional_ref: "professional:kinetic-care".to_string(),
            scope: "account".to_string(),
        },
    ));
}

fn bootstrap_int_03(pipeline: &IntegrationPipeline) {
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let _ = pipeline.apply_jaykoa(JayKoaReminderEvent {
        meta: IntegrationMeta {
            contract_id: "CK-INT-03".to_string(),
            actor_ref: "jaykoa:reminder-engine".to_string(),
            occurred_at: now.clone(),
        },
        payload: DeadlineReminderPayload {
            deadline_ref: "invoice:overdue:acme-2026-03".to_string(),
            due_at: "2026-03-20T09:00:00".to_string(),
            label: "Relance facture Acme".to_string(),
            context_ref: "purse-goal:month-control".to_string(),
        },
    });
    let _ = now;
}

fn common_integrations() -> Vec<IntegrationFlow> {
    vec![
        IntegrationFlow {
            source: "JayRDV",
            operation: "invoice.emit",
            contract: "CK-INT-02",
            state: "active",
        },
        IntegrationFlow {
            source: "JayRDV",
            operation: "payment.record",
            contract: "CK-INT-02",
            state: "active",
        },
        IntegrationFlow {
            source: "JayKoa",
            operation: "deadline.reminder.publish",
            contract: "CK-INT-03",
            state: "active",
        },
    ]
}

fn common_contracts() -> Vec<ContractItem> {
    vec![
        ContractItem {
            id: "CK-SVC-01",
            summary: "Unicite de service COG",
            status: "active",
            scope: "service",
        },
        ContractItem {
            id: "CK-SVC-02",
            summary: "Separation Purse et Account",
            status: "active",
            scope: "service",
        },
        ContractItem {
            id: "CK-SVC-03",
            summary: "Source de verite comptable",
            status: "active",
            scope: "service",
        },
        ContractItem {
            id: "CK-SEC-02",
            summary: "Mandat obligatoire ecriture",
            status: "active",
            scope: "securite",
        },
        ContractItem {
            id: "CK-AUD-01",
            summary: "Journalisation des ecritures",
            status: "active",
            scope: "audit",
        },
        ContractItem {
            id: "CK-AUD-03",
            summary: "Trace devis -> facture",
            status: "active",
            scope: "audit",
        },
    ]
}

fn account_toolkits() -> Vec<ToolkitItem> {
    vec![
        ToolkitItem {
            name: "TK-AUTH-CONTEXT",
            capability: "auth + contexte Account",
            audience: "Account",
            status: "live",
        },
        ToolkitItem {
            name: "TK-LEDGER",
            capability: "journal + balances",
            audience: "Account",
            status: "live",
        },
        ToolkitItem {
            name: "TK-QUOTES",
            capability: "devis et statuts",
            audience: "Account",
            status: "live",
        },
        ToolkitItem {
            name: "TK-INVOICES",
            capability: "facturation + numerotation",
            audience: "Account",
            status: "live",
        },
        ToolkitItem {
            name: "TK-PAYMENTS",
            capability: "paiements + rapprochement",
            audience: "Account",
            status: "live",
        },
        ToolkitItem {
            name: "TK-REPORTING",
            capability: "exports legaux",
            audience: "Account",
            status: "build",
        },
    ]
}

fn purse_toolkits() -> Vec<ToolkitItem> {
    vec![
        ToolkitItem {
            name: "TK-AUTH-CONTEXT",
            capability: "auth + contexte Purse",
            audience: "Purse",
            status: "live",
        },
        ToolkitItem {
            name: "TK-LEDGER",
            capability: "mouvements + categories",
            audience: "Purse",
            status: "live",
        },
        ToolkitItem {
            name: "TK-BUDGET-PURSE",
            capability: "budgets occasionnels",
            audience: "Purse",
            status: "live",
        },
        ToolkitItem {
            name: "TK-GOALS-PURSE",
            capability: "objectifs et alertes",
            audience: "Purse",
            status: "live",
        },
        ToolkitItem {
            name: "TK-REPORTING",
            capability: "rapports et export",
            audience: "Purse",
            status: "build",
        },
        ToolkitItem {
            name: "TK-REMINDERS",
            capability: "rappels JayKoa",
            audience: "Purse",
            status: "live",
        },
    ]
}

fn account_operators() -> Vec<OperatorItem> {
    vec![
        OperatorItem {
            name: "OP-ACCOUNT-CORE",
            responsibility: "onboarding + contexte + ledger",
            audience: "Account",
            status: "live",
        },
        OperatorItem {
            name: "OP-ACCOUNT-BILLING",
            responsibility: "devis, factures, encaissements",
            audience: "Account",
            status: "live",
        },
        OperatorItem {
            name: "OP-ACCOUNT-INTEGRATION",
            responsibility: "pipeline JayRDV",
            audience: "Account",
            status: "live",
        },
    ]
}

fn purse_operators() -> Vec<OperatorItem> {
    vec![
        OperatorItem {
            name: "OP-PURSE-CORE",
            responsibility: "mouvements, categories, dashboard",
            audience: "Purse",
            status: "live",
        },
        OperatorItem {
            name: "OP-PURSE-PLANNING",
            responsibility: "budgets et objectifs",
            audience: "Purse",
            status: "live",
        },
        OperatorItem {
            name: "OP-PURSE-REMINDER",
            responsibility: "rappels et notifications",
            audience: "Purse",
            status: "live",
        },
    ]
}

fn implementation_items() -> Vec<ImplementationItem> {
    vec![
        ImplementationItem {
            phase: "Phase 1",
            in_scope: "Purse coeur, Account coeur, devis/factures, paiements initiaux, integr. JayRDV",
            out_scope: "rapprochement bancaire avance, portail client externe",
            exit_criteria: "Parcours P1..P6 couverts + contrats CK-SVC/CK-OP/CK-TK essentiels",
        },
        ImplementationItem {
            phase: "Phase 2",
            in_scope: "extension reporting, rapprochement bancaire, automatisations",
            out_scope: "industrialisation complete multi-tenant",
            exit_criteria: "Maturite Build -> Live sur reporting, paiements et integr. avancees",
        },
        ImplementationItem {
            phase: "Phase 3",
            in_scope: "industrialisation, federation inter-COG, portail externe",
            out_scope: "fonctions hors domaine comptable",
            exit_criteria: "SLA production + gouvernance audit complete + performance cible",
        },
    ]
}

fn format_eur(amount: f64) -> String {
    format!("{amount:.0} EUR")
}
