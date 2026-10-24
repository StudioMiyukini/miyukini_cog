//! Registre : arbre de Merkle RFC 9162 (vecteurs de référence) et tête signée
//! (fraîcheur, seuil, monotonie, équivoque) — EX-03.

#![allow(clippy::format_collect, clippy::many_single_char_names)]

use miyusceau::*;

fn hex(octets: &[u8]) -> String {
    octets.iter().map(|b| format!("{b:02x}")).collect()
}

fn de_hex(s: &str) -> Empreinte {
    let mut e = [0u8; 32];
    for (i, o) in e.iter_mut().enumerate() {
        *o = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap();
    }
    e
}

/// Entrées de référence du jeu de test Certificate Transparency (RFC 6962).
fn entrees_ct() -> Vec<Vec<u8>> {
    vec![
        vec![],
        vec![0x00],
        vec![0x10],
        vec![0x20, 0x21],
        vec![0x30, 0x31],
        vec![0x40, 0x41, 0x42, 0x43],
        vec![0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57],
        (0x60..=0x6f).collect(),
    ]
}

fn feuilles_ct() -> Vec<Empreinte> {
    entrees_ct().iter().map(|e| hacher_feuille(e)).collect()
}

/// Racines de référence (tailles 1 à 8) du jeu CT.
const RACINES_CT: [&str; 8] = [
    "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d",
    "fac54203e7cc696cf0dfcb42c92a1d9dbaf70ad9e621f4bd8d98662f00e3c125",
    "aeb6bcfe274b70a14fb067a5e5578264db0fa9b51af5e0ba159158f329e06e77",
    "d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
    "4e3bbb1f7b478dcfe71fb631631519a3bca12c9aefca1612bfce4c13a86264d4",
    "76e67dadbcdf1e10e1b74ddc608abd2f98dfb16fbce75277b5232a127f2087ef",
    "ddb89be403809e325750d3d263cd78929c2942b7942a34b77e122c9594a74c8c",
    "5dc9da79a70659a9ad559cb701ded9a2ab9d823aad2f4960cfe370eff4604328",
];

