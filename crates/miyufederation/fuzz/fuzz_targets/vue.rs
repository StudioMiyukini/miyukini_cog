//! @id     miyufederation_fuzz_vue
//! @do     fuzzer_vue_et_registre_federal
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — vue fédérale : têtes de registre témoignées (≥ 3 membres), témoignage local, équivoque, carnet de pairs, persistance de la vue obtenue (EX-34, EX-35, EX-37, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : tête brute → `recevoir_tete` et `temoigner` (COG 0, journal du COG 1) ;
//! - 1 : suite d'opérations de 8 octets
//!   `op | k | seq | taille | racine | décalage (2) | cog` sur une fédération
//!   de 7 fondateurs : tête du COG `cog` signée par son journal et
//!   contresignée par les journaux de `k` autres membres (`recevoir_tete`),
//!   témoignage local, contact du carnet, actualisation ; puis la vue obtenue
//!   est scellée et reprise.
//!
//! Propriétés : aucune panique ; une tête n'est crue qu'avec ≥ 3 témoins
//! membres distincts ; deux têtes contradictoires acceptées (vue) ou
//! contresignées (témoignage) pour un même COG sont impossibles ; la vue
//! fédérale n'est jamais datée après l'heure locale ; **toute vue obtenue par
//! des opérations acceptées est rechargeable** (scellée, reprise, re-scellée
//! à l'identique : une transition ne peut pas produire un état que le
//! rechargement refuserait, ce qui désactiverait la fédération).

#![no_main]

#[path = "commun.rs"]
mod commun;

use commun::{cle_etat, cog_id, cogs, federation, parametres, FONDATEURS, MAINTENANT, T0};
use libfuzzer_sys::fuzz_target;
use miyufederation::{AncreRegistre, ErreurFederation, Federation, TEMOINS_MIN};
use miyusceau::{cosigner, CleSignature, TeteRegistre};

const ANCRE: AncreRegistre = AncreRegistre { taille: 0, racine: [0; 32] };

/// Journal du COG local (témoin), distinct des 7 fondateurs.
fn journal_local() -> CleSignature {
    CleSignature::depuis_graine(&[0xee; 32])
}

fn brute(octets: &[u8]) {
    let mut f = federation();
    let cog = cog_id(&cogs()[0]);
    let _ = f.recevoir_tete(&cog, octets, None, MAINTENANT);
    let _ = f.temoigner(&cog, octets, None, &journal_local(), MAINTENANT);
    assert!(f.vue_a_jour_au().is_none_or(|v| v <= MAINTENANT));
    recharger(&f, MAINTENANT);
}

/// La vue obtenue, scellée puis reprise à `t` (≥ son heure), est celle d'un
/// jumeau qui a tourné jusqu'à `t` (aucune suspension ici : même calendrier).
fn recharger(f: &Federation, t: u64) {
    let s = f.sceller_etat(cle_etat(), &ANCRE).expect("vue scellable");
    let (g, _, _) = Federation::reprendre(&s, cle_etat(), parametres(), None, t).expect("vue obtenue rechargeable");
    let mut jumeau = f.clone();
    jumeau.actualiser(t).expect("horloge monotone");
    assert_eq!(
        g.sceller_etat(cle_etat(), &ANCRE).expect("re-scellable"),
        jumeau.sceller_etat(cle_etat(), &ANCRE).expect("scellable"),
        "rechargement différent du jumeau"
    );
    assert_eq!(g.vue_a_jour_au(), f.vue_a_jour_au());
    assert_eq!(g.membres(), jumeau.membres());
}

fn suite(p: &[u8]) {
    let mut f = federation();
    let mut t = MAINTENANT;
    // Têtes acceptées par COG : (séquence, taille, racine).
    let mut vues: Vec<Vec<(u64, u64, u8)>> = vec![Vec::new(); FONDATEURS];
    for op in p.chunks_exact(8).take(64) {
        let [code, k, seq, taille, racine, d0, d1, c] = *op else { continue };
        t += u64::from(u16::from_be_bytes([d0, d1]) % 600);
        let i = usize::from(c) % FONDATEURS;
        let proprietaire = &cogs()[i];
        let cog = cog_id(proprietaire);
        let (seq, taille, racine) = (u64::from(seq % 4), u64::from(taille % 4), racine % 3);
        let emise = t.saturating_sub(u64::from(code >> 4) * 60).max(T0);
        let Some(tete) = proprietaire.tete(seq, taille, [racine; 32], emise) else { continue };
        let k = usize::from(k) % FONDATEURS;
        let temoins: Vec<&commun::CogFuzz> = cogs().iter().enumerate().filter(|(j, _)| *j != i).map(|(_, c)| c).take(k).collect();
        // Tête encodable mais indécodable (taille 0, racine non vide) : non cosignable, ignorée.
        let Ok(tete) = temoins.iter().try_fold(tete, |t, w| cosigner::<TeteRegistre>(&t, &w.journal)) else { continue };
        match code % 4 {
            0 => match f.recevoir_tete(&cog, &tete, None, t) {
                Ok(_) => {
                    assert!(temoins.len() >= TEMOINS_MIN, "tête crue avec {} témoins", temoins.len());
                    // Même séquence : même contenu (aucune équivoque crue).
                    assert!(
                        vues[i].iter().all(|&(s0, t0, r0)| s0 != seq || (t0, r0) == (taille, racine)),
                        "équivoque acceptée dans la vue"
                    );
                    vues[i].push((seq, taille, racine));
                }
                Err(ErreurFederation::RetourHorloge) => unreachable!("horloge monotone"),
                Err(_) => {}
            },
            1 => {
                let _ = f.temoigner(&cog, &tete, None, &journal_local(), t);
            }
            2 => {
                let _ = f.noter_contact(&cog, &format!("reseau-{}", k % 3), t);
            }
            _ => {
                let _ = f.actualiser(t);
            }
        }
        assert!(f.vue_a_jour_au().is_none_or(|v| v <= t));
    }
    recharger(&f, t);
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    if mode == 0 {
        brute(reste);
    } else {
        suite(reste);
    }
});
