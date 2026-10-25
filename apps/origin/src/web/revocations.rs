//! @id     origin_web_revocations
//! @do     suivre_revocations_cogs_reconnus
//! @role   securite
//! @layer  app
//! @human  origin — révocations des COG reconnus : miroir vérifié de leur registre, `RevocationsRegistre`, fraîcheur ≤ 15 min, refus sinon.

//! Révocations des COG reconnus vues d'Origin (lot 2 §8.2, A-003, A-007).
//!
//! Jusqu'ici Origin n'avait aucune liste de révocation des COG reconnus et se
//! reposait sur la durée ≤ 15 min des passeports. Désormais chaque COG reconnu
//! dont le registre est déclaré (`[[forum.cogs_reconnus]]` : `registre`,
//! `cle_journal`) a ici :
//!
//! - un [`MiroirRegistre`] (copie intégrale, racine recalculée, tête signée par
//!   la clé de journal épinglée, fraîcheur 24 h, monotonie, équivoque) ;
//! - une [`RevocationsRegistre`] (fonctionnalité `miyauth` de `miyuregistre`) :
//!   entrées `Revocation` au seuil 1 sur la clé de journal, horodatage = tête
//!   vérifiée, **fraîcheur 15 min**.
//!
//! # Transport (choix non fixé par la doc gelée, à consigner)
//!
//! Le COG **pousse** sa tête et ses entrées nouvelles (`POST
//! /api/auth/revocations/sync`, JSON `{cog_id, depuis, tete, entrees}`, titres
//! en base64 URL). Aucune authentification de transport : tout est vérifié par
//! les signatures (tête) et la racine Merkle (entrées) ; `depuis` doit être la
//! taille actuelle du miroir (sinon `409` avec la taille attendue). Au plus
//! [`MAX_ENTREES_SYNCHRO`] entrées par envoi.
//!
//! # Fail-closed
//!
//! COG sans registre déclaré, jamais synchronisé, tête de plus de 15 min, datée
//! dans le futur, verrou empoisonné ⇒ erreur de révocation, donc **refus** du
//! passeport ou du visa par miyauth. Miroir en mémoire : après redémarrage, le
//! COG renvoie son registre depuis l'index 0 (première tête acceptée au premier
//! usage, comme le registre fédéral) ; une tête rejouée de plus de 15 min ne rend
//! pas la liste fraîche.

use std::collections::BTreeMap;
use std::sync::Mutex;

use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{HorlogeConfiance, MiroirRegistre, RevocationsRegistre, StockageMemoire, SuiviTetes};
use miyusceau::{AncresDeConfiance, NiveauConfiance, PolitiqueTete};

use crate::config::ForumConfig;

/// Nombre maximal d'entrées par synchronisation (borne du relevé, A-006).
pub const MAX_ENTREES_SYNCHRO: usize = 4096;

/// Registre déclaré d'un COG reconnu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistreReconnu {
    /// COG reconnu.
    pub cog: CogId,
    /// Identifiant de son registre (32 o).
    pub registre: [u8; 32],
    /// Clé publique de journal : signe ses têtes et ses entrées de révocation.
    pub cle_journal: [u8; 32],
}

/// Refus d'une synchronisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusSynchro {
    /// COG sans registre déclaré.
    CogInconnu,
    /// `depuis` ≠ taille du miroir.
    Position {
        /// Taille actuelle du miroir.
        taille: u64,
    },
    /// Trop d'entrées dans un envoi.
    TropDEntrees,
    /// Tête ou entrées refusées par le registre (signature, fraîcheur, racine…).
    Registre(String),
    /// État indisponible.
    EtatIndisponible,
}

impl std::fmt::Display for RefusSynchro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CogInconnu => f.write_str("COG sans registre déclaré"),
            Self::Position { taille } => write!(f, "position incorrecte (taille du miroir : {taille})"),
            Self::TropDEntrees => write!(f, "plus de {MAX_ENTREES_SYNCHRO} entrées"),
            Self::Registre(e) => write!(f, "registre : {e}"),
            Self::EtatIndisponible => f.write_str("état indisponible"),
        }
    }
}

/// Synchronisation acceptée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Synchronise {
    /// Taille du miroir après synchronisation.
    pub taille: u64,
    /// Horodatage de la tête vérifiée (fraîcheur des révocations).
    pub a_jour_au: u64,
    /// Révocations nouvelles prises en compte.
    pub ajoutees: usize,
}

