//! Types fondamentaux de KindMother
//!
//! Définit les structures de base du moteur.

use crate::errors::KMError;
use crate::lifecycle::{WriteIntentLifecycle, WriteIntentState};
use crate::observability::{ObservableEvent, Observability};
use crate::state::KMState;

/// Contexte d'autorité
///
/// Représente le contexte d'autorisation pour une opération.
/// Permissions conceptuelles : can_read, can_write, can_sync.
#[derive(Debug, Clone)]
pub struct AuthorityContext {
    /// Identité de l'appelant (conceptuel)
    pub caller_identity: String,
    /// Permissions (conceptuel, non validé à ce stade)
    pub permissions: Vec<String>,
    /// Permission conceptuelle : peut lire
    pub can_read: bool,
    /// Permission conceptuelle : peut écrire
    pub can_write: bool,
    /// Permission conceptuelle : peut synchroniser (TOUJOURS refusé à ce stade)
    pub can_sync: bool,
}

impl AuthorityContext {
    /// Crée un nouveau contexte d'autorité
    pub fn new(caller_identity: String, permissions: Vec<String>) -> Self {
        Self {
            caller_identity,
            permissions,
            can_read: false,
            can_write: false,
            can_sync: false,
        }
    }

    /// Crée un nouveau contexte d'autorité avec permissions conceptuelles
    pub fn with_permissions(
        caller_identity: String,
        permissions: Vec<String>,
        can_read: bool,
        can_write: bool,
        can_sync: bool,
    ) -> Self {
        Self {
            caller_identity,
            permissions,
            can_read,
            can_write,
            can_sync,
        }
    }

    /// Vérifie si le contexte est valide (skeleton: toujours invalide)
    pub fn is_valid(&self) -> bool {
        // À ce stade skeleton, aucun contexte n'est valide
        false
    }
}

/// Contexte d'instance
///
/// Représente le contexte d'une instance KindMother.
#[derive(Debug, Clone)]
pub struct InstanceContext {
    /// Identité de l'instance (conceptuel)
    pub instance_id: String,
    /// Type d'instance (Mère ou Fille, conceptuel)
    pub instance_type: InstanceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceType {
    /// Instance Mère (source de vérité)
    Mother,
    /// Instance Fille (dérivée)
    Daughter,
}

impl InstanceContext {
    /// Crée un nouveau contexte d'instance
    pub fn new(instance_id: String, instance_type: InstanceType) -> Self {
        Self {
            instance_id,
            instance_type,
        }
    }

    /// Vérifie si le contexte est valide (skeleton: toujours invalide)
    pub fn is_valid(&self) -> bool {
        // À ce stade skeleton, aucune instance n'est valide
        false
    }
}

/// Contexte de domaine
///
/// Représente le contexte d'un domaine d'autorité.
#[derive(Debug, Clone)]
pub struct DomainContext {
    /// Identité du domaine (conceptuel)
    pub domain_id: String,
    /// Nom du domaine (Identity, RPG, Commerce, CMS, etc.)
    pub domain_name: String,
}

impl DomainContext {
    /// Crée un nouveau contexte de domaine
    pub fn new(domain_id: String, domain_name: String) -> Self {
        Self {
            domain_id,
            domain_name,
        }
    }

    /// Vérifie si le contexte est valide (skeleton: toujours invalide)
    pub fn is_valid(&self) -> bool {
        // À ce stade skeleton, aucun domaine n'est valide
        false
    }
}

/// WriteIntent
///
/// Représente une intention d'écriture avec son cycle de vie.
/// Conforme au contrat FONDATION "KindMother — Write Intent Lifecycle Contract".
#[derive(Debug, Clone)]
pub struct WriteIntent {
    /// Identité unique de l'intention (immuable)
    intent_id: String,
    /// Type d'opération (immuable)
    operation_type: String,
    /// État actuel dans le cycle de vie
    state: WriteIntentState,
}

impl WriteIntent {
    /// Crée un nouveau WriteIntent dans l'état CRÉÉE
    ///
    /// Conforme à la règle CREAT-3 : l'identité est attribuée par KindMother.
    /// Conforme à la règle CREAT-4 : l'intention est immuable après création.
    pub fn new(intent_id: String, operation_type: String) -> Self {
        println!(
            "[WriteIntent] Création d'une nouvelle intention {} (opération: {})",
            intent_id, operation_type
        );
        Self {
            intent_id,
            operation_type,
            state: WriteIntentState::Created,
        }
    }

