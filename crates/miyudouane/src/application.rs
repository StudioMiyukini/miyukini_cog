//! @id     miyudouane_application
//! @do     appliquer_regles_nft_privilegie
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — étape privilégiée et explicite : `nft -c -f` puis `nft -f` dans l'espace de noms d'une frontière.

//! # Application (étape privilégiée)
//!
//! **Seul** point du crate qui touche au système. Appelé uniquement par la CLI
//! avec `--appliquer` ; jamais par les tests ni par la génération.
//!
//! Étapes : audit du texte → écriture d'un fichier `0600` → `nft -c -f`
//! (contrôle à blanc) → `nft -f` (application atomique), dans l'espace de
//! noms demandé (`ip netns exec <ns>`). Tout écart arrête la procédure.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::Command;

use crate::hote::auditer_hote;
use crate::nft::auditer;

/// Échec de l'application.
#[derive(Debug)]
pub enum ErreurApplication {
    /// L'audit a trouvé des écarts : rien n'est appliqué.
    Audit(Vec<String>),
    /// Nom d'espace de noms refusé.
    NetnsInvalide,
    /// Erreur d'entrée-sortie.
    Es(std::io::Error),
    /// `nft` a refusé (étape, sortie d'erreur).
    Nft(&'static str, String),
}

impl std::fmt::Display for ErreurApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audit(e) => write!(f, "audit en échec : {}", e.join(" ; ")),
            Self::NetnsInvalide => f.write_str("espace de noms invalide"),
            Self::Es(e) => write!(f, "entrée-sortie : {e}"),
            Self::Nft(etape, err) => write!(f, "nft ({etape}) : {err}"),
        }
    }
}

impl std::error::Error for ErreurApplication {}

fn nft(netns: Option<&str>, args: &[&str]) -> Command {
    let mut c = if let Some(ns) = netns {
        let mut c = Command::new("ip");
        c.args(["netns", "exec", ns, "nft"]);
        c
    } else {
        Command::new("nft")
    };
    c.args(args);
    c
}

fn executer(mut c: Command, etape: &'static str) -> Result<(), ErreurApplication> {
    let sortie = c.output().map_err(ErreurApplication::Es)?;
    if sortie.status.success() {
        Ok(())
    } else {
        Err(ErreurApplication::Nft(etape, String::from_utf8_lossy(&sortie.stderr).into_owned()))
    }
}

/// Audite, contrôle (`nft -c -f`) puis applique (`nft -f`) un texte de règles.
///
/// `fichier` : chemin d'écriture (créé en `0600`, remplacé s'il existe).
pub fn appliquer(
    texte: &str,
    voies_declarees: &BTreeSet<String>,
    netns: Option<&str>,
    fichier: &Path,
) -> Result<(), ErreurApplication> {
    let ecarts = auditer(texte, voies_declarees);
    if !ecarts.is_empty() {
        return Err(ErreurApplication::Audit(ecarts));
    }
    if let Some(ns) = netns {
        if ns.is_empty() || !ns.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-') {
            return Err(ErreurApplication::NetnsInvalide);
        }
    }
    ecrire_controler_appliquer(texte, netns, fichier)
}

/// Table hôte `inet miyudouane_hote` ([`crate::hote`]) : audit propre à
/// l'hôte ([`auditer_hote`] : aucune autre table, jamais `flush ruleset`),
/// puis `nft -c -f` et `nft -f` **sur l'hôte** (pas d'espace de noms). Le
/// fichier ne remplace que cette table, en une transaction.
pub fn appliquer_hote(texte: &str, voies_exceptions: &BTreeSet<String>, fichier: &Path) -> Result<(), ErreurApplication> {
    let ecarts = auditer_hote(texte, voies_exceptions);
    if !ecarts.is_empty() {
        return Err(ErreurApplication::Audit(ecarts));
    }
    ecrire_controler_appliquer(texte, None, fichier)
}

fn ecrire_controler_appliquer(texte: &str, netns: Option<&str>, fichier: &Path) -> Result<(), ErreurApplication> {
    let _ = fs::remove_file(fichier);
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(fichier)
        .map_err(ErreurApplication::Es)?;
    f.write_all(texte.as_bytes()).map_err(ErreurApplication::Es)?;
    f.sync_all().map_err(ErreurApplication::Es)?;
    let chemin = fichier.to_string_lossy();
    executer(nft(netns, &["-c", "-f", &chemin]), "contrôle")?;
    executer(nft(netns, &["-f", &chemin]), "application")
}
