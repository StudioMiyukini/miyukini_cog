//! @id     miyuracine_tests_racine
//! @do     verifier_racine_et_passation
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — racine 2 sur 3, dérivation par usage, passation.

use std::collections::HashSet;

use miyuracine::{
    verifier_passation, ErreurRacine, NoeudSlip10, Passation, Racine, Usage, BUT_MIYUKINI_COG,
};
use zeroize::Zeroize;

/// Exposant 0 dans les tests (10 000 itérations) : la cérémonie réelle utilise
/// `EXPOSANT_DEFAUT`.
const E: u8 = 0;

fn racine_fixe() -> Racine {
    Racine::depuis_secret(&core::array::from_fn(|i| i as u8))
}

#[test]
fn deux_parts_quelconques_reconstituent_la_racine() {
    let racine = Racine::generer().unwrap();
    let parts = racine.partager(b"", E).unwrap();
    let t = parts.toutes();
    for (a, b) in [(0, 1), (0, 2), (1, 2), (1, 0), (2, 0), (2, 1)] {
        let r = Racine::recuperer(&[t[a], t[b]], b"").unwrap();
        assert_eq!(r.cle_publique_identite(), racine.cle_publique_identite());
        assert_eq!(r.cog_id(), racine.cog_id());
    }
    // Chaque part : 33 mots (256 bits), mêmes deux premiers mots (identifiant + exposant).
    let premiers: HashSet<String> = t
        .iter()
        .map(|p| p.split(' ').take(2).collect::<Vec<_>>().join(" "))
        .collect();
    assert_eq!(premiers.len(), 1);
    assert!(t.iter().all(|p| p.split(' ').count() == 33));
}

#[test]
fn une_part_seule_ou_trois_parts_refusees() {
    let racine = Racine::generer().unwrap();
    let parts = racine.partager(b"", E).unwrap();
    let t = parts.toutes();
    for part in t {
        assert_eq!(
            Racine::recuperer(&[part], b"").unwrap_err(),
            ErreurRacine::NombrePartsInvalide
        );
    }
    assert_eq!(
        Racine::recuperer(&t, b"").unwrap_err(),
        ErreurRacine::NombrePartsInvalide
    );
    assert_eq!(
        Racine::recuperer(&[], b"").unwrap_err(),
        ErreurRacine::AucunePart
    );
    assert_eq!(
        Racine::recuperer(&[t[1], t[1]], b"").unwrap_err(),
        ErreurRacine::IndicesDupliques
    );
}

#[test]
fn part_corrompue_refusee() {
    let racine = Racine::generer().unwrap();
    let parts = racine.partager(b"", E).unwrap();
    let t = parts.toutes();
    let mots: Vec<&str> = t[1].split(' ').collect();
    for position in [0, 4, 15, mots.len() - 1] {
        let mut alteres = mots.clone();
        alteres[position] = if mots[position] == "academic" {
            "acid"
        } else {
            "academic"
        };
        let corrompue = alteres.join(" ");
        assert_eq!(
            Racine::recuperer(&[t[0], &corrompue], b"").unwrap_err(),
            ErreurRacine::SommeControleInvalide,
            "mot {position} altéré"
        );
    }
    // Mot hors liste, part tronquée.
    let inconnu = t[1].replacen(mots[5], "zzzz", 1);
    assert_eq!(
        Racine::recuperer(&[t[0], &inconnu], b"").unwrap_err(),
        ErreurRacine::MotInconnu
    );
    let tronquee = mots[..mots.len() - 1].join(" ");
    assert!(Racine::recuperer(&[t[0], &tronquee], b"").is_err());
}

#[test]
fn parts_de_deux_racines_refusees() {
    let a = Racine::generer().unwrap().partager(b"", E).unwrap();
    let b = Racine::generer().unwrap().partager(b"", E).unwrap();
    let erreur = Racine::recuperer(&[a.toutes()[0], b.toutes()[1]], b"").unwrap_err();
    // Identifiants différents (ou, avec probabilité 2⁻¹⁵, empreinte invalide).
    assert!(
        matches!(
            erreur,
            ErreurRacine::PartsIncompatibles | ErreurRacine::EmpreinteInvalide
        ),
        "{erreur:?}"
    );
}

#[test]
fn schema_autre_que_deux_sur_trois_refuse() {
    use miyuracine::slip39::{generer_parts, Groupe};
    let ms = [9u8; 32];
    let trois_sur_cinq = generer_parts(
        1,
        &[Groupe {
            seuil: 3,
            nombre: 5,
        }],
        &ms,
        b"",
        E,
    )
    .unwrap();
    let g = &trois_sur_cinq[0];
    assert_eq!(
        Racine::recuperer(&[&g[0], &g[1]], b"").unwrap_err(),
        ErreurRacine::SchemaRacineInattendu
    );
    let court = generer_parts(
        1,
        &[Groupe {
            seuil: 2,
            nombre: 3,
        }],
        &[9u8; 16],
        b"",
        E,
    )
    .unwrap();
    assert_eq!(
        Racine::recuperer(&[&court[0][0], &court[0][1]], b"").unwrap_err(),
        ErreurRacine::SchemaRacineInattendu
    );
}

#[test]
fn phrase_secrete_change_la_racine_sans_erreur() {
    // Par conception SLIP-39, une mauvaise phrase donne une autre racine (non vérifiable).
    let racine = Racine::generer().unwrap();
    let parts = racine.partager(b"phrase", E).unwrap();
    let t = parts.toutes();
    assert_eq!(
        Racine::recuperer(&[t[0], t[2]], b"phrase")
            .unwrap()
            .cog_id(),
        racine.cog_id()
    );
    assert_ne!(
        Racine::recuperer(&[t[0], t[2]], b"autre").unwrap().cog_id(),
        racine.cog_id()
    );
}

