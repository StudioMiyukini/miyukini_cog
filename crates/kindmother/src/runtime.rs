//! Runtime Boundary de KindMother
//!
//! Implémente les frontières runtime qui valident toutes les opérations.
//! Conforme au contrat FONDATION "KindMother — Runtime Boundary & Enforcement Contract".

use crate::core::{AuthorityContext, DomainContext, InstanceContext, InstanceType, WriteIntent};
use crate::errors::KMError;
use crate::lifecycle::WriteIntentState;
use crate::state::KMState;
use crate::sync::{SyncIntent, SyncIntentState};
use crate::threat::{DetectedThreat, ThreatDetector};
use std::collections::HashSet;

/// Résultat d'une validation de Runtime Boundary
pub type BoundaryResult = Result<(), KMError>;

/// Runtime Boundary - Point d'entrée pour toutes les validations
///
/// Implémente les boundaries structurelles selon le contrat FONDATION.
pub struct RuntimeBoundary {
    /// Ensemble des identités d'intentions déjà vues (pour éviter les resoumissions)
    seen_intents: HashSet<String>,
    /// Détecteur de menaces intégré
    threat_detector: ThreatDetector,
}

impl RuntimeBoundary {
    /// Crée une nouvelle instance de RuntimeBoundary
    pub fn new() -> Self {
        Self {
            seen_intents: HashSet::new(),
            threat_detector: ThreatDetector::new(),
        }
    }

    /// Vérifie si une intention a déjà été vue
    pub fn has_seen_intent(&self, intent_id: &str) -> bool {
        self.seen_intents.contains(intent_id)
    }

    /// Marque une intention comme vue
    pub fn mark_intent_seen(&mut self, intent_id: String) {
        self.seen_intents.insert(intent_id);
    }

    /// Accède au détecteur de menaces
    pub fn threat_detector(&mut self) -> &mut ThreatDetector {
        &mut self.threat_detector
    }

    /// Obtient les menaces détectées
    pub fn detected_threats(&self) -> &[DetectedThreat] {
        self.threat_detector.detected_threats()
    }
}

impl Default for RuntimeBoundary {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeBoundary {
    /// Vérifie la Boundary d'état KM
    ///
    /// Vérifie que l'état du moteur permet l'opération demandée.
    /// - Healthy → autorisé
    /// - Degraded → lecture OK, écriture refusée
    /// - Quarantined → tout refusé
    pub fn check_km_state_boundary(
        current_state: KMState,
        operation: &str,
        is_write: bool,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification Boundary d'état KM: état={:?}, opération={}, écriture={}",
            current_state, operation, is_write
        );

        // Quarantined → tout refusé
        if current_state.is_quarantined() {
            println!("[RuntimeBoundary] Boundary d'état KM rejetée: moteur en quarantaine");
            return Err(KMError::InvalidState {
                current_state: current_state.to_string(),
                reason: "Le moteur est en quarantaine. Toutes les opérations sont bloquées.".to_string(),
            });
        }

        // Booting → refusé (pas encore opérationnel)
        if current_state == KMState::Booting {
            println!("[RuntimeBoundary] Boundary d'état KM rejetée: moteur en cours de démarrage");
            return Err(KMError::InvalidState {
                current_state: current_state.to_string(),
                reason: "Le moteur est en cours de démarrage. Aucune opération n'est autorisée.".to_string(),
            });
        }

        // Degraded → lecture OK, écriture refusée
        if current_state == KMState::Degraded {
            if is_write {
                println!("[RuntimeBoundary] Boundary d'état KM rejetée: moteur dégradé, écriture refusée");
                return Err(KMError::InvalidState {
                    current_state: current_state.to_string(),
                    reason: "Le moteur est en état dégradé. Les écritures sont refusées, seules les lectures sont autorisées.".to_string(),
                });
            } else {
                // Lecture autorisée en état Degraded
                println!("[RuntimeBoundary] Boundary d'état KM validée: moteur dégradé, lecture autorisée");
                return Ok(());
            }
        }

        // Healthy → autorisé (lecture et écriture)
        if current_state == KMState::Healthy {
            println!("[RuntimeBoundary] Boundary d'état KM validée: moteur sain");
            return Ok(());
        }

        // Cas par défaut (ne devrait jamais arriver)
        println!("[RuntimeBoundary] Boundary d'état KM rejetée: état inconnu");
        Err(KMError::InvalidState {
            current_state: current_state.to_string(),
            reason: "État du moteur inconnu ou invalide.".to_string(),
        })
    }

