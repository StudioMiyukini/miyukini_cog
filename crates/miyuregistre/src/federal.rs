//! @id     miyuregistre_federal
//! @do     suivre_registres_federes_contresignes
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « federal » (têtes des COG fédérés contresignées par Origin, anti-éclipse).

//! Registre fédéral (EX-34 « registre Merkle fédéral contresigné par Origin,
//! anti-éclipse » ; lot 2 §8 « un journal par COG »).
//!
//! Vue fédérale = l'ensemble des registres des COG reconnus, chacun suivi par
//! un [`SuiviTetes`]. Une tête fédérale n'est acceptée que **contresignée** :
//! 2 signatures sur l'ensemble exact {clé de journal du COG, Origin}
//! (le COG scelle sa tête, Origin la cosigne avec [`miyusceau::cosigner`]).
//! Un pair isolé (éclipse) ne peut donc pas présenter une histoire qu'Origin
//! n'a pas vue ; une équivoque produit une preuve signée par les deux.
//!
//! Toutes les têtes acceptées alimentent l'[`HorlogeConfiance`] commune.
//!
//! Consigné par l'amendement **A-006** : contresignature = cosignature du même
//! sceau, amorçage d'un COG au premier usage (TOFU). **À décider** (A-006) :
//! journal fédéral agrégé tenu par Origin.

use std::collections::BTreeMap;

use miyusceau::{Acceptation, AncresDeConfiance, NiveauConfiance, PolitiqueTete, PreuveCoherence, TeteRegistre};

use crate::erreurs::ErreurJournal;
use crate::suivi::SuiviTetes;
use crate::temps::HorlogeConfiance;

/// Suivi des registres des COG fédérés.
#[derive(Debug, Clone)]
pub struct RegistreFederal {
    origin: [u8; 32],
    registres: BTreeMap<[u8; 32], SuiviTetes>,
}

impl RegistreFederal {
    /// Vue fédérale vide ; `origin` = clé publique de contresignature d'Origin.
    pub fn new(origin: [u8; 32]) -> Self {
        Self { origin, registres: BTreeMap::new() }
    }

    /// Inscrit le registre d'un COG reconnu (`journal` = sa clé de journal).
    /// Politique : 2 sur {journal, Origin}, âge maximal `age_max_secs`.
    pub fn inscrire(
        &mut self,
        registre: [u8; 32],
        journal: &[u8; 32],
        age_max_secs: u64,
    ) -> Result<(), ErreurJournal> {
        if journal == &self.origin {
            return Err(ErreurJournal::Sceau(miyusceau::ErreurSceau::CleInvalide));
        }
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(journal, NiveauConfiance::Reconnu)?;
        ancres.ajouter(&self.origin, NiveauConfiance::Reconnu)?;
        let politique = PolitiqueTete::nouvelle(2, age_max_secs)?;
        self.registres.entry(registre).or_insert_with(|| SuiviTetes::new(registre, ancres, politique));
        Ok(())
    }

    /// Retire un COG (fin de reconnaissance, sanction).
    pub fn retirer(&mut self, registre: &[u8; 32]) -> bool {
        self.registres.remove(registre).is_some()
    }

    /// Accepte une tête contresignée d'un COG inscrit.
    pub fn accepter(
        &mut self,
        registre: &[u8; 32],
        sceau: &[u8],
        coherence: Option<&PreuveCoherence>,
        horloge: &mut HorlogeConfiance,
        maintenant: u64,
    ) -> Result<Acceptation, ErreurJournal> {
        let suivi = self.registres.get_mut(registre).ok_or(ErreurJournal::RegistreInconnu)?;
        suivi.accepter(sceau, coherence, horloge, maintenant)
    }

    /// Suivi d'un COG.
    pub fn suivi(&self, registre: &[u8; 32]) -> Option<&SuiviTetes> {
        self.registres.get(registre)
    }

    /// Dernières têtes fraîches de tous les COG (les registres gelés sont omis).
    pub fn tetes_fraiches(&self, maintenant: u64) -> Vec<&TeteRegistre> {
        self.registres.values().filter_map(|s| s.tete_fraiche(maintenant).ok()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::Journal;
    use crate::stockage::StockageMemoire;
    use miyusceau::{cosigner, CleSignature, ErreurRegistre, ErreurSceau};

    const T0: u64 = 1_800_000_000;

    #[test]
    fn tete_contresignee_exigee() {
        let origin = CleSignature::generer();
        let journal_cle = CleSignature::generer();
        let mut fed = RegistreFederal::new(origin.cle_publique());
        fed.inscrire([8; 32], &journal_cle.cle_publique(), 86_400).unwrap();
        let mut j = Journal::ouvrir([8; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();

        let seule = j.emettre_tete(&[&journal_cle], T0, 86_400).unwrap();
        assert!(matches!(
            fed.accepter(&[8; 32], &seule, None, &mut horloge, T0),
            Err(ErreurJournal::Registre(ErreurRegistre::Sceau(ErreurSceau::SeuilNonAtteint { requis: 2, obtenu: 1 })))
        ));
        let contresignee = cosigner::<TeteRegistre>(&seule, &origin).unwrap();
        assert_eq!(fed.accepter(&[8; 32], &contresignee, None, &mut horloge, T0).unwrap(), Acceptation::Nouvelle);
        assert_eq!(fed.tetes_fraiches(T0).len(), 1);
        assert_eq!(fed.tetes_fraiches(T0 + 3 * 86_400).len(), 0);
        assert_eq!(horloge.mtp(), Some(T0));
        assert!(matches!(
            fed.accepter(&[9; 32], &contresignee, None, &mut horloge, T0),
            Err(ErreurJournal::RegistreInconnu)
        ));
    }

    #[test]
    fn origin_seul_ne_suffit_pas() {
        let origin = CleSignature::generer();
        let journal_cle = CleSignature::generer();
        let mut fed = RegistreFederal::new(origin.cle_publique());
        fed.inscrire([8; 32], &journal_cle.cle_publique(), 86_400).unwrap();
        let mut j = Journal::ouvrir([8; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        let par_origin = j.emettre_tete(&[&origin], T0, 86_400).unwrap();
        assert!(fed.accepter(&[8; 32], &par_origin, None, &mut horloge, T0).is_err());
        assert!(fed.inscrire([7; 32], &origin.cle_publique(), 86_400).is_err());
    }
}
