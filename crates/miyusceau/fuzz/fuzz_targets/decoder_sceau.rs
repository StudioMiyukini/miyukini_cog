//! @id     miyusceau_fuzz_decoder_sceau
//! @do     fuzzer_decodage_sceau
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage strict d'un sceau quelconque (EX-10).

//! Propriétés : aucune panique ; tout sceau accepté par le décodeur est
//! canonique (ré-encodage identique aux octets d'entrée).

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyusceau::Sceau;

fuzz_target!(|octets: &[u8]| {
    if let Ok(sceau) = Sceau::decoder(octets) {
        assert_eq!(sceau.encoder(), octets, "décodage non canonique");
        let _ = sceau.octets_signes();
    }
});
