//! @id     maia_federee_base
//! @do     tenir_base_connaissances_etrangere
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « base » (base de connaissances étrangère séparée, admission par la vue fédérale, versions, quotas, plongements locaux).

//! Base de connaissances **étrangère** (IA-FEDEREE §1, §4).
//!
//! - séparée des connaissances nationales et de la mémoire de MAIA : aucune
//!   API n'en exporte le contenu hors d'un [`crate::Etranger`] ;
//! - une Fiche n'entre que vérifiée ([`crate::verifier_fiche`]) et si son
//!   émetteur est **admis** par la vue fédérale (invité ou membre, ni
//!   suspendu, ni révoqué, ni banni, vue fraîche : `Federation::acces_federe`) ;
//!   l'admission est **revérifiée** à chaque recherche (révocation postérieure) ;
//! - aucune modification silencieuse : une version égale au contenu
//!   différent, ou antérieure, est refusée ;
//! - au plus [`MAX_FICHES_PAR_EMETTEUR`] Fiches par émetteur (anti-inondation,
//!   choix à consigner) ;
//! - plongement **recalculé localement** à l'entrée (EX-62) ;
//! - **admission** ([`BaseEtrangere::recevoir`]) : [`crate::verifier_fiche`]
//!   (délégation de la clé `fiches` valide **maintenant** et à la date de la
//!   Fiche, fraîcheur bornée) puis **plancher de génération** : la plus haute
//!   génération de clé `fiches` vue pour un émetteur admis est mémorisée, une
//!   Fiche d'une génération inférieure est refusée (rotation : l'ancienne clé
//!   est retirée) ; le plancher survit au retrait des Fiches de l'émetteur et
//!   s'exporte / se restaure avec l'état ([`BaseEtrangere::planchers`],
//!   [`BaseEtrangere::restaurer_plancher`]) ;
//! - **relecture** ([`BaseEtrangere::relire`]) d'une Fiche déjà admise : elle
//!   garde le verdict de son admission (lisible après l'expiration de sa
//!   délégation) et n'admet rien de neuf.

use std::collections::BTreeMap;

use miyauth::CogId;
use miyufederation::{Federation, Statut};

use crate::erreurs::ErreurFederee;
use crate::fiche::{relire_fiche, verifier_fiche, FicheVerifiee, SignatureFiche};
use crate::plongement::PlongementLocal;
use crate::requete::RequeteTypee;

/// Fiches au plus par émetteur dans la base étrangère.
pub const MAX_FICHES_PAR_EMETTEUR: usize = 1_000;

/// Qualité fédérale d'un émetteur (poids : membre > invité).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Qualite {
    /// Invité : cité, jamais compté pour la corroboration.
    Invite,
    /// Membre : compte pour la corroboration.
    Membre,
}

/// Vue fédérale consultée pour admettre un émetteur.
pub trait VueFederale {
    /// Qualité du COG, ou motif de refus.
    fn qualifier(&self, cog: &CogId, maintenant: u64) -> Result<Qualite, String>;
}

