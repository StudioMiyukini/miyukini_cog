//! @id     miyusceau_fuzz_decoder_delegation
//! @do     fuzzer_decodage_delegation
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage de la délégation de clé d'usage (certificat 0x06, nature 0x45, A-022 §2).

//! Entrée quelconque traitée (1) comme charge TLV d'une délégation, (2) comme
//! sceau complet vérifié contre une identité épinglée fixe. Propriétés :
//! aucune panique ; toute délégation décodée satisfait les invariants du
//! constructeur, se ré-encode à l'identique (aller-retour exact) ; aucune
//! délégation n'est acceptée sans signature de l'identité épinglée.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyusceau::{
    decoder_titre, verifier_delegation, CleSignature, DelegationCle, EcrivainTlv, NiveauConfiance,
    Titre, DUREE_MAX_DELEGATION_SECS,
};

fuzz_target!(|donnees: &[u8]| {
    if let Ok(d) = decoder_titre::<DelegationCle>(donnees) {
        let mut e = EcrivainTlv::new();
        d.encoder(&mut e).expect("délégation décodée ré-encodable");
        assert_eq!(e.finir(), donnees, "délégation non canonique");
        assert!(d.indice() >= 1 && u64::from(d.indice()) < 1 << 31);
        assert!(u64::from(d.generation()) < 1 << 31);
        assert!(
            d.pas_avant() < d.pas_apres()
                && d.pas_apres() - d.pas_avant() <= DUREE_MAX_DELEGATION_SECS
        );
        let refaite = DelegationCle::nouvelle(
            d.cog_id(),
            d.indice(),
            d.generation(),
            *d.cle_publique(),
            d.pas_avant(),
            d.pas_apres(),
        )
        .expect("le constructeur accepte ce que le décodeur accepte");
        assert_eq!(refaite, d);
    }
    // Sceau quelconque : l'identité épinglée (graine fixe) n'a rien signé.
    let identite = CleSignature::depuis_graine(&[0x5a; 32]).cle_publique();
    let maintenant = donnees
        .get(..8)
        .map_or(0, |o| u64::from_be_bytes(o.try_into().unwrap_or([0; 8])));
    assert!(
        verifier_delegation(donnees, &identite, NiveauConfiance::Souverain, maintenant).is_err()
    );
});
