//! @id     miyuregistre_suivi_revocations
//! @do     suivre_registre_revocations_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — suivi unique du registre de révocation d'un COG (miroir vérifié + liste miyauth, RwLock, fraîcheur ≤ 15 min imposée) ; partagé par Origin, MiyuCloud et les façades.

//! Suivi **unique** du registre de révocation d'un COG (fonctionnalité `miyauth`).
//!
//! Remplace les trois copies (Origin, MiyuCloud, façade) qui divergeaient
//! (niveau de l'ancre, position exacte ou chevauchement, bornes, verrou,
//! fraîcheur recodée). [`SuiviRevocationsRegistre`] réunit :
//!
//! - un [`MiroirRegistre`] en mémoire (copie intégrale, racine recalculée,
//!   tête signée par la **clé de journal épinglée**, seuil 1, politique
//!   [`PolitiqueTete::journal_cog`], monotonie, équivoque, MTP) ;
//! - une [`RevocationsRegistre`] (entrées `Revocation` scellées par la même
//!   clé ; horodatage = tête vérifiée) ;
//! - une [`HorlogeConfiance`] propre.
//!
//! # Choix figés par le type (à consigner par amendement)
//!
//! - **Ancre unique** : la clé de journal, au **seuil 1**, au niveau
//!   [`NiveauConfiance::Reconnu`]. Justification : le suivi est toujours tenu
//!   par un **vérificateur extérieur** au registre (Origin pour un COG
//!   reconnu, MiyuCloud ou une façade pour le COG d'origine) ; la clé n'a
//!   d'autorité que sur ce registre et ne doit jamais être présentée comme
//!   souveraine (BorderGuard `Trusted`) à un consommateur du résultat — moindre
//!   privilège. Le niveau ne change pas la vérification (signature + seuil).
//! - **Fraîcheur** : [`FRAICHEUR_REVOCATIONS_SECS`] (15 min), **non
//!   configurable** ; mesurée sur la tête vérifiée, jamais sur l'heure de remise.
//! - **Chevauchement toléré** : [`SuiviRevocationsRegistre::appliquer`] accepte
//!   des entrées commençant à `depuis ≤ taille du miroir` (les entrées déjà
//!   détenues sont ignorées) et couvrant au moins la tête.
//! - **Bornes** : [`MAX_ENTREES_SUIVI`] entrées et [`MAX_OCTETS_SUIVI`] octets
//!   d'entrées au plus par registre suivi (au-delà : refus, rien n'est modifié).
//! - **Verrous** : un `RwLock` sur l'état ; les vérifications de révocation ne
//!   prennent que la lecture. Les applications sont sérialisées par un verrou
//!   distinct ; tête, racine et sceaux des entrées sont vérifiés **sous
//!   lecture** (les vérifications concurrentes continuent) ; l'écriture ne fait
//!   que poser le résultat. Rejeu à l'identique (même tête, rien de neuf) :
//!   accepté sans recalcul et sans écriture.
//!
//! # Fail-closed
//!
//! Jamais appliqué, autre COG émetteur ⇒ [`ErreurRevocation::ListeAbsente`] ;
//! tête de plus de 15 min ⇒ [`ErreurRevocation::Perimee`] ; tête datée dans le
//! futur ⇒ [`ErreurRevocation::HorlogeIncoherente`] ; verrou empoisonné ⇒
//! [`ErreurRevocation::Indisponible`]. Toute application refusée laisse l'état
//! inchangé (la liste détenue se périme d'elle-même).

use std::sync::{Mutex, RwLock};

use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyusceau::{verifier_tete, AncresDeConfiance, NiveauConfiance, PolitiqueTete};
use sha2::{Digest, Sha256};

use crate::erreurs::ErreurJournal;
use crate::revocation::{controler_fraicheur, revocations_des, RevocationsRegistre, FRAICHEUR_REVOCATIONS_SECS};
use crate::stockage::StockageMemoire;
use crate::suivi::{MiroirRegistre, SuiviTetes};
use crate::temps::HorlogeConfiance;

/// Nombre maximal d'entrées d'un registre suivi (2^20 = 1 048 576).
pub const MAX_ENTREES_SUIVI: u64 = 1 << 20;
/// Taille maximale cumulée des entrées d'un registre suivi (256 Mio).
pub const MAX_OCTETS_SUIVI: u64 = 256 << 20;

/// Niveau de l'ancre de journal (voir le module).
const NIVEAU_CLE_JOURNAL: NiveauConfiance = NiveauConfiance::Reconnu;

