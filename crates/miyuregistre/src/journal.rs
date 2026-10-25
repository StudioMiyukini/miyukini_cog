//! @id     miyuregistre_journal
//! @do     tenir_journal_merkle_emetteur
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « journal » (côté émetteur : ajout d'entrées, têtes signées, preuves).

//! Journal Merkle côté **émetteur** (registre d'un COG signé par sa clé de
//! journal, ou registre du canal officiel tenu par Origin).
//!
//! - Entrées : sceaux de type `0x04` ([`crate::EntreeRegistre`]), contrôlés
//!   structurellement à l'ajout (format strict, charge canonique) ; leurs
//!   signatures sont vérifiées par les consommateurs selon leur politique.
//! - Têtes : [`miyusceau::TeteRegistre`] signée par la ou les clés fournies ;
//!   séquence `0, 1, 2…` ; horodatage jamais en recul ; chaque tête émise est
//!   **inscrite durablement** (stockage des têtes) avant d'être rendue.
//! - Preuves d'inclusion et de cohérence pour toute taille déjà atteinte.

use miyusceau::{
    decoder_titre, hacher_feuille, racine, sceller_multi, CleSignature, Empreinte, PreuveCoherence, PreuveInclusion,
    Sceau, TeteRegistre, Titre, TypeSceau,
};

use crate::entree::EntreeRegistre;
use crate::erreurs::{ErreurJournal, ErreurStockage};
use crate::stockage::Stockage;

/// Contrôle structurel d'un sceau d'entrée (aucune signature vérifiée).
pub fn controler_structure_entree(octets: &[u8]) -> Result<EntreeRegistre, ErreurJournal> {
    let sceau = Sceau::decoder(octets)?;
    if sceau.type_sceau != TypeSceau::EntreeRegistre {
        return Err(miyusceau::ErreurSceau::TypeInattendu {
            attendu: EntreeRegistre::TYPE as u8,
            trouve: sceau.type_sceau as u8,
        }
        .into());
    }
    Ok(decoder_titre::<EntreeRegistre>(&sceau.charge)?)
}

fn derniere_tete<S: Stockage>(tetes: &S) -> Result<Option<TeteRegistre>, ErreurJournal> {
    if tetes.taille() == 0 {
        return Ok(None);
    }
    let octets = tetes.lire(tetes.taille() - 1)?;
    let sceau = Sceau::decoder(&octets)?;
    Ok(Some(decoder_titre::<TeteRegistre>(&sceau.charge)?))
}

/// Journal Merkle émetteur.
#[derive(Debug)]
pub struct Journal<S: Stockage> {
    registre: [u8; 32],
    entrees: S,
    tetes: S,
    feuilles: Vec<Empreinte>,
    derniere: Option<TeteRegistre>,
}

impl<S: Stockage> Journal<S> {
    /// Ouvre un journal à partir de ses deux stockages (entrées, têtes émises).
    ///
    /// Les entrées sont recontrôlées ; la dernière tête émise doit appartenir
    /// à ce registre et ne pas dépasser le nombre d'entrées stockées.
    pub fn ouvrir(registre: [u8; 32], entrees: S, tetes: S) -> Result<Self, ErreurJournal> {
        let mut feuilles = Vec::with_capacity(usize::try_from(entrees.taille()).unwrap_or(0));
        for i in 0..entrees.taille() {
            let octets = entrees.lire(i)?;
            controler_structure_entree(&octets)?;
            feuilles.push(hacher_feuille(&octets));
        }
        let derniere = derniere_tete(&tetes)?;
        if let Some(t) = &derniere {
            if t.registre != registre || t.taille > feuilles.len() as u64 {
                return Err(ErreurJournal::StockageIncoherent);
            }
            let n = usize::try_from(t.taille).map_err(|_| ErreurJournal::StockageIncoherent)?;
            if racine(&feuilles[..n]) != t.racine {
                return Err(ErreurJournal::StockageIncoherent);
            }
        }
        Ok(Self { registre, entrees, tetes, feuilles, derniere })
    }

    /// Identifiant du registre.
    pub fn registre(&self) -> &[u8; 32] {
        &self.registre
    }

    /// Nombre d'entrées.
    pub fn taille(&self) -> u64 {
        self.feuilles.len() as u64
    }

    /// Racine Merkle courante.
    pub fn racine(&self) -> Empreinte {
        racine(&self.feuilles)
    }

    /// Dernière tête émise.
    pub fn derniere_tete(&self) -> Option<&TeteRegistre> {
        self.derniere.as_ref()
    }

    /// Ajoute un sceau d'entrée (contrôle structurel) ; renvoie son index.
    pub fn ajouter(&mut self, octets: &[u8]) -> Result<u64, ErreurJournal> {
        controler_structure_entree(octets)?;
        let index = self.entrees.ajouter(octets)?;
        if index != self.feuilles.len() as u64 {
            return Err(ErreurJournal::StockageIncoherent);
        }
        self.feuilles.push(hacher_feuille(octets));
        Ok(index)
    }

    /// Lit l'entrée `index`.
    pub fn entree(&self, index: u64) -> Result<Vec<u8>, ErreurStockage> {
        self.entrees.lire(index)
    }

    /// Entrées `[depuis, depuis + nombre)` (bornées à la taille).
    pub fn entrees(&self, depuis: u64, nombre: u64) -> Result<Vec<Vec<u8>>, ErreurStockage> {
        let fin = depuis.saturating_add(nombre).min(self.taille());
        (depuis..fin).map(|i| self.entrees.lire(i)).collect()
    }

    /// Émet et inscrit une tête signée par `cles` couvrant toutes les entrées.
    pub fn emettre_tete(
        &mut self,
        cles: &[&CleSignature],
        emise_a: u64,
        validite_max_secs: u64,
    ) -> Result<Vec<u8>, ErreurJournal> {
        let sequence = match &self.derniere {
            None => 0,
            Some(d) => {
                if emise_a < d.emise_a {
                    return Err(ErreurJournal::HorodatageEnRecul);
                }
                d.sequence.checked_add(1).ok_or(ErreurJournal::TailleHorsJournal)?
            }
        };
        let tete = TeteRegistre {
            registre: self.registre,
            taille: self.taille(),
            racine: self.racine(),
            sequence,
            emise_a,
            validite_max_secs,
        };
        let sceau = sceller_multi(&tete, cles)?;
        // Contrôle canonique (validité dans les bornes) avant inscription.
        decoder_titre::<TeteRegistre>(&Sceau::decoder(&sceau)?.charge)?;
        self.tetes.ajouter(&sceau)?;
        self.derniere = Some(tete);
        Ok(sceau)
    }

    /// Dernier sceau de tête émis.
    pub fn sceau_derniere_tete(&self) -> Result<Option<Vec<u8>>, ErreurStockage> {
        match self.tetes.taille() {
            0 => Ok(None),
            n => self.tetes.lire(n - 1).map(Some),
        }
    }

    fn prefixe(&self, taille: u64) -> Result<&[Empreinte], ErreurJournal> {
        let n = usize::try_from(taille).map_err(|_| ErreurJournal::TailleHorsJournal)?;
        self.feuilles.get(..n).ok_or(ErreurJournal::TailleHorsJournal)
    }

    /// Preuve d'inclusion de l'entrée `index` dans l'arbre de taille `taille`.
    pub fn preuve_inclusion(&self, index: u64, taille: u64) -> Result<PreuveInclusion, ErreurJournal> {
        let feuilles = self.prefixe(taille)?;
        let i = usize::try_from(index).map_err(|_| ErreurJournal::TailleHorsJournal)?;
        PreuveInclusion::generer(feuilles, i).ok_or(ErreurJournal::TailleHorsJournal)
    }

    /// Preuve de cohérence entre les tailles `taille1` et `taille2`.
    pub fn preuve_coherence(&self, taille1: u64, taille2: u64) -> Result<PreuveCoherence, ErreurJournal> {
        let feuilles = self.prefixe(taille2)?;
        let t1 = usize::try_from(taille1).map_err(|_| ErreurJournal::TailleHorsJournal)?;
        PreuveCoherence::generer(feuilles, t1).ok_or(ErreurJournal::TailleHorsJournal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entree::{Corps, EntreeRegistre};
    use crate::stockage::StockageMemoire;
    use miyusceau::{sceller, verifier_tete, AncresDeConfiance, NiveauConfiance, PolitiqueTete};

    fn entree(n: u8) -> Vec<u8> {
        let cle = CleSignature::depuis_graine(&[9; 32]);
        let e = EntreeRegistre { inscrite_a: u64::from(n), corps: Corps::Evenement { objet: "test".into(), empreinte: [n; 32] } };
        sceller(&e, &cle).unwrap()
    }

    #[test]
    fn tetes_sequence_preuves_et_reouverture() {
        let journal_cle = CleSignature::generer();
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(&journal_cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
        let mut j = Journal::ouvrir([1; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        j.ajouter(&entree(1)).unwrap();
        j.ajouter(&entree(2)).unwrap();
        let t0 = j.emettre_tete(&[&journal_cle], 1_000, 3_600).unwrap();
        j.ajouter(&entree(3)).unwrap();
        let t1 = j.emettre_tete(&[&journal_cle], 1_000, 3_600).unwrap();
        let v0 = verifier_tete(&t0, &ancres, &PolitiqueTete::journal_cog(), &[1; 32], 1_000).unwrap();
        let v1 = verifier_tete(&t1, &ancres, &PolitiqueTete::journal_cog(), &[1; 32], 1_000).unwrap();
        assert_eq!((v0.tete().sequence, v0.tete().taille), (0, 2));
        assert_eq!((v1.tete().sequence, v1.tete().taille), (1, 3));
        let p = j.preuve_inclusion(2, 3).unwrap();
        p.verifier(&hacher_feuille(&entree(3)), &v1.tete().racine).unwrap();
        let c = j.preuve_coherence(2, 3).unwrap();
        c.verifier(&v0.tete().racine, &v1.tete().racine).unwrap();
        assert!(matches!(j.emettre_tete(&[&journal_cle], 999, 3_600), Err(ErreurJournal::HorodatageEnRecul)));
        assert!(matches!(j.preuve_inclusion(0, 4), Err(ErreurJournal::TailleHorsJournal)));

        // Réouverture : séquence reprise depuis le stockage des têtes.
        let Journal { entrees, tetes, .. } = j;
        let mut j = Journal::ouvrir([1; 32], entrees, tetes).unwrap();
        let t2 = j.emettre_tete(&[&journal_cle], 1_001, 3_600).unwrap();
        let v2 = verifier_tete(&t2, &ancres, &PolitiqueTete::journal_cog(), &[1; 32], 1_001).unwrap();
        assert_eq!(v2.tete().sequence, 2);
    }

    #[test]
    fn entree_mal_formee_refusee() {
        let mut j = Journal::ouvrir([1; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        assert!(j.ajouter(b"pas un sceau").is_err());
        let mut e = entree(1);
        let n = e.len();
        e[n - 1] ^= 1; // signature altérée : structure intacte, accepté (vérifié par les consommateurs)
        assert!(j.ajouter(&e).is_ok());
    }

    #[test]
    fn reouverture_sur_autre_registre_refusee() {
        let cle = CleSignature::generer();
        let mut j = Journal::ouvrir([1; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        j.ajouter(&entree(1)).unwrap();
        j.emettre_tete(&[&cle], 10, 60).unwrap();
        let Journal { entrees, tetes, .. } = j;
        assert!(matches!(Journal::ouvrir([2; 32], entrees, tetes), Err(ErreurJournal::StockageIncoherent)));
    }
}
