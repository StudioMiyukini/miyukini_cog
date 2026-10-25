//! @id     miyuregistre_suivi
//! @do     suivre_tetes_registre_mtp
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « suivi » (côté vérificateur : têtes, Median-Time-Past, miroir vérifié).

//! Côté **vérificateur** d'un registre.
//!
//! - [`SuiviTetes`] : dernière tête acceptée ([`EtatRegistre`] : signatures à
//!   seuil, registre attendu, fraîcheur, monotonie, équivoque, cohérence) **et**
//!   Median-Time-Past propre au registre : une tête dont l'horodatage est
//!   antérieur à la médiane des 11 dernières têtes acceptées de ce registre
//!   est refusée. Toute tête acceptée alimente l'[`HorlogeConfiance`] commune.
//! - [`MiroirRegistre`] : suivi + copie locale **intégrale** des entrées. Les
//!   entrées reçues doivent reconstruire exactement la racine signée ; la
//!   preuve de cohérence est recalculée sur la copie (moniteur au sens
//!   RFC 9162 : toute l'histoire est revérifiée, rien n'est cru sur parole).
//!   Toute révocation est ainsi visible du consommateur.

use miyusceau::{
    hacher_feuille, racine, verifier_tete, Acceptation, AncresDeConfiance, Empreinte, EtatRegistre, PolitiqueTete,
    PreuveCoherence, PreuveInclusion, TeteRegistre, TeteVerifiee,
};

use crate::erreurs::{ErreurJournal, ErreurStockage};
use crate::stockage::Stockage;
use crate::temps::{HorlogeConfiance, TempsMedian};

/// Acceptation préparée (non encore appliquée) d'une tête.
#[derive(Debug, Clone)]
pub struct TetePreparee {
    etat: EtatRegistre,
    acceptation: Acceptation,
    emise_a: u64,
    /// Tête acceptée au moment de la préparation (contrôle de concurrence).
    precedente: Option<TeteRegistre>,
}

impl TetePreparee {
    /// Tête qui serait retenue.
    pub fn tete(&self) -> &TeteRegistre {
        self.etat.tete()
    }

    /// Issue de l'acceptation.
    pub fn acceptation(&self) -> Acceptation {
        self.acceptation
    }

    /// Tête acceptée lors de la préparation (aucune : premier usage).
    pub fn etat_precedent(&self) -> Option<&TeteRegistre> {
        self.precedente.as_ref()
    }
}

/// Synchronisation préparée (tous contrôles faits) d'un [`MiroirRegistre`].
#[derive(Debug, Clone)]
pub struct SynchroPreparee {
    depuis: u64,
    /// Feuilles complètes après synchronisation (`None` : inchangées).
    feuilles: Option<Vec<Empreinte>>,
    tete: TetePreparee,
}

impl SynchroPreparee {
    /// Tête qui sera retenue.
    pub fn tete(&self) -> &TeteRegistre {
        self.tete.tete()
    }

    /// Taille du miroir au moment de la préparation.
    pub fn depuis(&self) -> u64 {
        self.depuis
    }
}

/// Suivi des têtes d'un registre : monotonie et Median-Time-Past.
#[derive(Debug, Clone)]
pub struct SuiviTetes {
    registre: [u8; 32],
    ancres: AncresDeConfiance,
    politique: PolitiqueTete,
    etat: Option<EtatRegistre>,
    temps: TempsMedian,
}

impl SuiviTetes {
    /// Suivi sans tête acceptée : la première tête valide sera retenue
    /// (confiance au premier usage, décidée par l'appelant).
    pub fn new(registre: [u8; 32], ancres: AncresDeConfiance, politique: PolitiqueTete) -> Self {
        Self { registre, ancres, politique, etat: None, temps: TempsMedian::new() }
    }