    /// Vérifie la Boundary d'appel
    ///
    /// Vérifie que l'appel CoreDataAPI est légal et bien formé.
    /// À ce stade, on vérifie uniquement que l'opération n'est pas vide.
    pub fn check_call_boundary(operation: &str) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary d'appel: {}", operation);

        if operation.is_empty() {
            println!("[RuntimeBoundary] Boundary d'appel rejetée: opération vide");
            return Err(KMError::IllegalOperation {
                reason: "L'opération ne peut pas être vide.".to_string(),
            });
        }

        println!("[RuntimeBoundary] Boundary d'appel validée");
        Ok(())
    }

    /// Vérifie la Boundary de contexte
    ///
    /// Vérifie que le contexte est complet et cohérent structurellement.
    /// - InstanceContext requis (présence + cohérence structurelle)
    /// - DomainContext requis (présence + cohérence structurelle)
    /// - AuthorityContext requis (présence + cohérence structurelle)
    /// Validation = présence + cohérence structurelle uniquement (pas de logique métier).
    pub fn check_context_boundary(
        authority: &AuthorityContext,
        instance: &InstanceContext,
        domain: &DomainContext,
    ) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary de contexte");

        // Vérification InstanceContext : présence + cohérence structurelle
        if instance.instance_id.is_empty() {
            println!("[RuntimeBoundary] Boundary de contexte rejetée: InstanceContext.instance_id vide");
            return Err(KMError::InvalidContext {
                reason: "InstanceContext requis: instance_id ne peut pas être vide.".to_string(),
            });
        }

        // Vérification DomainContext : présence + cohérence structurelle
        if domain.domain_id.is_empty() {
            println!("[RuntimeBoundary] Boundary de contexte rejetée: DomainContext.domain_id vide");
            return Err(KMError::InvalidContext {
                reason: "DomainContext requis: domain_id ne peut pas être vide.".to_string(),
            });
        }

        if domain.domain_name.is_empty() {
            println!("[RuntimeBoundary] Boundary de contexte rejetée: DomainContext.domain_name vide");
            return Err(KMError::InvalidContext {
                reason: "DomainContext requis: domain_name ne peut pas être vide.".to_string(),
            });
        }

        // Vérification AuthorityContext : présence + cohérence structurelle
        if authority.caller_identity.is_empty() {
            println!("[RuntimeBoundary] Boundary de contexte rejetée: AuthorityContext.caller_identity vide");
            return Err(KMError::InvalidContext {
                reason: "AuthorityContext requis: caller_identity ne peut pas être vide.".to_string(),
            });
        }

        // Cohérence structurelle : vérification que les identifiants ne sont pas identiques
        // (simple vérification de cohérence, pas de logique métier)
        if instance.instance_id == domain.domain_id {
            println!("[RuntimeBoundary] Boundary de contexte rejetée: instance_id et domain_id identiques (incohérence structurelle)");
            return Err(KMError::InvalidContext {
                reason: "Cohérence structurelle: instance_id et domain_id ne peuvent pas être identiques.".to_string(),
            });
        }

        println!("[RuntimeBoundary] Boundary de contexte validée");
        Ok(())
    }

    /// Vérifie la Boundary d'instance
    ///
    /// Vérifie que l'instance est valide structurellement.
    /// À ce stade, on vérifie uniquement la cohérence structurelle.
    pub fn check_instance_boundary(instance: &InstanceContext) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary d'instance");

        // Vérification structurelle : instance_id non vide (déjà vérifié dans check_context_boundary)
        // Ici on peut ajouter d'autres vérifications structurelles si nécessaire
        if instance.instance_id.is_empty() {
            println!("[RuntimeBoundary] Boundary d'instance rejetée: instance_id vide");
            return Err(KMError::InvalidInstance {
                reason: "InstanceContext requis: instance_id ne peut pas être vide.".to_string(),
            });
        }

        println!("[RuntimeBoundary] Boundary d'instance validée");
        Ok(())
    }

    /// Vérifie la Boundary de permissions
    ///
    /// Vérifie que les permissions conceptuelles sont suffisantes.
    /// - read → can_read requis
    /// - write → can_write requis
    /// - sync → TOUJOURS refusé
    /// Aucune escalade implicite : read ≠ write, write ≠ sync
    pub fn check_permissions_boundary(
        authority: &AuthorityContext,
        operation: &str,
        is_write: bool,
    ) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary de permissions: {} (écriture: {})", operation, is_write);

        // Détection du type d'opération (vérifie le nom de l'opération CoreDataAPI)
        let is_sync = operation.contains("sync") || operation.contains("synchronize");

        // Sync → TOUJOURS refusé
        if is_sync {
            println!("[RuntimeBoundary] Boundary de permissions rejetée: opération de synchronisation toujours refusée");
            return Err(KMError::InsufficientPermissions {
                reason: "Les opérations de synchronisation sont toujours refusées à ce stade.".to_string(),
            });
        }

        // Write → can_write requis
        if is_write {
            if !authority.can_write {
                println!("[RuntimeBoundary] Boundary de permissions rejetée: can_write requis pour les écritures");
                return Err(KMError::InsufficientPermissions {
                    reason: "Permission can_write requise pour les opérations d'écriture. Aucune escalade implicite depuis can_read.".to_string(),
                });
            }
            println!("[RuntimeBoundary] Boundary de permissions validée: can_write présent");
            return Ok(());
        }

        // Read → can_read requis
        if !authority.can_read {
            println!("[RuntimeBoundary] Boundary de permissions rejetée: can_read requis pour les lectures");
            return Err(KMError::InsufficientPermissions {
                reason: "Permission can_read requise pour les opérations de lecture.".to_string(),
            });
        }

        println!("[RuntimeBoundary] Boundary de permissions validée: can_read présent");
        Ok(())
    }

    /// Vérifie la Boundary de permissions pour une WriteIntent
    ///
    /// Vérifie que les permissions conceptuelles sont suffisantes pour une WriteIntent.
    /// Vérifie aussi le type d'opération dans le WriteIntent pour détecter les sync.
    pub fn check_permissions_boundary_with_intent(
        authority: &AuthorityContext,
        operation: &str,
        intent: &WriteIntent,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification Boundary de permissions avec WriteIntent: {} (intent: {}, op_type: {})",
            operation,
            intent.intent_id(),
            intent.operation_type()
        );

        // Détection du type d'opération : vérifie le nom de l'opération ET le type d'opération du WriteIntent
        let is_sync = operation.contains("sync") 
            || operation.contains("synchronize")
            || intent.operation_type().contains("sync")
            || intent.operation_type().contains("synchronize");

        // Sync → TOUJOURS refusé
        if is_sync {
            println!("[RuntimeBoundary] Boundary de permissions rejetée: opération de synchronisation toujours refusée");
            return Err(KMError::InsufficientPermissions {
                reason: "Les opérations de synchronisation sont toujours refusées à ce stade.".to_string(),
            });
        }

        // WriteIntent → can_write requis
        if !authority.can_write {
            println!("[RuntimeBoundary] Boundary de permissions rejetée: can_write requis pour les WriteIntent");
            return Err(KMError::InsufficientPermissions {
                reason: "Permission can_write requise pour les opérations d'écriture. Aucune escalade implicite depuis can_read.".to_string(),
            });
        }

        println!("[RuntimeBoundary] Boundary de permissions validée: can_write présent");
        Ok(())
    }

    /// Vérifie la Boundary de cohérence
    ///
    /// Vérifie que l'opération préserve la cohérence.
    /// À ce stade, cette boundary n'est pas implémentée (pas de logique métier).
    /// Elle sera implémentée dans une étape ultérieure.
    pub fn check_consistency_boundary(_operation: &str) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary de cohérence: {}", _operation);
        // À ce stade, on accepte toutes les opérations (pas de logique métier)
        // Cette boundary sera implémentée dans une étape ultérieure
        println!("[RuntimeBoundary] Boundary de cohérence validée (pas de logique métier à ce stade)");
        Ok(())
    }

    /// Vérifie la Boundary de contournement
    ///
    /// Vérifie qu'aucune tentative de contournement n'est détectée.
    /// Cette méthode est statique et ne fait pas de détection active.
    /// Pour la détection active, utiliser check_bypass_boundary_with_detection.
    pub fn check_bypass_boundary(_operation: &str) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary de contournement: {}", _operation);
        // Vérification structurelle : l'opération doit être valide
        // La détection active est faite via check_bypass_boundary_with_detection
        println!("[RuntimeBoundary] Boundary de contournement validée (structurel)");
        Ok(())
    }

    /// Vérifie la Boundary de contournement avec détection de menaces
    ///
    /// Utilise le ThreatDetector pour détecter les tentatives de contournement.
    /// Conforme au contrat FONDATION section V6.
    pub fn check_bypass_boundary_with_detection(
        &mut self,
        operation: &str,
        source_identity: &str,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification Boundary de contournement avec détection: {} (source: {})",
            operation, source_identity
        );

        // Détection de patterns suspects dans l'opération
        // Ces patterns pourraient indiquer une tentative de contournement
        let suspicious_patterns = ["direct_storage", "bypass", "internal_", "raw_"];
        
        for pattern in suspicious_patterns {
            if operation.to_lowercase().contains(pattern) {
                let threat = self.threat_detector.detect_bypass_attempt(
                    source_identity,
                    &format!("Pattern suspect '{}' détecté dans l'opération '{}'", pattern, operation),
                );
                
                println!(
                    "[RuntimeBoundary] Boundary de contournement rejetée: menace {} détectée",
                    threat.threat_id()
                );
                
                return Err(ThreatDetector::threat_to_error(&threat));
            }
        }

        println!("[RuntimeBoundary] Boundary de contournement validée");
        Ok(())
    }

    /// Vérifie la Boundary de charge
    ///
    /// Vérifie que la charge est raisonnable.
    /// Cette méthode est statique et ne fait pas de détection active.
    /// Pour la détection active, utiliser check_load_boundary_with_detection.
    pub fn check_load_boundary(_operation: &str) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification Boundary de charge: {}", _operation);
        // Vérification structurelle uniquement
        // La détection active est faite via check_load_boundary_with_detection
        println!("[RuntimeBoundary] Boundary de charge validée (structurel)");
        Ok(())
    }

    /// Vérifie la Boundary de charge avec détection de saturation
    ///
    /// Utilise le ThreatDetector pour détecter la saturation volontaire.
    /// Conforme au contrat FONDATION section V7.
    pub fn check_load_boundary_with_detection(
        &mut self,
        _operation: &str,
        source_identity: &str,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification Boundary de charge avec détection: {} (source: {})",
            _operation, source_identity
        );

        // Détection de saturation
        if let Some(threat) = self.threat_detector.detect_saturation(source_identity) {
            println!(
                "[RuntimeBoundary] Boundary de charge rejetée: menace {} détectée",
                threat.threat_id()
            );
            return Err(ThreatDetector::threat_to_error(&threat));
        }

        println!("[RuntimeBoundary] Boundary de charge validée");
        Ok(())
    }

    /// Vérifie les menaces pour une WriteIntent
    ///
    /// Utilise le ThreatDetector pour vérifier replay, resoumission, injection.
    pub fn check_threats_for_intent(
        &mut self,
        intent: &WriteIntent,
        source_identity: &str,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification des menaces pour intention {} (source: {})",
            intent.intent_id(), source_identity
        );

        // Vérifier toutes les menaces
        if let Some(threat) = self.threat_detector.check_all_threats(
            intent.intent_id(),
            intent.operation_type(),
            source_identity,
        ) {
            println!(
                "[RuntimeBoundary] Menace {} détectée pour intention {}",
                threat.threat_type().to_string(),
                intent.intent_id()
            );
            return Err(ThreatDetector::threat_to_error(&threat));
        }

        // Marquer l'intention comme traitée pour la détection de replay future
        self.threat_detector.mark_intent_processed(intent.intent_id());

        println!("[RuntimeBoundary] Aucune menace détectée pour intention {}", intent.intent_id());
        Ok(())
    }

    /// Enregistre un rejet pour détection de brute-force
    pub fn record_rejection(&mut self, source_identity: &str) -> Option<DetectedThreat> {
        self.threat_detector.record_rejection_and_detect_brute_force(source_identity)
    }

    /// Réinitialise le compteur de rejets après succès
    pub fn reset_rejection_count(&mut self, source_identity: &str) {
        self.threat_detector.reset_rejection_count(source_identity);
    }

    /// Vérifie la Boundary de synchronisation
    ///
    /// Vérifie que la synchronisation est structurellement valide :
    /// - L'instance source DOIT être une Instance Fille (pas Mère)
    /// - Le contexte de sync doit être cohérent
    /// - À ce stade, TOUTE synchronisation est REFUSÉE explicitement
    ///
    /// Conforme au contrat FONDATION "Sync & Conflict Resolution Contract" section 4.
    pub fn check_sync_boundary(
        sync_intent: &SyncIntent,
        instance: &InstanceContext,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification Boundary de synchronisation: {}",
            sync_intent.sync_id()
        );

        // Vérification 1: L'instance source DOIT être une Instance Fille
        if sync_intent.source_type() != InstanceType::Daughter {
            println!(
                "[RuntimeBoundary] Boundary de sync rejetée: la source {} n'est pas une Instance Fille",
                sync_intent.source_instance_id()
            );
            return Err(KMError::IllegalOperation {
                reason: "Seule une Instance Fille peut initier une synchronisation. L'Instance Mère est la source de vérité autoritaire.".to_string(),
            });
        }

        // Vérification 2: Cohérence du contexte - l'instance courante doit correspondre à la source
        if instance.instance_id != sync_intent.source_instance_id() {
            println!(
                "[RuntimeBoundary] Boundary de sync rejetée: incohérence entre instance courante ({}) et source sync ({})",
                instance.instance_id,
                sync_intent.source_instance_id()
            );
            return Err(KMError::InvalidContext {
                reason: format!(
                    "Incohérence de contexte: l'instance courante ({}) ne correspond pas à la source de synchronisation ({}).",
                    instance.instance_id,
                    sync_intent.source_instance_id()
                ),
            });
        }

        // Vérification 3: L'intention de sync doit être en état Pending ou Submitted
        if sync_intent.state() != SyncIntentState::Pending
            && sync_intent.state() != SyncIntentState::Submitted
        {
            println!(
                "[RuntimeBoundary] Boundary de sync rejetée: état invalide {}",
                sync_intent.state().to_string()
            );
            return Err(KMError::InvalidState {
                current_state: sync_intent.state().to_string().to_string(),
                reason: "L'intention de synchronisation doit être en état EN_ATTENTE ou SOUMISE.".to_string(),
            });
        }

        // Vérification 4: À CE STADE, TOUTE SYNCHRONISATION EST REFUSÉE EXPLICITEMENT
        // C'est une règle absolue du contrat : "sync → TOUJOURS refusé"
        println!(
            "[RuntimeBoundary] Boundary de sync rejetée: synchronisation non implémentée (refus explicite)"
        );
        Err(KMError::InsufficientPermissions {
            reason: "Les opérations de synchronisation sont toujours refusées à ce stade. Conforme au contrat FONDATION : la synchronisation sera implémentée dans une version ultérieure.".to_string(),
        })
    }

    /// Vérifie la Boundary de synchronisation avec contexte complet
    ///
    /// Traverse toutes les boundaries pertinentes pour une opération de synchronisation.
    pub fn check_all_sync_boundaries(
        &mut self,
        sync_intent: &SyncIntent,
        authority: &AuthorityContext,
        instance: &InstanceContext,
        domain: &DomainContext,
        current_state: KMState,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification de toutes les boundaries pour sync: {}",
            sync_intent.sync_id()
        );

        // Boundary 0: État KM (sync = écriture)
        Self::check_km_state_boundary(current_state, "sync", true)?;

        // Boundary 1: Appel
        Self::check_call_boundary("sync")?;

        // Boundary 2: Contexte
        Self::check_context_boundary(authority, instance, domain)?;

        // Boundary 3: Instance
        Self::check_instance_boundary(instance)?;

        // Boundary 4: Permissions (sync toujours refusé)
        // Note: check_permissions_boundary détecte déjà les opérations sync
        Self::check_permissions_boundary(authority, "sync", true)?;

        // Boundary 5: Sync spécifique
        Self::check_sync_boundary(sync_intent, instance)?;

        // Boundary 6: Cohérence
        Self::check_consistency_boundary("sync")?;

        // Boundary 7: Contournement
        Self::check_bypass_boundary("sync")?;

        // Boundary 8: Charge
        Self::check_load_boundary("sync")?;

        println!(
            "[RuntimeBoundary] Toutes les boundaries validées pour sync: {}",
            sync_intent.sync_id()
        );
        Ok(())
    }

    /// Vérifie la Boundary WriteIntent
    ///
    /// Vérifie que l'intention est valide pour soumission :
    /// - Intention doit être nouvelle (pas déjà vue)
    /// - Intention doit être en état Created
    /// - Aucune resoumission possible
    pub fn check_write_intent_boundary(
        &mut self,
        intent: &WriteIntent,
    ) -> BoundaryResult {
        let intent_id = intent.intent_id();
        println!("[RuntimeBoundary] Vérification Boundary WriteIntent: {}", intent_id);

        // Vérification 1: Intention doit être en état Created
        if intent.state() != WriteIntentState::Created {
            println!(
                "[RuntimeBoundary] Boundary WriteIntent rejetée: intention {} n'est pas en état Created (état actuel: {})",
                intent_id,
                intent.state().to_string()
            );
            return Err(KMError::IllegalOperation {
                reason: format!(
                    "L'intention {} doit être en état CRÉÉE pour être soumise. État actuel: {}.",
                    intent_id,
                    intent.state().to_string()
                ),
            });
        }

        // Vérification 2: Intention doit être nouvelle (pas déjà vue)
        if self.has_seen_intent(intent_id) {
            println!(
                "[RuntimeBoundary] Boundary WriteIntent rejetée: intention {} déjà vue (resoumission interdite)",
                intent_id
            );
            return Err(KMError::IllegalOperation {
                reason: format!(
                    "L'intention {} a déjà été soumise. Aucune resoumission n'est autorisée.",
                    intent_id
                ),
            });
        }

        // Marquer l'intention comme vue
        self.mark_intent_seen(intent_id.to_string());

        println!("[RuntimeBoundary] Boundary WriteIntent validée: intention {} acceptée", intent_id);
        Ok(())
    }

    /// Vérifie toutes les Runtime Boundaries
    ///
    /// Traverse toutes les boundaries dans l'ordre.
    /// Implémente les boundaries structurelles selon le contrat FONDATION.
    pub fn check_all_boundaries(
        &mut self,
        operation: &str,
        authority: &AuthorityContext,
        instance: &InstanceContext,
        domain: &DomainContext,
        current_state: KMState,
        is_write: bool,
    ) -> BoundaryResult {
        println!("[RuntimeBoundary] Vérification de toutes les boundaries pour: {}", operation);

        // Boundary 0: État KM (vérifiée en premier)
        Self::check_km_state_boundary(current_state, operation, is_write)?;

        // Boundary 1: Appel
        Self::check_call_boundary(operation)?;

        // Boundary 2: Contexte
        Self::check_context_boundary(authority, instance, domain)?;

        // Boundary 3: Instance
        Self::check_instance_boundary(instance)?;

        // Boundary 4: Permissions
        Self::check_permissions_boundary(authority, operation, is_write)?;

        // Boundary 5: Cohérence (pas de logique métier à ce stade)
        Self::check_consistency_boundary(operation)?;

        // Boundary 6: Contournement (pas de logique métier à ce stade)
        Self::check_bypass_boundary(operation)?;

        // Boundary 7: Charge (pas de logique métier à ce stade)
        Self::check_load_boundary(operation)?;

        println!("[RuntimeBoundary] Toutes les boundaries validées pour: {}", operation);
        Ok(())
    }

    /// Vérifie toutes les Runtime Boundaries pour une WriteIntent
    ///
    /// Traverse toutes les boundaries dans l'ordre, incluant la boundary WriteIntent.
    pub fn check_all_boundaries_with_intent(
        &mut self,
        operation: &str,
        intent: &WriteIntent,
        authority: &AuthorityContext,
        instance: &InstanceContext,
        domain: &DomainContext,
        current_state: KMState,
    ) -> BoundaryResult {
        println!(
            "[RuntimeBoundary] Vérification de toutes les boundaries avec WriteIntent pour: {} (intent: {})",
            operation,
            intent.intent_id()
        );

        // Boundary 0: État KM (écriture = true pour WriteIntent)
        Self::check_km_state_boundary(current_state, operation, true)?;

        // Boundary 1: Appel
        Self::check_call_boundary(operation)?;

        // Boundary 2: Contexte
        Self::check_context_boundary(authority, instance, domain)?;

        // Boundary 3: Instance
        Self::check_instance_boundary(instance)?;

        // Boundary WriteIntent: Vérifie que l'intention est nouvelle et en état Created
        self.check_write_intent_boundary(intent)?;

        // Boundary 4: Permissions (avec vérification du type d'opération du WriteIntent)
        Self::check_permissions_boundary_with_intent(authority, operation, intent)?;

        // Boundary 5: Cohérence (pas de logique métier à ce stade)
        Self::check_consistency_boundary(operation)?;

        // Boundary 6: Contournement (pas de logique métier à ce stade)
        Self::check_bypass_boundary(operation)?;

        // Boundary 7: Charge (pas de logique métier à ce stade)
        Self::check_load_boundary(operation)?;

        println!(
            "[RuntimeBoundary] Toutes les boundaries validées pour WriteIntent: {}",
            intent.intent_id()
        );
        Ok(())
    }
}
