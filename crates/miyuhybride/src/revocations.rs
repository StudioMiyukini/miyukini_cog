//! @id     miyuhybride_revocations
//! @do     rafraichir_revocations_facade
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — révocations d'une façade (fonctionnalité `revocations`) : listes miyauth tirées du registre vérifié, remises par la zone COG pendant la vie de la façade, fraîcheur ≤ 15 min, refus sinon.

//! # Révocations d'une façade (A-007, « rafraîchissement des révocations »)
//!
//! Jusqu'ici la façade recevait un **instantané** au lancement. Ici :
//!
//! - **source** : [`miyuregistre::RevocationsRegistre`] (fonctionnalité
//!   `miyauth` de `miyuregistre`), seule lecture admise des révocations
//!   miyauth ← registre (A-007 : entrées au seuil sur les ancres du COG,
//!   horodatage = tête vérifiée) ;
//! - **rafraîchissement** : la zone COG (Cores), qui tient le miroir vérifié
//!   du registre, rafraîchit la liste de chaque COG émetteur (COG hôte et COG
//!   reconnus) et la **remet** à la façade ([`RevocationsFacade::rafraichir`]
//!   ou [`RevocationsFacade::remettre`]) ; la façade ne lit que la poignée
//!   ([`SourceRevocation`]), elle ne touche jamais au registre ;
//! - **monotonie** : une liste plus ancienne, ou qui connaît moins de
//!   révocations que celle détenue, est refusée (une révocation ne disparaît
//!   pas) ;
//! - **fraîcheur** : 15 min au plus (A-007), mesurée sur l'horodatage de la
//!   **tête vérifiée**, pas sur l'heure de remise ;
//!
//! # Fail-closed
//!
//! Aucune liste pour l'émetteur, liste jamais rafraîchie, tête de plus de
//! 15 min, tête datée dans le futur, verrou empoisonné ⇒ erreur, donc **refus**
//! du titre par miyauth (aucune acceptation par défaut).

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{ErreurJournal, MiroirRegistre, RevocationsRegistre, Stockage};

/// Fraîcheur maximale des révocations d'une façade (A-007 : 15 min).
pub const FRAICHEUR_REVOCATIONS_FACADE_SECS: u64 = FRAICHEUR_MAX_DEFAUT_SECS;

/// Remise refusée.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusRemise {
    /// Liste jamais rafraîchie (aucune tête vérifiée).
    #[error("liste de révocation jamais rafraîchie")]
    JamaisRafraichie,
    /// Liste plus ancienne, ou moins complète, que celle détenue.
    #[error("liste de révocation en recul")]
    Recul,
    /// Rafraîchissement depuis le miroir refusé (tête gelée, stockage…).
    #[error("rafraîchissement refusé : {0}")]
    Registre(String),
    /// Verrou empoisonné.
    #[error("état indisponible")]
    EtatIndisponible,
}

/// Révocations remises à une façade, une liste par COG émetteur ; clonable
/// (la zone COG garde une poignée pour la rafraîchir, la façade lit l'autre).
#[derive(Debug, Clone, Default)]
pub struct RevocationsFacade {
    listes: Arc<RwLock<BTreeMap<CogId, RevocationsRegistre>>>,
}

impl RevocationsFacade {
    /// Aucune liste : tout titre est refusé.
    pub fn new() -> Self {
        Self::default()
    }

    /// Zone COG : remet la liste de `cog` (déjà rafraîchie sur le miroir vérifié).
    ///
    /// Une liste d'un autre COG remise sous `cog` ne sert à rien : miyauth
    /// l'interroge pour `cog` et elle répond « absente » (refus).
    pub fn remettre(&self, cog: CogId, liste: RevocationsRegistre) -> Result<(), RefusRemise> {
        let a_jour_au = liste.a_jour_au().ok_or(RefusRemise::JamaisRafraichie)?;
        let mut listes = self.listes.write().map_err(|_| RefusRemise::EtatIndisponible)?;
        if let Some(actuelle) = listes.get(&cog) {
            let recul = actuelle.a_jour_au().is_some_and(|a| a_jour_au < a) || liste.len() < actuelle.len();
            if recul {
                return Err(RefusRemise::Recul);
            }
        }
        listes.insert(cog, liste);
        Ok(())
    }

    /// Zone COG : rafraîchit `liste` sur le miroir vérifié du registre de
    /// `cog` (tête fraîche exigée), puis la remet. Erreur : rien n'est remis
    /// (la liste détenue par la façade se périme d'elle-même).
    pub fn rafraichir<S: Stockage>(
        &self,
        cog: CogId,
        liste: &mut RevocationsRegistre,
        miroir: &MiroirRegistre<S>,
        maintenant: u64,
    ) -> Result<usize, RefusRemise> {
        let ajoutees = liste.rafraichir(miroir, maintenant).map_err(|e: ErreurJournal| RefusRemise::Registre(e.to_string()))?;
        self.remettre(cog, liste.clone())?;
        Ok(ajoutees)
    }

    /// Horodatage (tête vérifiée) de la liste détenue pour `cog`.
    pub fn a_jour_au(&self, cog: &CogId) -> Option<u64> {
        self.listes.read().ok()?.get(cog)?.a_jour_au()
    }
}

impl SourceRevocation for RevocationsFacade {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        let listes = self.listes.read().map_err(|_| ErreurRevocation::Indisponible("verrou".into()))?;
        let liste = listes.get(emetteur).ok_or(ErreurRevocation::ListeAbsente)?;
        let a_jour_au = liste.a_jour_au().ok_or(ErreurRevocation::ListeAbsente)?;
        // Borne propre à la façade (indépendante du réglage de la liste).
        if a_jour_au.saturating_add(FRAICHEUR_REVOCATIONS_FACADE_SECS) < maintenant_secs {
            return Err(ErreurRevocation::Perimee { a_jour_au, maintenant: maintenant_secs });
        }
        liste.est_revoque(emetteur, cible, maintenant_secs)
    }
}
