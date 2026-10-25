//! @id     miyurelais_fuzz_trame_nflog
//! @do     fuzzer_decodeur_trames_nflog
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodeur netlink NFLOG du relais des Vigies, conversion en ligne MDOU| et relève par une Vigie (EX-10, C-26).

//! Entrée : octets quelconques, découpés en datagrammes (premier octet =
//! nombre de coupures). Un datagramme netlink vient du noyau, mais un défaut
//! du noyau ou une socket mal liée ne doit jamais faire paniquer le relais.
//!
//! Propriétés : aucune panique ; un datagramme est accepté en entier ou
//! refusé ; toute ligne produite est ASCII imprimable, sans espace double ni
//! final ; chaque ligne relayée est comptée exactement une fois par la Vigie.

#![no_main]

use std::collections::BTreeMap;

use libfuzzer_sys::fuzz_target;
use miyurelais::{convertir, decoder_datagramme, ConfigRelais, Contexte, Conversion, Message, Relais, TransportEssai};
use miyuvigie::{CapteurId, ConfigVigie, ControleSortie, JeuRegles, PlanAdressage, Vigie, VoieId, Zone};

fn ligne_saine(l: &str) {
    assert!(l.bytes().all(|b| (0x20..0x7f).contains(&b)), "ligne non imprimable : {l:?}");
    assert!(!l.contains("  ") && !l.ends_with(' ') && !l.starts_with(' '), "espacement : {l:?}");
}

fuzz_target!(|octets: &[u8]| {
    let Some((&n, reste)) = octets.split_first() else { return };
    let coupures = usize::from(n % 8) + 1;
    let pas = reste.len() / coupures + 1;
    let datagrammes: Vec<&[u8]> = reste.chunks(pas).collect();

    for d in &datagrammes {
        if let Ok(messages) = decoder_datagramme(d) {
            for m in messages {
                if let Message::Paquet(p) = m {
                    assert!(p.charge.len() <= miyurelais::netlink::CHARGE_MAX);
                    let ctx = Contexte { hote: "fuzz", horodatage_us: 1_729_760_000_000_000, entree: "sv0", sortie: "sv1" };
                    if let Ok(Conversion::Ligne(l)) = convertir(&p, &ctx) {
                        ligne_saine(&l);
                    }
                    let _ = miyurelais::meta_flux(&p, 0, Some("sv0"));
                }
            }
        }
    }

    let capteur = CapteurId::new("fuzz").expect("capteur");
    let transport = TransportEssai {
        file: datagrammes.iter().map(|d| Ok(d.to_vec())).collect(),
        noms: BTreeMap::from([(1, "sv0".to_owned()), (2, "sv1".to_owned()), (3, "nom invalide".to_owned())]),
    };
    let config = ConfigRelais { capteur: capteur.clone(), hote: "fuzz".into(), groupes: vec![101, 110, 111], delai_muet_ms: 1 };
    let mut relais = Relais::new(config, transport).expect("relais").avec_horloge(Box::new(|| 1_729_760_000_000_000)).avec_meta_flux();
    let mut vigie = Vigie::new(ConfigVigie {
        capteur,
        zone: Zone::Franche,
        voies_couvertes: vec![VoieId::new("jayoffice").expect("voie")],
        plan: PlanAdressage::indicatif(),
        regles: JeuRegles::vide(),
        controle_sortie: ControleSortie::new(),
        capacite_dedup: 1024,
        fenetre_rejeu_ms: 60_000,
    });
    vigie.declarer_interface_voie("sv0", VoieId::new("jayoffice").expect("voie"));
    if let Ok(r) = miyuvigie::SourceJournal::relever_lignes(&mut relais) {
        for l in &r.lignes {
            ligne_saine(l);
        }
        let lot = vigie.analyser_journal(r.lignes.iter().map(String::as_str), 1_729_760_001_000);
        let c = lot.compteurs;
        assert_eq!(
            c.acceptees + c.hors_convention + c.mal_formees + c.incoherentes + c.doublons + c.rejeus,
            r.lignes.len() as u64,
            "ligne non comptée ou comptée deux fois"
        );
    }
    let _ = relais.prendre_meta_flux();
});
