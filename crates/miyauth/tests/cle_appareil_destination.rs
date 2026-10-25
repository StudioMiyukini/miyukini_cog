//! Tests — une clé d'appareil par COG de destination (non-corrélation complète,
//! point ouvert d'A-003 ; EX-14, EX-36).
//!
//! @id     miyauth_cle_appareil_destination_tests
//! @role   test
//! @layer  toolkit
//! @do     tester_cle_appareil_par_destination

use ed25519_dalek::{Signer, SigningKey};
use miyauth::{
    cle_appareil_pour, emettre_passeport_lie, emettre_visa, lier_cle_appareil, message_liaison, prouver_possession,
    verifier_liaison, verifier_passeport, verifier_visa, AnnuaireEmetteurs, CibleRevocation, CogId, DemandeVisa,
    GraineAppareil, ListeRevocation, PolitiqueIdentite, Portees, RefusIdentite, RevocationsFederees, SecretTitulaire,
};
use miyusceau::{CleSignature, NiveauConfiance};

const T0: u64 = 1_800_000_000;
const AUDIENCE: &str = "jaykoa";

fn signer_pop(cle: &CleSignature, defi: &[u8; 32], audience: &str, liaison: &[u8; 32]) -> [u8; 64] {
    SigningKey::from_bytes(&cle.graine()).sign(&miyauth::message_possession(defi, audience, liaison)).to_bytes()
}

fn cog(c: char) -> CogId {
    CogId::new(&c.to_string().repeat(26)).unwrap()
}

/// COG d'origine `h`, deux destinations `a` et `b` ; un appareil (clé principale + graine).
struct Monde {
    h: CogId,
    cle_h: CleSignature,
    destinations: Vec<(CogId, CleSignature, AnnuaireEmetteurs)>,
    principale: CleSignature,
    graine: GraineAppareil,
    secret: SecretTitulaire,
    revocations: RevocationsFederees,
}

impl Monde {
    fn new() -> Self {
        let h = cog('h');
        let cle_h = CleSignature::generer();
        let mut revocations = RevocationsFederees::new();
        revocations.inserer(ListeRevocation::new(h.clone(), T0, 900));
        let destinations = ['a', 'b']
            .into_iter()
            .map(|c| {
                let d = cog(c);
                let cle = CleSignature::generer();
                let mut annuaire = AnnuaireEmetteurs::new();
                annuaire.ajouter(h.clone(), &cle_h.cle_publique(), NiveauConfiance::Reconnu).unwrap();
                annuaire.ajouter(d.clone(), &cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
                revocations.inserer(ListeRevocation::new(d.clone(), T0, 900));
                (d, cle, annuaire)
            })
            .collect();
        Self {
            h,
            cle_h,
            destinations,
            principale: CleSignature::generer(),
            graine: GraineAppareil::generer(),
            secret: SecretTitulaire::generer(),
            revocations,
        }
    }

    fn politique(&self, i: usize) -> PolitiqueIdentite<'_> {
        let (d, _, annuaire) = &self.destinations[i];
        PolitiqueIdentite { cog_local: d.clone(), annuaire, revocations: &self.revocations }
    }

    /// Passeport émis par `h` pour la destination `i`, avec la clé dérivée pour elle.
    fn passeport(&self, i: usize) -> Vec<u8> {
        let d = &self.destinations[i].0;
        let demande = lier_cle_appareil(&self.principale, &self.graine, &self.h, d).unwrap();
        let liaison = verifier_liaison(&self.h, &self.principale.cle_publique(), &demande).unwrap();
        emettre_passeport_lie(
            &self.h,
            &self.cle_h,
            &self.secret,
            &liaison,
            Portees::new(["jaykoa:read"]).unwrap(),
            T0 - 60,
            T0 + 3600,
        )
        .unwrap()
    }
}

