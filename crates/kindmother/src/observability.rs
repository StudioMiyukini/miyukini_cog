//! Observability & Audit - Observabilité et audit de KindMother
//!
//! Implémente l'observabilité conforme au contrat FONDATION :
//! "KindMother — Observability & Audit Contract"
//!
//! CARACTÉRISTIQUES :
//! - Événements conceptuels observables
//! - Journal d'intention immuable
//! - Traçabilité des rejets
//! - Traçabilité des quarantaines
//! - Aucune donnée métier exposée
//! - Aucune dépendance technique externe

use crate::lifecycle::WriteIntentState;
use crate::state::KMState;

/// Catégorie d'événement observable
///
/// Conforme au contrat FONDATION section 3.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCategory {
    /// Événements liés aux intentions
    Intent,
    /// Événements d'écriture
    Write,
    /// Événements de synchronisation
    Sync,
    /// Événements d'autorité
    Authority,
    /// Événements de sécurité
    Security,
    /// Événements d'échec
    Failure,
    /// Événements de cycle de vie
    Lifecycle,
}

impl EventCategory {
    /// Retourne une représentation textuelle
    pub fn to_string(&self) -> &'static str {
        match self {
            EventCategory::Intent => "INTENTION",
            EventCategory::Write => "ÉCRITURE",
            EventCategory::Sync => "SYNCHRONISATION",
            EventCategory::Authority => "AUTORITÉ",
            EventCategory::Security => "SÉCURITÉ",
            EventCategory::Failure => "ÉCHEC",
            EventCategory::Lifecycle => "CYCLE_DE_VIE",
        }
    }
}

/// Type d'événement observable
///
/// Conforme au contrat FONDATION section 3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservableEvent {
    // Catégorie 1 : Événements d'intention
    /// Création d'une intention
    IntentCreated { intent_id: String },
    /// Validation d'une intention
    IntentValidated { intent_id: String },
    /// Rejet d'une intention
    IntentRejected { intent_id: String, reason: String },
    /// Acceptation d'une intention
    IntentAccepted { intent_id: String },

    // Catégorie 2 : Événements d'écriture
    /// Application d'une écriture
    WriteApplied { intent_id: String },
    /// Persistance confirmée
    PersistenceConfirmed { intent_id: String, domain_id: String },
    /// Modification d'état
    StateModified { intent_id: String, description: String },

    // Catégorie 3 : Événements de synchronisation
    /// Déclenchement de synchronisation
    SyncTriggered { sync_id: String },
    /// Soumission d'intention à la Mère
    SyncSubmitted { sync_id: String },
    /// Validation par la Mère
    SyncValidated { sync_id: String },
    /// Conflit détecté
    ConflictDetected { sync_id: String, conflict_type: String },
    /// Synchronisation rejetée
    SyncRejected { sync_id: String, reason: String },

    // Catégorie 4 : Événements d'autorité
    /// Décision d'autorité (Mère)
    AuthorityDecision { decision_id: String, decision: String },
    /// Attribution de confiance
    TrustAttributed { target: String, level: String },
    /// Révocation de confiance
    TrustRevoked { target: String, reason: String },
    /// Passage d'Intention Certifiée
    CertifiedIntentPassed { intent_id: String, source_domain: String, target_domain: String },

    // Catégorie 5 : Événements de sécurité
    /// Détection de violation
    ViolationDetected { violation_type: String, source: String },
    /// Tentative de contournement
    BypassAttempt { source: String, description: String },
    /// Mise en quarantaine
    QuarantineEntered { entity_id: String, reason: String },
    /// Sortie de quarantaine
    QuarantineExited { entity_id: String, reason: String },
    /// Menace détectée
    ThreatDetected { threat_id: String, threat_type: String, source: String },

    // Catégorie 6 : Événements d'échec
    /// Détection de corruption
    CorruptionDetected { description: String },
    /// Déclenchement de dégradation
    DegradationTriggered { reason: String },
    /// Sortie de dégradation
    DegradationExited { reason: String },
    /// Panne de synchronisation
    SyncFailure { sync_id: String, reason: String },
    /// Récupération
    RecoveryCompleted { description: String },

    // Catégorie 7 : Événements de cycle de vie
    /// Initialisation d'instance
    InstanceInitialized { instance_id: String },
    /// Arrêt d'instance
    InstanceStopped { instance_id: String },
    /// Changement d'état de l'instance
    StateChanged { from: KMState, to: KMState, reason: String },
}

