//! @id     miyufederation_persistance
//! @do     persister_etat_federal_local
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « persistance » (état fédéral local MFED : dossier 0700, fichier 0600, écriture atomique, lien symbolique refusé, HMAC du secret local).

//! Persistance de l'état fédéral local (plan §8 « Reste » : persistance des
//! statuts ; non spécifié par la doc gelée, **à consigner par amendement**).
//!
//! Même patron que le canal officiel (A-006, A-007 : `etat.mcce`,
//! `quarantaine.mcqt`) :
//!
//! | Élément | Règle |
//! |---|---|
//! | dossier | fourni par l'appelant (ex. `<données>/federation/`), créé en `0700` (`DirBuilder::mode`, aucune fenêtre en droits ouverts) ; lien symbolique ou non-dossier ⇒ refus ; propriétaire ≠ uid effectif ⇒ refus |
//! | `verrou` | fichier `0600` du dossier, verrou `flock` **exclusif** tenu tant que le [`StockageFederal`] vit : un seul processus à la fois (deuxième ouverture ⇒ [`ErreurEtat::Verrouille`]) |
//! | `etat.mfed` | état scellé, format **`MFED`** (table des champs : [`crate::federation`], sous-module `etat`) ; fichier `0600` |
//! | écriture | fichier temporaire `.etat.mfed.tmp-<pid>` créé exclusivement en `0600`, `fsync`, renommage atomique, `fsync` du dossier : jamais d'état partiel |
//! | lecture | lien symbolique ou fichier non régulier ⇒ refus ; droits ouverts à d'autres que le propriétaire ⇒ refus ; propriétaire ≠ uid effectif ⇒ refus ; fichier ouvert = fichier inspecté (même inode) ; taille exacte, ≤ 64 Mio |
//! | ancre | l'état scelle la tête (taille, racine) du **registre local** à l'écriture ; au chargement elle doit être un préfixe du registre fourni par l'appelant (taille ≤, preuve de cohérence) |
//! | intégrité | HMAC-SHA256 sur tout le fichier ; clé = `HMAC-SHA256(secret local du COG, "miyukini/miyufederation/etat/v1")` (secret local de la zone COG, jamais exporté : celui du canal officiel, `secret-local`, convient) |
//!
//! # Fail-closed
//!
//! - Fichier **absent** : si l'appelant attendait un état
//!   (`etat_attendu = true`, cas normal après le premier démarrage) ⇒
//!   [`ErreurEtat::EtatAbsent`] (suppression de l'état = anomalie, à signaler) ;
//!   sinon premier usage : vue vide (fondateurs à réinscrire depuis l'image),
//!   signalé par [`Reprise::premier_usage`] — que l'appelant **doit** traiter
//!   comme anormal s'il avait déjà enregistré un état.
//! - Fichier présent mais illisible, altéré (un octet), tronqué, d'une
//!   version inconnue (dont `0x01`, sans ancre), lien symbolique, droits
//!   ouverts, propriétaire étranger, titre signé refusé à la revérification,
//!   incohérent, heure en recul, ancre incohérente avec le registre local ⇒
//!   [`ErreurEtat`] : **aucune** vue n'est rendue, la fédération reste
//!   désactivée ; à signaler (WorrySentinel, gravité haute) par l'appelant.
//!   Aucun statut « membre » n'est jamais obtenu d'un état refusé.
//! - **Rejeu d'un état authentique plus ancien** (M3) : l'ancre de l'état
//!   doit être un préfixe du registre local fourni ([`RegistreLocal`]) ; les
//!   révocations fédérales inscrites au registre **après** l'ancre sont
//!   ré-appliquées (un état ancien ne peut pas faire oublier une
//!   révocation). Un état ancré **au-delà** du registre (registre tronqué ou
//!   remplacé) ⇒ [`ErreurEtat::RegistreIncoherent`].
//! - **Révocations à publier** (M2) : toute révocation définitive (échéance
//!   des 72 h, en fonctionnement ou au rattrapage d'un arrêt) entre dans une
//!   file **persistée** dans l'état ; [`StockageFederal::charger`] rend
//!   toujours la file non acquittée ([`Reprise::revocations`]). Elle n'est
//!   purgée que par [`StockageFederal::acquitter`], après inscription au
//!   registre (entrée vérifiée). Un plantage entre l'inscription et l'accusé
//!   rend de nouveau la révocation : la réinscrire est sans effet (même
//!   cible), l'appelant peut aussi l'acquitter en désignant l'entrée.
//!
//! Limite : un accès en écriture au disque peut encore **supprimer** l'état
//! **et** le registre local ensemble, ou les remplacer par une paire
//! cohérente plus ancienne ; il ne peut ni forger ni modifier un état sans le
//! secret local. Les entrées du registre local ne sont contrôlées que
//! structurellement (leur ajout frauduleux ne peut que **révoquer** : sens
//! fail-closed).

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroize;