    /// Reprend un suivi persistant (`etat` = [`EtatRegistre::encoder`],
    /// `temps` = [`TempsMedian::encoder`]). Signatures revérifiées.
    pub fn reprendre(
        registre: [u8; 32],
        ancres: AncresDeConfiance,
        politique: PolitiqueTete,
        etat: &[u8],
        temps: &[u8],
    ) -> Result<Self, ErreurJournal> {
        let etat = EtatRegistre::decoder(etat, &ancres, &politique, &registre)?;
        let temps = TempsMedian::decoder(temps)?;
        Ok(Self { registre, ancres, politique, etat: Some(etat), temps })
    }

    /// Identifiant du registre suivi.
    pub fn registre(&self) -> &[u8; 32] {
        &self.registre
    }

    /// Politique des têtes.
    pub fn politique(&self) -> &PolitiqueTete {
        &self.politique
    }

    /// Dernier état accepté.
    pub fn etat(&self) -> Option<&EtatRegistre> {
        self.etat.as_ref()
    }

    /// Fenêtre du Median-Time-Past du registre.
    pub fn temps(&self) -> &TempsMedian {
        &self.temps
    }

    /// Vérifie une tête (sceau à seuil, registre, fraîcheur, MTP) et prépare
    /// son acceptation (monotonie, cohérence) sans modifier le suivi.
    pub fn preparer(
        &self,
        sceau: &[u8],
        coherence: Option<&PreuveCoherence>,
        maintenant: u64,
    ) -> Result<TetePreparee, ErreurJournal> {
        let verifiee: TeteVerifiee = verifier_tete(sceau, &self.ancres, &self.politique, &self.registre, maintenant)?;
        let emise_a = verifiee.tete().emise_a;
        self.temps.controler(emise_a)?;
        let precedente = self.etat.as_ref().map(|e| e.tete().clone());
        let (etat, acceptation) = match &self.etat {
            None => (EtatRegistre::initial(verifiee), Acceptation::Nouvelle),
            Some(e) => {
                let mut copie = e.clone();
                let a = copie.accepter(verifiee, coherence)?;
                (copie, a)
            }
        };
        Ok(TetePreparee { etat, acceptation, emise_a, precedente })
    }

    /// Applique une acceptation préparée : état, fenêtre MTP du registre et
    /// horloge de confiance commune.
    pub fn appliquer(&mut self, preparee: TetePreparee, horloge: &mut HorlogeConfiance) -> Acceptation {
        if preparee.acceptation == Acceptation::Nouvelle {
            self.temps.observer(preparee.emise_a);
            horloge.observer_tete(preparee.emise_a);
        }
        self.etat = Some(preparee.etat);
        preparee.acceptation
    }

    /// Vérifie puis accepte une tête.
    pub fn accepter(
        &mut self,
        sceau: &[u8],
        coherence: Option<&PreuveCoherence>,
        horloge: &mut HorlogeConfiance,
        maintenant: u64,
    ) -> Result<Acceptation, ErreurJournal> {
        let p = self.preparer(sceau, coherence, maintenant)?;
        Ok(self.appliquer(p, horloge))
    }

    /// Exige une tête acceptée encore fraîche.
    pub fn tete_fraiche(&self, maintenant: u64) -> Result<&TeteRegistre, ErreurJournal> {
        let etat = self.etat.as_ref().ok_or(ErreurJournal::AucuneTete)?;
        Ok(etat.exiger_fraicheur(&self.politique, maintenant)?)
    }

    /// Vérifie une preuve d'inclusion contre la dernière tête, si elle est fraîche.
    pub fn verifier_inclusion(
        &self,
        feuille: &Empreinte,
        preuve: &PreuveInclusion,
        maintenant: u64,
    ) -> Result<(), ErreurJournal> {
        let etat = self.etat.as_ref().ok_or(ErreurJournal::AucuneTete)?;
        Ok(etat.verifier_inclusion(feuille, preuve, &self.politique, maintenant)?)
    }
}

