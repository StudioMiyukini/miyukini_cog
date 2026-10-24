//! @id     miyukini_central_chemins
//! @do     resoudre_chemins_central
//! @role   securite
//! @layer  core
//! @human  miyukini-central — résolution des dossiers de données et de configuration (jamais le cwd en release).

//! Résolution des chemins de Central (CA-04, C-12, C-21).
//!
//! - **Release** : uniquement des chemins absolus issus du dossier de données
//!   utilisateur (`%LOCALAPPDATA%`, `$XDG_DATA_HOME`, `$HOME/.local/share`) ou du
//!   dossier de l'exécutable. Le répertoire courant n'est **jamais** utilisé :
//!   lancer Central depuis un dossier piégé ne doit ni charger sa base ni
//!   exécuter ses binaires.
//! - **Debug** : le répertoire courant reste utilisé pour la base auth et les
//!   données (confort de développement, comportement historique).
//!
//! Le résolveur pur [`resoudre_dossier_donnees`] reçoit l'environnement en
//! paramètre pour être testé sans toucher au processus.

use std::path::{Path, PathBuf};

/// Nom du dossier applicatif.
pub const DOSSIER_APP: &str = "Miyukini-COG";

/// Nom de la base auth Central.
pub const NOM_BASE_AUTH: &str = "central.db";

/// Erreur de résolution de chemin (fail-closed : pas de repli sur le cwd).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurChemin {
    /// Aucune variable d'environnement exploitable pour le dossier utilisateur.
    #[error("dossier de données utilisateur introuvable (LOCALAPPDATA, XDG_DATA_HOME ou HOME absolus requis)")]
    DossierUtilisateurIntrouvable,
    /// Répertoire courant illisible (debug uniquement).
    #[error("répertoire courant illisible")]
    RepertoireCourant,
}

/// Instantané de l'environnement utile à la résolution.
#[derive(Debug, Clone, Default)]
pub struct EnvChemins {
    /// `%LOCALAPPDATA%` (Windows).
    pub local_app_data: Option<PathBuf>,
    /// `$XDG_DATA_HOME`.
    pub xdg_data_home: Option<PathBuf>,
    /// `$XDG_CONFIG_HOME`.
    pub xdg_config_home: Option<PathBuf>,
    /// `$HOME`.
    pub home: Option<PathBuf>,
    /// Répertoire courant (utilisé en debug seulement).
    pub cwd: Option<PathBuf>,
}

impl EnvChemins {
    /// Lit l'environnement du processus.
    pub fn du_processus() -> Self {
        let var = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
        Self {
            local_app_data: var("LOCALAPPDATA"),
            xdg_data_home: var("XDG_DATA_HOME"),
            xdg_config_home: var("XDG_CONFIG_HOME"),
            home: var("HOME"),
            cwd: std::env::current_dir().ok(),
        }
    }
}

/// Ne garde qu'un chemin absolu (une variable relative serait relative au cwd).
fn absolu(p: Option<&PathBuf>) -> Option<&Path> {
    p.map(PathBuf::as_path).filter(|p| p.is_absolute())
}

/// Dossier utilisateur de données, jamais relatif.
fn dossier_utilisateur_donnees(env: &EnvChemins) -> Result<PathBuf, ErreurChemin> {
    if let Some(p) = absolu(env.local_app_data.as_ref()) {
        return Ok(p.join(DOSSIER_APP));
    }
    if let Some(p) = absolu(env.xdg_data_home.as_ref()) {
        return Ok(p.join(DOSSIER_APP));
    }
    if let Some(p) = absolu(env.home.as_ref()) {
        return Ok(p.join(".local").join("share").join(DOSSIER_APP));
    }
    Err(ErreurChemin::DossierUtilisateurIntrouvable)
}

/// Dossier des données de Central (base auth, données KindMother…).
///
/// `release = true` : dossier utilisateur absolu uniquement, jamais le cwd.
pub fn resoudre_dossier_donnees(env: &EnvChemins, release: bool) -> Result<PathBuf, ErreurChemin> {
    if release {
        dossier_utilisateur_donnees(env)
    } else {
        env.cwd.clone().ok_or(ErreurChemin::RepertoireCourant)
    }
}

