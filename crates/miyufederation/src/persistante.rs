//! @id     miyufederation_persistante
//! @do     persister_chaque_transition_federale
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « persistante » (vue fédérale dont chaque transition est écrite atomiquement sur un support : rechargement revérifié, fail-closed).

//! Vue fédérale **persistante** : chaque transition (candidature,
//! confirmation, actualisation des statuts, accusation, recours, tête
//! témoignée, note locale, carnet de pairs) est écrite **atomiquement** sur
//! un [`SupportEtat`] avant d'être visible en mémoire.
//!
//! # Garanties
//!
//! - **Rechargement revérifié** ([`FederationPersistante::charger`]) : même
//!   chemin que [`crate::StockageFederal::charger`] — titres signés
//!   revérifiés, statuts recalculés, HMAC du secret local, ancre préfixe du
//!   registre local, révocations du registre ré-appliquées, rattrapage de
//!   l'arrêt (une suspension échue pendant l'arrêt devient révocation datée de
//!   son échéance, une seule fois). Tout refus ⇒ `Err`, aucune vue n'est
//!   rendue (fédération désactivée ; l'appelant signale). Aucun statut n'est
//!   jamais lu tel quel : il est **recalculé** depuis les titres.
//! - **Écriture par transition** ([`FederationPersistante::transition`]) :
//!   l'opération s'applique à une **copie** ; la copie est scellée puis écrite
//!   (si elle diffère de l'état écrit) ; elle ne remplace la vue qu'après le
//!   succès de l'écriture. Une opération refusée qui a pourtant modifié la
//!   vue (horloge, note locale d'un faux signalement) est persistée aussi.
//! - **Échec ou interruption d'écriture** : la vue en mémoire reste le
//!   dernier état écrit et l'instance passe **hors service**
//!   ([`ErreurTransition::HorsService`]) : l'issue de l'écriture est inconnue,
//!   seul un nouveau [`FederationPersistante::charger`] (qui relit le support,
//!   source de vérité) rétablit le service. Un futur abandonné en cours
//!   d'écriture laisse aussi l'instance hors service.
//! - **Échéance des 72 h** : l'heure de la suspension (`depuis`) est dans
//!   l'état scellé (pièces revérifiées) ; l'échéance est recalculée au
//!   chargement avec l'heure de confiance fournie par l'appelant (la même
//!   horloge qu'en fonctionnement ; heure en recul ⇒ refus).
//! - **Registre local** : chaque écriture ancre l'état sur la tête courante
//!   du registre ; un registre qui ne prolonge plus l'ancre écrite (tronqué,
//!   remplacé) ⇒ refus et hors service.

use crate::erreurs::ErreurFederation;
use crate::federation::{Federation, Parametres, RevocationFederale, SuspensionFederale};
use crate::note::Observation;
use crate::persistance::{
    controler_inscription, reprendre_avec_registre, AncreRegistre, CleEtatFederal, ErreurEtat, RegistreLocal,
};
use crate::support::SupportEtat;

use miyauth::CogId;
use miyusceau::{Acceptation, CleSignature, Kid, PreuveCoherence};

/// Refus d'une transition persistante.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ErreurTransition {
    /// Opération refusée par les règles fédérales ; si elle a modifié la vue
    /// (horloge, note locale), la vue modifiée **a été** persistée.
    #[error(transparent)]
    Federation(#[from] ErreurFederation),
    /// Scellement ou écriture impossible : vue en mémoire = dernier état
    /// écrit ; si l'écriture avait commencé, l'instance est hors service.
    #[error(transparent)]
    Etat(#[from] ErreurEtat),
    /// Instance hors service après un échec ou une interruption d'écriture :
    /// recharger ([`FederationPersistante::charger`]).
    #[error("état fédéral hors service après un échec d'écriture : rechargement requis")]
    HorsService,
}

/// Bilan d'un chargement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BilanReprise {
    /// **Toutes** les révocations à publier non acquittées : à inscrire au
    /// registre local puis à acquitter ([`FederationPersistante::acquitter`]).
    pub revocations: Vec<RevocationFederale>,
    /// Révocations inscrites au registre après l'ancre, ré-appliquées (état
    /// plus ancien que le registre : à signaler).
    pub reappliquees: usize,
    /// Aucun état sur le support : vue vide, écrite aussitôt. Anormal si un
    /// état avait déjà été écrit (l'appelant passe alors `etat_attendu`).
    pub premier_usage: bool,
}

