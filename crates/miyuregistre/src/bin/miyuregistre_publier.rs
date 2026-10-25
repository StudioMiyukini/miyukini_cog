//! @id     miyuregistre_bin_publier
//! @do     publier_registre_revocations_periodiquement
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre-publier — binaire côté COG : émet une tête fraîche du registre de révocation (clé de journal lue dans un fichier strict) et la publie vers le dépôt MiyuCloud et/ou Origin, toutes les 5 min au plus.

//! `miyuregistre-publier` — éditeur périodique du registre de révocation d'un COG.
//!
//! ```text
//! miyuregistre-publier --journal <dossier> --registre <64 hex> --cle-journal <fichier>
//!                      [--depot <dossier>] [--origin <url> --cog <cog_id>]
//!                      [--intervalle <secondes>] [--une-fois]
//! ```
//!
//! - `--journal` : dossier du journal (`entrees.mrgj`, `tetes.mrgj`, verrou
//!   `verrou`) ; propriétaire = uid effectif, sans écriture pour le groupe ni
//!   les autres. Tout écrivain du journal prend le verrou `flock` exclusif
//!   `<journal>/verrou` (pris ici le temps d'émettre la tête et de lire les
//!   entrées, avec un délai de 10 s : au-delà, le cycle échoue explicitement).
//! - Horloge : une tête n'est jamais datée au-delà de `maintenant + tolérance`
//!   (`miyusceau::TOLERANCE_HORLOGE_SECS`) ; saut d'horloge en arrière au-delà
//!   ⇒ cycle refusé (« horloge en recul »), à corriger par l'opérateur.
//! - `--cle-journal` : fichier de la graine (voir `editeur::cle`) ; la clé
//!   n'est **jamais** passée en argument. La clé publique est affichée au
//!   démarrage (à épingler chez Origin et MiyuCloud).
//! - Au moins une destination : `--depot` (MiyuCloud) et/ou `--origin` (avec `--cog`).
//! - `--intervalle` : 30 à 300 s (défaut 240) ; `--une-fois` : un seul cycle.
//!
//! Codes de sortie : 0 succès ; 1 échec d'un cycle (`--une-fois`) ; 2 usage
//! ou configuration ; 3 Origin refuse les envois (`401` / `403`) et aucune
//! autre destination ne reste.

use std::fmt::Write as _;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use miyauth::CogId;
use miyuregistre::editeur::cle::lire_cle_journal;
use miyuregistre::editeur::depot::DepotRevocations;
use miyuregistre::editeur::http::TransportHttp;
use miyuregistre::editeur::origin::{maintenant_unix, PublicationOrigin};
use miyuregistre::editeur::{Instantane, INTERVALLE_MAX_SECS};
use miyuregistre::{Journal, StockageFichier};
use miyusceau::CleSignature;
use miyuregistre::editeur::verrou::{verrouiller_exclusif, DELAI_VERROU};
use rustix::fs::{Mode, OFlags};

/// Fichier des entrées du journal.
const JOURNAL_ENTREES: &str = "entrees.mrgj";
/// Fichier des têtes émises.
const JOURNAL_TETES: &str = "tetes.mrgj";
/// Verrou des écrivains du journal.
const JOURNAL_VERROU: &str = "verrou";
const INTERVALLE_DEFAUT_SECS: u64 = 240;
const INTERVALLE_MIN_SECS: u64 = 30;

const USAGE: &str = "usage : miyuregistre-publier --journal <dossier> --registre <64 hex> --cle-journal <fichier> \
[--depot <dossier>] [--origin <url> --cog <cog_id>] [--intervalle <30..=300 s>] [--une-fois]";

fn journaliser(message: &str) {
    eprintln!("miyuregistre-publier : {message}");
}

struct Options {
    journal: PathBuf,
    registre: [u8; 32],
    cle: PathBuf,
    depot: Option<PathBuf>,
    origin: Option<(String, CogId)>,
    intervalle: u64,
    une_fois: bool,
}

fn hex32(texte: &str) -> Option<[u8; 32]> {
    let t = texte.trim().as_bytes();
    if t.len() != 64 {
        return None;
    }
    let mut r = [0u8; 32];
    let (paires, _) = t.as_chunks::<2>();
    for (i, p) in paires.iter().enumerate() {
        r[i] = u8::from_str_radix(std::str::from_utf8(p).ok()?, 16).ok()?;
    }
    Some(r)
}

fn analyser(args: impl Iterator<Item = String>) -> Result<Options, String> {
    let (mut journal, mut registre, mut cle, mut depot, mut origin, mut cog) = (None, None, None, None, None, None);
    let (mut intervalle, mut une_fois) = (INTERVALLE_DEFAUT_SECS, false);
    let mut args = args;
    while let Some(a) = args.next() {
        let mut valeur = || args.next().ok_or(format!("{a} : valeur manquante"));
        match a.as_str() {
            "--journal" => journal = Some(PathBuf::from(valeur()?)),
            "--registre" => registre = Some(hex32(&valeur()?).ok_or("--registre : 64 caractères hexadécimaux attendus")?),
            "--cle-journal" => cle = Some(PathBuf::from(valeur()?)),
            "--depot" => depot = Some(PathBuf::from(valeur()?)),
            "--origin" => origin = Some(valeur()?),
            "--cog" => cog = Some(CogId::new(valeur()?.trim()).ok_or("--cog : identifiant de COG invalide")?),
            "--intervalle" => {
                intervalle = valeur()?.parse().map_err(|_| "--intervalle : entier attendu")?;
                if !(INTERVALLE_MIN_SECS..=INTERVALLE_MAX_SECS).contains(&intervalle) {
                    return Err(format!("--intervalle : {INTERVALLE_MIN_SECS} à {INTERVALLE_MAX_SECS} s (fraîcheur 15 min)"));
                }
            }
            "--une-fois" => une_fois = true,
            "--aide" | "--help" | "-h" => return Err(USAGE.into()),
            _ => return Err(format!("argument inconnu « {a} »\n{USAGE}")),
        }
    }
    let origin = match (origin, cog) {
        (Some(u), Some(c)) => Some((u, c)),
        (None, None) => None,
        _ => return Err("--origin et --cog vont ensemble".into()),
    };
    if depot.is_none() && origin.is_none() {
        return Err(format!("aucune destination (--depot et/ou --origin)\n{USAGE}"));
    }
    Ok(Options {
        journal: journal.ok_or(USAGE)?,
        registre: registre.ok_or(USAGE)?,
        cle: cle.ok_or(USAGE)?,
        depot,
        origin,
        intervalle,
        une_fois,
    })
}

