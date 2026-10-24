//! Propriétés déterministes (EX-10, complément stable du fuzzing) : sceaux
//! valides mutés (bits inversés, troncatures, insertions, suppressions,
//! écrasements). Aucune panique ; toute mutation qui modifie les octets
//! signés est refusée ; avec un seuil égal au nombre de signataires, toute
//! mutation est refusée.
//!
//! Générateur pseudo-aléatoire SplitMix64 à graine fixe : reproductible, sans
//! dépendance.

#![allow(clippy::many_single_char_names)]

use miyusceau::*;

const T0: u64 = 1_800_000_000;
const REG: [u8; 32] = [0x24; 32];
const MUTATIONS_PAR_GRAINE: usize = 1_000;

struct SplitMix(u64);

impl SplitMix {
    fn suivant(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn sous(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.suivant() % n as u64) as usize
        }
    }

    fn octet(&mut self) -> u8 {
        self.suivant() as u8
    }
}

fn muter(g: &mut SplitMix, original: &[u8]) -> Vec<u8> {
    let mut o = original.to_vec();
    match g.sous(7) {
        0 => {
            // 1 à 3 bits inversés.
            for _ in 0..=g.sous(3) {
                let i = g.sous(o.len());
                o[i] ^= 1 << g.sous(8);
            }
        }
        1 => o.truncate(g.sous(o.len())),
        2 => {
            let i = g.sous(o.len() + 1);
            let b = g.octet();
            o.insert(i, b);
        }
        3 => {
            let i = g.sous(o.len());
            o.remove(i);
        }
        4 => {
            // Écrasement d'une plage.
            let i = g.sous(o.len());
            let n = 1 + g.sous(16.min(o.len() - i));
            for x in &mut o[i..i + n] {
                *x = g.octet();
            }
        }
        5 => {
            // Octet remplacé par une valeur limite.
            let i = g.sous(o.len());
            o[i] = [0x00, 0x01, 0x7f, 0x80, 0xff][g.sous(5)];
        }
        _ => {
            // Duplication d'un fragment.
            let i = g.sous(o.len());
            let n = 1 + g.sous(40.min(o.len() - i));
            let fragment = o[i..i + n].to_vec();
            let j = g.sous(o.len() + 1);
            o.splice(j..j, fragment);
        }
    }
    o
}

fn cle(graine: u8) -> CleSignature {
    CleSignature::depuis_graine(&[graine; 32])
}

fn tete() -> TeteRegistre {
    let feuilles: Vec<Empreinte> = (0u8..5).map(|i| hacher_feuille(&[i])).collect();
    TeteRegistre {
        registre: REG,
        taille: 5,
        racine: racine(&feuilles),
        sequence: 9,
        emise_a: T0,
        validite_max_secs: AGE_MAX_TETE_DEFAUT_SECS,
    }
}

/// Exerce tous les décodeurs sur `octets` : aucune panique attendue.
fn exercer_decodeurs(octets: &[u8]) {
    if let Ok(s) = Sceau::decoder(octets) {
        let _ = decoder_titre::<TeteRegistre>(&s.charge);
        let _ = s.octets_signes();
    }
    let _ = decoder_titre::<TeteRegistre>(octets);
    let _ = PreuveInclusion::decoder(octets);
    let _ = PreuveCoherence::decoder(octets);
    let _ = EtatRegistre::decoder(octets, &AncresDeConfiance::new(), &PolitiqueTete::journal_cog(), &REG);
}

