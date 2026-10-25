//! @id     maia_federee_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles de maia-federee (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clé d'émetteur fixe identique à celle des cibles (`fuzz_targets/commun.rs`).

#[path = "../fuzz_targets/commun.rs"]
mod commun;

use std::fs;
use std::path::Path;

use commun::{cle, fiche, fiche_verifiee};
use maia_federee::fiche::Reprise;
use maia_federee::{sceller_fiche, Audience, RequeteTypee, TypeFiche};
use miyusceau::Sceau;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_mode(mode: u8, octets: &[u8]) -> Vec<u8> {
    [&[mode][..], octets].concat()
}

fn main() {
    let simple = sceller_fiche(&fiche("Paris est la capitale de la France."), &cle()).expect("fiche");
    let source = fiche_verifiee("Lyon est une grande ville.").expect("source");
    let mut avec_reprise = fiche("Reprise : Paris est la capitale.");
    avec_reprise.id = "geo.reprise".into();
    avec_reprise.reprises = vec![Reprise::de(&source)];
    avec_reprise.audience = Audience::Nommes(vec![source.cog().clone()]);
    let reprise = sceller_fiche(&avec_reprise, &cle()).expect("fiche reprise");
    for (nom, sceau) in [("simple", &simple), ("reprise", &reprise)] {
        let charge = Sceau::decoder(sceau).expect("sceau").charge;
        ecrire("fiche", &format!("charge_{nom}"), &avec_mode(0, &charge));
        ecrire("fiche", &format!("sceau_{nom}"), &avec_mode(1, sceau));
        ecrire("fiche", &format!("resignee_{nom}"), &avec_mode(2, &charge));
    }

    let r = RequeteTypee::nouvelle("capitale de la France", &[TypeFiche::Fait, TypeFiche::Definition], Some("fr"), 5)
        .expect("requête");
    ecrire("requete", "requete", &avec_mode(0, &r.encoder().expect("requête")));
    ecrire("requete", "construite", &[&[1u8, 5, 1, 0b11][..], "capitale de la France".as_bytes()].concat());
    ecrire("requete", "construite_homoglyphe", &[&[1u8, 3, 0, 0][..], "Pаris l’été".as_bytes()].concat());

    let texte = "Paris est la capitale de la France. Lyon est une grande ville.";
    let sortie = r#"{"affirmations":[{"sujet":"Paris","predicat":"capitale de","objet":"France"},{"sujet":"Lyon","predicat":"est une grande","objet":"ville"},{"sujet":"Berlin","predicat":"capitale de","objet":"France"}]}"#;
    let long = (texte.len() as u16).to_be_bytes();
    ecrire("lecteur", "ancres", &[&long[..], texte.as_bytes(), sortie.as_bytes()].concat());
    ecrire("lecteur", "hors_schema", &[&long[..], texte.as_bytes(), br#"{"affirmations":[],"tool_calls":[]}"#].concat());
}
