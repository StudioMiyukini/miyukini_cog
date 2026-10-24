//! @id     miyuracine_tests_hors_reseau
//! @do     verifier_absence_reseau
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — garde « hors ligne » (aucune dépendance ni API réseau).

//! La racine se manipule hors ligne : ce crate ne doit dépendre d'aucune pile
//! réseau et ne doit pas utiliser `std::net`.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

/// Crates de réseau interdits dans le graphe de dépendances normales.
const INTERDITS: &[&str] = &[
    "tokio",
    "mio",
    "socket2",
    "hyper",
    "reqwest",
    "ureq",
    "rustls",
    "native-tls",
    "openssl",
    "curl",
    "h2",
    "tungstenite",
    "async-std",
    "smol",
    "surf",
    "isahc",
    "attohttpc",
    "quinn",
    "libp2p",
    "tonic",
    "axum",
];

/// Graphe `nom → dépendances` lu dans `Cargo.lock` (toutes versions et toutes
/// cibles confondues, dépendances de développement comprises : sur-ensemble
/// prudent du graphe réel, sans téléchargement).
fn graphe_verrou() -> HashMap<String, BTreeSet<String>> {
    let verrou = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.lock");
    let texte = std::fs::read_to_string(&verrou).expect("Cargo.lock du workspace");
    let mut graphe: HashMap<String, BTreeSet<String>> = HashMap::new();
    let mut courant: Option<String> = None;
    let mut dans_dependances = false;
    for ligne in texte.lines().map(str::trim) {
        if ligne == "[[package]]" {
            courant = None;
            dans_dependances = false;
        } else if let Some(nom) = ligne
            .strip_prefix("name = \"")
            .and_then(|r| r.strip_suffix('"'))
        {
            graphe.entry(nom.to_string()).or_default();
            courant = Some(nom.to_string());
        } else if ligne == "dependencies = [" {
            dans_dependances = true;
        } else if dans_dependances && ligne == "]" {
            dans_dependances = false;
        } else if dans_dependances {
            let dep = ligne.trim_matches(|c| c == '"' || c == ',');
            let nom = dep.split(' ').next().unwrap_or_default().to_string();
            if let Some(c) = &courant {
                graphe.entry(c.clone()).or_default().insert(nom);
            }
        }
    }
    graphe
}

#[test]
fn aucune_dependance_reseau() {
    let graphe = graphe_verrou();
    let mut vus = BTreeSet::new();
    let mut pile = vec!["miyuracine".to_string()];
    while let Some(nom) = pile.pop() {
        if vus.insert(nom.clone()) {
            pile.extend(graphe.get(&nom).into_iter().flatten().cloned());
        }
    }
    for interdit in INTERDITS {
        assert!(
            !vus.contains(*interdit),
            "dépendance réseau interdite : {interdit}"
        );
    }
    assert!(
        vus.contains("miyusceau") && vus.contains("pbkdf2"),
        "graphe lu : {vus:?}"
    );
}

#[test]
fn aucune_api_reseau_dans_les_sources() {
    let mut fichiers = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("src")];
    let mut rs = Vec::new();
    while let Some(chemin) = fichiers.pop() {
        for entree in std::fs::read_dir(&chemin).unwrap() {
            let p = entree.unwrap().path();
            if p.is_dir() {
                fichiers.push(p);
            } else if p.extension().is_some_and(|e| e == "rs") {
                rs.push(p);
            }
        }
    }
    assert!(rs.len() >= 10);
    for f in rs {
        let texte = std::fs::read_to_string(&f).unwrap();
        for motif in [
            "std::net",
            "TcpStream",
            "TcpListener",
            "UdpSocket",
            "ToSocketAddrs",
        ] {
            assert!(!texte.contains(motif), "{} utilise {motif}", f.display());
        }
    }
}
