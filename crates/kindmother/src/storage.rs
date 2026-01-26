//! Storage interne autoritaire de KindMother
//!
//! Implémente la persistance interne selon le contrat FONDATION :
//! "KindMother — Persistence & Storage Contract"
//!
//! CARACTÉRISTIQUES :
//! - Persistance UNIQUEMENT après transition vers Applied
//! - Atomicité garantie (tout ou rien)
//! - Détection de corruption → passage en Degraded
//! - Isolation par domaine
//! - Aucun accès externe

use crate::core::WriteIntent;
use crate::errors::KMError;
use crate::lifecycle::WriteIntentState;
use std::collections::HashMap;

/// Storage interne autoritaire
///
/// Stockage opaque accessible uniquement depuis KindMother.
/// Aucune structure exposée publiquement.
pub struct InternalStorage {
    /// Stockage par domaine (isolation stricte)
    /// domain_id -> intent_id -> WriteIntent
    data: HashMap<String, HashMap<String, WriteIntent>>,
    /// État de corruption détectée
    corrupted: bool,
    /// Raison de la corruption (si détectée)
    corruption_reason: Option<String>,
}

impl InternalStorage {
    /// Crée un nouveau storage interne
    pub fn new() -> Self {
        println!("[InternalStorage] Création d'un nouveau storage interne");
        Self {
            data: HashMap::new(),
            corrupted: false,
            corruption_reason: None,
        }
    }

    /// Vérifie si le storage est corrompu
    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }

    /// Obtient la raison de la corruption (si détectée)
    pub fn corruption_reason(&self) -> Option<&str> {
        self.corruption_reason.as_deref()
    }

    /// Marque le storage comme corrompu
    fn mark_corrupted(&mut self, reason: String) {
        println!("[InternalStorage] Corruption détectée: {}", reason);
        self.corrupted = true;
        self.corruption_reason = Some(reason);
    }

    /// Vérifie la cohérence du storage
    ///
    /// Détecte les corruptions conceptuelles :
    /// - WriteIntent en état non-Applied dans le storage
    /// - Duplications d'intent_id
    /// - Incohérences structurelles
    fn check_consistency(&self) -> Result<(), String> {
        for (domain_id, domain_data) in &self.data {
            for (intent_id, intent) in domain_data {
                // Vérification 1: Seules les intentions Applied doivent être persistées
                if intent.state() != WriteIntentState::Applied {
                    return Err(format!(
                        "Corruption détectée: intention {} dans le domaine {} est en état {} au lieu de APPLIQUÉE",
                        intent_id,
                        domain_id,
                        intent.state().to_string()
                    ));
                }

                // Vérification 2: Cohérence de l'intent_id
                if intent.intent_id() != intent_id {
                    return Err(format!(
                        "Corruption détectée: intention {} dans le domaine {} a un intent_id incohérent",
                        intent_id,
                        domain_id
                    ));
                }
            }
        }

        Ok(())
    }

    /// Persiste une WriteIntent
    ///
    /// UNIQUEMENT si l'intention est en état Applied.
    /// Atomicité garantie : toute erreur annule la persistance.
    pub fn persist_intent(
        &mut self,
        intent: WriteIntent,
        domain_id: &str,
    ) -> Result<(), KMError> {
        println!(
            "[InternalStorage] Tentative de persistance de l'intention {} dans le domaine {}",
            intent.intent_id(),
            domain_id
        );

        // Vérification 1: Storage non corrompu
        if self.corrupted {
            return Err(KMError::ConsistencyViolation {
                reason: format!(
                    "Le storage est corrompu. Raison: {}",
                    self.corruption_reason.as_deref().unwrap_or("inconnue")
                ),
            });
        }

        // Vérification 2: Intention UNIQUEMENT en état Applied
        if intent.state() != WriteIntentState::Applied {
            return Err(KMError::IllegalOperation {
                reason: format!(
                    "Persistance refusée: l'intention {} n'est pas en état APPLIQUÉE (état actuel: {}). Seules les intentions APPLIQUÉES peuvent être persistées.",
                    intent.intent_id(),
                    intent.state().to_string()
                ),
            });
        }

        // Vérification 3: Cohérence avant persistance
        if let Err(reason) = self.check_consistency() {
            self.mark_corrupted(reason.clone());
            return Err(KMError::ConsistencyViolation {
                reason: format!("Corruption détectée avant persistance: {}", reason),
            });
        }

        // Persistance atomique
        let intent_id = intent.intent_id().to_string();
        let domain_id_str = domain_id.to_string();

        // Vérification 4: Pas de duplication (invariant)
        if let Some(domain_data) = self.data.get(&domain_id_str) {
            if domain_data.contains_key(&intent_id) {
                let reason = format!(
                    "Corruption détectée: intention {} déjà présente dans le domaine {}",
                    intent_id, domain_id
                );
                self.mark_corrupted(reason.clone());
                return Err(KMError::ConsistencyViolation {
                    reason: format!("Corruption détectée: {}", reason),
                });
            }
        }

        // Persistance effective (atomique)
        let domain_data = self.data.entry(domain_id_str).or_insert_with(HashMap::new);
        domain_data.insert(intent_id.clone(), intent);

        // Vérification 5: Cohérence après persistance
        if let Err(reason) = self.check_consistency() {
            // Rollback atomique en cas d'erreur
            if let Some(domain_data) = self.data.get_mut(domain_id) {
                domain_data.remove(&intent_id);
            }
            self.mark_corrupted(reason.clone());
            return Err(KMError::ConsistencyViolation {
                reason: format!("Corruption détectée après persistance: {}", reason),
            });
        }

        println!(
            "[InternalStorage] Intention {} persistée avec succès dans le domaine {}",
            intent_id,
            domain_id
        );
        Ok(())
    }

    /// Vérifie la cohérence du storage (appel externe pour détection proactive)
    ///
    /// Retourne true si le storage est cohérent, false si une corruption est détectée.
    pub fn verify_consistency(&mut self) -> bool {
        if self.corrupted {
            return false;
        }

        match self.check_consistency() {
            Ok(()) => {
                println!("[InternalStorage] Vérification de cohérence: OK");
                true
            }
            Err(reason) => {
                self.mark_corrupted(reason);
                false
            }
        }
    }

    /// Obtient le nombre d'intentions persistées dans un domaine
    ///
    /// Utilisé uniquement pour les tests et la vérification interne.
    pub(crate) fn count_persisted_intents(&self, domain_id: &str) -> usize {
        self.data
            .get(domain_id)
            .map(|domain_data| domain_data.len())
            .unwrap_or(0)
    }

    /// Simule un crash (pour les tests)
    ///
    /// Marque le storage comme corrompu pour simuler un crash.
    pub fn simulate_crash(&mut self) {
        self.mark_corrupted("Crash simulé pour test".to_string());
    }
}

impl Default for InternalStorage {
    fn default() -> Self {
        Self::new()
    }
}