/// Application refusée (rien n'est modifié).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusSuivi {
    /// Les entrées ne couvrent pas `taille..attendue` (trou ou manque).
    #[error("entrées {depuis}..{fin} ne couvrent pas {taille}..{attendue}")]
    Couverture {
        /// Premier index fourni.
        depuis: u64,
        /// Index suivant le dernier fourni.
        fin: u64,
        /// Taille actuelle du miroir.
        taille: u64,
        /// Taille annoncée par la tête (0 si la tête n'a pas été lue).
        attendue: u64,
    },
    /// Registre au-delà de [`MAX_ENTREES_SUIVI`] ou [`MAX_OCTETS_SUIVI`].
    #[error("registre au-delà des bornes du suivi")]
    Bornes,
    /// Tête ou entrées refusées par le registre (signature, fraîcheur, racine…).
    #[error("registre : {0}")]
    Registre(String),
    /// Verrou empoisonné.
    #[error("état du suivi indisponible")]
    EtatIndisponible,
}

impl From<ErreurJournal> for RefusSuivi {
    fn from(e: ErreurJournal) -> Self {
        Self::Registre(e.to_string())
    }
}

/// Application acceptée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Application {
    /// Taille du miroir après application.
    pub taille: u64,
    /// Horodatage de la tête vérifiée la plus récente (fraîcheur).
    pub a_jour_au: u64,
    /// Révocations nouvelles prises en compte.
    pub ajoutees: usize,
    /// Vrai si la tête était déjà acceptée et rien de neuf n'a été fourni.
    pub rejeu: bool,
}

#[derive(Debug)]
struct Etat {
    miroir: MiroirRegistre<StockageMemoire>,
    liste: RevocationsRegistre,
    horloge: HorlogeConfiance,
    /// SHA-256 du sceau de la dernière tête acceptée.
    tete: Option<[u8; 32]>,
    /// Octets d'entrées détenus.
    octets: u64,
}

/// Suivi du registre de révocation d'un COG (voir le module).
#[derive(Debug)]
pub struct SuiviRevocationsRegistre {
    cog: CogId,
    registre: [u8; 32],
    ancres: AncresDeConfiance,
    politique: PolitiqueTete,
    application: Mutex<()>,
    etat: RwLock<Etat>,
}

fn empreinte(octets: &[u8]) -> [u8; 32] {
    Sha256::digest(octets).into()
}

impl SuiviRevocationsRegistre {
    /// Suivi vide (refuse tout jusqu'à la première application réussie) du
    /// registre `registre` de `cog`, têtes et révocations signées par
    /// `cle_journal` (ancre unique, seuil 1). Clé invalide ⇒ erreur.
    pub fn new(cog: CogId, registre: [u8; 32], cle_journal: &[u8; 32]) -> Result<Self, RefusSuivi> {
        let mut ancres = AncresDeConfiance::new();
        ancres
            .ajouter(cle_journal, NIVEAU_CLE_JOURNAL)
            .map_err(|e| RefusSuivi::Registre(format!("clé de journal : {e}")))?;
        let politique = PolitiqueTete::journal_cog();
        let miroir = MiroirRegistre::ouvrir(SuiviTetes::new(registre, ancres.clone(), politique), StockageMemoire::new())?;
        let liste = RevocationsRegistre::new(cog.clone(), ancres.clone(), 1);
        Ok(Self {
            cog,
            registre,
            ancres,
            politique,
            application: Mutex::new(()),
            etat: RwLock::new(Etat { miroir, liste, horloge: HorlogeConfiance::new(), tete: None, octets: 0 }),
        })
    }

    /// COG suivi.
    pub fn cog(&self) -> &CogId {
        &self.cog
    }

    /// Identifiant du registre suivi.
    pub fn registre(&self) -> &[u8; 32] {
        &self.registre
    }

    /// Taille du miroir (`None` : verrou empoisonné).
    pub fn taille(&self) -> Option<u64> {
        Some(self.etat.read().ok()?.miroir.taille())
    }

    /// Horodatage de la tête vérifiée la plus récente.
    pub fn a_jour_au(&self) -> Option<u64> {
        self.etat.read().ok()?.liste.a_jour_au()
    }

    /// Instantané de la liste (ex. remise à une façade) ; `None` si jamais
    /// appliqué ou verrou empoisonné.
    pub fn liste(&self) -> Option<RevocationsRegistre> {
        let etat = self.etat.read().ok()?;
        etat.liste.a_jour_au()?;
        Some(etat.liste.clone())
    }

