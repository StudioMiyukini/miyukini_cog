//! @id     miyuregistre_federal
//! @do     suivre_registres_federes_temoignes
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « federal » (têtes des COG fédérés contresignées par ≥ 3 membres témoins, Origin facultatif, anti-éclipse).

//! Registre fédéral (EX-34 « registre Merkle fédéral contresigné, anti-éclipse » ;
//! lot 2 §8 « un journal par COG » ; §8.3).
//!
//! Vue fédérale = l'ensemble des registres des COG reconnus, chacun suivi par
//! un [`SuiviTetes`]. Une tête fédérale n'est acceptée que :
//!
//! - signée par la **clé de journal du COG propriétaire** ;
//! - **et** contresignée par **≥ 3 témoins distincts** ([`TEMOINS_MIN`]) :
//!   clés de journal des **membres actuels** (fournies par l'appelant, la
//!   fédération décidant qui est membre), **Origin** comptant pour **un**
//!   témoin au plus s'il est configuré. La clé du propriétaire n'est jamais
//!   comptée comme témoin.
//!
//! Règle unique ([`verifier_temoins`]) partagée avec `miyufederation`
//! (qui dépend de ce module : aucun cycle).
//!
//! Un pair isolé (éclipse) ne peut donc pas présenter une histoire que la
//! fédération n'a pas vue ; une équivoque produit une preuve signée par le
//! propriétaire. Toutes les têtes acceptées alimentent l'[`HorlogeConfiance`].
//!
//! Amendé **A-007** (décision du propriétaire, EX-30 prime sur A-006) :
//! ~~2 sur {clé de journal du COG, Origin}~~ → ≥ 3 témoins membres distincts,
//! Origin = un témoin. Consigné par **A-006** : contresignature = cosignature
//! du même sceau, amorçage d'un COG au premier usage (TOFU). **À décider**
//! (A-006) : journal fédéral agrégé tenu par Origin.

use std::collections::BTreeMap;

use miyusceau::{
    verifier_seuil, Acceptation, AncresDeConfiance, ErreurSceau, NiveauConfiance, PolitiqueTete, PreuveCoherence,
    TeteRegistre, MAX_SIGNATURES,
};

use crate::erreurs::ErreurJournal;
use crate::suivi::SuiviTetes;
use crate::temps::HorlogeConfiance;

/// Témoins distincts minimaux d'une tête fédérale (A-007, EX-34).
pub const TEMOINS_MIN: usize = 3;

/// Compte les témoins d'une tête fédérale et exige `requis` témoins distincts
/// (`TEMOINS_MIN..MAX_SIGNATURES` : la tête porte aussi la signature du propriétaire).
///
/// - `proprietaire` : clé de journal du COG dont c'est la tête (jamais témoin) ;
/// - `membres` : clés de journal des membres actuels (choisis par l'appelant) ;
/// - `origin` : clé de journal d'Origin, **un** témoin au plus.
///
/// Ne vérifie ni la signature du propriétaire, ni le registre, ni la
/// fraîcheur, ni la monotonie (voir [`SuiviTetes`] / `verifier_tete`).
/// Une signature épinglée invalide = refus du sceau entier.
/// Renvoie le nombre de témoins valides comptés.
pub fn verifier_temoins<'a>(
    sceau: &[u8],
    proprietaire: &[u8; 32],
    membres: impl IntoIterator<Item = &'a [u8; 32]>,
    origin: Option<&[u8; 32]>,
    requis: usize,
    maintenant: u64,
) -> Result<usize, ErreurJournal> {
    if !(TEMOINS_MIN..MAX_SIGNATURES).contains(&requis) {
        return Err(ErreurJournal::Sceau(ErreurSceau::SeuilInvalide));
    }
    let mut temoins = AncresDeConfiance::new();
    // `kid` uniques dans un sceau et dans les ancres : chaque clé compte une
    // fois (Origin aussi, même s'il figurait parmi les membres).
    for cle in membres {
        if cle != proprietaire {
            temoins.ajouter(cle, NiveauConfiance::Reconnu)?;
        }
    }
    if let Some(cle) = origin.filter(|c| *c != proprietaire) {
        temoins.ajouter(cle, NiveauConfiance::Reconnu)?;
    }
    match verifier_seuil::<TeteRegistre>(sceau, &temoins, requis, maintenant) {
        Ok(v) => Ok(v.signataires().len()),
        Err(ErreurSceau::SignataireInconnu) => Err(ErreurJournal::TemoinsInsuffisants { requis, obtenu: 0 }),
        Err(ErreurSceau::SeuilNonAtteint { obtenu, .. }) => Err(ErreurJournal::TemoinsInsuffisants { requis, obtenu }),
        Err(e) => Err(e.into()),
    }
}

/// Suivi des registres des COG fédérés.
#[derive(Debug, Clone)]
pub struct RegistreFederal {
    origin: Option<[u8; 32]>,
    temoins_min: usize,
    registres: BTreeMap<[u8; 32], Inscrit>,
}

/// Registre d'un COG inscrit : sa clé de journal (jamais témoin) et son suivi.
#[derive(Debug, Clone)]
struct Inscrit {
    journal: [u8; 32],
    suivi: SuiviTetes,
}

