//! @id     miyuracine_tests_cli
//! @do     verifier_cli_ceremonie
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — outil de cérémonie : refus des arguments, fichiers existants et sources invalides (C-62).

//! Le binaire `miyuracine` est lancé tel quel (sous-processus), avec `--sans-mlock`
//! sauf pour le test du verrouillage mémoire. Exposant SLIP-39 0 pour la vitesse.

#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use miyuracine::{verifier_passation, Racine};

const BIN: &str = env!("CARGO_BIN_EXE_miyuracine");

fn dossier() -> PathBuf {
    static N: AtomicU32 = AtomicU32::new(0);
    let d = std::env::temp_dir().join(format!(
        "miyuracine-cli-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn lancer(args: &[&str]) -> Output {
    Command::new(BIN)
        .arg("--sans-mlock")
        .args(args)
        .output()
        .unwrap()
}

fn chemin(p: &Path) -> &str {
    p.to_str().unwrap()
}

/// Échec attendu, message d'erreur contenant `motif`.
fn refus(args: &[&str], motif: &str) {
    let o = lancer(args);
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(!o.status.success(), "succès inattendu pour {args:?}");
    assert!(
        err.contains(motif),
        "{args:?} : « {motif} » absent de : {err}"
    );
}

fn succes(args: &[&str]) -> String {
    let o = lancer(args);
    assert!(
        o.status.success(),
        "{args:?} : {}",
        String::from_utf8_lossy(&o.stderr)
    );
    String::from_utf8(o.stdout).unwrap()
}

fn cog_id(sortie: &str) -> String {
    sortie
        .lines()
        .find_map(|l| l.strip_prefix("cog_id : "))
        .unwrap()
        .to_string()
}

/// Génère trois parts (exposant 0) dans un dossier neuf ; rend (dossier, parts).
fn generer() -> (PathBuf, Vec<PathBuf>) {
    let d = dossier();
    succes(&["generer", "--dossier", chemin(&d), "--exposant", "0"]);
    let parts: Vec<PathBuf> = (1..=3)
        .map(|i| d.join(format!("part-{i}-sur-3.txt")))
        .collect();
    (d, parts)
}

fn racine_de(parts: &[PathBuf]) -> Racine {
    let a = fs::read_to_string(&parts[0]).unwrap();
    let b = fs::read_to_string(&parts[1]).unwrap();
    Racine::recuperer(&[a.trim_end(), b.trim_end()], b"").unwrap()
}

// ─── Arguments ─────────────────────────────────────────────────────────────

#[test]
fn aide_et_commandes_invalides() {
    assert!(succes(&["aide"]).contains("UTILISATION"));
    assert!(succes(&["--help"]).contains("UTILISATION"));
    refus(&[], "commande manquante");
    refus(&["detruire"], "commande inconnue");
    // Sans --sans-mlock non plus : commande manquante.
    let o = Command::new(BIN).output().unwrap();
    assert!(!o.status.success());
}

#[test]
fn options_refusees() {
    refus(&["generer", "--part", "x"], "option inattendue");
    refus(&["recuperer", "--dossier", "x"], "option inattendue");
    refus(&["generer", "--exposant"], "valeur manquante");
    refus(&["generer", "--exposant", "16"], "exposant invalide");
    refus(&["generer", "--exposant", "-1"], "exposant invalide");
    refus(
        &["generer", "--exposant", "1", "--exposant", "2"],
        "répétée",
    );
    refus(&["deriver", "--usage", "inconnu"], "usage inconnu");
    refus(
        &["deriver", "--exporter-graine-usage", "x"],
        "usage inconnu",
    );
    refus(&["deriver", "--generation", "-3"], "génération invalide");
    refus(
        &["exporter-cle", "--argon2-memoire-mio", "beaucoup"],
        "mémoire Argon2id invalide",
    );
    refus(
        &["exporter-cle", "--argon2-iterations", "x"],
        "itérations Argon2id invalides",
    );
}

// ─── Génération et récupération ────────────────────────────────────────────

#[test]
fn generer_ecrit_trois_parts_0600_sans_ecraser() {
    let (d, parts) = generer();
    for p in &parts {
        let mode = fs::metadata(p).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "{}", p.display());
    }
    // Deuxième génération dans le même dossier : refus, parts intactes.
    let avant = fs::read(&parts[0]).unwrap();
    refus(
        &["generer", "--dossier", chemin(&d), "--exposant", "0"],
        "refus d'écraser",
    );
    assert_eq!(fs::read(&parts[0]).unwrap(), avant);
    // Destination qui n'est pas un répertoire.
    refus(
        &["generer", "--dossier", chemin(&parts[0])],
        "n'est pas un répertoire",
    );
}

#[test]
fn generer_sur_la_sortie_standard() {
    let d = dossier();
    let phrase = d.join("phrase");
    fs::write(&phrase, "phrase de test\r\n").unwrap();
    let s = succes(&[
        "generer",
        "--exposant",
        "0",
        "--phrase-fichier",
        chemin(&phrase),
    ]);
    let parts: Vec<&str> = s
        .lines()
        .filter_map(|l| {
            l.split_once(" : ")
                .filter(|(t, _)| t.starts_with("part "))
                .map(|(_, p)| p)
        })
        .collect();
    assert_eq!(parts.len(), 3);
    // La phrase (sans fin de ligne) est requise pour retrouver la même identité.
    let r = Racine::recuperer(&[parts[0], parts[2]], b"phrase de test").unwrap();
    assert_eq!(r.cog_id(), cog_id(&s));
    let autre = Racine::recuperer(&[parts[0], parts[2]], b"").unwrap();
    assert_ne!(autre.cog_id(), cog_id(&s));
}

#[test]
fn recuperer_deux_parts_quelconques() {
    let (_d, parts) = generer();
    let a = succes(&[
        "recuperer",
        "--part",
        chemin(&parts[0]),
        "--part",
        chemin(&parts[1]),
    ]);
    let b = succes(&[
        "recuperer",
        "--part",
        chemin(&parts[2]),
        "--part",
        chemin(&parts[0]),
    ]);
    assert_eq!(cog_id(&a), cog_id(&b));
    assert_eq!(cog_id(&a), racine_de(&parts).cog_id());
    assert!(a.contains("emission"));
}

#[test]
fn recuperer_refuse_sources_invalides() {
    let (d, parts) = generer();
    let p = |i: usize| chemin(&parts[i]).to_string();
    refus(&["recuperer", "--part", &p(0)], "exactement deux --part");
    refus(
        &[
            "recuperer",
            "--part",
            &p(0),
            "--part",
            &p(1),
            "--part",
            &p(2),
        ],
        "exactement deux --part",
    );
    refus(
        &["recuperer", "--part", &p(0), "--part", &p(0)],
        "récupération refusée",
    );
    let absent = d.join("absent");
    refus(
        &["recuperer", "--part", &p(0), "--part", chemin(&absent)],
        "lecture de",
    );
    let gros = d.join("gros");
    fs::write(&gros, vec![b'a'; 5000]).unwrap();
    refus(
        &["recuperer", "--part", &p(0), "--part", chemin(&gros)],
        "fichier trop grand",
    );
    let altere = d.join("altere");
    let mut texte = fs::read_to_string(&parts[1]).unwrap();
    texte = texte.replacen(texte.split(' ').nth(5).unwrap(), "zzz", 1);
    fs::write(&altere, texte).unwrap();
    refus(
        &["recuperer", "--part", &p(0), "--part", chemin(&altere)],
        "récupération refusée",
    );
    // Part d'une autre racine : mélange refusé.
    let (_d2, autres) = generer();
    refus(
        &["recuperer", "--part", &p(0), "--part", chemin(&autres[1])],
        "récupération refusée",
    );
    // Fichier de clé absent ou invalide : refus dès la lecture des paramètres.
    refus(&["recuperer", "--cle", chemin(&absent)], "entrée-sortie");
    let faux = d.join("faux.myrc");
    fs::write(&faux, [0u8; 122]).unwrap();
    fs::set_permissions(&faux, fs::Permissions::from_mode(0o600)).unwrap();
    refus(
        &["recuperer", "--cle", chemin(&faux)],
        "fichier de clé invalide",
    );
}

// ─── Dérivation et export ──────────────────────────────────────────────────

#[test]
fn deriver_affiche_et_refuse() {
    let (d, parts) = generer();
    let p0 = chemin(&parts[0]).to_string();
    let p1 = chemin(&parts[1]).to_string();
    let base = ["deriver", "--part", &p0, "--part", &p1];
    let racine = racine_de(&parts);
    let cle = racine.cle_publique(miyuracine::Usage::Emission, 2).unwrap();
    let hex = cle.iter().fold(String::new(), |mut s, b| {
        use std::fmt::Write as _;
        let _ = write!(s, "{b:02x}");
        s
    });
    let s = succes(&[&base[..], &["--usage", "emission", "--generation", "2"]].concat());
    assert!(s.contains(&hex));
    // L'identité n'a qu'une génération.
    refus(
        &[&base[..], &["--usage", "identite", "--generation", "1"]].concat(),
        "génération invalide",
    );
    // Ni --usage ni export, ou les deux.
    refus(&base, "préciser exactement une option");
    refus(
        &[
            &base[..],
            &["--usage", "journal", "--exporter-graine-usage", "journal"],
        ]
        .concat(),
        "préciser exactement une option",
    );
    let sortie = d.join("graine.bin");
    refus(
        &[
            &base[..],
            &["--usage", "journal", "--sortie", chemin(&sortie)],
        ]
        .concat(),
        "--sortie n'a de sens",
    );
    refus(
        &[&base[..], &["--exporter-graine-usage", "journal"]].concat(),
        "--sortie <fichier> est requis",
    );
    refus(
        &[
            &base[..],
            &[
                "--exporter-graine-usage",
                "journal",
                "--sortie",
                chemin(&sortie),
            ],
        ]
        .concat(),
        "--generation est requis",
    );
    assert!(!sortie.exists());
}

#[test]
fn exporter_graine_usage_fichier_neuf_0600() {
    let (d, parts) = generer();
    let p0 = chemin(&parts[0]).to_string();
    let p1 = chemin(&parts[1]).to_string();
    let sortie = d.join("graine.bin");
    let args = [
        "deriver",
        "--part",
        &p0,
        "--part",
        &p1,
        "--exporter-graine-usage",
        "editeur",
        "--generation",
        "0",
        "--sortie",
        chemin(&sortie),
    ];
    succes(&args);
    let graine = fs::read(&sortie).unwrap();
    assert_eq!(graine.len(), 32);
    assert_eq!(
        graine,
        racine_de(&parts)
            .graine(miyuracine::Usage::Editeur, 0)
            .unwrap()
            .as_slice()
    );
    assert_eq!(
        fs::metadata(&sortie).unwrap().permissions().mode() & 0o777,
        0o600
    );
    // Fichier existant : refus, contenu intact.
    refus(&args, "existe déjà");
    assert_eq!(fs::read(&sortie).unwrap(), graine);
    // Lien symbolique pendant : refus aussi (pas de suivi de lien).
    let lien = d.join("lien");
    std::os::unix::fs::symlink(d.join("cible-absente"), &lien).unwrap();
    let mut a = args;
    a[10] = chemin(&lien);
    refus(&a, "existe déjà");
    assert!(!d.join("cible-absente").exists());
    // Identité : génération facultative, avertissement explicite.
    let id = d.join("identite.bin");
    let o = lancer(&[
        "deriver",
        "--part",
        &p0,
        "--part",
        &p1,
        "--exporter-graine-usage",
        "identite",
        "--sortie",
        chemin(&id),
    ]);
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("IDENTITÉ"));
}