use miyuregistre::{controler_structure_entree, Cible, Corps, Journal, Stockage};
use miyusceau::Kid;

use crate::erreurs::ErreurFederation;
use crate::federation::{Federation, Parametres, RevocationFederale, MOTIF_REVOCATION_FEDERALE};

type HmacSha256 = Hmac<Sha256>;

/// Nom du fichier d'état dans le dossier de la fédération.
pub const FICHIER_ETAT: &str = "etat.mfed";
/// Nom du fichier de verrou (`flock` exclusif) dans le dossier de la fédération.
pub const FICHIER_VERROU: &str = "verrou";
const ETIQUETTE: &[u8] = b"miyukini/miyufederation/etat/v1";
/// Taille maximale du fichier d'état (entête et MAC compris).
pub const TAILLE_MAX_FICHIER: u64 = (crate::federation::TAILLE_MAX_ETAT + 9 + 32) as u64;

/// Refus de l'état fédéral persistant (tous fail-closed : fédération désactivée).
#[derive(Debug, Clone, thiserror::Error)]
pub enum ErreurEtat {
    /// Entrée-sortie (lecture, écriture, droits, renommage).
    #[error("état fédéral : {0}")]
    Fichier(String),
    /// Le chemin est un lien symbolique ou n'est pas un fichier (dossier) régulier.
    #[error("état fédéral : lien symbolique ou fichier non régulier refusé")]
    LienSymbolique,
    /// Fichier lisible ou modifiable par d'autres que le propriétaire.
    #[error("état fédéral : droits trop ouverts (0600 exigé)")]
    DroitsOuverts,
    /// État au-delà des bornes du format.
    #[error("état fédéral trop volumineux")]
    TropGrand,
    /// Magie, taille ou structure invalide (troncature comprise).
    #[error("état fédéral illisible : {0}")]
    Illisible(&'static str),
    /// Version du format inconnue.
    #[error("état fédéral d'une version inconnue ({0:#04x})")]
    VersionInconnue(u8),
    /// HMAC faux : état altéré ou scellé par un autre secret local.
    #[error("état fédéral altéré (HMAC)")]
    Altere,
    /// Titre signé refusé à la revérification.
    #[error("état fédéral : titre refusé ({0})")]
    Titre(ErreurFederation),
    /// Contenu authentique mais incohérent.
    #[error("état fédéral incohérent : {0}")]
    Incoherent(&'static str),
    /// Heure de reprise antérieure à l'heure de l'état.
    #[error("état fédéral : heure en recul")]
    RetourHorloge,
    /// Fichier d'état absent alors qu'un état était attendu (suppression).
    #[error("état fédéral absent alors qu'il était attendu")]
    EtatAbsent,
    /// Ancre de l'état incohérente avec le registre local (état d'un autre
    /// registre, registre tronqué ou remplacé).
    #[error("état fédéral : ancre incohérente avec le registre local")]
    RegistreIncoherent,
    /// Dossier ou fichier appartenant à un autre utilisateur que l'uid effectif.
    #[error("état fédéral : propriétaire différent de l'utilisateur effectif")]
    Proprietaire,
    /// Dossier déjà verrouillé par un autre processus (ou une autre ouverture).
    #[error("état fédéral : dossier verrouillé par un autre processus")]
    Verrouille,
}

/// Tête du registre local scellée dans l'état (`taille` entrées, racine Merkle).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AncreRegistre {
    /// Nombre d'entrées du registre local.
    pub taille: u64,
    /// Racine Merkle des `taille` premières entrées.
    pub racine: [u8; 32],
}

impl AncreRegistre {
    /// Tête courante d'un registre local.
    pub fn de(registre: &dyn RegistreLocal) -> Self {
        Self { taille: registre.taille(), racine: registre.racine() }
    }
}

/// Registre local (journal Merkle append-only du COG) tel que la
/// persistance fédérale le consulte. Implémenté pour [`miyuregistre::Journal`].
pub trait RegistreLocal {
    /// Nombre d'entrées.
    fn taille(&self) -> u64;
    /// Racine Merkle courante.
    fn racine(&self) -> [u8; 32];
    /// Vrai si `ancre` est un **préfixe** du registre courant (taille ≤,
    /// racine cohérente : histoire non réécrite).
    fn prolonge(&self, ancre: &AncreRegistre) -> bool;
    /// Octets (sceau) de l'entrée `index` ; `None` si illisible ou hors registre.
    fn entree(&self, index: u64) -> Option<Vec<u8>>;
}

impl<S: Stockage> RegistreLocal for Journal<S> {
    fn taille(&self) -> u64 {
        Journal::taille(self)
    }

