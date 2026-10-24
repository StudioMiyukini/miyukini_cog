//! Vecteurs de test du format Sceau v1 (refus attendus = fail-closed).

#![allow(clippy::format_collect, clippy::needless_range_loop, clippy::items_after_statements)]

use miyusceau::*;
use sha2::{Digest, Sha256};

/// Titre de test : tags 1 (nom), 2 (pas_avant), 3 (pas_apres), 9 (note optionnelle).
#[derive(Debug, Clone, PartialEq)]
struct Essai {
    nom: String,
    pas_avant: u64,
    pas_apres: u64,
    note: Option<String>,
}

impl Titre for Essai {
    const TYPE: TypeSceau = TypeSceau::Passeport;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        e.texte(1, &self.nom)?.u64(2, self.pas_avant)?.u64(3, self.pas_apres)?;
        if let Some(n) = &self.note {
            e.texte(9, n)?;
        }
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let nom = l.exiger_texte(1)?.to_string();
        let pas_avant = l.exiger_u64(2)?;
        let pas_apres = l.exiger_u64(3)?;
        let note = l.optionnel(9)?.map(|v| String::from_utf8(v.to_vec())).transpose()
            .map_err(|_| ErreurSceau::ValeurInvalide(9))?;
        Ok(Self { nom, pas_avant, pas_apres, note })
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.pas_avant, self.pas_apres))
    }
}

/// Même charge, autre type (séparation de domaine).
#[derive(Debug)]
struct EssaiVisa(#[allow(dead_code)] Essai);
impl Titre for EssaiVisa {
    const TYPE: TypeSceau = TypeSceau::Visa;
    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        self.0.encoder(e)
    }
    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        Essai::decoder(l).map(Self)
    }
}

const T0: u64 = 1_800_000_000;

fn essai() -> Essai {
    Essai { nom: "Miyukini".into(), pas_avant: T0, pas_apres: T0 + 3600, note: None }
}

fn cle_fixe() -> CleSignature {
    CleSignature::depuis_graine(&[7u8; 32])
}

fn ancres_pour(cle: &CleSignature) -> AncresDeConfiance {
    let mut a = AncresDeConfiance::new();
    a.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
    a
}

/// Re-signe une charge arbitraire (pour tester le décodage canonique au-delà de la signature).
fn sceller_charge_brute(cle: &CleSignature, type_sceau: TypeSceau, charge: Vec<u8>) -> Vec<u8> {
    let mut m = b"miyukini/sceau/v1\0".to_vec();
    m.push(1);
    m.push(type_sceau as u8);
    m.extend_from_slice(&cle.kid());
    m.extend_from_slice(&(charge.len() as u32).to_be_bytes());
    m.extend_from_slice(&charge);
    use ed25519_dalek::Signer;
    let sk = ed25519_dalek::SigningKey::from_bytes(&cle.graine());
    let signature = sk.sign(&m).to_bytes();
    Sceau { type_sceau, kid: cle.kid(), charge, signature }.encoder()
}

#[test]
fn aller_retour_valide() {
    let cle = cle_fixe();
    let octets = sceller(&essai(), &cle).unwrap();
    let v = verifier::<Essai>(&octets, &ancres_pour(&cle), T0 + 10).unwrap();
    assert_eq!(v.titre(), &essai());
    assert_eq!(v.signataire(), &cle.kid());
    assert_eq!(v.niveau(), NiveauConfiance::Souverain);
}

#[test]
fn vecteur_de_reference_stable() {
    // Ed25519 est déterministe : ce vecteur fige le format Sceau v1.
    // Toute modification involontaire de l'encodage le fera échouer.
    let octets = sceller(&essai(), &cle_fixe()).unwrap();
    assert_eq!(&octets[0..6], b"MSCE\x01\x01");
    let empreinte: String = Sha256::digest(&octets).iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(empreinte, VECTEUR_REFERENCE, "format Sceau v1 modifié");
}

/// SHA-256 du sceau de référence (graine [7; 32], titre `essai()`), figé le 2026-10-24.
const VECTEUR_REFERENCE: &str = "dae5f536b405d1bc39228132d119bf4bb7f684a9608970bee6aecad95be03fbd";

#[test]
fn signature_alteree_refusee() {
    let cle = cle_fixe();
    let mut octets = sceller(&essai(), &cle).unwrap();
    let n = octets.len();
    octets[n - 1] ^= 0x01;
    assert_eq!(verifier::<Essai>(&octets, &ancres_pour(&cle), T0).unwrap_err(), ErreurSceau::SignatureInvalide);
}

