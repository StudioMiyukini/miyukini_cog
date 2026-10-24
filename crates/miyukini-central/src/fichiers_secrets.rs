//! @id     miyukini_central_fichiers_secrets
//! @do     ecrire_fichier_secret
//! @role   securite
//! @layer  core
//! @human  miyukini-central — écriture des secrets en 0600 (dossiers 0700) et durcissement de l'existant.

//! Fichiers secrets de Central (CA-03, C-25).
//!
//! Sous Unix : fichier créé en `0600` dans un fichier temporaire du même
//! dossier puis renommé atomiquement ; dossier parent créé en `0700`. Les
//! fichiers déjà présents sont ramenés à `0600` au démarrage. Sous Windows,
//! le profil utilisateur (`%LOCALAPPDATA%`) porte déjà une ACL propre à
//! l'utilisateur : écriture atomique sans changement d'ACL.
//!
//! Aucune fonction de ce module ne journalise le contenu écrit.

use std::io::Write;
use std::path::Path;

/// Crée un dossier (et ses parents) ; le dernier niveau est mis en `0700` sous Unix.
pub fn creer_dossier_prive(dossier: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dossier)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dossier, std::fs::Permissions::from_mode(0o700))?;
    }
    Ok(())
}

/// Écrit un secret : dossier parent `0700`, fichier `0600`, remplacement atomique.
pub fn ecrire_secret(chemin: &Path, contenu: &[u8]) -> std::io::Result<()> {
    let parent = chemin
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.exists() {
        creer_dossier_prive(parent)?;
    }
    let nom = chemin
        .file_name()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "nom de fichier absent"))?;
    let mut nom_tmp = std::ffi::OsString::from(".");
    nom_tmp.push(nom);
    nom_tmp.push(format!(".tmp-{}", std::process::id()));
    let tmp = parent.join(nom_tmp);
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
        f.write_all(contenu)?;
        f.sync_all()?;
        drop(f);
        std::fs::rename(&tmp, chemin)
    })();
    if resultat.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    resultat
}

/// Ramène un fichier existant à `0600` (Unix). Absent = rien à faire.
pub fn durcir_fichier(chemin: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match std::fs::metadata(chemin) {
            Ok(meta) if meta.is_file() => {
                if meta.permissions().mode() & 0o077 != 0 {
                    std::fs::set_permissions(chemin, std::fs::Permissions::from_mode(0o600))?;
                }
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    #[cfg(not(unix))]
    let _ = chemin;
    Ok(())
}

/// Crée un fichier vide en `0600` s'il n'existe pas (avant ouverture SQLite :
/// SQLite reprend ensuite ces permissions pour ses journaux `-wal`/`-journal`),
/// sinon le durcit.
pub fn preparer_fichier_prive(chemin: &Path) -> std::io::Result<()> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    match options.open(chemin) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => durcir_fichier(chemin),
        Err(e) => Err(e),
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn mode(p: &Path) -> u32 {
        std::fs::metadata(p).unwrap().permissions().mode() & 0o777
    }

    fn tmp(tag: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("miyu-secret-{tag}-{}-{}", std::process::id(), rand::random::<u64>()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn secret_cree_en_0600_dossier_0700() {
        let racine = tmp("ecrire");
        let dossier = racine.join("conf");
        let f = dossier.join("secret.json");
        ecrire_secret(&f, b"{\"jeton\":\"x\"}").unwrap();
        assert_eq!(mode(&f), 0o600);
        assert_eq!(mode(&dossier), 0o700);
        assert_eq!(std::fs::read(&f).unwrap(), b"{\"jeton\":\"x\"}");
        // Réécriture : toujours 0600, contenu remplacé, pas de fichier temporaire restant.
        ecrire_secret(&f, b"2").unwrap();
        assert_eq!(mode(&f), 0o600);
        assert_eq!(std::fs::read_dir(&dossier).unwrap().count(), 1);
        let _ = std::fs::remove_dir_all(racine);
    }

    #[test]
    fn existant_durci_en_0600() {
        let racine = tmp("durcir");
        let f = racine.join("ancien.json");
        std::fs::write(&f, b"x").unwrap();
        std::fs::set_permissions(&f, std::fs::Permissions::from_mode(0o644)).unwrap();
        durcir_fichier(&f).unwrap();
        assert_eq!(mode(&f), 0o600);
        durcir_fichier(&racine.join("absent")).unwrap();
        let _ = std::fs::remove_dir_all(racine);
    }

    #[test]
    fn fichier_prive_prepare_en_0600() {
        let racine = tmp("preparer");
        let f = racine.join("central.db");
        preparer_fichier_prive(&f).unwrap();
        assert_eq!(mode(&f), 0o600);
        std::fs::set_permissions(&f, std::fs::Permissions::from_mode(0o644)).unwrap();
        preparer_fichier_prive(&f).unwrap();
        assert_eq!(mode(&f), 0o600);
        let _ = std::fs::remove_dir_all(racine);
    }
}