    /// Obtient l'identité de l'intention
    pub fn intent_id(&self) -> &str {
        &self.intent_id
    }

    /// Obtient le type d'opération
    pub fn operation_type(&self) -> &str {
        &self.operation_type
    }

    /// Obtient l'état actuel de l'intention
    pub fn state(&self) -> WriteIntentState {
        self.state
    }

    /// Démarre la validation (CRÉÉE → EN_VALIDATION)
    ///
    /// Conforme à la règle VALID-1 : toute Write Intent DOIT traverser les Runtime Boundaries.
    pub fn start_validation(&mut self) -> Result<(), KMError> {
        self.state = WriteIntentLifecycle::start_validation(self.state, &self.intent_id)?;
        Ok(())
    }

    /// Accepte l'intention (EN_VALIDATION → ACCEPTÉE)
    ///
    /// Conforme à la règle ACCEPT-1 : une intention ACCEPTÉE DOIT être appliquée.
    pub fn accept(&mut self) -> Result<(), KMError> {
        self.state = WriteIntentLifecycle::accept(self.state, &self.intent_id)?;
        Ok(())
    }

    /// Rejette l'intention (EN_VALIDATION → REJETÉE)
    ///
    /// Conforme à la règle REJECT-1 : un rejet DOIT indiquer explicitement la raison.
    pub fn reject(&mut self, reason: String) -> Result<(), KMError> {
        self.state = WriteIntentLifecycle::reject(self.state, &self.intent_id, reason)?;
        Ok(())
    }

    /// Applique l'intention (ACCEPTÉE → APPLIQUÉE)
    ///
    /// Conforme à la règle APPLY-1 : l'application est atomique (conceptuel, no-op en mémoire).
    pub fn apply(&mut self) -> Result<(), KMError> {
        self.state = WriteIntentLifecycle::apply(self.state, &self.intent_id)?;
        Ok(())
    }

    /// Archive l'intention (REJETÉE/APPLIQUÉE → ARCHIVÉE)
    ///
    /// Conforme à la règle ARCHIV-1 : toute intention terminée DOIT être archivée.
    pub fn archive(&mut self) -> Result<(), KMError> {
        self.state = WriteIntentLifecycle::archive(self.state, &self.intent_id)?;
        Ok(())
    }

    /// Vérifie si l'intention peut être réutilisée
    ///
    /// Conforme à la règle NOREUSE-1 : une Write Intent ne peut être soumise qu'une seule fois.
    pub fn can_reuse(&self) -> bool {
        WriteIntentLifecycle::can_reuse(self.state)
    }

    /// Vérifie si l'intention est dans un état terminal
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }
}

/// KindMother - Moteur principal
///
/// Structure principale du moteur KindMother.
/// Gère la persistance interne autoritaire.
pub struct KindMother {
    /// État actuel du moteur
    state: KMState,
    /// Storage interne autoritaire (opaque)
    storage: crate::storage::InternalStorage,
    /// Système d'observabilité
    observability: Observability,
    /// Identifiant de l'instance
    instance_id: String,
    /// Identifiant du domaine
    domain_id: String,
}

impl KindMother {
    /// Crée une nouvelle instance de KindMother
    ///
    /// Le moteur démarre dans l'état `Booting`.
    pub fn new() -> Self {
        Self::with_identity("km-default".to_string(), "default-domain".to_string())
    }