// ─── Rotation ──────────────────────────────────────────────────────────────

#[test]
fn rotation_scelle_la_passation() {
    let (d, parts) = generer();
    let p0 = chemin(&parts[0]).to_string();
    let p1 = chemin(&parts[1]).to_string();
    let nouveau = d.join("nouveau");
    fs::create_dir(&nouveau).unwrap();
    let passation = d.join("passation.msce");
    let s = succes(&[
        "rotation",
        "--part",
        &p0,
        "--part",
        &p1,
        "--passation",
        chemin(&passation),
        "--dossier",
        chemin(&nouveau),
        "--exposant",
        "0",
    ]);
    assert!(s.contains("Passation"));
    let ancienne = racine_de(&parts);
    let nouvelles: Vec<PathBuf> = (1..=3)
        .map(|i| nouveau.join(format!("part-{i}-sur-3.txt")))
        .collect();
    let nouvelle = racine_de(&nouvelles);
    assert_ne!(nouvelle.cog_id(), ancienne.cog_id());
    let sceau = fs::read(&passation).unwrap();
    let maintenant = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let p = verifier_passation(&sceau, &ancienne.cle_publique_identite(), maintenant).unwrap();
    assert_eq!(p.nouvelle_racine, nouvelle.cle_publique_identite());
    // Le sceau ne se vérifie pas avec la nouvelle racine comme ancre.
    assert!(verifier_passation(&sceau, &nouvelle.cle_publique_identite(), maintenant).is_err());
    // Passation existante : refus avant toute génération.
    refus(
        &[
            "rotation",
            "--part",
            &p0,
            "--part",
            &p1,
            "--passation",
            chemin(&passation),
        ],
        "existe déjà",
    );
    refus(
        &["rotation", "--part", &p0, "--part", &p1],
        "--passation <fichier> est requis",
    );
    // Dossier des nouvelles parts déjà occupé : refus, aucune passation écrite.
    let autre = d.join("autre.msce");
    refus(
        &[
            "rotation",
            "--part",
            &p0,
            "--part",
            &p1,
            "--passation",
            chemin(&autre),
            "--dossier",
            chemin(&nouveau),
        ],
        "refus d'écraser",
    );
    assert!(!autre.exists());
}

