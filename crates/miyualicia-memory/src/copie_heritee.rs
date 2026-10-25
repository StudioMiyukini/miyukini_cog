//! Copie héritée de la mémoire d'Alicia : présence et effacement sûr (A-019,
//! constat de revue, **à consigner par amendement**).
//!
//! L'ancienne base SQLite reste lisible tant qu'elle existe : un fait oublié
//! dans KindMother (pierre tombale) y survivrait, et pourrait être ressuscité
//! par un nouvel import si le marqueur `<base>.importe-kindmother`
//! disparaissait. Règles :
//!
//! - après un import **vérifié** (contrôle ligne à ligne du moteur
//!   `kindmother-heritage`, empreinte SHA-256 inchangée), la copie héritée est
//!   **effacée** ([`AncienneBase::Effacer`], défaut) : chaque fichier
//!   (`<base>`, `<base>-wal`, `<base>-shm`, `<base>-journal`) est réécrit de
//!   zéros sur toute sa longueur, synchronisé (`fsync`), puis supprimé, et le
//!   dossier synchronisé ; le marqueur est conservé et complété (date,
//!   fichiers effacés) : il garde l'empreinte SHA-256 de l'ancien fichier pour
//!   la traçabilité. Lien symbolique ou fichier non régulier ⇒ refus (la cible
//!   n'est jamais suivie) ;
//! - conservation **explicite** seulement ([`AncienneBase::Conserver`],
//!   commande `alicia-memoire-import --conserver-ancienne`) : tant qu'une
//!   copie héritée existe, [`crate::MemoryStore::forget`] **refuse**
//!   ([`crate::MemoryError::LegacyCopyPresent`]) pour que l'oubli ne soit
//!   jamais trompeur ; au démarrage, une copie présente avec son marqueur est
//!   signalée (avertissement journalisé).
//!
//! Limite : la réécriture en place efface le contenu sur un système de
//! fichiers classique ; sur un support à copie sur écriture, journalisé en
//! données ou un SSD (blocs réalloués), d'anciens blocs peuvent subsister
//! physiquement — le chiffrement du disque reste la protection de fond.

// @id service.alicia.memory.copie_heritee
// @role memory_legacy_copy_erasure
// @layer 7
// @do erase_legacy_alicia_memory_copy
// @human Présence et effacement sûr de l'ancienne base SQLite de la mémoire d'Alicia après import.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Sort de l'ancienne base SQLite après un import vérifié.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AncienneBase {
    /// Réécrite puis supprimée (défaut) : l'oubli reste possible.
    #[default]
    Effacer,
    /// Conservée (choix explicite) : l'oubli est refusé tant qu'elle existe.
    Conserver,
}

/// Fichiers de la copie héritée : la base et ses compagnons SQLite.
pub fn fichiers_copie_heritee(base: &Path) -> [PathBuf; 4] {
    let compagnon = |suffixe: &str| {
        let mut nom = base.as_os_str().to_os_string();
        nom.push(suffixe);
        PathBuf::from(nom)
    };
    [base.to_path_buf(), compagnon("-wal"), compagnon("-shm"), compagnon("-journal")]
}

/// Vrai si un fichier de la copie héritée existe encore (non vide, ou lien
/// symbolique, ou autre que fichier ordinaire : fail-closed).
pub fn copie_heritee_presente(base: &Path) -> bool {
    fichiers_copie_heritee(base).iter().any(|f| match std::fs::symlink_metadata(f) {
        Ok(m) => !m.file_type().is_file() || m.len() > 0,
        Err(e) => e.kind() != io::ErrorKind::NotFound,
    })
}

/// Réécrit de zéros, synchronise puis supprime un fichier ordinaire (absent :
/// `false`). Lien symbolique ou fichier non régulier ⇒ refus.
fn effacer_fichier(chemin: &Path) -> io::Result<bool> {
    let meta = match std::fs::symlink_metadata(chemin) {
        Ok(m) => m,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(e) => return Err(e),
    };
    if !meta.file_type().is_file() {
        return Err(io::Error::other(format!(
            "{} : lien symbolique ou fichier non régulier, effacement refusé",
            chemin.display()
        )));
    }
    let mut f = std::fs::OpenOptions::new().write(true).open(chemin)?;
    // Le fichier ouvert doit être celui inspecté (lien substitué entre-temps).
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let ouvert = f.metadata()?;
        if ouvert.dev() != meta.dev() || ouvert.ino() != meta.ino() {
            return Err(io::Error::other(format!("{} : fichier substitué pendant l'effacement", chemin.display())));
        }
    }
    let zeros = vec![0_u8; 64 * 1024];
    let mut reste = meta.len();
    while reste > 0 {
        let n = usize::try_from(reste.min(zeros.len() as u64)).unwrap_or(zeros.len());
        f.write_all(&zeros[..n])?;
        reste -= n as u64;
    }
    f.sync_all()?;
    drop(f);
    std::fs::remove_file(chemin)?;
    Ok(true)
}

