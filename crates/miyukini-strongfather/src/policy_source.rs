//! Source de politiques pour StrongFather
//!
//! Ce module définit le trait PolicySourceTrait et l'implémentation MemoryPolicySource
//! pour charger et valider les politiques depuis une source configurée.
//!
//! Conformité : Policy Source Contract, Boundary & Isolation Contract
//!
//! Invariants respectés :
//! - INV-POL-SOURCE : Source unique et configurée
//! - INV-SRC-1 à INV-SRC-8 : Invariants de source
//!
//! Garanties offertes :
//! - G-SRC-1 à G-SRC-6 : Garanties de source

use std::collections::{HashMap, HashSet};

use crate::error::SFError;
use crate::policy::{Policy, PolicySet};
use crate::types::{PolicyId, PolicyType};

/// Trait pour les sources de politiques
///
/// Définit l'interface pour charger des politiques depuis une source configurée.
/// Conforme à Policy Source Contract section 2.
///
/// **Caractéristiques :**
/// - Source unique : Une seule source par instance (INV-SRC-1)
/// - Configuration explicite : La source est toujours configurée (INV-SRC-2)
/// - Chargement atomique : Tout ou rien (INV-SRC-5, R-LOAD-2)
/// - Validation préalable : Les politiques sont validées avant utilisation (INV-SRC-3, R-VAL-1)
pub trait PolicySourceTrait {
    /// Charge les politiques depuis la source
    ///
    /// # Retour
    /// - `Ok(PolicySet)` : Politiques chargées et validées avec succès
    /// - `Err(SFError)` : Erreur de chargement ou de validation
    ///
    /// # Invariants
    /// - INV-SRC-5 : Chargement atomique (tout ou rien)
    /// - INV-SRC-3 : Validation préalable (R-VAL-1)
    /// - INV-SRC-6 : Isolation des évaluations (les politiques ne changent pas pendant évaluation)
    ///
    /// # Garanties
    /// - G-SRC-5 : Cohérence garantie (politiques validées avant activation)
    /// - G-SRC-6 : Complétude garantie (ensemble fermé)
    fn load(&self) -> Result<PolicySet, SFError>;
}

/// Source de politiques en mémoire
///
/// Implémentation de PolicySourceTrait pour charger des politiques depuis une collection en mémoire.
/// Conforme à Policy Source Contract section 3.1 (source déclarative statique).
///
/// **Caractéristiques :**
/// - Déclarative : Les politiques sont déclarées, pas générées (INTERD-SRC-1)
/// - Statique : Les politiques ne changent pas sans rechargement explicite
/// - Validée : Les politiques sont validées avant chargement (INV-SRC-3)
///
/// **Invariants respectés :**
/// - INV-SRC-1 : Unicité de la source (une seule instance par StrongFather)
/// - INV-SRC-2 : Configuration explicite (politiques fournies à la construction)
/// - INV-SRC-3 : Validation préalable (validation avant chargement)
/// - INV-SRC-4 : Immuabilité pendant évaluation (PolicySet est immuable)
/// - INV-SRC-5 : Chargement atomique (tout ou rien)
/// - INV-SRC-6 : Isolation des évaluations (PolicySet immuable)
/// - INV-SRC-7 : Pas d'injection (politiques fournies à la construction uniquement)
/// - INV-SRC-8 : Pas de génération (politiques déclaratives uniquement)
#[derive(Debug, Clone)]
pub struct MemoryPolicySource {
    /// Politiques à charger
    ///
    /// Les politiques sont fournies à la construction et ne peuvent pas être modifiées
    /// sans créer une nouvelle instance (INV-SRC-7, INV-SRC-8).
    policies: Vec<Policy>,
}

impl MemoryPolicySource {
    /// Crée une nouvelle source de politiques en mémoire
    ///
    /// # Arguments
    /// * `policies` - Vecteur de politiques à charger
    ///
    /// # Note
    /// Les politiques ne sont pas validées à la construction. La validation
    /// est effectuée lors de l'appel à `load()` (INV-SRC-3, R-VAL-1).
    pub fn new(policies: Vec<Policy>) -> Self {
        Self { policies }
    }

