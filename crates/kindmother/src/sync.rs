//! Sync & Conflict Resolution - Gestion de la synchronisation
//!
//! Implémente la synchronisation et la détection de conflits conforme au contrat FONDATION :
//! "KindMother — Sync & Conflict Resolution Contract"
//!
//! CARACTÉRISTIQUES :
//! - SyncIntent pour les opérations de synchronisation
//! - Détection de conflits (autoritaire, temporel, sémantique)
//! - AUCUNE résolution automatique
//! - AUCUNE synchronisation silencieuse
//! - Toute sync reste : explicite, traçable, validée, refusée par défaut

use crate::core::InstanceType;
use crate::errors::KMError;

/// État d'une SyncIntent dans son cycle de vie
///
/// Conforme au contrat FONDATION "Sync & Conflict Resolution Contract" section 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncIntentState {
    /// État initial après création
    Pending,
    /// Soumise pour validation par l'Instance Mère
    Submitted,
    /// Validée par l'Instance Mère
    Validated,
    /// Rejetée par l'Instance Mère
    Rejected,
    /// Appliquée après validation définitive
    Applied,
}

impl SyncIntentState {
    /// Vérifie si l'état est terminal (aucune transition possible)
    pub fn is_terminal(&self) -> bool {
        matches!(self, SyncIntentState::Rejected | SyncIntentState::Applied)
    }

    /// Vérifie si l'état permet l'application
    pub fn can_apply(&self) -> bool {
        matches!(self, SyncIntentState::Validated)
    }

    /// Retourne une représentation textuelle de l'état
    pub fn to_string(&self) -> &'static str {
        match self {
            SyncIntentState::Pending => "EN_ATTENTE",
            SyncIntentState::Submitted => "SOUMISE",
            SyncIntentState::Validated => "VALIDÉE",
            SyncIntentState::Rejected => "REJETÉE",
            SyncIntentState::Applied => "APPLIQUÉE",
        }
    }
}

/// Type de conflit détecté lors de la synchronisation
///
/// Conforme au contrat FONDATION section 3 "Types de conflits conceptuels".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictType {
    /// Conflit autoritaire : opération Fille vs décision définitive Mère
    /// La Mère a autorité définitive, sa décision est non négociable.
    Authoritative,
    /// Conflit temporel : modifications concurrentes sur même entité
    /// L'ordre temporel des modifications crée une incohérence.
    Temporal,
    /// Conflit sémantique : violation des contraintes de cohérence
    /// L'opération viole des règles métier établies par la Mère.
    Semantic,
}

impl ConflictType {
    /// Retourne une représentation textuelle du type de conflit
    pub fn to_string(&self) -> &'static str {
        match self {
            ConflictType::Authoritative => "AUTORITAIRE",
            ConflictType::Temporal => "TEMPOREL",
            ConflictType::Semantic => "SÉMANTIQUE",
        }
    }

    /// Retourne une description du type de conflit
    pub fn description(&self) -> &'static str {
        match self {
            ConflictType::Authoritative => {
                "Opération locale en contradiction avec une décision définitive de l'Instance Mère"
            }
            ConflictType::Temporal => {
                "Modifications concurrentes sur la même entité créant une incohérence temporelle"
            }
            ConflictType::Semantic => {
                "Violation des contraintes de cohérence sémantique établies par l'Instance Mère"
            }
        }
    }
}

/// Conflit détecté lors de la synchronisation
///
/// Représente un conflit conceptuel détecté, sans résolution automatique.
/// Conforme au contrat FONDATION section 3.
#[derive(Debug, Clone)]
pub struct Conflict {
    /// Identifiant unique du conflit
    conflict_id: String,
    /// Type de conflit
    conflict_type: ConflictType,
    /// Identifiant de l'intention en conflit
    intent_id: String,
    /// Raison détaillée du conflit
    reason: String,
    /// Moment de détection (conceptuel, représenté comme compteur)
    detected_at: u64,
}

