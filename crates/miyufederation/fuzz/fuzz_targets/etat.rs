//! @id     miyufederation_fuzz_etat
//! @do     fuzzer_etat_federal_mfed
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — état fédéral persistant `MFED` (statuts, sanctions, note, carnet) et assemblage des morceaux d'un support borné (C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : enveloppe brute → `Federation::reprendre` (secret local fixe) ;
//! - 1 : **corps** `MFED` v3 scellé par la cible (HMAC du secret local fixe
//!   recalculé : le fuzzing atteint la revérification des titres et la
//!   cohérence des statuts) ;
//! - 2 : idem en version 0x02 (sans carnet) ;
//! - autre : pointeur et morceaux (`generation (8) | morceaux (1) | taille (3)
//!   | empreinte (32) | morceaux hexadécimaux séparés par `,``) →
//!   `support::assembler_morceaux`.
//!
//! Propriétés : aucune panique ni allocation non bornée (bornes vérifiées
//! avant allocation) ; un état accepté se re-scelle en un **point fixe**
//! (re-scellé, repris et re-scellé à l'identique, mêmes statuts) ; aucun
//! statut membre d'un non-fondateur sans confirmations revérifiées ; un
//! assemblage accepté est exactement le découpage de ses octets.

#![no_main]

#[path = "commun.rs"]
mod commun;

use commun::{cle_etat, envelopper, parametres, JOUR, T0};
use libfuzzer_sys::fuzz_target;
use miyufederation::support::{assembler_morceaux, decouper, Pointeur};
use miyufederation::{AncreRegistre, Federation, Statut};

/// Heure de reprise : 10 jours après T0 (rattrapage des échéances de 72 h et des 7 jours).
const REPRISE: u64 = T0 + 10 * JOUR;
const ANCRE: AncreRegistre = AncreRegistre { taille: 0, racine: [0; 32] };

fn reprendre(octets: &[u8]) {
    let Ok((f, revocations, _)) = Federation::reprendre(octets, cle_etat(), parametres(), None, REPRISE) else { return };
    // Point fixe : re-scellé, repris à la même heure, re-scellé à l'identique.
    let s1 = f.sceller_etat(cle_etat(), &ANCRE).expect("état repris re-scellable");
    let (g, rev2, _) = Federation::reprendre(&s1, cle_etat(), parametres(), None, REPRISE).expect("état re-scellé relu");
    assert!(rev2.is_empty(), "révocation produite deux fois");
    assert_eq!(g.sceller_etat(cle_etat(), &ANCRE).expect("re-scellable"), s1, "état non stable");
    for r in &revocations {
        assert_eq!(f.statut(&r.cog), Some(Statut::Revoque));
    }
    for cog in f.membres() {
        assert_eq!(g.statut(&cog), Some(Statut::Membre));
    }
    assert_eq!(f.membres(), g.membres());
    assert!(f.carnet().len() <= miyufederation::MAX_PAIRS_CARNET);
}

fn morceaux(p: &[u8]) {
    if p.len() < 44 {
        return;
    }
    let (tete, reste) = p.split_at(44);
    let mut empreinte = [0u8; 32];
    empreinte.copy_from_slice(&tete[12..44]);
    let pointeur = Pointeur {
        generation: u64::from_be_bytes(tete[0..8].try_into().expect("8 octets")),
        morceaux: u32::from(tete[8]),
        taille: u64::from(u32::from_be_bytes([0, tete[9], tete[10], tete[11]])),
        empreinte,
    };
    let Ok(texte) = std::str::from_utf8(reste) else { return };
    let lus: Vec<(u64, &str)> = texte.split(',').enumerate().map(|(i, s)| (i as u64, s)).collect();
    if let Ok(etat) = assembler_morceaux(&pointeur, &lus) {
        let (p2, m2) = decouper(&etat, pointeur.generation).expect("état assemblé découpable");
        assert_eq!(p2, pointeur, "pointeur différent du découpage");
        assert_eq!(m2.iter().map(String::as_str).collect::<Vec<_>>(), lus.iter().map(|(_, s)| *s).collect::<Vec<_>>());
    }
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    match mode {
        0 => reprendre(reste),
        1 => reprendre(&envelopper(0x03, reste)),
        2 => reprendre(&envelopper(0x02, reste)),
        _ => morceaux(reste),
    }
});
