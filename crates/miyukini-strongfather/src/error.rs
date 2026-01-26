//! Types d'erreurs de StrongFather
//!
//! Ce module définit les catégories d'erreurs que StrongFather peut retourner.
//! Conformément au contrat Error & Rejection Model, les erreurs représentent
//! des dysfonctionnements internes qui empêchent la production d'une décision.
//!
//! **Distinction fondamentale :**
//! - **Erreur** : Dysfonctionnement interne, pas de décision produite (INV-ERR-1, INV-ERR-2)
//! - **Rejet** : Résultat normal d'évaluation, décision produite (REFUSÉE, AMBIGUË, DIFFÉRÉE)
//!
//! Les rejets ne sont PAS des erreurs. Ils sont représentés par des décisions
//! dans le module `decision.rs`.

use std::fmt;

/// Erreur générique de StrongFather
///
/// Conformément au contrat Error & Rejection Model (section 3), StrongFather
/// définit trois catégories d'erreurs :
///
/// 1. **StructuralError** : Erreurs causées par une incohérence ou une malformation
///    dans la structure des données internes de StrongFather.
///    Exemples : Politique malformée, règle de composition incohérente,
///    référence circulaire dans les politiques.
///
/// 2. **ConsistencyError** : Erreurs causées par une violation des invariants internes
///    de StrongFather.
///    Exemples : Violation d'un invariant de politique, incohérence dans l'état du moteur,
///    contradiction détectée dans les règles.
///
/// 3. **ResourceError** : Erreurs causées par l'indisponibilité de ressources nécessaires
///    à l'évaluation.
///    Exemples : Politiques non disponibles, contexte d'évaluation incomplet côté moteur,
///    capacité d'évaluation dépassée.
///
/// **Invariants respectés :**
/// - INV-ERR-1 : Distinction erreur/rejet (absolue)
/// - INV-ERR-2 : Erreur sans décision (mutuellement exclusif)
/// - INV-ERR-6 : Pas d'effet de bord sur erreur
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SFError {
    /// Erreur de structure
    ///
    /// Erreur causée par une incohérence ou une malformation dans la structure
    /// des données internes de StrongFather.
    ///
    /// **Gravité :** Critique
    /// **Conséquence :** Arrêt de l'évaluation, signalement d'erreur interne
    StructuralError {
        /// Description de l'erreur (obligatoire selon le contrat section 5.1)
        description: String,
        /// Contexte dans lequel l'erreur s'est produite (obligatoire selon le contrat section 5.1)
        context: String,
        /// Cause racine identifiée (optionnel selon le contrat section 5.2)
        root_cause: Option<String>,
        /// Recommandation pour résoudre l'erreur (optionnel selon le contrat section 5.2)
        recommendation: Option<String>,
    },
    /// Erreur de cohérence
    ///
    /// Erreur causée par une violation des invariants internes de StrongFather.
    ///
    /// **Gravité :** Critique
    /// **Conséquence :** Arrêt de l'évaluation, signalement d'erreur de cohérence
    ConsistencyError {
        /// Description de l'erreur (obligatoire selon le contrat section 5.1)
        description: String,
        /// Contexte dans lequel l'erreur s'est produite (obligatoire selon le contrat section 5.1)
        context: String,
        /// Cause racine identifiée (optionnel selon le contrat section 5.2)
        root_cause: Option<String>,
        /// Recommandation pour résoudre l'erreur (optionnel selon le contrat section 5.2)
        recommendation: Option<String>,
    },
    /// Erreur de ressource
    ///
    /// Erreur causée par l'indisponibilité de ressources nécessaires à l'évaluation.
    ///
    /// **Gravité :** Haute
    /// **Conséquence :** Échec de l'évaluation, possibilité de réessai
    ResourceError {
        /// Description de l'erreur (obligatoire selon le contrat section 5.1)
        description: String,
        /// Contexte dans lequel l'erreur s'est produite (obligatoire selon le contrat section 5.1)
        context: String,
        /// Cause racine identifiée (optionnel selon le contrat section 5.2)
        root_cause: Option<String>,
        /// Recommandation pour résoudre l'erreur (optionnel selon le contrat section 5.2)
        recommendation: Option<String>,
    },
}

impl SFError {
    /// Crée une erreur de structure
    pub fn structural(
        description: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        SFError::StructuralError {
            description: description.into(),
            context: context.into(),
            root_cause: None,
            recommendation: None,
        }
    }

    /// Crée une erreur de structure avec cause racine et recommandation
    pub fn structural_with_details(
        description: impl Into<String>,
        context: impl Into<String>,
        root_cause: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        SFError::StructuralError {
            description: description.into(),
            context: context.into(),
            root_cause: Some(root_cause.into()),
            recommendation: Some(recommendation.into()),
        }
    }

    /// Crée une erreur de cohérence
    pub fn consistency(
        description: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        SFError::ConsistencyError {
            description: description.into(),
            context: context.into(),
            root_cause: None,
            recommendation: None,
        }
    }

    /// Crée une erreur de cohérence avec cause racine et recommandation
    pub fn consistency_with_details(
        description: impl Into<String>,
        context: impl Into<String>,
        root_cause: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        SFError::ConsistencyError {
            description: description.into(),
            context: context.into(),
            root_cause: Some(root_cause.into()),
            recommendation: Some(recommendation.into()),
        }
    }

    /// Crée une erreur de ressource
    pub fn resource(
        description: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        SFError::ResourceError {
            description: description.into(),
            context: context.into(),
            root_cause: None,
            recommendation: None,
        }
    }