    /// Applique une tête signée `tete` et les entrées `entrees` (`entrees[k]`
    /// = entrée `depuis + k`). Chevauchement toléré : `depuis` ≤ taille du
    /// miroir, les entrées déjà détenues sont ignorées, celles au-delà de la
    /// tête aussi ; il faut couvrir `taille..tete.taille`.
    ///
    /// `maintenant` : heure murale (passée par l'horloge de confiance du suivi).
    /// Refus : rien n'est modifié.
    pub fn appliquer(
        &self,
        tete: &[u8],
        depuis: u64,
        entrees: &[Vec<u8>],
        maintenant: u64,
    ) -> Result<Application, RefusSuivi> {
        // Applications sérialisées : l'état ne change pas entre la préparation
        // (sous lecture) et l'écriture.
        let _application = self.application.lock().map_err(|_| RefusSuivi::EtatIndisponible)?;
        let empreinte_tete = empreinte(tete);
        let fin = depuis.saturating_add(entrees.len() as u64);

        let (preparee, nouvelles, cibles, horloge, octets) = {
            let etat = self.etat.read().map_err(|_| RefusSuivi::EtatIndisponible)?;
            let m = etat.miroir.taille();
            if depuis > m {
                return Err(RefusSuivi::Couverture { depuis, fin, taille: m, attendue: 0 });
            }
            if etat.tete == Some(empreinte_tete) && fin <= m {
                // Rejeu à l'identique : déjà vérifié et appliqué, rien de neuf.
                return Ok(Application {
                    taille: m,
                    a_jour_au: etat.liste.a_jour_au().unwrap_or_default(),
                    ajoutees: 0,
                    rejeu: true,
                });
            }
            let mut horloge = etat.horloge.clone();
            let maintenant = horloge.maintenant(maintenant).map_err(ErreurJournal::from)?;
            // Signature de la tête avant tout calcul (taille annoncée).
            let n = verifier_tete(tete, &self.ancres, &self.politique, &self.registre, maintenant)
                .map_err(ErreurJournal::from)?
                .tete()
                .taille;
            if n > MAX_ENTREES_SUIVI {
                return Err(RefusSuivi::Bornes);
            }
            if fin < n {
                return Err(RefusSuivi::Couverture { depuis, fin, taille: m, attendue: n });
            }
            let debut = usize::try_from(m.saturating_sub(depuis)).map_err(|_| RefusSuivi::Bornes)?;
            let arret = usize::try_from(n.saturating_sub(depuis)).map_err(|_| RefusSuivi::Bornes)?;
            let nouvelles = entrees.get(debut..arret.max(debut)).unwrap_or(&[]);
            let octets = nouvelles.iter().fold(etat.octets, |t, e| t.saturating_add(e.len() as u64));
            if octets > MAX_OCTETS_SUIVI {
                return Err(RefusSuivi::Bornes);
            }
            let preparee = etat.miroir.preparer_synchro(tete, nouvelles, maintenant)?;
            let cibles = revocations_des(nouvelles, &self.ancres, 1, maintenant);
            (preparee, nouvelles, cibles, horloge, octets)
        };

        let mut etat = self.etat.write().map_err(|_| RefusSuivi::EtatIndisponible)?;
        let Etat { miroir, liste, horloge: horloge_etat, tete: tete_etat, octets: octets_etat } = &mut *etat;
        let emise_a = preparee.tete().emise_a;
        let mut horloge = horloge;
        let intervalle = miroir.appliquer_synchro(preparee, nouvelles, &mut horloge)?;
        let ajoutees = liste.integrer(cibles, intervalle.end, emise_a);
        *horloge_etat = horloge;
        *tete_etat = Some(empreinte_tete);
        *octets_etat = octets;
        Ok(Application {
            taille: miroir.taille(),
            a_jour_au: liste.a_jour_au().unwrap_or(emise_a),
            ajoutees,
            rejeu: false,
        })
    }
}