#[test]
fn charge_alteree_refusee() {
    let cle = cle_fixe();
    let mut octets = sceller(&essai(), &cle).unwrap();
    octets[50] ^= 0x01;
    assert_eq!(verifier::<Essai>(&octets, &ancres_pour(&cle), T0).unwrap_err(), ErreurSceau::SignatureInvalide);
}

#[test]
fn signataire_inconnu_refuse() {
    let octets = sceller(&essai(), &cle_fixe()).unwrap();
    let autre = CleSignature::generer();
    assert_eq!(verifier::<Essai>(&octets, &ancres_pour(&autre), T0).unwrap_err(), ErreurSceau::SignataireInconnu);
    assert_eq!(verifier::<Essai>(&octets, &AncresDeConfiance::new(), T0).unwrap_err(), ErreurSceau::SignataireInconnu);
}

#[test]
fn kid_substitue_refuse() {
    // Sceau signé par A mais qui annonce le kid de B (ancre) : la signature ne vérifie pas.
    let a = cle_fixe();
    let b = CleSignature::generer();
    let mut sceau = Sceau::decoder(&sceller(&essai(), &a).unwrap()).unwrap();
    sceau.kid = b.kid();
    assert_eq!(verifier::<Essai>(&sceau.encoder(), &ancres_pour(&b), T0).unwrap_err(), ErreurSceau::SignatureInvalide);
}

#[test]
fn separation_de_domaine() {
    let cle = cle_fixe();
    let passeport = sceller(&essai(), &cle).unwrap();
    // Un passeport ne peut pas être lu comme visa…
    assert!(matches!(
        verifier::<EssaiVisa>(&passeport, &ancres_pour(&cle), T0).unwrap_err(),
        ErreurSceau::TypeInattendu { .. }
    ));
    // …ni en réécrivant l'octet de type (la signature couvre le type).
    let mut force = passeport.clone();
    force[5] = TypeSceau::Visa as u8;
    assert_eq!(verifier::<EssaiVisa>(&force, &ancres_pour(&cle), T0).unwrap_err(), ErreurSceau::SignatureInvalide);
}

#[test]
fn format_strict() {
    let cle = cle_fixe();
    let ancres = ancres_pour(&cle);
    let octets = sceller(&essai(), &cle).unwrap();

    let mut plus = octets.clone();
    plus.push(0);
    assert_eq!(verifier::<Essai>(&plus, &ancres, T0).unwrap_err(), ErreurSceau::TailleInvalide);
    assert_eq!(verifier::<Essai>(&octets[..octets.len() - 1], &ancres, T0).unwrap_err(), ErreurSceau::TailleInvalide);
    assert_eq!(verifier::<Essai>(&[], &ancres, T0).unwrap_err(), ErreurSceau::TailleInvalide);

    let mut magic = octets.clone();
    magic[0] = b'X';
    assert_eq!(verifier::<Essai>(&magic, &ancres, T0).unwrap_err(), ErreurSceau::MagicInvalide);

    let mut version = octets.clone();
    version[4] = 2;
    assert_eq!(verifier::<Essai>(&version, &ancres, T0).unwrap_err(), ErreurSceau::VersionNonSupportee(2));

    let mut type_inconnu = octets;
    type_inconnu[5] = 0xEE;
    assert_eq!(verifier::<Essai>(&type_inconnu, &ancres, T0).unwrap_err(), ErreurSceau::TypeInconnu(0xEE));
}

fn champ(tag: u8, v: &[u8]) -> Vec<u8> {
    let mut o = vec![tag];
    o.extend_from_slice(&(v.len() as u32).to_be_bytes());
    o.extend_from_slice(v);
    o
}

