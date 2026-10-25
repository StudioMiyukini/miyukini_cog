//! @id     miyufederation_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles de miyufederation (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! COG fixes identiques à ceux des cibles (`fuzz_targets/commun.rs`).

#[path = "../fuzz_targets/commun.rs"]
mod commun;

use std::fs;
use std::path::Path;

use commun::{cle_etat, cog_id, cogs, federation, CogFuzz, MAINTENANT, T0};
use miyufederation::support::decouper;
use miyufederation::titres::PREUVE_EQUIVOQUE;
use miyufederation::{Accusation, AncreRegistre, Confirmation, Federation, Preuve, Recours};
use miyusceau::{cosigner, Sceau, TeteRegistre};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_mode(mode: &[u8], octets: &[u8]) -> Vec<u8> {
    [mode, octets].concat()
}

fn main() {
    let c = cogs();
    let a = c[0].tete(1, 4, [1; 32], T0).expect("tête");
    let b = c[0].tete(1, 4, [2; 32], T0).expect("tête");
    let preuve = Preuve::equivoque(a.clone(), b.clone());
    let accusation = Accusation::depuis_preuve(c[0].identite.cle_publique(), &preuve, T0 + 50)
        .sceller(&c[1].identite)
        .expect("accusation");
    let recours = Recours {
        accuse: c[0].identite.cle_publique(),
        type_preuve: PREUVE_EQUIVOQUE,
        empreinte_preuve: preuve.empreinte(),
        accusation: [3; 32],
        emise_a: T0 + 60,
    }
    .sceller(&c[1].identite)
    .expect("recours");
    let confirmation = Confirmation::nouvelle(c[2].identite.cle_publique(), c[1].identite.cle_publique(), T0)
        .sceller(&c[1].identite)
        .expect("confirmation");

    for (nom, sceau) in [
        ("candidature", &c[0].candidature),
        ("accusation", &accusation),
        ("recours", &recours),
        ("confirmation", &confirmation),
    ] {
        ecrire("titres", &format!("charge_{nom}"), &avec_mode(&[0], &Sceau::decoder(sceau).expect("sceau").charge));
        ecrire("titres", &format!("sceau_{nom}"), &avec_mode(&[1], sceau));
    }
    ecrire("titres", "accusation_quorum", &[2, 5, 1, 1, 1, 1, 1, 2, 5]);
    ecrire("titres", "accusation_sous_quorum", &[2, 4, 1, 1, 1, 1, 1, 2, 5]);
    ecrire("titres", "accusation_recul", &[2, 5, 1, 3, 0, 2, 1, 0, 2]);
    ecrire("titres", "confirmation_construite", &[3, 1, 2, 0, 10, 0, 0x50, 0]);

    ecrire("preuve", "equivoque", &avec_mode(&[0], &preuve.encoder()));
    let recul = Preuve::equivoque(c[0].tete(1, 4, [1; 32], T0).expect("tête"), c[0].tete(2, 3, [1; 32], T0).expect("tête"));
    ecrire("preuve", "recul", &avec_mode(&[0], &recul.encoder()));
    ecrire("preuve", "assemblee", &[&[1u8, a.len().min(255) as u8][..], &a, &b].concat());

    graines_etat();
    graines_vue();
}

const ANCRE: AncreRegistre = AncreRegistre { taille: 0, racine: [0; 32] };

/// Accusation au quorum (6 membres 1..=6) contre le COG 0.
fn accuser(f: &Federation, t: u64) -> (Vec<u8>, Preuve) {
    let c = cogs();
    let preuve = Preuve::equivoque(c[0].tete(1, 4, [1; 32], t).expect("tête"), c[0].tete(1, 4, [2; 32], t).expect("tête"));
    let mut acc = f.accuser(&cog_id(&c[0]), &preuve, &c[1].identite, t).expect("accusation");
    for m in &c[2..7] {
        acc = f.contresigner_accusation(&acc, &preuve, &m.identite, t).expect("cosignature");
    }
    (acc, preuve)
}

/// Corps `MFED` (sans entête ni MAC) d'un état scellé.
fn corps(etat: &[u8]) -> &[u8] {
    &etat[9..etat.len() - 32]
}