/// Miroir vérifié d'un registre : suivi des têtes + copie intégrale des entrées.
#[derive(Debug)]
pub struct MiroirRegistre<S: Stockage> {
    suivi: SuiviTetes,
    stockage: S,
    feuilles: Vec<Empreinte>,
}

impl<S: Stockage> MiroirRegistre<S> {
    /// Ouvre un miroir. Sans `suivi` repris (aucune tête acceptée), toute
    /// entrée stockée est non confirmée et retirée. Avec un suivi, le stockage
    /// doit contenir au moins l'histoire de la tête acceptée (racine
    /// recalculée) ; l'excédent (non confirmé) est retiré.
    pub fn ouvrir(suivi: SuiviTetes, mut stockage: S) -> Result<Self, ErreurJournal> {
        let confirmee = suivi.etat().map_or(0, |e| e.tete().taille);
        if stockage.taille() < confirmee {
            return Err(ErreurJournal::StockageIncoherent);
        }
        stockage.tronquer(confirmee)?;
        let mut feuilles = Vec::with_capacity(usize::try_from(confirmee).unwrap_or(0));
        for i in 0..confirmee {
            feuilles.push(hacher_feuille(&stockage.lire(i)?));
        }
        if let Some(e) = suivi.etat() {
            if racine(&feuilles) != e.tete().racine {
                return Err(ErreurJournal::StockageIncoherent);
            }
        }
        Ok(Self { suivi, stockage, feuilles })
    }

    /// Suivi des têtes.
    pub fn suivi(&self) -> &SuiviTetes {
        &self.suivi
    }

    /// Nombre d'entrées confirmées.
    pub fn taille(&self) -> u64 {
        self.feuilles.len() as u64
    }

    /// Entrée confirmée `index`.
    pub fn entree(&self, index: u64) -> Result<Vec<u8>, ErreurStockage> {
        self.stockage.lire(index)
    }

    /// Synchronise le miroir sur une nouvelle tête et les entrées qui la
    /// prolongent (`entrees` = indices `taille()..tete.taille`).
    ///
    /// Ordre : tête vérifiée (seuil, registre, fraîcheur, MTP) → nombre
    /// d'entrées exact → racine reconstruite = racine signée → monotonie et
    /// cohérence (preuve recalculée) → écriture durable des entrées → état.
    /// Rien n'est modifié en cas de refus. Renvoie l'intervalle des nouveaux
    /// indices.
    pub fn synchroniser(
        &mut self,
        sceau_tete: &[u8],
        entrees: &[Vec<u8>],
        horloge: &mut HorlogeConfiance,
        maintenant: u64,
    ) -> Result<std::ops::Range<u64>, ErreurJournal> {
        let preparee = self.preparer_synchro(sceau_tete, entrees, maintenant)?;
        self.appliquer_synchro(preparee, entrees, horloge)
    }