/// Vue fédérale dont chaque transition est persistée (voir module).
#[derive(Debug)]
pub struct FederationPersistante<S: SupportEtat> {
    support: S,
    cle: CleEtatFederal,
    federation: Federation,
    /// Octets du dernier état écrit (ou relu tel quel).
    ecrit: Vec<u8>,
    /// Ancre scellée dans le dernier état écrit.
    ancre: AncreRegistre,
    hors_service: bool,
}

impl<S: SupportEtat> FederationPersistante<S> {
    /// Charge la vue depuis `support` (voir module) à l'heure de confiance
    /// `maintenant`, contre le registre local `registre`.
    ///
    /// - absent : `etat_attendu` ⇒ [`ErreurEtat::EtatAbsent`] ; sinon premier
    ///   usage (vue vide, écrite aussitôt pour que l'absence soit ensuite une
    ///   anomalie détectable) ;
    /// - tout refus (illisible, altéré, titre refusé, incohérent, heure en
    ///   recul, ancre incohérente) ⇒ `Err` ;
    /// - vue changée par la reprise (rattrapage, ré-application, registre
    ///   avancé, ancienne version du format) ⇒ réécrite avant de rendre la main.
    pub async fn charger<R: RegistreLocal + Sync + ?Sized>(
        support: S,
        secret_local: &[u8; 32],
        parametres: Parametres,
        origin: Option<[u8; 32]>,
        maintenant: u64,
        registre: &R,
        etat_attendu: bool,
    ) -> Result<(Self, BilanReprise), ErreurEtat> {
        let cle = CleEtatFederal::deriver(secret_local);
        let lu = support.lire().await?;
        let (federation, reappliquees, premier_usage, reecrire, ecrit) = match lu {
            None if etat_attendu => return Err(ErreurEtat::EtatAbsent),
            None => (Federation::new(parametres, origin), 0, true, true, Vec::new()),
            Some(octets) => {
                let r = reprendre_avec_registre(&octets, &cle, parametres, origin, maintenant, registre)?;
                (r.federation, r.reappliquees, false, r.a_reecrire, octets)
            }
        };
        let ancre = AncreRegistre::de(registre);
        let mut moi = Self { support, cle, federation, ecrit, ancre, hors_service: false };
        if reecrire {
            let octets = moi.federation.sceller_etat(&moi.cle, &ancre)?;
            moi.support.ecrire(&octets).await?;
            moi.ecrit = octets;
        }
        let revocations = moi.federation.revocations_a_publier();
        Ok((moi, BilanReprise { revocations, reappliquees, premier_usage }))
    }

    /// Vue courante (lecture seule : toute modification passe par
    /// [`Self::transition`]).
    pub fn federation(&self) -> &Federation {
        &self.federation
    }

    /// Support de stockage.
    pub fn support(&self) -> &S {
        &self.support
    }

    /// Faux après un échec ou une interruption d'écriture (recharger).
    pub fn en_service(&self) -> bool {
        !self.hors_service
    }

    /// Applique `op` à une copie de la vue, persiste la copie si elle a
    /// changé, puis la rend visible (voir module). Le résultat de `op` est
    /// rendu après la persistance.
    pub async fn transition<T, R, F>(&mut self, registre: &R, op: F) -> Result<T, ErreurTransition>
    where
        R: RegistreLocal + Sync + ?Sized,
        F: FnOnce(&mut Federation) -> Result<T, ErreurFederation>,
    {
        if self.hors_service {
            return Err(ErreurTransition::HorsService);
        }
        if !registre.prolonge(&self.ancre) {
            self.hors_service = true;
            return Err(ErreurEtat::RegistreIncoherent.into());
        }
        let mut suivante = self.federation.clone();
        let issue = op(&mut suivante);
        let ancre = AncreRegistre::de(registre);
        let octets = suivante.sceller_etat(&self.cle, &ancre)?;
        if octets != self.ecrit {
            // Hors service tant que l'écriture n'a pas abouti (échec ou futur abandonné).
            self.hors_service = true;
            self.support.ecrire(&octets).await?;
            self.hors_service = false;
            self.ecrit = octets;
            self.ancre = ancre;
        }
        self.federation = suivante;
        issue.map_err(ErreurTransition::from)
    }