    fn racine(&self) -> [u8; 32] {
        Journal::racine(self)
    }

    fn prolonge(&self, ancre: &AncreRegistre) -> bool {
        let taille = Journal::taille(self);
        if ancre.taille > taille {
            return false;
        }
        if ancre.taille == 0 {
            return ancre.racine == miyusceau::racine_vide();
        }
        if ancre.taille == taille {
            return ancre.racine == Journal::racine(self);
        }
        self.preuve_coherence(ancre.taille, taille)
            .is_ok_and(|p| p.verifier(&ancre.racine, &Journal::racine(self)).is_ok())
    }

    fn entree(&self, index: u64) -> Option<Vec<u8>> {
        Journal::entree(self, index).ok()
    }
}

/// `(kid, heure d'inscription)` si les octets sont une entrée de révocation
/// fédérale (`Revocation { cible: Cle(kid) }`, motif fédéral).
fn revocation_federale(octets: &[u8]) -> Option<(Kid, u64)> {
    let e = controler_structure_entree(octets).ok()?;
    match e.corps {
        Corps::Revocation { cible: Cible::Cle(kid), motif } if motif.starts_with(MOTIF_REVOCATION_FEDERALE) => {
            Some((kid, e.inscrite_a))
        }
        _ => None,
    }
}

/// Clé de scellement de l'état fédéral, dérivée du secret local du COG ;
/// effacée à la libération.
pub struct CleEtatFederal {
    cle: [u8; 32],
}

impl CleEtatFederal {
    /// Dérive la clé : `HMAC-SHA256(secret_local, "miyukini/miyufederation/etat/v1")`.
    pub fn deriver(secret_local: &[u8; 32]) -> Self {
        // Une clé HMAC de 32 octets est toujours acceptée.
        let mut mac = <HmacSha256 as Mac>::new_from_slice(secret_local).unwrap_or_else(|_| unreachable!());
        mac.update(ETIQUETTE);
        Self { cle: mac.finalize().into_bytes().into() }
    }

    pub(crate) fn mac(&self) -> HmacSha256 {
        <HmacSha256 as Mac>::new_from_slice(&self.cle).unwrap_or_else(|_| unreachable!())
    }
}

impl Drop for CleEtatFederal {
    fn drop(&mut self) {
        self.cle.zeroize();
    }
}

impl std::fmt::Debug for CleEtatFederal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CleEtatFederal(..)")
    }
}