impl ObservableEvent {
    /// Obtient la catégorie de l'événement
    pub fn category(&self) -> EventCategory {
        match self {
            ObservableEvent::IntentCreated { .. }
            | ObservableEvent::IntentValidated { .. }
            | ObservableEvent::IntentRejected { .. }
            | ObservableEvent::IntentAccepted { .. } => EventCategory::Intent,

            ObservableEvent::WriteApplied { .. }
            | ObservableEvent::PersistenceConfirmed { .. }
            | ObservableEvent::StateModified { .. } => EventCategory::Write,

            ObservableEvent::SyncTriggered { .. }
            | ObservableEvent::SyncSubmitted { .. }
            | ObservableEvent::SyncValidated { .. }
            | ObservableEvent::ConflictDetected { .. }
            | ObservableEvent::SyncRejected { .. } => EventCategory::Sync,

            ObservableEvent::AuthorityDecision { .. }
            | ObservableEvent::TrustAttributed { .. }
            | ObservableEvent::TrustRevoked { .. }
            | ObservableEvent::CertifiedIntentPassed { .. } => EventCategory::Authority,

            ObservableEvent::ViolationDetected { .. }
            | ObservableEvent::BypassAttempt { .. }
            | ObservableEvent::QuarantineEntered { .. }
            | ObservableEvent::QuarantineExited { .. }
            | ObservableEvent::ThreatDetected { .. } => EventCategory::Security,

            ObservableEvent::CorruptionDetected { .. }
            | ObservableEvent::DegradationTriggered { .. }
            | ObservableEvent::DegradationExited { .. }
            | ObservableEvent::SyncFailure { .. }
            | ObservableEvent::RecoveryCompleted { .. } => EventCategory::Failure,

            ObservableEvent::InstanceInitialized { .. }
            | ObservableEvent::InstanceStopped { .. }
            | ObservableEvent::StateChanged { .. } => EventCategory::Lifecycle,
        }
    }