    /// Accusé d'inscription : l'entrée `index` du registre local doit être
    /// la révocation fédérale de `kid` (sinon refus, rien n'est acquitté) ;
    /// elle quitte alors la file à publier, état réécrit. `Ok(false)` : elle
    /// n'y était pas.
    pub async fn acquitter<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        kid: &Kid,
        index: u64,
        registre: &R,
    ) -> Result<bool, ErreurTransition> {
        controler_inscription(registre, kid, index)?;
        self.transition(registre, |f| Ok(f.acquitter(kid))).await
    }

    /// [`Federation::ajouter_fondateur`], persisté.
    pub async fn ajouter_fondateur<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        candidature: &[u8],
        maintenant: u64,
    ) -> Result<CogId, ErreurTransition> {
        self.transition(registre, |f| f.ajouter_fondateur(candidature, maintenant)).await
    }

    /// [`Federation::recevoir_candidature`], persisté.
    pub async fn recevoir_candidature<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        candidature: &[u8],
        maintenant: u64,
    ) -> Result<CogId, ErreurTransition> {
        self.transition(registre, |f| f.recevoir_candidature(candidature, maintenant)).await
    }

    /// [`Federation::recevoir_confirmation`], persisté.
    pub async fn recevoir_confirmation<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        octets: &[u8],
        maintenant: u64,
    ) -> Result<(), ErreurTransition> {
        self.transition(registre, |f| f.recevoir_confirmation(octets, maintenant)).await
    }

    /// [`Federation::actualiser`] (statuts, échéances des 72 h), persisté.
    pub async fn actualiser<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        maintenant: u64,
    ) -> Result<Vec<RevocationFederale>, ErreurTransition> {
        self.transition(registre, |f| f.actualiser(maintenant)).await
    }

    /// [`Federation::recevoir_accusation`] (suspension), persisté.
    pub async fn recevoir_accusation<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        octets: &[u8],
        preuve: &crate::Preuve,
        maintenant: u64,
    ) -> Result<SuspensionFederale, ErreurTransition> {
        self.transition(registre, |f| f.recevoir_accusation(octets, preuve, maintenant)).await
    }

    /// [`Federation::recevoir_recours`] (levée), persisté.
    pub async fn recevoir_recours<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        octets: &[u8],
        maintenant: u64,
    ) -> Result<SuspensionFederale, ErreurTransition> {
        self.transition(registre, |f| f.recevoir_recours(octets, maintenant)).await
    }

    /// [`Federation::recevoir_tete`] (vue fédérale), persisté.
    pub async fn recevoir_tete<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        cog: &CogId,
        octets: &[u8],
        coherence: Option<&PreuveCoherence>,
        maintenant: u64,
    ) -> Result<Acceptation, ErreurTransition> {
        self.transition(registre, |f| f.recevoir_tete(cog, octets, coherence, maintenant)).await
    }

    /// [`Federation::temoigner`], persisté (la contresignature n'est rendue
    /// qu'une fois le témoignage écrit : jamais deux têtes contradictoires
    /// contresignées à travers un redémarrage).
    pub async fn temoigner<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        cog: &CogId,
        octets: &[u8],
        coherence: Option<&PreuveCoherence>,
        journal_local: &CleSignature,
        maintenant: u64,
    ) -> Result<Vec<u8>, ErreurTransition> {
        self.transition(registre, |f| f.temoigner(cog, octets, coherence, journal_local, maintenant)).await
    }

    /// [`Federation::observer`] (note locale), persisté.
    pub async fn observer<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        cog: &CogId,
        observation: Observation,
        maintenant: u64,
    ) -> Result<(), ErreurTransition> {
        self.transition(registre, |f| f.observer(cog, observation, maintenant)).await
    }

    /// [`Federation::noter_contact`] (carnet de pairs), persisté.
    pub async fn noter_contact<R: RegistreLocal + Sync + ?Sized>(
        &mut self,
        registre: &R,
        cog: &CogId,
        reseau: &str,
        maintenant: u64,
    ) -> Result<bool, ErreurTransition> {
        self.transition(registre, |f| f.noter_contact(cog, reseau, maintenant)).await
    }
}