/// @id MAUTH_CAD_001
/// @role test
/// @layer toolkit
/// @do mauth_cad_001_cle_distincte_par_destination
//@human / @human Deux COG visités reçoivent deux clés d'appareil et deux sujets sans lien ; aucune n'est la clé principale.
#[test]
fn mauth_cad_001_cle_distincte_par_destination() {
    let m = Monde::new();
    let pa = verifier_passeport(&m.passeport(0), &m.politique(0), T0).unwrap();
    let pb = verifier_passeport(&m.passeport(1), &m.politique(1), T0).unwrap();
    let (a, b) = (pa.passeport(), pb.passeport());

    // Corrélation impossible : tout ce qui dépend de l'appareil ou du titulaire diffère.
    assert_ne!(a.cle_appareil, b.cle_appareil);
    assert_ne!(a.sujet, b.sujet);
    assert_ne!(a.serie, b.serie);
    for p in [a, b] {
        assert_ne!(p.cle_appareil, m.principale.cle_publique());
    }
    // Ce qui reste commun est l'émetteur (COG d'origine), déjà exposé par `pw:<cog d'origine>:…`.
    assert_eq!(a.emetteur_cog, b.emetteur_cog);

    // Déterministe : l'appareil retrouve sa clé sans état.
    assert_eq!(cle_appareil_pour(&m.graine, &m.destinations[0].0).unwrap().cle_publique(), a.cle_appareil);
    // Une autre graine (autre appareil) donne une autre clé pour la même destination.
    let autre = GraineAppareil::generer();
    assert_ne!(cle_appareil_pour(&autre, &m.destinations[0].0).unwrap().cle_publique(), a.cle_appareil);
}

/// @id MAUTH_CAD_002
/// @role test
/// @layer toolkit
/// @do mauth_cad_002_possession_par_cle_de_destination
//@human / @human La possession se prouve avec la clé de la destination ; la clé principale ou celle d'un autre COG est refusée.
#[test]
fn mauth_cad_002_possession_par_cle_de_destination() {
    let m = Monde::new();
    let (d, cle_d, _) = &m.destinations[0];
    let p = m.passeport(0);
    let (defi, liaison) = ([1u8; 32], [2u8; 32]);
    let cle_a = cle_appareil_pour(&m.graine, d).unwrap();
    let cle_b = cle_appareil_pour(&m.graine, &m.destinations[1].0).unwrap();

    for mauvaise in [&m.principale, &cle_b] {
        let pv = verifier_passeport(&p, &m.politique(0), T0).unwrap();
        let sig = signer_pop(mauvaise, &defi, "visa", &liaison);
        assert_eq!(prouver_possession(pv, &defi, "visa", &liaison, &sig).err(), Some(RefusIdentite::PossessionInvalide));
    }
    let pv = verifier_passeport(&p, &m.politique(0), T0).unwrap();
    let possession = prouver_possession(pv, &defi, "visa", &liaison, &signer_pop(&cle_a, &defi, "visa", &liaison)).unwrap();
    let demande = DemandeVisa {
        audience: AUDIENCE.into(),
        portees: Portees::new(["jaykoa:read"]).unwrap(),
        pas_avant: T0,
        pas_apres: T0 + 600,
    };
    let visa = emettre_visa(d, cle_d, &possession, &demande).unwrap();
    let acces = verifier_visa(&visa, &p, AUDIENCE, &m.politique(0), T0 + 1).unwrap();
    assert_eq!(acces.visa().cle_appareil, cle_a.cle_publique());
    let sig = signer_pop(&cle_a, &[3; 32], AUDIENCE, &[4; 32]);
    assert!(acces.verifier_possession(&[3; 32], &[4; 32], &sig).is_ok());
    let sig = signer_pop(&m.principale, &[3; 32], AUDIENCE, &[4; 32]);
    assert_eq!(acces.verifier_possession(&[3; 32], &[4; 32], &sig).err(), Some(RefusIdentite::PossessionInvalide));

    // Passeport de `a` présenté à `b` : refusé (destination), la clé n'y sert à rien.
    assert_eq!(verifier_passeport(&p, &m.politique(1), T0).err(), Some(RefusIdentite::DestinationIncorrecte));
}

