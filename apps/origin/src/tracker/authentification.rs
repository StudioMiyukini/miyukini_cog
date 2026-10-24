//! @id     origin.tracker.authentification
//! @do     authentifier_message_tracker
//! @role   securite
//! @layer  domain
//! @human  Authentification des messages Tracker mutateurs (enveloppe signée, anti-rejeu).
//!
//! Constat O-02 : ANNOUNCE, WITHDRAW, HEARTBEAT, `CREATE/UPDATE/DELETE/JOIN_LOBBY` doivent être
//! signés par la clé d'identité du COG. Le `cog_id` n'est jamais lu du corps sans preuve :
//! il provient de l'enveloppe vérifiée, et un `cog_id` contradictoire dans le corps est refusé.
//! Anti-rejeu : compteur strictement croissant par `cog_id`, dans une fenêtre d'horloge.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

use miyuwebway_participant::identite::{
    maintenant_ms, EnveloppeTracker, ErreurIdentite, MessageTrackerVerifie,
};

use crate::relay::RegistreIdentites;

/// Fenêtre d'horloge acceptée pour le compteur (millisecondes, ± 5 min).
pub const FENETRE_COMPTEUR_MS: u64 = 5 * 60 * 1000;
/// Nombre maximal de compteurs suivis (C-13).
pub const MAX_COMPTEURS: usize = 1_000_000;

/// Refus d'un message Tracker (C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusTracker {
    /// Message non enveloppé ou enveloppe illisible.
    #[error("enveloppe signée requise")]
    EnveloppeAbsente,
    /// Signature, clé ou lien `cog_id` invalides.
    #[error("preuve invalide : {0}")]
    Preuve(ErreurIdentite),
    /// Aucune clé épinglée (COG jamais enregistré auprès du Relay).
    #[error("identité inconnue du Relay")]
    IdentiteInconnue,
    /// Clé différente de la clé épinglée.
    #[error("clé différente de la clé épinglée")]
    CleDifferente,
    /// Compteur hors de la fenêtre d'horloge.
    #[error("horodatage hors fenêtre")]
    HorsFenetre,
    /// Compteur déjà vu (rejeu).
    #[error("message rejoué")]
    Rejeu,
    /// Corps JSON invalide.
    #[error("corps invalide")]
    CorpsInvalide,
    /// `cog_id` du corps différent du `cog_id` authentifié.
    #[error("cog_id du corps différent de l'émetteur authentifié")]
    CogIdIncoherent,
    /// Trop de COG suivis.
    #[error("capacité dépassée")]
    Capacite,
}

/// Garde d'authentification du Tracker.
#[derive(Debug, Clone)]
pub struct GardeTracker {
    /// Clés épinglées par le Relay (TOFU). `None` : lien empreinte ↔ clé seulement.
    identites: Option<Arc<RegistreIdentites>>,
    /// Dernier compteur accepté par `cog_id`.
    compteurs: Arc<RwLock<HashMap<String, u64>>>,
}

