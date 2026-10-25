//! @id     miyuhybride_revocations
//! @do     rafraichir_revocations_facade
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — révocations d'une façade (fonctionnalité `revocations`) : listes miyauth tirées du registre vérifié, remises par la zone COG pendant la vie de la façade, fraîcheur ≤ 15 min, refus sinon.

//! # Révocations d'une façade (A-007, « rafraîchissement des révocations »)
//!
//! Jusqu'ici la façade recevait un **instantané** au lancement. Ici :
//!
//! - **source** : [`miyuregistre::SuiviRevocationsRegistre`] (fonctionnalité
//!   `miyauth` de `miyuregistre`), suivi **unique** du registre de révocation
//!   d'un COG (miroir vérifié + liste, fraîcheur ≤ 15 min imposée par le type),
//!   le même qu'Origin et MiyuCloud ;
//! - **rafraîchissement** : la zone COG (Cores) tient le suivi de chaque COG
//!   émetteur (COG hôte et COG reconnus) et **remet** à la façade l'instantané
//!   de sa liste ([`RevocationsFacade::rafraichir`] ou
//!   [`RevocationsFacade::remettre`]) ; la façade ne lit que la poignée
//!   ([`SourceRevocation`]), elle ne touche jamais au registre ;
//! - **monotonie et inclusion** : une liste plus ancienne, d'un autre COG, ou
//!   qui ne contient pas **toutes** les révocations de celle détenue, est
//!   refusée (une révocation ne disparaît jamais, même si le nombre de cibles
//!   ne baisse pas) ;
//! - **fraîcheur** : celle de [`RevocationsRegistre`], plafonnée à 15 min par
//!   construction et mesurée sur l'horodatage de la **tête vérifiée** ; elle
//!   n'est plus recodée ici.
//!
//! # Fail-closed
//!
//! Aucune liste pour l'émetteur, liste jamais rafraîchie, tête de plus de
//! 15 min, tête datée dans le futur, verrou empoisonné ⇒ erreur, donc **refus**
//! du titre par miyauth (aucune acceptation par défaut).

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{RevocationsRegistre, SuiviRevocationsRegistre};

pub use miyuregistre::controler_fraicheur as controler_fraicheur_revocations;

/// Fraîcheur maximale des révocations d'une façade (A-007 : 15 min) ; c'est
/// [`miyuregistre::FRAICHEUR_REVOCATIONS_SECS`].
pub const FRAICHEUR_REVOCATIONS_FACADE_SECS: u64 = miyuregistre::FRAICHEUR_REVOCATIONS_SECS;