// ─── Fichier de clé ────────────────────────────────────────────────────────

#[test]
fn exporter_cle_refuse_avant_tout_calcul() {
    let (d, parts) = generer();
    let p0 = chemin(&parts[0]).to_string();
    let p1 = chemin(&parts[1]).to_string();
    let sortie = d.join("racine.myrc");
    let mdp = d.join("mdp");
    fs::write(&mdp, "court\n").unwrap();
    let base = ["exporter-cle", "--part", &p0, "--part", &p1];
    refus(&base, "--sortie <fichier> est requis");
    refus(
        &[
            &base[..],
            &["--sortie", chemin(&sortie), "--argon2-memoire-mio", "1"],
        ]
        .concat(),
        "hors bornes",
    );
    refus(
        &[
            &base[..],
            &["--sortie", chemin(&sortie), "--argon2-iterations", "1"],
        ]
        .concat(),
        "hors bornes",
    );
    refus(
        &[
            &base[..],
            &[
                "--sortie",
                chemin(&sortie),
                "--argon2-memoire-mio",
                "4194304",
            ],
        ]
        .concat(),
        "hors bornes",
    );
    refus(
        &[&base[..], &["--sortie", chemin(&sortie)]].concat(),
        "--mot-de-passe-fichier <f> est requis",
    );
    refus(
        &[
            &base[..],
            &[
                "--sortie",
                chemin(&sortie),
                "--mot-de-passe-fichier",
                chemin(&mdp),
            ],
        ]
        .concat(),
        "mot de passe trop court",
    );
    assert!(!sortie.exists());
    fs::write(&sortie, b"occupe").unwrap();
    refus(
        &[&base[..], &["--sortie", chemin(&sortie)]].concat(),
        "existe déjà",
    );
    assert_eq!(fs::read(&sortie).unwrap(), b"occupe");
}