/// Dossier de configuration (mws-config.json, miyucloud-config.json…).
///
/// Ordre : `%LOCALAPPDATA%`, `$XDG_CONFIG_HOME`, `$HOME/.config`. Sans aucun
/// d'eux : cwd en debug, refus en release.
pub fn resoudre_dossier_config(env: &EnvChemins, release: bool) -> Result<PathBuf, ErreurChemin> {
    if let Some(p) = absolu(env.local_app_data.as_ref()) {
        return Ok(p.join(DOSSIER_APP));
    }
    if let Some(p) = absolu(env.xdg_config_home.as_ref()) {
        return Ok(p.join(DOSSIER_APP));
    }
    if let Some(p) = absolu(env.home.as_ref()) {
        return Ok(p.join(".config").join(DOSSIER_APP));
    }
    if release {
        Err(ErreurChemin::DossierUtilisateurIntrouvable)
    } else {
        env.cwd.clone().ok_or(ErreurChemin::RepertoireCourant)
    }
}

/// Vrai pour un build release (les tests unitaires utilisent le résolveur pur).
pub const fn est_release() -> bool {
    !cfg!(debug_assertions)
}

/// Dossier de données du processus courant.
pub fn dossier_donnees() -> Result<PathBuf, ErreurChemin> {
    resoudre_dossier_donnees(&EnvChemins::du_processus(), est_release())
}

/// Dossier de configuration du processus courant.
pub fn dossier_config() -> Result<PathBuf, ErreurChemin> {
    resoudre_dossier_config(&EnvChemins::du_processus(), est_release())
}

/// Chemin de la base auth Central (`central.db`).
pub fn chemin_base_auth() -> Result<PathBuf, ErreurChemin> {
    Ok(dossier_donnees()?.join(NOM_BASE_AUTH))
}

/// Dossier de l'exécutable courant (binaires compagnons : kindmother-server…).
pub fn dossier_executable() -> Option<PathBuf> {
    std::env::current_exe().ok()?.parent().map(Path::to_path_buf)
}

// Chemins de test au format Unix (absolus seulement sous Unix).
#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn env_complet() -> EnvChemins {
        EnvChemins {
            local_app_data: None,
            xdg_data_home: None,
            xdg_config_home: None,
            home: Some(PathBuf::from("/home/u")),
            cwd: Some(PathBuf::from("/tmp/piege")),
        }
    }

    #[test]
    fn release_n_utilise_jamais_le_cwd() {
        let env = env_complet();
        let d = resoudre_dossier_donnees(&env, true).unwrap();
        assert_eq!(d, PathBuf::from("/home/u/.local/share/Miyukini-COG"));
        assert!(!d.starts_with("/tmp/piege"));
        let c = resoudre_dossier_config(&env, true).unwrap();
        assert!(!c.starts_with("/tmp/piege"));
    }

    #[test]
    fn release_sans_dossier_utilisateur_refuse() {
        let env = EnvChemins { cwd: Some(PathBuf::from("/tmp/piege")), ..Default::default() };
        assert_eq!(
            resoudre_dossier_donnees(&env, true),
            Err(ErreurChemin::DossierUtilisateurIntrouvable)
        );
        assert_eq!(
            resoudre_dossier_config(&env, true),
            Err(ErreurChemin::DossierUtilisateurIntrouvable)
        );
    }

    #[test]
    fn variables_relatives_ignorees() {
        let env = EnvChemins {
            local_app_data: Some(PathBuf::from("relatif")),
            xdg_data_home: Some(PathBuf::from("./x")),
            home: Some(PathBuf::from("/home/u")),
            cwd: Some(PathBuf::from("/tmp/piege")),
            ..Default::default()
        };
        assert_eq!(
            resoudre_dossier_donnees(&env, true).unwrap(),
            PathBuf::from("/home/u/.local/share/Miyukini-COG")
        );
    }

    #[test]
    fn priorite_localappdata_puis_xdg() {
        let mut env = env_complet();
        env.xdg_data_home = Some(PathBuf::from("/xdg/data"));
        assert_eq!(
            resoudre_dossier_donnees(&env, true).unwrap(),
            PathBuf::from("/xdg/data/Miyukini-COG")
        );
        env.local_app_data = Some(PathBuf::from("/lad"));
        assert_eq!(resoudre_dossier_donnees(&env, true).unwrap(), PathBuf::from("/lad/Miyukini-COG"));
    }

    #[test]
    fn debug_conserve_le_cwd() {
        let env = env_complet();
        assert_eq!(resoudre_dossier_donnees(&env, false).unwrap(), PathBuf::from("/tmp/piege"));
    }
}
