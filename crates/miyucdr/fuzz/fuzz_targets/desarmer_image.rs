//! @id     miyucdr_fuzz_desarmer_image
//! @do     fuzzer_desarmement_images
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — désarmement et reconstruction d'images PNG, JPEG, GIF, WebP, BMP sur octets quelconques (EX-10, EX-45).

//! Entrée : `type (1) | octets`, type = PNG, JPEG, GIF, WebP ou BMP.
//! Propriétés : voir [`commun`] ; la sortie re-désarmée est acceptée.

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;

const TYPES: [&str; 5] = ["image/png", "image/jpeg", "image/gif", "image/webp", "image/bmp"];

fuzz_target!(|entree: &[u8]| {
    let Some((&t, octets)) = entree.split_first() else { return };
    let mime = TYPES[usize::from(t) % TYPES.len()];
    if let Some((type_reel, sortie)) = commun::desarmer_et_controler(mime, octets) {
        commun::reaccepter(type_reel, &sortie);
    }
});
