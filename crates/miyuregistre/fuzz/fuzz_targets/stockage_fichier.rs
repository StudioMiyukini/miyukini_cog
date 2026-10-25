//! @id     miyuregistre_fuzz_stockage_fichier
//! @do     fuzzer_lecteur_fichier_journal
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — lecteur du fichier append-only `MRGJ` et chemin de récupération d'une fin tronquée (EX-10, C-26).

//! Entrée : `ajout (2, longueur big-endian) | contenu du fichier`. Le contenu
//! est écrit tel quel dans un fichier temporaire (tmpfs si disponible), puis
//! ouvert par `StockageFichier::ouvrir`.
//!
//! Propriétés : aucune panique ; refus ⇒ fichier intact ; ouverture ⇒ le
//! fichier est un préfixe de l'original, l'excédent retiré est compté, chaque
//! enregistrement se relit, une réouverture ne récupère plus rien et relit
//! les mêmes enregistrements ; un ajout survit à la réouverture.

#![no_main]

use std::fs;
use std::path::PathBuf;

use libfuzzer_sys::fuzz_target;
use miyuregistre::{Stockage, StockageFichier};

fn chemin() -> PathBuf {
    let base = PathBuf::from("/dev/shm");
    let dossier = if base.is_dir() { base } else { std::env::temp_dir() };
    dossier.join(format!("miyuregistre-fuzz-{}.mrgj", std::process::id()))
}

fn relire(s: &StockageFichier) -> Vec<Vec<u8>> {
    (0..s.taille()).map(|i| s.lire(i).expect("enregistrement relisible")).collect()
}

fuzz_target!(|octets: &[u8]| {
    if octets.len() < 2 {
        return;
    }
    let ajout_len = usize::from(u16::from_be_bytes([octets[0], octets[1]])) % 512;
    let contenu = &octets[2..];
    let p = chemin();
    fs::write(&p, contenu).expect("écriture du fichier temporaire");

    match StockageFichier::ouvrir(&p) {
        Err(_) => {
            assert_eq!(fs::read(&p).expect("relecture"), contenu, "fichier modifié malgré le refus");
        }
        Ok(s) => {
            let apres = fs::read(&p).expect("relecture");
            if contenu.len() >= 5 {
                assert!(contenu.starts_with(&apres), "le fichier ouvert n'est pas un préfixe de l'original");
                assert_eq!(s.octets_recuperes(), (contenu.len() - apres.len()) as u64, "récupération mal comptée");
            } else {
                assert_eq!(apres, b"MRGJ\x01", "entête non rétablie");
            }
            let enregistrements = relire(&s);
            assert!(enregistrements.iter().all(|e| !e.is_empty()));
            drop(s);

            // Réouverture : rien à récupérer, mêmes enregistrements.
            let mut s = StockageFichier::ouvrir(&p).expect("réouverture d'un fichier récupéré");
            assert_eq!(s.octets_recuperes(), 0, "récupération non idempotente");
            assert_eq!(relire(&s), enregistrements);

            // Ajout puis réouverture.
            let nouveau: Vec<u8> = (0..=ajout_len).map(|i| i as u8).collect();
            let index = s.ajouter(&nouveau).expect("ajout");
            assert_eq!(index, enregistrements.len() as u64);
            drop(s);
            let s = StockageFichier::ouvrir(&p).expect("réouverture après ajout");
            let mut attendus = enregistrements;
            attendus.push(nouveau);
            assert_eq!(relire(&s), attendus, "ajout perdu à la réouverture");
        }
    }
    let _ = fs::remove_file(&p);
});