#[test]
fn encodage_non_canonique_refuse_meme_signe() {
    let cle = cle_fixe();
    let ancres = ancres_pour(&cle);
    let nom = champ(1, b"Miyukini");
    let avant = champ(2, &T0.to_be_bytes());
    let apres = champ(3, &(T0 + 60).to_be_bytes());

    // Désordre des tags.
    let desordre = [avant.clone(), nom.clone(), apres.clone()].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, desordre), &ancres, T0).unwrap_err(),
        ErreurSceau::TagsNonCroissants
    );
    // Doublon.
    let doublon = [nom.clone(), nom.clone(), avant.clone(), apres.clone()].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, doublon), &ancres, T0).unwrap_err(),
        ErreurSceau::TagsNonCroissants
    );
    // Champ inconnu au milieu (tag 4) puis en fin (tag 20).
    let inconnu = [nom.clone(), avant.clone(), apres.clone(), champ(4, b"x")].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, inconnu), &ancres, T0).unwrap_err(),
        ErreurSceau::ChampInconnu(4)
    );
    let inconnu_fin = [nom.clone(), avant.clone(), apres.clone(), champ(20, b"x")].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, inconnu_fin), &ancres, T0).unwrap_err(),
        ErreurSceau::ChampInconnu(20)
    );
    // Champ obligatoire manquant.
    let manquant = [nom.clone(), apres.clone()].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, manquant), &ancres, T0).unwrap_err(),
        ErreurSceau::ChampManquant(2)
    );
    // Entier de mauvaise taille.
    let entier_court = [nom.clone(), champ(2, &[0, 1]), apres.clone()].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, entier_court), &ancres, T0).unwrap_err(),
        ErreurSceau::ValeurInvalide(2)
    );
    // UTF-8 invalide.
    let utf8 = [champ(1, &[0xff, 0xfe]), avant.clone(), apres.clone()].concat();
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, utf8), &ancres, T0).unwrap_err(),
        ErreurSceau::ValeurInvalide(1)
    );
    // Longueur annoncée au-delà des données.
    let mut tronque = nom;
    tronque[4] = 200;
    assert_eq!(
        verifier::<Essai>(&sceller_charge_brute(&cle, TypeSceau::Passeport, tronque), &ancres, T0).unwrap_err(),
        ErreurSceau::ChampTronque
    );
}

#[test]
fn ecrivain_refuse_tags_non_croissants() {
    let mut e = EcrivainTlv::new();
    e.u64(5, 1).unwrap();
    assert_eq!(e.u64(5, 2).unwrap_err(), ErreurSceau::TagsNonCroissants);
    assert_eq!(e.u64(3, 2).unwrap_err(), ErreurSceau::TagsNonCroissants);
}

#[test]
fn champ_optionnel() {
    let cle = cle_fixe();
    let mut t = essai();
    t.note = Some("note".into());
    let v = verifier::<Essai>(&sceller(&t, &cle).unwrap(), &ancres_pour(&cle), T0).unwrap();
    assert_eq!(v.titre().note.as_deref(), Some("note"));
}

#[test]
fn fenetre_de_validite() {
    let cle = cle_fixe();
    let ancres = ancres_pour(&cle);
    let octets = sceller(&essai(), &cle).unwrap();
    let tol = TOLERANCE_HORLOGE_SECS;
    assert!(verifier::<Essai>(&octets, &ancres, T0 - tol).is_ok());
    assert_eq!(verifier::<Essai>(&octets, &ancres, T0 - tol - 1).unwrap_err(), ErreurSceau::PasEncoreValide);
    assert!(verifier::<Essai>(&octets, &ancres, T0 + 3600 + tol).is_ok());
    assert_eq!(verifier::<Essai>(&octets, &ancres, T0 + 3600 + tol + 1).unwrap_err(), ErreurSceau::Expire);
}

#[test]
fn cle_faible_refusee_comme_ancre() {
    // Point identité (ordre 1) : avec lui, la signature R=identité, S=0 vérifierait
    // n'importe quel message sous `verify` non strict.
    let mut identite = [0u8; 32];
    identite[0] = 1;
    let mut ancres = AncresDeConfiance::new();
    assert_eq!(ancres.ajouter(&identite, NiveauConfiance::Reconnu).unwrap_err(), ErreurSceau::CleInvalide);
    assert!(ancres.is_empty());
}

#[test]
fn signature_non_canonique_refusee() {
    // S + L (ordre du groupe) : même point, scalaire non canonique → malléabilité refusée.
    const L: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
    ];
    let cle = cle_fixe();
    let mut sceau = Sceau::decoder(&sceller(&essai(), &cle).unwrap()).unwrap();
    let mut retenue = 0u16;
    for i in 0..32 {
        let somme = u16::from(sceau.signature[32 + i]) + u16::from(L[i]) + retenue;
        sceau.signature[32 + i] = somme as u8;
        retenue = somme >> 8;
    }
    assert_eq!(
        verifier::<Essai>(&sceau.encoder(), &ancres_pour(&cle), T0).unwrap_err(),
        ErreurSceau::SignatureInvalide
    );
}

#[test]
fn debug_masque_le_secret() {
    let cle = cle_fixe();
    let affiche = format!("{cle:?}");
    assert!(affiche.contains("<masqué>"));
    let graine: String = cle.graine().iter().map(|b| format!("{b:02x}")).collect();
    assert!(!affiche.contains(&graine));
}
