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
//! | dossier | fourni par l'appelant (ex. `<données>/federation/`), créé en `0700` ; lien symbolique ou non-dossier ⇒ refus |
//! | `etat.mfed` | état scellé, format **`MFED`** (table des champs : [`crate::federation`], sous-module `etat`) ; fichier `0600` |
//! | écriture | fichier temporaire `.etat.mfed.tmp-<pid>` créé exclusivement en `0600`, `fsync`, renommage atomique, `fsync` du dossier : jamais d'état partiel |
//! | lecture | lien symbolique ou fichier non régulier ⇒ refus ; droits ouverts à d'autres que le propriétaire ⇒ refus ; fichier ouvert = fichier inspecté (même inode) ; taille exacte, ≤ 64 Mio |
//! | intégrité | HMAC-SHA256 sur tout le fichier ; clé = `HMAC-SHA256(secret local du COG, "miyukini/miyufederation/etat/v1")` (secret local de la zone COG, jamais exporté : celui du canal officiel, `secret-local`, convient) |
//!
//! # Fail-closed
//!
//! - Fichier **absent** ⇒ premier usage : vue vide (fondateurs à réinscrire
//!   depuis l'image), signalé par [`Reprise::premier_usage`].
//! - Fichier présent mais illisible, altéré (un octet), tronqué, d'une
//!   version inconnue, lien symbolique, droits ouverts, titre signé refusé à
//!   la revérification, incohérent, heure en recul ⇒ [`ErreurEtat`] :
//!   **aucune** vue n'est rendue, la fédération reste désactivée ; à
//!   signaler (WorrySentinel, gravité haute) par l'appelant. Aucun statut
//!   « membre » n'est jamais obtenu d'un état refusé.
//! - Suspensions échues pendant l'arrêt : révocations définitives datées de
//!   leur échéance, renvoyées **une seule fois** — l'état est réécrit avant
//!   de rendre la main (si la réécriture échoue : refus, rien n'est rendu).
//!
//! Limite : un accès en écriture au disque peut **supprimer** l'état
//! (retour au premier usage : statuts, suspensions et révocations locales
//! perdus — les révocations restent inscrites au registre) ou rejouer un
//! état authentique plus ancien du même COG. Il ne peut ni forger ni
//! modifier un état sans le secret local.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::Zeroize;

use crate::erreurs::ErreurFederation;
use crate::federation::{Federation, Parametres, RevocationFederale};

type HmacSha256 = Hmac<Sha256>;

/// Nom du fichier d'état dans le dossier de la fédération.
pub const FICHIER_ETAT: &str = "etat.mfed";
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
    /// Vue fédérale locale (revérifiée, rattrapée jusqu'à l'heure de reprise).
    pub federation: Federation,
    /// Révocations définitives produites pendant l'arrêt (à inscrire au registre).
    pub revocations: Vec<RevocationFederale>,
    /// Aucun état persistant : vue vide (à signaler si un état était attendu).
    pub premier_usage: bool,
}

/// Stockage de l'état fédéral local dans un dossier privé.
#[derive(Debug)]
pub struct StockageFederal {
    dossier: PathBuf,
    cle: CleEtatFederal,
}

fn io(contexte: &str, e: &std::io::Error) -> ErreurEtat {
    ErreurEtat::Fichier(format!("{contexte} : {e}"))
}

#[cfg(unix)]
fn mode(meta: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    meta.permissions().mode()
}

impl StockageFederal {
    /// Ouvre (crée en `0700` au besoin) le dossier de l'état fédéral.
    /// Lien symbolique ou non-dossier ⇒ refus ; droits ramenés à `0700`.
    pub fn ouvrir(dossier: &Path, secret_local: &[u8; 32]) -> Result<Self, ErreurEtat> {
        match std::fs::symlink_metadata(dossier) {
            Ok(meta) if !meta.file_type().is_dir() => return Err(ErreurEtat::LienSymbolique),
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir_all(dossier).map_err(|e| io("création du dossier", &e))?;
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
            if mode(&meta) & 0o077 != 0 {
                std::fs::set_permissions(dossier, std::fs::Permissions::from_mode(0o700))
                    .map_err(|e| io("droits du dossier", &e))?;
            }
        }
        Ok(Self { dossier: dossier.to_path_buf(), cle: CleEtatFederal::deriver(secret_local) })
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

    /// Scelle et écrit la vue (après chaque opération qui la modifie).
    pub fn enregistrer(&self, federation: &Federation) -> Result<(), ErreurEtat> {
        self.ecrire(&federation.sceller_etat(&self.cle)?)
    }

    /// Reprend la vue au démarrage (voir module) : absent ⇒ premier usage ;
    /// refus ⇒ `Err` (fédération désactivée). Si des révocations
    /// définitives ont été produites pendant l'arrêt, l'état est réécrit
    /// avant de les rendre (elles ne seront plus jamais renvoyées).
    pub fn charger(
        &self,
        parametres: Parametres,
        origin: Option<[u8; 32]>,
        maintenant: u64,
    ) -> Result<Reprise, ErreurEtat> {
        let Some(octets) = self.lire()? else {
            return Ok(Reprise { federation: Federation::new(parametres, origin), revocations: Vec::new(), premier_usage: true });
        };
        let (federation, revocations) = Federation::reprendre(&octets, &self.cle, parametres, origin, maintenant)?;
        if !revocations.is_empty() {
            self.enregistrer(&federation)?;
        }
        Ok(Reprise { federation, revocations, premier_usage: false })
    }
}
