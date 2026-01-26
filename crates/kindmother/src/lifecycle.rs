//! Write Intent Lifecycle - Gestion du cycle de vie des Write Intent
//!
//! Implémente le cycle de vie complet conforme au contrat FONDATION :
//! "KindMother — Write Intent Lifecycle Contract"
//!
//! États : CRÉÉE → EN_VALIDATION → [ACCEPTÉE | REJETÉE] → [APPLIQUÉE] → ARCHIVÉE

use crate::errors::KMError;

/// État d'une Write Intent dans son cycle de vie
///
/// Conforme au contrat FONDATION section 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteIntentState {
    /// État initial après création par un adaptateur
    Created,
    /// État pendant la traversée des Runtime Boundaries
    InValidation,
    /// État après validation réussie (toutes les boundaries passées)
    Accepted,
    /// État terminal : validation échouée
    Rejected,
    /// État après application effective (conceptuel, no-op en mémoire)
    Applied,
    /// État terminal définitif : intention archivée
    Archived,
}

impl WriteIntentState {
    /// Vérifie si l'état est terminal (aucune transition possible)
    pub fn is_terminal(&self) -> bool {
        matches!(self, WriteIntentState::Rejected | WriteIntentState::Archived)
    }

    /// Vérifie si l'état permet l'application
    pub fn can_apply(&self) -> bool {
        matches!(self, WriteIntentState::Accepted)
    }

    /// Vérifie si l'état permet l'archivage
    pub fn can_archive(&self) -> bool {
        matches!(
            self,
            WriteIntentState::Rejected | WriteIntentState::Applied
        )
    }

    /// Retourne une représentation textuelle de l'état
    pub fn to_string(&self) -> &'static str {
        match self {
            WriteIntentState::Created => "CRÉÉE",
            WriteIntentState::InValidation => "EN_VALIDATION",
            WriteIntentState::Accepted => "ACCEPTÉE",
            WriteIntentState::Rejected => "REJETÉE",
            WriteIntentState::Applied => "APPLIQUÉE",
            WriteIntentState::Archived => "ARCHIVÉE",
        }
    }
}

/// Gestionnaire du cycle de vie d'une Write Intent
///
/// Gère les transitions d'état conformément au contrat FONDATION.
pub struct WriteIntentLifecycle;

impl WriteIntentLifecycle {
    /// Tente une transition d'état
    ///
    /// Rejette explicitement toute transition invalide conformément au contrat.
    /// Conforme aux règles de transition du contrat section 3.
    pub fn transition(
        from: WriteIntentState,
        to: WriteIntentState,
        intent_id: &str,
    ) -> Result<(), KMError> {
        // Log conceptuel de la tentative de transition
        println!(
            "[WriteIntentLifecycle] Tentative de transition pour intent {}: {} → {}",
            intent_id,
            from.to_string(),
            to.to_string()
        );

        // Vérification des transitions autorisées
        let valid = match (from, to) {
            // CRÉÉE → EN_VALIDATION (soumission pour validation)
            (WriteIntentState::Created, WriteIntentState::InValidation) => true,
            // EN_VALIDATION → ACCEPTÉE (toutes les validations réussies)
            (WriteIntentState::InValidation, WriteIntentState::Accepted) => true,
            // EN_VALIDATION → REJETÉE (une validation échoue)
            (WriteIntentState::InValidation, WriteIntentState::Rejected) => true,
            // ACCEPTÉE → APPLIQUÉE (application effective)
            (WriteIntentState::Accepted, WriteIntentState::Applied) => true,
            // REJETÉE → ARCHIVÉE (archivage après rejet)
            (WriteIntentState::Rejected, WriteIntentState::Archived) => true,
            // APPLIQUÉE → ARCHIVÉE (archivage après application)
            (WriteIntentState::Applied, WriteIntentState::Archived) => true,
            // Toutes les autres transitions sont invalides
            _ => false,
        };

        if !valid {
            // Rejet explicite de la transition invalide
            let error = KMError::InvalidState {
                current_state: from.to_string().to_string(),
                reason: format!(
                    "Transition invalide vers {} depuis {}. Transition non autorisée par le contrat FONDATION.",
                    to.to_string(),
                    from.to_string()
                ),
            };
            println!(
                "[WriteIntentLifecycle] Transition rejetée pour intent {}: {}",
                intent_id, error
            );
            return Err(error);
        }

        // Log conceptuel de la transition réussie
        println!(
            "[WriteIntentLifecycle] Transition réussie pour intent {}: {} → {}",
            intent_id,
            from.to_string(),
            to.to_string()
        );

        Ok(())
    }