    /// Crée une nouvelle source de politiques en mémoire vide
    ///
    /// Utile pour les tests ou les cas où aucune politique n'est nécessaire.
    pub fn empty() -> Self {
        Self {
            policies: Vec::new(),
        }
    }
}

impl PolicySourceTrait for MemoryPolicySource {
    fn load(&self) -> Result<PolicySet, SFError> {
        // Validation préalable (INV-SRC-3, R-VAL-1)
        let validator = PolicyValidator::new();
        validator.validate_policies(&self.policies)?;

        // Création du PolicySet (chargement atomique, INV-SRC-5)
        let policy_set = PolicySet::from_vec(self.policies.clone());

        // Vérification de l'unicité des identifiants (VALID-STRUCT-1)
        // Cette vérification est déjà effectuée dans validate_policies,
        // mais on la refait ici pour garantir l'invariant INV-SRC-5 (atomique)
        let mut seen_ids = HashSet::new();
        for policy in policy_set.iter() {
            if !seen_ids.insert(policy.id()) {
                return Err(SFError::consistency(
                    format!("Identifiant de politique dupliqué : {}", policy.id()),
                    "Chargement des politiques",
                ));
            }
        }

        Ok(policy_set)
    }
}

/// Validateur de politiques
///
/// Valide les politiques selon les règles définies dans Policy Source Contract section 6.
///
/// **Validations effectuées :**
/// - Validation structurelle (VALID-STRUCT-1 à VALID-STRUCT-4)
/// - Validation de cohérence (VALID-COHER-1 à VALID-COHER-3)
/// - Validation de contenu (VALID-CONT-1 à VALID-CONT-3)
#[derive(Debug)]
struct PolicyValidator;

impl PolicyValidator {
    /// Crée un nouveau validateur de politiques
    pub fn new() -> Self {
        Self
    }

    /// Valide un ensemble de politiques
    ///
    /// # Arguments
    /// * `policies` - Vecteur de politiques à valider
    ///
    /// # Retour
    /// - `Ok(())` : Toutes les politiques sont valides
    /// - `Err(SFError)` : Erreur de validation détectée
    ///
    /// # Règles appliquées
    /// - R-VAL-1 : Aucune politique invalide ne peut être chargée
    /// - R-VAL-2 : La validation est effectuée avant le chargement
    /// - R-VAL-3 : Un échec de validation bloque le chargement
    pub fn validate_policies(&self, policies: &[Policy]) -> Result<(), SFError> {
        // Validation structurelle (VALID-STRUCT-*)
        for policy in policies {
            self.validate_policy_structure(policy)?;
        }

        // Validation de cohérence (VALID-COHER-*)
        self.validate_coherence(policies)?;

        // Validation de contenu (VALID-CONT-*)
        self.validate_content(policies)?;

        Ok(())
    }

    /// Valide la structure d'une politique
    ///
    /// Vérifie VALID-STRUCT-1 à VALID-STRUCT-4.
    fn validate_policy_structure(&self, policy: &Policy) -> Result<(), SFError> {
        // VALID-STRUCT-1 : Identifiant unique
        // Vérifié au niveau de l'ensemble dans validate_coherence

        // VALID-STRUCT-2 : Type valide
        // Le type est vérifié par le système de types Rust, mais on vérifie
        // que les références dans les composites sont valides
        match &policy.policy_type {
            PolicyType::Composite { policies, .. } => {
                if policies.is_empty() {
                    return Err(SFError::structural(
                        "Une politique composite doit référencer au moins une politique",
                        format!("Validation de la politique {}", policy.id()),
                    ));
                }
            }
            _ => {
                // Types simples : Permission, Constraint, Priority, Validation
                // Aucune validation supplémentaire nécessaire (vérifié par le type system)
            }
        }

        // VALID-STRUCT-3 : Composants obligatoires
        // Vérifié par la structure Policy (tous les champs sont obligatoires)

        // VALID-STRUCT-4 : Effet explicite
        // Vérifié par la structure Policy (effect est obligatoire)

        Ok(())
    }