/// Efface la copie héritée (voir module) ; rend les fichiers effacés.
/// Une erreur laisse les fichiers restants en place (l'oubli reste refusé).
pub(crate) fn effacer_copie_heritee(base: &Path) -> io::Result<Vec<PathBuf>> {
    let mut effaces = Vec::new();
    // Compagnons d'abord : la base reste désignée tant que tout n'est pas effacé.
    let [base_f, wal, shm, journal] = fichiers_copie_heritee(base);
    for f in [wal, shm, journal, base_f] {
        if effacer_fichier(&f)? {
            effaces.push(f);
        }
    }
    #[cfg(unix)]
    if let Some(dossier) = base.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::File::open(dossier)?.sync_all()?;
    }
    Ok(effaces)
}

/// Empreinte SHA-256 consignée dans le marqueur d'import (`sha256: <hex>`).
pub(crate) fn empreinte_du_marqueur(marqueur: &Path) -> Option<String> {
    let contenu = std::fs::read_to_string(marqueur).ok()?;
    contenu
        .lines()
        .find_map(|l| l.strip_prefix("sha256:"))
        .map(|v| v.trim().to_string())
        .filter(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit()))
}

/// Complète le marqueur (écriture atomique : temporaire puis renommage) :
/// date et fichiers effacés ; l'empreinte d'origine reste.
pub(crate) fn consigner_effacement(marqueur: &Path, effaces: &[PathBuf]) -> io::Result<()> {
    let mut contenu = std::fs::read_to_string(marqueur)?;
    if !contenu.is_empty() && !contenu.ends_with('\n') {
        contenu.push('\n');
    }
    {
        use std::fmt::Write as _;
        writeln!(
            contenu,
            "ancienne_base: effacee le {} (contenu réécrit de zéros puis fichier supprimé)",
            chrono::Utc::now().to_rfc3339()
        )
        .map_err(io::Error::other)?;
        for f in effaces {
            writeln!(contenu, "efface: {}", f.display()).map_err(io::Error::other)?;
        }
    }
    let mut tmp = marqueur.as_os_str().to_os_string();
    tmp.push(format!(".tmp-{}", std::process::id()));
    let tmp = PathBuf::from(tmp);
    let ecrit = (|| {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(contenu.as_bytes())?;
        f.sync_all()?;
        std::fs::rename(&tmp, marqueur)
    })();
    if ecrit.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    ecrit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effacement_reecrit_puis_supprime_et_refuse_les_liens() {
        let d = tempfile::tempdir().unwrap();
        let base = d.path().join("ancienne.db");
        std::fs::write(&base, b"SECRET-HERITE").unwrap();
        std::fs::write(d.path().join("ancienne.db-wal"), b"WAL-SECRET").unwrap();
        assert!(copie_heritee_presente(&base));
        let effaces = effacer_copie_heritee(&base).unwrap();
        assert_eq!(effaces.len(), 2);
        assert!(!copie_heritee_presente(&base));
        assert!(std::fs::read_dir(d.path()).unwrap().next().is_none());

        #[cfg(unix)]
        {
            let cible = d.path().join("cible");
            std::fs::write(&cible, b"CIBLE").unwrap();
            std::os::unix::fs::symlink(&cible, &base).unwrap();
            assert!(copie_heritee_presente(&base));
            assert!(effacer_copie_heritee(&base).is_err());
            assert_eq!(std::fs::read(&cible).unwrap(), b"CIBLE", "cible du lien jamais touchée");
        }
    }

    #[test]
    fn marqueur_empreinte_et_consignation() {
        let d = tempfile::tempdir().unwrap();
        let m = d.path().join("x.importe-kindmother");
        let sha = "ab".repeat(32);
        std::fs::write(&m, format!("# marqueur\nsha256: {sha}\n")).unwrap();
        assert_eq!(empreinte_du_marqueur(&m).as_deref(), Some(sha.as_str()));
        consigner_effacement(&m, &[d.path().join("x")]).unwrap();
        let c = std::fs::read_to_string(&m).unwrap();
        assert!(c.contains(&format!("sha256: {sha}")) && c.contains("ancienne_base: effacee"));
        std::fs::write(&m, b"").unwrap();
        assert_eq!(empreinte_du_marqueur(&m), None);
    }
}