/// Vue reprise au démarrage.
#[derive(Debug)]
pub struct Reprise {
    /// Vue fédérale locale (revérifiée, rattrapée jusqu'à l'heure de reprise,
    /// révocations inscrites au registre après l'ancre ré-appliquées).
    pub federation: Federation,
    /// **Toutes** les révocations à publier non acquittées (produites pendant
    /// l'arrêt ou avant) : à inscrire au registre puis à acquitter
    /// ([`StockageFederal::acquitter`]).
    pub revocations: Vec<RevocationFederale>,
    /// Révocations déjà inscrites au registre après l'ancre de l'état et
    /// ré-appliquées (état plus ancien que le registre : à signaler).
    pub reappliquees: usize,
    /// Aucun état persistant : vue vide. Anormal si un état avait déjà été
    /// enregistré (l'appelant passe alors `etat_attendu = true`).
    pub premier_usage: bool,
}

/// Stockage de l'état fédéral local dans un dossier privé, verrouillé
/// (`flock` exclusif) tant que la valeur vit.
#[derive(Debug)]
pub struct StockageFederal {
    dossier: PathBuf,
    cle: CleEtatFederal,
    /// Verrou exclusif tenu jusqu'à la libération (fermeture du descripteur).
    #[cfg(unix)]
    _verrou: std::fs::File,
}

fn io(contexte: &str, e: &std::io::Error) -> ErreurEtat {
    ErreurEtat::Fichier(format!("{contexte} : {e}"))
}

#[cfg(unix)]
fn mode(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    meta.permissions().mode()
}

/// Propriétaire = uid effectif du processus.
#[cfg(unix)]
fn controler_proprietaire(meta: &std::fs::Metadata) -> Result<(), ErreurEtat> {
    use std::os::unix::fs::MetadataExt;
    if meta.uid() == rustix::process::geteuid().as_raw() {
        Ok(())
    } else {
        Err(ErreurEtat::Proprietaire)
    }
}

/// Ouvre (crée en `0600`) le fichier de verrou sans suivre de lien, contrôle
/// qu'il est ordinaire et à nous, puis prend un `flock` exclusif non bloquant.
#[cfg(unix)]
fn verrouiller(dossier: &Path) -> Result<std::fs::File, ErreurEtat> {
    use rustix::fs::{flock, FlockOperation, Mode, OFlags};
    use rustix::io::Errno;

    let fd = rustix::fs::open(
        dossier.join(FICHIER_VERROU),
        OFlags::RDWR | OFlags::CREATE | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NOCTTY,
        Mode::from_raw_mode(0o600),
    )
    .map_err(|e| if e == Errno::LOOP { ErreurEtat::LienSymbolique } else { io("verrou", &e.into()) })?;
    let fichier = std::fs::File::from(fd);
    let meta = fichier.metadata().map_err(|e| io("verrou", &e))?;
    if !meta.file_type().is_file() {
        return Err(ErreurEtat::LienSymbolique);
    }
    controler_proprietaire(&meta)?;
    if mode(&meta) & 0o077 != 0 {
        return Err(ErreurEtat::DroitsOuverts);
    }
    match flock(&fichier, FlockOperation::NonBlockingLockExclusive) {
        Ok(()) => Ok(fichier),
        Err(e) if e == Errno::WOULDBLOCK || e == Errno::AGAIN => Err(ErreurEtat::Verrouille),
        Err(e) => Err(io("verrou", &e.into())),
    }
}

