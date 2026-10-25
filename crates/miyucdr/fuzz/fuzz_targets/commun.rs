//! @id     miyucdr_fuzz_commun
//! @do     outiller_cibles_fuzzing_cdr
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles du CDR : limites bornées, contrôle de la sortie reconstruite (EX-10, EX-45).

//! Propriétés vérifiées sur **chaque** désarmement :
//! - aucune panique (le crochet de libFuzzer interrompt aussi le fil de décodage) ;
//! - jamais de refus `Interne` (fil interrompu), `Sature` ni `DelaiDepasse`
//!   sous des limites bornées : un tel refus signale une panique confinée ou
//!   une entrée lente ;
//! - sortie admise : type réel conforme au type déclaré (même type pour les
//!   textes ; PNG, ou JPEG pour une source JPEG, pour les images), type
//!   **détecté** de la sortie égal à son type réel, tailles et dimensions dans
//!   les limites, et la sortie est elle-même acceptée par le CDR.

#![allow(dead_code)]

use std::time::Duration;

use miyucdr::{desarmer, detecter, Limites, Refus, TypeContenu, TypeDetecte};

pub fn limites() -> Limites {
    Limites {
        octets_max: 256 * 1024,
        texte_octets_max: 64 * 1024,
        cote_max: 2_048,
        pixels_max: 1_000_000,
        allocation_max: 64 * 1024 * 1024,
        delai_max: Duration::from_secs(20),
        json_profondeur_max: 64,
        csv_enregistrements_max: 10_000,
        csv_champs_max: 256,
        normaliser_nfc: true,
        qualite_jpeg: 90,
    }
}

fn detection_attendue(t: TypeContenu) -> TypeDetecte {
    match t {
        TypeContenu::Png => TypeDetecte::Png,
        TypeContenu::Jpeg => TypeDetecte::Jpeg,
        TypeContenu::Gif => TypeDetecte::Gif,
        TypeContenu::Webp => TypeDetecte::Webp,
        TypeContenu::Bmp => TypeDetecte::Bmp,
        TypeContenu::TexteBrut | TypeContenu::Csv | TypeContenu::Markdown | TypeContenu::Json => TypeDetecte::Texte,
    }
}

/// Désarme et contrôle ; renvoie la sortie admise.
pub fn desarmer_et_controler(mime: &str, octets: &[u8]) -> Option<(TypeContenu, Vec<u8>)> {
    let limites = limites();
    let r = desarmer(mime, octets, &limites);
    let r = match r {
        Ok(r) => r,
        Err(e @ (Refus::Interne(_) | Refus::Sature | Refus::DelaiDepasse)) => {
            panic!("refus anormal du CDR : {e:?}")
        }
        Err(_) => return None,
    };
    let declare = TypeContenu::depuis_mime(mime).expect("type admis : MIME reconnu");
    if declare.est_image() {
        assert!(r.type_reel.est_image(), "image reconstruite en {:?}", r.type_reel);
        assert!(
            r.type_reel == TypeContenu::Png || (r.type_reel == TypeContenu::Jpeg && declare == TypeContenu::Jpeg),
            "sortie {:?} pour une source {declare:?}",
            r.type_reel
        );
        let (l, h) = r.rapport.dimensions.expect("dimensions d'une image reconstruite");
        assert!(l >= 1 && h >= 1 && l <= limites.cote_max && h <= limites.cote_max);
        assert!(u64::from(l) * u64::from(h) <= limites.pixels_max);
    } else {
        assert_eq!(r.type_reel, declare, "type réel d'un texte modifié");
        assert!(r.octets.len() <= limites.texte_octets_max);
        let texte = std::str::from_utf8(&r.octets).expect("texte reconstruit en UTF-8");
        assert!(
            !texte.chars().any(|c| c.is_control() && c != '\n' && c != '\t'),
            "caractère de contrôle dans la sortie : {texte:?}"
        );
    }
    assert_eq!(r.rapport.octets_sortie, r.octets.len());
    assert_eq!(r.rapport.octets_entree, octets.len());
    if !r.octets.is_empty() {
        assert_eq!(detecter(&r.octets), detection_attendue(r.type_reel), "sortie mal détectée");
    }
    Some((r.type_reel, r.octets))
}

/// La sortie reconstruite doit être acceptée à son tour (même type réel).
pub fn reaccepter(type_reel: TypeContenu, sortie: &[u8]) -> Vec<u8> {
    let (t, o) = desarmer_et_controler(type_reel.mime(), sortie)
        .unwrap_or_else(|| panic!("sortie reconstruite refusée par le CDR ({type_reel:?}, {} octets)", sortie.len()));
    assert_eq!(t, type_reel, "re-désarmement : type réel modifié");
    o
}
