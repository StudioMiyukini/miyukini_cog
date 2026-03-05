//! Opérateur Presenter — consultation, effacement, paramètres.

use crate::aggregates::compute_aggregates;
use crate::data::{AuditEvent, MiyukiniWatchDb, Prefs};
use crate::errors::MiyukiniWatchError;
use std::sync::Arc;

/// Présente les données à l'utilisateur, gère l'effacement et les paramètres.
pub struct MiyukiniWatchPresenter {
    db: Arc<MiyukiniWatchDb>,
}

impl MiyukiniWatchPresenter {
    /// Crée un nouveau présentateur.
    #[must_use]
    pub fn new(db: Arc<MiyukiniWatchDb>) -> Self {
        Self { db }
    }

    /// Récupère les préférences.
    pub fn get_prefs(&self, profile_id: &str) -> Result<Prefs, MiyukiniWatchError> {
        self.db.get_prefs(profile_id)
    }

    /// Met à jour les préférences et enregistre l'audit.
    pub fn set_prefs(&self, prefs: &Prefs) -> Result<(), MiyukiniWatchError> {
        self.db.set_prefs(prefs)?;
        let _ = self.db.insert_audit(
            &prefs.profile_id,
            "prefs_updated",
            Some("Modification des préférences"),
            None,
        );
        Ok(())
    }

    /// Vérifie si des données existent pour le profil.
    pub fn has_data(&self, profile_id: &str) -> Result<bool, MiyukiniWatchError> {
        self.db.has_data(profile_id)
    }

    /// Vérifie si la collecte est active.
    pub fn is_collecting(&self, profile_id: &str) -> Result<bool, MiyukiniWatchError> {
        let prefs = self.db.get_prefs(profile_id)?;
        Ok(prefs.collect_enabled)
    }

    /// Récupère les agrégats (pour Miou ou l'UI).
    pub fn get_aggregates(
        &self,
        profile_id: &str,
    ) -> Result<Vec<crate::aggregates::Aggregate>, MiyukiniWatchError> {
        compute_aggregates(&self.db, profile_id)
    }

    /// Efface toutes les données du profil.
    pub fn delete_all(&self, profile_id: &str) -> Result<i64, MiyukiniWatchError> {
        let deleted = self.db.delete_all(profile_id)?;
        let _ = self.db.insert_audit(
            profile_id,
            "delete_all",
            Some("Effacement total par l'utilisateur"),
            Some(deleted),
        );
        Ok(deleted)
    }

    /// Liste les événements d'audit.
    pub fn list_audit(
        &self,
        profile_id: &str,
        limit: i32,
    ) -> Result<Vec<AuditEvent>, MiyukiniWatchError> {
        self.db.list_audit_events(profile_id, limit)
    }
}