impl StockageFederal {
    /// Ouvre (crée en `0700` au besoin, sans fenêtre en droits ouverts) le
    /// dossier de l'état fédéral et prend son verrou exclusif.
    /// Lien symbolique ou non-dossier ⇒ refus ; propriétaire ≠ uid effectif ⇒
    /// refus ; droits ramenés à `0700` ; déjà verrouillé ⇒
    /// [`ErreurEtat::Verrouille`].
    pub fn ouvrir(dossier: &Path, secret_local: &[u8; 32]) -> Result<Self, ErreurEtat> {
        match std::fs::symlink_metadata(dossier) {
            Ok(meta) if !meta.file_type().is_dir() => return Err(ErreurEtat::LienSymbolique),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let mut createur = std::fs::DirBuilder::new();
                createur.recursive(true);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::DirBuilderExt;
                    createur.mode(0o700);
                }
                createur.create(dossier).map_err(|e| io("création du dossier", &e))?;
            }
            Err(e) => return Err(io("dossier", &e)),
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let meta = std::fs::symlink_metadata(dossier).map_err(|e| io("dossier", &e))?;
            if !meta.file_type().is_dir() {
                return Err(ErreurEtat::LienSymbolique);
            }
            controler_proprietaire(&meta)?;
            if mode(&meta) & 0o077 != 0 {
                std::fs::set_permissions(dossier, std::fs::Permissions::from_mode(0o700))
                    .map_err(|e| io("droits du dossier", &e))?;
            }
        }
        Ok(Self {
            dossier: dossier.to_path_buf(),
            cle: CleEtatFederal::deriver(secret_local),
            #[cfg(unix)]
            _verrou: verrouiller(dossier)?,
        })
    }

    /// Chemin du fichier d'état.
    pub fn chemin(&self) -> PathBuf {
        self.dossier.join(FICHIER_ETAT)
    }

    /// Lit les octets de l'état (absent ⇒ `None`), voir module.
    pub fn lire(&self) -> Result<Option<Vec<u8>>, ErreurEtat> {
        let chemin = self.chemin();
        let meta = match std::fs::symlink_metadata(&chemin) {
            Ok(m) => m,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(io("lecture", &e)),
        };
        if !meta.file_type().is_file() {
            return Err(ErreurEtat::LienSymbolique);
        }
        #[cfg(unix)]
        if mode(&meta) & 0o077 != 0 {
            return Err(ErreurEtat::DroitsOuverts);
        }
        #[cfg(unix)]
        controler_proprietaire(&meta)?;
        if meta.len() > TAILLE_MAX_FICHIER {
            return Err(ErreurEtat::TropGrand);
        }
        let fichier = std::fs::File::open(&chemin).map_err(|e| io("lecture", &e))?;
        // Le fichier ouvert doit être celui inspecté (lien substitué entre-temps).
        let ouvert = fichier.metadata().map_err(|e| io("lecture", &e))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if ouvert.dev() != meta.dev() || ouvert.ino() != meta.ino() {
                return Err(ErreurEtat::LienSymbolique);
            }
        }
        if !ouvert.is_file() {
            return Err(ErreurEtat::LienSymbolique);
        }
        let mut octets = Vec::with_capacity(meta.len() as usize);
        fichier.take(TAILLE_MAX_FICHIER + 1).read_to_end(&mut octets).map_err(|e| io("lecture", &e))?;
        if octets.len() as u64 != meta.len() {
            return Err(ErreurEtat::Illisible("taille modifiée pendant la lecture"));
        }
        Ok(Some(octets))
    }

    /// Écrit atomiquement des octets d'état (fichier `0600`).
    pub fn ecrire(&self, octets: &[u8]) -> Result<(), ErreurEtat> {
        let chemin = self.chemin();
        match std::fs::symlink_metadata(&chemin) {
            Ok(m) if !m.file_type().is_file() => return Err(ErreurEtat::LienSymbolique),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(io("écriture", &e)),
        }
        let tmp = self.dossier.join(format!(".{FICHIER_ETAT}.tmp-{}", std::process::id()));
        let _ = std::fs::remove_file(&tmp);
        let resultat = (|| {
            let mut options = std::fs::OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut f = options.open(&tmp)?;
            f.write_all(octets)?;
            f.sync_all()?;
            drop(f);
            std::fs::rename(&tmp, &chemin)?;
            #[cfg(unix)]
            std::fs::File::open(&self.dossier)?.sync_all()?;
            Ok::<(), std::io::Error>(())
        })();
        if let Err(e) = resultat {
            let _ = std::fs::remove_file(&tmp);
            return Err(io("écriture", &e));
        }
        Ok(())
    }

    /// Scelle et écrit la vue (après chaque opération qui la modifie),
    /// ancrée sur la tête courante du registre local.
    pub fn enregistrer(&self, federation: &Federation, registre: &dyn RegistreLocal) -> Result<(), ErreurEtat> {
        self.ecrire(&federation.sceller_etat(&self.cle, &AncreRegistre::de(registre))?)
    }

    /// Reprend la vue au démarrage (voir module).
    ///
    /// - absent : `etat_attendu` ⇒ [`ErreurEtat::EtatAbsent`] ; sinon premier usage ;
    /// - ancre non préfixe de `registre` ⇒ [`ErreurEtat::RegistreIncoherent`] ;
    /// - révocations fédérales inscrites au registre après l'ancre ⇒ ré-appliquées ;
    /// - tout autre refus ⇒ `Err` (fédération désactivée).
    ///
    /// Si la vue a changé (rattrapage, ré-application) ou si le registre a
    /// avancé, l'état est réécrit (nouvelle ancre) avant de rendre la main ;
    /// échec ⇒ `Err`. [`Reprise::revocations`] contient **toute** la file
    /// non acquittée.
    pub fn charger(
        &self,
        parametres: Parametres,
        origin: Option<[u8; 32]>,
        maintenant: u64,
        registre: &dyn RegistreLocal,
        etat_attendu: bool,
    ) -> Result<Reprise, ErreurEtat> {
        let Some(octets) = self.lire()? else {
            if etat_attendu {
                return Err(ErreurEtat::EtatAbsent);
            }
            return Ok(Reprise {
                federation: Federation::new(parametres, origin),
                revocations: Vec::new(),
                reappliquees: 0,
                premier_usage: true,
            });
        };
        let (mut federation, definitives, ancre) =
            Federation::reprendre(&octets, &self.cle, parametres, origin, maintenant)?;
        if !registre.prolonge(&ancre) {
            return Err(ErreurEtat::RegistreIncoherent);
        }
        let mut reappliquees = 0;
        for index in ancre.taille..registre.taille() {
            let entree = registre.entree(index).ok_or(ErreurEtat::RegistreIncoherent)?;
            if let Some((kid, inscrite_a)) = revocation_federale(&entree) {
                if federation.appliquer_revocation_inscrite(kid, inscrite_a) {
                    reappliquees += 1;
                }
            }
        }
        if !definitives.is_empty() || reappliquees > 0 || AncreRegistre::de(registre) != ancre {
            self.enregistrer(&federation, registre)?;
        }
        let revocations = federation.revocations_a_publier();
        Ok(Reprise { federation, revocations, reappliquees, premier_usage: false })
    }

    /// Accusé d'inscription : l'entrée `index` du registre local doit être la
    /// révocation fédérale de `kid` ; elle est alors retirée de la file à
    /// publier et l'état est réécrit (ancré sur le registre). Entrée absente
    /// ou différente ⇒ refus, rien n'est acquitté.
    pub fn acquitter(
        &self,
        federation: &mut Federation,
        kid: &Kid,
        index: u64,
        registre: &dyn RegistreLocal,
    ) -> Result<(), ErreurEtat> {
        let inscrite = registre.entree(index).and_then(|e| revocation_federale(&e));
        if inscrite.is_none_or(|(k, _)| k != *kid) {
            return Err(ErreurEtat::Incoherent("révocation absente du registre à l'index donné"));
        }
        if federation.acquitter(kid) {
            self.enregistrer(federation, registre)?;
        }
        Ok(())
    }
}