impl SourceRevocation for SuiviRevocationsRegistre {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        if emetteur != &self.cog {
            return Err(ErreurRevocation::ListeAbsente);
        }
        let etat = self.etat.read().map_err(|_| ErreurRevocation::Indisponible("verrou".into()))?;
        let a_jour_au = etat.liste.a_jour_au().ok_or(ErreurRevocation::ListeAbsente)?;
        // Plafond du type (15 min) contrôlé ici aussi : il ne dépend d'aucun réglage de la liste.
        controler_fraicheur(a_jour_au, maintenant_secs)?;
        etat.liste.est_revoque(emetteur, cible, maintenant_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entree::{Cible, Corps, EntreeRegistre};
    use crate::journal::Journal;
    use miyusceau::{sceller, CleSignature};

    const T0: u64 = 1_800_000_000;
    const REG: [u8; 32] = [0x31; 32];

    fn cog() -> CogId {
        CogId::new("abcdefghijklmnopqrstuvwxyz").unwrap()
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

        fn suivi(&self) -> SuiviRevocationsRegistre {
            SuiviRevocationsRegistre::new(cog(), REG, &self.cle.cle_publique()).unwrap()
        }

        fn revoquer(&mut self, kid: [u8; 32], t: u64) {
            let e = EntreeRegistre { inscrite_a: t, corps: Corps::Revocation { cible: Cible::Cle(kid), motif: "x".into() } };
            self.journal.ajouter(&sceller(&e, &self.cle).unwrap()).unwrap();
        }

        fn publier(&mut self, t: u64) -> (Vec<u8>, Vec<Vec<u8>>) {
            (self.journal.emettre_tete(&[&self.cle], t, 86_400).unwrap(), self.journal.entrees(0, u64::MAX).unwrap())
        }
    }

    #[test]
    fn chevauchement_tolere_et_trou_refuse() {
        let mut c = Cog::new();
        let s = c.suivi();
        c.revoquer([1; 32], T0);
        c.revoquer([2; 32], T0);
        let (tete, entrees) = c.publier(T0);
        let a = s.appliquer(&tete, 0, &entrees, T0).unwrap();
        assert_eq!((a.taille, a.ajoutees, a.a_jour_au, a.rejeu), (2, 2, T0, false));
        // Nouvelle entrée, envoi depuis 0 (chevauchement) : accepté, seule la nouvelle compte.
        c.revoquer([3; 32], T0 + 10);
        let (tete, entrees) = c.publier(T0 + 10);
        assert_eq!(s.appliquer(&tete, 0, &entrees, T0 + 10).unwrap().ajoutees, 1);
        // Trou (depuis > taille) et manque (fin < taille de la tête) : refus.
        c.revoquer([4; 32], T0 + 20);
        let (tete, entrees) = c.publier(T0 + 20);
        assert!(matches!(s.appliquer(&tete, 4, &entrees[3..], T0 + 20), Err(RefusSuivi::Couverture { .. })));
        assert!(matches!(s.appliquer(&tete, 0, &entrees[..3], T0 + 20), Err(RefusSuivi::Couverture { attendue: 4, .. })));
        assert_eq!(s.taille(), Some(3));
        assert_eq!(s.est_revoque(&cog(), &CibleRevocation::Cle([4; 32]), T0 + 20), Ok(false));
        assert_eq!(s.appliquer(&tete, 3, &entrees[3..], T0 + 20).unwrap().ajoutees, 1);
        assert_eq!(s.est_revoque(&cog(), &CibleRevocation::Cle([4; 32]), T0 + 20), Ok(true));
    }

    #[test]
    fn rejeu_identique_sans_recalcul() {
        let mut c = Cog::new();
        let s = c.suivi();
        c.revoquer([1; 32], T0);
        let (tete, entrees) = c.publier(T0);
        s.appliquer(&tete, 0, &entrees, T0).unwrap();
        // Même tête, rien de neuf : rejeu, même avec des entrées déjà détenues
        // (et même si elles sont fausses : elles ne sont pas relues).
        let r = s.appliquer(&tete, 0, &[b"faux".to_vec()], T0 + 5).unwrap();
        assert!(r.rejeu);
        assert_eq!((r.taille, r.ajoutees, r.a_jour_au), (1, 0, T0));
        // Un rejeu ne rafraîchit pas : la liste se périme 15 min après la tête.
        assert!(matches!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([9; 32]), T0 + FRAICHEUR_REVOCATIONS_SECS + 1),
            Err(ErreurRevocation::Perimee { .. })
        ));
    }

    #[test]
    fn fraicheur_imposee_et_cle_unique() {
        let mut c = Cog::new();
        let s = c.suivi();
        assert_eq!(s.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0), Err(ErreurRevocation::ListeAbsente));
        assert!(s.liste().is_none());
        let (tete, entrees) = c.publier(T0);
        s.appliquer(&tete, 0, &entrees, T0).unwrap();
        assert_eq!(s.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0 + FRAICHEUR_REVOCATIONS_SECS), Ok(false));
        assert!(matches!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0 + FRAICHEUR_REVOCATIONS_SECS + 1),
            Err(ErreurRevocation::Perimee { .. })
        ));
        assert_eq!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0 - 3_600),
            Err(ErreurRevocation::HorlogeIncoherente)
        );
        let autre = CogId::new("zyxwvutsrqponmlkjihgfedcba").unwrap();
        assert_eq!(s.est_revoque(&autre, &CibleRevocation::Cle([1; 32]), T0), Err(ErreurRevocation::ListeAbsente));
        // Tête d'un autre journal : refus, état inchangé.
        let mut intrus = Cog::new();
        let (tete, entrees) = intrus.publier(T0 + 1);
        assert!(matches!(s.appliquer(&tete, 0, &entrees, T0 + 1), Err(RefusSuivi::Registre(_))));
        assert_eq!(s.a_jour_au(), Some(T0));
        assert!(SuiviRevocationsRegistre::new(cog(), REG, &[0; 32]).is_err());
    }
}
