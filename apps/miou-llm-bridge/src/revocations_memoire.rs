//! @id     maia_revocations_memoire
//! @do     verifier_revocations_registre_pour_visas
//! @role   security
//! @layer  app
//! @human  MAIA — révocations des passeports et visas présentés aux routes mémoire : miroir vérifié du
//! @human  registre du COG (tête signée par la clé de journal épinglée), fraîcheur ≤ 15 min, refus sinon.

//! # Révocations vues par les routes mémoire (I4)
//!
//! Source : [`miyuregistre::SuiviRevocationsRegistre`] (suivi partagé avec
//! MiyuCloud, Origin et les façades) : tête signée par la **clé de journal**
//! épinglée, racine recalculée sur la copie intégrale des entrées,
//! monotonie, fraîcheur ≤ 15 min imposée par le type.
//!
//! La zone COG dépose dans un répertoire (`MAIA_MEMOIRE_REVOCATIONS_DEPOT`) :
//!
//! - [`FICHIER_TETE`] : dernière tête signée du registre ;
//! - [`FICHIER_ENTREES`] : entrées du registre au format `MRGJ`.
//!
//! Le dépôt n'est **pas de confiance** : tout est revérifié ; le journal est
//! copié (lecture bornée) dans un fichier **privé** du dossier d'état avant
//! d'être ouvert, le dépôt n'est jamais modifié. Relevé toutes les
//! [`INTERVALLE_RELEVE_SECS`].
//!
//! Fail-closed : jamais relevée, relevé refusé puis péremption (15 min),
//! autre COG émetteur, aucune source ⇒ tout visa est refusé.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{RefusSuivi, Stockage, StockageFichier, SuiviRevocationsRegistre};

/// Fichier de la tête signée dans le dépôt.
pub const FICHIER_TETE: &str = "tete.sceau";
/// Fichier des entrées (format `MRGJ`) dans le dépôt.
pub const FICHIER_ENTREES: &str = "entrees.mrgj";
/// Taille maximale du fichier de tête.
pub const TAILLE_MAX_TETE: u64 = 1 << 20;
/// Taille maximale du journal relevé.
pub const TAILLE_MAX_JOURNAL: u64 = 256 << 20;
/// Période du relevé (bien inférieure à la fraîcheur de 15 min).
pub const INTERVALLE_RELEVE_SECS: u64 = 60;
/// Nom de la copie privée du journal (dans le dossier d'état).
const COPIE_JOURNAL: &str = "revocations-copie.mrgj";

/// Relevé refusé (rien n'est modifié).
#[derive(Debug, thiserror::Error)]
pub enum ErreurReleve {
    /// Aucune source configurée.
    #[error("aucune source de révocation configurée")]
    NonConfiguree,
    /// Fichier du dépôt absent, illisible ou trop grand.
    #[error("dépôt de révocations : {0}")]
    Depot(String),
    /// Tête ou entrées refusées par le registre.
    #[error("registre refusé : {0}")]
    Registre(String),
    /// État indisponible (verrou empoisonné).
    #[error("état des révocations indisponible")]
    Etat,
}

#[derive(Debug)]
struct Interieur {
    suivi: SuiviRevocationsRegistre,
    depot: Option<PathBuf>,
    copie: Option<PathBuf>,
}

/// Révocations du COG vues des routes mémoire ; clonable (tâche de relevé et
/// vérificateur partagent l'état).
#[derive(Debug, Clone)]
pub struct RevocationsMemoire {
    interieur: Option<Arc<Interieur>>,
}

impl RevocationsMemoire {
    /// Aucune source : tout titre est refusé.
    pub fn absentes() -> Self {
        Self { interieur: None }
    }

    /// Source pour le COG `cog` (registre `registre`, clé de journal
    /// `cle_journal`) relevant `depot` ; copie privée du journal dans
    /// `dossier_etat`. Liste vide : refuse tout jusqu'au premier relevé.
    pub fn new(
        cog: CogId,
        registre: [u8; 32],
        cle_journal: &[u8; 32],
        depot: Option<PathBuf>,
        dossier_etat: Option<&Path>,
    ) -> Result<Self, String> {
        let suivi = SuiviRevocationsRegistre::new(cog, registre, cle_journal)
            .map_err(|e| format!("clé de journal : {e}"))?;
        Ok(Self {
            interieur: Some(Arc::new(Interieur {
                suivi,
                depot,
                copie: dossier_etat.map(|d| d.join(COPIE_JOURNAL)),
            })),
        })
    }

    /// Vrai si une source est configurée.
    pub fn configuree(&self) -> bool {
        self.interieur.is_some()
    }

    /// Horodatage (tête vérifiée) de la liste détenue.
    pub fn a_jour_au(&self) -> Option<u64> {
        self.interieur.as_ref()?.suivi.a_jour_au()
    }

