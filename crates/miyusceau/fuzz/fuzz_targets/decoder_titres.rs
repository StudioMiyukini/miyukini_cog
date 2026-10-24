//! @id     miyusceau_fuzz_decoder_titres
//! @do     fuzzer_decodage_titres
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage canonique de chaque type de titre de miyusceau (EX-10).

//! Charge TLV quelconque décodée en titre (seul `TeteRegistre` est défini
//! dans miyusceau ; les autres titres vivent dans leurs crates), puis état
//! persistant du registre. Propriétés : aucune panique ; tout titre accepté
//! est canonique (ré-encodage identique) ; un état n'est jamais accepté
//! sans ancre.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyusceau::{decoder_titre, AncresDeConfiance, EcrivainTlv, EtatRegistre, PolitiqueTete, TeteRegistre, Titre};

fuzz_target!(|charge: &[u8]| {
    if let Ok(tete) = decoder_titre::<TeteRegistre>(charge) {
        let mut e = EcrivainTlv::new();
        tete.encoder(&mut e).expect("titre décodé ré-encodable");
        assert_eq!(e.finir(), charge, "titre non canonique");
    }
    assert!(EtatRegistre::decoder(charge, &AncresDeConfiance::new(), &PolitiqueTete::journal_cog(), &[0u8; 32]).is_err());
});