    /// Retourne une description de l'événement
    pub fn description(&self) -> String {
        match self {
            ObservableEvent::IntentCreated { intent_id } => {
                format!("Intention {} créée", intent_id)
            }
            ObservableEvent::IntentValidated { intent_id } => {
                format!("Intention {} validée", intent_id)
            }
            ObservableEvent::IntentRejected { intent_id, reason } => {
                format!("Intention {} rejetée: {}", intent_id, reason)
            }
            ObservableEvent::IntentAccepted { intent_id } => {
                format!("Intention {} acceptée", intent_id)
            }
            ObservableEvent::WriteApplied { intent_id } => {
                format!("Écriture appliquée pour intention {}", intent_id)
            }
            ObservableEvent::PersistenceConfirmed { intent_id, domain_id } => {
                format!("Persistance confirmée pour intention {} dans domaine {}", intent_id, domain_id)
            }
            ObservableEvent::StateModified { intent_id, description } => {
                format!("État modifié par intention {}: {}", intent_id, description)
            }
            ObservableEvent::SyncTriggered { sync_id } => {
                format!("Synchronisation {} déclenchée", sync_id)
            }
            ObservableEvent::SyncSubmitted { sync_id } => {
                format!("Synchronisation {} soumise", sync_id)
            }
            ObservableEvent::SyncValidated { sync_id } => {
                format!("Synchronisation {} validée", sync_id)
            }
            ObservableEvent::ConflictDetected { sync_id, conflict_type } => {
                format!("Conflit {} détecté pour sync {}", conflict_type, sync_id)
            }
            ObservableEvent::SyncRejected { sync_id, reason } => {
                format!("Synchronisation {} rejetée: {}", sync_id, reason)
            }
            ObservableEvent::AuthorityDecision { decision_id, decision } => {
                format!("Décision d'autorité {}: {}", decision_id, decision)
            }
            ObservableEvent::TrustAttributed { target, level } => {
                format!("Confiance {} attribuée à {}", level, target)
            }
            ObservableEvent::TrustRevoked { target, reason } => {
                format!("Confiance révoquée pour {}: {}", target, reason)
            }
            ObservableEvent::CertifiedIntentPassed { intent_id, source_domain, target_domain } => {
                format!("Intention certifiée {} passée de {} à {}", intent_id, source_domain, target_domain)
            }
            ObservableEvent::ViolationDetected { violation_type, source } => {
                format!("Violation {} détectée de {}", violation_type, source)
            }
            ObservableEvent::BypassAttempt { source, description } => {
                format!("Tentative de contournement de {}: {}", source, description)
            }
            ObservableEvent::QuarantineEntered { entity_id, reason } => {
                format!("Entité {} mise en quarantaine: {}", entity_id, reason)
            }
            ObservableEvent::QuarantineExited { entity_id, reason } => {
                format!("Entité {} sortie de quarantaine: {}", entity_id, reason)
            }
            ObservableEvent::ThreatDetected { threat_id, threat_type, source } => {
                format!("Menace {} ({}) détectée de {}", threat_id, threat_type, source)
            }
            ObservableEvent::CorruptionDetected { description } => {
                format!("Corruption détectée: {}", description)
            }
            ObservableEvent::DegradationTriggered { reason } => {
                format!("Dégradation déclenchée: {}", reason)
            }
            ObservableEvent::DegradationExited { reason } => {
                format!("Sortie de dégradation: {}", reason)
            }
            ObservableEvent::SyncFailure { sync_id, reason } => {
                format!("Échec de synchronisation {}: {}", sync_id, reason)
            }
            ObservableEvent::RecoveryCompleted { description } => {
                format!("Récupération terminée: {}", description)
            }
            ObservableEvent::InstanceInitialized { instance_id } => {
                format!("Instance {} initialisée", instance_id)
            }
            ObservableEvent::InstanceStopped { instance_id } => {
                format!("Instance {} arrêtée", instance_id)
            }
            ObservableEvent::StateChanged { from, to, reason } => {
                format!("État changé de {:?} à {:?}: {}", from, to, reason)
            }
        }
    }
}

/// Contexte d'un événement
#[derive(Debug, Clone)]
pub struct EventContext {
    /// Identifiant de l'instance
    pub instance_id: String,
    /// Identifiant du domaine
    pub domain_id: String,
    /// Identité de l'acteur (si applicable)
    pub actor_identity: Option<String>,
    /// Contexte additionnel
    pub additional: Option<String>,
}

impl EventContext {
    /// Crée un nouveau contexte d'événement
    pub fn new(instance_id: String, domain_id: String) -> Self {
        Self {
            instance_id,
            domain_id,
            actor_identity: None,
            additional: None,
        }
    }

    /// Ajoute l'identité de l'acteur
    pub fn with_actor(mut self, actor_identity: String) -> Self {
        self.actor_identity = Some(actor_identity);
        self
    }

    /// Ajoute un contexte additionnel
    pub fn with_additional(mut self, additional: String) -> Self {
        self.additional = Some(additional);
        self
    }
}

/// Événement enregistré
///
/// Structure immuable représentant un événement observé.
/// Conforme au contrat FONDATION section 3.2.
#[derive(Debug, Clone)]
pub struct Event {
    /// Identifiant unique de l'événement
    event_id: String,
    /// Type d'événement
    event_type: ObservableEvent,
    /// Moment de l'événement (conceptuel, compteur)
    moment: u64,
    /// Contexte de l'événement
    context: EventContext,
    /// Résultat de l'événement (si applicable)
    result: Option<String>,
}

impl Event {
    /// Crée un nouvel événement
    pub fn new(
        event_id: String,
        event_type: ObservableEvent,
        moment: u64,
        context: EventContext,
        result: Option<String>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            moment,
            context,
            result,
        }
    }

    /// Obtient l'identifiant de l'événement
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Obtient le type d'événement
    pub fn event_type(&self) -> &ObservableEvent {
        &self.event_type
    }

    /// Obtient la catégorie de l'événement
    pub fn category(&self) -> EventCategory {
        self.event_type.category()
    }

    /// Obtient le moment de l'événement
    pub fn moment(&self) -> u64 {
        self.moment
    }

    /// Obtient le contexte de l'événement
    pub fn context(&self) -> &EventContext {
        &self.context
    }

    /// Obtient le résultat de l'événement
    pub fn result(&self) -> Option<&str> {
        self.result.as_deref()
    }
}

