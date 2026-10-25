//! Vecteurs de test du format Sceau v1.1 (refus attendus = fail-closed).

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

/// Octets signés recalculés indépendamment de la crate (spécification A-002).
fn octets_signes_spec(alg: u8, type_sceau: TypeSceau, charge: &[u8]) -> Vec<u8> {
    let mut m = b"miyukini/sceau/v1.1\0".to_vec();
    m.push(0x02);
    m.push(alg);
    m.push(type_sceau as u8);
    m.extend_from_slice(&(charge.len() as u32).to_be_bytes());
    m.extend_from_slice(charge);
    m
}

/// Signature Ed25519 brute (hors API) d'un message arbitraire.
fn signer_brut(cle: &CleSignature, message: &[u8]) -> Vec<u8> {
    use ed25519_dalek::Signer;
    ed25519_dalek::SigningKey::from_bytes(&cle.graine()).sign(message).to_bytes().to_vec()
}

/// Re-signe une charge arbitraire (pour tester le décodage canonique au-delà de la signature).
fn sceller_charge_brute(cle: &CleSignature, type_sceau: TypeSceau, charge: Vec<u8>) -> Vec<u8> {
    let octets = signer_brut(cle, &octets_signes_spec(0x01, type_sceau, &charge));
    Sceau {
        algorithme: Algorithme::Ed25519,
        type_sceau,
        charge,
        signatures: vec![SignatureSceau { kid: cle.kid(), algorithme: Algorithme::Ed25519, octets }],
    }
    .encoder_brut()
}

fn hex_sha256(octets: &[u8]) -> String {
    Sha256::digest(octets).iter().map(|b| format!("{b:02x}")).collect()
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
    // Ed25519 est déterministe : ce vecteur fige le format Sceau v1.1.
    // Toute modification involontaire de l'encodage le fera échouer.
    let cle = cle_fixe();
    let octets = sceller(&essai(), &cle).unwrap();
    // magic, version 0x02, alg Ed25519, type passeport.
    assert_eq!(&octets[0..7], b"MSCE\x02\x01\x01");
    let sceau = Sceau::decoder(&octets).unwrap();
    assert_eq!(sceau.signatures.len(), 1);
    // Octets signés conformes à la spécification, signature vérifiable hors crate.
    assert_eq!(sceau.octets_signes(), octets_signes_spec(0x01, TypeSceau::Passeport, &sceau.charge));
    assert_eq!(sceau.signatures[0].octets, signer_brut(&cle, &sceau.octets_signes()));
    // Disposition octet par octet, reconstruite d'après la spécification.
    let mut attendu = b"MSCE\x02\x01\x01".to_vec();
    attendu.extend_from_slice(&(sceau.charge.len() as u32).to_be_bytes());
    attendu.extend_from_slice(&sceau.charge);
    attendu.push(1);
    attendu.extend_from_slice(&cle.kid());
    attendu.extend_from_slice(&[0x01, 0x00, 0x40]);
    attendu.extend_from_slice(&signer_brut(&cle, &sceau.octets_signes()));
    assert_eq!(octets, attendu);
    assert_eq!(hex_sha256(&octets), VECTEUR_REFERENCE, "format Sceau v1.1 modifié");
}

/// SHA-256 du sceau de référence (graine [7; 32], titre `essai()`), figé le 2026-10-24 (v1.1).
const VECTEUR_REFERENCE: &str = "83e9bac5dfb2a81c2de85a5dadd2d699b00cf9d9af1b38ca80b9a2bd64119a65";

#[test]
fn vecteur_de_reference_2_sur_2_stable() {
    // Deux signataires fixes (graines [7; 32] et [8; 32]) : fige le bloc multi-signature.
    let a = cle_fixe();
    let b = CleSignature::depuis_graine(&[8u8; 32]);
    let octets = sceller_multi(&essai(), &[&a, &b]).unwrap();
    // L'ordre des clés passées n'influe pas : bloc canonique trié par kid.
    assert_eq!(sceller_multi(&essai(), &[&b, &a]).unwrap(), octets);
    // Cosigner un sceau simple donne exactement les mêmes octets.
    assert_eq!(cosigner::<Essai>(&sceller(&essai(), &a).unwrap(), &b).unwrap(), octets);
    assert_eq!(cosigner::<Essai>(&sceller(&essai(), &b).unwrap(), &a).unwrap(), octets);
    let sceau = Sceau::decoder(&octets).unwrap();
    assert_eq!(sceau.signatures.len(), 2);
    assert!(sceau.signatures[0].kid < sceau.signatures[1].kid);
    assert_eq!(hex_sha256(&octets), VECTEUR_REFERENCE_2_SUR_2, "bloc multi-signature modifié");
}

