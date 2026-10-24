//! Tests Passeport / Visa / révocation — lot 2 incrément 4 (EX-14, EX-36).
//!
//! @id     miyauth_passeport_visa_tests
//! @role   test
//! @layer  toolkit
//! @do     tester_passeport_visa_revocation

use ed25519_dalek::{Signer, SigningKey};
use miyauth::{
    emettre_passeport, emettre_visa, identifiant_pairwise, identity_resolve, prouver_possession, verifier_passeport,
    verifier_visa, AnnuaireEmetteurs, CibleRevocation, CogId, DemandePasseport, DemandeVisa, ErreurRevocation,
    GovernedContext, IdentityArtefacts, IdentityRole, ListeRevocation, Portees, PolitiqueIdentite,
    PossessionProuvee, RefusIdentite, RevocationsFederees, SecretTitulaire,
};
use miyusceau::{CleSignature, ErreurSceau, NiveauConfiance};

const T0: u64 = 1_800_000_000;
const JOUR: u64 = 24 * 3600;
const AUDIENCE: &str = "jaykoa";

fn cog(c: char) -> CogId {
    CogId::new(&c.to_string().repeat(26)).unwrap()
}

/// Deux COG : `h` (origine du titulaire) et `v` (visité), vus depuis `v`.
struct Monde {
    h: CogId,
    v: CogId,
    cle_h: CleSignature,
    cle_v: CleSignature,
    appareil: CleSignature,
    secret: SecretTitulaire,
    annuaire_v: AnnuaireEmetteurs,
    annuaire_h: AnnuaireEmetteurs,
    revocations: RevocationsFederees,
}

impl Monde {
    fn new() -> Self {
        let (h, v) = (cog('h'), cog('v'));
        let (cle_h, cle_v) = (CleSignature::generer(), CleSignature::generer());
        let mut annuaire_v = AnnuaireEmetteurs::new();
        annuaire_v.ajouter(h.clone(), &cle_h.cle_publique(), NiveauConfiance::Reconnu).unwrap();
        annuaire_v.ajouter(v.clone(), &cle_v.cle_publique(), NiveauConfiance::Souverain).unwrap();
        let mut annuaire_h = AnnuaireEmetteurs::new();
        annuaire_h.ajouter(h.clone(), &cle_h.cle_publique(), NiveauConfiance::Souverain).unwrap();
        let mut revocations = RevocationsFederees::new();
        revocations.inserer(ListeRevocation::new(h.clone(), T0, 900));
        revocations.inserer(ListeRevocation::new(v.clone(), T0, 900));
        Self {
            h,
            v,
            cle_h,
            cle_v,
            appareil: CleSignature::generer(),
            secret: SecretTitulaire::generer(),
            annuaire_v,
            annuaire_h,
            revocations,
        }
    }