/// Remise refusée.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusRemise {
    /// Liste jamais rafraîchie (aucune tête vérifiée).
    #[error("liste de révocation jamais rafraîchie")]
    JamaisRafraichie,
    /// Liste plus ancienne que celle détenue, ou qui n'inclut pas toutes ses révocations.
    #[error("liste de révocation en recul")]
    Recul,
    /// Liste d'un autre COG que celui sous lequel elle est remise.
    #[error("liste de révocation d'un autre COG")]
    AutreCog,
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

    /// Zone COG : remet la liste de `cog` (instantané d'un suivi vérifié).
    ///
    /// Refus : liste d'un autre COG, jamais rafraîchie, plus ancienne que la
    /// liste détenue ou ne l'incluant pas (nouvel ensemble ⊇ ancien exigé).
    pub fn remettre(&self, cog: CogId, liste: RevocationsRegistre) -> Result<(), RefusRemise> {
        if liste.cog() != &cog {
            return Err(RefusRemise::AutreCog);
        }
        let a_jour_au = liste.a_jour_au().ok_or(RefusRemise::JamaisRafraichie)?;
        let mut listes = self.listes.write().map_err(|_| RefusRemise::EtatIndisponible)?;
        if let Some(actuelle) = listes.get(&cog) {
            let plus_ancienne = actuelle.a_jour_au().is_some_and(|a| a_jour_au < a);
            if plus_ancienne || !liste.inclut(actuelle) {
                return Err(RefusRemise::Recul);
            }
        }
        listes.insert(cog, liste);
        Ok(())
    }

    /// Zone COG : remet l'instantané de la liste tenue par `suivi`. Erreur :
    /// rien n'est remis (la liste détenue par la façade se périme d'elle-même).
    pub fn rafraichir(&self, suivi: &SuiviRevocationsRegistre) -> Result<(), RefusRemise> {
        let liste = suivi.liste().ok_or(RefusRemise::JamaisRafraichie)?;
        self.remettre(suivi.cog().clone(), liste)
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
        // Fraîcheur (≤ 15 min, tête vérifiée) appliquée par la liste elle-même.
        listes.get(emetteur).ok_or(ErreurRevocation::ListeAbsente)?.est_revoque(emetteur, cible, maintenant_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuregistre::{Cible, Corps, EntreeRegistre, Journal, StockageMemoire};
    use miyusceau::{sceller, CleSignature};

    const T0: u64 = 1_800_000_000;
    const REG: [u8; 32] = [0x46; 32];

    fn cog(c: char) -> CogId {
        CogId::new(&c.to_string().repeat(26)).unwrap()
    }

    struct Cog {
        cle: CleSignature,
        journal: Journal<StockageMemoire>,
    }

    impl Cog {
        fn new() -> Self {
            Self {
                cle: CleSignature::generer(),
                journal: Journal::ouvrir(REG, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
            }
        }

        fn revoquer(&mut self, kid: [u8; 32]) {
            let e = EntreeRegistre { inscrite_a: T0, corps: Corps::Revocation { cible: Cible::Cle(kid), motif: "x".into() } };
            self.journal.ajouter(&sceller(&e, &self.cle).unwrap()).unwrap();
        }

        /// Suivi neuf du registre, appliqué à `t`.
        fn suivi(&mut self, c: &CogId, t: u64) -> SuiviRevocationsRegistre {
            let s = SuiviRevocationsRegistre::new(c.clone(), REG, &self.cle.cle_publique()).unwrap();
            let tete = self.journal.emettre_tete(&[&self.cle], t, 86_400).unwrap();
            s.appliquer(&tete, 0, &self.journal.entrees(0, u64::MAX).unwrap(), t).unwrap();
            s
        }
    }

    #[test]
    fn remise_exige_l_inclusion() {
        let hote = cog('h');
        let facade = RevocationsFacade::new();
        let mut cog_a = Cog::new();
        cog_a.revoquer([1; 32]);
        facade.rafraichir(&cog_a.suivi(&hote, T0)).unwrap();
        assert_eq!(facade.est_revoque(&hote, &CibleRevocation::Cle([1; 32]), T0), Ok(true));

        // Liste plus récente, aussi longue, mais sans la révocation [1] : refusée.
        let mut cog_b = Cog::new();
        cog_b.revoquer([2; 32]);
        assert_eq!(facade.rafraichir(&cog_b.suivi(&hote, T0 + 10)), Err(RefusRemise::Recul));
        assert_eq!(facade.est_revoque(&hote, &CibleRevocation::Cle([1; 32]), T0 + 10), Ok(true));
        // Sur-ensemble plus récent : accepté.
        cog_a.revoquer([3; 32]);
        facade.rafraichir(&cog_a.suivi(&hote, T0 + 20)).unwrap();
        assert_eq!(facade.a_jour_au(&hote), Some(T0 + 20));
        // Mêmes révocations mais tête plus ancienne : refusée.
        let mut cog_c = Cog::new();
        cog_c.revoquer([1; 32]);
        cog_c.revoquer([3; 32]);
        assert_eq!(facade.rafraichir(&cog_c.suivi(&hote, T0 + 5)), Err(RefusRemise::Recul));
        // Liste d'un autre COG remise sous `h` : refusée.
        let autre = cog_a.suivi(&cog('z'), T0 + 30);
        assert_eq!(facade.remettre(hote.clone(), autre.liste().unwrap()), Err(RefusRemise::AutreCog));
        // Fraîcheur de la liste (15 min) appliquée sans recodage.
        assert!(matches!(
            facade.est_revoque(&hote, &CibleRevocation::Cle([9; 32]), T0 + 20 + FRAICHEUR_REVOCATIONS_FACADE_SECS + 1),
            Err(ErreurRevocation::Perimee { .. })
        ));
    }
}