    /// Crée une nouvelle instance de KindMother avec identité explicite
    ///
    /// Le moteur démarre dans l'état `Booting`.
    pub fn with_identity(instance_id: String, domain_id: String) -> Self {
        println!(
            "[KindMother] Création d'une nouvelle instance {} dans domaine {}",
            instance_id, domain_id
        );

        let mut km = Self {
            state: KMState::Booting,
            storage: crate::storage::InternalStorage::new(),
            observability: Observability::new(instance_id.clone(), domain_id.clone()),
            instance_id,
            domain_id,
        };

        // Enregistrer l'événement d'initialisation
        km.observability.record_event(ObservableEvent::InstanceInitialized {
            instance_id: km.instance_id.clone(),
        });

        km
    }

    /// Obtient l'identifiant de l'instance
    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    /// Obtient l'identifiant du domaine
    pub fn domain_id(&self) -> &str {
        &self.domain_id
    }

    /// Démarre le moteur
    ///
    /// Transitionne de `Booting` vers `Healthy` si le démarrage réussit.
    /// À ce stade skeleton, le démarrage échoue toujours.
    pub fn start(&mut self) -> Result<(), KMError> {
        println!("[KindMother] Tentative de démarrage");
        
        if self.state != KMState::Booting {
            return Err(KMError::InvalidState {
                current_state: self.state.to_string(),
                reason: "Le moteur n'est pas en état Booting".to_string(),
            });
        }

        // À ce stade skeleton, le démarrage échoue toujours
        println!("[KindMother] Démarrage rejeté (skeleton)");
        Err(KMError::NotImplemented {
            operation: "start".to_string(),
        })
    }

    /// Obtient l'état actuel du moteur
    pub fn state(&self) -> KMState {
        self.state
    }

    /// Force une transition d'état (pour les tests et exemples)
    pub fn set_state(&mut self, state: KMState) {
        println!("[KindMother] Transition d'état vers: {:?}", state);
        self.state = state;
    }

    /// Persiste une WriteIntent
    ///
    /// UNIQUEMENT si l'intention est en état Applied.
    /// Conforme au contrat FONDATION : persistance uniquement après application.
    pub fn persist_intent(&mut self, intent: WriteIntent, domain_id: &str) -> Result<(), KMError> {
        // Vérification de corruption avant persistance
        if self.storage.is_corrupted() {
            // Passage automatique en Degraded si corruption détectée
            if self.state != KMState::Quarantined {
                println!(
                    "[KindMother] Corruption détectée, passage en Degraded. Raison: {}",
                    self.storage.corruption_reason().unwrap_or("inconnue")
                );
                self.state = KMState::Degraded;
            }
            return Err(KMError::ConsistencyViolation {
                reason: format!(
                    "Le storage est corrompu. Raison: {}",
                    self.storage.corruption_reason().unwrap_or("inconnue")
                ),
            });
        }

        // Persistance atomique
        match self.storage.persist_intent(intent, domain_id) {
            Ok(()) => {
                // Vérification de cohérence après persistance
                if !self.storage.verify_consistency() {
                    // Corruption détectée après persistance → passage en Degraded
                    println!(
                        "[KindMother] Corruption détectée après persistance, passage en Degraded"
                    );
                    self.state = KMState::Degraded;
                    return Err(KMError::ConsistencyViolation {
                        reason: format!(
                            "Corruption détectée après persistance. Raison: {}",
                            self.storage.corruption_reason().unwrap_or("inconnue")
                        ),
                    });
                }
                Ok(())
            }
            Err(e) => {
                // En cas d'erreur de persistance, vérifier si corruption détectée
                if self.storage.is_corrupted() {
                    if self.state != KMState::Quarantined {
                        println!(
                            "[KindMother] Corruption détectée lors de la persistance, passage en Degraded"
                        );
                        self.state = KMState::Degraded;
                    }
                }
                Err(e)
            }
        }
    }

    /// Vérifie la cohérence du storage (détection proactive)
    ///
    /// Retourne true si cohérent, false si corruption détectée.
    /// En cas de corruption, passe automatiquement en Degraded.
    pub fn verify_storage_consistency(&mut self) -> bool {
        if !self.storage.verify_consistency() {
            if self.state != KMState::Quarantined {
                println!(
                    "[KindMother] Corruption détectée lors de la vérification, passage en Degraded"
                );
                self.state = KMState::Degraded;
            }
            false
        } else {
            true
        }
    }