/// @id MAUTH_CAD_003
/// @role test
/// @layer toolkit
/// @do mauth_cad_003_liaison_refusee
//@human / @human Liaison altérée, non autorisée, sans possession, corrélante ou d'un autre COG d'origine : refus.
#[test]
#[allow(clippy::many_single_char_names)]
fn mauth_cad_003_liaison_refusee() {
    let m = Monde::new();
    let (a, b) = (&m.destinations[0].0, &m.destinations[1].0);
    let cle_principale = m.principale.cle_publique();
    let bonne = lier_cle_appareil(&m.principale, &m.graine, &m.h, a).unwrap();
    assert!(verifier_liaison(&m.h, &cle_principale, &bonne).is_ok());

    // Destination modifiée après signature.
    let mut d = bonne.clone();
    d.destination = b.clone();
    assert_eq!(verifier_liaison(&m.h, &cle_principale, &d).err(), Some(RefusIdentite::PossessionInvalide));
    // Clé de destination substituée (celle d'autrui) : signature de possession invalide.
    let mut d = bonne.clone();
    d.cle_destination = CleSignature::generer().cle_publique();
    assert_eq!(verifier_liaison(&m.h, &cle_principale, &d).err(), Some(RefusIdentite::PossessionInvalide));
    // Non autorisée par la clé principale appairée (autre appareil).
    let intrus = CleSignature::generer();
    let d = lier_cle_appareil(&intrus, &m.graine, &m.h, a).unwrap();
    assert_eq!(verifier_liaison(&m.h, &cle_principale, &d).err(), Some(RefusIdentite::PossessionInvalide));
    // Liaison faite pour un autre COG d'origine.
    let d = lier_cle_appareil(&m.principale, &m.graine, &cog('x'), a).unwrap();
    assert_eq!(verifier_liaison(&m.h, &cle_principale, &d).err(), Some(RefusIdentite::PossessionInvalide));

    // Clé de destination = clé principale (corrélante), même correctement signée.
    let msg = message_liaison(&m.h, a, &cle_principale);
    let sig = SigningKey::from_bytes(&m.principale.graine()).sign(&msg).to_bytes();
    let correlante = miyauth::DemandeLiaison {
        destination: a.clone(),
        cle_destination: cle_principale,
        signature_principale: sig,
        signature_destination: sig,
    };
    assert_eq!(
        verifier_liaison(&m.h, &cle_principale, &correlante).err(),
        Some(RefusIdentite::ParametreInvalide("cle_destination"))
    );

    // Liaison vérifiée par `h`, émission par un autre COG : refus.
    let liaison = verifier_liaison(&m.h, &cle_principale, &bonne).unwrap();
    let autre = CleSignature::generer();
    let r = emettre_passeport_lie(
        &cog('x'),
        &autre,
        &m.secret,
        &liaison,
        Portees::new(["jaykoa:read"]).unwrap(),
        T0,
        T0 + 60,
    );
    assert_eq!(r.err(), Some(RefusIdentite::EmetteurIncoherent));
}

/// @id MAUTH_CAD_004
/// @role test
/// @layer toolkit
/// @do mauth_cad_004_revocation_par_cle_de_destination
//@human / @human Révoquer l'appareil chez son COG d'origine = révoquer chaque clé de destination liée.
#[test]
fn mauth_cad_004_revocation_par_cle_de_destination() {
    let mut m = Monde::new();
    let (pa, pb) = (m.passeport(0), m.passeport(1));
    let liaisons: Vec<_> = m
        .destinations
        .iter()
        .map(|(d, _, _)| {
            let demande = lier_cle_appareil(&m.principale, &m.graine, &m.h, d).unwrap();
            verifier_liaison(&m.h, &m.principale.cle_publique(), &demande).unwrap()
        })
        .collect();
    let h = m.h.clone();

    // Révoquer la seule clé principale ne touche pas les clés de destination (non reliables) :
    // le COG d'origine doit révoquer chaque liaison qu'il a acceptée.
    m.revocations.liste_mut(&h).unwrap().revoquer(CibleRevocation::appareil(&m.principale.cle_publique()));
    assert!(verifier_passeport(&pa, &m.politique(0), T0).is_ok());

    m.revocations.liste_mut(&h).unwrap().revoquer(liaisons[0].cible_revocation());
    let cible_a = liaisons[0].cible_revocation();
    assert_eq!(verifier_passeport(&pa, &m.politique(0), T0).err(), Some(RefusIdentite::Revoque(cible_a)));
    assert!(verifier_passeport(&pb, &m.politique(1), T0).is_ok());
    m.revocations.liste_mut(&h).unwrap().revoquer(liaisons[1].cible_revocation());
    assert!(matches!(verifier_passeport(&pb, &m.politique(1), T0), Err(RefusIdentite::Revoque(_))));
}