#[derive(Debug)]
struct SuiviCog {
    miroir: MiroirRegistre<StockageMemoire>,
    liste: RevocationsRegistre,
    horloge: HorlogeConfiance,
}

/// Révocations des COG reconnus (une liste par COG, tirée de son registre vérifié).
#[derive(Debug, Default)]
pub struct RevocationsReconnues {
    cogs: Mutex<BTreeMap<CogId, SuiviCog>>,
}

fn ancres(cle: &[u8; 32]) -> Result<AncresDeConfiance, String> {
    let mut a = AncresDeConfiance::new();
    a.ajouter(cle, NiveauConfiance::Reconnu).map_err(|e| e.to_string())?;
    Ok(a)
}

impl RevocationsReconnues {
    /// Suivi des registres déclarés. Doublon de COG ou clé invalide ⇒ erreur.
    pub fn new(registres: &[RegistreReconnu]) -> Result<Self, String> {
        let mut cogs = BTreeMap::new();
        for r in registres {
            let ancres_journal = ancres(&r.cle_journal).map_err(|e| format!("cle_journal ({}) : {e}", r.cog))?;
            let suivi = SuiviTetes::new(r.registre, ancres_journal.clone(), PolitiqueTete::journal_cog());
            let miroir = MiroirRegistre::ouvrir(suivi, StockageMemoire::new()).map_err(|e| format!("registre ({}) : {e}", r.cog))?;
            let liste = RevocationsRegistre::new(r.cog.clone(), ancres_journal, 1);
            let ancien = cogs.insert(r.cog.clone(), SuiviCog { miroir, liste, horloge: HorlogeConfiance::new() });
            if ancien.is_some() {
                return Err(format!("registre déclaré deux fois pour {}", r.cog));
            }
        }
        Ok(Self { cogs: Mutex::new(cogs) })
    }

    /// Registres déclarés dans `[[forum.cogs_reconnus]]` (`registre` + `cle_journal`).
    /// Un seul des deux champs, valeur invalide ⇒ erreur (aucune entrée ignorée).
    pub fn depuis_config(config: &ForumConfig) -> Result<Self, String> {
        let hex32 = |texte: &str, champ: &str| -> Result<[u8; 32], String> {
            hex::decode(texte.trim())
                .ok()
                .and_then(|v| v.try_into().ok())
                .ok_or_else(|| format!("forum.cogs_reconnus.{champ} : 32 octets hexadécimaux attendus"))
        };
        let mut registres = Vec::new();
        for c in &config.cogs_reconnus {
            let cog = CogId::new(c.cog_id.trim()).ok_or_else(|| format!("forum.cogs_reconnus : cog_id invalide « {} »", c.cog_id))?;
            match (c.registre.as_deref(), c.cle_journal.as_deref()) {
                (None, None) => {}
                (Some(registre), Some(cle)) => registres.push(RegistreReconnu {
                    cog,
                    registre: hex32(registre, "registre")?,
                    cle_journal: hex32(cle, "cle_journal")?,
                }),
                _ => return Err(format!("forum.cogs_reconnus ({cog}) : `registre` et `cle_journal` vont ensemble")),
            }
        }
        Self::new(&registres)
    }

    /// Vrai si le registre de `cog` est suivi.
    pub fn suit(&self, cog: &CogId) -> bool {
        self.cogs.lock().is_ok_and(|c| c.contains_key(cog))
    }

    /// Taille du miroir de `cog`.
    pub fn taille(&self, cog: &CogId) -> Option<u64> {
        self.cogs.lock().ok()?.get(cog).map(|s| s.miroir.taille())
    }