    /// Première phase de [`MiroirRegistre::synchroniser`] : **tous** les
    /// contrôles (tête, nombre d'entrées, racine, monotonie, cohérence) sur
    /// `&self`, sans rien modifier. Permet de vérifier sous un verrou de
    /// lecture et de n'écrire que brièvement.
    ///
    /// Tête de même taille que le miroir : la racine est comparée à celle de
    /// la tête acceptée, sans recalcul.
    pub fn preparer_synchro(
        &self,
        sceau_tete: &[u8],
        entrees: &[Vec<u8>],
        maintenant: u64,
    ) -> Result<SynchroPreparee, ErreurJournal> {
        let m = self.taille();
        // Vérification de la tête seule d'abord (signature avant tout calcul).
        let verifiee = verifier_tete(
            sceau_tete,
            &self.suivi.ancres,
            &self.suivi.politique,
            &self.suivi.registre,
            maintenant,
        )?;
        let n = verifiee.tete().taille;
        if n < m {
            // Taille en recul : `preparer` produit le refus (équivoque, retour).
            self.suivi.preparer(sceau_tete, None, maintenant)?;
            return Err(ErreurJournal::StockageIncoherent);
        }
        let attendues = n - m;
        if entrees.len() as u64 != attendues {
            return Err(ErreurJournal::EntreesIncoherentes { attendues, recues: entrees.len() as u64 });
        }
        let racine_signee = verifiee.tete().racine;
        let (feuilles, coherence) = if n == m {
            // Rien de neuf : la racine doit être celle déjà acceptée (ou l'arbre vide).
            let actuelle = self.suivi.etat().map_or_else(|| racine(&self.feuilles), |e| e.tete().racine);
            if actuelle != racine_signee {
                return Err(ErreurJournal::RacineDifferente);
            }
            (None, None)
        } else {
            let mut feuilles = self.feuilles.clone();
            feuilles.extend(entrees.iter().map(|e| hacher_feuille(e)));
            if racine(&feuilles) != racine_signee {
                return Err(ErreurJournal::RacineDifferente);
            }
            let coherence = if m > 0 {
                PreuveCoherence::generer(&feuilles, usize::try_from(m).map_err(|_| ErreurJournal::TailleHorsJournal)?)
            } else {
                None
            };
            (Some(feuilles), coherence)
        };
        let tete = self.suivi.preparer(sceau_tete, coherence.as_ref(), maintenant)?;
        Ok(SynchroPreparee { depuis: m, feuilles, tete })
    }

    /// Seconde phase : écrit les entrées préparées et applique la tête.
    ///
    /// `entrees` doit être exactement celles passées à
    /// [`MiroirRegistre::preparer_synchro`] (chaque empreinte est recontrôlée)
    /// et le miroir ne doit pas avoir changé depuis ; sinon refus, rien n'est
    /// modifié.
    pub fn appliquer_synchro(
        &mut self,
        preparee: SynchroPreparee,
        entrees: &[Vec<u8>],
        horloge: &mut HorlogeConfiance,
    ) -> Result<std::ops::Range<u64>, ErreurJournal> {
        let m = self.taille();
        let n = preparee.tete.tete().taille;
        let etat_inchange = match (self.suivi.etat(), preparee.tete.etat_precedent()) {
            (None, None) => true,
            (Some(e), Some(p)) => e.tete() == p,
            _ => false,
        };
        if preparee.depuis != m || !etat_inchange || entrees.len() as u64 != n.saturating_sub(m) {
            return Err(ErreurJournal::StockageIncoherent);
        }
        if let Some(feuilles) = &preparee.feuilles {
            let debut = usize::try_from(m).map_err(|_| ErreurJournal::TailleHorsJournal)?;
            let nouvelles = feuilles.get(debut..).ok_or(ErreurJournal::StockageIncoherent)?;
            if nouvelles.len() != entrees.len() || nouvelles.iter().zip(entrees).any(|(f, e)| *f != hacher_feuille(e)) {
                return Err(ErreurJournal::RacineDifferente);
            }
        }
        for (k, e) in entrees.iter().enumerate() {
            if let Err(err) = self.stockage.ajouter(e) {
                let _ = self.stockage.tronquer(m);
                return Err(err.into());
            }
            debug_assert_eq!(self.stockage.taille(), m + k as u64 + 1);
        }
        if let Some(feuilles) = preparee.feuilles {
            self.feuilles = feuilles;
        }
        self.suivi.appliquer(preparee.tete, horloge);
        Ok(m..n)
    }

    /// Vérifie l'inclusion de l'entrée confirmée `index` contre la dernière
    /// tête, si elle est fraîche (preuve RFC 9162 calculée sur la copie).
    pub fn verifier_inclusion(&self, index: u64, maintenant: u64) -> Result<(), ErreurJournal> {
        let i = usize::try_from(index).map_err(|_| ErreurJournal::TailleHorsJournal)?;
        let preuve = PreuveInclusion::generer(&self.feuilles, i).ok_or(ErreurJournal::TailleHorsJournal)?;
        let feuille = *self.feuilles.get(i).ok_or(ErreurJournal::TailleHorsJournal)?;
        self.suivi.verifier_inclusion(&feuille, &preuve, maintenant)
    }