/// Contrôle le dossier du journal (dossier réel, propriétaire, aucune écriture groupe/autres).
fn controler_journal(dossier: &Path) -> Result<(), String> {
    let m = std::fs::symlink_metadata(dossier).map_err(|e| format!("journal {} : {e}", dossier.display()))?;
    if !m.is_dir() {
        return Err(format!("journal {} : dossier attendu (lien symbolique refusé)", dossier.display()));
    }
    if m.uid() != rustix::process::geteuid().as_raw() || m.mode() & 0o022 != 0 {
        return Err(format!("journal {} : propriétaire ou droits refusés (écriture groupe/autres)", dossier.display()));
    }
    Ok(())
}

/// Verrou exclusif des écrivains du journal (relâché à la destruction).
fn verrouiller(dossier: &Path) -> Result<File, String> {
    let fd = rustix::fs::open(
        dossier.join(JOURNAL_VERROU),
        OFlags::RDWR | OFlags::CREATE | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        Mode::from_raw_mode(0o600),
    )
    .map_err(|e| format!("verrou du journal : {e}"))?;
    // Délai borné : un écrivain figé ne bloque jamais le cycle indéfiniment.
    verrouiller_exclusif(&fd, DELAI_VERROU).map_err(|e| format!("verrou du journal : {e}"))?;
    Ok(File::from(fd))
}

/// Émet une tête fraîche sous le verrou du journal et en prend l'instantané.
fn instantane(o: &Options, cle: &CleSignature) -> Result<Instantane, String> {
    controler_journal(&o.journal)?;
    let _verrou = verrouiller(&o.journal)?;
    let entrees = StockageFichier::ouvrir(o.journal.join(JOURNAL_ENTREES)).map_err(|e| format!("{JOURNAL_ENTREES} : {e}"))?;
    let tetes = StockageFichier::ouvrir(o.journal.join(JOURNAL_TETES)).map_err(|e| format!("{JOURNAL_TETES} : {e}"))?;
    let mut journal = Journal::ouvrir(o.registre, entrees, tetes).map_err(|e| format!("journal : {e}"))?;
    Instantane::emettre(&mut journal, cle, maintenant_unix()).map_err(|e| format!("émission de la tête : {e}"))
}

fn principal() -> Result<(), (u8, String)> {
    let o = analyser(std::env::args().skip(1)).map_err(|e| (2, e))?;
    let cle = lire_cle_journal(&o.cle).map_err(|e| (2, e.to_string()))?;
    let publique = cle.cle_publique().iter().fold(String::with_capacity(64), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    });
    journaliser(&format!("clé publique de journal {publique}"));
    let mut depot = match &o.depot {
        Some(d) => Some(DepotRevocations::ouvrir(d).map_err(|e| (2, e.to_string()))?),
        None => None,
    };
    let mut origin = match &o.origin {
        Some((url, cog)) => Some(PublicationOrigin::new(cog.clone(), &cle, TransportHttp::new(url).map_err(|e| (2, e))?)),
        None => None,
    };
    loop {
        let debut = Instant::now();
        let mut echec = false;
        match instantane(&o, &cle) {
            Err(e) => {
                journaliser(&e);
                echec = true;
            }
            Ok(inst) => {
                if let Some(d) = depot.as_mut() {
                    if let Err(e) = d.publier(&inst) {
                        journaliser(&format!("dépôt {} : {e}", d.chemin().display()));
                        echec = true;
                    }
                }
                if let Some(c) = origin.as_mut() {
                    match c.publier(&inst) {
                        Ok(b) if b.ajoutees > 0 || b.conflits > 0 => journaliser(&format!(
                            "Origin à jour : {} entrées, {} nouvelles révocations, {} envois, {} reprises",
                            b.taille, b.ajoutees, b.envois, b.conflits
                        )),
                        Ok(_) => {}
                        Err(e) if e.est_definitive() => {
                            journaliser(&format!("{e} — publication vers Origin abandonnée"));
                            origin = None;
                            echec = true;
                            if depot.is_none() {
                                return Err((3, "aucune destination restante".into()));
                            }
                        }
                        Err(e) => {
                            journaliser(&format!("Origin : {e}"));
                            echec = true;
                        }
                    }
                }
            }
        }
        if o.une_fois {
            return if echec { Err((1, "cycle en échec".into())) } else { Ok(()) };
        }
        std::thread::sleep(Duration::from_secs(o.intervalle).saturating_sub(debut.elapsed()));
    }
}

fn main() -> ExitCode {
    match principal() {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            journaliser(&message);
            ExitCode::from(code)
        }
    }
}
