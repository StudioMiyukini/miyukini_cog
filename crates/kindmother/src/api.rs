//! CoreDataAPI - Surface d'appel unique de KindMother
//!
//! Définit l'interface conceptuelle entre les adaptateurs et KindMother.
//! Implémente les Runtime Boundaries selon le contrat FONDATION.

use crate::core::{
    AuthorityContext, DomainContext, InstanceContext, WriteIntent,
};
use crate::errors::KMError;
use crate::runtime::RuntimeBoundary;
use crate::state::KMState;
use crate::sync::SyncIntent;

/// CoreDataAPI - Surface d'appel unique
///
/// Implémente les Runtime Boundaries selon le contrat FONDATION.
pub struct CoreDataAPI {
    /// État actuel du moteur
    state: KMState,
    /// Runtime Boundary pour la validation
    boundary: RuntimeBoundary,
}

impl CoreDataAPI {
    /// Crée une nouvelle instance de CoreDataAPI
    pub fn new(state: KMState) -> Self {
        println!("[CoreDataAPI] Création d'une nouvelle instance");
        Self {
            state,
            boundary: RuntimeBoundary::new(),
        }
    }

    /// Soumet un WriteIntent pour validation et application
    ///
    /// Traverse toutes les Runtime Boundaries selon le contrat FONDATION.
    pub fn submit_write_intent(
        &mut self,
        intent: WriteIntent,
        authority: AuthorityContext,
        instance: InstanceContext,
        domain: DomainContext,
    ) -> Result<(), KMError> {
        println!(
            "[CoreDataAPI] Tentative de soumission de WriteIntent: {}",
            intent.intent_id()
        );

        // Traverse toutes les Runtime Boundaries (incluant la boundary WriteIntent)
        self.boundary.check_all_boundaries_with_intent(
            "submit_write_intent",
            &intent,
            &authority,
            &instance,
            &domain,
            self.state,
        )?;

        // À ce stade, les boundaries sont validées mais l'opération n'est pas implémentée
        // (pas de persistance, pas d'application réelle)
        println!(
            "[CoreDataAPI] WriteIntent {} validée par toutes les boundaries (opération conceptuelle, pas d'application réelle)",
            intent.intent_id()
        );
        Ok(())
    }

    /// Soumet un batch de WriteIntent pour validation et application atomique
    ///
    /// Traverse toutes les Runtime Boundaries pour chaque intention.
    pub fn submit_batch_write_intent(
        &mut self,
        intents: Vec<WriteIntent>,
        authority: AuthorityContext,
        instance: InstanceContext,
        domain: DomainContext,
    ) -> Result<(), KMError> {
        println!(
            "[CoreDataAPI] Tentative de soumission de batch WriteIntent ({} intentions)",
            intents.len()
        );

        // Vérifie les boundaries pour chaque intention
        for intent in &intents {
            self.boundary.check_all_boundaries_with_intent(
                "submit_batch_write_intent",
                intent,
                &authority,
                &instance,
                &domain,
                self.state,
            )?;
        }

        println!(
            "[CoreDataAPI] Batch WriteIntent validé par toutes les boundaries ({} intentions, opération conceptuelle)",
            intents.len()
        );
        Ok(())
    }

    /// Lit une entité par identifiant
    ///
    /// Traverse toutes les Runtime Boundaries (lecture = is_write = false).
    pub fn read_entity(
        &mut self,
        _entity_id: &str,
        authority: AuthorityContext,
        instance: InstanceContext,
        domain: DomainContext,
    ) -> Result<(), KMError> {
        println!("[CoreDataAPI] Tentative de lecture d'entité: {}", _entity_id);

        // Traverse toutes les Runtime Boundaries (lecture, pas d'écriture)
        self.boundary.check_all_boundaries(
            "read_entity",
            &authority,
            &instance,
            &domain,
            self.state,
            false, // is_write = false pour une lecture
        )?;

        // À ce stade, les boundaries sont validées mais l'opération n'est pas implémentée
        // (pas de persistance, pas de lecture réelle)
        println!(
            "[CoreDataAPI] Lecture d'entité {} validée par toutes les boundaries (opération conceptuelle, pas de lecture réelle)",
            _entity_id
        );
        Ok(())
    }

    /// Met à jour l'état du moteur (pour les tests)
    #[cfg(test)]
    pub fn set_state(&mut self, state: KMState) {
        self.state = state;
    }

    /// Obtient l'état actuel du moteur
    pub fn state(&self) -> KMState {
        self.state
    }

    /// Soumet une SyncIntent pour validation
    ///
    /// ATTENTION : À ce stade, TOUTE synchronisation est REFUSÉE explicitement.
    /// Cette méthode existe pour la structure, mais retourne toujours une erreur.
    ///
    /// Conforme au contrat FONDATION "Sync & Conflict Resolution Contract".
    pub fn submit_sync_intent(
        &mut self,
        sync_intent: &SyncIntent,
        authority: AuthorityContext,
        instance: InstanceContext,
        domain: DomainContext,
    ) -> Result<(), KMError> {
        println!(
            "[CoreDataAPI] Tentative de soumission de SyncIntent: {}",
            sync_intent.sync_id()
        );

        // Traverse toutes les Runtime Boundaries pour la synchronisation
        // Note: Cette méthode rejette TOUJOURS la synchronisation à ce stade
        let result = self.boundary.check_all_sync_boundaries(
            sync_intent,
            &authority,
            &instance,
            &domain,
            self.state,
        );

        match &result {
            Ok(()) => {
                // Ce cas ne devrait jamais arriver car sync est toujours refusé
                println!(
                    "[CoreDataAPI] SyncIntent {} validée (ne devrait pas arriver)",
                    sync_intent.sync_id()
                );
            }
            Err(e) => {
                println!(
                    "[CoreDataAPI] SyncIntent {} rejetée explicitement: {}",
                    sync_intent.sync_id(),
                    e
                );
            }
        }

        result
    }
}
