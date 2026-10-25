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
//! - plongement **recalculé localement** à l'entrée (EX-62).

use std::collections::BTreeMap;

use miyauth::CogId;
use miyufederation::{Federation, Statut};

use crate::erreurs::ErreurFederee;
use crate::fiche::{verifier_fiche, FicheVerifiee};
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
}

impl BaseEtrangere {
    /// Base vide du COG `cog_local` (ses propres Fiches n'y entrent pas).
    pub fn new(cog_local: CogId) -> Self {
        Self { cog_local, fiches: BTreeMap::new() }
    }

    /// Nombre de Fiches.
    pub fn len(&self) -> usize {
        self.fiches.len()
    }

    /// Vrai si vide.
    pub fn is_empty(&self) -> bool {
        self.fiches.is_empty()
    }

    /// Reçoit une Fiche signée (fail-closed). Rend son empreinte.
    pub fn recevoir(&mut self, octets: &[u8], vue: &dyn VueFederale, maintenant: u64) -> Result<[u8; 32], ErreurFederee> {
        let fiche = verifier_fiche(octets, maintenant)?;
        if fiche.cog() == &self.cog_local {
            return Err(ErreurFederee::FicheLocale);
        }
        vue.qualifier(fiche.cog(), maintenant).map_err(ErreurFederee::EmetteurRefuse)?;
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

    /// Retire toutes les Fiches d'un émetteur (blacklist, note locale).
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
    use crate::fiche::{sceller_fiche, Fiche};
    use miyusceau::CleSignature;

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
}