impl VueFederale for Federation {
    fn qualifier(&self, cog: &CogId, maintenant: u64) -> Result<Qualite, String> {
        match self.acces_federe(cog, maintenant) {
            Ok(Statut::Membre) => Ok(Qualite::Membre),
            Ok(Statut::Invite) => Ok(Qualite::Invite),
            Ok(autre) => Err(format!("statut {autre:?}")),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
struct Entree {
    fiche: FicheVerifiee,
    plongement: PlongementLocal,
}

/// Base de connaissances étrangère.
#[derive(Debug, Clone)]
pub struct BaseEtrangere {
    cog_local: CogId,
    fiches: BTreeMap<(CogId, String), Entree>,
    /// Plus haute génération de clé `fiches` vue par émetteur admis.
    planchers: BTreeMap<CogId, u32>,
}

impl BaseEtrangere {
    /// Base vide du COG `cog_local` (ses propres Fiches n'y entrent pas).
    pub fn new(cog_local: CogId) -> Self {
        Self { cog_local, fiches: BTreeMap::new(), planchers: BTreeMap::new() }
    }

    /// Nombre de Fiches.
    pub fn len(&self) -> usize {
        self.fiches.len()
    }

    /// Vrai si vide.
    pub fn is_empty(&self) -> bool {
        self.fiches.is_empty()
    }

    /// Plancher de génération de clé `fiches` de l'émetteur `cog` (`None` :
    /// aucune Fiche déléguée admise).
    pub fn plancher_generation(&self, cog: &CogId) -> Option<u32> {
        self.planchers.get(cog).copied()
    }

    /// Planchers de génération par émetteur, à rendre durables avec l'état de
    /// la base (ordre déterministe).
    pub fn planchers(&self) -> impl Iterator<Item = (&CogId, u32)> {
        self.planchers.iter().map(|(c, g)| (c, *g))
    }

    /// Restaure un plancher relu de l'état durable : le plancher ne fait que
    /// **monter** (le plus haut des deux est gardé).
    pub fn restaurer_plancher(&mut self, cog: CogId, generation: u32) {
        let p = self.planchers.entry(cog).or_insert(generation);
        *p = (*p).max(generation);
    }

    /// Reçoit une Fiche signée (fail-closed) : **admission**. Rend son empreinte.
    ///
    /// Contrôles : [`verifier_fiche`] (fenêtre de la délégation à `maintenant`
    /// et à la date, fraîcheur), Fiche étrangère, émetteur qualifié par la vue
    /// fédérale, génération de clé `fiches` ≥ plancher de l'émetteur (puis
    /// plancher relevé), versions, quota. Une Fiche **identique** déjà admise
    /// est acquittée sans être réadmise, même si elle ne passerait plus
    /// l'admission (relecture, [`Self::relire`]).
    pub fn recevoir(&mut self, octets: &[u8], vue: &dyn VueFederale, maintenant: u64) -> Result<[u8; 32], ErreurFederee> {
        let fiche = match verifier_fiche(octets, maintenant) {
            Ok(f) => f,
            Err(e) => return self.relire(octets, maintenant).map(FicheVerifiee::empreinte).map_err(|_| e),
        };
        if fiche.cog() == &self.cog_local {
            return Err(ErreurFederee::FicheLocale);
        }
        vue.qualifier(fiche.cog(), maintenant).map_err(ErreurFederee::EmetteurRefuse)?;
        if let SignatureFiche::Deleguee { generation, .. } = fiche.signature() {
            if self.plancher_generation(fiche.cog()).is_some_and(|p| *generation < p) {
                return Err(ErreurFederee::Delegation("génération de clé « fiches » antérieure à la dernière vue (rotation)"));
            }
            // Délégation vérifiée, signée par l'identité d'un émetteur admis.
            self.restaurer_plancher(fiche.cog().clone(), *generation);
        }
        let cle = (fiche.cog().clone(), fiche.fiche().id.clone());
        if let Some(existante) = self.fiches.get(&cle) {
            let (v_old, v_new) = (existante.fiche.fiche().version, fiche.fiche().version);
            if v_new == v_old && existante.fiche.empreinte() == fiche.empreinte() {
                return Ok(fiche.empreinte());
            }
            if v_new <= v_old {
                return Err(ErreurFederee::VersionObsolete);
            }
        } else if self.fiches.keys().filter(|(c, _)| c == fiche.cog()).count() >= MAX_FICHES_PAR_EMETTEUR {
            return Err(ErreurFederee::BasePleine);
        }
        let f = fiche.fiche();
        let plongement = PlongementLocal::calculer(&format!("{} {}", f.sujet, f.texte));
        let empreinte = fiche.empreinte();
        self.fiches.insert(cle, Entree { fiche, plongement });
        Ok(empreinte)
    }

    /// **Relit** une Fiche déjà admise (octets relus d'un stockage, d'un pair
    /// qui la renvoie…) : signature revérifiée par [`relire_fiche`] (délégation
    /// couvrant la date de la Fiche, sans exigence sur `maintenant`), puis
    /// présence **à l'identique** (même empreinte) dans la base. Rend l'entrée
    /// admise : le verdict d'admission enregistré, rien de neuf.
    pub fn relire(&self, octets: &[u8], maintenant: u64) -> Result<&FicheVerifiee, ErreurFederee> {
        let lue = relire_fiche(octets, maintenant)?;
        match self.fiches.get(&(lue.cog().clone(), lue.fiche().id.clone())) {
            Some(e) if e.fiche.empreinte() == lue.empreinte() => Ok(&e.fiche),
            _ => Err(ErreurFederee::NonAdmise),
        }
    }

    /// Retire toutes les Fiches d'un émetteur (blacklist, note locale). Son
    /// plancher de génération est gardé.
    pub fn retirer_emetteur(&mut self, cog: &CogId) -> usize {
        let avant = self.fiches.len();
        self.fiches.retain(|(c, _), _| c != cog);
        avant - self.fiches.len()
    }

    /// Fiches pertinentes pour la requête, émetteurs **re-qualifiés** à
    /// l'instant `maintenant`, triées (score de termes, similarité locale,
    /// puis émetteur et identifiant : déterministe), au plus `requete.max()`.
    pub fn rechercher(&self, requete: &RequeteTypee, vue: &dyn VueFederale, maintenant: u64) -> Vec<(&FicheVerifiee, Qualite)> {
        let q = PlongementLocal::calculer(requete.sujet());
        let mut trouvees: Vec<(usize, f32, &FicheVerifiee, Qualite)> = self
            .fiches
            .values()
            .filter_map(|e| {
                let score = requete.score(&e.fiche);
                if score == 0 {
                    return None;
                }
                let qualite = vue.qualifier(e.fiche.cog(), maintenant).ok()?;
                Some((score, q.similarite(&e.plongement), &e.fiche, qualite))
            })
            .collect();
        trouvees.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| b.1.total_cmp(&a.1))
                .then_with(|| a.2.cog().cmp(b.2.cog()))
                .then_with(|| a.2.fiche().id.cmp(&b.2.fiche().id))
        });
        trouvees.into_iter().take(usize::from(requete.max())).map(|(_, _, f, q)| (f, q)).collect()
    }
}

