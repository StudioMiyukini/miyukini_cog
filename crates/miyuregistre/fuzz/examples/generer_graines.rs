//! @id     miyuregistre_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles du registre (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.

use std::fs;
use std::path::Path;

use miyuregistre::{
    Categorie, Cible, Corps, EntreeRegistre, HorlogeConfiance, Publication, Stockage, StockageFichier, TempsMedian,
};
use miyusceau::{sceller, CleSignature, EcrivainTlv, Titre};

const T0: u64 = 1_800_000_000;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_mode(mode: u8, octets: &[u8]) -> Vec<u8> {
    let mut v = vec![mode];
    v.extend_from_slice(octets);
    v
}

fn main() {
    let cle = CleSignature::depuis_graine(&[41; 32]);
    let entrees = [
        (
            "publication",
            Corps::Publication(Publication {
                categorie: Categorie::Cores,
                composant: "strongfather".into(),
                version: 3,
                empreinte: [7; 32],
                taille: 12,
            }),
        ),
        ("revocation", Corps::Revocation { cible: Cible::Annonce([2; 32]), motif: "clé compromise".into() }),
        ("emission", Corps::Emission { certificat: [5; 32] }),
        ("decret", Corps::Decret { objet: "frontiere:ext".into(), empreinte: [6; 32] }),
        ("evenement", Corps::Evenement { objet: "e1".into(), empreinte: [8; 32] }),
    ];
    let mut scelles = Vec::new();
    for (nom, corps) in entrees {
        let e = EntreeRegistre { inscrite_a: T0, corps };
        let mut ecrivain = EcrivainTlv::new();
        e.encoder(&mut ecrivain).expect("charge");
        ecrire("entree", &format!("charge_{nom}"), &avec_mode(1, &ecrivain.finir()));
        let s = sceller(&e, &cle).expect("sceau");
        ecrire("entree", &format!("sceau_{nom}"), &avec_mode(0, &s));
        scelles.push(s);
    }
    let mut temps = TempsMedian::new();
    for k in 0..5 {
        temps.observer(T0 + k * 10);
    }
    ecrire("entree", "temps", &avec_mode(2, &temps.encoder()));
    let mut horloge = HorlogeConfiance::new();
    horloge.observer_tete(T0);
    horloge.maintenant(T0 + 5).expect("horloge");
    ecrire("entree", "horloge", &avec_mode(3, &horloge.encoder()));

    // Fichiers MRGJ : sain, fin tronquée, dernier enregistrement déchiré.
    let chemin = std::env::temp_dir().join(format!("miyuregistre-graines-{}.mrgj", std::process::id()));
    let _ = fs::remove_file(&chemin);
    {
        let mut s = StockageFichier::ouvrir(&chemin).expect("stockage");
        for e in scelles.iter().take(3) {
            s.ajouter(e).expect("ajout");
        }
        s.ajouter(b"abc").expect("ajout");
    }
    let sain = fs::read(&chemin).expect("lecture");
    let _ = fs::remove_file(&chemin);
    let avec_ajout = |c: &[u8]| {
        let mut v = vec![0, 16];
        v.extend_from_slice(c);
        v
    };
    ecrire("stockage_fichier", "sain", &avec_ajout(&sain));
    ecrire("stockage_fichier", "tronque", &avec_ajout(&sain[..sain.len() - 5]));
    let mut dechire = sain.clone();
    let n = dechire.len();
    dechire[n - 1] ^= 0xFF;
    ecrire("stockage_fichier", "dechire", &avec_ajout(&dechire));
    ecrire("stockage_fichier", "entete_partielle", &avec_ajout(b"MRG"));

    // Miroir : ajout, tête, synchronisation, inclusion…
    let ops: &[[u8; 3]] = &[[0, 1, 1], [1, 0, 1], [2, 0, 0], [0, 2, 2], [0, 3, 3], [1, 0, 2], [2, 0, 0], [6, 1, 0]];
    ecrire("miroir", "honnete", &ops.concat());
    let ops: &[[u8; 3]] = &[[0, 1, 1], [0, 2, 2], [1, 0, 1], [4, 1, 0], [4, 0, 1], [5, 3, 1], [3, 0, 0], [2, 0, 0]];
    ecrire("miroir", "fourbe", &ops.concat());
}
