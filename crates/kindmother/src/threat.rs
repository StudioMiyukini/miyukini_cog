//! Threat Model Enforcement - Détection des menaces
//!
//! Implémente la détection des menaces conforme au contrat FONDATION :
//! "KindMother — Threat Model & Attack Surface Contract"
//!
//! CARACTÉRISTIQUES :
//! - Détection de bypass CoreDataAPI
//! - Détection de replay d'intention
//! - Détection de resoumission
//! - Détection de saturation volontaire
//! - AUCUNE auto-réparation
//! - AUCUNE escalade implicite
//! - Dégradation contrôlée (Degraded, Quarantined)

use crate::errors::KMError;
use std::collections::{HashMap, HashSet};

/// Type de menace détectée
///
/// Conforme au contrat FONDATION section 4 "Types d'attaques reconnus".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatType {
    /// Tentative de contourner la CoreDataAPI
    /// Gravité : CRITIQUE
    Bypass,
    /// Tentative de réutiliser une intention déjà traitée
    /// Gravité : ÉLEVÉE
    Replay,
    /// Tentative de resoumettre une intention rejetée
    /// Gravité : ÉLEVÉE
    Resubmission,
    /// Saturation volontaire du système
    /// Gravité : MOYENNE
    Saturation,
    /// Brute-force contextuel (exploration systématique)
    /// Gravité : MOYENNE
    BruteForce,
    /// Injection d'intention malveillante
    /// Gravité : ÉLEVÉE
    Injection,
}

impl ThreatType {
    /// Retourne une représentation textuelle du type de menace
    pub fn to_string(&self) -> &'static str {
        match self {
            ThreatType::Bypass => "BYPASS",
            ThreatType::Replay => "REPLAY",
            ThreatType::Resubmission => "RESOUMISSION",
            ThreatType::Saturation => "SATURATION",
            ThreatType::BruteForce => "BRUTE_FORCE",
            ThreatType::Injection => "INJECTION",
        }
    }

    /// Retourne la gravité de la menace
    pub fn severity(&self) -> ThreatSeverity {
        match self {
            ThreatType::Bypass => ThreatSeverity::Critical,
            ThreatType::Replay => ThreatSeverity::High,
            ThreatType::Resubmission => ThreatSeverity::High,
            ThreatType::Saturation => ThreatSeverity::Medium,
            ThreatType::BruteForce => ThreatSeverity::Medium,
            ThreatType::Injection => ThreatSeverity::High,
        }
    }

    /// Retourne une description de la menace
    pub fn description(&self) -> &'static str {
        match self {
            ThreatType::Bypass => "Tentative d'accéder aux données sans passer par la CoreDataAPI",
            ThreatType::Replay => "Tentative de réutiliser une intention déjà traitée",
            ThreatType::Resubmission => "Tentative de resoumettre une intention rejetée",
            ThreatType::Saturation => "Volume d'opérations excessif pour perturber le système",
            ThreatType::BruteForce => "Exploration systématique des contextes pour trouver des failles",
            ThreatType::Injection => "Tentative d'injecter une intention malveillante",
        }
    }
}

/// Gravité d'une menace
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    /// Gravité faible
    Low,
    /// Gravité moyenne
    Medium,
    /// Gravité élevée
    High,
    /// Gravité critique
    Critical,
}

impl ThreatSeverity {
    /// Retourne une représentation textuelle
    pub fn to_string(&self) -> &'static str {
        match self {
            ThreatSeverity::Low => "FAIBLE",
            ThreatSeverity::Medium => "MOYENNE",
            ThreatSeverity::High => "ÉLEVÉE",
            ThreatSeverity::Critical => "CRITIQUE",
        }
    }
}

/// Menace détectée
///
/// Représente une menace détectée avec son contexte complet.
#[derive(Debug, Clone)]
pub struct DetectedThreat {
    /// Identifiant unique de la menace
    threat_id: String,
    /// Type de menace
    threat_type: ThreatType,
    /// Identité de la source (appelant)
    source_identity: String,
    /// Raison détaillée de la détection
    reason: String,
    /// Moment de détection (conceptuel, compteur)
    detected_at: u64,
    /// Identifiant de l'intention concernée (si applicable)
    intent_id: Option<String>,
}

