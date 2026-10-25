//! @id     miyucloud_infrastructure_services_miyu_revocations
//! @do     verifier_revocations_registre_cog
//! @role   securite
//! @layer  app
//! @human  miyucloud — révocations miyauth lues dans le registre vérifié du COG d'origine (miroir, fraîcheur ≤ 15 min, refus sinon).

//! Révocations du vérificateur miyauth de MiyuCloud (lot 2 §8.2, A-003, A-007).
//!
//! # Source
//!
//! [`miyuregistre::SuiviRevocationsRegistre`] (fonctionnalité `miyauth`), le
//! suivi **unique** partagé avec Origin et les façades : miroir vérifié du
//! registre du COG d'origine, tête signée par la **clé de journal épinglée**
//! (ancre unique, seuil 1, politique `journal_cog`), racine recalculée sur la
//! copie intégrale des entrées, monotonie et cohérence, `RwLock`, fraîcheur
//! ≤ 15 min imposée par le type. Seules les entrées `Revocation` scellées par
//! cette clé comptent (A-007).
//!
//! # Relevé (choix non fixé par la doc gelée)
//!
//! La zone COG dépose dans un répertoire (`MIYUCLOUD_MIYU_REVOCATIONS_DEPOT`) :
//!
//! - [`FICHIER_TETE`] : la dernière tête signée du registre (Sceau v1.1) ;
//! - [`FICHIER_ENTREES`] : les entrées du registre au format de stockage
//!   `MRGJ` de `miyuregistre` (copie du journal).
//!
//! Le dépôt n'est **pas de confiance** : tout est revérifié ; MiyuCloud copie
//! le journal dans un fichier privé avant de l'ouvrir (jamais d'écriture dans
//! le dépôt) ; la copie et la lecture de la tête sont **bornées**
//! ([`TAILLE_MAX_JOURNAL`], [`TAILLE_MAX_TETE`]) quelle que soit la taille
//! annoncée (fichier qui grossit entre le contrôle et la lecture : refus).
//! Le relevé est refait toutes les [`INTERVALLE_RELEVE_SECS`].
//!
//! # Fail-closed
//!
//! - Fraîcheur : **15 min** au plus (A-007), mesurée sur l'horodatage de la
//!   **tête vérifiée** ; au-delà ⇒ [`ErreurRevocation::Perimee`] ⇒ visa refusé.
//! - Aucun relevé réussi, autre COG émetteur ⇒ [`ErreurRevocation::ListeAbsente`].
//! - Relevé refusé (signature, racine, équivoque, recul, fichier trop grand
//!   ou illisible) ⇒ rien n'est modifié ; la liste détenue se périme d'elle-même.
//! - **Aucune source configurée** alors que le visa est actif ⇒ source
//!   [`RevocationsMiyuCloud::absentes`] : **tout visa est refusé** (choix
//!   fail-closed ; l'ancien comportement « le COG contrôle à l'émission »
//!   n'existe plus).

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{RefusSuivi, Stockage, StockageFichier, SuiviRevocationsRegistre};

use super::miyu_auth_service::analyser_cle_publique;

/// Fichier de la tête signée dans le dépôt.
pub const FICHIER_TETE: &str = "tete.sceau";
/// Fichier des entrées (format `MRGJ`) dans le dépôt.
pub const FICHIER_ENTREES: &str = "entrees.mrgj";
/// Taille maximale du fichier de tête (un sceau fait au plus ~92 Kio).
pub const TAILLE_MAX_TETE: u64 = 1 << 20;
/// Taille maximale du journal relevé (borne mémoire du relevé).
pub const TAILLE_MAX_JOURNAL: u64 = 256 << 20;
/// Période du relevé du dépôt (bien inférieure à la fraîcheur de 15 min).
pub const INTERVALLE_RELEVE_SECS: u64 = 60;
/// Fraîcheur maximale des révocations (A-007 : 15 min, imposée par
/// [`SuiviRevocationsRegistre`]).
pub const FRAICHEUR_REVOCATIONS_SECS: u64 = miyuregistre::FRAICHEUR_REVOCATIONS_SECS;

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

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration de la source de révocations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigRevocations {
    /// Identifiant du registre du COG d'origine (32 octets).
    pub registre: [u8; 32],
    /// Clé publique de journal du COG d'origine (signe têtes et révocations).
    pub cle_journal: [u8; 32],
    /// Répertoire du dépôt.
    pub depot: PathBuf,
}