/// SHA-256 du sceau 2 sur 2 de référence (graines [7; 32] et [8; 32], titre `essai()`), figé le 2026-10-24.
const VECTEUR_REFERENCE_2_SUR_2: &str = "ce68683e354af8e53416b5cb33347f01b2fc635cbd240ae1676f33820374d761";

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
    octets[20] ^= 0x01;
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
    sceau.signatures[0].kid = b.kid();
    assert_eq!(verifier::<Essai>(&sceau.encoder().unwrap(), &ancres_pour(&b), T0).unwrap_err(), ErreurSceau::SignatureInvalide);
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
    force[6] = TypeSceau::Visa as u8;
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

    // Sceau v1.0 (0x01) abandonné : aucune voie héritée.
    for v in [0x00, 0x01, 0x03, 0xFF] {
        let mut version = octets.clone();
        version[4] = v;
        assert_eq!(verifier::<Essai>(&version, &ancres, T0).unwrap_err(), ErreurSceau::VersionNonSupportee(v));
    }

    let mut type_inconnu = octets;
    type_inconnu[6] = 0xEE;
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
        let somme = u16::from(sceau.signatures[0].octets[32 + i]) + u16::from(L[i]) + retenue;
        sceau.signatures[0].octets[32 + i] = somme as u8;
        retenue = somme >> 8;
    }
    assert_eq!(
        verifier::<Essai>(&sceau.encoder().unwrap(), &ancres_pour(&cle), T0).unwrap_err(),
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

// ---------------------------------------------------------------------------
// Sceau v1.1 : algorithme et multi-signature (amendement A-002)
// ---------------------------------------------------------------------------

fn cle_b() -> CleSignature {
    CleSignature::depuis_graine(&[8u8; 32])
}

fn ancres_de(cles: &[(&CleSignature, NiveauConfiance)]) -> AncresDeConfiance {
    let mut a = AncresDeConfiance::new();
    for (c, n) in cles {
        a.ajouter(&c.cle_publique(), *n).unwrap();
    }
    a
}

#[test]
fn seuil_atteint_ou_non() {
    let (a, b) = (cle_fixe(), cle_b());
    let deux = sceller_multi(&essai(), &[&a, &b]).unwrap();
    let un = sceller(&essai(), &a).unwrap();
    let ancres = ancres_de(&[(&a, NiveauConfiance::Souverain), (&b, NiveauConfiance::Souverain)]);

    let v = verifier_seuil::<Essai>(&deux, &ancres, 2, T0).unwrap();
    let mut attendus = [a.kid(), b.kid()];
    attendus.sort_unstable();
    assert_eq!(v.signataires(), &attendus);
    assert_eq!(v.signataire(), &attendus[0]);
    assert!(verifier_seuil::<Essai>(&deux, &ancres, 1, T0).is_ok());
    assert!(verifier::<Essai>(&deux, &ancres, T0).is_ok());

    // Une seule signature : 2 sur 2 non atteint.
    assert_eq!(
        verifier_seuil::<Essai>(&un, &ancres, 2, T0).unwrap_err(),
        ErreurSceau::SeuilNonAtteint { requis: 2, obtenu: 1 }
    );
    // Deux signatures mais une seule ancre épinglée.
    let une_ancre = ancres_de(&[(&a, NiveauConfiance::Souverain)]);
    assert_eq!(
        verifier_seuil::<Essai>(&deux, &une_ancre, 2, T0).unwrap_err(),
        ErreurSceau::SeuilNonAtteint { requis: 2, obtenu: 1 }
    );
    // Seuil 3 avec deux signataires.
    assert_eq!(
        verifier_seuil::<Essai>(&deux, &ancres, 3, T0).unwrap_err(),
        ErreurSceau::SeuilNonAtteint { requis: 3, obtenu: 2 }
    );
    // Aucun signataire épinglé.
    assert_eq!(
        verifier_seuil::<Essai>(&deux, &AncresDeConfiance::new(), 2, T0).unwrap_err(),
        ErreurSceau::SignataireInconnu
    );
    // Seuil hors bornes.
    for seuil in [0, MAX_SIGNATURES + 1] {
        assert_eq!(verifier_seuil::<Essai>(&deux, &ancres, seuil, T0).unwrap_err(), ErreurSceau::SeuilInvalide);
    }
}

#[test]
fn meme_signataire_compte_une_fois() {
    let a = cle_fixe();
    // À la construction : doublon refusé.
    assert_eq!(sceller_multi(&essai(), &[&a, &a]).unwrap_err(), ErreurSceau::SignataireDuplique);
    let un = sceller(&essai(), &a).unwrap();
    assert_eq!(cosigner::<Essai>(&un, &a).unwrap_err(), ErreurSceau::SignataireDuplique);

    // Forgé à la main : deux entrées du même kid (signatures valides) → refusé au décodage.
    let mut sceau = Sceau::decoder(&un).unwrap();
    sceau.signatures.push(sceau.signatures[0].clone());
    let double = sceau.encoder_brut();
    assert_eq!(Sceau::decoder(&double).unwrap_err(), ErreurSceau::SignataireDuplique);
    assert_eq!(
        verifier_seuil::<Essai>(&double, &ancres_pour(&a), 2, T0).unwrap_err(),
        ErreurSceau::SignataireDuplique
    );
}

#[test]
fn signatures_non_triees_refusees() {
    let deux = sceller_multi(&essai(), &[&cle_fixe(), &cle_b()]).unwrap();
    let mut sceau = Sceau::decoder(&deux).unwrap();
    sceau.signatures.reverse();
    assert_eq!(Sceau::decoder(&sceau.encoder_brut()).unwrap_err(), ErreurSceau::SignaturesNonTriees);
}

#[test]
fn signature_epinglee_invalide_refuse_tout_le_sceau() {
    // Pas d'entrée ignorée : une signature invalide d'un signataire épinglé
    // invalide le sceau même si le seuil est atteint par ailleurs.
    let (a, b) = (cle_fixe(), cle_b());
    let mut sceau = Sceau::decoder(&sceller_multi(&essai(), &[&a, &b]).unwrap()).unwrap();
    sceau.signatures[1].octets[0] ^= 0x01;
    let ancres = ancres_de(&[(&a, NiveauConfiance::Souverain), (&b, NiveauConfiance::Souverain)]);
    assert_eq!(
        verifier_seuil::<Essai>(&sceau.encoder().unwrap(), &ancres, 1, T0).unwrap_err(),
        ErreurSceau::SignatureInvalide
    );
}

#[test]
fn signataire_non_epingle_ignore() {
    // Un cosignataire inconnu (bien formé) n'est ni compté ni bloquant.
    let (a, inconnu) = (cle_fixe(), cle_b());
    let deux = sceller_multi(&essai(), &[&a, &inconnu]).unwrap();
    let v = verifier::<Essai>(&deux, &ancres_pour(&a), T0).unwrap();
    assert_eq!(v.signataires(), &[a.kid()]);
}

#[test]
fn niveau_le_plus_faible_des_signataires() {
    let (a, b) = (cle_fixe(), cle_b());
    let deux = sceller_multi(&essai(), &[&a, &b]).unwrap();
    let mixte = ancres_de(&[(&a, NiveauConfiance::Souverain), (&b, NiveauConfiance::Reconnu)]);
    assert_eq!(verifier_seuil::<Essai>(&deux, &mixte, 2, T0).unwrap().niveau(), NiveauConfiance::Reconnu);
    let souverains = ancres_de(&[(&a, NiveauConfiance::Souverain), (&b, NiveauConfiance::Souverain)]);
    assert_eq!(verifier_seuil::<Essai>(&deux, &souverains, 2, T0).unwrap().niveau(), NiveauConfiance::Souverain);
}

/// Construit un sceau d'algorithme `alg` avec une signature de longueur `taille`.
fn sceau_algorithme(alg: Algorithme, taille: usize) -> Vec<u8> {
    let base = Sceau::decoder(&sceller(&essai(), &cle_fixe()).unwrap()).unwrap();
    Sceau {
        algorithme: alg,
        signatures: vec![SignatureSceau { kid: cle_fixe().kid(), algorithme: alg, octets: vec![0xAB; taille] }],
        ..base
    }
    .encoder_brut()
}

#[test]
fn algorithmes_reserves_refuses() {
    let ancres = ancres_pour(&cle_fixe());
    for alg in [Algorithme::MlDsa65, Algorithme::HybrideEd25519MlDsa65] {
        let octets = sceau_algorithme(alg, alg.taille_signature());
        // Bien formé au décodage…
        assert!(Sceau::decoder(&octets).is_ok());
        // …mais refusé à la vérification (fail-closed).
        assert_eq!(
            verifier::<Essai>(&octets, &ancres, T0).unwrap_err(),
            ErreurSceau::AlgorithmeNonPrisEnCharge(alg as u8)
        );
        // On ne cosigne pas non plus un sceau d'algorithme non implémenté.
        assert_eq!(
            cosigner::<Essai>(&octets, &cle_b()).unwrap_err(),
            ErreurSceau::AlgorithmeNonPrisEnCharge(alg as u8)
        );
    }
    assert_eq!(Algorithme::MlDsa65.taille_signature(), 3_309);
    assert_eq!(Algorithme::HybrideEd25519MlDsa65.taille_signature(), 3_373);
}

#[test]
fn algorithme_inconnu_refuse() {
    let octets = sceller(&essai(), &cle_fixe()).unwrap();
    for alg in [0x00, 0x04, 0xFF] {
        let mut entete = octets.clone();
        entete[5] = alg;
        assert_eq!(Sceau::decoder(&entete).unwrap_err(), ErreurSceau::AlgorithmeInconnu(alg));
    }
    // Code d'algorithme inconnu dans l'entrée de signature (après le kid).
    let pos_alg_entree = octets.len() - 64 - 2 - 1;
    let mut entree = octets;
    entree[pos_alg_entree] = 0x04;
    assert_eq!(Sceau::decoder(&entree).unwrap_err(), ErreurSceau::AlgorithmeInconnu(0x04));
}

#[test]
fn signature_reinterpretee_sous_autre_algorithme_refusee() {
    let cle = cle_fixe();
    let ancres = ancres_pour(&cle);
    let octets = sceller(&essai(), &cle).unwrap();
    let pos_alg_entree = octets.len() - 64 - 2 - 1;

    // 1. Entête seule réécrite : incohérence avec l'entrée.
    let mut entete = octets.clone();
    entete[5] = Algorithme::MlDsa65 as u8;
    assert_eq!(Sceau::decoder(&entete).unwrap_err(), ErreurSceau::AlgorithmeIncoherent);

    // 2. Entête et entrée réécrites : longueur de signature non conforme.
    let mut les_deux = entete;
    les_deux[pos_alg_entree] = Algorithme::MlDsa65 as u8;
    assert_eq!(Sceau::decoder(&les_deux).unwrap_err(), ErreurSceau::TailleInvalide);

    // 3. Signature Ed25519 valide complétée à la taille ML-DSA : refusée (non pris en charge).
    let mut sceau = Sceau::decoder(&octets).unwrap();
    sceau.algorithme = Algorithme::MlDsa65;
    sceau.signatures[0].algorithme = Algorithme::MlDsa65;
    sceau.signatures[0].octets.resize(Algorithme::MlDsa65.taille_signature(), 0);
    assert_eq!(
        verifier::<Essai>(&sceau.encoder().unwrap(), &ancres, T0).unwrap_err(),
        ErreurSceau::AlgorithmeNonPrisEnCharge(0x02)
    );

    // 4. Séparation de domaine : une signature Ed25519 produite sous le contexte
    //    d'un autre algorithme (alg = 0x02 dans les octets signés) ne vérifie pas
    //    dans un sceau Ed25519.
    let charge = Sceau::decoder(&octets).unwrap().charge;
    let autre_contexte = signer_brut(&cle, &octets_signes_spec(0x02, TypeSceau::Passeport, &charge));
    let mut sceau = Sceau::decoder(&octets).unwrap();
    sceau.signatures[0].octets = autre_contexte;
    assert_eq!(verifier::<Essai>(&sceau.encoder().unwrap(), &ancres, T0).unwrap_err(), ErreurSceau::SignatureInvalide);

    // 5. Signature d'un Sceau v1.0 (ancienne étiquette) : ne vérifie pas non plus.
    let mut v1 = b"miyukini/sceau/v1\0".to_vec();
    v1.extend_from_slice(&[0x01, TypeSceau::Passeport as u8]);
    v1.extend_from_slice(&cle.kid());
    v1.extend_from_slice(&(charge.len() as u32).to_be_bytes());
    v1.extend_from_slice(&charge);
    let mut sceau = Sceau::decoder(&octets).unwrap();
    sceau.signatures[0].octets = signer_brut(&cle, &v1);
    assert_eq!(verifier::<Essai>(&sceau.encoder().unwrap(), &ancres, T0).unwrap_err(), ErreurSceau::SignatureInvalide);
}

#[test]
fn nombre_de_signatures_borne() {
    let octets = sceller(&essai(), &cle_fixe()).unwrap();
    let pos_nombre = octets.len() - (32 + 1 + 2 + 64) - 1;
    assert_eq!(octets[pos_nombre], 1);

    // 0 signature : refus.
    let zero = octets[..=pos_nombre].iter().copied().enumerate().map(|(i, b)| if i == pos_nombre { 0 } else { b }).collect::<Vec<_>>();
    assert_eq!(Sceau::decoder(&zero).unwrap_err(), ErreurSceau::NombreSignaturesInvalide(0));
    // 9 ou 255 signatures annoncées : refus avant toute lecture.
    for n in [9u8, 255] {
        let mut trop = octets.clone();
        trop[pos_nombre] = n;
        assert_eq!(Sceau::decoder(&trop).unwrap_err(), ErreurSceau::NombreSignaturesInvalide(n));
    }
    // 2 annoncées, 1 présente : troncature.
    let mut manque = octets;
    manque[pos_nombre] = 2;
    assert_eq!(Sceau::decoder(&manque).unwrap_err(), ErreurSceau::TailleInvalide);

    // 8 signataires : accepté ; 9 : refusé à la construction et à la cosignature.
    let cles: Vec<CleSignature> = (0u8..9).map(|i| CleSignature::depuis_graine(&[i + 20; 32])).collect();
    let refs: Vec<&CleSignature> = cles.iter().collect();
    let huit = sceller_multi(&essai(), &refs[..8]).unwrap();
    let mut ancres = AncresDeConfiance::new();
    for c in &cles[..8] {
        ancres.ajouter(&c.cle_publique(), NiveauConfiance::Souverain).unwrap();
    }
    assert_eq!(verifier_seuil::<Essai>(&huit, &ancres, 8, T0).unwrap().signataires().len(), 8);
    assert_eq!(sceller_multi(&essai(), &refs).unwrap_err(), ErreurSceau::NombreSignaturesInvalide(9));
    assert_eq!(cosigner::<Essai>(&huit, &cles[8]).unwrap_err(), ErreurSceau::NombreSignaturesInvalide(9));
    assert_eq!(sceller_multi::<Essai>(&essai(), &[]).unwrap_err(), ErreurSceau::NombreSignaturesInvalide(0));
}

#[test]
fn troncature_et_octets_en_trop_refuses() {
    let octets = sceller_multi(&essai(), &[&cle_fixe(), &cle_b()]).unwrap();
    // Toute troncature est refusée (jamais de panique).
    for n in 0..octets.len() {
        assert!(Sceau::decoder(&octets[..n]).is_err(), "troncature à {n} acceptée");
    }
    let mut plus = octets.clone();
    plus.push(0);
    assert_eq!(Sceau::decoder(&plus).unwrap_err(), ErreurSceau::TailleInvalide);
    // Longueur de signature annoncée différente de celle de l'algorithme.
    let pos_sig_len = octets.len() - 64 - 2;
    let mut longueur = octets;
    longueur[pos_sig_len + 1] = 63;
    assert_eq!(Sceau::decoder(&longueur).unwrap_err(), ErreurSceau::TailleInvalide);
    // Au-delà de la taille maximale.
    assert_eq!(Sceau::decoder(&vec![0u8; TAILLE_MAX_SCEAU + 1]).unwrap_err(), ErreurSceau::TailleInvalide);
}

#[test]
fn cosigner_controle_le_titre() {
    let a = cle_fixe();
    let passeport = sceller(&essai(), &a).unwrap();
    // Mauvais type attendu.
    assert!(matches!(
        cosigner::<EssaiVisa>(&passeport, &cle_b()).unwrap_err(),
        ErreurSceau::TypeInattendu { .. }
    ));
    // Charge non canonique : on ne cosigne pas ce qu'on ne sait pas lire.
    let nom = champ(1, b"x");
    let brut = sceller_charge_brute(&a, TypeSceau::Passeport, [nom.clone(), nom].concat());
    assert_eq!(cosigner::<Essai>(&brut, &cle_b()).unwrap_err(), ErreurSceau::TagsNonCroissants);
    // Cosignature valide : vérifiable à 2 sur 2.
    let deux = cosigner::<Essai>(&passeport, &cle_b()).unwrap();
    let ancres = ancres_de(&[(&a, NiveauConfiance::Souverain), (&cle_b(), NiveauConfiance::Souverain)]);
    assert!(verifier_seuil::<Essai>(&deux, &ancres, 2, T0).is_ok());
}

#[test]
fn identite_du_titre_independante_des_signatures() {
    let a = cle_fixe();
    let un = Sceau::decoder(&sceller(&essai(), &a).unwrap()).unwrap();
    let deux = Sceau::decoder(&sceller_multi(&essai(), &[&a, &cle_b()]).unwrap()).unwrap();
    assert_eq!(un.octets_signes(), deux.octets_signes());
}

#[test]
fn taille_maximale_conforme_a_la_specification() {
    // Lot 2 §4.1 : 11 + 65 536 + 1 + 8 × (32 + 1 + 2 + 3 373).
    assert_eq!(TAILLE_MAX_SCEAU, 92_812);
}