    /// Valide une intention (transition CRÉÉE → EN_VALIDATION)
    ///
    /// Conforme à la règle VALID-1 : toute Write Intent DOIT traverser les Runtime Boundaries.
    pub fn start_validation(
        current_state: WriteIntentState,
        intent_id: &str,
    ) -> Result<WriteIntentState, KMError> {
        Self::transition(current_state, WriteIntentState::InValidation, intent_id)?;
        Ok(WriteIntentState::InValidation)
    }

    /// Accepte une intention (transition EN_VALIDATION → ACCEPTÉE)
    ///
    /// Conforme à la règle ACCEPT-1 : une intention ACCEPTÉE DOIT être appliquée.
    pub fn accept(
        current_state: WriteIntentState,
        intent_id: &str,
    ) -> Result<WriteIntentState, KMError> {
        Self::transition(current_state, WriteIntentState::Accepted, intent_id)?;
        Ok(WriteIntentState::Accepted)
    }

    /// Rejette une intention (transition EN_VALIDATION → REJETÉE)
    ///
    /// Conforme à la règle REJECT-1 : un rejet DOIT indiquer explicitement la raison.
    pub fn reject(
        current_state: WriteIntentState,
        intent_id: &str,
        reason: String,
    ) -> Result<WriteIntentState, KMError> {
        Self::transition(current_state, WriteIntentState::Rejected, intent_id)?;
        println!(
            "[WriteIntentLifecycle] Intention {} rejetée. Raison: {}",
            intent_id, reason
        );
        Ok(WriteIntentState::Rejected)
    }

    /// Applique une intention (transition ACCEPTÉE → APPLIQUÉE)
    ///
    /// Conforme à la règle APPLY-1 : l'application est atomique (conceptuel, no-op en mémoire).
    pub fn apply(
        current_state: WriteIntentState,
        intent_id: &str,
    ) -> Result<WriteIntentState, KMError> {
        Self::transition(current_state, WriteIntentState::Applied, intent_id)?;
        println!(
            "[WriteIntentLifecycle] Intention {} appliquée (conceptuel, no-op en mémoire)",
            intent_id
        );
        Ok(WriteIntentState::Applied)
    }

    /// Archive une intention (transition REJETÉE/APPLIQUÉE → ARCHIVÉE)
    ///
    /// Conforme à la règle ARCHIV-1 : toute intention terminée DOIT être archivée.
    pub fn archive(
        current_state: WriteIntentState,
        intent_id: &str,
    ) -> Result<WriteIntentState, KMError> {
        Self::transition(current_state, WriteIntentState::Archived, intent_id)?;
        println!(
            "[WriteIntentLifecycle] Intention {} archivée pour traçabilité",
            intent_id
        );
        Ok(WriteIntentState::Archived)
    }

    /// Vérifie si une intention peut être réutilisée
    ///
    /// Conforme à la règle NOREUSE-1 : une Write Intent ne peut être soumise qu'une seule fois.
    pub fn can_reuse(current_state: WriteIntentState) -> bool {
        // Seule l'état CRÉÉE permet la soumission
        matches!(current_state, WriteIntentState::Created)
    }

    /// Vérifie si une intention peut être validée à nouveau
    ///
    /// Conforme à la règle NOREUSE-2 : une intention rejetée ne peut pas être réessayée.
    pub fn can_validate_again(current_state: WriteIntentState) -> bool {
        // Seule l'état CRÉÉE permet la validation
        matches!(current_state, WriteIntentState::Created)
    }
}