impl ConfigRevocations {
    /// Lit `MIYUCLOUD_MIYU_REGISTRE`, `MIYUCLOUD_MIYU_CLE_JOURNAL` et
    /// `MIYUCLOUD_MIYU_REVOCATIONS_DEPOT`. Tous absents : `Ok(None)` ; un
    /// sous-ensemble ou une valeur invalide : erreur (refus de démarrer).
    pub fn depuis_env() -> Result<Option<Self>, String> {
        Self::depuis_valeurs(
            std::env::var("MIYUCLOUD_MIYU_REGISTRE").ok().as_deref(),
            std::env::var("MIYUCLOUD_MIYU_CLE_JOURNAL").ok().as_deref(),
            std::env::var("MIYUCLOUD_MIYU_REVOCATIONS_DEPOT")
                .ok()
                .as_deref(),
        )
    }

    /// Variante pure de [`Self::depuis_env`].
    pub fn depuis_valeurs(
        registre: Option<&str>,
        cle_journal: Option<&str>,
        depot: Option<&str>,
    ) -> Result<Option<Self>, String> {
        match (nettoyer(registre), nettoyer(cle_journal), nettoyer(depot)) {
            (None, None, None) => Ok(None),
            (Some(r), Some(k), Some(d)) => {
                let registre: [u8; 32] = hex::decode(r)
                    .ok()
                    .and_then(|v| v.try_into().ok())
                    .ok_or("MIYUCLOUD_MIYU_REGISTRE : 64 caractères hexadécimaux attendus")?;
                let cle_journal = analyser_cle_publique(k).ok_or(
                    "MIYUCLOUD_MIYU_CLE_JOURNAL : clé publique Ed25519 (64 hex ou base64 URL) attendue",
                )?;
                Ok(Some(Self {
                    registre,
                    cle_journal,
                    depot: PathBuf::from(d),
                }))
            }
            _ => Err("MIYUCLOUD_MIYU_REGISTRE, MIYUCLOUD_MIYU_CLE_JOURNAL et \
                 MIYUCLOUD_MIYU_REVOCATIONS_DEPOT vont ensemble"
                .to_string()),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Source
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug)]
struct Interieur {
    suivi: SuiviRevocationsRegistre,
    depot: Option<PathBuf>,
}

/// Révocations du COG d'origine vues de MiyuCloud ; clonable (la tâche de
/// relevé et le vérificateur partagent le même état).
#[derive(Debug, Clone)]
pub struct RevocationsMiyuCloud {
    interieur: Option<Arc<Interieur>>,
}

impl RevocationsMiyuCloud {
    /// Aucune source : tout titre est refusé ([`ErreurRevocation::ListeAbsente`]).
    pub fn absentes() -> Self {
        Self { interieur: None }
    }

    /// Source pour le COG d'origine `cog`, registre et clé de journal de
    /// `config`. Liste vide : refuse tout jusqu'au premier relevé réussi.
    pub fn new(cog: CogId, config: &ConfigRevocations) -> Result<Self, String> {
        Self::construire(
            cog,
            config.registre,
            &config.cle_journal,
            Some(config.depot.clone()),
        )
    }