/// Entrée du journal d'intention
///
/// Conforme au contrat FONDATION section 4.
#[derive(Debug, Clone)]
pub struct IntentJournalEntry {
    /// Identifiant de l'intention
    intent_id: String,
    /// Moment de création
    created_at: u64,
    /// Origine (instance, adaptateur)
    origin: String,
    /// Type d'opération
    operation_type: String,
    /// Contexte d'appel
    context: EventContext,
    /// États traversés
    states_traversed: Vec<WriteIntentState>,
    /// Résultat final
    final_result: Option<String>,
    /// Raison du rejet (si applicable)
    rejection_reason: Option<String>,
}

impl IntentJournalEntry {
    /// Crée une nouvelle entrée de journal
    pub fn new(
        intent_id: String,
        created_at: u64,
        origin: String,
        operation_type: String,
        context: EventContext,
    ) -> Self {
        Self {
            intent_id,
            created_at,
            origin,
            operation_type,
            context,
            states_traversed: vec![WriteIntentState::Created],
            final_result: None,
            rejection_reason: None,
        }
    }

    /// Obtient l'identifiant de l'intention
    pub fn intent_id(&self) -> &str {
        &self.intent_id
    }

    /// Obtient le moment de création
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Obtient l'origine
    pub fn origin(&self) -> &str {
        &self.origin
    }

    /// Obtient le type d'opération
    pub fn operation_type(&self) -> &str {
        &self.operation_type
    }

    /// Obtient les états traversés
    pub fn states_traversed(&self) -> &[WriteIntentState] {
        &self.states_traversed
    }

    /// Obtient le résultat final
    pub fn final_result(&self) -> Option<&str> {
        self.final_result.as_deref()
    }

    /// Obtient la raison du rejet
    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }

    /// Obtient le contexte
    pub fn context(&self) -> &EventContext {
        &self.context
    }
}

/// Journal d'intention immuable
///
/// Enregistre toutes les intentions et leur cycle de vie.
/// Conforme au contrat FONDATION section 4.
pub struct IntentJournal {
    /// Entrées du journal (append-only)
    entries: Vec<IntentJournalEntry>,
    /// Index par intent_id pour recherche rapide
    index: std::collections::HashMap<String, usize>,
}

impl IntentJournal {
    /// Crée un nouveau journal
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: std::collections::HashMap::new(),
        }
    }

    /// Enregistre une nouvelle intention
    pub fn record_intent(
        &mut self,
        intent_id: String,
        created_at: u64,
        origin: String,
        operation_type: String,
        context: EventContext,
    ) {
        let entry = IntentJournalEntry::new(
            intent_id.clone(),
            created_at,
            origin,
            operation_type,
            context,
        );
        let idx = self.entries.len();
        self.entries.push(entry);
        self.index.insert(intent_id, idx);

        println!("[IntentJournal] Intention enregistrée: index={}", idx);
    }

    /// Enregistre une transition d'état
    pub fn record_transition(&mut self, intent_id: &str, new_state: WriteIntentState) {
        if let Some(&idx) = self.index.get(intent_id) {
            if let Some(entry) = self.entries.get_mut(idx) {
                entry.states_traversed.push(new_state);
                println!(
                    "[IntentJournal] Transition enregistrée pour {}: {:?}",
                    intent_id, new_state
                );
            }
        }
    }

    /// Enregistre le résultat final
    pub fn record_result(&mut self, intent_id: &str, result: String, rejection_reason: Option<String>) {
        if let Some(&idx) = self.index.get(intent_id) {
            if let Some(entry) = self.entries.get_mut(idx) {
                entry.final_result = Some(result);
                entry.rejection_reason = rejection_reason;
                println!(
                    "[IntentJournal] Résultat enregistré pour {}: {:?}",
                    intent_id, entry.final_result
                );
            }
        }
    }

    /// Obtient une entrée par intent_id
    pub fn get_entry(&self, intent_id: &str) -> Option<&IntentJournalEntry> {
        self.index.get(intent_id).and_then(|&idx| self.entries.get(idx))
    }

    /// Obtient toutes les entrées
    pub fn entries(&self) -> &[IntentJournalEntry] {
        &self.entries
    }

    /// Obtient le nombre d'entrées
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for IntentJournal {
    fn default() -> Self {
        Self::new()
    }
}

