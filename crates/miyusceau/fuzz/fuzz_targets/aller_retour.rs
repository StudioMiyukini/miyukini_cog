//! @id     miyusceau_fuzz_aller_retour
//! @do     fuzzer_aller_retour_encodeurs
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — symétrie encodeur / décodeur (tête de registre, sceau, preuves Merkle).

//! Entrée : `sélecteur (1) | octets de construction`. Une valeur structurée
//! est construite depuis l'entrée **sans passer par un décodeur**, puis :
//!
//! - si l'encodeur l'accepte : `decoder(encoder(x)) == x` (et, pour une tête,
//!   elle se scelle et se vérifie à son instant d'émission) ;
//! - s'il la refuse : le décodeur refuse aussi sa forme forgée sans contrôle
//!   (ni encodeur trop laxiste, ni encodeur trop strict).

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyusceau::merkle::MAX_CHEMIN;
use miyusceau::{
    decoder_titre, racine_vide, sceller, verifier, Algorithme, AncresDeConfiance, CleSignature, EcrivainTlv, Empreinte,
    NiveauConfiance, PreuveCoherence, PreuveInclusion, Sceau, SignatureSceau, TeteRegistre, Titre, TypeSceau,
    AGE_MAX_TETE_PLAFOND_SECS,
};

/// Lecture tolérante : octets manquants = zéros.
struct Curseur<'a>(&'a [u8]);

impl Curseur<'_> {
    fn octet(&mut self) -> u8 {
        let (premier, reste) = self.0.split_first().map_or((0, &[][..]), |(p, r)| (*p, r));
        self.0 = reste;
        premier
    }

    fn u64(&mut self) -> u64 {
        u64::from_be_bytes(std::array::from_fn(|_| self.octet()))
    }

    fn b32(&mut self) -> [u8; 32] {
        std::array::from_fn(|_| self.octet())
    }

    fn octets(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.octet()).collect()
    }
}

fn charge_tete_brute(t: &TeteRegistre) -> Vec<u8> {
    let mut e = EcrivainTlv::new();
    e.b32(1, &t.registre)
        .and_then(|e| e.u64(2, t.taille))
        .and_then(|e| e.b32(3, &t.racine))
        .and_then(|e| e.u64(4, t.sequence))
        .and_then(|e| e.u64(5, t.emise_a))
        .and_then(|e| e.u64(6, t.validite_max_secs))
        .expect("tags croissants");
    e.finir()
}

fn tete(drapeaux: u8, c: &mut Curseur<'_>) {
    let mut t = TeteRegistre {
        registre: c.b32(),
        taille: c.u64(),
        racine: c.b32(),
        sequence: c.u64(),
        emise_a: c.u64(),
        validite_max_secs: c.u64(),
    };
    // Rapproche le fuzzeur des bornes des invariants.
    if drapeaux & 0x08 != 0 {
        t.taille = 0;
    }
    if drapeaux & 0x10 != 0 {
        t.racine = racine_vide();
    }
    if drapeaux & 0x20 != 0 {
        t.validite_max_secs %= AGE_MAX_TETE_PLAFOND_SECS + 2;
    }
    let mut e = EcrivainTlv::new();
    let encodage = t.encoder(&mut e).map(|()| e.finir());
    let construite = TeteRegistre::nouvelle(t.registre, t.taille, t.racine, t.sequence, t.emise_a, t.validite_max_secs);
    assert_eq!(construite.as_ref().err(), encodage.as_ref().err(), "constructeur et encodeur divergent");
    match encodage {
        Ok(charge) => {
            assert_eq!(charge, charge_tete_brute(&t));
            assert_eq!(decoder_titre::<TeteRegistre>(&charge).expect("tête encodée relisible"), t);
            let cle = CleSignature::depuis_graine(&[31; 32]);
            let mut ancres = AncresDeConfiance::new();
            ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).expect("clé fixe valide");
            let sceau = sceller(&t, &cle).expect("tête encodable scellable");
            let v = verifier::<TeteRegistre>(&sceau, &ancres, t.emise_a).expect("tête scellée vérifiable");
            assert_eq!(v.titre(), &t);
        }
        Err(erreur) => {
            assert_eq!(sceller(&t, &CleSignature::depuis_graine(&[31; 32])).unwrap_err(), erreur);
            assert!(decoder_titre::<TeteRegistre>(&charge_tete_brute(&t)).is_err(), "encodeur trop strict");
        }
    }
}

fn algorithme(o: u8) -> Algorithme {
    match o % 3 {
        0 => Algorithme::Ed25519,
        1 => Algorithme::MlDsa65,
        _ => Algorithme::HybrideEd25519MlDsa65,
    }
}

fn sceau(drapeaux: u8, c: &mut Curseur<'_>) {
    let alg = algorithme(c.octet());
    let type_sceau = TypeSceau::try_from(c.octet() % 10).unwrap_or(TypeSceau::TeteRegistre);
    let longueur = usize::from(u16::from_be_bytes([c.octet(), c.octet()]));
    // Bit 0x08 : charge à la borne (64 Kio ± 1) ; sinon courte.
    let longueur = if drapeaux & 0x08 != 0 { 65_535 + longueur % 3 } else { longueur % 512 };
    let charge = c.octets(longueur);
    let nombre = usize::from(c.octet() % 10);
    let mut signatures = Vec::with_capacity(nombre);
    for _ in 0..nombre {
        let kid = c.b32();
        let alg_entree = if drapeaux & 0x10 != 0 { algorithme(c.octet()) } else { alg };
        let taille = if drapeaux & 0x20 != 0 { usize::from(c.octet()) } else { alg_entree.taille_signature() };
        signatures.push(SignatureSceau { kid, algorithme: alg_entree, octets: c.octets(taille) });
    }
    if drapeaux & 0x40 != 0 {
        signatures.sort_by_key(|s| s.kid);
    }
    let s = Sceau { algorithme: alg, type_sceau, charge, signatures };
    match s.encoder() {
        Ok(octets) => {
            assert_eq!(octets, s.encoder_brut());
            assert_eq!(Sceau::decoder(&octets).expect("sceau encodé relisible"), s);
        }
        Err(_) => assert!(Sceau::decoder(&s.encoder_brut()).is_err(), "encodeur de sceau trop strict"),
    }
}

fn preuves(c: &mut Curseur<'_>) {
    let (a, b) = (c.u64(), c.u64());
    let n = usize::from(c.octet() % 80);
    let chemin: Vec<Empreinte> = (0..n).map(|_| c.b32()).collect();
    let inclusion = PreuveInclusion { index: a, taille: b, chemin: chemin.clone() };
    let coherence = PreuveCoherence { taille1: a, taille2: b, chemin };
    match (inclusion.encoder(), coherence.encoder()) {
        (Ok(oi), Ok(oc)) => {
            assert!(n <= MAX_CHEMIN);
            assert_eq!(PreuveInclusion::decoder(&oi).expect("inclusion relisible"), inclusion);
            assert_eq!(PreuveCoherence::decoder(&oc).expect("cohérence relisible"), coherence);
        }
        (Err(_), Err(_)) => assert!(n > MAX_CHEMIN),
        _ => panic!("encodeurs de preuves incohérents"),
    }
}

fuzz_target!(|donnees: &[u8]| {
    let Some((&selecteur, reste)) = donnees.split_first() else {
        return;
    };
    let mut c = Curseur(reste);
    match selecteur % 3 {
        0 => tete(selecteur, &mut c),
        1 => sceau(selecteur, &mut c),
        _ => preuves(&mut c),
    }
});