    /// Applique un relevé (tête signée, entrées depuis l'index `depuis`).
    pub fn appliquer_releve(
        &self,
        tete: &[u8],
        depuis: u64,
        entrees: &[Vec<u8>],
        maintenant: u64,
    ) -> Result<usize, ErreurReleve> {
        let i = self.interieur.as_ref().ok_or(ErreurReleve::NonConfiguree)?;
        match i.suivi.appliquer(tete, depuis, entrees, maintenant) {
            Ok(a) => Ok(a.ajoutees),
            Err(RefusSuivi::EtatIndisponible) => Err(ErreurReleve::Etat),
            Err(e) => Err(ErreurReleve::Registre(e.to_string())),
        }
    }

    /// Relève le dépôt configuré.
    pub async fn relever_depot(&self, maintenant: u64) -> Result<usize, ErreurReleve> {
        let i = self.interieur.as_ref().ok_or(ErreurReleve::NonConfiguree)?;
        let depot = i.depot.clone().ok_or(ErreurReleve::NonConfiguree)?;
        let copie = i.copie.clone().ok_or(ErreurReleve::NonConfiguree)?;
        let depuis = i.suivi.taille().ok_or(ErreurReleve::Etat)?;
        let (tete, entrees) =
            tokio::task::spawn_blocking(move || lire_depot(&depot, &copie, depuis))
                .await
                .map_err(|e| ErreurReleve::Depot(format!("tâche de lecture : {e}")))??;
        self.appliquer_releve(&tete, depuis, &entrees, maintenant)
    }

    /// Tâche de relevé périodique.
    pub fn lancer_releve(self) {
        if !self.configuree() {
            return;
        }
        tokio::spawn(async move {
            let mut intervalle =
                tokio::time::interval(std::time::Duration::from_secs(INTERVALLE_RELEVE_SECS));
            let mut dernier_ok = true;
            loop {
                intervalle.tick().await;
                match self
                    .relever_depot(miyauth::http::service::maintenant_unix())
                    .await
                {
                    Ok(n) => {
                        if n > 0 || !dernier_ok {
                            tracing::info!(target: "maia::memoire_http", "révocations : relevé accepté ({n} nouvelle(s))");
                        }
                        dernier_ok = true;
                    }
                    Err(e) => {
                        // La liste détenue se périme (15 min) ⇒ visas refusés.
                        tracing::warn!(target: "maia::memoire_http", "révocations : relevé refusé : {e}");
                        dernier_ok = false;
                    }
                }
            }
        });
    }
}

/// Lit au plus `max` octets de `chemin` dans `sortie` ; refus au-delà.
fn lire_borne(
    chemin: &Path,
    max: u64,
    sortie: &mut impl std::io::Write,
) -> Result<u64, ErreurReleve> {
    let nom = chemin
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let fichier =
        std::fs::File::open(chemin).map_err(|e| ErreurReleve::Depot(format!("{nom} : {e}")))?;
    let copies = std::io::copy(&mut fichier.take(max.saturating_add(1)), sortie)
        .map_err(|e| ErreurReleve::Depot(format!("{nom} : {e}")))?;
    if copies > max {
        return Err(ErreurReleve::Depot(format!(
            "{nom} : taille au-delà de {max} octets"
        )));
    }
    Ok(copies)
}

/// Lit la tête et les entrées `depuis..` du dépôt (copie privée bornée du journal).
fn lire_depot(
    depot: &Path,
    copie: &Path,
    depuis: u64,
) -> Result<(Vec<u8>, Vec<Vec<u8>>), ErreurReleve> {
    let err = |quoi: &str, e: &dyn std::fmt::Display| ErreurReleve::Depot(format!("{quoi} : {e}"));
    let mut tete = Vec::new();
    if lire_borne(&depot.join(FICHIER_TETE), TAILLE_MAX_TETE, &mut tete)? == 0 {
        return Err(ErreurReleve::Depot(format!("{FICHIER_TETE} : vide")));
    }
    // Copie privée neuve (création exclusive, 0600) : `StockageFichier::ouvrir`
    // peut réparer une fin tronquée, jamais sur le dépôt.
    let _ = std::fs::remove_file(copie);
    let mut f = {
        use std::os::unix::fs::OpenOptionsExt;
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(copie)
            .map_err(|e| err("copie du journal", &e))?
    };
    lire_borne(&depot.join(FICHIER_ENTREES), TAILLE_MAX_JOURNAL, &mut f)?;
    f.sync_all().map_err(|e| err("copie du journal", &e))?;
    drop(f);
    let stockage = StockageFichier::ouvrir(copie).map_err(|e| err(FICHIER_ENTREES, &e))?;
    let entrees = (depuis..stockage.taille())
        .map(|k| stockage.lire(k))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| err(FICHIER_ENTREES, &e))?;
    Ok((tete, entrees))
}