    /// Synchronise le miroir de `cog` sur `tete` et les entrées `depuis..` puis
    /// rafraîchit sa liste. Rien n'est modifié en cas de refus.
    pub fn synchroniser(
        &self,
        cog: &CogId,
        depuis: u64,
        tete: &[u8],
        entrees: &[Vec<u8>],
        maintenant: u64,
    ) -> Result<Synchronise, RefusSynchro> {
        if entrees.len() > MAX_ENTREES_SYNCHRO {
            return Err(RefusSynchro::TropDEntrees);
        }
        let mut cogs = self.cogs.lock().map_err(|_| RefusSynchro::EtatIndisponible)?;
        let suivi = cogs.get_mut(cog).ok_or(RefusSynchro::CogInconnu)?;
        let taille = suivi.miroir.taille();
        if depuis != taille {
            return Err(RefusSynchro::Position { taille });
        }
        let maintenant = suivi.horloge.maintenant(maintenant).map_err(|e| RefusSynchro::Registre(e.to_string()))?;
        suivi
            .miroir
            .synchroniser(tete, entrees, &mut suivi.horloge, maintenant)
            .map_err(|e| RefusSynchro::Registre(e.to_string()))?;
        let ajoutees = suivi.liste.rafraichir(&suivi.miroir, maintenant).map_err(|e| RefusSynchro::Registre(e.to_string()))?;
        Ok(Synchronise {
            taille: suivi.miroir.taille(),
            a_jour_au: suivi.liste.a_jour_au().unwrap_or(0),
            ajoutees,
        })
    }
}

impl SourceRevocation for RevocationsReconnues {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        let cogs = self.cogs.lock().map_err(|_| ErreurRevocation::Indisponible("verrou".into()))?;
        cogs.get(emetteur).ok_or(ErreurRevocation::ListeAbsente)?.liste.est_revoque(emetteur, cible, maintenant_secs)
    }
}

/// Banc d'essai : COG reconnu tenant un journal (tests seulement).
#[cfg(test)]
pub(crate) mod banc {
    use super::*;
    use miyuregistre::{cible_serie, Cible, Corps, EntreeRegistre, Journal};
    use miyusceau::{sceller, CleSignature};

    /// Registre d'un COG reconnu (côté COG).
    pub(crate) struct RegistreCog {
        pub(crate) cog: CogId,
        pub(crate) registre: [u8; 32],
        pub(crate) cle_journal: CleSignature,
        pub(crate) journal: Journal<StockageMemoire>,
    }