impl DetectedThreat {
    /// Crée une nouvelle menace détectée
    pub fn new(
        threat_id: String,
        threat_type: ThreatType,
        source_identity: String,
        reason: String,
        detected_at: u64,
        intent_id: Option<String>,
    ) -> Self {
        println!(
            "[DetectedThreat] Menace {} détectée: type={}, source={}, raison={}",
            threat_id,
            threat_type.to_string(),
            source_identity,
            reason
        );
        Self {
            threat_id,
            threat_type,
            source_identity,
            reason,
            detected_at,
            intent_id,
        }
    }

    /// Obtient l'identifiant de la menace
    pub fn threat_id(&self) -> &str {
        &self.threat_id
    }

    /// Obtient le type de menace
    pub fn threat_type(&self) -> ThreatType {
        self.threat_type
    }

    /// Obtient la gravité
    pub fn severity(&self) -> ThreatSeverity {
        self.threat_type.severity()
    }

    /// Obtient l'identité de la source
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    /// Obtient la raison
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Obtient le moment de détection
    pub fn detected_at(&self) -> u64 {
        self.detected_at
    }

    /// Obtient l'identifiant de l'intention (si applicable)
    pub fn intent_id(&self) -> Option<&str> {
        self.intent_id.as_deref()
    }
}

/// Configuration du détecteur de menaces
#[derive(Debug, Clone)]
pub struct ThreatDetectorConfig {
    /// Seuil de saturation (nombre d'appels par période)
    pub saturation_threshold: u32,
    /// Seuil de brute-force (nombre de rejets consécutifs)
    pub brute_force_threshold: u32,
    /// Période de comptage (conceptuelle)
    pub counting_period: u64,
}

impl Default for ThreatDetectorConfig {
    fn default() -> Self {
        Self {
            saturation_threshold: 100,  // 100 appels par période
            brute_force_threshold: 10,  // 10 rejets consécutifs
            counting_period: 60,        // période conceptuelle
        }
    }
}

/// Détecteur de menaces
///
/// Détecte les différents types de menaces selon le contrat FONDATION.
/// AUCUNE auto-réparation - uniquement détection.
///
/// Conforme au contrat FONDATION "Threat Model & Attack Surface Contract".
pub struct ThreatDetector {
    /// Configuration
    config: ThreatDetectorConfig,
    /// Compteur pour générer des identifiants de menace
    threat_counter: u64,
    /// Compteur pour le moment de détection
    detection_counter: u64,
    /// Intentions déjà traitées (pour détection replay)
    processed_intents: HashSet<String>,
    /// Intentions rejetées (pour détection resoumission)
    rejected_intents: HashSet<String>,
    /// Compteur d'appels par source (pour détection saturation)
    call_counts: HashMap<String, u32>,
    /// Compteur de rejets consécutifs par source (pour détection brute-force)
    consecutive_rejections: HashMap<String, u32>,
    /// Menaces détectées
    detected_threats: Vec<DetectedThreat>,
}

impl ThreatDetector {
    /// Crée un nouveau détecteur de menaces
    pub fn new() -> Self {
        println!("[ThreatDetector] Création d'un nouveau détecteur de menaces");
        Self {
            config: ThreatDetectorConfig::default(),
            threat_counter: 0,
            detection_counter: 0,
            processed_intents: HashSet::new(),
            rejected_intents: HashSet::new(),
            call_counts: HashMap::new(),
            consecutive_rejections: HashMap::new(),
            detected_threats: Vec::new(),
        }
    }

    /// Crée un détecteur avec configuration personnalisée
    pub fn with_config(config: ThreatDetectorConfig) -> Self {
        println!("[ThreatDetector] Création d'un détecteur avec configuration personnalisée");
        Self {
            config,
            threat_counter: 0,
            detection_counter: 0,
            processed_intents: HashSet::new(),
            rejected_intents: HashSet::new(),
            call_counts: HashMap::new(),
            consecutive_rejections: HashMap::new(),
            detected_threats: Vec::new(),
        }
    }

    /// Génère un identifiant unique pour une menace
    fn generate_threat_id(&mut self) -> String {
        self.threat_counter += 1;
        format!("threat-{}", self.threat_counter)
    }

