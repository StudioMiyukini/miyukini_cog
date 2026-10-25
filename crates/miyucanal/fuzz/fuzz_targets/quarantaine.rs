//! @id     miyucanal_fuzz_quarantaine
//! @do     fuzzer_etat_quarantaine
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — lecture de l'état persistant de quarantaine `MCQT` (fail-closed, A-006) (EX-10, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : état brut → `ouvrir_quarantaine` ;
//! - autre : vues quelconques (`identité (1) | index (1) | vue_le (2)` par vue)
//!   scellées avec la clé fixe, puis altérées par les 3 derniers octets
//!   (position, masque, troncature).
//!
//! Propriétés : aucune panique ; un état illisible ne rend rien ;
//! enregistrements retenus + refusés = enregistrements lus ; aucune vue
//! retenue n'est forgée (toujours une vue scellée, la première vue la plus
//! tardive par annonce) ; état intact ⇒ aucune altération signalée.

#![no_main]

use std::collections::HashMap;

use libfuzzer_sys::fuzz_target;
use miyucanal::{ouvrir_quarantaine, sceller_quarantaine, CleQuarantaine, VuePersistee};

const REGISTRE: [u8; 32] = [7; 32];

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    let cle = CleQuarantaine::deriver(&[9; 32]);
    if mode == 0 {
        let etat = ouvrir_quarantaine(reste, &REGISTRE, &cle);
        if etat.illisible {
            assert!(etat.vues.is_empty() && etat.refuses == 0);
        } else {
            let n = (reste.len() - 41) / 80;
            assert!(etat.vues.len() as u64 + etat.refuses <= n as u64);
        }
        return;
    }
    if reste.len() < 3 {
        return;
    }
    let (corps, alteration) = reste.split_at(reste.len() - 3);
    let vues: Vec<VuePersistee> = corps
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| VuePersistee {
            identite: [c[0] % 8; 32],
            index: u64::from(c[1]),
            vue_le: u64::from(u16::from_be_bytes([c[2], c[3]])),
        })
        .collect();
    let mut etat_scelle = sceller_quarantaine(&REGISTRE, &vues, &cle);

    // Attendu sans altération : la première vue la plus tardive par annonce.
    let mut attendues: HashMap<[u8; 32], VuePersistee> = HashMap::new();
    for v in &vues {
        let r = attendues.entry(v.identite).or_insert(*v);
        if v.vue_le > r.vue_le {
            *r = *v;
        }
    }
    let intact = ouvrir_quarantaine(&etat_scelle, &REGISTRE, &cle);
    assert!(!intact.altere(), "état intact signalé altéré");
    assert_eq!(intact.vues, attendues);
    assert!(ouvrir_quarantaine(&etat_scelle, &[8; 32], &cle).illisible, "état d'un autre registre admis");

    // Altération : bit retourné et/ou troncature.
    let (pos, masque, coupe) = (alteration[0], alteration[1], alteration[2]);
    if !etat_scelle.is_empty() && masque != 0 {
        let p = usize::from(pos) * 13 % etat_scelle.len();
        etat_scelle[p] ^= masque;
    }
    if coupe & 0x80 != 0 {
        let garde = etat_scelle.len().saturating_sub(usize::from(coupe & 0x7F));
        etat_scelle.truncate(garde);
    }
    let etat = ouvrir_quarantaine(&etat_scelle, &REGISTRE, &cle);
    for (id, v) in &etat.vues {
        assert_eq!(&v.identite, id);
        assert!(vues.contains(v), "vue forgée retenue : {v:?}");
    }
    if etat.illisible {
        assert!(etat.vues.is_empty());
    }
});
