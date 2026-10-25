//! @id     jayrdv_facade_tests_architecture
//! @do     verifier_absence_chemin_direct
//! @role   test
//! @layer  service
//! @human  jayrdv-facade — règle d'architecture : aucune dépendance (même transitive) vers le moteur, KindMother ou le Central ; aucune poignée du côté zone COG dans les sources.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

/// Paquets que la façade ne doit jamais atteindre (moteur, données, Central).
fn interdit(nom: &str) -> bool {
    nom.starts_with("kindmother")
        || nom == "jayrdv"
        || nom == "jayrdv-client"
        || nom == "miyukini-central"
        || nom == "central"
        || nom == "miyusql"
}

#[test]
fn aucune_dependance_vers_moteur_kindmother_ou_central() {
    let manifeste = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let sortie = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--offline", "--manifest-path"])
        .arg(&manifeste)
        .output()
        .expect("cargo metadata");
    assert!(sortie.status.success(), "{}", String::from_utf8_lossy(&sortie.stderr));
    let meta: serde_json::Value = serde_json::from_slice(&sortie.stdout).expect("json");

    let noms: BTreeMap<&str, &str> = meta["packages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| (p["id"].as_str().unwrap(), p["name"].as_str().unwrap()))
        .collect();
    let racine = noms.iter().find(|(_, n)| **n == "jayrdv-facade").map(|(id, _)| *id).expect("jayrdv-facade");
    let noeuds: BTreeMap<&str, &serde_json::Value> =
        meta["resolve"]["nodes"].as_array().unwrap().iter().map(|n| (n["id"].as_str().unwrap(), n)).collect();

    // Fermeture des dépendances normales et de construction (hors dev).
    let mut vus = BTreeSet::new();
    let mut pile = vec![racine];
    while let Some(id) = pile.pop() {
        if !vus.insert(id) {
            continue;
        }
        for dep in noeuds[id]["deps"].as_array().unwrap() {
            let normale = dep["dep_kinds"].as_array().unwrap().iter().any(|k| k["kind"].as_str() != Some("dev"));
            if normale {
                pile.push(dep["pkg"].as_str().unwrap());
            }
        }
    }
    let atteints: Vec<&str> = vus.iter().map(|id| noms[id]).collect();
    let fautifs: Vec<&&str> = atteints.iter().filter(|n| interdit(n)).collect();
    assert!(fautifs.is_empty(), "chemin direct interdit vers : {fautifs:?}");
    // Le seul lien vers la zone COG est le Portail (via miyuhybride).
    assert!(atteints.contains(&"miyuhybride") && atteints.contains(&"miyuportail"));
}

#[test]
fn sources_sans_poignee_du_cote_zone_cog() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for entree in std::fs::read_dir(src).unwrap() {
        let chemin = entree.unwrap().path();
        let texte = std::fs::read_to_string(&chemin).unwrap();
        for motif in
            ["GuichetMoteur", "PortailInterne", "ServiceHybride", "Superviseur", ".relever(", ".publier(", "kindmother"]
        {
            let code: String =
                texte.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
            assert!(!code.contains(motif), "{} utilise « {motif} »", chemin.display());
        }
    }
}