impl Conflict {
    /// Crée un nouveau conflit
    pub fn new(
        conflict_id: String,
        conflict_type: ConflictType,
        intent_id: String,
        reason: String,
        detected_at: u64,
    ) -> Self {
        println!(
            "[Conflict] Conflit {} détecté: type={}, intent={}, raison={}",
            conflict_id,
            conflict_type.to_string(),
            intent_id,
            reason
        );
        Self {
            conflict_id,
            conflict_type,
            intent_id,
            reason,
            detected_at,
        }
    }

    /// Obtient l'identifiant du conflit
    pub fn conflict_id(&self) -> &str {
        &self.conflict_id
    }

    /// Obtient le type de conflit
    pub fn conflict_type(&self) -> ConflictType {
        self.conflict_type
    }

    /// Obtient l'identifiant de l'intention en conflit
    pub fn intent_id(&self) -> &str {
        &self.intent_id
    }

    /// Obtient la raison du conflit
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Obtient le moment de détection
    pub fn detected_at(&self) -> u64 {
        self.detected_at
    }
}

/// SyncIntent - Intention de synchronisation
///
/// Représente une intention de synchronisation entre Instance Fille et Instance Mère.
/// Conforme au contrat FONDATION section 2.
///
/// RÈGLES ABSOLUES :
/// - AUCUNE résolution automatique de conflit
/// - AUCUNE synchronisation silencieuse
/// - Toute sync reste : explicite, traçable, validée, refusée par défaut
#[derive(Debug, Clone)]
pub struct SyncIntent {
    /// Identifiant unique de l'intention de synchronisation
    sync_id: String,
    /// Identifiant de l'instance source (Fille)
    source_instance_id: String,
    /// Identifiant de l'instance cible (Mère)
    target_instance_id: String,
    /// Type d'instance source (doit être Daughter)
    source_type: InstanceType,
    /// État actuel de la synchronisation
    state: SyncIntentState,
    /// Conflit détecté (si applicable)
    detected_conflict: Option<Conflict>,
    /// Raison du rejet (si applicable)
    rejection_reason: Option<String>,
}

impl SyncIntent {
    /// Crée une nouvelle intention de synchronisation
    ///
    /// L'intention est créée en état Pending.
    /// La source DOIT être une Instance Fille.
    pub fn new(
        sync_id: String,
        source_instance_id: String,
        target_instance_id: String,
        source_type: InstanceType,
    ) -> Result<Self, KMError> {
        println!(
            "[SyncIntent] Création d'une intention de synchronisation {} (source: {}, cible: {})",
            sync_id, source_instance_id, target_instance_id
        );

        // Validation structurelle : la source DOIT être une Instance Fille
        if source_type != InstanceType::Daughter {
            println!(
                "[SyncIntent] Création rejetée: la source doit être une Instance Fille"
            );
            return Err(KMError::IllegalOperation {
                reason: "Seule une Instance Fille peut initier une synchronisation. L'Instance Mère est la source de vérité.".to_string(),
            });
        }

        Ok(Self {
            sync_id,
            source_instance_id,
            target_instance_id,
            source_type,
            state: SyncIntentState::Pending,
            detected_conflict: None,
            rejection_reason: None,
        })
    }

    /// Obtient l'identifiant de la synchronisation
    pub fn sync_id(&self) -> &str {
        &self.sync_id
    }

    /// Obtient l'identifiant de l'instance source
    pub fn source_instance_id(&self) -> &str {
        &self.source_instance_id
    }

    /// Obtient l'identifiant de l'instance cible
    pub fn target_instance_id(&self) -> &str {
        &self.target_instance_id
    }

    /// Obtient le type de l'instance source
    pub fn source_type(&self) -> InstanceType {
        self.source_type
    }

    /// Obtient l'état actuel
    pub fn state(&self) -> SyncIntentState {
        self.state
    }

    /// Obtient le conflit détecté (si applicable)
    pub fn detected_conflict(&self) -> Option<&Conflict> {
        self.detected_conflict.as_ref()
    }