    /// Index d'une feuille confirmée, si présente.
    pub fn index_de(&self, feuille: &Empreinte) -> Option<u64> {
        self.feuilles.iter().position(|f| f == feuille).map(|i| i as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entree::{Corps, EntreeRegistre};
    use crate::journal::Journal;
    use crate::stockage::StockageMemoire;
    use miyusceau::{sceller, CleSignature, ErreurRegistre, NiveauConfiance};

    const T0: u64 = 1_800_000_000;

    fn entree(n: u8) -> Vec<u8> {
        let cle = CleSignature::depuis_graine(&[5; 32]);
        let e = EntreeRegistre { inscrite_a: T0, corps: Corps::Evenement { objet: "e".into(), empreinte: [n; 32] } };
        sceller(&e, &cle).unwrap()
    }

    struct Banc {
        cle: CleSignature,
        journal: Journal<StockageMemoire>,
        miroir: MiroirRegistre<StockageMemoire>,
        horloge: HorlogeConfiance,
    }

    fn banc() -> Banc {
        let cle = CleSignature::generer();
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
        let suivi = SuiviTetes::new([3; 32], ancres, PolitiqueTete::journal_cog());
        Banc {
            cle,
            journal: Journal::ouvrir([3; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap(),
            miroir: MiroirRegistre::ouvrir(suivi, StockageMemoire::new()).unwrap(),
            horloge: HorlogeConfiance::new(),
        }
    }

    impl Banc {
        fn publier(&mut self, n: u8, t: u64) -> Vec<u8> {
            self.journal.ajouter(&entree(n)).unwrap();
            self.journal.emettre_tete(&[&self.cle], t, 86_400).unwrap()
        }

        fn synchroniser(&mut self, tete: &[u8], t: u64) -> Result<std::ops::Range<u64>, ErreurJournal> {
            let nouvelles = self.journal.entrees(self.miroir.taille(), u64::MAX).unwrap();
            self.miroir.synchroniser(tete, &nouvelles, &mut self.horloge, t)
        }
    }

    #[test]
    fn miroir_suit_le_journal_et_prouve_l_inclusion() {
        let mut b = banc();
        let t = b.publier(1, T0);
        assert_eq!(b.synchroniser(&t, T0).unwrap(), 0..1);
        b.publier(2, T0 + 10);
        b.journal.ajouter(&entree(3)).unwrap();
        let t3 = b.journal.emettre_tete(&[&b.cle], T0 + 20, 86_400).unwrap();
        assert_eq!(b.synchroniser(&t3, T0 + 20).unwrap(), 1..3);
        b.miroir.verifier_inclusion(2, T0 + 20).unwrap();
        assert_eq!(b.horloge.mtp(), Some(T0 + 20));
        // Tête gelée : inclusion refusée.
        assert!(matches!(
            b.miroir.verifier_inclusion(2, T0 + 20 + 2 * 86_400),
            Err(ErreurJournal::Registre(ErreurRegistre::TeteGelee { .. }))
        ));
    }

    #[test]
    fn entrees_alterees_ou_incompletes_refusees() {
        let mut b = banc();
        let t = b.publier(1, T0);
        let mut alteree = entree(1);
        alteree[10] ^= 1;
        assert!(matches!(
            b.miroir.synchroniser(&t, &[alteree], &mut b.horloge, T0),
            Err(ErreurJournal::RacineDifferente)
        ));
        assert!(matches!(
            b.miroir.synchroniser(&t, &[], &mut b.horloge, T0),
            Err(ErreurJournal::EntreesIncoherentes { attendues: 1, recues: 0 })
        ));
        assert_eq!(b.miroir.taille(), 0);
        assert!(b.miroir.suivi().etat().is_none());
    }

    #[test]
    fn retour_arriere_et_equivoque_refuses() {
        let mut b = banc();
        let t0 = b.publier(1, T0);
        let t1 = b.publier(2, T0 + 10);
        b.synchroniser(&t1, T0 + 10).unwrap();
        // Ancienne tête rejouée.
        assert!(matches!(
            b.miroir.synchroniser(&t0, &[], &mut b.horloge, T0 + 10),
            Err(ErreurJournal::Temps(_) | ErreurJournal::Registre(ErreurRegistre::RetourArriere))
        ));
        // Histoire réécrite par la clé de journal : autre entrée à l'index 1.
        let mut fourbe = Journal::ouvrir([3; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        fourbe.ajouter(&entree(1)).unwrap();
        fourbe.ajouter(&entree(9)).unwrap();
        fourbe.ajouter(&entree(10)).unwrap();
        for s in 0..3 {
            fourbe.emettre_tete(&[&b.cle], T0 + 20 + s, 86_400).unwrap();
        }
        let tf = fourbe.sceau_derniere_tete().unwrap().unwrap();
        let nouvelles = fourbe.entrees(2, 1).unwrap();
        assert!(matches!(
            b.miroir.synchroniser(&tf, &nouvelles, &mut b.horloge, T0 + 30),
            Err(ErreurJournal::RacineDifferente)
        ));
    }

    #[test]
    fn mtp_refuse_une_tete_anterieure_a_la_mediane() {
        let cle = CleSignature::generer();
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
        let mut suivi = SuiviTetes::new([4; 32], ancres.clone(), PolitiqueTete::journal_cog());
        let mut horloge = HorlogeConfiance::new();
        let mut j = Journal::ouvrir([4; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        for k in 0..11 {
            let s = j.emettre_tete(&[&cle], T0 + 100 * k, 86_400).unwrap();
            suivi.accepter(&s, None, &mut horloge, T0 + 100 * k).unwrap();
        }
        assert_eq!(suivi.temps().mtp(), Some(T0 + 500));
        // Une tête à séquence supérieure mais horodatée sous la médiane : refus MTP.
        // (État remis à une tête plus ancienne pour isoler la règle MTP de la monotonie.)
        let mut isole = SuiviTetes::new([4; 32], ancres, PolitiqueTete::journal_cog());
        isole.temps = suivi.temps().clone();
        let mut j2 = Journal::ouvrir([4; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        let s = j2.emettre_tete(&[&cle], T0 + 400, 86_400).unwrap();
        assert!(matches!(
            isole.accepter(&s, None, &mut horloge, T0 + 1_100),
            Err(ErreurJournal::Temps(crate::ErreurTemps::AnterieurMedian { .. }))
        ));
        // Horloge murale recalée sous le MTP commun : refus.
        assert!(horloge.maintenant(T0 + 100).is_err());
    }

    #[test]
    fn reouverture_du_miroir() {
        let mut b = banc();
        let t = b.publier(1, T0);
        b.synchroniser(&t, T0).unwrap();
        let t = b.publier(2, T0 + 1);
        b.synchroniser(&t, T0 + 1).unwrap();
        let etat = b.miroir.suivi().etat().unwrap().encoder();
        let temps = b.miroir.suivi().temps().encoder();
        let Banc { cle, miroir, .. } = b;
        let MiroirRegistre { mut stockage, .. } = miroir;
        // Entrée non confirmée ajoutée avant la coupure : retirée à la réouverture.
        stockage.ajouter(&entree(7)).unwrap();
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
        let suivi = SuiviTetes::reprendre([3; 32], ancres, PolitiqueTete::journal_cog(), &etat, &temps).unwrap();
        let m = MiroirRegistre::ouvrir(suivi, stockage).unwrap();
        assert_eq!(m.taille(), 2);
        m.verifier_inclusion(1, T0 + 1).unwrap();
    }
}