// ─── Durcissement ──────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
#[test]
fn sans_derogation_le_verrouillage_memoire_est_exige() {
    // Racine (CAP_IPC_LOCK) : le verrouillage réussit, rien à vérifier.
    if rustix::process::getuid().is_root() {
        return;
    }
    // RLIMIT_MEMLOCK nul : mlockall impossible ⇒ refus avant toute lecture.
    let o = Command::new("sh")
        .arg("-c")
        .arg(format!("ulimit -l 0 && exec '{BIN}' generer --exposant 0"))
        .output()
        .unwrap();
    assert!(!o.status.success());
    assert!(o.stdout.is_empty(), "aucune part ne doit sortir");
    assert!(String::from_utf8_lossy(&o.stderr).contains("mémoire non verrouillable"));
}

#[test]
fn exporter_cle_puis_relire_par_cle() {
    let (d, parts) = generer();
    let p0 = chemin(&parts[0]).to_string();
    let p1 = chemin(&parts[1]).to_string();
    let sortie = d.join("racine.myrc");
    let mdp = d.join("mdp");
    fs::write(&mdp, "mot de passe de cérémonie\n").unwrap();
    let s = succes(&[
        "exporter-cle",
        "--part",
        &p0,
        "--part",
        &p1,
        "--sortie",
        chemin(&sortie),
        "--mot-de-passe-fichier",
        chemin(&mdp),
        "--argon2-memoire-mio",
        "256",
        "--argon2-iterations",
        "3",
    ]);
    assert!(s.contains("écrit et vérifié"));
    assert_eq!(fs::metadata(&sortie).unwrap().len(), 122);
    assert_eq!(
        fs::metadata(&sortie).unwrap().permissions().mode() & 0o777,
        0o600
    );
    let attendu = racine_de(&parts).cog_id();
    let r = succes(&[
        "recuperer",
        "--cle",
        chemin(&sortie),
        "--mot-de-passe-fichier",
        chemin(&mdp),
    ]);
    assert_eq!(cog_id(&r), attendu);
    // --cle exclut --part ; mot de passe requis (mauvais mot de passe : tests unitaires).
    refus(
        &["recuperer", "--cle", chemin(&sortie), "--part", &p0],
        "--cle exclut --part",
    );
    refus(
        &["recuperer", "--cle", chemin(&sortie)],
        "--mot-de-passe-fichier <f> est requis",
    );
    // Droits trop larges : refus avant lecture.
    fs::set_permissions(&sortie, fs::Permissions::from_mode(0o644)).unwrap();
    refus(
        &[
            "recuperer",
            "--cle",
            chemin(&sortie),
            "--mot-de-passe-fichier",
            chemin(&mdp),
        ],
        "refusé",
    );
}