    impl RegistreCog {
        pub(crate) fn new(cog: CogId) -> Self {
            let registre = [0x52; 32];
            Self {
                journal: Journal::ouvrir(registre, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
                cog,
                registre,
                cle_journal: CleSignature::generer(),
            }
        }

        pub(crate) fn declaration(&self) -> RegistreReconnu {
            RegistreReconnu { cog: self.cog.clone(), registre: self.registre, cle_journal: self.cle_journal.cle_publique() }
        }

        /// Inscrit une révocation signée par `cle` (la clé de journal par défaut).
        pub(crate) fn revoquer(&mut self, cible: &CibleRevocation, cle: Option<&CleSignature>, t: u64) {
            let cible = match cible {
                CibleRevocation::Cle(kid) => Cible::Cle(*kid),
                CibleRevocation::Serie(s) => cible_serie(s),
            };
            let e = EntreeRegistre { inscrite_a: t, corps: Corps::Revocation { cible, motif: "test".into() } };
            let octets = sceller(&e, cle.unwrap_or(&self.cle_journal)).unwrap();
            self.journal.ajouter(&octets).unwrap();
        }

        /// Tête émise à `t` et entrées depuis `depuis`.
        pub(crate) fn publier(&mut self, depuis: u64, t: u64) -> (Vec<u8>, Vec<Vec<u8>>) {
            let tete = self.journal.emettre_tete(&[&self.cle_journal], t, 86_400).unwrap();
            (tete, self.journal.entrees(depuis, u64::MAX).unwrap())
        }

        /// Synchronise `origin` (tête à `t`).
        pub(crate) fn pousser(&mut self, origin: &RevocationsReconnues, t: u64) -> Result<Synchronise, RefusSynchro> {
            let depuis = origin.taille(&self.cog).unwrap_or(0);
            let (tete, entrees) = self.publier(depuis, t);
            origin.synchroniser(&self.cog, depuis, &tete, &entrees, t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::banc::RegistreCog;
    use super::*;
    use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
    use miyusceau::CleSignature;

    const T0: u64 = 1_800_000_000;

    fn cog(c: char) -> CogId {
        CogId::new(&c.to_string().repeat(26)).unwrap()
    }

    #[test]
    fn revocations_du_registre_appliquees() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        let cible = CibleRevocation::Cle([7; 32]);
        // Jamais synchronisé : liste absente (refus).
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0), Err(ErreurRevocation::ListeAbsente));

        r.revoquer(&cible, None, T0);
        r.revoquer(&CibleRevocation::Serie([4; 16]), None, T0);
        // Signée par une clé non épinglée : ignorée.
        let intrus = CleSignature::generer();
        r.revoquer(&CibleRevocation::Cle([8; 32]), Some(&intrus), T0);
        let s = r.pousser(&origin, T0).unwrap();
        assert_eq!((s.taille, s.a_jour_au, s.ajoutees), (3, T0, 2));
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0), Ok(true));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Serie([4; 16]), T0), Ok(true));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([8; 32]), T0), Ok(false));
        // COG non déclaré : absente.
        assert_eq!(origin.est_revoque(&cog('z'), &cible, T0), Err(ErreurRevocation::ListeAbsente));

        // Synchronisation incrémentale.
        r.revoquer(&CibleRevocation::Cle([9; 32]), None, T0 + 60);
        assert_eq!(r.pousser(&origin, T0 + 60).unwrap().ajoutees, 1);
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([9; 32]), T0 + 60), Ok(true));
    }

    #[test]
    fn declaration_depuis_config() {
        use crate::config::{CogReconnuConfig, ForumConfig};
        let entree = |registre: Option<&str>, cle: Option<&str>| CogReconnuConfig {
            cog_id: "h".repeat(26),
            cle_emission: "11".repeat(32),
            registre: registre.map(str::to_string),
            cle_journal: cle.map(str::to_string),
        };
        let config = |e: CogReconnuConfig| ForumConfig { cogs_reconnus: vec![e], ..Default::default() };
        let cle = hex::encode(CleSignature::generer().cle_publique());
        let r = RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), Some(&cle)))).unwrap();
        assert!(r.suit(&cog('h')));
        // Sans registre déclaré : non suivi (ses passeports seront refusés).
        assert!(!RevocationsReconnues::depuis_config(&config(entree(None, None))).unwrap().suit(&cog('h')));
        // Un seul des deux champs, ou valeur invalide : erreur.
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), None))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(None, Some(&cle)))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some("52"), Some(&cle)))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), Some(&"00".repeat(32))))).is_err());
    }

    #[test]
    fn perimee_ou_invalide_refusee() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        r.pousser(&origin, T0).unwrap();
        let cible = CibleRevocation::Cle([1; 32]);
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0 + FRAICHEUR_MAX_DEFAUT_SECS), Ok(false));
        // Au-delà de 15 min sans nouvelle tête : périmée, refus.
        assert!(matches!(
            origin.est_revoque(&r.cog, &cible, T0 + FRAICHEUR_MAX_DEFAUT_SECS + 1),
            Err(ErreurRevocation::Perimee { .. })
        ));

        // Mauvaise position : 409 avec la taille.
        let (tete, entrees) = r.publier(0, T0 + 10);
        assert_eq!(origin.synchroniser(&r.cog, 5, &tete, &entrees, T0 + 10), Err(RefusSynchro::Position { taille: 0 }));
        // Tête signée par une autre clé que la clé de journal : refus, état inchangé.
        let mut faux = RegistreCog::new(r.cog.clone());
        let (tete, entrees) = faux.publier(0, T0 + 20);
        assert!(matches!(origin.synchroniser(&r.cog, 0, &tete, &entrees, T0 + 20), Err(RefusSynchro::Registre(_))));
        // Entrée omise (racine différente) : refus.
        r.revoquer(&cible, None, T0 + 30);
        let (tete, _) = r.publier(0, T0 + 30);
        assert!(matches!(origin.synchroniser(&r.cog, 0, &tete, &[], T0 + 30), Err(RefusSynchro::Registre(_))));
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0 + 30), Ok(false));
        // COG inconnu, trop d'entrées.
        assert_eq!(origin.synchroniser(&cog('z'), 0, &tete, &[], T0), Err(RefusSynchro::CogInconnu));
        let trop = vec![Vec::new(); MAX_ENTREES_SYNCHRO + 1];
        assert_eq!(origin.synchroniser(&r.cog, 0, &tete, &trop, T0), Err(RefusSynchro::TropDEntrees));
        // Déclaration en double refusée.
        assert!(RevocationsReconnues::new(&[r.declaration(), r.declaration()]).is_err());
    }
}