fn campagne(graine: u64, cles: &[&CleSignature]) {
    let mut ancres = AncresDeConfiance::new();
    for c in cles {
        ancres.ajouter(&c.cle_publique(), NiveauConfiance::Souverain).unwrap();
    }
    let original = sceller_multi(&tete(), cles).unwrap();
    let signes_original = Sceau::decoder(&original).unwrap().octets_signes();
    let tous = cles.len();
    let politique_tous = PolitiqueTete::nouvelle(tous, AGE_MAX_TETE_DEFAUT_SECS).unwrap();
    let politique_un = PolitiqueTete::journal_cog();
    verifier_tete(&original, &ancres, &politique_tous, &REG, T0).unwrap();

    let mut g = SplitMix(graine);
    let mut refus = 0usize;
    for _ in 0..MUTATIONS_PAR_GRAINE {
        let mute = muter(&mut g, &original);
        exercer_decodeurs(&mute);
        if mute == original {
            continue;
        }
        // Seuil = tous les signataires : toute mutation est refusée.
        assert!(
            verifier_seuil::<TeteRegistre>(&mute, &ancres, tous, T0).is_err(),
            "mutation acceptée (seuil {tous}) : {mute:02x?}"
        );
        assert!(verifier_tete(&mute, &ancres, &politique_tous, &REG, T0).is_err());
        // Seuil 1 : acceptée seulement si les octets signés sont intacts.
        let signes_intacts = Sceau::decoder(&mute).is_ok_and(|s| s.octets_signes() == signes_original);
        match verifier_tete(&mute, &ancres, &politique_un, &REG, T0) {
            Ok(v) => {
                assert!(signes_intacts, "octets signés modifiés mais acceptés : {mute:02x?}");
                assert_eq!(v.tete(), &tete());
            }
            Err(_) => refus += 1,
        }
    }
    assert!(refus > MUTATIONS_PAR_GRAINE / 2);
}

#[test]
fn mutations_sceau_un_signataire() {
    campagne(0x5eed_0001, &[&cle(31)]);
}

#[test]
fn mutations_sceau_deux_signataires() {
    campagne(0x5eed_0002, &[&cle(31), &cle(32)]);
}

#[test]
fn mutations_etat_et_preuves() {
    let c = cle(33);
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&c.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let pol = PolitiqueTete::journal_cog();
    let etat = EtatRegistre::initial(verifier_tete(&sceller(&tete(), &c).unwrap(), &ancres, &pol, &REG, T0).unwrap());
    let octets_etat = etat.encoder();
    let feuilles: Vec<Empreinte> = (0u8..5).map(|i| hacher_feuille(&[i])).collect();
    let inclusion_ref = PreuveInclusion::generer(&feuilles, 3).unwrap();
    let coherence_ref = PreuveCoherence::generer(&feuilles, 3).unwrap();
    let inclusion = inclusion_ref.encoder();
    let coherence = coherence_ref.encoder();
    let r3 = racine(&feuilles[..3]);
    let r5 = racine(&feuilles);

    let mut g = SplitMix(0x5eed_0003);
    for _ in 0..MUTATIONS_PAR_GRAINE {
        let m = muter(&mut g, &octets_etat);
        exercer_decodeurs(&m);
        if m != octets_etat {
            // Un état altéré n'est jamais rechargé avec une autre tête.
            if let Ok(relu) = EtatRegistre::decoder(&m, &ancres, &pol, &REG) {
                assert_eq!(relu.tete(), etat.tete());
            }
        }
        let m = muter(&mut g, &inclusion);
        exercer_decodeurs(&m);
        // À taille liée (celle de la tête signée), seule la preuve exacte vérifie.
        if let Ok(p) = PreuveInclusion::decoder(&m) {
            if p.taille == 5 && p.verifier(&feuilles[3], &r5).is_ok() {
                assert_eq!(p, inclusion_ref);
            }
        }
        let m = muter(&mut g, &coherence);
        exercer_decodeurs(&m);
        if let Ok(p) = PreuveCoherence::decoder(&m) {
            if p.taille1 == 3 && p.taille2 == 5 && p.verifier(&r3, &r5).is_ok() {
                assert_eq!(p, coherence_ref);
            }
        }
    }
}

#[test]
fn octets_aleatoires_sans_panique() {
    let mut g = SplitMix(0x5eed_0004);
    let ancres = AncresDeConfiance::new();
    for _ in 0..5_000 {
        let n = g.sous(300);
        let mut o: Vec<u8> = (0..n).map(|_| g.octet()).collect();
        if g.sous(2) == 0 && o.len() >= 7 {
            // Entête plausible pour atteindre les couches profondes.
            o[..7].copy_from_slice(b"MSCE\x02\x01\x05");
        }
        exercer_decodeurs(&o);
        assert!(verifier_seuil::<TeteRegistre>(&o, &ancres, 1, T0).is_err());
    }
}