impl GardeTracker {
    /// Nouvelle garde.
    #[must_use]
    pub fn new(identites: Option<Arc<RegistreIdentites>>) -> Self {
        Self {
            identites,
            compteurs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Authentifie un message mutateur de type `type_message`.
    ///
    /// Ordre : enveloppe → signature (`verify_strict`) et lien `cog_id` ↔ clé → clé épinglée
    /// → fenêtre d'horloge → cohérence du corps → compteur (mis à jour seulement si tout est
    /// valide, pour qu'un message non authentifié ne puisse pas faire avancer le compteur).
    pub async fn authentifier(
        &self,
        type_message: u8,
        payload: &[u8],
    ) -> Result<MessageTrackerVerifie, RefusTracker> {
        let enveloppe: EnveloppeTracker =
            serde_json::from_slice(payload).map_err(|_| RefusTracker::EnveloppeAbsente)?;
        let verifie = enveloppe
            .verifier(type_message)
            .map_err(RefusTracker::Preuve)?;

        if let Some(identites) = &self.identites {
            match identites.cle_epinglee(&verifie.cog_id).await {
                Some(cle) if cle == verifie.cle_publique => {}
                Some(_) => return Err(RefusTracker::CleDifferente),
                None => return Err(RefusTracker::IdentiteInconnue),
            }
        }

        let maintenant = maintenant_ms();
        if verifie.compteur < maintenant.saturating_sub(FENETRE_COMPTEUR_MS)
            || verifie.compteur > maintenant.saturating_add(FENETRE_COMPTEUR_MS)
        {
            return Err(RefusTracker::HorsFenetre);
        }

        let corps: serde_json::Value =
            serde_json::from_str(&verifie.corps).map_err(|_| RefusTracker::CorpsInvalide)?;
        if let Some(cog_id_corps) = corps.get("cog_id") {
            if cog_id_corps.as_str() != Some(verifie.cog_id.as_str()) {
                return Err(RefusTracker::CogIdIncoherent);
            }
        }

        let mut compteurs = self.compteurs.write().await;
        match compteurs.get(&verifie.cog_id) {
            Some(dernier) if verifie.compteur <= *dernier => {
                warn!("Tracker: message rejoué pour {}", verifie.cog_id);
                return Err(RefusTracker::Rejeu);
            }
            Some(_) => {}
            None => {
                if compteurs.len() >= MAX_COMPTEURS {
                    // Purge des compteurs sortis de la fenêtre (rejeu impossible au-delà).
                    let seuil = maintenant.saturating_sub(FENETRE_COMPTEUR_MS);
                    compteurs.retain(|_, c| *c >= seuil);
                    if compteurs.len() >= MAX_COMPTEURS {
                        return Err(RefusTracker::Capacite);
                    }
                }
            }
        }
        compteurs.insert(verifie.cog_id.clone(), verifie.compteur);
        Ok(verifie)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuwebway_participant::identite::IdentiteCog;

    const WITHDRAW: u8 = 0x03;
    const HEARTBEAT: u8 = 0x04;

    fn signe(id: &IdentiteCog, t: u8, compteur: u64, corps: &str) -> Vec<u8> {
        let env = EnveloppeTracker::sceller(id, t, compteur, corps.to_string()).expect("sceller");
        serde_json::to_vec(&env).expect("json")
    }

    async fn garde_avec(id: &IdentiteCog) -> GardeTracker {
        let identites = Arc::new(RegistreIdentites::new());
        identites
            .epingler(&id.cog_id(), &id.cle_publique())
            .await
            .expect("épinglage");
        GardeTracker::new(Some(identites))
    }

    #[tokio::test]
    async fn withdraw_non_authentifie_refuse() {
        let id = IdentiteCog::generer();
        let garde = garde_avec(&id).await;
        let brut = format!(r#"{{"cog_id":"{}"}}"#, id.cog_id());
        assert_eq!(
            garde.authentifier(WITHDRAW, brut.as_bytes()).await,
            Err(RefusTracker::EnveloppeAbsente)
        );
    }

    #[tokio::test]
    async fn heartbeat_non_authentifie_refuse() {
        let id = IdentiteCog::generer();
        let garde = garde_avec(&id).await;
        let brut = format!(r#"{{"cog_id":"{}","timestamp":1}}"#, id.cog_id());
        assert_eq!(
            garde.authentifier(HEARTBEAT, brut.as_bytes()).await,
            Err(RefusTracker::EnveloppeAbsente)
        );
    }

    #[tokio::test]
    async fn withdraw_signe_par_autre_cle_refuse() {
        let victime = IdentiteCog::generer();
        let attaquant = IdentiteCog::generer();
        let garde = garde_avec(&victime).await;
        // L'attaquant signe avec sa clé en se faisant passer pour la victime.
        let mut env = EnveloppeTracker::sceller(
            &attaquant,
            WITHDRAW,
            maintenant_ms(),
            format!(r#"{{"cog_id":"{}"}}"#, victime.cog_id()),
        )
        .expect("sceller");
        env.cog_id = victime.cog_id();
        let r = garde
            .authentifier(WITHDRAW, &serde_json::to_vec(&env).expect("json"))
            .await;
        assert!(matches!(r, Err(RefusTracker::Preuve(_))));
        // Signé par l'attaquant pour lui-même mais avec le cog_id victime dans le corps.
        let p = signe(
            &attaquant,
            WITHDRAW,
            maintenant_ms(),
            &format!(r#"{{"cog_id":"{}"}}"#, victime.cog_id()),
        );
        let identites = Arc::new(RegistreIdentites::new());
        identites
            .epingler(&attaquant.cog_id(), &attaquant.cle_publique())
            .await
            .expect("épinglage");
        let garde2 = GardeTracker::new(Some(identites));
        assert_eq!(
            garde2.authentifier(WITHDRAW, &p).await,
            Err(RefusTracker::CogIdIncoherent)
        );
    }

    #[tokio::test]
    async fn identite_non_epinglee_refusee() {
        let id = IdentiteCog::generer();
        let garde = GardeTracker::new(Some(Arc::new(RegistreIdentites::new())));
        let p = signe(&id, HEARTBEAT, maintenant_ms(), "{}");
        assert_eq!(
            garde.authentifier(HEARTBEAT, &p).await,
            Err(RefusTracker::IdentiteInconnue)
        );
    }

    #[tokio::test]
    async fn rejeu_et_fenetre_refuses() {
        let id = IdentiteCog::generer();
        let garde = garde_avec(&id).await;
        let c = maintenant_ms();
        let p = signe(&id, HEARTBEAT, c, "{}");
        assert!(garde.authentifier(HEARTBEAT, &p).await.is_ok());
        // Rejeu exact.
        assert_eq!(garde.authentifier(HEARTBEAT, &p).await, Err(RefusTracker::Rejeu));
        // Compteur plus ancien.
        let p = signe(&id, HEARTBEAT, c - 1, "{}");
        assert_eq!(garde.authentifier(HEARTBEAT, &p).await, Err(RefusTracker::Rejeu));
        // Hors fenêtre.
        let p = signe(&id, HEARTBEAT, c - FENETRE_COMPTEUR_MS - 10_000, "{}");
        assert_eq!(
            garde.authentifier(HEARTBEAT, &p).await,
            Err(RefusTracker::HorsFenetre)
        );
        // Message suivant valide.
        let p = signe(&id, HEARTBEAT, c + 1, "{}");
        assert!(garde.authentifier(HEARTBEAT, &p).await.is_ok());
    }

    #[tokio::test]
    async fn heartbeat_rejoue_en_withdraw_refuse() {
        let id = IdentiteCog::generer();
        let garde = garde_avec(&id).await;
        let p = signe(&id, HEARTBEAT, maintenant_ms(), "{}");
        assert!(matches!(
            garde.authentifier(WITHDRAW, &p).await,
            Err(RefusTracker::Preuve(_))
        ));
    }
}
