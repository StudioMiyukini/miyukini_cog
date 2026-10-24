//! @id     maia_confirmations
//! @do     suspendre_actions_jusqu_a_confirmation
//! @role   security
//! @layer  app
//! @human  MAIA — registre des actions suspendues en attente de confirmation humaine (K-02, C-51).

//! Registre des actions d'outils suspendues par la garde « contenu étranger ».
//!
//! Une action suspendue est **figée** (skill + paramètres exacts) au moment
//! du blocage. L'utilisateur la voit telle quelle et la confirme dans une
//! requête séparée (`POST /v1/confirmations/{id}`), hors de la boucle du
//! modèle : le modèle ne peut ni la modifier ni la confirmer lui-même.
//!
//! Propriétés : usage unique, expiration ([`DUREE_VIE_SECS`]), capacité bornée
//! ([`CAPACITE_MAX`], C-13), refus par défaut de tout identifiant inconnu (C-12).

use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::garde_etranger;

/// Durée de vie d'une action en attente (secondes).
pub const DUREE_VIE_SECS: u64 = 300;
/// Nombre maximal d'actions en attente simultanées.
pub const CAPACITE_MAX: usize = 32;

/// Action suspendue, présentée à l'utilisateur.
#[derive(Debug, Clone, Serialize)]
pub struct ActionEnAttente {
    /// Identifiant aléatoire (128 bits) à présenter pour confirmer.
    pub id: String,
    /// Agent à l'origine de l'appel.
    pub agent_id: String,
    /// Skill visé.
    pub skill_id: String,
    /// Paramètres exacts, figés.
    pub params: HashMap<String, serde_json::Value>,
    /// Description exacte de l'action (skill + paramètres canoniques).
    pub action: String,
    /// Raison de la suspension.
    pub motif: String,
    /// Secondes restantes avant expiration (calculé à l'affichage).
    pub expire_dans_s: u64,
    #[serde(skip)]
    cree_a: Instant,
}

/// Refus de confirmation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusConfirmation {
    /// Identifiant inconnu ou déjà consommé.
    #[error("action en attente inconnue ou déjà traitée")]
    Inconnue,
    /// Action expirée.
    #[error("action en attente expirée (durée de vie {DUREE_VIE_SECS} s)")]
    Expiree,
    /// Registre plein.
    #[error("trop d'actions en attente de confirmation ({CAPACITE_MAX})")]
    RegistrePlein,
    /// Verrou interne empoisonné.
    #[error("registre des confirmations indisponible")]
    Indisponible,
}

/// Registre des actions en attente (partagé dans l'état du serveur).
#[derive(Debug, Default)]
pub struct RegistreConfirmations {
    actions: Mutex<HashMap<String, ActionEnAttente>>,
}

impl RegistreConfirmations {
    /// Nouveau registre vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Suspend une action et retourne sa fiche (avec son identifiant).
    pub fn suspendre(
        &self,
        agent_id: &str,
        skill_id: &str,
        params: HashMap<String, serde_json::Value>,
        motif: &str,
    ) -> Result<ActionEnAttente, RefusConfirmation> {
        let mut actions = self
            .actions
            .lock()
            .map_err(|_| RefusConfirmation::Indisponible)?;
        purger(&mut actions);
        if actions.len() >= CAPACITE_MAX {
            return Err(RefusConfirmation::RegistrePlein);
        }
        let action = ActionEnAttente {
            id: garde_etranger::identifiant_aleatoire(),
            agent_id: agent_id.to_string(),
            skill_id: skill_id.to_string(),
            action: garde_etranger::decrire_action(skill_id, &params),
            params,
            motif: motif.to_string(),
            expire_dans_s: DUREE_VIE_SECS,
            cree_a: Instant::now(),
        };
        actions.insert(action.id.clone(), action.clone());
        Ok(action)
    }

    /// Liste les actions en attente non expirées.
    pub fn lister(&self) -> Vec<ActionEnAttente> {
        let Ok(mut actions) = self.actions.lock() else {
            return Vec::new();
        };
        purger(&mut actions);
        actions
            .values()
            .map(|a| {
                let mut a = a.clone();
                a.expire_dans_s =
                    DUREE_VIE_SECS.saturating_sub(a.cree_a.elapsed().as_secs());
                a
            })
            .collect()
    }

    /// Retire l'action (usage unique). Expirée ou inconnue : refus.
    pub fn prendre(&self, id: &str) -> Result<ActionEnAttente, RefusConfirmation> {
        let mut actions = self
            .actions
            .lock()
            .map_err(|_| RefusConfirmation::Indisponible)?;
        let action = actions.remove(id).ok_or(RefusConfirmation::Inconnue)?;
        if action.cree_a.elapsed() > Duration::from_secs(DUREE_VIE_SECS) {
            return Err(RefusConfirmation::Expiree);
        }
        Ok(action)
    }

    /// Nombre d'actions actuellement stockées (tests, supervision).
    pub fn len(&self) -> usize {
        self.actions.lock().map_or(0, |a| a.len())
    }

    /// Vrai si aucune action n'est en attente.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Vieillit artificiellement une action (tests d'expiration).
    #[cfg(test)]
    fn vieillir(&self, id: &str, age: Duration) {
        if let Ok(mut actions) = self.actions.lock() {
            if let Some(a) = actions.get_mut(id) {
                if let Some(t) = Instant::now().checked_sub(age) {
                    a.cree_a = t;
                }
            }
        }
    }
}

/// Retire les actions expirées.
fn purger(actions: &mut HashMap<String, ActionEnAttente>) {
    let limite = Duration::from_secs(DUREE_VIE_SECS);
    actions.retain(|_, a| a.cree_a.elapsed() <= limite);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> HashMap<String, serde_json::Value> {
        let mut p = HashMap::new();
        p.insert("action".into(), "write".into());
        p
    }

    #[test]
    fn suspendre_puis_prendre_une_seule_fois() {
        let r = RegistreConfirmations::new();
        let a = r.suspendre("victor", "file_ops", params(), "test").unwrap();
        assert_eq!(a.action, r#"file_ops {"action":"write"}"#);
        assert_eq!(r.lister().len(), 1);
        let pris = r.prendre(&a.id).unwrap();
        assert_eq!(pris.skill_id, "file_ops");
        assert_eq!(r.prendre(&a.id).unwrap_err(), RefusConfirmation::Inconnue);
    }

    #[test]
    fn identifiant_inconnu_refuse() {
        let r = RegistreConfirmations::new();
        assert_eq!(r.prendre("nimporte").unwrap_err(), RefusConfirmation::Inconnue);
    }

    #[test]
    fn action_expiree_refusee() {
        let r = RegistreConfirmations::new();
        let a = r.suspendre("victor", "file_ops", params(), "test").unwrap();
        r.vieillir(&a.id, Duration::from_secs(DUREE_VIE_SECS + 1));
        assert_eq!(r.prendre(&a.id).unwrap_err(), RefusConfirmation::Expiree);
    }

    #[test]
    fn capacite_bornee() {
        let r = RegistreConfirmations::new();
        for _ in 0..CAPACITE_MAX {
            r.suspendre("victor", "file_ops", params(), "test").unwrap();
        }
        assert_eq!(
            r.suspendre("victor", "file_ops", params(), "test").unwrap_err(),
            RefusConfirmation::RegistrePlein
        );
    }
}
