//! @id     maia_memoire_fuzz_ident
//! @do     fuzzer_identifiants_memoire
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — identifiants hiérarchiques de la mémoire (`IdMemoire`, `Branche`, `Chemin`) : grammaire stricte, aller-retour (EX-51, C-26).

//! Entrée : texte quelconque (UTF-8 avec remplacement) → `IdMemoire::parse`,
//! `Branche::parse`, `Chemin::parse`, et désérialisation JSON des deux types.
//!
//! Propriétés : aucune panique ; texte accepté inchangé (`as_str`, `Display`,
//! JSON aller-retour) ; alphabet `[a-z0-9_.]` ; une note n'est jamais une
//! branche (et inversement) ; `IdMemoire::nouveau(branche, type, numéro)`
//! reconstruit l'identifiant ; une branche contient ses notes, jamais les
//! préfixes partiels ; `Chemin` = note sinon branche.

#![no_main]

use libfuzzer_sys::fuzz_target;
use maia_memoire::ident::{MAX_LONGUEUR_SEGMENT, MAX_SEGMENTS_BRANCHE};
use maia_memoire::{Branche, Chemin, IdMemoire, TypeUnite};

fn alphabet(texte: &str) {
    assert!(texte.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'.'));
    assert!(texte.split('.').all(|s| !s.is_empty() && s.len() <= MAX_LONGUEUR_SEGMENT));
}

fn controler_branche(b: &Branche, texte: &str) {
    assert_eq!(b.as_str(), texte);
    assert_eq!(b.to_string(), texte);
    alphabet(texte);
    assert!((1..=MAX_SEGMENTS_BRANCHE).contains(&b.profondeur()));
    assert_eq!(b.domaine(), texte.split('.').next().unwrap_or_default());
    for n in 0..=b.profondeur() + 1 {
        let t = b.tronquer(n);
        assert_eq!(Branche::parse(t.as_str()).as_ref(), Ok(&t), "troncature invalide");
        assert!(texte.starts_with(t.as_str()));
    }
    let json = serde_json::to_string(b).expect("json");
    assert_eq!(serde_json::from_str::<Branche>(&json).ok().as_ref(), Some(b));
    let note = IdMemoire::nouveau(b, TypeUnite::Fait, 1).expect("note de la branche");
    assert!(b.contient(&note));
    assert_eq!(&note.branche(), b);
}

fn controler_note(id: &IdMemoire, texte: &str) {
    assert_eq!(id.as_str(), texte);
    assert_eq!(id.to_string(), texte);
    alphabet(texte);
    assert!(id.numero() >= 1);
    let branche = id.branche();
    assert_eq!(Branche::parse(branche.as_str()).as_ref(), Ok(&branche), "branche d'une note invalide");
    assert!(branche.contient(id));
    assert_eq!(IdMemoire::nouveau(&branche, id.type_unite(), id.numero()).as_ref(), Ok(id));
    assert_eq!(id.domaine(), branche.domaine());
    let json = serde_json::to_string(id).expect("json");
    assert_eq!(serde_json::from_str::<IdMemoire>(&json).ok().as_ref(), Some(id));
}

fuzz_target!(|octets: &[u8]| {
    let texte = String::from_utf8_lossy(octets);
    let texte = texte.as_ref();
    let note = IdMemoire::parse(texte);
    let branche = Branche::parse(texte);
    assert!(!(note.is_ok() && branche.is_ok()), "« {texte} » à la fois note et branche");
    if let Ok(id) = &note {
        controler_note(id, texte);
    }
    if let Ok(b) = &branche {
        controler_branche(b, texte);
    }
    match Chemin::parse(texte) {
        Ok(Chemin::Note(n)) => assert_eq!(note.as_ref(), Ok(&n)),
        Ok(Chemin::Branche(b)) => assert_eq!(branche.as_ref(), Ok(&b)),
        Err(_) => assert!(note.is_err() && branche.is_err()),
    }
    // Désérialisation JSON d'une chaîne quelconque : mêmes règles que `parse`.
    let json = serde_json::to_string(texte).expect("json");
    assert_eq!(serde_json::from_str::<IdMemoire>(&json).is_ok(), note.is_ok());
    assert_eq!(serde_json::from_str::<Branche>(&json).is_ok(), branche.is_ok());
    // Une branche ne contient jamais une note d'un préfixe partiel de segment.
    if let (Some((tete, _)), Ok(b)) = (texte.rsplit_once('.'), &branche) {
        if let Ok(partielle) = Branche::parse(&tete[..tete.len().saturating_sub(1)]) {
            if let Ok(n) = IdMemoire::nouveau(b, TypeUnite::Resume, 7) {
                assert_eq!(partielle.contient(&n), n.as_str().starts_with(&format!("{}.", partielle.as_str())));
            }
        }
    }
});
