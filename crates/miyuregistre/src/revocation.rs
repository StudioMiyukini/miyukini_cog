//! @id     miyuregistre_revocation
//! @do     adapter_revocations_registre_miyauth
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « revocation » (fonctionnalité `miyauth` : `SourceRevocation` adossée au miroir vérifié d'un registre).

//! Adaptateur [`SourceRevocation`] ← registre vérifié (lot 2 §8.2, A-003,
//! A-006 ; « adaptateur ← tête de registre vérifiée »).
//!
//! [`RevocationsRegistre`] lit les entrées `Revocation` d'un
//! [`MiroirRegistre`] (copie intégrale, racine recalculée, tête vérifiée) :
//!
//! - seules les entrées dont le sceau atteint le **seuil** sur les ancres
//!   fournies (ex. clé de journal du COG) sont comptées ; une entrée d'un autre
//!   signataire ou d'une autre nature est ignorée ;
//! - cibles reprises : `Cle(kid)` → [`CibleRevocation::Cle`] ; `Serie` →
//!   [`CibleRevocation::Serie`] (voir convention ci-dessous) ; `Annonce` et
//!   `Artefact` relèvent du canal officiel et ne concernent pas miyauth ;
//! - l'horodatage de la liste est celui de la **tête vérifiée** du miroir lors
//!   du dernier [`RevocationsRegistre::rafraichir`].
//!
//! # Fail-closed
//!
//! - aucune tête vue, autre COG émetteur ⇒ [`ErreurRevocation::ListeAbsente`] ;
//! - tête plus ancienne que la fraîcheur maximale (15 min par défaut, durée
//!   d'un visa) ⇒ [`ErreurRevocation::Perimee`] ;
//! - tête datée dans le futur au-delà de la tolérance ⇒ [`ErreurRevocation::HorlogeIncoherente`] ;
//! - rafraîchissement sur une tête gelée (non fraîche selon la politique du
//!   registre) ou un stockage illisible ⇒ erreur, liste inchangée (elle se
//!   périme ensuite d'elle-même).
//!
//! # Convention de série (non spécifiée par la doc gelée, à amender)
//!
//! Une série miyauth fait 16 octets, la cible d'une entrée 32 : la série est
//! inscrite **complétée de 16 octets nuls** ([`cible_serie`]) ; une cible
//! `Serie` dont les 16 derniers octets ne sont pas nuls n'est pas une série
//! miyauth et est ignorée.

use std::collections::HashSet;

use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
use miyauth::titres::Serie;
use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyusceau::{verifier_seuil, AncresDeConfiance, TOLERANCE_HORLOGE_SECS};

use crate::entree::{Cible, Corps, EntreeRegistre};
use crate::erreurs::ErreurJournal;
use crate::stockage::Stockage;
use crate::suivi::MiroirRegistre;

/// Cible d'entrée de registre pour une série miyauth (complétée de zéros).
pub fn cible_serie(serie: &Serie) -> Cible {
    let mut v = [0u8; 32];
    v[..16].copy_from_slice(serie);
    Cible::Serie(v)
}

fn vers_miyauth(cible: &Cible) -> Option<CibleRevocation> {
    match cible {
        Cible::Cle(kid) => Some(CibleRevocation::Cle(*kid)),
        Cible::Serie(v) if v[16..].iter().all(|b| *b == 0) => {
            let mut s: Serie = [0; 16];
            s.copy_from_slice(&v[..16]);
            Some(CibleRevocation::Serie(s))
        }
        Cible::Serie(_) | Cible::Annonce(_) | Cible::Artefact(_) => None,
    }
}

/// Liste de révocation d'un COG tirée de son registre vérifié.
#[derive(Debug, Clone)]
pub struct RevocationsRegistre {
    cog: CogId,
    ancres: AncresDeConfiance,
    seuil: usize,
    fraicheur_max: u64,
    cibles: HashSet<CibleRevocation>,
    /// Entrées déjà lues (`0..lues`).
    lues: u64,
    /// Horodatage de la tête vérifiée du dernier rafraîchissement.
    a_jour_au: Option<u64>,
}

impl RevocationsRegistre {
    /// Liste vide (refuse tout tant qu'aucun rafraîchissement n'a réussi) pour
    /// `cog`, entrées signées au `seuil` sur `ancres`, fraîcheur 15 min.
    pub fn new(cog: CogId, ancres: AncresDeConfiance, seuil: usize) -> Self {
        Self {
            cog,
            ancres,
            seuil: seuil.max(1),
            fraicheur_max: FRAICHEUR_MAX_DEFAUT_SECS,
            cibles: HashSet::new(),
            lues: 0,
            a_jour_au: None,
        }
    }

    /// Fraîcheur maximale plus **courte** que 15 min ; une valeur plus longue est ramenée à 15 min.
    pub fn avec_fraicheur(mut self, secs: u64) -> Self {
        self.fraicheur_max = secs.min(FRAICHEUR_MAX_DEFAUT_SECS);
        self
    }

    /// Nombre de cibles révoquées connues.
    pub fn len(&self) -> usize {
        self.cibles.len()
    }

    /// Vrai si aucune cible révoquée n'est connue.
    pub fn is_empty(&self) -> bool {
        self.cibles.is_empty()
    }

    /// Horodatage de la tête du dernier rafraîchissement réussi.
    pub fn a_jour_au(&self) -> Option<u64> {
        self.a_jour_au
    }

    /// Relit les entrées nouvelles du miroir, sous une tête **fraîche**
    /// (politique du registre). Renvoie le nombre de révocations ajoutées.
    /// En cas d'erreur la liste est inchangée.
    pub fn rafraichir<S: Stockage>(&mut self, miroir: &MiroirRegistre<S>, maintenant: u64) -> Result<usize, ErreurJournal> {
        let tete = miroir.suivi().tete_fraiche(maintenant)?;
        let (taille, emise_a) = (tete.taille, tete.emise_a);
        if taille < self.lues || miroir.taille() < taille {
            return Err(ErreurJournal::StockageIncoherent);
        }
        let mut nouvelles = Vec::new();
        for index in self.lues..taille {
            let octets = miroir.entree(index)?;
            // Signature vérifiée avant tout décodage ; autre signataire : ignorée.
            let Ok(verifie) = verifier_seuil::<EntreeRegistre>(&octets, &self.ancres, self.seuil, maintenant) else {
                continue;
            };
            if let Corps::Revocation { cible, .. } = verifie.into_titre().corps {
                nouvelles.extend(vers_miyauth(&cible));
            }
        }
        let avant = self.cibles.len();
        self.cibles.extend(nouvelles);
        self.lues = taille;
        self.a_jour_au = Some(self.a_jour_au.map_or(emise_a, |a| a.max(emise_a)));
        Ok(self.cibles.len() - avant)
    }
}

impl SourceRevocation for RevocationsRegistre {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        if emetteur != &self.cog {
            return Err(ErreurRevocation::ListeAbsente);
        }
        let a_jour_au = self.a_jour_au.ok_or(ErreurRevocation::ListeAbsente)?;
        if a_jour_au > maintenant_secs.saturating_add(TOLERANCE_HORLOGE_SECS) {
            return Err(ErreurRevocation::HorlogeIncoherente);
        }
        if a_jour_au.saturating_add(self.fraicheur_max) < maintenant_secs {
            return Err(ErreurRevocation::Perimee { a_jour_au, maintenant: maintenant_secs });
        }
        Ok(self.cibles.contains(cible))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::Journal;
    use crate::stockage::StockageMemoire;
    use crate::suivi::SuiviTetes;
    use crate::temps::HorlogeConfiance;
    use miyusceau::{sceller, CleSignature, NiveauConfiance, PolitiqueTete};

    const T0: u64 = 1_800_000_000;
    const REG: [u8; 32] = [3; 32];

    fn cog() -> CogId {
        CogId::new("abcdefghijklmnopqrstuvwxyz").unwrap()
    }

    fn revocation(cible: Cible, cle: &CleSignature) -> Vec<u8> {
        let e = EntreeRegistre { inscrite_a: T0, corps: Corps::Revocation { cible, motif: "perte".into() } };
        sceller(&e, cle).unwrap()
    }

    struct Banc {
        journal_cle: CleSignature,
        journal: Journal<StockageMemoire>,
        miroir: MiroirRegistre<StockageMemoire>,
        horloge: HorlogeConfiance,
    }

    impl Banc {
        fn new() -> Self {
            let journal_cle = CleSignature::generer();
            let mut ancres = AncresDeConfiance::new();
            ancres.ajouter(&journal_cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
            let suivi = SuiviTetes::new(REG, ancres, PolitiqueTete::journal_cog());
            Self {
                journal: Journal::ouvrir(REG, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
                miroir: MiroirRegistre::ouvrir(suivi, StockageMemoire::new()).unwrap(),
                horloge: HorlogeConfiance::new(),
                journal_cle,
            }
        }

        fn ancres(&self) -> AncresDeConfiance {
            let mut a = AncresDeConfiance::new();
            a.ajouter(&self.journal_cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
            a
        }

        /// Inscrit, émet une tête à `t` et synchronise le miroir.
        fn publier(&mut self, entrees: &[Vec<u8>], t: u64) {
            let m = self.miroir.taille();
            for e in entrees {
                self.journal.ajouter(e).unwrap();
            }
            let tete = self.journal.emettre_tete(&[&self.journal_cle], t, 86_400).unwrap();
            let nouvelles = self.journal.entrees(m, u64::MAX).unwrap();
            self.miroir.synchroniser(&tete, &nouvelles, &mut self.horloge, t).unwrap();
        }
    }

    #[test]
    fn kid_et_serie_revoques_refuses() {
        let mut b = Banc::new();
        let revoque = [9u8; 32];
        let serie: Serie = [4; 16];
        let intrus = CleSignature::generer();
        b.publier(
            &[
                revocation(Cible::Cle(revoque), &b.journal_cle),
                revocation(cible_serie(&serie), &b.journal_cle),
                // Signée par une clé non épinglée : ignorée.
                revocation(Cible::Cle([7; 32]), &intrus),
                // Série « 32 octets » non miyauth et artefact : ignorés.
                revocation(Cible::Serie([1; 32]), &b.journal_cle),
                revocation(Cible::Artefact([2; 32]), &b.journal_cle),
            ],
            T0,
        );
        let mut r = RevocationsRegistre::new(cog(), b.ancres(), 1);
        assert_eq!(r.rafraichir(&b.miroir, T0).unwrap(), 2);
        assert_eq!(r.est_revoque(&cog(), &CibleRevocation::Cle(revoque), T0), Ok(true));
        assert_eq!(r.est_revoque(&cog(), &CibleRevocation::Serie(serie), T0), Ok(true));
        assert_eq!(r.est_revoque(&cog(), &CibleRevocation::Cle([7; 32]), T0), Ok(false));
        assert_eq!(r.est_revoque(&cog(), &CibleRevocation::Serie([1; 16]), T0), Ok(false));
        // Autre COG émetteur : aucune liste.
        let autre = CogId::new("zyxwvutsrqponmlkjihgfedcba").unwrap();
        assert_eq!(r.est_revoque(&autre, &CibleRevocation::Cle(revoque), T0), Err(ErreurRevocation::ListeAbsente));

        // Rafraîchissement incrémental : seule la nouvelle entrée est lue.
        b.publier(&[revocation(Cible::Cle([5; 32]), &b.journal_cle)], T0 + 60);
        assert_eq!(r.rafraichir(&b.miroir, T0 + 60).unwrap(), 1);
        assert_eq!(r.est_revoque(&cog(), &CibleRevocation::Cle([5; 32]), T0 + 60), Ok(true));
        assert_eq!(r.a_jour_au(), Some(T0 + 60));
    }

    #[test]
    fn tete_perimee_ou_absente_echoue_ferme() {
        let mut b = Banc::new();
        let mut r = RevocationsRegistre::new(cog(), b.ancres(), 1);
        // Aucune tête : liste absente, rafraîchissement refusé.
        assert!(r.rafraichir(&b.miroir, T0).is_err());
        assert_eq!(r.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0), Err(ErreurRevocation::ListeAbsente));

        b.publier(&[revocation(Cible::Cle([1; 32]), &b.journal_cle)], T0);
        r.rafraichir(&b.miroir, T0).unwrap();
        // Au-delà de 15 min sans tête plus récente : périmée, même pour une cible non révoquée.
        let tard = T0 + FRAICHEUR_MAX_DEFAUT_SECS + 1;
        assert!(matches!(
            r.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), tard),
            Err(ErreurRevocation::Perimee { a_jour_au: T0, .. })
        ));
        // Tête gelée (au-delà de 24 h) : rafraîchissement refusé, liste inchangée.
        assert!(r.rafraichir(&b.miroir, T0 + 2 * 86_400).is_err());
        assert_eq!(r.a_jour_au(), Some(T0));
        // Horloge locale très en retard sur la tête : refus.
        assert_eq!(
            r.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), T0 - 3_600),
            Err(ErreurRevocation::HorlogeIncoherente)
        );
        // Fraîcheur plus courte admise, plus longue ramenée à 15 min.
        let courte = RevocationsRegistre::new(cog(), b.ancres(), 1).avec_fraicheur(60);
        assert_eq!(courte.fraicheur_max, 60);
        let longue = RevocationsRegistre::new(cog(), b.ancres(), 1).avec_fraicheur(86_400);
        assert_eq!(longue.fraicheur_max, FRAICHEUR_MAX_DEFAUT_SECS);
    }
}