impl RegistreFederal {
    /// Vue fédérale vide ; `origin` = clé de journal d'Origin comme témoin
    /// facultatif (`None` : fédération sans Origin, EX-30). Exige
    /// [`TEMOINS_MIN`] témoins.
    pub fn new(origin: Option<[u8; 32]>) -> Self {
        Self { origin, temoins_min: TEMOINS_MIN, registres: BTreeMap::new() }
    }

    /// Durcit le nombre de témoins exigés (`3..=7`).
    pub fn avec_temoins(mut self, n: usize) -> Result<Self, ErreurJournal> {
        if !(TEMOINS_MIN..MAX_SIGNATURES).contains(&n) {
            return Err(ErreurJournal::Sceau(ErreurSceau::SeuilInvalide));
        }
        self.temoins_min = n;
        Ok(self)
    }

    /// Témoins exigés.
    pub fn temoins_min(&self) -> usize {
        self.temoins_min
    }

    /// Inscrit le registre d'un COG reconnu (`journal` = sa clé de journal,
    /// distincte de celle d'Origin). Politique du propriétaire : 1 signature
    /// (sa clé de journal), âge maximal `age_max_secs` ; les témoins sont
    /// exigés à l'acceptation.
    pub fn inscrire(
        &mut self,
        registre: [u8; 32],
        journal: &[u8; 32],
        age_max_secs: u64,
    ) -> Result<(), ErreurJournal> {
        if self.origin.as_ref() == Some(journal) {
            return Err(ErreurJournal::Sceau(ErreurSceau::CleInvalide));
        }
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(journal, NiveauConfiance::Reconnu)?;
        let politique = PolitiqueTete::nouvelle(1, age_max_secs)?;
        let journal = *journal;
        self.registres
            .entry(registre)
            .or_insert_with(|| Inscrit { journal, suivi: SuiviTetes::new(registre, ancres, politique) });
        Ok(())
    }

    /// Retire un COG (fin de reconnaissance, sanction).
    pub fn retirer(&mut self, registre: &[u8; 32]) -> bool {
        self.registres.remove(registre).is_some()
    }

