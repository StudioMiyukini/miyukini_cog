//! `alicia-memoire-import` — import unique de l'ancienne base SQLite de la
//! mémoire d'Alicia dans KindMother (K-13).
//!
//! ```text
//! alicia-memoire-import <ancienne-base.db> [--adresse HÔTE:PORT] [--justificatifs DOSSIER]
//! ```
//!
//! Adresse par défaut : `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051` ;
//! justificatif `maia-_alicia` à l'emplacement standard de KindMother.
//! Idempotent (marqueur `<base>.importe-kindmother`) ; le fichier hérité n'est
//! jamais modifié ni supprimé. Code de sortie : 0 succès, 1 échec, 2 usage.

// @id service.alicia.memory.bin.import
// @role memory_legacy_import_cli
// @layer 7
// @do import_legacy_alicia_memory_cli
// @human Commande explicite d'import de l'ancienne mémoire SQLite d'Alicia dans KindMother.

use std::path::PathBuf;
use std::process::ExitCode;

use miyualicia_memory::{ConfigKindMother, MemoryStore};

const USAGE: &str =
    "usage : alicia-memoire-import <ancienne-base.db> [--adresse HÔTE:PORT] [--justificatifs DOSSIER]";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut base: Option<PathBuf> = None;
    let mut cfg = ConfigKindMother::depuis_env();
    while let Some(a) = args.next() {
        match a.as_str() {
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
    match store.importer_base_heritee(&base) {
        Ok(rapport) => {
            print!("{rapport}");
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
