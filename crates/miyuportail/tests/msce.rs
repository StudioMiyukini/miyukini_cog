//! @id     miyuportail_tests_msce
//! @do     tester_format_msce_et_decodeurs
//! @role   test
//! @layer  toolkit
//! @human  miyuportail — vecteurs figés de la trame MSCE et accord strict des deux décodeurs (C-61, EX-46).

#![allow(clippy::many_single_char_names, clippy::format_collect)]

mod commun;

use commun::*;
use miyuportail::msce::{
    decoder_interne, decoder_trame, encoder_interne, kid, sceller, Contexte, TAILLE_MAX_TRAME, TYPE_DECLARATION,
};
use miyuportail::validation2;
use miyuportail::*;
use sha2::{Digest, Sha256};

fn hex(o: &[u8]) -> String {
    o.iter().map(|b| format!("{b:02x}")).collect()
}

fn ancres() -> Ancres {
    Ancres { emetteur: cle(7).verifying_key().to_bytes(), auteurs: vec![cle(9).verifying_key().to_bytes()] }
}

fn trame_reference() -> Vec<u8> {
    let s = id("jayoffice.docs");
    sceller(&declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples()), &[&cle(7), &cle(9)]).unwrap()
}

/// Vecteur figé : toute modification du format casse ce test (C-61).
#[test]
fn vecteur_trame_fige() {
    let t = trame_reference();
    // Entête : "MSCE" 0x02 0x01 0x08, puis longueur de charge.
    assert_eq!(&t[..7], b"MSCE\x02\x01\x08");
    assert_eq!(TYPE_DECLARATION, 0x08);
    let charge = &t[11..11 + u32::from_be_bytes([t[7], t[8], t[9], t[10]]) as usize];
    assert_eq!(
        hex(charge),
        [
            "010000000e6a61796f66666963652e646f6373", // 1 service « jayoffice.docs »
            "020000000101",                           // 2 sens entrante
            "03000000080000000000000001",             // 3 version du traité 1
            "0400000008000000000000000a",             // 4 type 10
            "05000000080000000000000001",             // 5 séquence 1
            "0600000008000000006ab13b80",             // 6 émise le 1 790 000 000
            "0700000019",                             // 7 champs (25 o)
            "0100000007626f6e6a6f7572",               //   1 texte « bonjour »
            "0200000008000000000000002a",             //   2 entier 42
        ]
        .concat()
    );
    assert_eq!(t[11 + charge.len()], 2, "deux signatures");
    assert_eq!(t.len(), 11 + charge.len() + 1 + 2 * 99);
    assert_eq!(hex(&Sha256::digest(&t)), EMPREINTE_REFERENCE);
}

/// SHA-256 de la trame de référence (clés `[7; 32]` façade, `[9; 32]` auteur ; Ed25519 déterministe).
const EMPREINTE_REFERENCE: &str = "bbfb440d1ac796938dd2a02ad79e9de355c731c7e3203983a3b6b065bd689257";

#[test]
fn vecteur_format_interne_fige() {
    let s = id("jayoffice.docs");
    let mut d = declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples());
    d.signataires = vec![[0xAA; 32]];
    let o = encoder_interne(&d);
    assert_eq!(&o[..4], b"MPI1");
    assert_eq!(decoder_interne(&o).unwrap(), d);
    let mut trop = o.clone();
    trop.push(0);
    assert!(decoder_interne(&trop).is_err());
}

#[test]
fn deux_decodeurs_d_accord_sur_la_reference() {
    let s = id("jayoffice.docs");
    let tr = traite();
    let a = ancres();
    let ctx = Contexte { service: &s, sens: Sens::Entrante, traite: &tr, ancres: &a };
    let t = trame_reference();
    let d1 = decoder_trame(&t, &ctx).unwrap();
    let d2 = validation2::valider(&t, &ctx).unwrap();
    assert_eq!(d1, d2);
    assert_eq!(d1.champs, champs_simples());
    let mut kids = vec![kid(&a.emetteur), kid(&a.auteurs[0])];
    kids.sort_unstable();
    assert_eq!(d1.signataires, kids);
}

