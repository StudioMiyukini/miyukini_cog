//! Délégation de clé d'usage (certificat `0x06`, nature `0x45`, A-022 §2) :
//! acceptation, refus, encodage canonique.

use miyusceau::{
    cog_id_depuis_cle, decoder_titre, sceller, verifier, verifier_delegation, AncresDeConfiance,
    CleSignature, DelegationCle, EcrivainTlv, ErreurSceau, LecteurTlv, NiveauConfiance, Sceau,
    Titre, TypeSceau, DUREE_MAX_DELEGATION_SECS, NATURE_DELEGATION,
};

const T0: u64 = 1_800_000_000;
const JOUR: u64 = 86_400;

fn identite() -> CleSignature {
    CleSignature::depuis_graine(&[0x11; 32])
}

fn usage() -> CleSignature {
    CleSignature::depuis_graine(&[0x22; 32])
}

fn delegation(indice: u32, generation: u32) -> DelegationCle {
    let id = identite();
    DelegationCle::nouvelle(
        &cog_id_depuis_cle(&id.cle_publique()),
        indice,
        generation,
        usage().cle_publique(),
        T0,
        T0 + 90 * JOUR,
    )
    .unwrap()
}

/// Titre brut pour forger des charges (champs arbitraires, décodage permissif).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Brut(Vec<(u8, Vec<u8>)>);

impl Titre for Brut {
    const TYPE: TypeSceau = TypeSceau::Certificat;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        for (t, v) in &self.0 {
            e.octets(*t, v)?;
        }
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let mut v = Vec::new();
        for t in 0..=u8::MAX {
            if let Some(o) = l.optionnel(t)? {
                v.push((t, o.to_vec()));
            }
        }
        Ok(Self(v))
    }
}

fn champs_valides() -> Vec<(u8, Vec<u8>)> {
    let id = identite();
    vec![
        (1, vec![NATURE_DELEGATION]),
        (2, cog_id_depuis_cle(&id.cle_publique()).into_bytes()),
        (3, 5u64.to_be_bytes().to_vec()),
        (4, 0u64.to_be_bytes().to_vec()),
        (5, usage().cle_publique().to_vec()),
        (6, T0.to_be_bytes().to_vec()),
        (7, (T0 + 90 * JOUR).to_be_bytes().to_vec()),
    ]
}

fn forger(champs: Vec<(u8, Vec<u8>)>) -> Vec<u8> {
    sceller(&Brut(champs), &identite()).unwrap()
}

#[test]
fn delegation_valide_acceptee() {
    let d = delegation(5, 3);
    let octets = d.sceller(&identite()).unwrap();
    let v = verifier_delegation(
        &octets,
        &identite().cle_publique(),
        NiveauConfiance::Souverain,
        T0 + JOUR,
    )
    .unwrap();
    assert_eq!(v.titre(), &d);
    assert_eq!(v.titre().indice(), 5);
    assert_eq!(v.titre().generation(), 3);
    assert_eq!(v.titre().kid(), usage().kid());
    assert_eq!(v.niveau(), NiveauConfiance::Souverain);
    // Le format brut correspond aux champs forgés à la main (disposition des tags).
    let mut d0 = champs_valides();
    d0[3] = (4, 3u64.to_be_bytes().to_vec());
    assert_eq!(
        Sceau::decoder(&octets).unwrap().charge,
        Sceau::decoder(&forger(d0)).unwrap().charge
    );
}

