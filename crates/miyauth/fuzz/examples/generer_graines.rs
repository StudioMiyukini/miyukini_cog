//! @id     miyauth_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles de miyauth (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clés et COG fixes identiques à ceux des cibles (`fuzz_targets/commun.rs`).

#[path = "../fuzz_targets/commun.rs"]
mod commun;

use std::fs;
use std::path::Path;

use miyauth::http::{b64_encoder, formater_autorisation_passeport, formater_autorisation_visa, formater_pop};
use miyauth::{empreinte_titre, verifier_visa, IdentityRole, Portees, Visa};
use miyusceau::{sceller, EcrivainTlv, Titre};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn charge<T: Titre>(t: &T) -> Vec<u8> {
    let mut e = EcrivainTlv::new();
    t.encoder(&mut e).expect("charge");
    e.finir()
}

fn avec_mode(mode: u8, octets: &[u8]) -> Vec<u8> {
    [&[mode][..], octets].concat()
}

fn main() {
    let mut visa_scelle = Vec::new();
    let mut passeport_scelle = Vec::new();
    for (nom, etranger) in [("local", false), ("etranger", true)] {
        let p = commun::passeport(etranger);
        let scelle = commun::sceller_passeport(&p);
        ecrire("titres", &format!("passeport_{nom}"), &avec_mode(0, &scelle));
        ecrire("titres", &format!("charge_passeport_{nom}"), &avec_mode(1, &charge(&p)));
        let v = Visa {
            serie: [8; 16],
            passeport: empreinte_titre(&scelle).expect("empreinte"),
            emetteur_cog: commun::cog_local(),
            sujet: p.sujet,
            cle_appareil: p.cle_appareil,
            audience: commun::AUDIENCE.into(),
            role: if etranger { IdentityRole::Visitor } else { IdentityRole::Citizen },
            portees: Portees::new(["lecture"]).expect("portées"),
            pas_avant: commun::T0,
            pas_apres: commun::T0 + 600,
        };
        let visa = sceller(&v, &commun::cle_locale()).expect("visa");
        // Graines utiles : le visa et son passeport passent la vérification complète.
        let annuaire = commun::annuaire();
        let revocations = commun::SansRevocation;
        verifier_visa(&visa, &scelle, commun::AUDIENCE, &commun::politique(&annuaire, &revocations), commun::T0 + 60)
            .expect("graine de visa valide");
        ecrire("titres", &format!("visa_{nom}"), &avec_mode(2, &visa));
        ecrire("titres", &format!("charge_visa_{nom}"), &avec_mode(3, &charge(&v)));
        visa_scelle = visa;
        passeport_scelle = scelle;
    }

    ecrire("entetes_http", "pop", formater_pop(&[1; 16], &[2; 64]).as_bytes());
    ecrire("entetes_http", "visa", formater_autorisation_visa(&visa_scelle, &passeport_scelle).as_bytes());
    ecrire("entetes_http", "passeport", formater_autorisation_passeport(&passeport_scelle).as_bytes());
    let signee = format!("{}\n1800000000\n{}\n{}", b64_encoder(&[3; 32]), b64_encoder(&[4; 16]), b64_encoder(&[5; 64]));
    ecrire("entetes_http", "requete_signee", signee.as_bytes());
    ecrire("entetes_http", "cle_hex", "0303030303030303030303030303030303030303030303030303030303030303".as_bytes());
    ecrire("entetes_http", "usage", b"origin/forum-sync\nPOST\n/api/v1/sujets\n{\"titre\":\"x\"}");
}
