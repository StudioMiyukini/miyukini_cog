//! `alicia-memoire-import` — import unique de l'ancienne base SQLite de la
//! mémoire d'Alicia dans KindMother (K-13).
//!
//! ```text
//! alicia-memoire-import <ancienne-base.db> [--adresse HÔTE:PORT] [--justificatifs DOSSIER]
//!                       [--conserver-ancienne]
//! ```
//!
//! Adresse par défaut : `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051` ;
//! justificatif `maia-_alicia` à l'emplacement standard de KindMother.
//! Idempotent (marqueur `<base>.importe-kindmother`, qui garde l'empreinte
//! SHA-256 de l'ancien fichier). Après l'import vérifié, l'ancienne base (et
//! ses `-wal`, `-shm`, `-journal`) est **réécrite puis supprimée** ; relancée
//! sur une copie déjà importée et identique, la commande l'efface.
//! `--conserver-ancienne` la laisse intacte : l'oubli d'un fait est alors
//! refusé tant qu'elle existe. Code de sortie : 0 succès, 1 échec, 2 usage.

// @id service.alicia.memory.bin.import
// @role memory_legacy_import_cli
// @layer 7
// @do import_legacy_alicia_memory_cli
// @human Commande explicite d'import de l'ancienne mémoire SQLite d'Alicia dans KindMother.

use std::path::PathBuf;
use std::process::ExitCode;

use miyualicia_memory::{AncienneBase, ConfigKindMother, MemoryStore};

const USAGE: &str = "usage : alicia-memoire-import <ancienne-base.db> [--adresse HÔTE:PORT] \
                     [--justificatifs DOSSIER] [--conserver-ancienne]";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut base: Option<PathBuf> = None;
    let mut cfg = ConfigKindMother::depuis_env();
    let mut ancienne = AncienneBase::Effacer;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--conserver-ancienne" => ancienne = AncienneBase::Conserver,
            "--adresse" => match args.next() {
                Some(v) => cfg.adresse = v,
                None => return usage(),
            },
            "--justificatifs" => match args.next() {
                Some(v) => cfg = cfg.avec_justificatifs(v),
                None => return usage(),
            },
            "-h" | "--help" => {
                println!("{USAGE}");
                return ExitCode::SUCCESS;
            }
            _ if base.is_none() && !a.starts_with("--") => base = Some(PathBuf::from(a)),
            _ => return usage(),
        }
    }
    let Some(base) = base else {
        return usage();
    };
    if !base.is_file() {
        eprintln!("erreur : {} : fichier introuvable", base.display());
        return ExitCode::FAILURE;
    }
    let store = match MemoryStore::connecter(&cfg) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("erreur : {e}");
            return ExitCode::FAILURE;
        }
    };
    match store.importer_base_heritee_avec(&base, ancienne) {
        Ok(rapport) => {
            print!("{rapport}");
            match ancienne {
                AncienneBase::Effacer => {
                    println!("  ancienne base : effacée (réécrite puis supprimée), empreinte gardée au marqueur.");
                }
                AncienneBase::Conserver => println!(
                    "  ancienne base : conservée à la demande ; l'oubli est refusé tant qu'elle existe."
                ),
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}

fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}