#[test]
fn signee_par_une_autre_identite_refusee() {
    let autre = CleSignature::depuis_graine(&[0x33; 32]);
    // Signée par une autre clé, mais annonçant le cog_id du COG épinglé.
    let octets = forger(champs_valides());
    let pirate = Sceau::decoder(&octets).unwrap();
    let mut forge = pirate.clone();
    forge.signatures.clear();
    let par_autre = sceller(&Brut(champs_valides()), &autre).unwrap();
    forge.signatures = Sceau::decoder(&par_autre).unwrap().signatures;
    let e = verifier_delegation(
        &forge.encoder().unwrap(),
        &identite().cle_publique(),
        NiveauConfiance::Souverain,
        T0,
    )
    .unwrap_err();
    assert_eq!(e, ErreurSceau::SignataireInconnu);
    // L'autre identité ne peut pas non plus déléguer pour le cog_id du COG.
    assert_eq!(
        delegation(5, 0).sceller(&autre).unwrap_err(),
        ErreurSceau::ValeurInvalide(2)
    );
    // Signée par l'autre identité pour son propre cog_id : refusée contre l'identité épinglée.
    let d = DelegationCle::nouvelle(
        &cog_id_depuis_cle(&autre.cle_publique()),
        5,
        0,
        usage().cle_publique(),
        T0,
        T0 + JOUR,
    )
    .unwrap();
    let octets = d.sceller(&autre).unwrap();
    assert!(verifier_delegation(
        &octets,
        &identite().cle_publique(),
        NiveauConfiance::Souverain,
        T0
    )
    .is_err());
}

#[test]
fn cog_id_different_de_la_cle_signataire_refuse() {
    let mut c = champs_valides();
    c[1] = (2, cog_id_depuis_cle(&[9u8; 32]).into_bytes());
    let e = verifier_delegation(
        &forger(c),
        &identite().cle_publique(),
        NiveauConfiance::Souverain,
        T0,
    )
    .unwrap_err();
    assert_eq!(e, ErreurSceau::ValeurInvalide(2));
}

#[test]
fn hors_fenetre_refusee() {
    let octets = delegation(1, 0).sceller(&identite()).unwrap();
    let cle = identite().cle_publique();
    assert_eq!(
        verifier_delegation(&octets, &cle, NiveauConfiance::Souverain, T0 - 3_600).unwrap_err(),
        ErreurSceau::PasEncoreValide
    );
    assert_eq!(
        verifier_delegation(&octets, &cle, NiveauConfiance::Souverain, T0 + 91 * JOUR).unwrap_err(),
        ErreurSceau::Expire
    );
    let d = delegation(1, 0);
    assert!(
        d.couvre(T0)
            && d.couvre(T0 + 90 * JOUR)
            && !d.couvre(T0 + 91 * JOUR)
            && !d.couvre(T0 - JOUR)
    );
}

#[test]
fn plus_de_90_jours_refusee_partout() {
    let cog = cog_id_depuis_cle(&identite().cle_publique());
    let cle = usage().cle_publique();
    assert_eq!(
        DelegationCle::nouvelle(&cog, 5, 0, cle, T0, T0 + DUREE_MAX_DELEGATION_SECS + 1)
            .unwrap_err(),
        ErreurSceau::ValeurInvalide(7)
    );
    assert!(DelegationCle::nouvelle(&cog, 5, 0, cle, T0, T0).is_err());
    assert!(DelegationCle::nouvelle(&cog, 5, 0, cle, T0, T0 - 1).is_err());
    // Forgée puis signée : le décodeur la refuse aussi.
    let mut c = champs_valides();
    c[6] = (7, (T0 + 90 * JOUR + 1).to_be_bytes().to_vec());
    let e = verifier_delegation(
        &forger(c),
        &identite().cle_publique(),
        NiveauConfiance::Souverain,
        T0,
    )
    .unwrap_err();
    assert_eq!(e, ErreurSceau::ValeurInvalide(7));
}

#[test]
fn indice_et_generation_hors_plage_refuses() {
    for (tag, valeur) in [
        (3u8, 0u64),
        (3, 1 << 31),
        (3, u64::MAX),
        (4, 1 << 31),
        (4, u64::MAX),
    ] {
        let mut c = champs_valides();
        c[usize::from(tag) - 1] = (tag, valeur.to_be_bytes().to_vec());
        assert_eq!(
            decoder_titre::<DelegationCle>(&Sceau::decoder(&forger(c)).unwrap().charge)
                .unwrap_err(),
            ErreurSceau::ValeurInvalide(tag),
            "tag {tag} = {valeur}"
        );
    }
}