    fn politique_v(&self) -> PolitiqueIdentite<'_> {
        PolitiqueIdentite { cog_local: self.v.clone(), annuaire: &self.annuaire_v, revocations: &self.revocations }
    }

    fn politique_h(&self) -> PolitiqueIdentite<'_> {
        PolitiqueIdentite { cog_local: self.h.clone(), annuaire: &self.annuaire_h, revocations: &self.revocations }
    }

    fn passeport_pour(&self, destination: &CogId, pas_avant: u64, pas_apres: u64) -> Vec<u8> {
        let demande = DemandePasseport {
            secret: &self.secret,
            destination: destination.clone(),
            cle_appareil: self.appareil.cle_publique(),
            portees: Portees::new(["services:*:read"]).unwrap(),
            pas_avant,
            pas_apres,
        };
        emettre_passeport(&self.h, &self.cle_h, &demande).unwrap()
    }

    fn passeport(&self) -> Vec<u8> {
        self.passeport_pour(&self.v, T0 - JOUR, T0 + 29 * JOUR)
    }

    fn signer_pop(&self, defi: &[u8; 32], audience: &str, liaison: &[u8; 32]) -> [u8; 64] {
        let sk = SigningKey::from_bytes(&self.appareil.graine());
        sk.sign(&miyauth::message_possession(defi, audience, liaison)).to_bytes()
    }

    fn possession(&self, politique: &PolitiqueIdentite<'_>, passeport: &[u8]) -> PossessionProuvee {
        let pv = verifier_passeport(passeport, politique, T0).unwrap();
        let (defi, liaison) = ([7u8; 32], [9u8; 32]);
        let sig = self.signer_pop(&defi, "visa", &liaison);
        prouver_possession(pv, &defi, "visa", &liaison, &sig).unwrap()
    }

    fn demande_visa(&self) -> DemandeVisa {
        DemandeVisa {
            audience: AUDIENCE.to_string(),
            portees: Portees::new(["jaykoa:read"]).unwrap(),
            pas_avant: T0,
            pas_apres: T0 + 600,
        }
    }

    /// Passeport (h → v) et visa émis par v.
    fn acces(&self) -> (Vec<u8>, Vec<u8>) {
        let p = self.passeport();
        let pp = self.possession(&self.politique_v(), &p);
        let visa = emettre_visa(&self.v, &self.cle_v, &pp, &self.demande_visa()).unwrap();
        (p, visa)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Aller-retour
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_001
/// @role test
/// @layer toolkit
/// @do mauth_pv_001_aller_retour_visiteur
//@human / @human Passeport émis par h pour v, visa émis par v : accès Visitor, sujet pairwise, PoP de session.
#[test]
fn mauth_pv_001_aller_retour_visiteur() {
    let m = Monde::new();
    let (p, visa) = m.acces();
    let acces = verifier_visa(&visa, &p, AUDIENCE, &m.politique_v(), T0 + 60).unwrap();
    assert_eq!(acces.role(), IdentityRole::Visitor);
    assert_eq!(acces.sujet(), &identifiant_pairwise(&m.secret, &m.v));
    assert_eq!(acces.passeport().niveau(), NiveauConfiance::Reconnu);

    let (defi, liaison) = ([1u8; 32], [2u8; 32]);
    let sig = m.signer_pop(&defi, AUDIENCE, &liaison);
    acces.verifier_possession(&defi, &liaison, &sig).unwrap();
    // Liaison de canal différente (relais) : refus.
    assert_eq!(acces.verifier_possession(&defi, &[3u8; 32], &sig), Err(RefusIdentite::PossessionInvalide));

    let ctx = GovernedContext::new("mandat".into(), 2);
    let c = identity_resolve(&ctx, &IdentityArtefacts::default(), Some(&acces)).unwrap();
    assert_eq!(c.role, IdentityRole::Visitor);
    assert!(c.opaque_id.starts_with("pw:") && c.opaque_id.len() == 3 + 64);
}

/// @id MAUTH_PV_002
/// @role test
/// @layer toolkit
/// @do mauth_pv_002_aller_retour_citoyen
//@human / @human Passeport et visa émis par le COG d'origine : accès Citizen, niveau Souverain.
#[test]
fn mauth_pv_002_aller_retour_citoyen() {
    let m = Monde::new();
    let p = m.passeport_pour(&m.h, T0 - JOUR, T0 + JOUR);
    let pp = m.possession(&m.politique_h(), &p);
    let visa = emettre_visa(&m.h, &m.cle_h, &pp, &m.demande_visa()).unwrap();
    let acces = verifier_visa(&visa, &p, AUDIENCE, &m.politique_h(), T0).unwrap();
    assert_eq!(acces.role(), IdentityRole::Citizen);
    assert_eq!(acces.passeport().niveau(), NiveauConfiance::Souverain);
    // Le même passeport ne vaut rien à v (destination h).
    assert_eq!(
        verifier_passeport(&p, &m.politique_v(), T0).unwrap_err(),
        RefusIdentite::DestinationIncorrecte
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Validité temporelle
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_010
/// @role test
/// @layer toolkit
/// @do mauth_pv_010_expire_et_pas_encore_valide
//@human / @human Passeport et visa expirés ou pas encore valides, au-delà de la tolérance de 120 s.
#[test]
fn mauth_pv_010_expire_et_pas_encore_valide() {
    let m = Monde::new();
    let pol = m.politique_v();
    let p = m.passeport_pour(&m.v, T0 + 1_000, T0 + 2_000);
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::Sceau(ErreurSceau::PasEncoreValide));
    let p = m.passeport_pour(&m.v, T0 - 2_000, T0 - 1_000);
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::Sceau(ErreurSceau::Expire));
    // Dans la tolérance d'horloge (≤ 120 s) : accepté.
    let p = m.passeport_pour(&m.v, T0 + 100, T0 + 2_000);
    assert!(verifier_passeport(&p, &pol, T0).is_ok());

    // Visa expiré (15 min plus tard), passeport encore valide.
    let (p, visa) = m.acces();
    let mut revs = RevocationsFederees::new();
    revs.inserer(ListeRevocation::new(m.h.clone(), T0 + 1_000, 900));
    revs.inserer(ListeRevocation::new(m.v.clone(), T0 + 1_000, 900));
    let pol2 = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &m.annuaire_v, revocations: &revs };
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &pol2, T0 + 1_000).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::Expire)
    );
}

