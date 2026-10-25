//! @id     miyuracine_tests_durcissement
//! @do     verifier_durcissement_processus
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — core dumps désactivés et politique de verrouillage mémoire (NC-ANSSI-25).

//! Binaire de test dédié : le durcissement modifie le processus entier.

#![cfg(target_os = "linux")]

use miyuracine::{durcir_processus, ErreurRacine, PolitiqueVerrouillage};
use rustix::process::{dumpable_behavior, getrlimit, DumpableBehavior, Resource};

#[test]
fn durcissement_core_dumps_et_politique_mlock() {
    assert_eq!(
        PolitiqueVerrouillage::default(),
        PolitiqueVerrouillage::Exiger
    );

    // Exiger avec un budget impossible : refus fail-closed (si la limite est finie).
    let limite = getrlimit(Resource::Memlock);
    if limite.maximum.is_some() {
        assert_eq!(
            durcir_processus(PolitiqueVerrouillage::Exiger, u64::MAX / 2).unwrap_err(),
            ErreurRacine::VerrouillageMemoire
        );
    }

    // Avertir : pas de verrouillage, core dumps toujours désactivés.
    let etat = durcir_processus(PolitiqueVerrouillage::Avertir, 0).unwrap();
    assert!(etat.core_dumps_desactives);
    assert!(!etat.memoire_verrouillee);
    assert_eq!(dumpable_behavior().unwrap(), DumpableBehavior::NotDumpable);
    let core = getrlimit(Resource::Core);
    assert_eq!(core.current, Some(0));
    assert_eq!(core.maximum, Some(0));
}
