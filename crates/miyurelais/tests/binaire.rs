//! @id     miyurelais_test_binaire
//! @do     verifier_refus_du_binaire_et_de_la_socket
//! @role   test
//! @layer  operateur
//! @human  Binaire miyurelais lancé en sous-processus : arguments refusés (code 2), ouverture NFLOG refusée sans CAP_NET_ADMIN (code 3) ; SocketNflog::ouvrir sans droit : refus du noyau rendu, jamais une socket à moitié liée.

#![cfg(target_os = "linux")]
#![allow(clippy::unwrap_used)]

use std::process::{Command, Output};

use miyurelais::{ParametresInstance, SocketNflog};

fn lancer(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_miyurelais")).args(args).output().unwrap()
}

fn erreur(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

/// Vrai si le processus a le droit de lier un groupe NFLOG (essai réel).
fn privilegie() -> bool {
    SocketNflog::ouvrir(&[], ParametresInstance::default()).is_ok()
        && SocketNflog::ouvrir(&[65_000], ParametresInstance::default()).is_ok()
}

#[test]
fn arguments_refuses_code_2() {
    let cas: &[(&[&str], &str)] = &[
        (&[], "obligatoires"),
        (&["--hote", "h", "--sans-veille"], "obligatoires"),
        (&["--groupe", "110", "--sans-veille"], "obligatoires"),
        (&["--groupe", "110", "--hote", "h"], "explicitement"),
        (&["--groupe", "110", "--hote", "h", "--voie-active", "--sans-veille"], "explicitement"),
        (&["--groupe"], "valeur manquante après --groupe"),
        (&["--groupe", "x"], "groupe invalide"),
        (&["--groupe", "70000"], "groupe invalide"),
        (&["--groupe", "1", "--copie", "-1"], "copie invalide"),
        (&["--groupe", "1", "--delai-muet-s", "dix"], "délai invalide"),
        (&["--groupe", "1", "--releve-ms", ""], "période invalide"),
        (&["--groupe", "1", "--duree-s", "1.5"], "durée invalide"),
        (&["--groupe", "1", "--hote", "h", "--sans-veille", "--inconnu"], "argument inconnu : --inconnu"),
    ];
    for (args, motif) in cas {
        let o = lancer(args);
        assert_eq!(o.status.code(), Some(2), "{args:?} : {}", erreur(&o));
        assert!(erreur(&o).contains(motif), "{args:?} : {}", erreur(&o));
        assert!(o.stdout.is_empty(), "{args:?} : aucune ligne sur la sortie standard");
    }
}

/// Hôte invalide : refusé par la configuration du relais (code 2), après
/// l'ouverture de la socket (code 3 si celle-ci est refusée d'abord).
#[test]
fn hote_invalide_jamais_de_sortie() {
    let o = lancer(&["--groupe", "65001", "--hote", "a b", "--sans-veille", "--duree-s", "0"]);
    let code = o.status.code();
    assert!(code == Some(2) || code == Some(3), "{code:?} : {}", erreur(&o));
    assert!(o.stdout.is_empty());
    if code == Some(2) {
        assert!(erreur(&o).contains("nom d'hôte invalide"), "{}", erreur(&o));
    }
}

/// Sans `CAP_NET_ADMIN` : l'ouverture NFLOG est refusée, code 3, rien sur la
/// sortie standard. Avec le droit : fin normale après `--duree-s 0`.
#[test]
fn ouverture_refusee_code_3_ou_fin_normale() {
    let o = lancer(&["--groupe", "65002", "--hote", "essai", "--sans-veille", "--duree-s", "0", "--releve-ms", "1"]);
    if privilegie() {
        assert_eq!(o.status.code(), Some(0), "{}", erreur(&o));
        assert!(erreur(&o).contains("STATS datagrammes="), "{}", erreur(&o));
    } else {
        assert_eq!(o.status.code(), Some(3), "{}", erreur(&o));
        let e = erreur(&o);
        assert!(e.contains("ouverture NFLOG") && e.contains("groupe 65002"), "{e}");
        assert!(o.stdout.is_empty());
    }
}

/// `SocketNflog::ouvrir` sans droit : le refus du noyau (`EPERM`) est rendu
/// avec le groupe en cause ; aucune liste vide ne passe pour liée.
#[test]
fn socket_refusee_sans_droit() {
    // Aucun groupe : socket ouverte, rien de lié, tampon relevé.
    let s = SocketNflog::ouvrir(&[], ParametresInstance::default()).unwrap();
    assert!(s.groupes().is_empty());
    let t = s.tampon_effectif();
    assert!(t.effectif > 0);
    match SocketNflog::ouvrir(&[65_003], ParametresInstance::default()) {
        Ok(s) => assert_eq!(s.groupes(), &[65_003], "lié : le groupe demandé, lui seul"),
        Err(e) => {
            let m = e.to_string();
            assert!(m.starts_with("ouverture NFLOG : liaison au groupe 65003 : "), "{m}");
            assert!(m.contains("refus du noyau") || m.contains("acquittement"), "{m}");
            assert!(!t.force, "SO_RCVBUFFORCE exige CAP_NET_ADMIN");
        }
    }
}
