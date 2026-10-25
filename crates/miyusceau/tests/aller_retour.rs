//! Propriété « aller-retour » (encodeur ⇔ décodeur), complément stable de la
//! cible de fuzz `aller_retour` : pour toute valeur que l'encodeur accepte,
//! `decoder(encoder(x)) == x` ; toute valeur que l'encodeur refuse est aussi
//! refusée par le décodeur (forgée sans contrôle). Un émetteur ne peut donc
//! jamais signer ce que les vérificateurs refusent.
//!
//! Valeurs tirées par SplitMix64 à graine fixe (reproductible, sans
//! dépendance), biaisées vers les bornes des invariants.

#![allow(clippy::many_single_char_names)]

use miyusceau::merkle::MAX_CHEMIN;
use miyusceau::*;

const T0: u64 = 1_800_000_000;
const ITERATIONS: usize = 2_000;

struct SplitMix(u64);

impl SplitMix {
    fn suivant(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn sous(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.suivant() % n as u64) as usize
        }
    }

    fn b32(&mut self) -> [u8; 32] {
        let mut o = [0u8; 32];
        for x in &mut o {
            *x = self.suivant() as u8;
        }
        o
    }

    /// Entier biaisé vers les bornes de `bornes` (et leurs voisins).
    fn u64_borne(&mut self, bornes: &[u64]) -> u64 {
        if self.sous(4) == 0 {
            return self.suivant();
        }
        let b = bornes[self.sous(bornes.len())];
        match self.sous(3) {
            0 => b.wrapping_sub(1),
            1 => b,
            _ => b.wrapping_add(1),
        }
    }
}

fn tete_aleatoire(g: &mut SplitMix) -> TeteRegistre {
    let taille = if g.sous(3) == 0 { 0 } else { g.u64_borne(&[0, 1, u64::MAX]) };
    let racine = if g.sous(2) == 0 { racine_vide() } else { g.b32() };
    TeteRegistre {
        registre: g.b32(),
        taille,
        racine,
        sequence: g.u64_borne(&[0, u64::MAX]),
        emise_a: g.u64_borne(&[0, T0, u64::MAX]),
        validite_max_secs: g.u64_borne(&[0, 1, AGE_MAX_TETE_DEFAUT_SECS, AGE_MAX_TETE_PLAFOND_SECS, u64::MAX]),
    }
}

/// Charge TLV écrite champ par champ, sans les contrôles de l'encodeur.
fn charge_tete_brute(t: &TeteRegistre) -> Vec<u8> {
    let mut e = EcrivainTlv::new();
    e.b32(1, &t.registre)
        .unwrap()
        .u64(2, t.taille)
        .unwrap()
        .b32(3, &t.racine)
        .unwrap()
        .u64(4, t.sequence)
        .unwrap()
        .u64(5, t.emise_a)
        .unwrap()
        .u64(6, t.validite_max_secs)
        .unwrap();
    e.finir()
}

#[test]
fn tete_registre_aller_retour() {
    let mut g = SplitMix(0xa11e_0001);
    let cle = CleSignature::depuis_graine(&[41; 32]);
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let (mut acceptees, mut refusees) = (0usize, 0usize);
    for _ in 0..ITERATIONS {
        let t = tete_aleatoire(&mut g);
        let mut e = EcrivainTlv::new();
        let encodage = t.encoder(&mut e).map(|()| e.finir());
        let construite =
            TeteRegistre::nouvelle(t.registre, t.taille, t.racine, t.sequence, t.emise_a, t.validite_max_secs);
        // Le constructeur et l'encodeur appliquent exactement les mêmes invariants.
        assert_eq!(construite.as_ref().err(), encodage.as_ref().err(), "{t:?}");
        assert_eq!(t.valider().err().as_ref(), encodage.as_ref().err());
        match encodage {
            Ok(charge) => {
                acceptees += 1;
                assert_eq!(charge, charge_tete_brute(&t));
                assert_eq!(decoder_titre::<TeteRegistre>(&charge).unwrap(), t);
                // Scellée puis vérifiée à son instant d'émission : relue à l'identique.
                let sceau = sceller(&t, &cle).unwrap();
                let v = verifier::<TeteRegistre>(&sceau, &ancres, t.emise_a).unwrap();
                assert_eq!(v.titre(), &t);
            }
            Err(erreur) => {
                refusees += 1;
                assert_eq!(sceller(&t, &cle).unwrap_err(), erreur);
                assert_eq!(decoder_titre::<TeteRegistre>(&charge_tete_brute(&t)).unwrap_err(), erreur);
            }
        }
    }
    assert!(acceptees > ITERATIONS / 10 && refusees > ITERATIONS / 10, "{acceptees} / {refusees}");
}

const ALGORITHMES: [Algorithme; 3] = [Algorithme::Ed25519, Algorithme::MlDsa65, Algorithme::HybrideEd25519MlDsa65];

