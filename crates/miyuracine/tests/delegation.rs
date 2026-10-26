//! @id     miyuracine_tests_delegation
//! @do     verifier_delegations_racine
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — délégations 0x45 des clés d'usage signées par l'identité (A-022 §2).

use miyuracine::{ErreurRacine, Racine, Usage};
use miyusceau::{verifier_delegation, ErreurSceau, NiveauConfiance};

const T0: u64 = 1_800_000_000;
const JOUR: u64 = 86_400;

#[test]
fn delegation_de_chaque_usage_certifie_la_cle_derivee() {
    let racine = Racine::depuis_secret(&[0x42; 32]);
    let identite = racine.cle_publique_identite();
    for usage in Usage::TOUS.into_iter().filter(|u| u.a_generations()) {
        let sceau = racine.deleguer(usage, 2, T0, T0 + 90 * JOUR).unwrap();
        let d = verifier_delegation(&sceau, &identite, NiveauConfiance::Souverain, T0 + JOUR)
            .unwrap()
            .into_titre();
        assert_eq!(d.indice(), usage.indice());
        assert_eq!(d.generation(), 2);
        assert_eq!(d.cle_publique(), &racine.cle_publique(usage, 2).unwrap());
        assert_eq!(d.cog_id(), racine.cog_id());
    }
}

#[test]
fn refus_identite_duree_et_autre_racine() {
    let racine = Racine::depuis_secret(&[0x42; 32]);
    assert_eq!(
        racine
            .deleguer(Usage::Identite, 0, T0, T0 + JOUR)
            .unwrap_err(),
        ErreurRacine::DelegationIdentite
    );
    assert_eq!(
        racine
            .deleguer(Usage::Visas, 0, T0, T0 + 90 * JOUR + 1)
            .unwrap_err(),
        ErreurRacine::Sceau(ErreurSceau::ValeurInvalide(7))
    );
    assert!(racine.deleguer(Usage::Visas, 0, T0, T0).is_err());
    // Une délégation d'une racine ne se vérifie pas contre une autre identité.
    let autre = Racine::depuis_secret(&[0x43; 32]);
    let sceau = racine.deleguer(Usage::Visas, 0, T0, T0 + JOUR).unwrap();
    assert!(verifier_delegation(
        &sceau,
        &autre.cle_publique_identite(),
        NiveauConfiance::Souverain,
        T0
    )
    .is_err());
}