#[cfg(test)]
pub(crate) mod essais {
    //! Vue fédérale factice pour les tests de la crate.
    use super::*;

    #[derive(Default)]
    pub(crate) struct VueFigee(pub BTreeMap<CogId, Qualite>);

    impl VueFederale for VueFigee {
        fn qualifier(&self, cog: &CogId, _: u64) -> Result<Qualite, String> {
            self.0.get(cog).copied().ok_or_else(|| "inconnu".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::essais::VueFigee;
    use super::*;
    use crate::fiche::essais::fiche;
    use crate::fiche::{sceller_fiche, Fiche, SignataireFiches, FRAICHEUR_ADMISSION_SECS, INDICE_FICHES};
    use miyusceau::{CleSignature, DelegationCle};

    const T: u64 = 1_800_000_100;

    fn cog_de(cle: &CleSignature) -> CogId {
        CogId::new(&miyuracine::cog_id_depuis_cle(&cle.cle_publique())).unwrap()
    }

    #[test]
    fn admission_versions_et_retrait() {
        let local = CleSignature::generer();
        let a = CleSignature::generer();
        let inconnu = CleSignature::generer();
        let mut vue = VueFigee::default();
        vue.0.insert(cog_de(&a), Qualite::Membre);
        let mut base = BaseEtrangere::new(cog_de(&local));

        let f1 = fiche(&a, "geo.paris", "Paris est la capitale de la France.", &[]);
        let o1 = sceller_fiche(&f1, &a).unwrap();
        base.recevoir(&o1, &vue, T).unwrap();
        base.recevoir(&o1, &vue, T).unwrap(); // idempotent
        // Même version, contenu différent : modification silencieuse refusée.
        let f1b = Fiche { texte: "Lyon est la capitale.".into(), ..f1.clone() };
        assert_eq!(base.recevoir(&sceller_fiche(&f1b, &a).unwrap(), &vue, T), Err(ErreurFederee::VersionObsolete));
        let f2 = Fiche { version: 2, ..f1b };
        base.recevoir(&sceller_fiche(&f2, &a).unwrap(), &vue, T).unwrap();
        assert_eq!(base.recevoir(&o1, &vue, T), Err(ErreurFederee::VersionObsolete));

        let fi = fiche(&inconnu, "x", "texte", &[]);
        assert!(matches!(base.recevoir(&sceller_fiche(&fi, &inconnu).unwrap(), &vue, T), Err(ErreurFederee::EmetteurRefuse(_))));
        let fl = fiche(&local, "x", "texte", &[]);
        assert_eq!(base.recevoir(&sceller_fiche(&fl, &local).unwrap(), &vue, T), Err(ErreurFederee::FicheLocale));

        let r = RequeteTypee::nouvelle("capitale", &[], None, 5).unwrap();
        assert_eq!(base.rechercher(&r, &vue, T).len(), 1);
        // Émetteur révoqué après réception : exclu des recherches.
        let vide = VueFigee::default();
        assert!(base.rechercher(&r, &vide, T).is_empty());
        assert_eq!(base.retirer_emetteur(&cog_de(&a)), 1);
        assert!(base.is_empty());
    }

    // ── Clé `fiches` déléguée : admission, plancher de génération, relecture ──

    const JOUR: u64 = 86_400;

    /// Signataire `fiches` de génération `generation` de `identite`, délégation sur [debut, fin].
    fn signataire(identite: &CleSignature, generation: u32, debut: u64, fin: u64, maintenant: u64) -> SignataireFiches {
        let fiches = CleSignature::generer();
        let cog = miyuracine::cog_id_depuis_cle(&identite.cle_publique());
        let d = DelegationCle::nouvelle(&cog, INDICE_FICHES, generation, fiches.cle_publique(), debut, fin)
            .unwrap()
            .sceller(identite)
            .unwrap();
        SignataireFiches::deleguee(fiches, identite.cle_publique(), d, maintenant).unwrap()
    }

    fn fiche_datee(identite: &CleSignature, id: &str, date: u64) -> Fiche {
        Fiche { date, ..fiche(identite, id, "Paris est la capitale de la France.", &[]) }
    }

    fn base_et_vue(a: &CleSignature) -> (BaseEtrangere, VueFigee) {
        let mut vue = VueFigee::default();
        vue.0.insert(cog_de(a), Qualite::Membre);
        (BaseEtrangere::new(cog_de(&CleSignature::generer())), vue)
    }

    #[test]
    fn admission_fiche_antidatee_dans_une_fenetre_expiree_refusee() {
        let a = CleSignature::generer();
        let (mut base, vue) = base_et_vue(&a);
        // Délégation [T-30j, T-10j] ; la clé `fiches` est compromise après son expiration.
        let s = signataire(&a, 0, T - 30 * JOUR, T - 10 * JOUR, T - 20 * JOUR);
        let antidatee = s.sceller(&fiche_datee(&a, "geo.paris", T - 20 * JOUR), T - 20 * JOUR).unwrap();
        assert!(matches!(base.recevoir(&antidatee, &vue, T), Err(ErreurFederee::Delegation(_))));
        assert!(base.is_empty());
        assert_eq!(base.plancher_generation(&cog_de(&a)), None);
    }

    #[test]
    fn admission_fiche_plus_ancienne_que_la_borne_refusee() {
        let a = CleSignature::generer();
        let (mut base, vue) = base_et_vue(&a);
        let s = signataire(&a, 0, T - 60 * JOUR, T + 30 * JOUR, T);
        let vieille = fiche_datee(&a, "geo.paris", T - FRAICHEUR_ADMISSION_SECS - 1);
        let octets = s.sceller(&vieille, T).unwrap();
        assert_eq!(base.recevoir(&octets, &vue, T), Err(ErreurFederee::FichePerimee));
        assert!(base.is_empty());
        let fraiche = s.sceller(&fiche_datee(&a, "geo.paris", T - JOUR), T).unwrap();
        base.recevoir(&fraiche, &vue, T).unwrap();
        assert_eq!(base.len(), 1);
    }

    #[test]
    fn plancher_de_generation_apres_rotation() {
        let a = CleSignature::generer();
        let (mut base, vue) = base_et_vue(&a);
        let cog = cog_de(&a);
        let g0 = signataire(&a, 0, T - JOUR, T + 60 * JOUR, T);
        let g1 = signataire(&a, 1, T - JOUR, T + 60 * JOUR, T);
        let g2 = signataire(&a, 2, T - JOUR, T + 60 * JOUR, T);
        base.recevoir(&g0.sceller(&fiche_datee(&a, "a", T), T).unwrap(), &vue, T).unwrap();
        assert_eq!(base.plancher_generation(&cog), Some(0));
        base.recevoir(&g1.sceller(&fiche_datee(&a, "b", T), T).unwrap(), &vue, T).unwrap();
        assert_eq!(base.plancher_generation(&cog), Some(1));
        // Ancienne génération, encore dans sa fenêtre : refusée après la rotation.
        let ancienne = g0.sceller(&fiche_datee(&a, "c", T), T).unwrap();
        assert!(matches!(base.recevoir(&ancienne, &vue, T), Err(ErreurFederee::Delegation(_))));
        // Nouvelle version d'une Fiche déjà admise, signée par l'ancienne clé : refusée aussi.
        let v2 = Fiche { version: 2, ..fiche_datee(&a, "a", T) };
        assert!(matches!(base.recevoir(&g0.sceller(&v2, T).unwrap(), &vue, T), Err(ErreurFederee::Delegation(_))));
        // Même génération : admise.
        base.recevoir(&g1.sceller(&fiche_datee(&a, "d", T), T).unwrap(), &vue, T).unwrap();
        assert_eq!(base.len(), 3);
        // Le plancher survit au retrait de l'émetteur et ne fait que monter.
        base.retirer_emetteur(&cog);
        assert!(matches!(base.recevoir(&ancienne, &vue, T), Err(ErreurFederee::Delegation(_))));
        base.restaurer_plancher(cog.clone(), 0);
        assert_eq!(base.plancher_generation(&cog), Some(1));
        base.restaurer_plancher(cog.clone(), 2);
        assert_eq!(base.planchers().collect::<Vec<_>>(), vec![(&cog, 2)]);
        assert!(matches!(
            base.recevoir(&g1.sceller(&fiche_datee(&a, "e", T), T).unwrap(), &vue, T),
            Err(ErreurFederee::Delegation(_))
        ));
        base.recevoir(&g2.sceller(&fiche_datee(&a, "e", T), T).unwrap(), &vue, T).unwrap();
        // Un émetteur non admis ne relève aucun plancher.
        let inconnu = CleSignature::generer();
        let gi = signataire(&inconnu, 7, T - JOUR, T + JOUR, T);
        assert!(base.recevoir(&gi.sceller(&fiche_datee(&inconnu, "x", T), T).unwrap(), &vue, T).is_err());
        assert_eq!(base.plancher_generation(&cog_de(&inconnu)), None);
    }

    #[test]
    fn fiche_admise_relue_apres_expiration_de_sa_delegation() {
        let a = CleSignature::generer();
        let (mut base, vue) = base_et_vue(&a);
        let s = signataire(&a, 0, T - JOUR, T + 10 * JOUR, T);
        let octets = s.sceller(&fiche_datee(&a, "geo.paris", T), T).unwrap();
        let empreinte = base.recevoir(&octets, &vue, T).unwrap();
        let apres = T + 200 * JOUR;
        // Relecture : verdict d'admission conservé, toujours lisible et recherchable.
        assert_eq!(base.relire(&octets, apres).unwrap().empreinte(), empreinte);
        let r = RequeteTypee::nouvelle("capitale", &[], None, 5).unwrap();
        assert_eq!(base.rechercher(&r, &vue, apres).len(), 1);
        // Renvoyée par un pair : acquittée, rien de réadmis.
        assert_eq!(base.recevoir(&octets, &vue, apres), Ok(empreinte));
        assert_eq!(base.len(), 1);
        // Une autre Fiche de la même clé, antidatée dans l'ancienne fenêtre : refusée,
        // et la relecture ne l'admet pas.
        let autre = s.sceller(&fiche_datee(&a, "geo.lyon", T + JOUR), T + JOUR).unwrap();
        assert!(matches!(base.recevoir(&autre, &vue, apres), Err(ErreurFederee::Delegation(_))));
        assert_eq!(base.relire(&autre, apres).unwrap_err(), ErreurFederee::NonAdmise);
        assert_eq!(base.len(), 1);
        // Même (cog, id), contenu différent antidaté : refusé, l'entrée admise reste.
        let v2 = s.sceller(&Fiche { version: 2, ..fiche_datee(&a, "geo.paris", T) }, T).unwrap();
        assert!(base.recevoir(&v2, &vue, apres).is_err());
        assert_eq!(base.relire(&v2, apres).unwrap_err(), ErreurFederee::NonAdmise);
        assert_eq!(base.relire(&octets, apres).unwrap().fiche().version, 1);
    }
}
