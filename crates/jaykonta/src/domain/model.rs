//! Modele de domaine JayKonta (MVP implementation).

use crate::theme::FeatureStatus;

/// Point d'entree fonctionnel JayKonta.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntryPoint {
    /// Vue entreprise (JayKonta).
    Account,
    /// Vue personnel (JayBudget/Purse).
    Purse,
}

impl EntryPoint {
    /// Label lisible en UI.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Account => "Account",
            Self::Purse => "Purse",
        }
    }
}

/// Sections de navigation du dashboard.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DashboardSection {
    /// Vue globale.
    Overview,
    /// Devis et factures.
    QuotesInvoices,
    /// Comptabilite coeur.
    Ledger,
    /// Paiements.
    Payments,
    /// Rapports et export.
    Reports,
    /// Integrations inter-services.
    Integrations,
    /// Contrats normatifs.
    Contracts,
    /// Toolkits disponibles.
    Toolkits,
    /// Operateurs metier.
    Operators,
    /// Bornage implementation.
    Implementation,
}

impl DashboardSection {
    /// Titre de la section.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Overview => "Vue globale",
            Self::QuotesInvoices => "Devis & Factures",
            Self::Ledger => "Comptabilite",
            Self::Payments => "Paiements",
            Self::Reports => "Rapports",
            Self::Integrations => "Integrations",
            Self::Contracts => "Contrats",
            Self::Toolkits => "Toolkits",
            Self::Operators => "Operateurs",
            Self::Implementation => "Bornage",
        }
    }

    /// Liste ordonnee des sections affichees dans la sidebar.
    pub const ALL: [Self; 10] = [
        Self::Overview,
        Self::QuotesInvoices,
        Self::Ledger,
        Self::Payments,
        Self::Reports,
        Self::Integrations,
        Self::Contracts,
        Self::Toolkits,
        Self::Operators,
        Self::Implementation,
    ];
}

/// Element KPI.
#[derive(Clone)]
pub struct KpiMetric {
    /// Label KPI.
    pub label: &'static str,
    /// Valeur principale.
    pub value: String,
    /// Delta ou detail.
    pub delta: String,
}

/// Action rapide.
#[derive(Clone, Copy)]
pub struct QuickAction {
    /// Intitule d'action.
    pub label: &'static str,
    /// Etat d'action.
    pub state: &'static str,
}

/// Fonctionnalite strategique.
#[derive(Clone, Copy)]
pub struct Capability {
    /// Nom de module.
    pub name: &'static str,
    /// Statut d'avancement.
    pub status: FeatureStatus,
}

/// Flux d'integration vers ou depuis JayKonta.
#[derive(Clone, Copy)]
pub struct IntegrationFlow {
    /// Origine du flux.
    pub source: &'static str,
    /// Evenement/operation.
    pub operation: &'static str,
    /// Contrat associe.
    pub contract: &'static str,
    /// Etat de readiness.
    pub state: &'static str,
}

/// Signal de risque et conformite.
#[derive(Clone, Copy)]
pub struct RiskSignal {
    /// Nom du controle.
    pub label: &'static str,
    /// Valeur de signal.
    pub value: &'static str,
    /// Severity textuelle.
    pub severity: &'static str,
}

/// Contrat fonctionnel ou technique.
#[derive(Clone, Copy)]
pub struct ContractItem {
    /// Identifiant contrat.
    pub id: &'static str,
    /// Description courte.
    pub summary: &'static str,
    /// Statut implementation.
    pub status: &'static str,
    /// Portee principale.
    pub scope: &'static str,
}

/// Toolkit fonctionnel expose par JayKonta.
#[derive(Clone, Copy)]
pub struct ToolkitItem {
    /// Nom toolkit.
    pub name: &'static str,
    /// Capacite principale.
    pub capability: &'static str,
    /// Public principal.
    pub audience: &'static str,
    /// Etat.
    pub status: &'static str,
}

/// Operateur metier JayKonta.
#[derive(Clone, Copy)]
pub struct OperatorItem {
    /// Nom operateur.
    pub name: &'static str,
    /// Responsabilite metier.
    pub responsibility: &'static str,
    /// Public cible.
    pub audience: &'static str,
    /// Etat implementation.
    pub status: &'static str,
}

/// Element de bornage implementation.
#[derive(Clone, Copy)]
pub struct ImplementationItem {
    /// Phase.
    pub phase: &'static str,
    /// In scope.
    pub in_scope: &'static str,
    /// Hors scope.
    pub out_scope: &'static str,
    /// Critere de sortie.
    pub exit_criteria: &'static str,
}

/// Modele de vue complet consomme par l'UI.
pub struct DashboardViewModel {
    /// Titre hero.
    pub hero_title: &'static str,
    /// Sous-titre hero.
    pub hero_subtitle: &'static str,
    /// Score de sante operationnelle.
    pub health_score: &'static str,
    /// Alerte principale.
    pub top_alert: &'static str,
    /// KPI.
    pub kpis: Vec<KpiMetric>,
    /// Actions rapides.
    pub quick_actions: Vec<QuickAction>,
    /// Fonctionnalites strat.
    pub capabilities: Vec<Capability>,
    /// Flux integration.
    pub integrations: Vec<IntegrationFlow>,
    /// Risques.
    pub risks: Vec<RiskSignal>,
    /// Contrats.
    pub contracts: Vec<ContractItem>,
    /// Toolkits.
    pub toolkits: Vec<ToolkitItem>,
    /// Operateurs.
    pub operators: Vec<OperatorItem>,
    /// Bornage.
    pub implementation: Vec<ImplementationItem>,
}
