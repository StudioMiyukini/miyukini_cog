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
//!
//! Mise en forme `tc` : [`appliquer_tc`] (audit, refus de toute racine
//! étrangère avant modification, `tc -batch` dans l'espace de la frontière).
//!
//! ## Ordre sûr (frontière externe, A-023)
//!
//! Le filet `DEBIT` 5/4 par sens ([`crate::nft::MiseEnForme::Appliquee`])
//! relève le plafond nftables de 25 % : il n'est sûr que si `tc` met
//! effectivement le trafic en forme. L'installateur procède donc toujours,
//! à la première installation comme à chaque mise à jour :
//!
//! 1. **(a)** règles **strictes** ([`appliquer`], `miyudouane regles …
//!    --appliquer`) — `limit rate over D`, sans rafale ajoutée ;
//! 2. **(b)** mise en forme ([`appliquer_tc`], `miyudouane tc … --appliquer`) ;
//!    refusée si la table `inet miyudouane` de l'espace est absente ou porte
//!    déjà un filet (l'étape (a) n'a pas été faite) ; en cas d'échec, la
//!    racine de la Douane est retirée et **les règles strictes restent** ;
//! 3. **(c)** seulement si (b) a réussi : règles avec filet
//!    ([`appliquer_avec_filet`], `miyudouane regles --frontiere externe
//!    --mise-en-forme-appliquee --appliquer`) ; refusées si une interface de la
//!    frontière ne porte pas la racine `tc` de la Douane.
//!
//! Aucune étape n'est automatique : un échec en (b) ou en (c) laisse la
//! frontière sous la règle stricte. Retirer la mise en forme impose de
//! ré-appliquer d'abord (a).

use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::Command;

use crate::hote::auditer_hote;
use crate::nft::{auditer, auditer_selon, porte_filet, MiseEnForme, TABLE_FRONTIERE};
use crate::tc::{auditer_tc, classer_racine, PlanTc, RacineTc, POIGNEE_RACINE};

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
    /// `tc` a refusé (étape, sortie d'erreur).
    Tc(&'static str, String),
    /// Une interface porte une configuration `tc` qui n'est pas celle de la
    /// Douane : rien n'est touché.
    TcEtranger(String),
    /// Ordre (a) strict → (b) `tc` → (c) filet non respecté : rien n'est touché.
    Ordre(String),
}

impl std::fmt::Display for ErreurApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audit(e) => write!(f, "audit en échec : {}", e.join(" ; ")),
            Self::NetnsInvalide => f.write_str("espace de noms invalide"),
            Self::Es(e) => write!(f, "entrée-sortie : {e}"),
            Self::Nft(etape, err) => write!(f, "nft ({etape}) : {err}"),
            Self::Tc(etape, err) => write!(f, "tc ({etape}) : {err}"),
            Self::TcEtranger(e) => write!(f, "configuration tc étrangère, rien n'est touché : {e}"),
            Self::Ordre(e) => write!(f, "ordre d'application (règles strictes → tc → filet) : {e}"),
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
        if !netns_valide(ns) {
            return Err(ErreurApplication::NetnsInvalide);
        }
    }
    ecrire_controler_appliquer(texte, netns, fichier)
}

/// Étape (c) de l'ordre sûr : règles de la frontière externe avec filet 5/4
/// ([`MiseEnForme::Appliquee`]). Audit selon cette forme ([`auditer_selon`]),
/// puis vérification que **chaque** interface de `interfaces` porte la racine
/// `tc` de la Douane (`htb 4d44:`) ; sinon refus avant toute modification
/// ([`ErreurApplication::Ordre`]) et les règles en place (strictes) restent.
pub fn appliquer_avec_filet(
    texte: &str,
    voies_declarees: &BTreeSet<String>,
    netns: &str,
    interfaces: &BTreeSet<String>,
    fichier: &Path,
) -> Result<(), ErreurApplication> {
    let ecarts = auditer_selon(texte, voies_declarees, MiseEnForme::Appliquee);
    if !ecarts.is_empty() {
        return Err(ErreurApplication::Audit(ecarts));
    }
    if !netns_valide(netns) {
        return Err(ErreurApplication::NetnsInvalide);
    }
    if interfaces.is_empty() {
        return Err(ErreurApplication::Ordre("aucune interface de frontière".into()));
    }
    let mut sans = Vec::new();
    for dev in interfaces {
        match classer_racine(&sortie_tc(tc(netns, &["qdisc", "show", "dev", dev, "root"]), "lecture")?) {
            RacineTc::Douane => {}
            RacineTc::Defaut => sans.push(format!("{dev} : file par défaut")),
            RacineTc::Etrangere(e) => sans.push(format!("{dev} : {e}")),
        }
    }
    if !sans.is_empty() {
        return Err(ErreurApplication::Ordre(format!(
            "mise en forme tc absente ({}) : appliquer d'abord `miyudouane tc … --appliquer`",
            sans.join(" ; ")
        )));
    }
    ecrire_controler_appliquer(texte, Some(netns), fichier)
}