    /// Accepte une tête d'un COG inscrit : signature du propriétaire,
    /// registre, fraîcheur, Median-Time-Past et monotonie d'abord (une
    /// équivoque signée par le propriétaire est une preuve même sans
    /// témoins), puis ≥ `temoins_min` témoins parmi `membres` (clés de journal
    /// des membres actuels) et Origin. Rien n'est modifié en cas de refus.
    pub fn accepter<'a>(
        &mut self,
        registre: &[u8; 32],
        sceau: &[u8],
        coherence: Option<&PreuveCoherence>,
        membres: impl IntoIterator<Item = &'a [u8; 32]>,
        horloge: &mut HorlogeConfiance,
        maintenant: u64,
    ) -> Result<Acceptation, ErreurJournal> {
        let inscrit = self.registres.get_mut(registre).ok_or(ErreurJournal::RegistreInconnu)?;
        let preparee = inscrit.suivi.preparer(sceau, coherence, maintenant)?;
        verifier_temoins(sceau, &inscrit.journal, membres, self.origin.as_ref(), self.temoins_min, maintenant)?;
        Ok(inscrit.suivi.appliquer(preparee, horloge))
    }

    /// Suivi d'un COG.
    pub fn suivi(&self, registre: &[u8; 32]) -> Option<&SuiviTetes> {
        self.registres.get(registre).map(|i| &i.suivi)
    }

    /// Dernières têtes fraîches de tous les COG (les registres gelés sont omis).
    pub fn tetes_fraiches(&self, maintenant: u64) -> Vec<&TeteRegistre> {
        self.registres.values().filter_map(|i| i.suivi.tete_fraiche(maintenant).ok()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::Journal;
    use crate::stockage::StockageMemoire;
    use miyusceau::{cosigner, CleSignature, ErreurRegistre};

    const T0: u64 = 1_800_000_000;

    struct Banc {
        fed: RegistreFederal,
        journal_cle: CleSignature,
        membres: Vec<CleSignature>,
        origin: CleSignature,
        j: Journal<StockageMemoire>,
        horloge: HorlogeConfiance,
    }

    fn banc(avec_origin: bool) -> Banc {
        let origin = CleSignature::generer();
        let journal_cle = CleSignature::generer();
        let mut fed = RegistreFederal::new(avec_origin.then(|| origin.cle_publique()));
        fed.inscrire([8; 32], &journal_cle.cle_publique(), 86_400).unwrap();
        let j = Journal::ouvrir([8; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        let membres = (0..4).map(|_| CleSignature::generer()).collect();
        Banc { fed, journal_cle, membres, origin, j, horloge: HorlogeConfiance::new() }
    }

    fn temoigner(sceau: &[u8], cles: &[&CleSignature]) -> Vec<u8> {
        cles.iter().fold(sceau.to_vec(), |s, c| cosigner::<TeteRegistre>(&s, c).unwrap())
    }

    impl Banc {
        fn journaux(&self) -> Vec<[u8; 32]> {
            self.membres.iter().map(CleSignature::cle_publique).collect()
        }

        fn accepter(&mut self, sceau: &[u8], t: u64) -> Result<Acceptation, ErreurJournal> {
            let journaux = self.journaux();
            self.fed.accepter(&[8; 32], sceau, None, &journaux, &mut self.horloge, t)
        }
    }

    #[test]
    fn trois_temoins_membres_sans_origin() {
        let mut b = banc(false);
        let seule = b.j.emettre_tete(&[&b.journal_cle], T0, 86_400).unwrap();
        assert!(matches!(b.accepter(&seule, T0), Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 0 })));
        let deux = temoigner(&seule, &[&b.membres[0], &b.membres[1]]);
        assert!(matches!(b.accepter(&deux, T0), Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 2 })));
        // Refus sans effet : aucune tête retenue, horloge intacte.
        assert!(b.fed.suivi(&[8; 32]).unwrap().etat().is_none());
        assert_eq!(b.horloge.mtp(), None);
        let trois = temoigner(&deux, &[&b.membres[2]]);
        assert_eq!(b.accepter(&trois, T0).unwrap(), Acceptation::Nouvelle);
        assert_eq!(b.fed.tetes_fraiches(T0).len(), 1);
        assert_eq!(b.fed.tetes_fraiches(T0 + 3 * 86_400).len(), 0);
        assert_eq!(b.horloge.mtp(), Some(T0));
        let journaux = b.journaux();
        assert!(matches!(
            b.fed.accepter(&[9; 32], &trois, None, &journaux, &mut b.horloge, T0),
            Err(ErreurJournal::RegistreInconnu)
        ));
    }

    #[test]
    fn temoins_non_membres_ignores() {
        let mut b = banc(false);
        let seule = b.j.emettre_tete(&[&b.journal_cle], T0, 86_400).unwrap();
        let inconnu = CleSignature::generer();
        let h = temoigner(&seule, &[&b.membres[0], &b.membres[1], &inconnu]);
        assert!(matches!(b.accepter(&h, T0), Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 2 })));
        // Le propriétaire n'est jamais son propre témoin, même s'il est membre.
        let mut journaux = b.journaux();
        journaux.truncate(2);
        journaux.push(b.journal_cle.cle_publique());
        let h2 = temoigner(&seule, &[&b.membres[0], &b.membres[1]]);
        assert!(matches!(
            b.fed.accepter(&[8; 32], &h2, None, &journaux, &mut b.horloge, T0),
            Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 2 })
        ));
    }

    #[test]
    fn origin_compte_pour_un_temoin() {
        let mut b = banc(true);
        let seule = b.j.emettre_tete(&[&b.journal_cle], T0, 86_400).unwrap();
        // Origin seul : refusé.
        let par_origin = temoigner(&seule, &[&b.origin]);
        assert!(matches!(
            b.accepter(&par_origin, T0),
            Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 1 })
        ));
        // Origin + 1 membre : refusé.
        let o1 = temoigner(&par_origin, &[&b.membres[0]]);
        assert!(matches!(b.accepter(&o1, T0), Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 2 })));
        // Origin + 2 membres : accepté.
        let o2 = temoigner(&o1, &[&b.membres[1]]);
        assert_eq!(b.accepter(&o2, T0).unwrap(), Acceptation::Nouvelle);
        // Origin listé aussi parmi les membres : compté une seule fois.
        let suivante = b.j.emettre_tete(&[&b.journal_cle], T0 + 10, 86_400).unwrap();
        let h = temoigner(&suivante, &[&b.origin, &b.membres[0]]);
        let mut journaux = b.journaux();
        journaux.push(b.origin.cle_publique());
        assert!(matches!(
            b.fed.accepter(&[8; 32], &h, None, &journaux, &mut b.horloge, T0 + 10),
            Err(ErreurJournal::TemoinsInsuffisants { requis: 3, obtenu: 2 })
        ));
    }

    #[test]
    fn signature_du_proprietaire_exigee() {
        let mut b = banc(true);
        // Tête signée par Origin seul (pas par le propriétaire) puis témoignée.
        let par_origin = b.j.emettre_tete(&[&b.origin], T0, 86_400).unwrap();
        let h = temoigner(&par_origin, &[&b.membres[0], &b.membres[1], &b.membres[2]]);
        assert!(matches!(
            b.accepter(&h, T0),
            Err(ErreurJournal::Registre(ErreurRegistre::Sceau(ErreurSceau::SignataireInconnu)))
        ));
        // La clé d'Origin ne peut pas être celle d'un COG inscrit.
        assert!(b.fed.inscrire([7; 32], &b.origin.cle_publique(), 86_400).is_err());
    }

    #[test]
    fn temoins_exiges_bornes() {
        assert!(RegistreFederal::new(None).avec_temoins(2).is_err());
        assert!(RegistreFederal::new(None).avec_temoins(8).is_err());
        assert_eq!(RegistreFederal::new(None).avec_temoins(5).unwrap().temoins_min(), 5);
        assert!(verifier_temoins(&[], &[0; 32], [], None, 2, T0).is_err());
    }
}
