//! @id     maia_federee_corroboration
//! @do     corroborer_par_k_cog_independants
//! @role   securite
//! @layer  toolkit
//! @human  maia-federee — module « corroboration » (« établi » si ≥ k membres distincts concordent, sinon « source unique » ; plafond d'extraits par source ; EX-61).

//! Corroboration et plafond par source (IA-FEDEREE §4, EX-61).
//!
//! - **établi** seulement si ≥ *k* COG **membres distincts** (identités
//!   fédérales distinctes) rendent la même relation (clé de comparaison
//!   normalisée) ; sinon **source unique** ;
//! - un **invité** est cité mais ne compte jamais pour *k* (poids membre > invité) ;
//! - plusieurs Fiches d'un même COG comptent pour **un** ;
//! - une Fiche **republiée** (`reprises`, A-009) est citée mais ne compte jamais ;
//! - **plafond** : au plus N extraits par COG et par réponse (anti-inondation).
//!
//! Choix consignés (A-009) : *k* = 3 par défaut (2 à 16, durcissable),
//! N = 3 par défaut (1 à 10) ; l'indépendance se limite aux identités
//! fédérales distinctes (pas encore d'analyse d'opérateur commun).

use std::collections::{BTreeMap, BTreeSet};

use miyauth::CogId;

use crate::base::Qualite;
use crate::erreurs::ErreurFederee;
use crate::fiche::Relation;
use crate::lecteur::{Ancrage, ExtraitType};
use crate::normalisation::cle_comparaison;

/// *k* par défaut : COG membres distincts requis pour « établi ».
pub const K_DEFAUT: usize = 3;
/// Plafond par défaut d'extraits par COG et par réponse.
pub const PLAFOND_DEFAUT: usize = 3;

/// Politique de corroboration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolitiqueCorroboration {
    k: usize,
    plafond: usize,
}

impl Default for PolitiqueCorroboration {
    fn default() -> Self {
        Self { k: K_DEFAUT, plafond: PLAFOND_DEFAUT }
    }
}

impl PolitiqueCorroboration {
    /// Politique validée : `k ≥ 2`, `1 ≤ plafond ≤ 10`.
    pub fn new(k: usize, plafond: usize) -> Result<Self, ErreurFederee> {
        if !(2..=16).contains(&k) {
            return Err(ErreurFederee::ParametreInvalide("k"));
        }
        if !(1..=10).contains(&plafond) {
            return Err(ErreurFederee::ParametreInvalide("plafond"));
        }
        Ok(Self { k, plafond })
    }

    /// *k*.
    pub fn k(&self) -> usize {
        self.k
    }

    /// Plafond par source.
    pub fn plafond(&self) -> usize {
        self.plafond
    }
}

/// Source citée d'une affirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// COG émetteur.
    pub cog: CogId,
    /// Identifiant de la Fiche.
    pub fiche: String,
    /// Version de la Fiche.
    pub version: u64,
    /// Empreinte des octets signés.
    pub empreinte: [u8; 32],
    /// Qualité fédérale de l'émetteur au moment de la réponse.
    pub qualite: Qualite,
    /// Ancrage de l'extrait dans la Fiche.
    pub ancrage: Ancrage,
    /// Fiche republiée (`reprises`) : citée, jamais comptée pour *k* (A-009).
    pub reprise: bool,
}

/// Degré d'établissement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Etablissement {
    /// ≥ *k* membres distincts concordent.
    Etabli {
        /// Membres distincts concordants.
        confirmateurs: usize,
    },
    /// Moins de *k* membres distincts.
    SourceUnique,
}

/// Affirmation corroborée (contenu étranger : rendu délimité uniquement).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Affirmation {
    /// Relation (forme de la première source).
    pub relation: Relation,
    /// Sources, une par COG.
    pub sources: Vec<Source>,
    /// Établissement.
    pub etablissement: Etablissement,
}

