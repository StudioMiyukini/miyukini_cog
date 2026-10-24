//! @id     miyumarket_signer_cli
//! @do     signer_paquet_dev
//! @role   outillage
//! @layer  core
//! @human  miyumarket — outil de signature des paquets Market (éditeur / développement).

//! Outil de signature des paquets Market (Sceau v1, type `ManifestePaquet`).
//!
//! ```text
//! miyumarket-signer cle-nouvelle <fichier-cle>
//! miyumarket-signer cle-publique <fichier-cle>
//! miyumarket-signer signer --cle <fichier-cle> --id <id> --version <v> [--binaire <nom>] \
//!                          --entree <archive.zip> --sortie <paquet.msp>
//! ```
//!
//! Le fichier de clé contient la graine Ed25519 en hexadécimal ; il est créé
//! en 0600 (Unix) et n'est jamais écrasé. La clé publique affichée se colle
//! dans `market-editeurs.json` de Central (voir docs/tools/MiyuSceau).

use std::path::Path;
use std::process::ExitCode;

use miyumarket::signature::{hex_decoder, hex_encoder, signer_paquet};
use miyusceau::CleSignature;

fn usage() -> ExitCode {
    eprintln!(
        "usage :\n  miyumarket-signer cle-nouvelle <fichier-cle>\n  miyumarket-signer cle-publique <fichier-cle>\n  \
         miyumarket-signer signer --cle <fichier-cle> --id <id> --version <v> [--binaire <nom>] --entree <archive.zip> --sortie <paquet.msp>"
    );
    ExitCode::from(2)
}

/// Écrit un fichier neuf (jamais d'écrasement), 0600 sous Unix.
fn ecrire_fichier_prive(chemin: &Path, contenu: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut f = options.open(chemin)?;
    f.write_all(contenu)?;
    f.sync_all()
}

fn lire_cle(chemin: &Path) -> Result<CleSignature, String> {
    let texte = std::fs::read_to_string(chemin)
        .map_err(|e| format!("lecture de la clé {} : {e}", chemin.display()))?;
    let graine: [u8; 32] = hex_decoder(&texte)
        .and_then(|v| v.try_into().ok())
        .ok_or_else(|| "fichier de clé invalide (64 caractères hexadécimaux attendus)".to_string())?;
    Ok(CleSignature::depuis_graine(&graine))
}

fn valeur<'a>(args: &'a [String], nom: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == nom).map(|w| w[1].as_str())
}

fn executer(args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some("cle-nouvelle") => {
            let chemin = args.get(1).ok_or("fichier de clé manquant")?;
            let cle = CleSignature::generer();
            ecrire_fichier_prive(Path::new(chemin), hex_encoder(&cle.graine()).as_bytes())
                .map_err(|e| format!("écriture de {chemin} : {e}"))?;
            println!("{}", hex_encoder(&cle.cle_publique()));
            Ok(())
        }
        Some("cle-publique") => {
            let chemin = args.get(1).ok_or("fichier de clé manquant")?;
            let cle = lire_cle(Path::new(chemin))?;
            println!("{}", hex_encoder(&cle.cle_publique()));
            Ok(())
        }
        Some("signer") => {
            let cle = lire_cle(Path::new(valeur(args, "--cle").ok_or("--cle manquant")?))?;
            let id = valeur(args, "--id").ok_or("--id manquant")?;
            let version = valeur(args, "--version").ok_or("--version manquant")?;
            let binaire = valeur(args, "--binaire");
            let entree = valeur(args, "--entree").ok_or("--entree manquant")?;
            let sortie = valeur(args, "--sortie").ok_or("--sortie manquant")?;
            let archive =
                std::fs::read(entree).map_err(|e| format!("lecture de {entree} : {e}"))?;
            if miyumarket::signature::est_paquet_signe(&archive) {
                return Err("l'entrée est déjà un paquet signé".into());
            }
            let paquet = signer_paquet(&archive, id, version, binaire, &cle)
                .map_err(|e| format!("signature : {e}"))?;
            std::fs::write(sortie, paquet).map_err(|e| format!("écriture de {sortie} : {e}"))?;
            println!("paquet signé : {sortie} (éditeur {})", hex_encoder(&cle.cle_publique()));
            Ok(())
        }
        _ => Err(String::new()),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match executer(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) if e.is_empty() => usage(),
        Err(e) => {
            eprintln!("erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