    /// Obtient le nombre d'intentions persistées dans un domaine
    ///
    /// Utilisé uniquement pour les tests et la vérification interne.
    /// Aucune API publique de lecture n'est exposée.
    pub fn count_persisted_intents(&self, domain_id: &str) -> usize {
        self.storage.count_persisted_intents(domain_id)
    }

    /// Simule un crash du storage (pour les tests)
    ///
    /// Marque le storage comme corrompu pour simuler un crash.
    pub fn simulate_storage_crash(&mut self) {
        self.storage.simulate_crash();
    }

    /// Transition explicite vers l'état Degraded
    ///
    /// Appelé en cas de détection de problème nécessitant une dégradation contrôlée.
    /// Les lectures restent autorisées, les écritures sont refusées.
    ///
    /// Conforme au contrat FONDATION "Runtime Boundary & Enforcement Contract" (R4).
    ///
    /// Retourne une erreur si la transition n'est pas autorisée :
    /// - Depuis Quarantined : transition interdite
    /// - Depuis Degraded : déjà en Degraded
    pub fn transition_to_degraded(&mut self, reason: &str) -> Result<(), KMError> {
        println!(
            "[KindMother] Tentative de transition vers Degraded. Raison: {}",
            reason
        );

        match self.state {
            KMState::Quarantined => {
                println!("[KindMother] Transition vers Degraded refusée: moteur en Quarantined");
                Err(KMError::InvalidState {
                    current_state: self.state.to_string(),
                    reason: "Impossible de passer en Degraded depuis Quarantined. La quarantaine est un état bloquant qui nécessite une intervention manuelle.".to_string(),
                })
            }
            KMState::Degraded => {
                println!("[KindMother] Déjà en état Degraded");
                // Ce n'est pas une erreur, juste un no-op
                Ok(())
            }
            KMState::Booting | KMState::Healthy => {
                let from_state = self.state;
                println!(
                    "[KindMother] Transition de {:?} vers Degraded effectuée. Raison: {}",
                    from_state, reason
                );
                self.state = KMState::Degraded;

                // Enregistrer l'événement
                self.observability.record_event(ObservableEvent::StateChanged {
                    from: from_state,
                    to: KMState::Degraded,
                    reason: reason.to_string(),
                });
                self.observability.record_event(ObservableEvent::DegradationTriggered {
                    reason: reason.to_string(),
                });

                Ok(())
            }
        }
    }

    /// Transition explicite vers l'état Quarantined
    ///
    /// Appelé en cas de détection de menace grave ou de corruption irréparable.
    /// TOUTES les opérations sont bloquées.
    ///
    /// Conforme au contrat FONDATION "Runtime Boundary & Enforcement Contract" (R3).
    ///
    /// ATTENTION : La quarantaine est un état bloquant.
    /// Aucune opération ne sera autorisée tant que la quarantaine n'est pas levée manuellement.
    pub fn transition_to_quarantined(&mut self, reason: &str) -> Result<(), KMError> {
        println!(
            "[KindMother] Tentative de transition vers Quarantined. Raison: {}",
            reason
        );

        if self.state == KMState::Quarantined {
            println!("[KindMother] Déjà en état Quarantined");
            // Ce n'est pas une erreur, juste un no-op
            return Ok(());
        }

        let from_state = self.state;
        println!(
            "[KindMother] Transition de {:?} vers Quarantined effectuée. Raison: {}",
            from_state, reason
        );
        self.state = KMState::Quarantined;

        // Enregistrer les événements
        self.observability.record_event(ObservableEvent::StateChanged {
            from: from_state,
            to: KMState::Quarantined,
            reason: reason.to_string(),
        });
        self.observability.record_event(ObservableEvent::QuarantineEntered {
            entity_id: self.instance_id.clone(),
            reason: reason.to_string(),
        });

        Ok(())
    }