    /// Obtient le moment de détection actuel
    fn get_detection_moment(&mut self) -> u64 {
        self.detection_counter += 1;
        self.detection_counter
    }

    /// Enregistre une menace détectée
    fn record_threat(&mut self, threat: DetectedThreat) {
        self.detected_threats.push(threat);
    }

    /// Détecte une tentative de bypass de la CoreDataAPI
    ///
    /// Cette méthode est appelée quand on détecte qu'une opération
    /// n'est pas passée par la CoreDataAPI.
    ///
    /// Conforme au contrat section 4.1.
    pub fn detect_bypass_attempt(
        &mut self,
        source_identity: &str,
        reason: &str,
    ) -> DetectedThreat {
        let threat_id = self.generate_threat_id();
        let detected_at = self.get_detection_moment();

        println!(
            "[ThreatDetector] Tentative de BYPASS détectée de la source {}",
            source_identity
        );

        let threat = DetectedThreat::new(
            threat_id,
            ThreatType::Bypass,
            source_identity.to_string(),
            reason.to_string(),
            detected_at,
            None,
        );

        self.record_threat(threat.clone());
        threat
    }

    /// Détecte une tentative de replay d'intention
    ///
    /// Vérifie si une intention a déjà été traitée.
    ///
    /// Conforme au contrat section 4.4.
    pub fn detect_replay(
        &mut self,
        intent_id: &str,
        source_identity: &str,
    ) -> Option<DetectedThreat> {
        if self.processed_intents.contains(intent_id) {
            let threat_id = self.generate_threat_id();
            let detected_at = self.get_detection_moment();

            println!(
                "[ThreatDetector] Tentative de REPLAY détectée pour intention {}",
                intent_id
            );

            let threat = DetectedThreat::new(
                threat_id,
                ThreatType::Replay,
                source_identity.to_string(),
                format!("L'intention {} a déjà été traitée. Tentative de replay détectée.", intent_id),
                detected_at,
                Some(intent_id.to_string()),
            );

            self.record_threat(threat.clone());
            Some(threat)
        } else {
            None
        }
    }

    /// Marque une intention comme traitée
    pub fn mark_intent_processed(&mut self, intent_id: &str) {
        self.processed_intents.insert(intent_id.to_string());
    }

    /// Détecte une tentative de resoumission d'intention rejetée
    ///
    /// Vérifie si une intention a déjà été rejetée.
    ///
    /// Conforme au contrat section 4.4.
    pub fn detect_resubmission(
        &mut self,
        intent_id: &str,
        source_identity: &str,
    ) -> Option<DetectedThreat> {
        if self.rejected_intents.contains(intent_id) {
            let threat_id = self.generate_threat_id();
            let detected_at = self.get_detection_moment();

            println!(
                "[ThreatDetector] Tentative de RESOUMISSION détectée pour intention {}",
                intent_id
            );

            let threat = DetectedThreat::new(
                threat_id,
                ThreatType::Resubmission,
                source_identity.to_string(),
                format!("L'intention {} a déjà été rejetée. Tentative de resoumission détectée.", intent_id),
                detected_at,
                Some(intent_id.to_string()),
            );

            self.record_threat(threat.clone());
            Some(threat)
        } else {
            None
        }
    }

    /// Marque une intention comme rejetée
    pub fn mark_intent_rejected(&mut self, intent_id: &str) {
        self.rejected_intents.insert(intent_id.to_string());
    }

    /// Détecte une saturation volontaire
    ///
    /// Vérifie si le nombre d'appels d'une source dépasse le seuil.
    ///
    /// Conforme au contrat section 4.6.
    pub fn detect_saturation(
        &mut self,
        source_identity: &str,
    ) -> Option<DetectedThreat> {
        // Incrémenter et copier le compte pour éviter le borrow multiple
        let count = {
            let entry = self.call_counts.entry(source_identity.to_string()).or_insert(0);
            *entry += 1;
            *entry
        };

        let threshold = self.config.saturation_threshold;
        if count > threshold {
            let threat_id = self.generate_threat_id();
            let detected_at = self.get_detection_moment();

            println!(
                "[ThreatDetector] SATURATION détectée de la source {} ({} appels)",
                source_identity, count
            );

            let threat = DetectedThreat::new(
                threat_id,
                ThreatType::Saturation,
                source_identity.to_string(),
                format!(
                    "La source {} a effectué {} appels, dépassant le seuil de {}.",
                    source_identity, count, threshold
                ),
                detected_at,
                None,
            );

            self.record_threat(threat.clone());
            Some(threat)
        } else {
            None
        }
    }