/// @id MAUTH_PV_011
/// @role test
/// @layer toolkit
/// @do mauth_pv_011_durees_maximales
//@human / @human Passeport > 30 j, visa > 15 min, visa hors fenêtre du passeport : émission refusée.
#[test]
fn mauth_pv_011_durees_maximales() {
    let m = Monde::new();
    let demande = DemandePasseport {
        secret: &m.secret,
        destination: m.v.clone(),
        cle_appareil: m.appareil.cle_publique(),
        portees: Portees::default(),
        pas_avant: T0,
        pas_apres: T0 + 31 * JOUR,
    };
    assert!(matches!(emettre_passeport(&m.h, &m.cle_h, &demande), Err(RefusIdentite::Sceau(_))));

    let p = m.passeport_pour(&m.v, T0 - 60, T0 + 300);
    let pp = m.possession(&m.politique_v(), &p);
    let mut d = m.demande_visa();
    d.pas_apres = T0 + 16 * 60;
    d.pas_avant = T0;
    assert_eq!(emettre_visa(&m.v, &m.cle_v, &pp, &d).unwrap_err(), RefusIdentite::FenetreHorsPasseport);
    let p = m.passeport();
    let pp = m.possession(&m.politique_v(), &p);
    assert!(matches!(emettre_visa(&m.v, &m.cle_v, &pp, &d), Err(RefusIdentite::Sceau(_))));
}

// ═══════════════════════════════════════════════════════════════════════════
// Ancres
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_020
/// @role test
/// @layer toolkit
/// @do mauth_pv_020_mauvaise_ancre
//@human / @human Émetteur non épinglé, épinglé sous un autre cog_id, ou au mauvais niveau : refus.
#[test]
fn mauth_pv_020_mauvaise_ancre() {
    let m = Monde::new();
    let p = m.passeport();

    // Annuaire sans la clé de h.
    let mut sans_h = AnnuaireEmetteurs::new();
    sans_h.ajouter(m.v.clone(), &m.cle_v.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let pol = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &sans_h, revocations: &m.revocations };
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::Sceau(ErreurSceau::SignataireInconnu));

    // Clé de h épinglée sous le cog_id d'un autre COG.
    let mut usurpe = AnnuaireEmetteurs::new();
    usurpe.ajouter(cog('x'), &m.cle_h.cle_publique(), NiveauConfiance::Reconnu).unwrap();
    let pol = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &usurpe, revocations: &m.revocations };
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::EmetteurIncoherent);

    // COG étranger épinglé comme souverain : configuration incohérente, refus.
    let mut niveau = AnnuaireEmetteurs::new();
    niveau.ajouter(m.h.clone(), &m.cle_h.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let pol = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &niveau, revocations: &m.revocations };
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::NiveauIncoherent);

    // Passeport scellé par une clé non déclarée dans `emetteur_kid` (clé de v se faisant passer pour h).
    let faux = {
        let demande = DemandePasseport {
            secret: &m.secret,
            destination: m.v.clone(),
            cle_appareil: m.appareil.cle_publique(),
            portees: Portees::default(),
            pas_avant: T0,
            pas_apres: T0 + JOUR,
        };
        emettre_passeport(&m.h, &m.cle_v, &demande).unwrap()
    };
    assert_eq!(verifier_passeport(&faux, &m.politique_v(), T0).unwrap_err(), RefusIdentite::EmetteurIncoherent);
}