    /// Obtient la raison du rejet (si applicable)
    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }

    /// Soumet l'intention pour validation (Pending → Submitted)
    ///
    /// Conforme à la règle SYNC-3 : Validation par la Mère.
    pub fn submit(&mut self) -> Result<(), KMError> {
        if self.state != SyncIntentState::Pending {
            return Err(KMError::InvalidState {
                current_state: self.state.to_string().to_string(),
                reason: "Seule une intention en attente peut être soumise.".to_string(),
            });
        }

        println!(
            "[SyncIntent] Intention {} soumise pour validation",
            self.sync_id
        );
        self.state = SyncIntentState::Submitted;
        Ok(())
    }

    /// Valide l'intention (Submitted → Validated)
    ///
    /// ATTENTION : À ce stade, toute synchronisation est REFUSÉE.
    /// Cette méthode existe pour la structure, mais ne doit PAS être appelée.
    pub fn validate(&mut self) -> Result<(), KMError> {
        if self.state != SyncIntentState::Submitted {
            return Err(KMError::InvalidState {
                current_state: self.state.to_string().to_string(),
                reason: "Seule une intention soumise peut être validée.".to_string(),
            });
        }

        // À ce stade, la validation est conceptuelle uniquement
        // La synchronisation réelle n'est PAS implémentée
        println!(
            "[SyncIntent] Intention {} validée (conceptuel, pas d'application réelle)",
            self.sync_id
        );
        self.state = SyncIntentState::Validated;
        Ok(())
    }

    /// Rejette l'intention avec un conflit détecté (Submitted → Rejected)
    ///
    /// Conforme aux règles SYNC-1 et SYNC-2 : Autorité exclusive de la Mère.
    pub fn reject_with_conflict(&mut self, conflict: Conflict) -> Result<(), KMError> {
        if self.state != SyncIntentState::Submitted {
            return Err(KMError::InvalidState {
                current_state: self.state.to_string().to_string(),
                reason: "Seule une intention soumise peut être rejetée.".to_string(),
            });
        }

        let reason = format!(
            "Conflit {} détecté: {}",
            conflict.conflict_type().to_string(),
            conflict.reason()
        );
        println!(
            "[SyncIntent] Intention {} rejetée: {}",
            self.sync_id, reason
        );
        self.rejection_reason = Some(reason);
        self.detected_conflict = Some(conflict);
        self.state = SyncIntentState::Rejected;
        Ok(())
    }

    /// Rejette l'intention avec une raison (Submitted → Rejected)
    ///
    /// Conforme aux règles SYNC-1 et SYNC-2 : Autorité exclusive de la Mère.
    pub fn reject(&mut self, reason: String) -> Result<(), KMError> {
        if self.state != SyncIntentState::Submitted {
            return Err(KMError::InvalidState {
                current_state: self.state.to_string().to_string(),
                reason: "Seule une intention soumise peut être rejetée.".to_string(),
            });
        }

        println!(
            "[SyncIntent] Intention {} rejetée: {}",
            self.sync_id, reason
        );
        self.rejection_reason = Some(reason);
        self.state = SyncIntentState::Rejected;
        Ok(())
    }

    /// Applique l'intention validée (Validated → Applied)
    ///
    /// ATTENTION : À ce stade, l'application est conceptuelle uniquement.
    /// Aucune donnée n'est réellement synchronisée.
    pub fn apply(&mut self) -> Result<(), KMError> {
        if self.state != SyncIntentState::Validated {
            return Err(KMError::InvalidState {
                current_state: self.state.to_string().to_string(),
                reason: "Seule une intention validée peut être appliquée.".to_string(),
            });
        }

        // Application conceptuelle uniquement
        println!(
            "[SyncIntent] Intention {} appliquée (conceptuel, no-op)",
            self.sync_id
        );
        self.state = SyncIntentState::Applied;
        Ok(())
    }

    /// Vérifie si l'intention est dans un état terminal
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }
}

