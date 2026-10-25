//! @id     miyuregistre_editeur_depot
//! @do     deposer_registre_revocations_miyucloud
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — dépôt local du registre de révocation pour MiyuCloud (tete.sceau + entrees.mrgj) : écriture atomique relative au dossier, droits 0640/0750, aucun lien suivi, entrées puis tête.

//! Dépôt du registre de révocation lu par MiyuCloud
//! (`MIYUCLOUD_MIYU_REVOCATIONS_DEPOT`).
//!
//! # Contenu
//!
//! - [`FICHIER_ENTREES`] : toutes les entrées couvertes par la tête, au format
//!   de stockage `MRGJ` ([`crate::StockageFichier`]) ;
//! - [`FICHIER_TETE`] : le sceau de la tête signée.
//!
//! # Protocole (ordre des écritures)
//!
//! Chaque fichier est remplacé **atomiquement** : écrit dans un temporaire du
//! même dossier (`.<nom>.tmp`, `O_CREAT | O_EXCL | O_NOFOLLOW`, `0640`),
//! `fsync`, `renameat` sur le nom final, `fsync` du dossier. Un lecteur voit
//! donc toujours un fichier complet, ancien ou nouveau.
//!
//! L'éditeur écrit **les entrées d'abord, la tête ensuite** ; le lecteur
//! (MiyuCloud) lit **la tête d'abord, les entrées ensuite**. Le journal étant
//! append-only, les entrées publiées ne font que s'allonger. Si le lecteur lit
//! la tête de la publication `k`, les entrées de `k` étaient déjà en place ;
//! les entrées qu'il lit ensuite sont celles de `k` ou d'une publication
//! ultérieure, donc un **sur-ensemble** : elles couvrent toujours la tête lue
//! (les entrées au-delà de la tête sont ignorées par
//! [`crate::SuiviRevocationsRegistre::appliquer`]). L'ordre inverse (tête
//! d'abord) laisserait voir une tête neuve avec des entrées anciennes
//! (couverture insuffisante, relevé refusé).
//!
//! Les entrées ne sont réécrites que si elles ont changé depuis la dernière
//! publication de ce processus (même taille, même racine et fichier en place
//! de la longueur attendue ⇒ seule la tête est remplacée).
//!
//! # Sûreté du dossier
//!
//! - Le dossier est créé en `0750` s'il manque (parent existant) ; il est
//!   ouvert en `O_DIRECTORY | O_NOFOLLOW` (dernier composant : lien ⇒ refus)
//!   et toutes les opérations sont **relatives à ce descripteur** (`openat`,
//!   `renameat`, `unlinkat`) : remplacer le chemin après l'ouverture n'a aucun
//!   effet, un lien symbolique posé à la place d'un fichier est remplacé, pas
//!   suivi.
//! - Propriétaire = uid effectif ; aucun droit d'écriture pour le groupe, aucun
//!   droit pour les autres (`0750` ou plus strict) ; fichiers en `0640`
//!   (lecture par le groupe du service MiyuCloud).
//! - Verrou `flock` exclusif non bloquant sur le dossier, tenu tant que le
//!   dépôt est ouvert : un seul éditeur à la fois.
//! - Bornes de MiyuCloud respectées avant écriture : tête ≤
//!   [`TAILLE_MAX_TETE`], fichier d'entrées ≤ [`TAILLE_MAX_FICHIER_ENTREES`].

use std::fs::File;
use std::io::{BufWriter, Write};
use std::os::fd::OwnedFd;
use std::os::unix::fs::{DirBuilderExt, MetadataExt};
use std::path::{Path, PathBuf};

use miyusceau::Empreinte;
use rustix::fs::{AtFlags, FlockOperation, Mode, OFlags};
use rustix::io::Errno;

use super::Instantane;
use crate::stockage::{encoder_enregistrement, entete_fichier, TAILLE_MAX_ENREGISTREMENT};