#[test]
fn interoperabilite_vecteurs_officiels_deux_sur_trois_256_bits() {
    // Vecteurs officiels 23 (« Basic sharing 2-of-3 (256 bits) ») et 45
    // (« Extendable basic sharing 2-of-3 (256 bits) ») : exactement le schéma de la racine.
    let vecteurs: serde_json::Value =
        serde_json::from_str(include_str!("vecteurs/slip39-vectors.json")).unwrap();
    for n in [23usize, 45] {
        let v = &vecteurs[n - 1];
        let parts: Vec<&str> = v[1]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m.as_str().unwrap())
            .collect();
        let attendu = v[2].as_str().unwrap();
        let r = Racine::recuperer(&parts, b"TREZOR").unwrap();
        let ms: Vec<u8> = (0..32)
            .map(|i| u8::from_str_radix(&attendu[2 * i..2 * i + 2], 16).unwrap())
            .collect();
        let reference = Racine::depuis_secret(&ms.try_into().unwrap());
        assert_eq!(
            r.cle_publique_identite(),
            reference.cle_publique_identite(),
            "vecteur {n}"
        );
    }
}

#[test]
fn cles_par_usage_stables_et_distinctes() {
    let racine = racine_fixe();
    let mut vues = HashSet::new();
    for usage in Usage::TOUS {
        let generations: &[u32] = if usage.a_generations() {
            &[0, 1, 2]
        } else {
            &[0]
        };
        for &g in generations {
            let pk = racine.cle_publique(usage, g).unwrap();
            assert_eq!(pk, racine.cle_publique(usage, g).unwrap(), "stable");
            assert!(vues.insert(pk), "clé distincte : {usage} g{g}");
        }
    }
    assert_eq!(vues.len(), 13);
    // Le chemin est bien m/MCOG'/1'/0' depuis le maître SLIP-10 du secret.
    let secret: [u8; 32] = core::array::from_fn(|i| i as u8);
    let attendu = NoeudSlip10::maitre(&secret)
        .unwrap()
        .deriver(&[BUT_MIYUKINI_COG, 1, 0])
        .unwrap();
    assert_eq!(
        racine.cle_publique(Usage::Emission, 0).unwrap(),
        attendu.cle_signature().cle_publique()
    );
    // Identité : génération unique.
    assert_eq!(
        racine.cle(Usage::Identite, 1).unwrap_err(),
        ErreurRacine::GenerationInvalide
    );
    // La graine exportée est celle de la clé de signature.
    let mut graine = racine.graine(Usage::Journal, 0).unwrap();
    assert_eq!(
        miyusceau::CleSignature::depuis_graine(&graine).cle_publique(),
        racine.cle_publique(Usage::Journal, 0).unwrap()
    );
    graine.zeroize();
    assert_eq!(*graine, [0u8; 32]);
}

#[test]
fn passation_signee_par_les_deux_racines() {
    let ancienne = racine_fixe();
    let nouvelle = Racine::generer().unwrap();
    let p = Passation {
        ancienne_racine: ancienne.cle_publique_identite(),
        nouvelle_racine: nouvelle.cle_publique_identite(),
        emise_le: 1_793_000_000,
    };
    let cle_a = ancienne.cle(Usage::Identite, 0).unwrap();
    let cle_n = nouvelle.cle(Usage::Identite, 0).unwrap();
    let sceau = p.sceller(&cle_a, &cle_n).unwrap();
    assert_eq!(
        verifier_passation(&sceau, &p.ancienne_racine, 1_793_000_100).unwrap(),
        p
    );

    // Autre ancienne racine épinglée : refus.
    let autre = Racine::generer().unwrap().cle_publique_identite();
    assert!(verifier_passation(&sceau, &autre, 0).is_err());

    // Octet altéré : refus.
    let mut altere = sceau.clone();
    let dernier = altere.len() - 1;
    altere[dernier] ^= 1;
    assert!(verifier_passation(&altere, &p.ancienne_racine, 0).is_err());

    // Signée par l'ancienne seule (pas de preuve de possession de la nouvelle) : refus.
    let seule = miyusceau::sceller(&p, &cle_a).unwrap();
    assert!(verifier_passation(&seule, &p.ancienne_racine, 0).is_err());

    // Clés ne correspondant pas aux champs, racines identiques : refus au scellement.
    assert_eq!(
        p.sceller(&cle_n, &cle_a).unwrap_err(),
        ErreurRacine::PassationAncienneRacineInattendue
    );
    let identique = Passation {
        nouvelle_racine: p.ancienne_racine,
        ..p.clone()
    };
    assert_eq!(
        identique.sceller(&cle_a, &cle_a).unwrap_err(),
        ErreurRacine::PassationRacinesIdentiques
    );

    // Une clé d'usage (émission) ne peut pas se faire passer pour l'identité.
    let emission = ancienne.cle(Usage::Emission, 0).unwrap();
    let usurpee = miyusceau::sceller_multi(&p, &[&emission, &cle_n]).unwrap();
    assert!(verifier_passation(&usurpee, &p.ancienne_racine, 0).is_err());
}

#[test]
fn affichages_masques() {
    let racine = racine_fixe();
    let parts = racine.partager(b"", E).unwrap();
    let debut: String = parts.toutes()[0]
        .split(' ')
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    let d = format!(
        "{parts:?} {racine:?} {:?}",
        racine.noeud(Usage::Emission, 0).unwrap()
    );
    assert!(!d.contains(&debut));
    assert!(d.contains("<masqué>"));
}