    /// Valide la cohérence d'un ensemble de politiques
    ///
    /// Vérifie VALID-COHER-1 à VALID-COHER-3.
    fn validate_coherence(&self, policies: &[Policy]) -> Result<(), SFError> {
        // VALID-STRUCT-1 : Identifiant unique (vérifié ici pour l'ensemble)
        let mut seen_ids = HashSet::new();
        for policy in policies {
            if !seen_ids.insert(policy.id()) {
                return Err(SFError::consistency(
                    format!("Identifiant de politique dupliqué : {}", policy.id()),
                    "Validation de cohérence",
                ));
            }
        }

        // Création d'un index pour les recherches rapides
        let policy_index: HashMap<&PolicyId, &Policy> =
            policies.iter().map(|p| (p.id(), p)).collect();

        // VALID-COHER-2 : Références valides
        // Vérifie que toutes les références dans les politiques composites pointent vers des politiques existantes
        for policy in policies {
            if let PolicyType::Composite { policies: ref_policies, .. } = &policy.policy_type {
                for ref_id in ref_policies {
                    if !policy_index.contains_key(ref_id) {
                        return Err(SFError::consistency(
                            format!(
                                "Politique composite {} référence une politique inexistante : {}",
                                policy.id(),
                                ref_id
                            ),
                            "Validation de cohérence",
                        ));
                    }
                }
            }
        }

        // VALID-COHER-3 : Pas de cycle dans les composites
        // Détecte les cycles de référence dans les politiques composites
        self.detect_cycles(policies, &policy_index)?;

        // VALID-COHER-1 : Pas de contradiction directe
        // Note : La détection de contradictions est complexe et dépend du domaine.
        // Pour l'instant, on se contente de vérifier les invariants structurels.
        // Les contradictions logiques seront détectées lors de l'évaluation par le PolicyEngine.

        Ok(())
    }

    /// Détecte les cycles dans les politiques composites
    ///
    /// Utilise un algorithme de détection de cycles (DFS) pour vérifier VALID-COHER-3.
    fn detect_cycles(
        &self,
        policies: &[Policy],
        policy_index: &HashMap<&PolicyId, &Policy>,
    ) -> Result<(), SFError> {
        // État de visite pour chaque politique
        enum VisitState {
            White,  // Non visitée
            Gray,   // En cours de visite (dans le chemin actuel)
            Black,  // Visite terminée
        }

        let mut visit_state: HashMap<&PolicyId, VisitState> = policies
            .iter()
            .map(|p| (p.id(), VisitState::White))
            .collect();

        // Fonction récursive de détection de cycles
        // Utilise une durée de vie explicite pour toutes les références
        fn dfs<'a>(
            policy_id: &'a PolicyId,
            policy_index: &HashMap<&'a PolicyId, &'a Policy>,
            visit_state: &mut HashMap<&'a PolicyId, VisitState>,
        ) -> Result<(), SFError> {
            match visit_state.get(policy_id) {
                Some(VisitState::Gray) => {
                    // Cycle détecté
                    return Err(SFError::consistency(
                        format!("Cycle détecté dans les politiques composites : {}", policy_id),
                        "Validation de cohérence",
                    ));
                }
                Some(VisitState::Black) => {
                    // Déjà visitée, pas de cycle
                    return Ok(());
                }
                _ => {}
            }

            // Marquer comme en cours de visite
            visit_state.insert(policy_id, VisitState::Gray);

            // Visiter les politiques référencées
            if let Some(policy) = policy_index.get(policy_id) {
                if let PolicyType::Composite { policies: ref_policies, .. } = &policy.policy_type {
                    for ref_id in ref_policies {
                        dfs(ref_id, policy_index, visit_state)?;
                    }
                }
            }

            // Marquer comme visitée
            visit_state.insert(policy_id, VisitState::Black);
            Ok(())
        }