impl SourceRevocation for RevocationsMemoire {
    fn est_revoque(
        &self,
        emetteur: &CogId,
        cible: &CibleRevocation,
        maintenant_secs: u64,
    ) -> Result<bool, ErreurRevocation> {
        self.interieur
            .as_ref()
            .ok_or(ErreurRevocation::ListeAbsente)?
            .suivi
            .est_revoque(emetteur, cible, maintenant_secs)
    }
}

#[cfg(test)]
pub(crate) mod banc {
    //! Registre d'un COG d'essai (journal signé) et sa source.
    use super::*;
    use miyuregistre::{Cible, Corps, EntreeRegistre, Journal, StockageMemoire};
    use miyusceau::{sceller, CleSignature};

    pub(crate) const REGISTRE: [u8; 32] = [0x4d; 32];

    pub(crate) struct RegistreCog {
        pub cle_journal: CleSignature,
        pub journal: Journal<StockageMemoire>,
    }

    impl RegistreCog {
        pub(crate) fn new() -> Self {
            Self {
                cle_journal: CleSignature::generer(),
                journal: Journal::ouvrir(REGISTRE, StockageMemoire::new(), StockageMemoire::new())
                    .expect("journal"),
            }
        }

        pub(crate) fn source(&self, cog: &CogId) -> RevocationsMemoire {
            RevocationsMemoire::new(
                cog.clone(),
                REGISTRE,
                &self.cle_journal.cle_publique(),
                None,
                None,
            )
            .expect("source")
        }

        pub(crate) fn revoquer(&mut self, cible: Cible, t: u64) {
            let e = EntreeRegistre {
                inscrite_a: t,
                corps: Corps::Revocation {
                    cible,
                    motif: "perte".into(),
                },
            };
            self.journal
                .ajouter(&sceller(&e, &self.cle_journal).expect("sceau"))
                .expect("ajout");
        }

        pub(crate) fn publier(&mut self, t: u64) -> (Vec<u8>, Vec<Vec<u8>>) {
            let tete = self
                .journal
                .emettre_tete(&[&self.cle_journal], t, 86_400)
                .expect("tête");
            (tete, self.journal.entrees(0, u64::MAX).expect("entrées"))
        }

        pub(crate) fn relever(&mut self, source: &RevocationsMemoire, t: u64) -> usize {
            let (tete, entrees) = self.publier(t);
            source
                .appliquer_releve(&tete, 0, &entrees, t)
                .expect("relevé")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::banc::*;
    use super::*;

    const T0: u64 = 1_900_000_000;

    fn cog() -> CogId {
        CogId::new("abcdefghijklmnopqrstuvwxyz").expect("cog")
    }

    #[test]
    fn absente_ou_jamais_relevee_refuse_tout() {
        let s = RevocationsMemoire::absentes();
        assert_eq!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0),
            Err(ErreurRevocation::ListeAbsente)
        );
        let r = RegistreCog::new();
        let s = r.source(&cog());
        assert!(s
            .est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0)
            .is_err());
    }

    #[test]
    fn revocation_vue_puis_peremption() {
        let mut r = RegistreCog::new();
        let s = r.source(&cog());
        r.revoquer(miyuregistre::Cible::Cle([9; 32]), T0);
        assert_eq!(r.relever(&s, T0), 1);
        assert_eq!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([9; 32]), T0),
            Ok(true)
        );
        assert_eq!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), T0),
            Ok(false)
        );
        assert!(s
            .est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), T0 + 16 * 60)
            .is_err());
    }

    #[tokio::test]
    async fn releve_du_depot_par_copie_privee() {
        let base = std::env::temp_dir().join(format!(
            "maia-revocations-{}",
            crate::garde_etranger::identifiant_aleatoire()
        ));
        let depot = base.join("depot");
        let etat = base.join("etat");
        std::fs::create_dir_all(&depot).expect("dépôt");
        std::fs::create_dir_all(&etat).expect("état");
        let mut r = RegistreCog::new();
        r.revoquer(miyuregistre::Cible::Cle([7; 32]), T0);
        let t = miyauth::http::service::maintenant_unix();
        let (tete, entrees) = r.publier(t);
        std::fs::write(depot.join(FICHIER_TETE), &tete).expect("tête");
        {
            let mut st = StockageFichier::ouvrir(depot.join(FICHIER_ENTREES)).expect("st");
            for e in &entrees {
                st.ajouter(e).expect("ajout");
            }
        }
        let s = RevocationsMemoire::new(
            cog(),
            REGISTRE,
            &r.cle_journal.cle_publique(),
            Some(depot.clone()),
            Some(&etat),
        )
        .expect("source");
        assert_eq!(s.relever_depot(t).await.expect("relevé"), 1);
        assert_eq!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([7; 32]), t),
            Ok(true)
        );
        // Dépôt incomplet : refus, rien n'est modifié.
        std::fs::remove_file(depot.join(FICHIER_TETE)).expect("rm");
        assert!(s.relever_depot(t).await.is_err());
        let _ = std::fs::remove_dir_all(&base);
    }
}