fn netns_valide(ns: &str) -> bool {
    !ns.is_empty() && ns.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn tc(netns: &str, args: &[&str]) -> Command {
    let mut c = Command::new("ip");
    c.args(["netns", "exec", netns, "tc"]);
    c.args(args);
    c
}

fn sortie_tc(c: Command, etape: &'static str) -> Result<String, ErreurApplication> {
    let mut c = c;
    let sortie = c.output().map_err(ErreurApplication::Es)?;
    if sortie.status.success() {
        Ok(String::from_utf8_lossy(&sortie.stdout).into_owned())
    } else {
        Err(ErreurApplication::Tc(etape, String::from_utf8_lossy(&sortie.stderr).into_owned()))
    }
}

/// Mise en forme `tc` ([`crate::tc`]) dans l'espace de noms de la frontière
/// (étape (b) de l'ordre sûr, voir le module) :
///
/// 1. audit ([`auditer_tc`] : écart au rendu attendu ⇒ refus) ;
///    puis ordre : la table `inet miyudouane` de l'espace doit exister et
///    **ne porter aucun filet** (`nft list table`, [`porte_filet`]) — les
///    règles strictes (a) sont en place ; sinon refus avant toute
///    modification ([`ErreurApplication::Ordre`]) ;
/// 2. lecture de la racine de **chaque** interface du plan ; une racine qui
///    n'est ni celle de la Douane (`htb 4d44:`) ni la file par défaut du noyau
///    ⇒ refus **avant toute modification** ([`ErreurApplication::TcEtranger`]) ;
/// 3. écriture du fichier `0600` ;
/// 4. retrait de la seule racine `4d44:` là où elle existe ;
/// 5. `tc -batch` (un seul processus, arrêt à la première erreur ; `qdisc
///    add … root` échoue si une racine étrangère est apparue entre temps).
///
/// En cas d'échec en 4 ou 5, la racine de la Douane est retirée des
/// interfaces où elle se trouve (retour à la file par défaut ; les règles
/// **strictes** vérifiées en 1 restent en place) et l'erreur est rendue.
pub fn appliquer_tc(texte: &str, attendu: &PlanTc, fichier: &Path) -> Result<(), ErreurApplication> {
    let ecarts = auditer_tc(texte, attendu);
    if !ecarts.is_empty() {
        return Err(ErreurApplication::Audit(ecarts));
    }
    let ns = attendu.netns.as_str();
    if !netns_valide(ns) {
        return Err(ErreurApplication::NetnsInvalide);
    }
    let mut lister = Command::new("ip");
    lister.args(["netns", "exec", ns, "nft", "list", "table", "inet", TABLE_FRONTIERE]);
    let en_place = lister.output().map_err(ErreurApplication::Es)?;
    if !en_place.status.success() {
        return Err(ErreurApplication::Ordre(format!(
            "table inet {TABLE_FRONTIERE} absente de {ns} : appliquer d'abord les règles strictes"
        )));
    }
    if porte_filet(&String::from_utf8_lossy(&en_place.stdout)) {
        return Err(ErreurApplication::Ordre(format!(
            "filet 5/4 en place dans {ns} : ré-appliquer d'abord les règles strictes"
        )));
    }
    let mut a_retirer = Vec::new();
    let mut etrangeres = Vec::new();
    for i in &attendu.interfaces {
        let dev = i.interface.as_str();
        match classer_racine(&sortie_tc(tc(ns, &["qdisc", "show", "dev", dev, "root"]), "lecture")?) {
            RacineTc::Defaut => {}
            RacineTc::Douane => a_retirer.push(dev),
            RacineTc::Etrangere(e) => etrangeres.push(format!("{dev} : {e}")),
        }
    }
    if !etrangeres.is_empty() {
        return Err(ErreurApplication::TcEtranger(etrangeres.join(" ; ")));
    }
    ecrire(texte, fichier)?;
    let chemin = fichier.to_string_lossy().into_owned();
    // `handle 4d44:` : le noyau refuse la suppression si la racine a changé
    // de main entre la lecture et le retrait.
    let poignee = format!("{POIGNEE_RACINE:x}:");
    let resultat = a_retirer
        .iter()
        .try_for_each(|dev| {
            sortie_tc(tc(ns, &["qdisc", "del", "dev", dev, "root", "handle", &poignee]), "retrait").map(|_| ())
        })
        .and_then(|()| sortie_tc(tc(ns, &["-batch", &chemin]), "application").map(|_| ()));
    if resultat.is_err() {
        retirer_racines_douane(attendu);
    }
    resultat
}

/// Retire la racine de la Douane (et elle seule) des interfaces du plan.
fn retirer_racines_douane(attendu: &PlanTc) {
    let ns = attendu.netns.as_str();
    for i in &attendu.interfaces {
        let dev = i.interface.as_str();
        let lue = sortie_tc(tc(ns, &["qdisc", "show", "dev", dev, "root"]), "lecture");
        if lue.is_ok_and(|s| classer_racine(&s) == RacineTc::Douane) {
            let poignee = format!("{POIGNEE_RACINE:x}:");
            let _ = sortie_tc(tc(ns, &["qdisc", "del", "dev", dev, "root", "handle", &poignee]), "retrait");
        }
    }
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

/// Écrit le texte dans un fichier neuf `0600` (remplace l'ancien).
fn ecrire(texte: &str, fichier: &Path) -> Result<(), ErreurApplication> {
    let _ = fs::remove_file(fichier);
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(fichier)
        .map_err(ErreurApplication::Es)?;
    f.write_all(texte.as_bytes()).map_err(ErreurApplication::Es)?;
    f.sync_all().map_err(ErreurApplication::Es)
}

fn ecrire_controler_appliquer(texte: &str, netns: Option<&str>, fichier: &Path) -> Result<(), ErreurApplication> {
    ecrire(texte, fichier)?;
    let chemin = fichier.to_string_lossy();
    executer(nft(netns, &["-c", "-f", &chemin]), "contrôle")?;
    executer(nft(netns, &["-f", &chemin]), "application")
}