/// Corrobore des extraits (dans l'ordre de pertinence) : plafond par COG,
/// regroupement par relation, comptage des membres distincts.
pub fn corroborer(extraits: Vec<(Source, ExtraitType)>, politique: &PolitiqueCorroboration) -> Vec<Affirmation> {
    let mut par_cog: BTreeMap<CogId, usize> = BTreeMap::new();
    let mut ordre: Vec<(String, String, String)> = Vec::new();
    let mut groupes: BTreeMap<(String, String, String), Affirmation> = BTreeMap::new();
    for (source, extrait) in extraits {
        let compte = par_cog.entry(source.cog.clone()).or_insert(0);
        if *compte >= politique.plafond {
            continue;
        }
        let r = &extrait.relation;
        let cle = (cle_comparaison(&r.sujet), cle_comparaison(&r.predicat), cle_comparaison(&r.objet));
        let groupe = groupes.entry(cle.clone()).or_insert_with(|| {
            ordre.push(cle.clone());
            Affirmation { relation: extrait.relation.clone(), sources: Vec::new(), etablissement: Etablissement::SourceUnique }
        });
        if groupe.sources.iter().any(|s| s.cog == source.cog) {
            continue;
        }
        *compte += 1;
        groupe.sources.push(source);
    }
    ordre
        .into_iter()
        .filter_map(|cle| groupes.remove(&cle))
        .map(|mut a| {
            let membres: BTreeSet<&CogId> =
                a.sources.iter().filter(|s| s.qualite == Qualite::Membre && !s.reprise).map(|s| &s.cog).collect();
            if membres.len() >= politique.k {
                a.etablissement = Etablissement::Etabli { confirmateurs: membres.len() };
            }
            a
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cog(g: u8) -> CogId {
        CogId::new(&miyuracine::cog_id_depuis_cle(&[g; 32])).unwrap()
    }

    #[allow(clippy::many_single_char_names)]
    fn ex(g: u8, q: Qualite, s: &str, p: &str, o: &str) -> (Source, ExtraitType) {
        (
            Source { cog: cog(g), fiche: format!("f{g}"), version: 1, empreinte: [g; 32], qualite: q, ancrage: Ancrage::Texte, reprise: false },
            ExtraitType { relation: Relation::normalisee(s, p, o).unwrap(), ancrage: Ancrage::Texte },
        )
    }

    #[test]
    fn k_membres_distincts_requis() {
        let p = PolitiqueCorroboration::default();
        let a = corroborer(
            vec![
                ex(1, Qualite::Membre, "Paris", "capitale de", "France"),
                ex(2, Qualite::Membre, "paris", "Capitale de", "France."),
                ex(2, Qualite::Membre, "Paris", "capitale de", "France"), // même COG : une fois
                ex(3, Qualite::Invite, "Paris", "capitale de", "France"), // invité : cité, non compté
            ],
            &p,
        );
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].sources.len(), 3);
        assert_eq!(a[0].etablissement, Etablissement::SourceUnique);
        let a = corroborer(
            (1..=3).map(|g| ex(g, Qualite::Membre, "Paris", "capitale de", "France")).collect(),
            &p,
        );
        assert_eq!(a[0].etablissement, Etablissement::Etabli { confirmateurs: 3 });
    }

    #[test]
    fn reprise_citee_jamais_comptee() {
        let p = PolitiqueCorroboration::default();
        let mut extraits: Vec<_> = (1..=3).map(|g| ex(g, Qualite::Membre, "Paris", "capitale de", "France")).collect();
        extraits[2].0.reprise = true;
        let a = corroborer(extraits, &p);
        assert_eq!(a[0].sources.len(), 3);
        assert_eq!(a[0].etablissement, Etablissement::SourceUnique);
    }

    #[test]
    fn plafond_par_source() {
        let p = PolitiqueCorroboration::new(2, 2).unwrap();
        let a = corroborer((0..10).map(|i| ex(7, Qualite::Membre, &format!("s{i}"), "p", "o")).collect(), &p);
        assert_eq!(a.len(), 2);
        assert!(PolitiqueCorroboration::new(1, 3).is_err());
        assert!(PolitiqueCorroboration::new(3, 0).is_err());
    }
}
