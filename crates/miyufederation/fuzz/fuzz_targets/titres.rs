//! @id     miyufederation_fuzz_titres
//! @do     fuzzer_titres_federaux
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — titres fédéraux : certificats 0x06 (accusation 0x42, candidature 0x43, recours 0x44) et confirmation 0x07 (EX-30, EX-32, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : charge TLV brute → décodage en candidature, confirmation,
//!   accusation, recours ;
//! - 1 : sceau brut → `recevoir_candidature`, `recevoir_confirmation`,
//!   `recevoir_accusation`, `recevoir_recours` d'une fédération de 7
//!   fondateurs ;
//! - 2 : `k | seq A | taille A | racine A | seq B | taille B | racine B | k recours` :
//!   deux têtes signées par le journal du COG 0, accusation d'équivoque
//!   signée par les membres 1..=k, puis recours signé par les membres 1..=k' ;
//! - autre : `confirmatrice | reconnue | émise (2) | durée (3)` : confirmation
//!   signée par la confirmatrice → `recevoir_confirmation` puis `actualiser`.
//!
//! Propriétés : aucune panique ; titres canoniques (ré-encodage identique) et
//! natures exclusives (une charge ne décode qu'un seul titre) ; une
//! accusation n'aboutit (suspension) qu'au quorum de 6 membres **et** sur une
//! équivoque réelle ; un recours ne lève la suspension qu'au quorum de 6, et
//! l'accusation levée n'est plus rejouable.

#![no_main]

#[path = "commun.rs"]
mod commun;

use commun::{cogs, federation, JOUR, MAINTENANT, T0};
use libfuzzer_sys::fuzz_target;
use miyufederation::{
    Accusation, Candidature, Confirmation, ErreurFederation, Preuve, Recours, QUORUM_BLACKLIST, QUORUM_RECOURS,
};
use miyusceau::{decoder_titre, EcrivainTlv, Sceau, TeteRegistre, Titre};

fn canonique<T: Titre>(charge: &[u8]) -> bool {
    let Ok(t) = decoder_titre::<T>(charge) else { return false };
    let mut e = EcrivainTlv::new();
    t.encoder(&mut e).expect("titre décodé ré-encodable");
    assert_eq!(e.finir(), charge, "titre non canonique");
    true
}

fn charge_brute(charge: &[u8]) {
    let decodes = [
        canonique::<Candidature>(charge),
        canonique::<Confirmation>(charge),
        canonique::<Accusation>(charge),
        canonique::<Recours>(charge),
    ];
    assert!(decodes.iter().filter(|d| **d).count() <= 1, "charge décodée en plusieurs titres : {decodes:?}");
}

fn sceau_brut(octets: &[u8]) {
    let mut f = federation();
    if let Ok(cog) = f.recevoir_candidature(octets, MAINTENANT) {
        assert!(f.statut(&cog).is_some());
    }
    let _ = f.recevoir_confirmation(octets, MAINTENANT);
    let preuve = cogs()[0]
        .tete(1, 1, [1; 32], T0)
        .zip(cogs()[0].tete(1, 1, [2; 32], T0))
        .map(|(a, b)| Preuve::equivoque(a, b))
        .expect("preuve fixe");
    let _ = f.recevoir_accusation(octets, &preuve, MAINTENANT);
    let _ = f.recevoir_recours(octets, MAINTENANT);
    let _ = f.actualiser(MAINTENANT + JOUR);
}

/// Mêmes règles que `Preuve::verifier` (têtes valides du même registre).
fn contradictoires(a: (u64, u64, u8), b: (u64, u64, u8)) -> bool {
    let (x, y) = if a.0 <= b.0 { (a, b) } else { (b, a) };
    (x.0 == y.0 && x != y) || y.1 < x.1 || (y.1 == x.1 && y.2 != x.2)
}

fn signer<F: Fn(&[u8], &miyusceau::CleSignature) -> Result<Vec<u8>, ErreurFederation>>(
    mut octets: Vec<u8>,
    n: usize,
    cosigner: F,
) -> Vec<u8> {
    for c in &cogs()[2..=n] {
        octets = cosigner(&octets, &c.identite).expect("cosignature");
    }
    octets
}

fn accusation_et_recours(p: &[u8]) {
    let [k, sa, ta, ra, sb, tb, rb, k2, ..] = *p else { return };
    let (k, k2) = (1 + usize::from(k) % 6, 1 + usize::from(k2) % 6);
    let a = (u64::from(sa % 4), u64::from(ta % 4), ra % 3);
    let b = (u64::from(sb % 4), u64::from(tb % 4), rb % 3);
    let accuse = &cogs()[0];
    let (Some(ta), Some(tb)) = (accuse.tete(a.0, a.1, [a.2; 32], T0), accuse.tete(b.0, b.1, [b.2; 32], T0)) else {
        return;
    };
    // Tête encodable mais indécodable (taille 0, racine non vide) : preuve refusée.
    let lisible = |t: &[u8]| decoder_titre::<TeteRegistre>(&Sceau::decoder(t).expect("sceau").charge).is_ok();
    let tetes_valides = lisible(&ta) && lisible(&tb);
    let preuve = Preuve::equivoque(ta, tb);
    assert_eq!(Preuve::decoder(&preuve.encoder()).ok().as_ref(), Some(&preuve));
    let acc = Accusation::depuis_preuve(accuse.identite.cle_publique(), &preuve, T0 + 50);
    let octets = signer(acc.sceller(&cogs()[1].identite).expect("accusation"), k, Accusation::cosigner);

    let mut f = federation();
    let issue = f.recevoir_accusation(&octets, &preuve, MAINTENANT);
    let attendu = k >= QUORUM_BLACKLIST && tetes_valides && contradictoires(a, b);
    assert_eq!(issue.is_ok(), attendu, "accusation k={k} {a:?}/{b:?} : {issue:?}");
    let Ok(suspension) = issue else { return };
    assert!(f.cog_suspendu(&suspension.cog));

    let recours = Recours {
        accuse: suspension.racine,
        type_preuve: suspension.type_preuve,
        empreinte_preuve: suspension.empreinte_preuve,
        accusation: suspension.accusation,
        emise_a: MAINTENANT,
    };
    let octets_r = signer(recours.sceller(&cogs()[1].identite).expect("recours"), k2, Recours::cosigner);
    let issue = f.recevoir_recours(&octets_r, MAINTENANT + 1);
    assert_eq!(issue.is_ok(), k2 >= QUORUM_RECOURS, "recours k'={k2} : {issue:?}");
    if issue.is_ok() {
        assert!(!f.cog_suspendu(&suspension.cog));
        assert!(matches!(f.recevoir_accusation(&octets, &preuve, MAINTENANT + 2), Err(ErreurFederation::AccusationLevee)));
    }
}

fn confirmation(p: &[u8]) {
    let [i, j, e0, e1, d0, d1, d2, ..] = *p else { return };
    let (confirmatrice, reconnue) = (&cogs()[usize::from(i) % 7], &cogs()[usize::from(j) % 7]);
    let emise_a = T0 + u64::from(u16::from_be_bytes([e0, e1]));
    let duree = u64::from(u32::from_be_bytes([0, d0, d1, d2]));
    let c = Confirmation {
        reconnue: reconnue.identite.cle_publique(),
        confirmatrice: confirmatrice.identite.cle_publique(),
        emise_a,
        expire_a: emise_a.saturating_add(duree),
    };
    let Ok(octets) = c.sceller(&confirmatrice.identite) else { return };
    let mut f = federation();
    let _ = f.recevoir_confirmation(&octets, MAINTENANT);
    let _ = f.actualiser(MAINTENANT + JOUR);
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    match mode {
        0 => charge_brute(reste),
        1 => sceau_brut(reste),
        2 => accusation_et_recours(reste),
        _ => confirmation(reste),
    }
});
