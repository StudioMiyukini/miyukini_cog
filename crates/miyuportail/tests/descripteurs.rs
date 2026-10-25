//! @id     miyuportail_tests_descripteurs
//! @do     verifier_refus_descripteurs_transmissibles
//! @role   test
//! @layer  toolkit
//! @human  miyuportail — M1 : le parent refuse de lancer le sas si un descripteur > 2 est transmissible (sans FD_CLOEXEC).

//! Binaire de test dédié : retirer `FD_CLOEXEC` touche tout le processus.

#![cfg(target_os = "linux")]

mod commun;

use commun::*;
use miyuportail::*;
use rustix::io::{fcntl_getfd, fcntl_setfd, FdFlags};

#[test]
fn descripteur_sans_cloexec_refuse_le_lancement() {
    let m = monde(Box::new(sas_reel()), Box::new(RefusParDefaut));
    let s = m.voie_ouverte("docs");

    // Témoin : processus propre, le sas réel décode.
    m.portail.deposer(&s, m.trame(&s, 1)).unwrap();
    let r = m.portail.relever(&s, 1).unwrap();
    assert!(r[0].is_ok(), "{r:?}");

    // Un descripteur ouvert sans FD_CLOEXEC : lancement refusé.
    let fichier = std::fs::File::open("/dev/null").unwrap();
    fcntl_setfd(&fichier, FdFlags::empty()).unwrap();
    assert!(!fcntl_getfd(&fichier).unwrap().contains(FdFlags::CLOEXEC));
    m.portail.deposer(&s, m.trame(&s, 2)).unwrap();
    let r = m.portail.relever(&s, 1).unwrap();
    assert_eq!(r, vec![Err(ErreurPortail::Sas(ErreurSas::DescripteurHerite))]);

    // FD_CLOEXEC rétabli : le sas repart.
    fcntl_setfd(&fichier, FdFlags::CLOEXEC).unwrap();
    m.portail.deposer(&s, m.trame(&s, 3)).unwrap();
    let r = m.portail.relever(&s, 1).unwrap();
    assert!(r[0].is_ok(), "{r:?}");
}