/// Entrée du journal des rejets
///
/// Conforme au contrat FONDATION section 6.
#[derive(Debug, Clone)]
pub struct RejectionEntry {
    /// Identifiant du rejet
    rejection_id: String,
    /// Moment du rejet
    moment: u64,
    /// Type de rejet
    rejection_type: String,
    /// Raison détaillée
    reason: String,
    /// Contexte
    context: EventContext,
    /// Boundary ayant provoqué le rejet
    boundary: String,
    /// État du système au moment du rejet
    system_state: KMState,
    /// Identifiant de l'intention (si applicable)
    intent_id: Option<String>,
}

impl RejectionEntry {
    /// Crée une nouvelle entrée de rejet
    pub fn new(
        rejection_id: String,
        moment: u64,
        rejection_type: String,
        reason: String,
        context: EventContext,
        boundary: String,
        system_state: KMState,
        intent_id: Option<String>,
    ) -> Self {
        Self {
            rejection_id,
            moment,
            rejection_type,
            reason,
            context,
            boundary,
            system_state,
            intent_id,
        }
    }

    /// Obtient l'identifiant du rejet
    pub fn rejection_id(&self) -> &str {
        &self.rejection_id
    }

    /// Obtient le moment du rejet
    pub fn moment(&self) -> u64 {
        self.moment
    }

    /// Obtient le type de rejet
    pub fn rejection_type(&self) -> &str {
        &self.rejection_type
    }

    /// Obtient la raison
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Obtient la boundary
    pub fn boundary(&self) -> &str {
        &self.boundary
    }

    /// Obtient l'état du système
    pub fn system_state(&self) -> KMState {
        self.system_state
    }

    /// Obtient l'identifiant de l'intention
    pub fn intent_id(&self) -> Option<&str> {
        self.intent_id.as_deref()
    }

    /// Obtient le contexte
    pub fn context(&self) -> &EventContext {
        &self.context
    }
}

/// Journal des rejets
///
/// Conforme au contrat FONDATION section 6.
pub struct RejectionLog {
    /// Entrées du journal (append-only)
    entries: Vec<RejectionEntry>,
    /// Compteur pour générer des IDs
    counter: u64,
}

impl RejectionLog {
    /// Crée un nouveau journal des rejets
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            counter: 0,
        }
    }

    /// Enregistre un rejet
    pub fn record_rejection(
        &mut self,
        moment: u64,
        rejection_type: String,
        reason: String,
        context: EventContext,
        boundary: String,
        system_state: KMState,
        intent_id: Option<String>,
    ) -> String {
        self.counter += 1;
        let rejection_id = format!("rej-{}", self.counter);

        let entry = RejectionEntry::new(
            rejection_id.clone(),
            moment,
            rejection_type,
            reason,
            context,
            boundary,
            system_state,
            intent_id,
        );

        println!(
            "[RejectionLog] Rejet {} enregistré: {}",
            rejection_id,
            entry.reason()
        );

        self.entries.push(entry);
        rejection_id
    }

    /// Obtient toutes les entrées
    pub fn entries(&self) -> &[RejectionEntry] {
        &self.entries
    }

    /// Obtient le nombre de rejets
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for RejectionLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Entrée du journal des quarantaines
///
/// Conforme au contrat FONDATION section 7.
#[derive(Debug, Clone)]
pub struct QuarantineEntry {
    /// Identifiant de la quarantaine
    quarantine_id: String,
    /// Identifiant de l'entité quarantainée
    entity_id: String,
    /// Moment d'entrée en quarantaine
    entered_at: u64,
    /// Raison de la quarantaine
    reason: String,
    /// Violation ayant déclenché la quarantaine
    violation: String,
    /// Niveau de quarantaine
    level: String,
    /// Conditions de sortie
    exit_conditions: String,
    /// Moment de sortie (si applicable)
    exited_at: Option<u64>,
    /// Raison de sortie (si applicable)
    exit_reason: Option<String>,
}