/// @id MAUTH_PV_021
/// @role test
/// @layer toolkit
/// @do mauth_pv_021_visa_etranger
//@human / @human Visa scellé par un COG reconnu mais non local : refus.
#[test]
fn mauth_pv_021_visa_etranger() {
    let m = Monde::new();
    let p = m.passeport();
    let pp = m.possession(&m.politique_v(), &p);
    let visa = emettre_visa(&m.v, &m.cle_h, &pp, &m.demande_visa()).unwrap();
    assert_eq!(verifier_visa(&visa, &p, AUDIENCE, &m.politique_v(), T0).unwrap_err(), RefusIdentite::VisaEtranger);
}

// ═══════════════════════════════════════════════════════════════════════════
// Liens passeport ↔ visa ↔ clé d'appareil
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_030
/// @role test
/// @layer toolkit
/// @do mauth_pv_030_visa_pour_un_autre_passeport
//@human / @human Visa émis pour le passeport A présenté avec le passeport B : refus.
#[test]
fn mauth_pv_030_visa_pour_un_autre_passeport() {
    let m = Monde::new();
    let (_, visa_a) = m.acces();
    let p_b = m.passeport();
    assert_eq!(
        verifier_visa(&visa_a, &p_b, AUDIENCE, &m.politique_v(), T0).unwrap_err(),
        RefusIdentite::PasseportDifferent
    );
}