fn graines_etat() {
    let c = cogs();
    // Vue riche : candidat invité, carnet, note, tête témoignée, COG 0 suspendu.
    let mut f = federation();
    let invite = CogFuzz::nouveau(40);
    let t = MAINTENANT;
    f.recevoir_candidature(&invite.candidature, t).expect("candidature");
    for m in &c[1..4] {
        let conf = Confirmation::nouvelle(invite.identite.cle_publique(), m.identite.cle_publique(), t)
            .sceller(&m.identite)
            .expect("confirmation");
        f.recevoir_confirmation(&conf, t).expect("confirmation reçue");
    }
    f.actualiser(t).expect("actualiser");
    let mut tete = c[1].tete(1, 2, [5; 32], t).expect("tête");
    for m in &c[2..5] {
        tete = cosigner::<TeteRegistre>(&tete, &m.journal).expect("cosignature");
    }
    f.recevoir_tete(&cog_id(&c[1]), &tete, None, t).expect("tête témoignée");
    f.noter_contact(&cog_id(&c[2]), "reseau-a", t).expect("contact");
    f.noter_contact(&cog_id(&c[3]), "reseau-b", t).expect("contact");
    let (acc, preuve) = accuser(&f, t + 10);
    let suspension = f.recevoir_accusation(&acc, &preuve, t + 10).expect("suspension");
    let suspendu = f.sceller_etat(cle_etat(), &ANCRE).expect("scellé");
    ecrire("etat", "suspendu_enveloppe", &avec_mode(&[0], &suspendu));
    ecrire("etat", "suspendu_corps", &avec_mode(&[1], corps(&suspendu)));
    // Recours accepté par les 6 autres membres.
    let mut levee = f.clone();
    let mut r = levee.ouvrir_recours(&suspension.cog, &c[1].identite, t + 20).expect("recours");
    for m in &c[2..7] {
        r = levee.contresigner_recours(&r, &m.identite, t + 20).expect("cosignature");
    }
    levee.recevoir_recours(&r, t + 20).expect("recours accepté");
    ecrire("etat", "levee_corps", &avec_mode(&[1], corps(&levee.sceller_etat(cle_etat(), &ANCRE).expect("scellé"))));
    // Révocation à l'échéance des 72 h.
    let mut revoque = f.clone();
    revoque.actualiser(suspension.echeance()).expect("échéance");
    ecrire("etat", "revoque_corps", &avec_mode(&[1], corps(&revoque.sceller_etat(cle_etat(), &ANCRE).expect("scellé"))));
    // Version 0x02 : corps v3 à carnet vide, sans le compte final du carnet.
    let mut v2 = federation();
    v2.actualiser(t).expect("actualiser");
    let o = v2.sceller_etat(cle_etat(), &ANCRE).expect("scellé");
    let b = corps(&o);
    ecrire("etat", "v2_corps", &avec_mode(&[2], &b[..b.len() - 4]));
    // Assemblage de morceaux (pointeur + morceaux hexadécimaux).
    let (p, m) = decouper(&suspendu, 3).expect("découpage");
    let mut graine = vec![3u8];
    graine.extend_from_slice(&p.generation.to_be_bytes());
    graine.push(p.morceaux as u8);
    graine.extend_from_slice(&(p.taille as u32).to_be_bytes()[1..]);
    graine.extend_from_slice(&p.empreinte);
    graine.extend_from_slice(m.join(",").as_bytes());
    ecrire("etat", "morceaux", &graine);
}

fn graines_vue() {
    let c = cogs();
    let mut tete = c[0].tete(1, 2, [1; 32], MAINTENANT).expect("tête");
    for m in &c[1..4] {
        tete = cosigner::<TeteRegistre>(&tete, &m.journal).expect("cosignature");
    }
    ecrire("vue", "tete_temoignee", &avec_mode(&[0], &tete));
    // op | k | seq | taille | racine | décalage (2) | cog
    ecrire("vue", "suite_acceptee", &[1, 0, 3, 1, 2, 1, 0, 10, 0, 3, 3, 2, 3, 1, 0, 10, 0]);
    ecrire("vue", "suite_equivoque", &[1, 0, 4, 1, 2, 1, 0, 10, 2, 0, 4, 1, 2, 2, 0, 10, 2]);
    ecrire("vue", "suite_temoignage", &[1, 1, 0, 1, 2, 1, 0, 5, 1, 1, 0, 1, 2, 2, 0, 5, 1, 2, 2, 0, 0, 0, 0, 5, 3]);
}
