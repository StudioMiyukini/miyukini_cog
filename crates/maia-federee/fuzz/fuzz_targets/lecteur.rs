//! @id     maia_federee_fuzz_lecteur
//! @do     fuzzer_sortie_maia_lecteur
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — sortie de MAIA-Lecteur (`lecteur::extraire`) : JSON strict, extraits ancrés dans la Fiche signée (EX-59, C-26).

//! Entrée : `long. texte (2, BE) | texte de la Fiche | sortie du lecteur`.
//! La Fiche type (relation signée « Paris | capitale de | France ») porte
//! le texte fuzzé ; le `ModeleLecteur` rend la sortie fuzzée telle quelle.
//!
//! Propriétés : aucune panique ; sortie trop longue ⇒ aucun extrait ;
//! au plus 8 extraits, sans doublon, termes normalisés d'au plus 120
//! caractères ; chaque extrait est ancré : égal (clé de comparaison) à une
//! relation signée, ou chacun de ses termes figure dans le texte normalisé.

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use maia_federee::lecteur::{MAX_EXTRAITS_PAR_FICHE, MAX_SORTIE_LECTEUR, MAX_TERME_EXTRAIT};
use maia_federee::normalisation::{cle_comparaison, normaliser_ligne};
use maia_federee::{extraire, Ancrage, ModeleLecteur, Relation, RequeteTypee};

struct Fixe(String);

impl ModeleLecteur for Fixe {
    fn lire(&self, _invite: &str) -> String {
        self.0.clone()
    }
}

fn cle(r: &Relation) -> (String, String, String) {
    (cle_comparaison(&r.sujet), cle_comparaison(&r.predicat), cle_comparaison(&r.objet))
}

fuzz_target!(|octets: &[u8]| {
    let [a, b, ref reste @ ..] = *octets else { return };
    let n = usize::from(u16::from_be_bytes([a, b])).min(reste.len());
    let (texte, sortie) = reste.split_at(n);
    let texte = String::from_utf8_lossy(texte);
    let texte = if texte.trim().is_empty() { "Paris est la capitale de la France." } else { texte.as_ref() };
    let Some(fiche) = commun::fiche_verifiee(texte) else { return };
    let sortie = String::from_utf8_lossy(sortie).into_owned();
    let requete = RequeteTypee::nouvelle("capitale", &[], None, 3).expect("requête");
    let extraits = extraire(&Fixe(sortie.clone()), &requete, &fiche);

    if sortie.len() > MAX_SORTIE_LECTEUR {
        assert!(extraits.is_empty(), "sortie trop longue analysée");
    }
    assert!(extraits.len() <= MAX_EXTRAITS_PAR_FICHE);
    let texte_cle = cle_comparaison(&fiche.fiche().texte);
    for (i, e) in extraits.iter().enumerate() {
        let r = &e.relation;
        assert!(!extraits[..i].iter().any(|x| x.relation == *r), "extrait en double");
        for t in [&r.sujet, &r.predicat, &r.objet] {
            assert!(!t.is_empty() && t.chars().count() <= MAX_TERME_EXTRAIT);
            assert_eq!(&normaliser_ligne(t), t, "terme non normalisé");
        }
        let c = cle(r);
        match e.ancrage {
            Ancrage::RelationSignee => {
                assert!(fiche.fiche().relations.iter().any(|s| cle(s) == c), "relation non signée : {r:?}");
            }
            Ancrage::Texte => {
                for t in [&c.0, &c.1, &c.2] {
                    assert!(t.chars().count() >= 2 && texte_cle.contains(t.as_str()), "terme absent du texte : {t:?}");
                }
            }
        }
    }
});