    /// Enregistre un rejet et détecte un brute-force potentiel
    ///
    /// Vérifie si le nombre de rejets consécutifs dépasse le seuil.
    ///
    /// Conforme au contrat section 4.5.
    pub fn record_rejection_and_detect_brute_force(
        &mut self,
        source_identity: &str,
    ) -> Option<DetectedThreat> {
        // Incrémenter et copier le compte pour éviter le borrow multiple
        let count = {
            let entry = self.consecutive_rejections.entry(source_identity.to_string()).or_insert(0);
            *entry += 1;
            *entry
        };

        let threshold = self.config.brute_force_threshold;
        if count > threshold {
            let threat_id = self.generate_threat_id();
            let detected_at = self.get_detection_moment();

            println!(
                "[ThreatDetector] BRUTE_FORCE potentiel détecté de la source {} ({} rejets consécutifs)",
                source_identity, count
            );

            let threat = DetectedThreat::new(
                threat_id,
                ThreatType::BruteForce,
                source_identity.to_string(),
                format!(
                    "La source {} a eu {} rejets consécutifs, dépassant le seuil de {}. Pattern de brute-force suspecté.",
                    source_identity, count, threshold
                ),
                detected_at,
                None,
            );

            self.record_threat(threat.clone());
            Some(threat)
        } else {
            None
        }
    }

    /// Réinitialise le compteur de rejets consécutifs pour une source
    /// (appelé après une opération réussie)
    pub fn reset_rejection_count(&mut self, source_identity: &str) {
        self.consecutive_rejections.insert(source_identity.to_string(), 0);
    }

    /// Détecte une tentative d'injection
    ///
    /// Vérifie si une intention contient des patterns suspects.
    /// Cette méthode est conceptuelle - les patterns réels dépendent du domaine.
    ///
    /// Conforme au contrat section 4.2.
    pub fn detect_injection(
        &mut self,
        intent_id: &str,
        operation_type: &str,
        source_identity: &str,
    ) -> Option<DetectedThreat> {
        // Détection conceptuelle de patterns suspects
        // Dans une implémentation réelle, cela dépendrait du domaine
        let suspicious_patterns = ["<script>", "DROP TABLE", "; --", "{{", "}}"];
        
        for pattern in suspicious_patterns {
            if operation_type.contains(pattern) {
                let threat_id = self.generate_threat_id();
                let detected_at = self.get_detection_moment();

                println!(
                    "[ThreatDetector] Tentative d'INJECTION détectée dans l'intention {}",
                    intent_id
                );

                let threat = DetectedThreat::new(
                    threat_id,
                    ThreatType::Injection,
                    source_identity.to_string(),
                    format!(
                        "Pattern suspect '{}' détecté dans l'opération '{}' de l'intention {}.",
                        pattern, operation_type, intent_id
                    ),
                    detected_at,
                    Some(intent_id.to_string()),
                );

                self.record_threat(threat.clone());
                return Some(threat);
            }
        }

        None
    }

    /// Vérifie toutes les menaces pour une intention
    ///
    /// Retourne la première menace détectée, ou None si aucune.
    pub fn check_all_threats(
        &mut self,
        intent_id: &str,
        operation_type: &str,
        source_identity: &str,
    ) -> Option<DetectedThreat> {
        // 1. Vérifier le replay
        if let Some(threat) = self.detect_replay(intent_id, source_identity) {
            return Some(threat);
        }

        // 2. Vérifier la resoumission
        if let Some(threat) = self.detect_resubmission(intent_id, source_identity) {
            return Some(threat);
        }

        // 3. Vérifier l'injection
        if let Some(threat) = self.detect_injection(intent_id, operation_type, source_identity) {
            return Some(threat);
        }

        // 4. Vérifier la saturation
        if let Some(threat) = self.detect_saturation(source_identity) {
            return Some(threat);
        }

        None
    }