    fn construire(
        cog: CogId,
        registre: [u8; 32],
        cle_journal: &[u8; 32],
        depot: Option<PathBuf>,
    ) -> Result<Self, String> {
        let suivi = SuiviRevocationsRegistre::new(cog, registre, cle_journal)
            .map_err(|e| format!("MIYUCLOUD_MIYU_CLE_JOURNAL : {e}"))?;
        Ok(Self {
            interieur: Some(Arc::new(Interieur { suivi, depot })),
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

    /// Applique un relevé : tête signée `tete` et entrées du registre à partir
    /// de l'index `depuis` (`entrees[k]` = entrée `depuis + k`, chevauchement
    /// toléré). Rend le nombre de révocations ajoutées. Refus : rien n'est modifié.
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

    /// Relève le dépôt configuré (fichiers [`FICHIER_TETE`] et
    /// [`FICHIER_ENTREES`]). Le journal est copié dans un fichier privé avant
    /// d'être ouvert : le dépôt n'est jamais modifié.
    pub async fn relever_depot(&self, maintenant: u64) -> Result<usize, ErreurReleve> {
        let i = self.interieur.as_ref().ok_or(ErreurReleve::NonConfiguree)?;
        let depot = i.depot.clone().ok_or(ErreurReleve::NonConfiguree)?;
        let depuis = i.suivi.taille().ok_or(ErreurReleve::Etat)?;
        let (tete, entrees) = tokio::task::spawn_blocking(move || lire_depot(&depot, depuis))
            .await
            .map_err(|e| ErreurReleve::Depot(format!("tâche de lecture : {e}")))??;
        self.appliquer_releve(&tete, depuis, &entrees, maintenant)
    }

    /// Tâche de relevé périodique (toutes les [`INTERVALLE_RELEVE_SECS`]).
    pub fn lancer_releve(self, maintenant: fn() -> u64) {
        if !self.configuree() {
            return;
        }
        tokio::spawn(async move {
            let mut intervalle =
                tokio::time::interval(std::time::Duration::from_secs(INTERVALLE_RELEVE_SECS));
            let mut dernier_ok = true;
            loop {
                intervalle.tick().await;
                match self.relever_depot(maintenant()).await {
                    Ok(n) => {
                        if n > 0 || !dernier_ok {
                            tracing::info!(
                                "Révocations miyauth : relevé accepté ({n} nouvelle(s), à jour au {:?})",
                                self.a_jour_au()
                            );
                        }
                        dernier_ok = true;
                    }
                    Err(e) => {
                        // Refus : la liste détenue se périme (15 min) ⇒ visas refusés.
                        tracing::warn!("Révocations miyauth : relevé refusé : {e}");
                        dernier_ok = false;
                    }
                }
            }
        });
    }
}

/// Valeur de configuration renseignée (espaces retirés), sinon `None`.
fn nettoyer(v: Option<&str>) -> Option<&str> {
    v.map(str::trim).filter(|s| !s.is_empty())
}

/// Lit au plus `max` octets de `chemin` ; refus si le fichier en contient
/// davantage (même s'il a grossi depuis un contrôle de taille).
fn lire_borne(chemin: &Path, max: u64, sortie: &mut impl std::io::Write) -> Result<u64, ErreurReleve> {
    let nom = chemin.file_name().map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    let fichier = std::fs::File::open(chemin)
        .map_err(|e| ErreurReleve::Depot(format!("{nom} : {e}")))?;
    let copies = std::io::copy(&mut fichier.take(max.saturating_add(1)), sortie)
        .map_err(|e| ErreurReleve::Depot(format!("{nom} : {e}")))?;
    if copies > max {
        return Err(ErreurReleve::Depot(format!(
            "{nom} : taille au-delà de {max} octets"
        )));
    }
    Ok(copies)
}

/// Lit la tête et les entrées `depuis..` du dépôt (lectures bornées).
fn lire_depot(depot: &Path, depuis: u64) -> Result<(Vec<u8>, Vec<Vec<u8>>), ErreurReleve> {
    let err = |quoi: &str, e: &dyn std::fmt::Display| ErreurReleve::Depot(format!("{quoi} : {e}"));
    let mut tete = Vec::new();
    let taille_tete = lire_borne(&depot.join(FICHIER_TETE), TAILLE_MAX_TETE, &mut tete)?;
    if taille_tete == 0 {
        return Err(ErreurReleve::Depot(format!("{FICHIER_TETE} : vide")));
    }
    // Copie privée **bornée** : `StockageFichier::ouvrir` peut réparer une fin
    // tronquée, jamais sur le dépôt ; au-delà de TAILLE_MAX_JOURNAL, refus.
    let mut copie = tempfile::NamedTempFile::new().map_err(|e| err("copie du journal", &e))?;
    lire_borne(&depot.join(FICHIER_ENTREES), TAILLE_MAX_JOURNAL, copie.as_file_mut())?;
    copie
        .as_file_mut()
        .sync_all()
        .map_err(|e| err("copie du journal", &e))?;
    let stockage = StockageFichier::ouvrir(copie.path()).map_err(|e| err(FICHIER_ENTREES, &e))?;
    let entrees = (depuis..stockage.taille())
        .map(|k| stockage.lire(k))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| err(FICHIER_ENTREES, &e))?;
    Ok((tete, entrees))
}

impl SourceRevocation for RevocationsMiyuCloud {
    fn est_revoque(
        &self,
        emetteur: &CogId,
        cible: &CibleRevocation,
        maintenant_secs: u64,
    ) -> Result<bool, ErreurRevocation> {
        // Émetteur, fraîcheur (≤ 15 min, tête vérifiée) et verrou : contrôlés par le suivi.
        self.interieur
            .as_ref()
            .ok_or(ErreurRevocation::ListeAbsente)?
            .suivi
            .est_revoque(emetteur, cible, maintenant_secs)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registre de test
// ═══════════════════════════════════════════════════════════════════════════════

/// Registre d'un COG de test (journal signé) et sa source MiyuCloud.
#[cfg(test)]
pub(crate) mod banc {
    use super::*;
    use miyuregistre::{Cible, Corps, EntreeRegistre, Journal, StockageMemoire};
    use miyusceau::{CleSignature, sceller};

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
                    .unwrap(),
            }
        }

        /// Source MiyuCloud (sans dépôt) épinglée sur ce registre.
        pub(crate) fn source(&self, cog: &CogId) -> RevocationsMiyuCloud {
            RevocationsMiyuCloud::construire(
                cog.clone(),
                REGISTRE,
                &self.cle_journal.cle_publique(),
                None,
            )
            .unwrap()
        }

        /// Inscrit une révocation de clé signée par `cle` (par défaut, la clé de journal).
        pub(crate) fn revoquer_cle(&mut self, kid: [u8; 32], cle: Option<&CleSignature>, t: u64) {
            let cle = cle.unwrap_or(&self.cle_journal);
            let e = EntreeRegistre {
                inscrite_a: t,
                corps: Corps::Revocation {
                    cible: Cible::Cle(kid),
                    motif: "perte".into(),
                },
            };
            self.journal.ajouter(&sceller(&e, cle).unwrap()).unwrap();
        }

        /// Émet une tête à `t` ; rend (tête, toutes les entrées).
        pub(crate) fn publier(&mut self, t: u64) -> (Vec<u8>, Vec<Vec<u8>>) {
            let tete = self
                .journal
                .emettre_tete(&[&self.cle_journal], t, 86_400)
                .unwrap();
            (tete, self.journal.entrees(0, u64::MAX).unwrap())
        }

        /// Émet une tête et l'applique à `source`.
        pub(crate) fn relever(&mut self, source: &RevocationsMiyuCloud, t: u64) -> usize {
            let (tete, entrees) = self.publier(t);
            source.appliquer_releve(&tete, 0, &entrees, t).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::banc::*;
    use super::*;
    use miyusceau::{CleSignature, empreinte_cle};

    const T0: u64 = 1_900_000_000;

    fn cog() -> CogId {
        CogId::new("abcdefghijklmnopqrstuvwxyz").unwrap()
    }

    #[test]
    fn configuration_tout_ou_rien() {
        let cle = hex::encode([7u8; 32]);
        let reg = hex::encode([1u8; 32]);
        assert_eq!(
            ConfigRevocations::depuis_valeurs(None, None, None).unwrap(),
            None
        );
        assert!(ConfigRevocations::depuis_valeurs(Some(&reg), Some(&cle), None).is_err());
        assert!(ConfigRevocations::depuis_valeurs(None, None, Some("/x")).is_err());
        assert!(ConfigRevocations::depuis_valeurs(Some("zz"), Some(&cle), Some("/x")).is_err());
        let c = ConfigRevocations::depuis_valeurs(Some(&reg), Some(&cle), Some("/x"))
            .unwrap()
            .unwrap();
        assert_eq!(c.registre, [1u8; 32]);
        assert_eq!(c.cle_journal, [7u8; 32]);
    }

    #[test]
    fn source_absente_refuse_tout() {
        let s = RevocationsMiyuCloud::absentes();
        assert_eq!(
            s.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0),
            Err(ErreurRevocation::ListeAbsente)
        );
        assert!(matches!(
            s.appliquer_releve(b"x", 0, &[], T0),
            Err(ErreurReleve::NonConfiguree)
        ));
    }

    #[test]
    fn revocation_vue_puis_peremption_apres_15_min() {
        let mut r = RegistreCog::new();
        let source = r.source(&cog());
        // Jamais relevée : liste absente ⇒ refus.
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), T0),
            Err(ErreurRevocation::ListeAbsente)
        );
        let revoque = empreinte_cle(&[9u8; 32]);
        r.revoquer_cle(revoque, None, T0);
        assert_eq!(r.relever(&source, T0), 1);
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle(revoque), T0),
            Ok(true)
        );
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), T0),
            Ok(false)
        );
        // Autre COG émetteur : absente.
        let autre = CogId::new("zyxwvutsrqponmlkjihgfedcba").unwrap();
        assert_eq!(
            source.est_revoque(&autre, &CibleRevocation::Cle([2; 32]), T0),
            Err(ErreurRevocation::ListeAbsente)
        );
        // 15 min + 1 s sans nouvelle tête : périmée, même pour une cible saine.
        let tard = T0 + FRAICHEUR_REVOCATIONS_SECS + 1;
        assert!(matches!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), tard),
            Err(ErreurRevocation::Perimee { a_jour_au: T0, .. })
        ));
        // Nouvelle tête : de nouveau fraîche.
        assert_eq!(r.relever(&source, tard), 0);
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([2; 32]), tard),
            Ok(false)
        );
        assert_eq!(source.a_jour_au(), Some(tard));
    }

    #[test]
    fn releve_falsifie_refuse_sans_rien_modifier() {
        let mut r = RegistreCog::new();
        let source = r.source(&cog());
        assert_eq!(r.relever(&source, T0), 0);

        // Révocation signée par une clé non épinglée : ignorée.
        let intrus = CleSignature::generer();
        r.revoquer_cle([5; 32], Some(&intrus), T0 + 10);
        r.relever(&source, T0 + 10);
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([5; 32]), T0 + 10),
            Ok(false)
        );

        // Tête d'un autre journal (clé non épinglée) : refusée, état inchangé.
        let mut autre = RegistreCog::new();
        let (tete, entrees) = autre.publier(T0 + 20);
        assert!(matches!(
            source.appliquer_releve(&tete, 0, &entrees, T0 + 20),
            Err(ErreurReleve::Registre(_))
        ));
        assert_eq!(source.a_jour_au(), Some(T0 + 10));

        // Entrées altérées (racine différente) : refus.
        r.revoquer_cle([6; 32], None, T0 + 30);
        let (tete, mut entrees) = r.publier(T0 + 30);
        let derniere = entrees.len() - 1;
        entrees[derniere] = entrees[0].clone();
        assert!(
            source
                .appliquer_releve(&tete, 0, &entrees, T0 + 30)
                .is_err()
        );
        // Entrées manquantes : refus.
        assert!(
            source
                .appliquer_releve(&tete, 0, &entrees[..1], T0 + 30)
                .is_err()
        );
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([6; 32]), T0 + 30),
            Ok(false)
        );
    }

    #[tokio::test]
    async fn releve_du_depot_fichier() {
        let mut r = RegistreCog::new();
        let dir = tempfile::tempdir().unwrap();
        let revoque = [8u8; 32];
        r.revoquer_cle(revoque, None, T0);
        let (tete, entrees) = r.publier(T0);
        // Dépôt : tête + journal MRGJ.
        std::fs::write(dir.path().join(FICHIER_TETE), &tete).unwrap();
        {
            let mut s = StockageFichier::ouvrir(dir.path().join(FICHIER_ENTREES)).unwrap();
            for e in &entrees {
                s.ajouter(e).unwrap();
            }
        }
        let avant = std::fs::read(dir.path().join(FICHIER_ENTREES)).unwrap();
        let config = ConfigRevocations {
            registre: REGISTRE,
            cle_journal: r.cle_journal.cle_publique(),
            depot: dir.path().to_path_buf(),
        };
        let source = RevocationsMiyuCloud::new(cog(), &config).unwrap();
        assert_eq!(source.relever_depot(T0).await.unwrap(), 1);
        assert_eq!(
            source.est_revoque(&cog(), &CibleRevocation::Cle(revoque), T0),
            Ok(true)
        );
        // Le dépôt n'est jamais modifié.
        assert_eq!(
            std::fs::read(dir.path().join(FICHIER_ENTREES)).unwrap(),
            avant
        );

        // Journal au-delà de la borne : copie bornée, refus (aucune lecture au-delà).
        let mut tampon = Vec::new();
        let petit = dir.path().join("petit");
        std::fs::write(&petit, [7u8; 10]).unwrap();
        assert_eq!(lire_borne(&petit, 10, &mut tampon).unwrap(), 10);
        tampon.clear();
        assert!(matches!(
            lire_borne(&petit, 9, &mut tampon),
            Err(ErreurReleve::Depot(_))
        ));
        assert_eq!(tampon.len(), 10, "au plus max + 1 octets lus");

        // Dépôt vidé : relevé refusé, la liste se périme ensuite.
        std::fs::remove_file(dir.path().join(FICHIER_TETE)).unwrap();
        assert!(matches!(
            source.relever_depot(T0 + 60).await,
            Err(ErreurReleve::Depot(_))
        ));
        assert!(matches!(
            source.est_revoque(&cog(), &CibleRevocation::Cle([1; 32]), T0 + 16 * 60),
            Err(ErreurRevocation::Perimee { .. })
        ));
    }
}
