//! @id     miyudouane_tests_cli
//! @do     tester_cli_douane
//! @role   test
//! @layer  operateur
//! @human  miyudouane — CLI : règles de zone, coupure de voie, scripts de mise en place et de retrait (sans rien appliquer).

mod commun;

use std::path::PathBuf;
use std::process::Command;

use borderguard::{sceller_arrete, Arrete, Constitution};
use commun::*;

const SANDBOXES: [&str; 6] =
    ["cog:jayoffice", "franche:jayoffice", "franche:site", "cog:alicia", "locale:alicia", "cog:site"];

struct Banc {
    dossier: PathBuf,
    ancre: String,
}

impl Banc {
    fn nouveau(nom: &str) -> Self {
        let dossier = std::env::temp_dir().join(format!("miyudouane-cli-{nom}-{}", std::process::id()));
        std::fs::create_dir_all(&dossier).unwrap();
        let ancre = cle().cle_publique().iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
            s
        });
        Self { dossier, ancre }
    }

    fn sceller(&self, nom: &str, a: &Arrete) -> String {
        let chemin = self.dossier.join(nom);
        std::fs::write(&chemin, sceller_arrete(a, &cle(), &Constitution::reference()).unwrap()).unwrap();
        chemin.display().to_string()
    }

    fn lancer(&self, args: &[&str]) -> (bool, String, String) {
        assert!(self.dossier.exists());
        let mut c = Command::new(env!("CARGO_BIN_EXE_miyudouane"));
        c.args(args).args(["--maintenant", &(T0 + 1).to_string()]);
        for s in SANDBOXES {
            c.args(["--sandbox", s]);
        }
        let o = c.output().unwrap();
        (o.status.success(), String::from_utf8_lossy(&o.stdout).into_owned(), String::from_utf8_lossy(&o.stderr).into_owned())
    }
}

impl Drop for Banc {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dossier);
    }
}

#[test]
fn zone_cog_depuis_deux_frontieres() {
    let b = Banc::nouveau("zone");
    let pi = b.sceller("pi.arrete", &arrete_portail(1));
    let pl = b.sceller("pl.arrete", &arrete_local(1));
    let (ok, texte, err) = b.lancer(&[
        "zone",
        "--zone",
        "cog",
        "--arrete",
        &format!("portail-interne={pi}"),
        "--arrete",
        &format!("portail-local={pl}"),
        "--ancre",
        &b.ancre,
    ]);
    assert!(ok, "{err}");
    // Même texte que la référence de bibliothèque (même plan, mêmes arrêtés).
    let reference = std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/zone-cog.nft")).unwrap();
    assert_eq!(texte, reference);

    // Coupure d'une voie : ses règles disparaissent du pont, les autres restent.
    let (ok, coupe, err) = b.lancer(&[
        "zone",
        "--zone",
        "cog",
        "--arrete",
        &format!("portail-interne={pi}"),
        "--arrete",
        &format!("portail-local={pl}"),
        "--ancre",
        &b.ancre,
        "--couper",
        "site",
    ]);
    assert!(ok, "{err}");
    assert!(!coupe.contains("voie=site"));
    assert!(coupe.contains("voie=jayoffice") && coupe.contains("voie=alicia"));
}

#[test]
fn zone_refuse_les_incoherences() {
    let b = Banc::nouveau("refus");
    let pi = b.sceller("pi.arrete", &arrete_portail(1));
    let ext = b.sceller("ext.arrete", &arrete_externe(1));
    // Frontière annoncée ≠ frontière de l'arrêté.
    let (ok, _, err) = b.lancer(&["zone", "--zone", "cog", "--arrete", &format!("portail-local={pi}"), "--ancre", &b.ancre]);
    assert!(!ok && err.contains("portail-interne"), "{err}");
    // Frontière qui ne touche pas la zone.
    let (ok, _, err) = b.lancer(&["zone", "--zone", "cog", "--arrete", &format!("externe={ext}"), "--ancre", &b.ancre]);
    assert!(!ok && err.contains("ne touche pas"), "{err}");
    // Mauvaise ancre : refus (fail-closed).
    let autre = "11".repeat(32);
    let (ok, _, _) = b.lancer(&["zone", "--zone", "cog", "--arrete", &format!("portail-interne={pi}"), "--ancre", &autre]);
    assert!(!ok);
    // Forme sans « frontière= ».
    let (ok, _, _) = b.lancer(&["zone", "--zone", "cog", "--arrete", &pi, "--ancre", &b.ancre]);
    assert!(!ok);
}

#[test]
fn regles_avec_coupure() {
    let b = Banc::nouveau("coupure");
    let pi = b.sceller("pi.arrete", &arrete_portail(1));
    let (ok, texte, err) =
        b.lancer(&["regles", "--frontiere", "portail-interne", "--arrete", &pi, "--ancre", &b.ancre, "--couper", "site"]);
    assert!(ok, "{err}");
    assert!(texte.contains("voies coupees : site"));
    assert!(!texte.contains("voie=site"));
    assert!(texte.contains("voie=jayoffice"));
    let (ok, _, err) =
        b.lancer(&["regles", "--frontiere", "portail-interne", "--arrete", &pi, "--ancre", &b.ancre, "--couper", "absente"]);
    assert!(!ok && err.contains("absente"), "{err}");
    let (ok, _, _) = b.lancer(&["fermeture", "--frontiere", "portail-interne", "--couper", "site"]);
    assert!(!ok);
}

#[test]
fn scripts_seuls() {
    let b = Banc::nouveau("scripts");
    let (ok, mise, _) = b.lancer(&["mise-en-place"]);
    assert!(ok);
    assert!(mise.starts_with("#!/bin/sh\n") && !mise.contains("[Service]"));
    assert!(mise.contains("ip netns add mcog-z-cog"));
    let (ok, retrait, _) = b.lancer(&["retrait"]);
    assert!(ok);
    assert!(retrait.contains("ip netns del mcog-z-cog"));
    assert!(!retrait.contains("netns add"));
}