impl QuarantineEntry {
    /// Crée une nouvelle entrée de quarantaine
    pub fn new(
        quarantine_id: String,
        entity_id: String,
        entered_at: u64,
        reason: String,
        violation: String,
        level: String,
        exit_conditions: String,
    ) -> Self {
        Self {
            quarantine_id,
            entity_id,
            entered_at,
            reason,
            violation,
            level,
            exit_conditions,
            exited_at: None,
            exit_reason: None,
        }
    }

    /// Obtient l'identifiant de la quarantaine
    pub fn quarantine_id(&self) -> &str {
        &self.quarantine_id
    }

    /// Obtient l'identifiant de l'entité
    pub fn entity_id(&self) -> &str {
        &self.entity_id
    }

    /// Obtient le moment d'entrée
    pub fn entered_at(&self) -> u64 {
        self.entered_at
    }

    /// Obtient la raison
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Obtient la violation
    pub fn violation(&self) -> &str {
        &self.violation
    }

    /// Obtient le niveau de quarantaine
    pub fn level(&self) -> &str {
        &self.level
    }

    /// Obtient les conditions de sortie
    pub fn exit_conditions(&self) -> &str {
        &self.exit_conditions
    }

    /// Vérifie si la quarantaine est active
    pub fn is_active(&self) -> bool {
        self.exited_at.is_none()
    }

    /// Obtient le moment de sortie
    pub fn exited_at(&self) -> Option<u64> {
        self.exited_at
    }

    /// Obtient la raison de sortie
    pub fn exit_reason(&self) -> Option<&str> {
        self.exit_reason.as_deref()
    }
}

/// Journal des quarantaines
///
/// Conforme au contrat FONDATION section 7.
pub struct QuarantineLog {
    /// Entrées du journal (append-only)
    entries: Vec<QuarantineEntry>,
    /// Index par entité pour les quarantaines actives
    active_quarantines: std::collections::HashMap<String, usize>,
    /// Compteur pour générer des IDs
    counter: u64,
}

impl QuarantineLog {
    /// Crée un nouveau journal des quarantaines
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            active_quarantines: std::collections::HashMap::new(),
            counter: 0,
        }
    }

    /// Enregistre l'entrée en quarantaine
    pub fn record_quarantine_entry(
        &mut self,
        entity_id: String,
        entered_at: u64,
        reason: String,
        violation: String,
        level: String,
        exit_conditions: String,
    ) -> String {
        self.counter += 1;
        let quarantine_id = format!("quar-{}", self.counter);

        let entry = QuarantineEntry::new(
            quarantine_id.clone(),
            entity_id.clone(),
            entered_at,
            reason,
            violation,
            level,
            exit_conditions,
        );

        println!(
            "[QuarantineLog] Entrée en quarantaine {} pour entité {}",
            quarantine_id, entity_id
        );

        let idx = self.entries.len();
        self.entries.push(entry);
        self.active_quarantines.insert(entity_id, idx);

        quarantine_id
    }

    /// Enregistre la sortie de quarantaine
    pub fn record_quarantine_exit(
        &mut self,
        entity_id: &str,
        exited_at: u64,
        exit_reason: String,
    ) -> bool {
        if let Some(&idx) = self.active_quarantines.get(entity_id) {
            if let Some(entry) = self.entries.get_mut(idx) {
                entry.exited_at = Some(exited_at);
                entry.exit_reason = Some(exit_reason);
                self.active_quarantines.remove(entity_id);

                println!(
                    "[QuarantineLog] Sortie de quarantaine {} pour entité {}",
                    entry.quarantine_id(), entity_id
                );

                return true;
            }
        }
        false
    }

    /// Vérifie si une entité est en quarantaine
    pub fn is_quarantined(&self, entity_id: &str) -> bool {
        self.active_quarantines.contains_key(entity_id)
    }

    /// Obtient la quarantaine active d'une entité
    pub fn get_active_quarantine(&self, entity_id: &str) -> Option<&QuarantineEntry> {
        self.active_quarantines
            .get(entity_id)
            .and_then(|&idx| self.entries.get(idx))
    }

    /// Obtient toutes les entrées
    pub fn entries(&self) -> &[QuarantineEntry] {
        &self.entries
    }

    /// Obtient le nombre de quarantaines (total)
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Obtient le nombre de quarantaines actives
    pub fn active_count(&self) -> usize {
        self.active_quarantines.len()
    }
}