/// Toute altération d'un octet, toute troncature et tout ajout sont refusés
/// par **les deux** décodeurs.
#[test]
fn toute_alteration_refusee_par_les_deux_decodeurs() {
    let s = id("jayoffice.docs");
    let tr = traite();
    let a = ancres();
    let ctx = Contexte { service: &s, sens: Sens::Entrante, traite: &tr, ancres: &a };
    let t = trame_reference();
    for i in 0..t.len() {
        let mut m = t.clone();
        m[i] ^= 0x01;
        assert!(decoder_trame(&m, &ctx).is_err(), "décodeur 1 accepte l'altération de l'octet {i}");
        assert!(validation2::valider(&m, &ctx).is_err(), "décodeur 2 accepte l'altération de l'octet {i}");
    }
    for n in 0..t.len() {
        assert!(decoder_trame(&t[..n], &ctx).is_err());
        assert!(validation2::valider(&t[..n], &ctx).is_err());
    }
    let mut plus = t.clone();
    plus.push(0);
    assert!(decoder_trame(&plus, &ctx).is_err());
    assert!(validation2::valider(&plus, &ctx).is_err());
}

fn refus_des_deux(t: &[u8], ctx: &Contexte<'_>) -> (RefusDecodage, RefusDecodage) {
    (decoder_trame(t, ctx).unwrap_err(), validation2::valider(t, ctx).unwrap_err())
}

#[test]
fn politique_de_signatures() {
    let s = id("jayoffice.docs");
    let tr = traite();
    let a = ancres();
    let ctx = Contexte { service: &s, sens: Sens::Entrante, traite: &tr, ancres: &a };
    let d = declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples());
    // Façade seule.
    let t = sceller(&d, &[&cle(7)]).unwrap();
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Signataires, RefusDecodage::Signataires));
    // Auteur inconnu.
    let t = sceller(&d, &[&cle(7), &cle(11)]).unwrap();
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Signataires, RefusDecodage::Signataires));
    // Deux auteurs sans façade.
    let a2 = Ancres { emetteur: a.emetteur, auteurs: vec![cle(9).verifying_key().to_bytes(), cle(11).verifying_key().to_bytes()] };
    let ctx2 = Contexte { ancres: &a2, ..ctx };
    let t = sceller(&d, &[&cle(9), &cle(11)]).unwrap();
    assert_eq!(refus_des_deux(&t, &ctx2), (RefusDecodage::Signataires, RefusDecodage::Signataires));
    // Trois signatures.
    let t = sceller(&d, &[&cle(7), &cle(9), &cle(11)]).unwrap();
    assert!(decoder_trame(&t, &ctx2).is_err());
    assert!(validation2::valider(&t, &ctx2).is_err());
}

#[test]
fn rupture_et_traite() {
    let s = id("jayoffice.docs");
    let tr = traite();
    let a = ancres();
    let ctx = Contexte { service: &s, sens: Sens::Entrante, traite: &tr, ancres: &a };
    let signer = |d: &Declaration| sceller(d, &[&cle(7), &cle(9)]).unwrap();

    // Mauvais service.
    let autre = id("autre");
    let t = signer(&declaration(&autre, Sens::Entrante, ENTRANT, 1, champs_simples()));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Service, RefusDecodage::Service));
    // Mauvais sens.
    let t = signer(&declaration(&s, Sens::Sortante, ENTRANT, 1, champs_simples()));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Sens, RefusDecodage::Sens));
    // Type sortant présenté en entrée.
    let t = signer(&declaration(&s, Sens::Entrante, SORTANT, 1, champs_simples()));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::TypeDeclaration, RefusDecodage::TypeDeclaration));
    // Version de traité.
    let mut d = declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples());
    d.version_traite = 2;
    assert_eq!(refus_des_deux(&signer(&d), &ctx), (RefusDecodage::VersionTraite, RefusDecodage::VersionTraite));
    // Champ hors traité.
    let mut c = champs_simples();
    c.push(Champ { tag: 9, valeur: Valeur::Entier(1) });
    let t = signer(&declaration(&s, Sens::Entrante, ENTRANT, 1, c));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Champ, RefusDecodage::Champ));
    // Champ obligatoire manquant.
    let t = signer(&declaration(&s, Sens::Entrante, ENTRANT, 1, vec![Champ { tag: 2, valeur: Valeur::Entier(1) }]));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Champ, RefusDecodage::Champ));
    // Texte trop long.
    let t = signer(&declaration(&s, Sens::Entrante, ENTRANT, 1, vec![Champ { tag: 1, valeur: Valeur::Texte("x".repeat(129)) }]));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Champ, RefusDecodage::Champ));
    // Caractère de contrôle.
    let t = signer(&declaration(&s, Sens::Entrante, ENTRANT, 1, vec![Champ { tag: 1, valeur: Valeur::Texte("a\u{1b}[31m".into()) }]));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Valeur, RefusDecodage::Valeur));
    // Nature incohérente : entier sur 4 octets (Octets dans un champ Entier).
    let t = signer(&declaration(
        &s,
        Sens::Entrante,
        ENTRANT,
        1,
        vec![Champ { tag: 1, valeur: Valeur::Texte("a".into()) }, Champ { tag: 2, valeur: Valeur::Octets(vec![0; 4]) }],
    ));
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Valeur, RefusDecodage::Valeur));
}