/// Fichier de la tête signée dans le dépôt (nom lu par MiyuCloud).
pub const FICHIER_TETE: &str = "tete.sceau";
/// Fichier des entrées (format `MRGJ`) dans le dépôt (nom lu par MiyuCloud).
pub const FICHIER_ENTREES: &str = "entrees.mrgj";
/// Droits des fichiers déposés.
pub const MODE_FICHIERS: u32 = 0o640;
/// Droits du dossier créé.
pub const MODE_DOSSIER: u32 = 0o750;
/// Taille maximale du fichier de tête (borne de MiyuCloud).
pub const TAILLE_MAX_TETE: u64 = 1 << 20;
/// Taille maximale du fichier d'entrées (borne de MiyuCloud).
pub const TAILLE_MAX_FICHIER_ENTREES: u64 = 256 << 20;

/// Dépôt refusé ou en échec.
#[derive(Debug, thiserror::Error)]
pub enum ErreurDepot {
    /// Dossier ou contenu refusé.
    #[error("dépôt refusé : {0}")]
    Refuse(&'static str),
    /// Contenu au-delà des bornes du lecteur.
    #[error("dépôt : {0} au-delà de la borne du lecteur")]
    Bornes(&'static str),
    /// Enregistrement invalide (entrée vide ou trop grande).
    #[error("dépôt : {0}")]
    Stockage(#[from] crate::erreurs::ErreurStockage),
    /// Erreur d'entrée/sortie.
    #[error("dépôt : {0}")]
    EntreeSortie(#[from] std::io::Error),
}

fn es(e: Errno) -> ErreurDepot {
    ErreurDepot::EntreeSortie(e.into())
}

/// Dépôt ouvert (verrou exclusif tenu jusqu'à sa destruction).
#[derive(Debug)]
pub struct DepotRevocations {
    chemin: PathBuf,
    dossier: OwnedFd,
    /// Entrées publiées par ce processus : (taille, racine, longueur du fichier).
    publiees: Option<(u64, Empreinte, u64)>,
}

impl DepotRevocations {
    /// Ouvre (ou crée en `0750`) le dossier de dépôt `chemin` et prend son verrou.
    pub fn ouvrir(chemin: &Path) -> Result<Self, ErreurDepot> {
        match std::fs::DirBuilder::new().mode(MODE_DOSSIER).create(chemin) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e.into()),
        }
        let dossier = rustix::fs::open(chemin, OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC, Mode::empty())
            .map_err(|e| match e {
                Errno::LOOP | Errno::NOTDIR => ErreurDepot::Refuse("le dépôt doit être un dossier, pas un lien symbolique"),
                e => es(e),
            })?;
        let meta = rustix::fs::fstat(&dossier).map_err(es)?;
        if meta.st_uid != rustix::process::geteuid().as_raw() {
            return Err(ErreurDepot::Refuse("dossier : propriétaire différent de l'utilisateur effectif"));
        }
        if meta.st_mode & 0o027 != 0 {
            return Err(ErreurDepot::Refuse("dossier : droits trop larges (attendu 0750 ou plus strict)"));
        }
        rustix::fs::flock(&dossier, FlockOperation::NonBlockingLockExclusive).map_err(|e| {
            if e == Errno::WOULDBLOCK {
                ErreurDepot::Refuse("dépôt déjà tenu par un autre éditeur")
            } else {
                es(e)
            }
        })?;
        let depot = Self { chemin: chemin.to_path_buf(), dossier, publiees: None };
        // Temporaires d'une publication interrompue (verrou tenu : aucun autre écrivain).
        for nom in [FICHIER_ENTREES, FICHIER_TETE] {
            depot.retirer(&temporaire(nom))?;
        }
        Ok(depot)
    }

    /// Chemin du dossier.
    pub fn chemin(&self) -> &Path {
        &self.chemin
    }

    fn retirer(&self, nom: &str) -> Result<(), ErreurDepot> {
        match rustix::fs::unlinkat(&self.dossier, nom, AtFlags::empty()) {
            Ok(()) | Err(Errno::NOENT) => Ok(()),
            Err(e) => Err(es(e)),
        }
    }

    /// Remplace atomiquement `nom` par le contenu produit par `ecrire`.
    fn remplacer(
        &self,
        nom: &str,
        ecrire: impl FnOnce(&mut BufWriter<&File>) -> std::io::Result<()>,
    ) -> Result<u64, ErreurDepot> {
        let tmp = temporaire(nom);
        self.retirer(&tmp)?;
        let fd = rustix::fs::openat(
            &self.dossier,
            tmp.as_str(),
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::from_raw_mode(MODE_FICHIERS),
        )
        .map_err(es)?;
        let fichier = File::from(fd);
        let resultat = (|| -> Result<u64, ErreurDepot> {
            // Indépendant du umask.
            rustix::fs::fchmod(&fichier, Mode::from_raw_mode(MODE_FICHIERS)).map_err(es)?;
            let mut tampon = BufWriter::with_capacity(1 << 16, &fichier);
            ecrire(&mut tampon)?;
            tampon.flush()?;
            drop(tampon);
            fichier.sync_all()?;
            let longueur = fichier.metadata()?.len();
            rustix::fs::renameat(&self.dossier, tmp.as_str(), &self.dossier, nom).map_err(es)?;
            rustix::fs::fsync(&self.dossier).map_err(es)?;
            Ok(longueur)
        })();
        if resultat.is_err() {
            let _ = self.retirer(&tmp);
        }
        resultat
    }

    /// Vrai si `FICHIER_ENTREES` est un fichier ordinaire de `longueur` octets.
    fn entrees_en_place(&self, longueur: u64) -> bool {
        rustix::fs::statat(&self.dossier, FICHIER_ENTREES, AtFlags::SYMLINK_NOFOLLOW)
            .is_ok_and(|m| rustix::fs::FileType::from_raw_mode(m.st_mode) == rustix::fs::FileType::RegularFile
                && u64::try_from(m.st_size).ok() == Some(longueur))
    }

    /// Publie `instantane` : entrées (si changées) **puis** tête (voir le module).
    pub fn publier(&mut self, instantane: &Instantane) -> Result<(), ErreurDepot> {
        if instantane.tete.is_empty() {
            return Err(ErreurDepot::Refuse("tête vide"));
        }
        if instantane.tete.len() as u64 > TAILLE_MAX_TETE {
            return Err(ErreurDepot::Bornes("tête"));
        }
        let longueur = instantane
            .entrees
            .iter()
            .fold(entete_fichier().len() as u64, |t, e| t.saturating_add(12).saturating_add(e.len() as u64));
        if longueur > TAILLE_MAX_FICHIER_ENTREES {
            return Err(ErreurDepot::Bornes("fichier d'entrées"));
        }
        let inchangees = self.publiees == Some((instantane.taille, instantane.racine, longueur)) && self.entrees_en_place(longueur);
        if !inchangees {
            // Contrôle de toutes les entrées avant d'écrire quoi que ce soit.
            if instantane.entrees.iter().any(|e| e.is_empty() || e.len() > TAILLE_MAX_ENREGISTREMENT) {
                return Err(crate::erreurs::ErreurStockage::TailleInvalide.into());
            }
            self.publiees = None;
            let ecrite = self.remplacer(FICHIER_ENTREES, |f| {
                f.write_all(&entete_fichier())?;
                instantane.entrees.iter().try_for_each(|e| {
                    f.write_all(&encoder_enregistrement(e).map_err(std::io::Error::other)?)
                })
            })?;
            if ecrite != longueur {
                return Err(ErreurDepot::Refuse("longueur du fichier d'entrées inattendue"));
            }
            self.publiees = Some((instantane.taille, instantane.racine, longueur));
        }
        self.remplacer(FICHIER_TETE, |f| f.write_all(&instantane.tete))?;
        Ok(())
    }
}

fn temporaire(nom: &str) -> String {
    format!(".{nom}.tmp")
}