        // Parcourir toutes les politiques composites
        for policy in policies {
            if matches!(policy.policy_type, PolicyType::Composite { .. }) {
                dfs(policy.id(), policy_index, &mut visit_state)?;
            }
        }

        Ok(())
    }

    /// Valide le contenu des politiques
    ///
    /// Vérifie VALID-CONT-1 à VALID-CONT-3.
    ///
    /// Note : La validation de contenu est limitée à la détection de patterns évidents.
    /// Une validation complète nécessiterait une analyse sémantique plus approfondie.
    fn validate_content(&self, policies: &[Policy]) -> Result<(), SFError> {
        // Mots-clés interdits pour détecter la logique d'exécution ou métier
        let execution_keywords = [
            "execute", "exec", "run", "call", "invoke", "perform", "do_action",
            "save", "delete", "update", "create", "write", "read_file",
            "send", "receive", "fetch", "post", "get", "put", "delete",
        ];

        let business_keywords = [
            "invoice", "payment", "order", "customer", "product", "cart",
            "checkout", "billing", "shipping", "delivery", "refund",
        ];

        let temporal_keywords = [
            "timestamp", "datetime", "now()", "current_time", "get_time",
            "time()", "date()", "millis", "nanos", "epoch",
        ];

        for policy in policies {
            let rule_expr = policy.rule().expression().to_lowercase();

            // VALID-CONT-1 : Pas de logique d'exécution
            for keyword in &execution_keywords {
                if rule_expr.contains(keyword) {
                    return Err(SFError::structural(
                        format!(
                            "Politique {} contient une logique d'exécution détectée (mot-clé: {})",
                            policy.id(),
                            keyword
                        ),
                        "Validation de contenu",
                    ));
                }
            }

            // VALID-CONT-2 : Pas de logique métier spécifique
            for keyword in &business_keywords {
                if rule_expr.contains(keyword) {
                    return Err(SFError::structural(
                        format!(
                            "Politique {} contient une logique métier spécifique détectée (mot-clé: {})",
                            policy.id(),
                            keyword
                        ),
                        "Validation de contenu",
                    ));
                }
            }

            // VALID-CONT-3 : Pas de logique temporelle technique
            for keyword in &temporal_keywords {
                if rule_expr.contains(keyword) {
                    return Err(SFError::structural(
                        format!(
                            "Politique {} contient une logique temporelle technique détectée (mot-clé: {})",
                            policy.id(),
                            keyword
                        ),
                        "Validation de contenu",
                    ));
                }
            }

            // Vérifier aussi dans les critères de condition
            if let Some(criteria) = policy.condition().criteria() {
                let criteria_lower = criteria.to_lowercase();
                for keyword in &execution_keywords {
                    if criteria_lower.contains(keyword) {
                        return Err(SFError::structural(
                            format!(
                                "Condition de la politique {} contient une logique d'exécution détectée (mot-clé: {})",
                                policy.id(),
                                keyword
                            ),
                            "Validation de contenu",
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::{PolicyCondition, PolicyRule};
    use crate::types::{LogicalOperator, PolicyEffect, PolicyPriority, PolicyType};

    #[test]
    fn test_memory_policy_source_empty() {
        let source = MemoryPolicySource::empty();
        let result = source.load();
        assert!(result.is_ok());
        let policy_set = result.unwrap();
        assert!(policy_set.is_empty());
    }

    #[test]
    fn test_memory_policy_source_load_valid() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("actor.role == 'admin'"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_ok());
        let policy_set = result.unwrap();
        assert_eq!(policy_set.len(), 1);
        assert!(policy_set.contains(&"POL-001".to_string()));
    }

    #[test]
    fn test_memory_policy_source_duplicate_id() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule1"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-001", // Dupliqué
                PolicyType::Constraint,
                PolicyCondition::always(),
                PolicyRule::new("rule2"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::ConsistencyError { description, .. }) => {
                assert!(description.contains("dupliqué"));
            }
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_memory_policy_source_composite_valid_reference() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule1"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-002",
                PolicyType::Composite {
                    operator: LogicalOperator::And,
                    policies: vec!["POL-001".to_string()],
                },
                PolicyCondition::always(),
                PolicyRule::new("composite_rule"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_ok());
    }

    #[test]
    fn test_memory_policy_source_composite_invalid_reference() {
        let policies = vec![Policy::new(
            "POL-002",
            PolicyType::Composite {
                operator: LogicalOperator::And,
                policies: vec!["POL-999".to_string()], // Référence inexistante
            },
            PolicyCondition::always(),
            PolicyRule::new("composite_rule"),
            PolicyEffect::Require,
            PolicyPriority::new(50),
        )];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::ConsistencyError { description, .. }) => {
                assert!(description.contains("inexistante"));
            }
            _ => panic!("Expected ConsistencyError"),
        }
    }

    #[test]
    fn test_memory_policy_source_cycle_detection() {
        let policies = vec![
            Policy::new(
                "POL-A",
                PolicyType::Composite {
                    operator: LogicalOperator::And,
                    policies: vec!["POL-B".to_string()],
                },
                PolicyCondition::always(),
                PolicyRule::new("rule_a"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
            Policy::new(
                "POL-B",
                PolicyType::Composite {
                    operator: LogicalOperator::And,
                    policies: vec!["POL-A".to_string()], // Cycle : A -> B -> A
                },
                PolicyCondition::always(),
                PolicyRule::new("rule_b"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::ConsistencyError { description, .. }) => {
                assert!(description.contains("Cycle"));
            }
            _ => panic!("Expected ConsistencyError for cycle detection"),
        }
    }

    #[test]
    fn test_memory_policy_source_execution_keyword_detection() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("actor.execute_action()"), // Mot-clé d'exécution
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::StructuralError { description, .. }) => {
                assert!(description.contains("logique d'exécution"));
            }
            _ => panic!("Expected StructuralError for execution keyword"),
        }
    }

    #[test]
    fn test_memory_policy_source_business_keyword_detection() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("order.status == 'pending'"), // Mot-clé métier
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::StructuralError { description, .. }) => {
                assert!(description.contains("logique métier"));
            }
            _ => panic!("Expected StructuralError for business keyword"),
        }
    }

    #[test]
    fn test_memory_policy_source_temporal_keyword_detection() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("timestamp > 0"), // Mot-clé temporel
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::StructuralError { description, .. }) => {
                assert!(description.contains("logique temporelle"));
            }
            _ => panic!("Expected StructuralError for temporal keyword"),
        }
    }

    #[test]
    fn test_memory_policy_source_composite_empty_policies() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Composite {
                operator: LogicalOperator::And,
                policies: vec![], // Vide
            },
            PolicyCondition::always(),
            PolicyRule::new("rule"),
            PolicyEffect::Require,
            PolicyPriority::new(50),
        )];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_err());
        match result {
            Err(SFError::StructuralError { description, .. }) => {
                assert!(description.contains("au moins une politique"));
            }
            _ => panic!("Expected StructuralError for empty composite"),
        }
    }

    #[test]
    fn test_memory_policy_source_multiple_valid_policies() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("actor.role == 'admin'"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-002",
                PolicyType::Constraint,
                PolicyCondition::when("action_type == Creation"),
                PolicyRule::new("resource.owner == actor.id"),
                PolicyEffect::Require,
                PolicyPriority::new(75),
            ),
            Policy::new(
                "POL-003",
                PolicyType::Priority,
                PolicyCondition::always(),
                PolicyRule::new("actor.priority > 50"),
                PolicyEffect::Prioritize { level: 150 },
                PolicyPriority::new(50),
            ),
        ];

        let source = MemoryPolicySource::new(policies);
        let result = source.load();
        assert!(result.is_ok());
        let policy_set = result.unwrap();
        assert_eq!(policy_set.len(), 3);
    }
}