fn sceau_aleatoire(g: &mut SplitMix) -> Sceau {
    let algorithme = ALGORITHMES[g.sous(3)];
    let taille_charge = match g.sous(4) {
        0 => TAILLE_MAX_CHARGE + g.sous(2),
        1 => 0,
        _ => g.sous(200),
    };
    let charge = (0..taille_charge).map(|_| g.suivant() as u8).collect();
    let nombre = if g.sous(6) == 0 { [0, MAX_SIGNATURES + 1][g.sous(2)] } else { 1 + g.sous(3) };
    let mut kids: Vec<Kid> = (0..nombre).map(|_| g.b32()).collect();
    if g.sous(3) != 0 {
        kids.sort_unstable();
    }
    if nombre >= 2 && g.sous(8) == 0 {
        kids[1] = kids[0];
    }
    let signatures = kids
        .into_iter()
        .map(|kid| {
            let alg = if g.sous(10) == 0 { ALGORITHMES[g.sous(3)] } else { algorithme };
            let longueur = if g.sous(10) == 0 { g.sous(100) } else { alg.taille_signature() };
            SignatureSceau { kid, algorithme: alg, octets: (0..longueur).map(|_| g.suivant() as u8).collect() }
        })
        .collect();
    let type_sceau = [TypeSceau::Passeport, TypeSceau::TeteRegistre, TypeSceau::Fiche][g.sous(3)];
    Sceau { algorithme, type_sceau, charge, signatures }
}

#[test]
fn sceau_aller_retour() {
    let mut g = SplitMix(0xa11e_0002);
    let (mut acceptes, mut refuses) = (0usize, 0usize);
    for _ in 0..ITERATIONS {
        let s = sceau_aleatoire(&mut g);
        if let Ok(octets) = s.encoder() {
            acceptes += 1;
            assert_eq!(octets, s.encoder_brut());
            assert_eq!(Sceau::decoder(&octets).unwrap(), s);
        } else {
            refuses += 1;
            assert!(Sceau::decoder(&s.encoder_brut()).is_err(), "encodeur trop strict : {s:?}");
        }
    }
    assert!(acceptes > ITERATIONS / 10 && refuses > ITERATIONS / 10, "{acceptes} / {refuses}");
}

#[test]
fn preuves_merkle_aller_retour() {
    let mut g = SplitMix(0xa11e_0003);
    for _ in 0..ITERATIONS {
        let longueur = if g.sous(2) == 0 { g.sous(MAX_CHEMIN + 1) } else { MAX_CHEMIN - 1 + g.sous(4) };
        let chemin: Vec<Empreinte> = (0..longueur).map(|_| g.b32()).collect();
        let (a, b) = (g.u64_borne(&[0, 1, u64::MAX]), g.u64_borne(&[0, 1, u64::MAX]));
        let inclusion = PreuveInclusion { index: a, taille: b, chemin: chemin.clone() };
        let coherence = PreuveCoherence { taille1: a, taille2: b, chemin };
        match (inclusion.encoder(), coherence.encoder()) {
            (Ok(oi), Ok(oc)) => {
                assert!(longueur <= MAX_CHEMIN);
                assert_eq!(PreuveInclusion::decoder(&oi).unwrap(), inclusion);
                assert_eq!(PreuveCoherence::decoder(&oc).unwrap(), coherence);
            }
            (Err(ei), Err(ec)) => {
                assert!(longueur > MAX_CHEMIN);
                assert_eq!((ei, ec), (ErreurRegistre::PreuveInvalide, ErreurRegistre::PreuveInvalide));
            }
            autre => panic!("encodeurs de preuves incohérents : {autre:?}"),
        }
    }
}

/// Titre dont l'encodeur n'est pas canonique (écrit un champ que son décodeur
/// ignore en le lisant comme optionnel puis en le normalisant) : `sceller`
/// doit le refuser plutôt que signer des octets relus autrement.
#[derive(Debug, PartialEq, Eq)]
struct NonCanonique(u64);

impl Titre for NonCanonique {
    const TYPE: TypeSceau = TypeSceau::Passeport;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        e.u64(1, self.0)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        // Normalisation : la valeur relue diffère de la valeur encodée.
        Ok(Self(l.exiger_u64(1)? | 1))
    }
}

/// Titre dont le décodeur impose un invariant que l'encodeur ignore.
struct Asymetrique(u64);

impl Titre for Asymetrique {
    const TYPE: TypeSceau = TypeSceau::Passeport;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        e.u64(1, self.0)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let v = l.exiger_u64(1)?;
        if v == 0 {
            return Err(ErreurSceau::ValeurInvalide(1));
        }
        Ok(Self(v))
    }
}

#[test]
fn sceller_refuse_tout_titre_non_relisible() {
    let cle = CleSignature::depuis_graine(&[42; 32]);
    // Titre défini hors de miyusceau : le contrôle d'aller-retour est générique.
    assert_eq!(sceller(&Asymetrique(0), &cle).unwrap_err(), ErreurSceau::ValeurInvalide(1));
    assert!(sceller(&Asymetrique(7), &cle).is_ok());
    assert_eq!(sceller(&NonCanonique(2), &cle).unwrap_err(), ErreurSceau::EncodageNonCanonique);
    assert_eq!(
        sceller_multi(&NonCanonique(2), &[&cle, &CleSignature::depuis_graine(&[43; 32])]).unwrap_err(),
        ErreurSceau::EncodageNonCanonique
    );
    let impair = sceller(&NonCanonique(3), &cle).unwrap();
    assert!(cosigner::<NonCanonique>(&impair, &CleSignature::depuis_graine(&[43; 32])).is_ok());
}