/// Détecteur de conflits conceptuel
///
/// Détecte les conflits lors de la synchronisation.
/// AUCUNE résolution automatique - uniquement détection et typage.
///
/// Conforme au contrat FONDATION section 3.
pub struct ConflictDetector {
    /// Compteur pour générer des identifiants de conflit
    conflict_counter: u64,
    /// Compteur pour le moment de détection
    detection_counter: u64,
}

impl ConflictDetector {
    /// Crée un nouveau détecteur de conflits
    pub fn new() -> Self {
        Self {
            conflict_counter: 0,
            detection_counter: 0,
        }
    }

    /// Génère un identifiant unique pour un conflit
    fn generate_conflict_id(&mut self) -> String {
        self.conflict_counter += 1;
        format!("conflict-{}", self.conflict_counter)
    }

    /// Obtient le moment de détection actuel
    fn get_detection_moment(&mut self) -> u64 {
        self.detection_counter += 1;
        self.detection_counter
    }

    /// Détecte un conflit autoritaire
    ///
    /// Appelé quand une opération locale de la Fille entre en contradiction
    /// avec une décision définitive de la Mère.
    ///
    /// Conforme au contrat section 3.1.
    pub fn detect_authoritative_conflict(
        &mut self,
        intent_id: &str,
        reason: &str,
    ) -> Conflict {
        let conflict_id = self.generate_conflict_id();
        let detected_at = self.get_detection_moment();

        println!(
            "[ConflictDetector] Conflit autoritaire détecté pour intention {}",
            intent_id
        );

        Conflict::new(
            conflict_id,
            ConflictType::Authoritative,
            intent_id.to_string(),
            reason.to_string(),
            detected_at,
        )
    }

    /// Détecte un conflit temporel
    ///
    /// Appelé quand des modifications concurrentes ont été effectuées
    /// sur la même entité dans les deux instances.
    ///
    /// Conforme au contrat section 3.2.
    pub fn detect_temporal_conflict(
        &mut self,
        intent_id: &str,
        reason: &str,
    ) -> Conflict {
        let conflict_id = self.generate_conflict_id();
        let detected_at = self.get_detection_moment();

        println!(
            "[ConflictDetector] Conflit temporel détecté pour intention {}",
            intent_id
        );

        Conflict::new(
            conflict_id,
            ConflictType::Temporal,
            intent_id.to_string(),
            reason.to_string(),
            detected_at,
        )
    }

    /// Détecte un conflit sémantique
    ///
    /// Appelé quand une opération locale viole les contraintes de cohérence
    /// établies par la Mère.
    ///
    /// Conforme au contrat section 3.3.
    pub fn detect_semantic_conflict(
        &mut self,
        intent_id: &str,
        reason: &str,
    ) -> Conflict {
        let conflict_id = self.generate_conflict_id();
        let detected_at = self.get_detection_moment();

        println!(
            "[ConflictDetector] Conflit sémantique détecté pour intention {}",
            intent_id
        );

        Conflict::new(
            conflict_id,
            ConflictType::Semantic,
            intent_id.to_string(),
            reason.to_string(),
            detected_at,
        )
    }