#[test]
fn champ_inconnu_manquant_ou_mal_forme_refuse() {
    let charge = |c| Sceau::decoder(&forger(c)).unwrap().charge;
    // Champ supplémentaire (tag 8) : inconnu.
    let mut c = champs_valides();
    c.push((8, vec![0]));
    assert_eq!(
        decoder_titre::<DelegationCle>(&charge(c)).unwrap_err(),
        ErreurSceau::ChampInconnu(8)
    );
    // Champ intercalé (tag 0) : inconnu.
    let mut c = champs_valides();
    c.insert(0, (0, vec![]));
    assert_eq!(
        decoder_titre::<DelegationCle>(&charge(c)).unwrap_err(),
        ErreurSceau::ChampInconnu(0)
    );
    // Champ manquant.
    let mut c = champs_valides();
    c.remove(4);
    assert_eq!(
        decoder_titre::<DelegationCle>(&charge(c)).unwrap_err(),
        ErreurSceau::ChampManquant(5)
    );
    // Autre nature (passation 0x01, arrêté 0x41).
    for nature in [0x01u8, 0x41, 0x46] {
        let mut c = champs_valides();
        c[0] = (1, vec![nature]);
        assert_eq!(
            decoder_titre::<DelegationCle>(&charge(c)).unwrap_err(),
            ErreurSceau::ValeurInvalide(1)
        );
    }
    // Nature sur deux octets, entier sur 4 octets, clé faible, cog_id en majuscules.
    let mut c = champs_valides();
    c[0] = (1, vec![NATURE_DELEGATION, 0]);
    assert!(decoder_titre::<DelegationCle>(&charge(c)).is_err());
    let mut c = champs_valides();
    c[2] = (3, 5u32.to_be_bytes().to_vec());
    assert!(decoder_titre::<DelegationCle>(&charge(c)).is_err());
    let mut c = champs_valides();
    c[4] = (5, [0u8; 32].to_vec());
    assert_eq!(
        decoder_titre::<DelegationCle>(&charge(c)).unwrap_err(),
        ErreurSceau::ValeurInvalide(5)
    );
    let mut c = champs_valides();
    c[1] = (
        2,
        cog_id_depuis_cle(&identite().cle_publique())
            .to_uppercase()
            .into_bytes(),
    );
    assert!(decoder_titre::<DelegationCle>(&charge(c)).is_err());
}

#[test]
fn identite_ne_se_delegue_pas_elle_meme() {
    let id = identite();
    let d = DelegationCle::nouvelle(
        &cog_id_depuis_cle(&id.cle_publique()),
        1,
        0,
        id.cle_publique(),
        T0,
        T0 + JOUR,
    )
    .unwrap();
    assert_eq!(d.sceller(&id).unwrap_err(), ErreurSceau::ValeurInvalide(5));
    // Forgée quand même : refusée à la vérification.
    let mut c = champs_valides();
    c[4] = (5, id.cle_publique().to_vec());
    let e = verifier_delegation(
        &forger(c),
        &id.cle_publique(),
        NiveauConfiance::Souverain,
        T0,
    )
    .unwrap_err();
    assert_eq!(e, ErreurSceau::ValeurInvalide(5));
}

#[test]
fn aller_retour_exact() {
    for (indice, generation) in [(1, 0), (5, 0), (6, 7), ((1 << 31) - 1, (1 << 31) - 1)] {
        let d = delegation(indice, generation);
        let octets = d.sceller(&identite()).unwrap();
        let charge = Sceau::decoder(&octets).unwrap().charge;
        let relu = decoder_titre::<DelegationCle>(&charge).unwrap();
        assert_eq!(relu, d);
        let mut e = EcrivainTlv::new();
        relu.encoder(&mut e).unwrap();
        assert_eq!(e.finir(), charge);
    }
}

#[test]
fn verification_generique_applique_la_fenetre() {
    let octets = delegation(5, 0).sceller(&identite()).unwrap();
    let mut ancres = AncresDeConfiance::new();
    ancres
        .ajouter(&identite().cle_publique(), NiveauConfiance::Souverain)
        .unwrap();
    assert!(verifier::<DelegationCle>(&octets, &ancres, T0).is_ok());
    assert_eq!(
        verifier::<DelegationCle>(&octets, &ancres, T0 + 100 * JOUR).unwrap_err(),
        ErreurSceau::Expire
    );
}