/// @id MAUTH_PV_031
/// @role test
/// @layer toolkit
/// @do mauth_pv_031_audience_et_possession
//@human / @human Audience différente ou preuve de possession d'une autre clé : refus.
#[test]
fn mauth_pv_031_audience_et_possession() {
    let m = Monde::new();
    let (p, visa) = m.acces();
    assert_eq!(
        verifier_visa(&visa, &p, "miyucloud", &m.politique_v(), T0).unwrap_err(),
        RefusIdentite::AudienceIncorrecte
    );
    // Possession : signature d'une autre clé (vol du fichier passeport sans la clé).
    let pv = verifier_passeport(&p, &m.politique_v(), T0).unwrap();
    let voleur = SigningKey::from_bytes(&CleSignature::generer().graine());
    let (defi, liaison) = ([5u8; 32], [6u8; 32]);
    let sig = voleur.sign(&miyauth::message_possession(&defi, "visa", &liaison)).to_bytes();
    assert_eq!(
        prouver_possession(pv, &defi, "visa", &liaison, &sig).unwrap_err(),
        RefusIdentite::PossessionInvalide
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// Identifiant par paire (EX-36)
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_040
/// @role test
/// @layer toolkit
/// @do mauth_pv_040_pairwise
//@human / @human Identifiants différents par COG visité, stables par COG, différents par titulaire.
#[test]
fn mauth_pv_040_pairwise() {
    let s = SecretTitulaire::depuis_octets([42u8; 32]);
    let (a, b) = (cog('a'), cog('b'));
    assert_eq!(identifiant_pairwise(&s, &a), identifiant_pairwise(&s, &a));
    assert_ne!(identifiant_pairwise(&s, &a), identifiant_pairwise(&s, &b));
    let s2 = SecretTitulaire::depuis_octets([43u8; 32]);
    assert_ne!(identifiant_pairwise(&s, &a), identifiant_pairwise(&s2, &a));
    assert!(!format!("{s:?}").contains("42"));

    // Dans les passeports : sujet différent par destination, stable entre émissions.
    let m = Monde::new();
    let p_v1 = verifier_passeport(&m.passeport(), &m.politique_v(), T0).unwrap();
    let p_v2 = verifier_passeport(&m.passeport(), &m.politique_v(), T0).unwrap();
    assert_eq!(p_v1.passeport().sujet, p_v2.passeport().sujet);
    assert_ne!(p_v1.passeport().serie, p_v2.passeport().serie);
    let p_h = verifier_passeport(&m.passeport_pour(&m.h, T0, T0 + JOUR), &m.politique_h(), T0).unwrap();
    assert_ne!(p_v1.passeport().sujet, p_h.passeport().sujet);
}

/// @id MAUTH_PV_041
/// @role test
/// @layer toolkit
/// @do mauth_pv_041_cog_id_forme
//@human / @human cog_id : 26 caractères base32 minuscules uniquement.
#[test]
fn mauth_pv_041_cog_id_forme() {
    assert!(CogId::new("abcdefghijklmnopqrstuvwxyz").is_some());
    assert!(CogId::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").is_none());
    assert!(CogId::new("abcdefghijklmnopqrstuvwxy1").is_none());
    assert!(CogId::new("abc").is_none());
}

// ═══════════════════════════════════════════════════════════════════════════
// Révocation (EX-14)
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_050
/// @role test
/// @layer toolkit
/// @do mauth_pv_050_appareil_revoque
//@human / @human Appareil révoqué par son COG d'origine ou banni localement : passeport et visa refusés.
#[test]
fn mauth_pv_050_appareil_revoque() {
    let mut m = Monde::new();
    let (p, visa) = m.acces();
    let cible = CibleRevocation::appareil(&m.appareil.cle_publique());
    m.revocations.liste_mut(&m.h).unwrap().revoquer(cible);
    assert_eq!(verifier_visa(&visa, &p, AUDIENCE, &m.politique_v(), T0).unwrap_err(), RefusIdentite::Revoque(cible));

    // Bannissement local par le COG hôte.
    let mut m = Monde::new();
    let (p, visa) = m.acces();
    let cible = cible_appareil(&m);
    m.revocations.liste_mut(&m.v).unwrap().revoquer(cible);
    assert!(matches!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique_v(), T0),
        Err(RefusIdentite::Revoque(_))
    ));
}

fn cible_appareil(m: &Monde) -> CibleRevocation {
    CibleRevocation::appareil(&m.appareil.cle_publique())
}

/// @id MAUTH_PV_051
/// @role test
/// @layer toolkit
/// @do mauth_pv_051_serie_et_cle_emission_revoquees
//@human / @human Passeport (série), visa (série) ou clé d'émission révoqués : refus.
#[test]
fn mauth_pv_051_serie_et_cle_emission_revoquees() {
    let mut m = Monde::new();
    let (p, visa) = m.acces();
    let serie_p = verifier_passeport(&p, &m.politique_v(), T0).unwrap().passeport().serie;
    let serie_v = verifier_visa(&visa, &p, AUDIENCE, &m.politique_v(), T0).unwrap().visa().serie;

    m.revocations.liste_mut(&m.v).unwrap().revoquer(CibleRevocation::Serie(serie_v));
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique_v(), T0).unwrap_err(),
        RefusIdentite::Revoque(CibleRevocation::Serie(serie_v))
    );
    m.revocations.liste_mut(&m.h).unwrap().revoquer(CibleRevocation::Serie(serie_p));
    assert_eq!(
        verifier_passeport(&p, &m.politique_v(), T0).unwrap_err(),
        RefusIdentite::Revoque(CibleRevocation::Serie(serie_p))
    );

    let mut m = Monde::new();
    let p = m.passeport();
    let kid_h = m.cle_h.kid();
    m.revocations.liste_mut(&m.h).unwrap().revoquer(CibleRevocation::Cle(kid_h));
    assert_eq!(
        verifier_passeport(&p, &m.politique_v(), T0).unwrap_err(),
        RefusIdentite::Revoque(CibleRevocation::Cle(kid_h))
    );
}