impl Default for QuarantineLog {
    fn default() -> Self {
        Self::new()
    }
}

/// Système d'observabilité central
///
/// Regroupe tous les journaux et gère les événements.
/// Conforme au contrat FONDATION "Observability & Audit Contract".
pub struct Observability {
    /// Tous les événements enregistrés
    events: Vec<Event>,
    /// Journal d'intention
    intent_journal: IntentJournal,
    /// Journal des rejets
    rejection_log: RejectionLog,
    /// Journal des quarantaines
    quarantine_log: QuarantineLog,
    /// Compteur pour générer des IDs d'événements
    event_counter: u64,
    /// Compteur pour le moment (horloge conceptuelle)
    moment_counter: u64,
    /// Contexte par défaut (instance, domaine)
    default_context: EventContext,
}

impl Observability {
    /// Crée un nouveau système d'observabilité
    pub fn new(instance_id: String, domain_id: String) -> Self {
        println!(
            "[Observability] Création du système d'observabilité pour instance {} dans domaine {}",
            instance_id, domain_id
        );
        Self {
            events: Vec::new(),
            intent_journal: IntentJournal::new(),
            rejection_log: RejectionLog::new(),
            quarantine_log: QuarantineLog::new(),
            event_counter: 0,
            moment_counter: 0,
            default_context: EventContext::new(instance_id, domain_id),
        }
    }

    /// Génère un identifiant unique pour un événement
    fn generate_event_id(&mut self) -> String {
        self.event_counter += 1;
        format!("evt-{}", self.event_counter)
    }

    /// Obtient le moment actuel
    fn get_moment(&mut self) -> u64 {
        self.moment_counter += 1;
        self.moment_counter
    }

    /// Enregistre un événement
    pub fn record_event(&mut self, event_type: ObservableEvent) -> String {
        self.record_event_with_context(event_type, self.default_context.clone(), None)
    }

    /// Enregistre un événement avec contexte personnalisé
    pub fn record_event_with_context(
        &mut self,
        event_type: ObservableEvent,
        context: EventContext,
        result: Option<String>,
    ) -> String {
        let event_id = self.generate_event_id();
        let moment = self.get_moment();

        println!(
            "[Observability] Événement {}: {}",
            event_id,
            event_type.description()
        );

        let event = Event::new(event_id.clone(), event_type, moment, context, result);
        self.events.push(event);

        event_id
    }

    /// Accède au journal d'intention
    pub fn intent_journal(&self) -> &IntentJournal {
        &self.intent_journal
    }

    /// Accède au journal d'intention (mutable)
    pub fn intent_journal_mut(&mut self) -> &mut IntentJournal {
        &mut self.intent_journal
    }

    /// Accède au journal des rejets
    pub fn rejection_log(&self) -> &RejectionLog {
        &self.rejection_log
    }

    /// Accède au journal des rejets (mutable)
    pub fn rejection_log_mut(&mut self) -> &mut RejectionLog {
        &mut self.rejection_log
    }

    /// Accède au journal des quarantaines
    pub fn quarantine_log(&self) -> &QuarantineLog {
        &self.quarantine_log
    }

    /// Accède au journal des quarantaines (mutable)
    pub fn quarantine_log_mut(&mut self) -> &mut QuarantineLog {
        &mut self.quarantine_log
    }