    /// Obtient le nombre de menaces détectées
    pub fn threat_count(&self) -> usize {
        self.detected_threats.len()
    }

    /// Obtient toutes les menaces détectées
    pub fn detected_threats(&self) -> &[DetectedThreat] {
        &self.detected_threats
    }

    /// Obtient les menaces d'un type spécifique
    pub fn threats_of_type(&self, threat_type: ThreatType) -> Vec<&DetectedThreat> {
        self.detected_threats
            .iter()
            .filter(|t| t.threat_type() == threat_type)
            .collect()
    }

    /// Obtient les menaces d'une gravité minimale
    pub fn threats_with_min_severity(&self, min_severity: ThreatSeverity) -> Vec<&DetectedThreat> {
        self.detected_threats
            .iter()
            .filter(|t| t.severity() >= min_severity)
            .collect()
    }

    /// Réinitialise les compteurs périodiques (pour simulation de passage du temps)
    pub fn reset_periodic_counters(&mut self) {
        self.call_counts.clear();
        // Note: on ne réinitialise PAS consecutive_rejections car c'est
        // réinitialisé par opération réussie, pas par période
    }

    /// Convertit une menace en erreur KMError
    pub fn threat_to_error(threat: &DetectedThreat) -> KMError {
        match threat.threat_type() {
            ThreatType::Bypass => KMError::BypassAttempt {
                reason: threat.reason().to_string(),
            },
            ThreatType::Replay | ThreatType::Resubmission => KMError::IllegalOperation {
                reason: threat.reason().to_string(),
            },
            ThreatType::Saturation => KMError::ExcessiveLoad {
                reason: threat.reason().to_string(),
            },
            ThreatType::BruteForce => KMError::BypassAttempt {
                reason: threat.reason().to_string(),
            },
            ThreatType::Injection => KMError::ConsistencyViolation {
                reason: threat.reason().to_string(),
            },
        }
    }
}

impl Default for ThreatDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_replay() {
        let mut detector = ThreatDetector::new();
        
        // Première fois - pas de replay
        assert!(detector.detect_replay("intent-1", "user-1").is_none());
        
        // Marquer comme traité
        detector.mark_intent_processed("intent-1");
        
        // Deuxième fois - replay détecté
        let threat = detector.detect_replay("intent-1", "user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::Replay);
    }

    #[test]
    fn test_detect_resubmission() {
        let mut detector = ThreatDetector::new();
        
        // Première fois - pas de resoumission
        assert!(detector.detect_resubmission("intent-1", "user-1").is_none());
        
        // Marquer comme rejeté
        detector.mark_intent_rejected("intent-1");
        
        // Deuxième fois - resoumission détectée
        let threat = detector.detect_resubmission("intent-1", "user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::Resubmission);
    }

    #[test]
    fn test_detect_saturation() {
        let config = ThreatDetectorConfig {
            saturation_threshold: 5,
            brute_force_threshold: 10,
            counting_period: 60,
        };
        let mut detector = ThreatDetector::with_config(config);
        
        // 5 appels - pas de saturation
        for _ in 0..5 {
            assert!(detector.detect_saturation("user-1").is_none());
        }
        
        // 6ème appel - saturation détectée
        let threat = detector.detect_saturation("user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::Saturation);
    }

    #[test]
    fn test_detect_brute_force() {
        let config = ThreatDetectorConfig {
            saturation_threshold: 100,
            brute_force_threshold: 3,
            counting_period: 60,
        };
        let mut detector = ThreatDetector::with_config(config);
        
        // 3 rejets - pas de brute-force
        for _ in 0..3 {
            assert!(detector.record_rejection_and_detect_brute_force("user-1").is_none());
        }
        
        // 4ème rejet - brute-force détecté
        let threat = detector.record_rejection_and_detect_brute_force("user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::BruteForce);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(ThreatSeverity::Low < ThreatSeverity::Medium);
        assert!(ThreatSeverity::Medium < ThreatSeverity::High);
        assert!(ThreatSeverity::High < ThreatSeverity::Critical);
    }
}