    /// Type un conflit à partir d'indicateurs conceptuels
    ///
    /// Retourne le type de conflit approprié basé sur les indicateurs fournis.
    /// Ne résout PAS le conflit - uniquement classification.
    pub fn type_conflict(
        &mut self,
        intent_id: &str,
        has_mother_decision: bool,
        has_concurrent_modification: bool,
        has_constraint_violation: bool,
        reason: &str,
    ) -> Option<Conflict> {
        // Priorité : Autoritaire > Temporel > Sémantique
        if has_mother_decision {
            Some(self.detect_authoritative_conflict(intent_id, reason))
        } else if has_concurrent_modification {
            Some(self.detect_temporal_conflict(intent_id, reason))
        } else if has_constraint_violation {
            Some(self.detect_semantic_conflict(intent_id, reason))
        } else {
            // Aucun conflit détecté
            None
        }
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Gestionnaire de synchronisation conceptuel
///
/// Gère la synchronisation entre Instance Fille et Instance Mère.
/// AUCUNE synchronisation réelle - structure en place pour extension future.
///
/// Conforme au contrat FONDATION.
pub struct SyncManager {
    /// Détecteur de conflits
    conflict_detector: ConflictDetector,
    /// Compteur pour générer des identifiants de synchronisation
    sync_counter: u64,
}

impl SyncManager {
    /// Crée un nouveau gestionnaire de synchronisation
    pub fn new() -> Self {
        println!("[SyncManager] Création d'un nouveau gestionnaire de synchronisation");
        Self {
            conflict_detector: ConflictDetector::new(),
            sync_counter: 0,
        }
    }

    /// Génère un identifiant unique pour une synchronisation
    pub fn generate_sync_id(&mut self) -> String {
        self.sync_counter += 1;
        format!("sync-{}", self.sync_counter)
    }

    /// Accède au détecteur de conflits
    pub fn conflict_detector(&mut self) -> &mut ConflictDetector {
        &mut self.conflict_detector
    }

    /// Tente de créer une intention de synchronisation
    ///
    /// ATTENTION : À ce stade, TOUTE synchronisation est REFUSÉE explicitement.
    /// Cette méthode existe pour la structure, mais retourne toujours une erreur.
    pub fn create_sync_intent(
        &mut self,
        source_instance_id: String,
        target_instance_id: String,
        source_type: InstanceType,
    ) -> Result<SyncIntent, KMError> {
        let sync_id = self.generate_sync_id();
        SyncIntent::new(sync_id, source_instance_id, target_instance_id, source_type)
    }

    /// Soumet une intention de synchronisation pour validation
    ///
    /// ATTENTION : À ce stade, TOUTE synchronisation est REFUSÉE explicitement.
    /// Conforme à la règle : "sync → TOUJOURS refusé".
    pub fn submit_sync_intent(&mut self, intent: &mut SyncIntent) -> Result<(), KMError> {
        // Soumission de l'intention
        intent.submit()?;

        // À ce stade, TOUTE synchronisation est REFUSÉE
        // La synchronisation réelle n'est pas implémentée
        println!(
            "[SyncManager] Synchronisation {} refusée: synchronisation non implémentée à ce stade",
            intent.sync_id()
        );

        intent.reject(
            "La synchronisation n'est pas implémentée à ce stade. Toute opération de synchronisation est explicitement refusée.".to_string()
        )?;

        Err(KMError::InsufficientPermissions {
            reason: "Les opérations de synchronisation sont toujours refusées à ce stade. La synchronisation sera implémentée dans une version ultérieure.".to_string(),
        })
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_intent_creation_daughter() {
        let result = SyncIntent::new(
            "sync-1".to_string(),
            "daughter-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Daughter,
        );
        assert!(result.is_ok());
        let intent = result.unwrap();
        assert_eq!(intent.state(), SyncIntentState::Pending);
    }

    #[test]
    fn test_sync_intent_creation_mother_rejected() {
        let result = SyncIntent::new(
            "sync-1".to_string(),
            "mother-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Mother,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_conflict_detection() {
        let mut detector = ConflictDetector::new();
        
        let conflict = detector.detect_authoritative_conflict(
            "intent-1",
            "Entité supprimée par la Mère",
        );
        assert_eq!(conflict.conflict_type(), ConflictType::Authoritative);
        
        let conflict = detector.detect_temporal_conflict(
            "intent-2",
            "Modification concurrente",
        );
        assert_eq!(conflict.conflict_type(), ConflictType::Temporal);
        
        let conflict = detector.detect_semantic_conflict(
            "intent-3",
            "Violation de contrainte",
        );
        assert_eq!(conflict.conflict_type(), ConflictType::Semantic);
    }

    #[test]
    fn test_sync_always_rejected() {
        let mut manager = SyncManager::new();
        let mut intent = manager.create_sync_intent(
            "daughter-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Daughter,
        ).unwrap();

        let result = manager.submit_sync_intent(&mut intent);
        assert!(result.is_err());
        assert_eq!(intent.state(), SyncIntentState::Rejected);
    }
}