    /// Enregistre un rejet (raccourci)
    pub fn record_rejection(
        &mut self,
        rejection_type: String,
        reason: String,
        boundary: String,
        system_state: KMState,
        intent_id: Option<String>,
    ) -> String {
        let moment = self.get_moment();
        self.rejection_log.record_rejection(
            moment,
            rejection_type,
            reason,
            self.default_context.clone(),
            boundary,
            system_state,
            intent_id,
        )
    }

    /// Enregistre une entrée en quarantaine (raccourci)
    pub fn record_quarantine_entry(
        &mut self,
        entity_id: String,
        reason: String,
        violation: String,
        level: String,
        exit_conditions: String,
    ) -> String {
        let entered_at = self.get_moment();
        self.quarantine_log.record_quarantine_entry(
            entity_id,
            entered_at,
            reason,
            violation,
            level,
            exit_conditions,
        )
    }

    /// Enregistre une sortie de quarantaine (raccourci)
    pub fn record_quarantine_exit(&mut self, entity_id: &str, exit_reason: String) -> bool {
        let exited_at = self.get_moment();
        self.quarantine_log.record_quarantine_exit(entity_id, exited_at, exit_reason)
    }

    /// Obtient tous les événements
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Obtient les événements d'une catégorie
    pub fn events_by_category(&self, category: EventCategory) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| e.category() == category)
            .collect()
    }

    /// Obtient le nombre total d'événements
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Obtient le contexte par défaut
    pub fn default_context(&self) -> &EventContext {
        &self.default_context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let mut obs = Observability::new("instance-1".to_string(), "domain-1".to_string());
        
        let event_id = obs.record_event(ObservableEvent::IntentCreated {
            intent_id: "intent-1".to_string(),
        });
        
        assert!(event_id.starts_with("evt-"));
        assert_eq!(obs.event_count(), 1);
    }

    #[test]
    fn test_intent_journal() {
        let mut journal = IntentJournal::new();
        let context = EventContext::new("instance-1".to_string(), "domain-1".to_string());
        
        journal.record_intent(
            "intent-1".to_string(),
            1,
            "user-1".to_string(),
            "create".to_string(),
            context,
        );
        
        assert_eq!(journal.count(), 1);
        
        let entry = journal.get_entry("intent-1").unwrap();
        assert_eq!(entry.intent_id(), "intent-1");
        assert_eq!(entry.states_traversed().len(), 1);
        
        journal.record_transition("intent-1", WriteIntentState::InValidation);
        let entry = journal.get_entry("intent-1").unwrap();
        assert_eq!(entry.states_traversed().len(), 2);
    }

    #[test]
    fn test_rejection_log() {
        let mut log = RejectionLog::new();
        let context = EventContext::new("instance-1".to_string(), "domain-1".to_string());
        
        let rejection_id = log.record_rejection(
            1,
            "permission".to_string(),
            "Permission insuffisante".to_string(),
            context,
            "permissions_boundary".to_string(),
            KMState::Healthy,
            Some("intent-1".to_string()),
        );
        
        assert!(rejection_id.starts_with("rej-"));
        assert_eq!(log.count(), 1);
    }

    #[test]
    fn test_quarantine_log() {
        let mut log = QuarantineLog::new();
        
        let quarantine_id = log.record_quarantine_entry(
            "user-1".to_string(),
            1,
            "Comportement suspect".to_string(),
            "brute_force".to_string(),
            "full".to_string(),
            "Vérification manuelle".to_string(),
        );
        
        assert!(quarantine_id.starts_with("quar-"));
        assert!(log.is_quarantined("user-1"));
        assert_eq!(log.active_count(), 1);
        
        log.record_quarantine_exit("user-1", 2, "Vérification OK".to_string());
        assert!(!log.is_quarantined("user-1"));
        assert_eq!(log.active_count(), 0);
    }

    #[test]
    fn test_event_categories() {
        assert_eq!(
            ObservableEvent::IntentCreated { intent_id: "x".to_string() }.category(),
            EventCategory::Intent
        );
        assert_eq!(
            ObservableEvent::WriteApplied { intent_id: "x".to_string() }.category(),
            EventCategory::Write
        );
        assert_eq!(
            ObservableEvent::SyncTriggered { sync_id: "x".to_string() }.category(),
            EventCategory::Sync
        );
    }
}
