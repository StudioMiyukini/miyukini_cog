//! @id     maia_federee_fuzz_fiche
//! @do     fuzzer_fiche_sceau_0x09
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — Fiche fédérée (titre Sceau 0x09, reprises tag 0x0D) : décodage canonique, vérification, normalisation (EX-57, A-009, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : charge TLV brute → titre `Fiche` ;
//! - 1 : sceau brut → `verifier_fiche` ;
//! - autre : charge brute décodée en `Fiche`, émetteur remplacé par la clé
//!   fixe, scellée par celle-ci → `verifier_fiche`.
//!
//! Propriétés : aucune panique ; titre canonique (ré-encodage identique) ;
//! une Fiche vérifiée est signée par son émetteur, son `cog_id` est celui de
//! la clé, son empreinte celle des octets signés, ses champs normalisés et
//! bornés (texte ≤ 8 000 caractères, ≤ 16 reprises aux identifiants
//! valides) ; une Fiche bien formée signée par son émetteur n'est refusée
//! que pour sa date ou sa normalisation.

#![no_main]

#[path = "commun.rs"]
mod commun;

use commun::{cle, MAINTENANT};
use libfuzzer_sys::fuzz_target;
use maia_federee::fiche::{MAX_RELATIONS, MAX_REPRISES, MAX_SOURCES, MAX_TEXTE};
use maia_federee::normalisation::{normaliser, normaliser_ligne};
use maia_federee::{cog_de_cle, sceller_fiche, verifier_fiche, ErreurFederee, Fiche, FicheVerifiee};
use miyusceau::{decoder_titre, EcrivainTlv, Sceau, Titre};
use sha2::{Digest, Sha256};

fn controler_verifiee(v: &FicheVerifiee, octets: &[u8]) {
    let f = v.fiche();
    assert_eq!(Some(v.cog()), cog_de_cle(&f.emetteur).as_ref(), "cog_id ≠ clé de l'émetteur");
    let signes = Sceau::decoder(octets).expect("sceau").octets_signes();
    assert_eq!(v.empreinte(), <[u8; 32]>::from(Sha256::digest(signes)));
    assert!(!f.texte.is_empty() && f.texte.chars().count() <= MAX_TEXTE);
    assert!(!f.sujet.is_empty() && !f.sujet.contains('\n'));
    assert_eq!(normaliser_ligne(&f.sujet), f.sujet, "sujet non normalisé");
    assert_eq!(normaliser(&f.texte), f.texte, "texte non normalisé");
    assert!(f.relations.len() <= MAX_RELATIONS && f.sources.len() <= MAX_SOURCES);
    for r in &f.relations {
        for t in [&r.sujet, &r.predicat, &r.objet] {
            assert!(!t.is_empty() && !t.contains('\n'));
        }
    }
    assert!(f.reprises.len() <= MAX_REPRISES);
    assert_eq!(v.est_reprise(), !f.reprises.is_empty());
    assert!(f.reprises.iter().all(|r| r.version >= 1 && !r.id.is_empty()));
    assert!(f.date <= MAINTENANT + miyusceau::TOLERANCE_HORLOGE_SECS);
}

fn canonique(charge: &[u8]) -> Option<Fiche> {
    let f = decoder_titre::<Fiche>(charge).ok()?;
    let mut e = EcrivainTlv::new();
    f.encoder(&mut e).expect("Fiche décodée ré-encodable");
    assert_eq!(e.finir(), charge, "Fiche non canonique");
    Some(f)
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    match mode {
        0 => {
            canonique(reste);
        }
        1 => {
            if let Ok(v) = verifier_fiche(reste, MAINTENANT) {
                controler_verifiee(&v, reste);
            }
        }
        _ => {
            let Some(mut f) = canonique(reste) else { return };
            f.emetteur = cle().cle_publique();
            let Ok(sceau) = sceller_fiche(&f, &cle()) else { return };
            match verifier_fiche(&sceau, MAINTENANT) {
                Ok(v) => controler_verifiee(&v, &sceau),
                Err(ErreurFederee::DateFuture) => assert!(f.date > MAINTENANT),
                Err(ErreurFederee::ChampInvalide(_)) => {}
                Err(e) => panic!("Fiche bien formée signée par son émetteur refusée : {e:?}"),
            }
        }
    }
});
