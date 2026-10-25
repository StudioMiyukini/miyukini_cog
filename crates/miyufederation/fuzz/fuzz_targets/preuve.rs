//! @id     miyufederation_fuzz_preuve
//! @do     fuzzer_preuve_faute_mfpv
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — preuves de faute `MFPV` (équivoque de têtes signées) : décodage canonique et vérification (EX-32, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : preuve brute → `Preuve::decoder`, puis `verifier` contre le journal
//!   du COG 0 ;
//! - autre : deux sceaux quelconques (`reste` coupé selon son 1er octet)
//!   assemblés par `Preuve::equivoque`, puis encodés et relus.
//!
//! Propriétés : aucune panique ; décodage canonique (ré-encodage identique,
//! sceaux rangés) ; empreinte = SHA-256 de l'encodage, indépendante de
//! l'ordre des sceaux ; une preuve ne vérifie que sur des têtes signées par
//! le journal du mis en cause, de son registre.

#![no_main]

#[path = "commun.rs"]
mod commun;

use commun::{cogs, MAINTENANT};
use libfuzzer_sys::fuzz_target;
use miyufederation::Preuve;
use miyusceau::{decoder_titre, verifier_seuil, AncresDeConfiance, NiveauConfiance, TeteRegistre};

fn controler(octets: &[u8]) {
    let Ok(p) = Preuve::decoder(octets) else { return };
    assert_eq!(p.encoder(), octets, "preuve non canonique");
    let Preuve::Equivoque { premiere, seconde } = &p;
    assert!(premiere <= seconde, "sceaux non rangés");
    assert_eq!(Preuve::equivoque(seconde.clone(), premiere.clone()), p);
    let c = &cogs()[0];
    if p.verifier(&c.journal.cle_publique(), &c.registre, MAINTENANT).is_ok() {
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(&c.journal.cle_publique(), NiveauConfiance::Reconnu).expect("ancre");
        for s in [premiere, seconde] {
            let t = decoder_titre::<TeteRegistre>(&miyusceau::Sceau::decoder(s).expect("sceau").charge).expect("tête");
            let v = verifier_seuil::<TeteRegistre>(s, &ancres, 1, t.emise_a).expect("tête signée par le journal");
            assert_eq!(v.titre().registre, c.registre, "tête d'un autre registre admise");
        }
    }
    // Autre COG : jamais vérifiée avec la clé du COG 0.
    let autre = &cogs()[1];
    if p.verifier(&autre.journal.cle_publique(), &autre.registre, MAINTENANT).is_ok() {
        assert!(p.verifier(&c.journal.cle_publique(), &c.registre, MAINTENANT).is_err());
    }
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    if mode == 0 {
        controler(reste);
        return;
    }
    let Some((&coupe, reste)) = reste.split_first() else { return };
    let (a, b) = reste.split_at(usize::from(coupe).min(reste.len()));
    let p = Preuve::equivoque(a.to_vec(), b.to_vec());
    let q = Preuve::equivoque(b.to_vec(), a.to_vec());
    assert_eq!(p, q);
    assert_eq!(p.empreinte(), q.empreinte());
    let encode = p.encoder();
    match Preuve::decoder(&encode) {
        Ok(relue) => assert_eq!(relue, p, "aller-retour différent"),
        // Refus seulement si l'un des deux n'est pas un sceau bien formé.
        Err(_) => assert!(miyusceau::Sceau::decoder(a).is_err() || miyusceau::Sceau::decoder(b).is_err()),
    }
    controler(&encode);
});
