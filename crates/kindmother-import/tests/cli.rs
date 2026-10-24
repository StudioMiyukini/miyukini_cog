//! @id     kindmother_import_tests_cli
//! @do     verifier_cli_kindmother_import
//! @role   test
//! @layer  outillage
//! @human  `kindmother-import` de bout en bout : usage, import, relance no-op, échec sans marqueur.

use std::process::Command;

use kindmother_client::heritage::chemin_marqueur_import;
use kindmother_client::SqlValue;

const DDL_JAYKOA: &str = "
CREATE TABLE agendas (id TEXT PRIMARY KEY, profile_id TEXT NOT NULL, name TEXT NOT NULL,
    description TEXT, color TEXT DEFAULT '#4285F4', calendar_type TEXT DEFAULT 'personal',
    visible INTEGER DEFAULT 1, is_default INTEGER DEFAULT 0, source_service TEXT,
    created_at TEXT, updated_at TEXT);
";

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_kindmother-import"))
}

#[test]
fn usage_import_et_relance() {
    let sortie = cli().output().expect("exécution");
    assert_eq!(sortie.status.code(), Some(2), "usage sans argument");
    let sortie = cli().args(["inconnu", "x.db"]).output().expect("exécution");
    assert_eq!(sortie.status.code(), Some(2), "service inconnu");

    let tmp = tempfile_dir();
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(tmp.join("km")).expect("serveur éphémère");
    let base = tmp.join("jaykoa.db");
    kindmother_heritage::fabrique::creer_base_heritee(
        &base,
        DDL_JAYKOA,
        &[(
            "agendas",
            vec![
                ("id", SqlValue::Text("ag1".into())),
                ("profile_id", SqlValue::Text("p1".into())),
                ("name", SqlValue::Text("Perso".into())),
            ],
        )],
    )
    .expect("base héritée");
    let lancer = || {
        cli()
            .args(["jaykoa", base.to_str().expect("chemin")])
            .env("KINDMOTHER_ADDR", addr.to_string())
            .env(kindmother_client::ENV_CREDENTIALS_DIR, &creds)
            .output()
            .expect("exécution")
    };

    let sortie = lancer();
    let texte = String::from_utf8_lossy(&sortie.stdout);
    assert!(
        sortie.status.success(),
        "{texte}\n{}",
        String::from_utf8_lossy(&sortie.stderr)
    );
    assert!(texte.contains("agendas"), "{texte}");
    assert!(chemin_marqueur_import(&base).is_file());

    let sortie = lancer();
    assert!(sortie.status.success());
    assert!(String::from_utf8_lossy(&sortie.stdout).contains("déjà importée"));

    // Échec (KindMother injoignable) : code 1, pas de marqueur.
    let autre = tmp.join("autre.db");
    std::fs::copy(&base, &autre).expect("copie");
    let sortie = cli()
        .args(["jaykoa", autre.to_str().expect("chemin")])
        .env("KINDMOTHER_ADDR", "127.0.0.1:1")
        .env(kindmother_client::ENV_CREDENTIALS_DIR, &creds)
        .output()
        .expect("exécution");
    assert_eq!(sortie.status.code(), Some(1));
    assert!(!chemin_marqueur_import(&autre).exists());
    let _ = std::fs::remove_dir_all(&tmp);
}

fn tempfile_dir() -> std::path::PathBuf {
    let d = std::env::temp_dir().join(format!("kindmother-import-cli-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).expect("tempdir");
    d
}