    /// Tente de sortir de l'état Quarantined (intervention manuelle)
    ///
    /// Cette méthode représente une intervention manuelle pour sortir de la quarantaine.
    /// Elle ne doit être appelée que après vérification et correction du problème.
    ///
    /// ATTENTION : Cette méthode est dangereuse. Elle doit être utilisée avec précaution.
    /// L'état cible est Degraded, pas Healthy, pour permettre une reprise progressive.
    pub fn lift_quarantine(&mut self, reason: &str) -> Result<(), KMError> {
        println!(
            "[KindMother] Tentative de levée de quarantaine. Raison: {}",
            reason
        );

        if self.state != KMState::Quarantined {
            println!("[KindMother] Levée de quarantaine impossible: pas en Quarantined");
            return Err(KMError::InvalidState {
                current_state: self.state.to_string(),
                reason: "Impossible de lever la quarantaine: le moteur n'est pas en quarantaine.".to_string(),
            });
        }

        // Transition vers Degraded pour une reprise progressive
        println!(
            "[KindMother] Quarantaine levée. Transition vers Degraded. Raison: {}",
            reason
        );
        self.state = KMState::Degraded;

        // Enregistrer les événements
        self.observability.record_event(ObservableEvent::QuarantineExited {
            entity_id: self.instance_id.clone(),
            reason: reason.to_string(),
        });
        self.observability.record_event(ObservableEvent::StateChanged {
            from: KMState::Quarantined,
            to: KMState::Degraded,
            reason: reason.to_string(),
        });

        Ok(())
    }

    /// Tente de récupérer vers l'état Healthy
    ///
    /// Cette méthode représente une récupération complète après dégradation.
    /// Elle ne doit être appelée que depuis l'état Degraded.
    pub fn recover_to_healthy(&mut self, reason: &str) -> Result<(), KMError> {
        println!(
            "[KindMother] Tentative de récupération vers Healthy. Raison: {}",
            reason
        );

        match self.state {
            KMState::Quarantined => {
                println!("[KindMother] Récupération impossible: en Quarantined");
                Err(KMError::InvalidState {
                    current_state: self.state.to_string(),
                    reason: "Impossible de récupérer vers Healthy depuis Quarantined. La quarantaine doit d'abord être levée.".to_string(),
                })
            }
            KMState::Healthy => {
                println!("[KindMother] Déjà en état Healthy");
                Ok(())
            }
            KMState::Booting => {
                // Transition depuis Booting vers Healthy (démarrage réussi)
                println!(
                    "[KindMother] Démarrage réussi. Transition vers Healthy. Raison: {}",
                    reason
                );
                self.state = KMState::Healthy;

                // Enregistrer l'événement
                self.observability.record_event(ObservableEvent::StateChanged {
                    from: KMState::Booting,
                    to: KMState::Healthy,
                    reason: reason.to_string(),
                });

                Ok(())
            }
            KMState::Degraded => {
                // Vérification de la cohérence du storage avant récupération
                if !self.verify_storage_consistency() {
                    println!(
                        "[KindMother] Récupération impossible: storage corrompu"
                    );
                    return Err(KMError::ConsistencyViolation {
                        reason: "Impossible de récupérer vers Healthy: le storage est encore corrompu.".to_string(),
                    });
                }

                println!(
                    "[KindMother] Récupération réussie. Transition vers Healthy. Raison: {}",
                    reason
                );
                self.state = KMState::Healthy;

                // Enregistrer les événements
                self.observability.record_event(ObservableEvent::DegradationExited {
                    reason: reason.to_string(),
                });
                self.observability.record_event(ObservableEvent::StateChanged {
                    from: KMState::Degraded,
                    to: KMState::Healthy,
                    reason: reason.to_string(),
                });
                self.observability.record_event(ObservableEvent::RecoveryCompleted {
                    description: reason.to_string(),
                });

                Ok(())
            }
        }
    }

    /// Accède au système d'observabilité
    pub fn observability(&self) -> &Observability {
        &self.observability
    }

    /// Accède au système d'observabilité (mutable)
    pub fn observability_mut(&mut self) -> &mut Observability {
        &mut self.observability
    }
}

impl Default for KindMother {
    fn default() -> Self {
        Self::new()
    }
}
