//! @id     kindmother_import_main
//! @do     importer_base_heritee_en_ligne_de_commande
//! @role   outillage
//! @layer  outillage
//! @human  `kindmother-import <service> <chemin_base_heritee>` : import unique,
//!         idempotent et non destructif d'une base SQLite héritée dans KindMother (K-13).

//! CLI d'import des bases SQLite héritées.
//!
//! ```text
//! kindmother-import <service> <chemin_base_heritee>
//!   service : jaykonta | jay1tribu | jaykoa | miyukiniwatch | miyukini-central (alias central)
//! ```
//!
//! - Adresse du service : `KINDMOTHER_ADDR` (défaut `127.0.0.1:50051`).
//! - Justificatif : celui de l'opérateur du service, comme le service lui-même
//!   (`OperatorCredential::load` : `KINDMOTHER_CREDENTIALS_DIR`, sinon
//!   `$KINDMOTHER_DATA_DIR/credentials`, sinon `./data/kindmother/credentials`).
//! - Le fichier hérité est ouvert en lecture seule et jamais modifié ; le marqueur
//!   `<chemin>.importe-kindmother` est écrit après une vérification complète, ce qui
//!   autorise le service à démarrer. Relancer après succès : rien à faire.
//!
//! Codes de sortie : 0 succès (ou déjà importé), 1 échec (aucun marqueur écrit),
//! 2 usage.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use kindmother_client::{KindMotherClient, OperatorCredential};
use kindmother_heritage::{ErreurImport, PlanImport, RapportImport};

/// Adresse par défaut du service KindMother.
const ADRESSE_DEFAUT: &str = "127.0.0.1:50051";

/// Services importables : nom et alias éventuel.
const SERVICES: &[(&str, &str)] = &[
    ("jaykonta", ""),
    ("jay1tribu", ""),
    ("jaykoa", ""),
    ("miyukiniwatch", ""),
    ("miyukini-central", "central"),
];

fn usage() -> String {
    let noms: Vec<String> = SERVICES
        .iter()
        .map(|(n, a)| {
            if a.is_empty() {
                (*n).to_string()
            } else {
                format!("{n} (alias {a})")
            }
        })
        .collect();
    format!(
        "usage : kindmother-import <service> <chemin_base_heritee>\n\
         services : {}\n\
         environnement : KINDMOTHER_ADDR (défaut {ADRESSE_DEFAUT}), KINDMOTHER_CREDENTIALS_DIR \
         ou KINDMOTHER_DATA_DIR (justificatif de l'opérateur du service)",
        noms.join(", ")
    )
}

/// Plan du service nommé.
fn plan_du_service(nom: &str) -> Option<PlanImport> {
    Some(match nom {
        "jaykonta" => jaykonta::data::import_heritage::plan(),
        "jay1tribu" => jay1tribu::data::import_heritage::plan(),
        "jaykoa" => jaykoa::data::import_heritage::plan(),
        "miyukiniwatch" => miyukiniwatch::data::import_heritage::plan(),
        "miyukini-central" | "central" => miyukini_central::auth::import_heritage::plan(),
        _ => return None,
    })
}

/// Import par la fonction de bibliothèque du service.
async fn importer(
    service: &str,
    chemin: &Path,
    client: &KindMotherClient,
) -> Result<RapportImport, ErreurImport> {
    match service {
        "jaykonta" => jaykonta::data::import_heritage::importer_base_heritee(chemin, client).await,
        "jay1tribu" => {
            jay1tribu::data::import_heritage::importer_base_heritee(chemin, client).await
        }
        "jaykoa" => jaykoa::data::import_heritage::importer_base_heritee(chemin, client).await,
        "miyukiniwatch" => {
            miyukiniwatch::data::import_heritage::importer_base_heritee(chemin, client).await
        }
        _ => miyukini_central::auth::import_heritage::importer_base_heritee(chemin, client).await,
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("{}", usage());
        return ExitCode::SUCCESS;
    }
    let [service, chemin] = args.as_slice() else {
        eprintln!("{}", usage());
        return ExitCode::from(2);
    };
    let Some(plan) = plan_du_service(service) else {
        eprintln!("service inconnu : {service}\n{}", usage());
        return ExitCode::from(2);
    };
    let chemin = PathBuf::from(chemin);

    let adresse = std::env::var("KINDMOTHER_ADDR")
        .ok()
        .filter(|a| !a.trim().is_empty())
        .unwrap_or_else(|| ADRESSE_DEFAUT.to_string());
    let justificatif = match OperatorCredential::load(plan.operateur) {
        Ok(j) => j,
        Err(e) => {
            eprintln!(
                "justificatif de l'opérateur {} introuvable ({}) : {e}",
                plan.operateur,
                OperatorCredential::default_dir().display()
            );
            return ExitCode::FAILURE;
        }
    };
    let client = match KindMotherClient::connect(&adresse, &justificatif, plan.base).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("connexion à KindMother ({adresse}) impossible : {e}");
            return ExitCode::FAILURE;
        }
    };

    println!(
        "import de {} ({}) vers KindMother {adresse}, base « {} »…",
        chemin.display(),
        plan.service,
        plan.base
    );
    match importer(plan.service, &chemin, &client).await {
        Ok(rapport) => {
            print!("{rapport}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!(
                "ÉCHEC de l'import ({}) : {e}\n\
                 Aucun marqueur écrit : le service refusera toujours de démarrer. Le fichier \
                 hérité n'a pas été modifié ; l'import peut être relancé (reprise sûre).",
                plan.service
            );
            ExitCode::FAILURE
        }
    }
}