/// @id MAUTH_PV_052
/// @role test
/// @layer toolkit
/// @do mauth_pv_052_liste_perimee_ou_absente
//@human / @human Liste périmée, absente ou datée dans le futur : refus (fail-closed).
#[test]
fn mauth_pv_052_liste_perimee_ou_absente() {
    let m = Monde::new();
    let p = m.passeport();

    // Périmée : à jour au T0, fraîcheur 900 s, vérification à T0 + 901.
    let r = verifier_passeport(&p, &m.politique_v(), T0 + 901).unwrap_err();
    assert_eq!(r, RefusIdentite::Revocation(ErreurRevocation::Perimee { a_jour_au: T0, maintenant: T0 + 901 }));

    // Absente pour le COG émetteur.
    let mut seule_v = RevocationsFederees::new();
    seule_v.inserer(ListeRevocation::new(m.v.clone(), T0, 900));
    let pol = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &m.annuaire_v, revocations: &seule_v };
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::Revocation(ErreurRevocation::ListeAbsente));

    // Une liste unique d'un autre COG ne répond pas pour h.
    let liste_v = ListeRevocation::new(m.v.clone(), T0, 900);
    let pol = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &m.annuaire_v, revocations: &liste_v };
    assert_eq!(verifier_passeport(&p, &pol, T0).unwrap_err(), RefusIdentite::Revocation(ErreurRevocation::ListeAbsente));

    // Datée dans le futur (horloge locale en retard ou liste falsifiée).
    let mut futur = RevocationsFederees::new();
    futur.inserer(ListeRevocation::new(m.h.clone(), T0 + 3_600, 900));
    futur.inserer(ListeRevocation::new(m.v.clone(), T0, 900));
    let pol = PolitiqueIdentite { cog_local: m.v.clone(), annuaire: &m.annuaire_v, revocations: &futur };
    assert_eq!(
        verifier_passeport(&p, &pol, T0).unwrap_err(),
        RefusIdentite::Revocation(ErreurRevocation::HorlogeIncoherente)
    );

    // Rafraîchissement : pas de recul.
    let mut l = ListeRevocation::new(m.h.clone(), T0, 900);
    assert!(!l.rafraichir(T0 - 1));
    assert!(l.rafraichir(T0 + 10));
}

// ═══════════════════════════════════════════════════════════════════════════
// Altération
// ═══════════════════════════════════════════════════════════════════════════

/// @id MAUTH_PV_060
/// @role test
/// @layer toolkit
/// @do mauth_pv_060_octets_alteres
//@human / @human Toute modification d'un octet du passeport ou du visa, troncature ou ajout : refus.
#[test]
fn mauth_pv_060_octets_alteres() {
    let m = Monde::new();
    let pol = m.politique_v();
    let (p, visa) = m.acces();
    for i in 0..p.len() {
        let mut x = p.clone();
        x[i] ^= 0x01;
        assert!(verifier_visa(&visa, &x, AUDIENCE, &pol, T0).is_err(), "passeport octet {i}");
    }
    for i in 0..visa.len() {
        let mut x = visa.clone();
        x[i] ^= 0x01;
        assert!(verifier_visa(&x, &p, AUDIENCE, &pol, T0).is_err(), "visa octet {i}");
    }
    assert!(verifier_visa(&visa[..visa.len() - 1], &p, AUDIENCE, &pol, T0).is_err());
    let mut long = p.clone();
    long.push(0);
    assert!(verifier_visa(&visa, &long, AUDIENCE, &pol, T0).is_err());
    // Passeport présenté comme visa (séparation de domaine par type).
    assert_eq!(
        verifier_visa(&p, &p, AUDIENCE, &pol, T0).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::TypeInattendu { attendu: 0x02, trouve: 0x01 })
    );
}

/// @id MAUTH_PV_061
/// @role test
/// @layer toolkit
/// @do mauth_pv_061_portees_canoniques
//@human / @human Portées : tri et dédoublonnage, refus des portées vides ou avec espaces.
#[test]
fn mauth_pv_061_portees_canoniques() {
    let p = Portees::new(["b", "a", "b"]).unwrap();
    assert_eq!(p.as_slice(), ["a".to_string(), "b".to_string()]);
    assert!(p.contient("a") && !p.contient("c"));
    assert!(Portees::new([""]).is_none());
    assert!(Portees::new(["a b"]).is_none());
    assert!(Portees::new((0..33).map(|i| format!("p{i:02}"))).is_none());
}