    /// Crée une erreur de ressource avec cause racine et recommandation
    pub fn resource_with_details(
        description: impl Into<String>,
        context: impl Into<String>,
        root_cause: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        SFError::ResourceError {
            description: description.into(),
            context: context.into(),
            root_cause: Some(root_cause.into()),
            recommendation: Some(recommendation.into()),
        }
    }

    /// Retourne la catégorie de l'erreur sous forme de chaîne
    pub fn category(&self) -> &'static str {
        match self {
            SFError::StructuralError { .. } => "STRUCTURE",
            SFError::ConsistencyError { .. } => "COHÉRENCE",
            SFError::ResourceError { .. } => "RESSOURCE",
        }
    }

    /// Retourne la description de l'erreur
    pub fn description(&self) -> &str {
        match self {
            SFError::StructuralError { description, .. }
            | SFError::ConsistencyError { description, .. }
            | SFError::ResourceError { description, .. } => description,
        }
    }

    /// Retourne le contexte de l'erreur
    pub fn context(&self) -> &str {
        match self {
            SFError::StructuralError { context, .. }
            | SFError::ConsistencyError { context, .. }
            | SFError::ResourceError { context, .. } => context,
        }
    }
}

impl fmt::Display for SFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SFError::StructuralError {
                description,
                context,
                root_cause,
                recommendation,
            } => {
                write!(f, "Erreur de structure: {}", description)?;
                write!(f, " (contexte: {})", context)?;
                if let Some(cause) = root_cause {
                    write!(f, " [cause: {}]", cause)?;
                }
                if let Some(rec) = recommendation {
                    write!(f, " [recommandation: {}]", rec)?;
                }
                Ok(())
            }
            SFError::ConsistencyError {
                description,
                context,
                root_cause,
                recommendation,
            } => {
                write!(f, "Erreur de cohérence: {}", description)?;
                write!(f, " (contexte: {})", context)?;
                if let Some(cause) = root_cause {
                    write!(f, " [cause: {}]", cause)?;
                }
                if let Some(rec) = recommendation {
                    write!(f, " [recommandation: {}]", rec)?;
                }
                Ok(())
            }
            SFError::ResourceError {
                description,
                context,
                root_cause,
                recommendation,
            } => {
                write!(f, "Erreur de ressource: {}", description)?;
                write!(f, " (contexte: {})", context)?;
                if let Some(cause) = root_cause {
                    write!(f, " [cause: {}]", cause)?;
                }
                if let Some(rec) = recommendation {
                    write!(f, " [recommandation: {}]", rec)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for SFError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structural_error_creation() {
        let err = SFError::structural(
            "Politique malformée détectée",
            "Chargement des politiques",
        );
        assert_eq!(err.category(), "STRUCTURE");
        assert_eq!(err.description(), "Politique malformée détectée");
        assert_eq!(err.context(), "Chargement des politiques");
    }

    #[test]
    fn test_consistency_error_creation() {
        let err = SFError::consistency(
            "Violation d'invariant détectée",
            "Évaluation de politique",
        );
        assert_eq!(err.category(), "COHÉRENCE");
        assert_eq!(err.description(), "Violation d'invariant détectée");
        assert_eq!(err.context(), "Évaluation de politique");
    }

    #[test]
    fn test_resource_error_creation() {
        let err = SFError::resource(
            "Politiques non disponibles",
            "Chargement depuis la source",
        );
        assert_eq!(err.category(), "RESSOURCE");
        assert_eq!(err.description(), "Politiques non disponibles");
        assert_eq!(err.context(), "Chargement depuis la source");
    }

    #[test]
    fn test_error_with_details() {
        let err = SFError::structural_with_details(
            "Référence circulaire dans les politiques",
            "Validation des politiques",
            "Politique A référence B qui référence A",
            "Vérifier les dépendances entre politiques",
        );
        assert_eq!(err.category(), "STRUCTURE");
        match &err {
            SFError::StructuralError {
                root_cause,
                recommendation,
                ..
            } => {
                assert!(root_cause.is_some());
                assert!(recommendation.is_some());
            }
            _ => panic!("Expected StructuralError"),
        }
    }

    #[test]
    fn test_error_display() {
        let err = SFError::structural(
            "Politique malformée",
            "Chargement",
        );
        let display = format!("{}", err);
        assert!(display.contains("Erreur de structure"));
        assert!(display.contains("Politique malformée"));
        assert!(display.contains("Chargement"));
    }

    #[test]
    fn test_error_display_with_details() {
        let err = SFError::resource_with_details(
            "Politiques non disponibles",
            "Chargement",
            "Source de politiques inaccessible",
            "Vérifier la connectivité",
        );
        let display = format!("{}", err);
        assert!(display.contains("Erreur de ressource"));
        assert!(display.contains("Politiques non disponibles"));
        assert!(display.contains("Source de politiques inaccessible"));
        assert!(display.contains("Vérifier la connectivité"));
    }

    #[test]
    fn test_error_clone() {
        let err1 = SFError::consistency(
            "Test",
            "Context",
        );
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_partial_eq() {
        let err1 = SFError::structural("Test", "Context");
        let err2 = SFError::structural("Test", "Context");
        let err3 = SFError::structural("Different", "Context");
        
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_categories() {
        assert_eq!(
            SFError::structural("", "").category(),
            "STRUCTURE"
        );
        assert_eq!(
            SFError::consistency("", "").category(),
            "COHÉRENCE"
        );
        assert_eq!(
            SFError::resource("", "").category(),
            "RESSOURCE"
        );
    }
}
