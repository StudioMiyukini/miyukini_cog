//! @id     miyuregistre_fuzz_miroir
//! @do     fuzzer_ingestion_miroir_registre
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — ingestion d'un miroir de registre : têtes et entrées non fiables, ordre quelconque (EX-10, C-26).

//! Un journal honnête (clé de tête fixe) évolue au gré de l'entrée ; un
//! dépôt non fiable rejoue, tronque ou altère têtes et entrées avant de les
//! remettre au miroir. Entrée : suite d'opérations de 3 octets `op | a | b`.
//!
//! Propriétés : aucune panique ; un refus ne modifie pas le miroir ; le
//! miroir ne contient **jamais** autre chose qu'un préfixe exact du journal
//! honnête ; toute entrée confirmée prouve son inclusion tant que la tête est
//! fraîche.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyuregistre::{Corps, EntreeRegistre, HorlogeConfiance, Journal, MiroirRegistre, StockageMemoire, SuiviTetes};
use miyusceau::{
    decoder_titre, sceller, AncresDeConfiance, CleSignature, NiveauConfiance, PolitiqueTete, Sceau, TeteRegistre,
};

const T0: u64 = 1_800_000_000;
const REGISTRE: [u8; 32] = [3; 32];

fn taille_tete(sceau: &[u8]) -> u64 {
    let s = Sceau::decoder(sceau).expect("tête émise décodable");
    decoder_titre::<TeteRegistre>(&s.charge).expect("tête émise").taille
}

fuzz_target!(|octets: &[u8]| {
    let cle_tete = CleSignature::depuis_graine(&[42; 32]);
    let cle_entree = CleSignature::depuis_graine(&[43; 32]);
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cle_tete.cle_publique(), NiveauConfiance::Souverain).expect("ancre");
    let suivi = SuiviTetes::new(REGISTRE, ancres, PolitiqueTete::journal_cog());
    let mut journal = Journal::ouvrir(REGISTRE, StockageMemoire::new(), StockageMemoire::new()).expect("journal");
    let mut miroir = MiroirRegistre::ouvrir(suivi, StockageMemoire::new()).expect("miroir");
    let mut horloge = HorlogeConfiance::new();
    let mut tetes: Vec<Vec<u8>> = Vec::new();
    let mut t = T0;

    for op in octets.as_chunks::<3>().0 {
        let (code, a, b) = (op[0], op[1], op[2]);
        let avant = miroir.taille();
        let resultat = match code % 7 {
            0 => {
                let e = EntreeRegistre {
                    inscrite_a: t,
                    corps: Corps::Evenement { objet: format!("e{a}"), empreinte: [b; 32] },
                };
                journal.ajouter(&sceller(&e, &cle_entree).expect("entrée")).expect("ajout au journal");
                continue;
            }
            1 => {
                t += u64::from(b) * 60 + 1;
                if let Ok(s) = journal.emettre_tete(&[&cle_tete], t, 86_400 * (1 + u64::from(a % 3))) {
                    tetes.push(s);
                }
                continue;
            }
            // Synchronisation honnête (tête choisie, éventuellement ancienne).
            2 | 3 => {
                let Some(tete) =
                    (if code % 7 == 2 { tetes.last() } else { tetes.get(usize::from(a) % tetes.len().max(1)) })
                else {
                    continue;
                };
                let n = taille_tete(tete);
                let m = miroir.taille();
                let entrees = if n > m { journal.entrees(m, n - m).expect("entrées") } else { Vec::new() };
                let maintenant = t + u64::from(b) * 3_600;
                miroir.synchroniser(tete, &entrees, &mut horloge, maintenant).map(|r| (r, n, m))
            }
            // Entrées altérées, tronquées ou en excès : refus obligatoire.
            4 => {
                let Some(tete) = tetes.last() else { continue };
                let n = taille_tete(tete);
                let m = miroir.taille();
                if n <= m {
                    continue;
                }
                let mut entrees = journal.entrees(m, n - m).expect("entrées");
                match b % 3 {
                    0 => {
                        let k = usize::from(a) % entrees.len();
                        let pos = usize::from(b) % entrees[k].len();
                        entrees[k][pos] ^= 1 | a;
                    }
                    1 => {
                        entrees.pop();
                    }
                    _ => entrees.push(vec![a; 1 + usize::from(b)]),
                }
                let r = miroir.synchroniser(tete, &entrees, &mut horloge, t);
                assert!(r.is_err(), "entrées altérées acceptées par le miroir");
                r.map(|r| (r, n, m))
            }
            // Tête altérée (octet quelconque).
            5 => {
                let Some(tete) = tetes.last() else { continue };
                let mut tete = tete.clone();
                let pos = usize::from(a) * 7 % tete.len();
                tete[pos] ^= 1 | b;
                let n = miroir.taille();
                miroir.synchroniser(&tete, &[], &mut horloge, t).map(|r| (r, n, n))
            }
            _ => {
                let index = u64::from(a);
                let r = miroir.verifier_inclusion(index, t + u64::from(b) * 3_600);
                if r.is_ok() {
                    assert!(index < miroir.taille(), "inclusion prouvée hors du miroir");
                }
                continue;
            }
        };
        match resultat {
            Ok((intervalle, n, m)) => {
                assert_eq!(intervalle, m..n);
                assert_eq!(miroir.taille(), n);
            }
            Err(_) => assert_eq!(miroir.taille(), avant, "miroir modifié malgré le refus"),
        }
        // Invariant : préfixe exact du journal honnête.
        assert!(miroir.taille() <= journal.taille());
        for i in 0..miroir.taille() {
            assert_eq!(miroir.entree(i).expect("entrée du miroir"), journal.entree(i).expect("entrée du journal"));
        }
    }
});