#[test]
fn bornes_de_taille() {
    let s = id("jayoffice.docs");
    let tr = traite();
    let a = ancres();
    let ctx = Contexte { service: &s, sens: Sens::Entrante, traite: &tr, ancres: &a };
    let enorme = vec![0u8; TAILLE_MAX_TRAME + 1];
    assert_eq!(refus_des_deux(&enorme, &ctx), (RefusDecodage::Taille, RefusDecodage::Taille));
    // Longueur de charge annoncée au-delà de 64 Kio.
    let mut t = b"MSCE\x02\x01\x08".to_vec();
    t.extend_from_slice(&(65_537u32).to_be_bytes());
    assert_eq!(refus_des_deux(&t, &ctx), (RefusDecodage::Taille, RefusDecodage::Taille));
    // Sceau d'identité (type passeport) : hors liste.
    let mut p = trame_reference();
    p[6] = 0x01;
    assert_eq!(refus_des_deux(&p, &ctx), (RefusDecodage::TypeTrame, RefusDecodage::TypeTrame));
    // Algorithme post-quantique réservé : refusé.
    let mut q = trame_reference();
    q[5] = 0x02;
    assert_eq!(refus_des_deux(&q, &ctx), (RefusDecodage::Algorithme, RefusDecodage::Algorithme));
}

/// `miyusceau` refuse une déclaration de voie (type inconnu) : aucune confusion avec un titre d'identité.
#[test]
fn miyusceau_refuse_le_type_declaration() {
    assert!(miyusceau::Sceau::decoder(&trame_reference()).is_err());
}

// ---------------------------------------------------------------- cosignature (lot 5)

#[test]
fn cosignature_externe_identique_au_scellement_conjoint() {
    use ed25519_dalek::Signer;
    use miyuportail::msce::{encoder_charge, octets_signes, sceller_cosigne};
    let s = id("jayoffice.docs");
    let d = declaration(&s, Sens::Entrante, ENTRANT, 1, champs_simples());
    // L'auteur signe sur son appareil ; la façade ajoute sa clé de session.
    let sig = cle(9).sign(&octets_signes(&encoder_charge(&d).unwrap())).to_bytes();
    let cosigne = sceller_cosigne(&d, &[&cle(7)], &[(cle(9).verifying_key().to_bytes(), sig)]).unwrap();
    assert_eq!(cosigne, trame_reference(), "même trame canonique");
    let ctx = Contexte { service: &s, sens: Sens::Entrante, traite: &traite(), ancres: &ancres() };
    assert_eq!(decoder_trame(&cosigne, &ctx).unwrap().champs, champs_simples());
    // Cosignature fausse (autre charge) : aucune trame produite.
    let autre = declaration(&s, Sens::Entrante, ENTRANT, 2, champs_simples());
    let fausse = cle(9).sign(&octets_signes(&encoder_charge(&autre).unwrap())).to_bytes();
    assert_eq!(
        sceller_cosigne(&d, &[&cle(7)], &[(cle(9).verifying_key().to_bytes(), fausse)]),
        Err(RefusDecodage::Signature)
    );
    // Doublon (même clé locale et externe) et absence de clé locale : refus.
    let propre = cle(7).sign(&octets_signes(&encoder_charge(&d).unwrap())).to_bytes();
    assert_eq!(
        sceller_cosigne(&d, &[&cle(7)], &[(cle(7).verifying_key().to_bytes(), propre)]),
        Err(RefusDecodage::Signataires)
    );
    assert_eq!(sceller_cosigne(&d, &[], &[(cle(9).verifying_key().to_bytes(), sig)]), Err(RefusDecodage::Signataires));
}