#[test]
fn merkle_racines_de_reference() {
    assert_eq!(hex(&racine(&[])), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    assert_eq!(racine_vide(), racine(&[]));
    let f = feuilles_ct();
    for (n, attendu) in RACINES_CT.iter().enumerate() {
        assert_eq!(hex(&racine(&f[..=n])), *attendu, "taille {}", n + 1);
    }
}

#[test]
fn merkle_preuves_inclusion_de_reference() {
    let f = feuilles_ct();
    // (index, taille, chemin) du jeu CT.
    let cas: [(usize, usize, &[&str]); 4] = [
        (0, 1, &[]),
        (
            0,
            8,
            &[
                "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7",
                "5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e",
                "6b47aaf29ee3c2af9af889bc1fb9254dabd31177f16232dd6aab035ca39bf6e4",
            ],
        ),
        (
            5,
            8,
            &[
                "bc1a0643b12e4d2d7c77918f44e0f4f79a838b6cf9ec5b5c283e1f4d88599e6b",
                "ca854ea128ed050b41b35ffc1b87b8eb2bde461e9e3b5596ece6b9d5975a0ae0",
                "d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
            ],
        ),
        (
            2,
            3,
            &["fac54203e7cc696cf0dfcb42c92a1d9dbaf70ad9e621f4bd8d98662f00e3c125"],
        ),
    ];
    for (index, taille, chemin) in cas {
        let preuve = PreuveInclusion::generer(&f[..taille], index).unwrap();
        let attendu: Vec<Empreinte> = chemin.iter().map(|h| de_hex(h)).collect();
        assert_eq!(preuve.chemin, attendu, "inclusion {index}/{taille}");
        preuve.verifier(&f[index], &racine(&f[..taille])).unwrap();
    }
}

#[test]
fn merkle_preuves_coherence_de_reference() {
    let f = feuilles_ct();
    let cas: [(usize, usize, &[&str]); 4] = [
        (1, 1, &[]),
        (
            1,
            8,
            &[
                "96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7",
                "5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e",
                "6b47aaf29ee3c2af9af889bc1fb9254dabd31177f16232dd6aab035ca39bf6e4",
            ],
        ),
        (
            6,
            8,
            &[
                "0ebc5d3437fbe2db158b9f126a1d118e308181031d0a949f8dededebc558ef6a",
                "ca854ea128ed050b41b35ffc1b87b8eb2bde461e9e3b5596ece6b9d5975a0ae0",
                "d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
            ],
        ),
        (
            2,
            5,
            &[
                "5f083f0a1a33ca076a95279832580db3e0ef4584bdff1f54c8a360f50de3031e",
                "bc1a0643b12e4d2d7c77918f44e0f4f79a838b6cf9ec5b5c283e1f4d88599e6b",
            ],
        ),
    ];
    for (m, n, chemin) in cas {
        let preuve = PreuveCoherence::generer(&f[..n], m).unwrap();
        let attendu: Vec<Empreinte> = chemin.iter().map(|h| de_hex(h)).collect();
        assert_eq!(preuve.chemin, attendu, "cohérence {m}→{n}");
        preuve.verifier(&racine(&f[..m]), &racine(&f[..n])).unwrap();
    }
}

#[test]
fn merkle_toutes_preuves_jusqua_40_et_falsifications() {
    let feuilles: Vec<Empreinte> = (0u32..40).map(|i| hacher_feuille(&i.to_be_bytes())).collect();
    for n in 1..=feuilles.len() {
        let r = racine(&feuilles[..n]);
        for i in 0..n {
            let p = PreuveInclusion::generer(&feuilles[..n], i).unwrap();
            p.verifier(&feuilles[i], &r).unwrap();
            assert_eq!(PreuveInclusion::decoder(&p.encoder()).unwrap(), p);
            // Mauvaise feuille, mauvais index, mauvaise taille, chemin altéré : refus.
            assert!(p.verifier(&hacher_feuille(b"autre"), &r).is_err());
            let mut q = p.clone();
            q.index = (q.index + 1) % n as u64;
            if n > 1 {
                assert!(q.verifier(&feuilles[i], &r).is_err());
            }
            // Taille annoncée fausse contre la vraie racine de cette taille
            // (la racine signée lie la taille : `EtatRegistre` exige l'égalité).
            if n < feuilles.len() {
                let mut q = p.clone();
                q.taille += 1;
                assert!(q.verifier(&feuilles[i], &racine(&feuilles[..=n])).is_err(), "n={n} i={i}");
            }
            if let Some(premier) = p.chemin.first() {
                let mut q = p.clone();
                q.chemin[0][0] ^= 1;
                assert!(q.verifier(&feuilles[i], &r).is_err());
                let mut q = p.clone();
                q.chemin.push(*premier);
                assert!(q.verifier(&feuilles[i], &r).is_err());
                let mut q = p.clone();
                q.chemin.pop();
                assert!(q.verifier(&feuilles[i], &r).is_err());
            }
        }
        for m in 1..=n {
            let r1 = racine(&feuilles[..m]);
            let p = PreuveCoherence::generer(&feuilles[..n], m).unwrap();
            p.verifier(&r1, &r).unwrap();
            assert_eq!(PreuveCoherence::decoder(&p.encoder()).unwrap(), p);
            if m < n {
                // Histoire réécrite : ancienne racine différente.
                let mut reecrite = feuilles[..n].to_vec();
                reecrite[m - 1] = hacher_feuille(b"substitution");
                assert!(p.verifier(&r1, &racine(&reecrite)).is_err());
                assert!(p.verifier(&hacher_feuille(b"x"), &r).is_err());
                let mut q = p.clone();
                q.chemin[0][31] ^= 0x80;
                assert!(q.verifier(&r1, &r).is_err());
            }
        }
    }
}

#[test]
fn merkle_decodage_strict() {
    let p = PreuveInclusion { index: 1, taille: 2, chemin: vec![[3u8; 32]] };
    let o = p.encoder();
    assert_eq!(o.len(), 17 + 32);
    assert!(PreuveInclusion::decoder(&o[..o.len() - 1]).is_err());
    let mut plus = o.clone();
    plus.push(0);
    assert!(PreuveInclusion::decoder(&plus).is_err());
    let mut trop = vec![0u8; 17];
    trop[16] = 65;
    trop.extend(std::iter::repeat_n(0u8, 65 * 32));
    assert!(PreuveInclusion::decoder(&trop).is_err());
    assert!(PreuveCoherence::decoder(&[]).is_err());
    // Tailles incohérentes : refus sans panique.
    let q = PreuveCoherence { taille1: 0, taille2: 3, chemin: vec![] };
    assert!(q.verifier(&[0; 32], &[0; 32]).is_err());
    let q = PreuveCoherence { taille1: 4, taille2: 3, chemin: vec![[0; 32]] };
    assert!(q.verifier(&[0; 32], &[0; 32]).is_err());
    let q = PreuveInclusion { index: 0, taille: 0, chemin: vec![] };
    assert!(q.verifier(&[0; 32], &[0; 32]).is_err());
    let q = PreuveInclusion { index: u64::MAX, taille: u64::MAX, chemin: vec![[0; 32]; 64] };
    assert!(q.verifier(&[0; 32], &[0; 32]).is_err());
}

// ───────────────────────────── Tête signée ─────────────────────────────

const T0: u64 = 1_800_000_000;
const REG: [u8; 32] = [0x42; 32];

fn journal() -> CleSignature {
    CleSignature::depuis_graine(&[11u8; 32])
}

fn editeur() -> CleSignature {
    CleSignature::depuis_graine(&[12u8; 32])
}

fn ancres(cles: &[&CleSignature]) -> AncresDeConfiance {
    let mut a = AncresDeConfiance::new();
    for c in cles {
        a.ajouter(&c.cle_publique(), NiveauConfiance::Reconnu).unwrap();
    }
    a
}

fn arbre(n: usize) -> Vec<Empreinte> {
    (0..n).map(|i| hacher_feuille(format!("entrée {i}").as_bytes())).collect()
}

fn tete(sequence: u64, taille: usize, emise_a: u64) -> TeteRegistre {
    TeteRegistre {
        registre: REG,
        taille: taille as u64,
        racine: racine(&arbre(taille)),
        sequence,
        emise_a,
        validite_max_secs: AGE_MAX_TETE_DEFAUT_SECS,
    }
}

fn sceller_tete(t: &TeteRegistre) -> Vec<u8> {
    sceller(t, &journal()).unwrap()
}

fn etat_initial(t: &TeteRegistre) -> EtatRegistre {
    let pol = PolitiqueTete::journal_cog();
    EtatRegistre::initial(verifier_tete(&sceller_tete(t), &ancres(&[&journal()]), &pol, &REG, t.emise_a).unwrap())
}

#[test]
fn tete_aller_retour_et_vecteur() {
    let t = tete(1, 5, T0);
    let o = sceller_tete(&t);
    let v = verifier_tete(&o, &ancres(&[&journal()]), &PolitiqueTete::journal_cog(), &REG, T0 + 60).unwrap();
    assert_eq!(v.tete(), &t);
    assert_eq!(v.sceau(), &o[..]);
    // Encodage canonique : tags 1..6, tailles fixes.
    let sceau = Sceau::decoder(&o).unwrap();
    assert_eq!(sceau.type_sceau, TypeSceau::TeteRegistre);
    assert_eq!(sceau.charge.len(), 6 * 5 + 32 + 8 + 32 + 8 + 8 + 8);
    assert_eq!(decoder_titre::<TeteRegistre>(&sceau.charge).unwrap(), t);
}

#[test]
fn tete_perimee_refusee_registre_gele() {
    let a = ancres(&[&journal()]);
    let pol = PolitiqueTete::journal_cog();
    let o = sceller_tete(&tete(1, 3, T0));
    // Dans la fenêtre (y compris tolérance) : accepté.
    verifier_tete(&o, &a, &pol, &REG, T0 + AGE_MAX_TETE_DEFAUT_SECS + TOLERANCE_HORLOGE_SECS).unwrap();
    // Au-delà : registre gelé.
    let e = verifier_tete(&o, &a, &pol, &REG, T0 + AGE_MAX_TETE_DEFAUT_SECS + TOLERANCE_HORLOGE_SECS + 1).unwrap_err();
    assert_eq!(e, ErreurRegistre::TeteGelee { emise_a: T0, age_max_secs: AGE_MAX_TETE_DEFAUT_SECS });
    // Politique plus stricte que la tête : c'est elle qui s'applique.
    let stricte = PolitiqueTete::nouvelle(1, 3600).unwrap();
    verifier_tete(&o, &a, &stricte, &REG, T0 + 3600).unwrap();
    assert_eq!(
        verifier_tete(&o, &a, &stricte, &REG, T0 + 3600 + TOLERANCE_HORLOGE_SECS + 1).unwrap_err(),
        ErreurRegistre::TeteGelee { emise_a: T0, age_max_secs: 3600 }
    );
    // Tête plus stricte que la politique.
    let mut courte = tete(1, 3, T0);
    courte.validite_max_secs = 600;
    let o = sceller_tete(&courte);
    assert!(matches!(
        verifier_tete(&o, &a, &pol, &REG, T0 + 600 + TOLERANCE_HORLOGE_SECS + 1),
        Err(ErreurRegistre::TeteGelee { age_max_secs: 600, .. })
    ));
    // Tête datée du futur : refus.
    assert_eq!(
        verifier_tete(&o, &a, &pol, &REG, T0 - TOLERANCE_HORLOGE_SECS - 1).unwrap_err(),
        ErreurRegistre::Sceau(ErreurSceau::PasEncoreValide)
    );
}

#[test]
fn validite_et_politique_bornees() {
    assert_eq!(PolitiqueTete::nouvelle(0, 60).unwrap_err(), ErreurRegistre::PolitiqueInvalide);
    assert_eq!(PolitiqueTete::nouvelle(9, 60).unwrap_err(), ErreurRegistre::PolitiqueInvalide);
    assert_eq!(PolitiqueTete::nouvelle(1, 0).unwrap_err(), ErreurRegistre::PolitiqueInvalide);
    assert_eq!(
        PolitiqueTete::nouvelle(1, AGE_MAX_TETE_PLAFOND_SECS + 1).unwrap_err(),
        ErreurRegistre::PolitiqueInvalide
    );
    assert_eq!(PolitiqueTete::canal_officiel().seuil(), 2);
    let a = ancres(&[&journal()]);
    for validite in [0, AGE_MAX_TETE_PLAFOND_SECS + 1, u64::MAX] {
        let mut t = tete(1, 3, T0);
        t.validite_max_secs = validite;
        assert_eq!(
            verifier_tete(&sceller_tete(&t), &a, &PolitiqueTete::journal_cog(), &REG, T0).unwrap_err(),
            ErreurRegistre::Sceau(ErreurSceau::ValeurInvalide(6))
        );
    }
    // Arbre vide : racine imposée.
    let mut t = tete(1, 0, T0);
    verifier_tete(&sceller_tete(&t), &a, &PolitiqueTete::journal_cog(), &REG, T0).unwrap();
    t.racine = [0; 32];
    assert_eq!(
        verifier_tete(&sceller_tete(&t), &a, &PolitiqueTete::journal_cog(), &REG, T0).unwrap_err(),
        ErreurRegistre::Sceau(ErreurSceau::ValeurInvalide(3))
    );
}

#[test]
fn seuil_2_sur_2_du_canal_officiel() {
    let (j, e) = (journal(), editeur());
    let a = ancres(&[&j, &e]);
    let pol = PolitiqueTete::canal_officiel();
    let t = tete(1, 4, T0);
    let simple = sceller(&t, &j).unwrap();
    assert_eq!(
        verifier_tete(&simple, &a, &pol, &REG, T0).unwrap_err(),
        ErreurRegistre::Sceau(ErreurSceau::SeuilNonAtteint { requis: 2, obtenu: 1 })
    );
    let double = cosigner::<TeteRegistre>(&simple, &e).unwrap();
    let v = verifier_tete(&double, &a, &pol, &REG, T0).unwrap();
    assert_eq!(v.verifie().signataires().len(), 2);
    // Autre registre : refus.
    assert_eq!(verifier_tete(&double, &a, &pol, &[0; 32], T0).unwrap_err(), ErreurRegistre::RegistreInattendu);
}

#[test]
fn monotonie_et_equivoque() {
    let a = ancres(&[&journal()]);
    let pol = PolitiqueTete::journal_cog();
    let t5 = tete(5, 8, T0);
    let mut etat = etat_initial(&t5);
    let o5 = sceller_tete(&t5);

    // Même tête : identique, état inchangé.
    assert_eq!(etat.verifier_et_accepter(&o5, &a, &pol, T0, None).unwrap(), Acceptation::Identique);

    // Séquence en recul : retour en arrière.
    let ancienne = sceller_tete(&tete(4, 6, T0 - 10));
    assert_eq!(etat.verifier_et_accepter(&ancienne, &a, &pol, T0, None).unwrap_err(), ErreurRegistre::RetourArriere);

    // Même séquence, racine différente : équivoque avec les deux sceaux.
    let mut fourche = t5.clone();
    fourche.racine = hacher_feuille(b"fourche");
    let of = sceller_tete(&fourche);
    match etat.verifier_et_accepter(&of, &a, &pol, T0, None).unwrap_err() {
        ErreurRegistre::Equivoque(p) => {
            assert_eq!(p.acceptee, o5);
            assert_eq!(p.contradictoire, of);
        }
        autre => panic!("attendu équivoque, obtenu {autre:?}"),
    }

    // Séquence supérieure, taille en recul : équivoque.
    let recul = sceller_tete(&tete(6, 7, T0 + 1));
    assert!(matches!(etat.verifier_et_accepter(&recul, &a, &pol, T0 + 1, None), Err(ErreurRegistre::Equivoque(_))));

    // Même taille, racine différente, séquence supérieure : équivoque.
    let mut meme_taille = tete(6, 8, T0 + 1);
    meme_taille.racine = hacher_feuille(b"autre histoire");
    let o = sceller_tete(&meme_taille);
    assert!(matches!(etat.verifier_et_accepter(&o, &a, &pol, T0 + 1, None), Err(ErreurRegistre::Equivoque(_))));

    // Horodatage en recul : retour en arrière.
    let o = sceller_tete(&tete(6, 8, T0 - 1));
    assert_eq!(etat.verifier_et_accepter(&o, &a, &pol, T0, None).unwrap_err(), ErreurRegistre::RetourArriere);

    // Re-signature de fraîcheur (même arbre, séquence +1) : sans preuve.
    let o6 = sceller_tete(&tete(6, 8, T0 + 100));
    assert_eq!(etat.verifier_et_accepter(&o6, &a, &pol, T0 + 100, None).unwrap(), Acceptation::Nouvelle);
    assert_eq!(etat.tete().sequence, 6);

    // Croissance : preuve de cohérence exigée et vérifiée.
    let o7 = sceller_tete(&tete(7, 12, T0 + 200));
    assert_eq!(etat.verifier_et_accepter(&o7, &a, &pol, T0 + 200, None).unwrap_err(), ErreurRegistre::CoherenceManquante);
    let mauvaise = PreuveCoherence::generer(&arbre(12), 7).unwrap();
    assert_eq!(
        etat.verifier_et_accepter(&o7, &a, &pol, T0 + 200, Some(&mauvaise)).unwrap_err(),
        ErreurRegistre::PreuveInvalide
    );
    let mut reecrit = arbre(12);
    reecrit[0] = hacher_feuille(b"substitution");
    let mut t_reecrit = tete(7, 12, T0 + 200);
    t_reecrit.racine = racine(&reecrit);
    let fausse = PreuveCoherence::generer(&reecrit, 8).unwrap();
    assert_eq!(
        etat.verifier_et_accepter(&sceller_tete(&t_reecrit), &a, &pol, T0 + 200, Some(&fausse)).unwrap_err(),
        ErreurRegistre::PreuveInvalide
    );
    assert_eq!(etat.tete().sequence, 6, "état inchangé après refus");
    let bonne = PreuveCoherence::generer(&arbre(12), 8).unwrap();
    assert_eq!(etat.verifier_et_accepter(&o7, &a, &pol, T0 + 200, Some(&bonne)).unwrap(), Acceptation::Nouvelle);
    assert_eq!(etat.tete().taille, 12);

    // Tête périmée présentée : refus anti-gel, même avec séquence supérieure.
    let o8 = sceller_tete(&tete(8, 12, T0 + 300));
    assert!(matches!(
        etat.verifier_et_accepter(&o8, &a, &pol, T0 + 300 + 2 * AGE_MAX_TETE_DEFAUT_SECS, None),
        Err(ErreurRegistre::TeteGelee { .. })
    ));
}

#[test]
fn amorcage_depuis_arbre_vide_sans_preuve() {
    let a = ancres(&[&journal()]);
    let pol = PolitiqueTete::journal_cog();
    let mut etat = etat_initial(&tete(0, 0, T0));
    let o = sceller_tete(&tete(1, 3, T0 + 1));
    assert_eq!(etat.verifier_et_accepter(&o, &a, &pol, T0 + 1, None).unwrap(), Acceptation::Nouvelle);
}

#[test]
fn etat_persistant_et_inclusion() {
    let a = ancres(&[&journal()]);
    let pol = PolitiqueTete::journal_cog();
    let t = tete(3, 9, T0);
    let etat = etat_initial(&t);
    let octets = etat.encoder();
    assert_eq!(&octets[..5], b"MSRG\x01");
    let relu = EtatRegistre::decoder(&octets, &a, &pol).unwrap();
    assert_eq!(relu, etat);
    // Rechargé même périmé (la fraîcheur se contrôle à l'usage).
    assert!(matches!(relu.exiger_fraicheur(&pol, T0 + 10 * AGE_MAX_TETE_DEFAUT_SECS), Err(ErreurRegistre::TeteGelee { .. })));

    // État altéré ou ancre inconnue : refus.
    let mut altere = octets.clone();
    let n = altere.len();
    altere[n - 1] ^= 1;
    assert!(EtatRegistre::decoder(&altere, &a, &pol).is_err());
    assert!(EtatRegistre::decoder(&octets, &AncresDeConfiance::new(), &pol).is_err());
    assert_eq!(EtatRegistre::decoder(&octets[..n - 1], &a, &pol).unwrap_err(), ErreurRegistre::EtatInvalide);
    assert_eq!(EtatRegistre::decoder(b"MSRG", &a, &pol).unwrap_err(), ErreurRegistre::EtatInvalide);

    // Inclusion vérifiée contre la tête retenue, seulement si fraîche.
    let feuilles = arbre(9);
    let p = PreuveInclusion::generer(&feuilles, 4).unwrap();
    relu.verifier_inclusion(&feuilles[4], &p, &pol, T0 + 10).unwrap();
    assert!(relu.verifier_inclusion(&feuilles[5], &p, &pol, T0 + 10).is_err());
    assert!(matches!(
        relu.verifier_inclusion(&feuilles[4], &p, &pol, T0 + 10 * AGE_MAX_TETE_DEFAUT_SECS),
        Err(ErreurRegistre::TeteGelee { .. })
    ));
    let autre_taille = PreuveInclusion::generer(&arbre(10), 4).unwrap();
    assert_eq!(
        relu.verifier_inclusion(&feuilles[4], &autre_taille, &pol, T0 + 10).unwrap_err(),
        ErreurRegistre::PreuveInvalide
    );
}
