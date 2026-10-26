//! Chaîne racine → clé d'usage (A-022 §1 et §2) : passeports signés par
//! l'indice 1, visas par l'indice 5, chacun certifié par une délégation `0x45`
//! signée par l'identité épinglée.
//!
//! @id     miyauth_delegation_chaine_tests
//! @role   test
//! @layer  toolkit
//! @do     tester_chaine_delegation_visas

use ed25519_dalek::{Signer, SigningKey};
use miyauth::{
    emettre_passeport, emettre_visa, prouver_possession, verifier_passeport, verifier_visa,
    AnnuaireEmetteurs, CogId, DemandePasseport, DemandeVisa, IdentityRole, ListeRevocation,
    PolitiqueIdentite, Portees, PossessionProuvee, RefusIdentite, RevocationsFederees,
    SecretTitulaire, INDICE_USAGE_EMISSION, INDICE_USAGE_VISAS,
};
use miyuracine::{Racine, Usage};
use miyusceau::{
    cog_id_depuis_cle, sceller, CleSignature, DelegationCle, EcrivainTlv, ErreurSceau, LecteurTlv,
    NiveauConfiance, Sceau, Titre, TypeSceau,
};

const T0: u64 = 1_800_000_000;
const JOUR: u64 = 86_400;
const AUDIENCE: &str = "jaykoa";

/// Titre brut (champs arbitraires) d'un type donné, pour forger des charges.
macro_rules! titre_brut {
    ($nom:ident, $type:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct $nom(Vec<(u8, Vec<u8>)>);

        impl Titre for $nom {
            const TYPE: TypeSceau = $type;

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
    };
}
titre_brut!(CertificatBrut, TypeSceau::Certificat);
titre_brut!(VisaBrut, TypeSceau::Visa);

fn champs(octets: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let charge = Sceau::decoder(octets).unwrap().charge;
    miyusceau::decoder_titre::<CertificatBrut>(&charge)
        .unwrap()
        .0
}

/// COG local à racine épinglée ; titulaire citoyen de ce COG.
struct Monde {
    racine: Racine,
    cog: CogId,
    annuaire: AnnuaireEmetteurs,
    revocations: RevocationsFederees,
    appareil: CleSignature,
    secret: SecretTitulaire,
}

impl Monde {
    /// Racine épinglée, délégations `emission` g0 et `visas` g0 enregistrées.
    fn new() -> Self {
        let mut m = Self::sans_delegation();
        m.deleguer(Usage::Emission, 0, T0 - JOUR, T0 + 89 * JOUR)
            .unwrap();
        m.deleguer(Usage::Visas, 0, T0 - JOUR, T0 + 89 * JOUR)
            .unwrap();
        m
    }

    fn sans_delegation() -> Self {
        let racine = Racine::depuis_secret(&[0x51; 32]);
        let cog = CogId::new(&racine.cog_id()).unwrap();
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire
            .epingler_racine(
                cog.clone(),
                &racine.cle_publique_identite(),
                NiveauConfiance::Souverain,
            )
            .unwrap();
        let mut revocations = RevocationsFederees::new();
        revocations.inserer(ListeRevocation::new(cog.clone(), T0, 900));
        Self {
            racine,
            cog,
            annuaire,
            revocations,
            appareil: CleSignature::depuis_graine(&[0x61; 32]),
            secret: SecretTitulaire::generer(),
        }
    }

    fn delegation(&self, usage: Usage, g: u32, debut: u64, fin: u64) -> Vec<u8> {
        self.racine.deleguer(usage, g, debut, fin).unwrap()
    }

    fn deleguer(
        &mut self,
        usage: Usage,
        g: u32,
        debut: u64,
        fin: u64,
    ) -> Result<[u8; 32], RefusIdentite> {
        let d = self.delegation(usage, g, debut, fin);
        self.annuaire.ajouter_delegation(&d, debut.max(T0 - JOUR))
    }

    fn identite(&self) -> CleSignature {
        self.racine.cle(Usage::Identite, 0).unwrap()
    }

    fn cle(&self, usage: Usage) -> CleSignature {
        self.racine.cle(usage, 0).unwrap()
    }

    fn politique(&self) -> PolitiqueIdentite<'_> {
        self.politique_avec(&self.annuaire)
    }

    fn politique_avec<'a>(&'a self, annuaire: &'a AnnuaireEmetteurs) -> PolitiqueIdentite<'a> {
        PolitiqueIdentite {
            cog_local: self.cog.clone(),
            annuaire,
            revocations: &self.revocations,
        }
    }

    fn passeport_signe_par(&self, cle: &CleSignature) -> Vec<u8> {
        let demande = DemandePasseport {
            secret: &self.secret,
            destination: self.cog.clone(),
            cle_appareil: self.appareil.cle_publique(),
            portees: Portees::new(["jaykoa:read"]).unwrap(),
            pas_avant: T0 - JOUR,
            pas_apres: T0 + 20 * JOUR,
        };
        emettre_passeport(&self.cog, cle, &demande).unwrap()
    }

    fn passeport(&self) -> Vec<u8> {
        self.passeport_signe_par(&self.cle(Usage::Emission))
    }

    fn possession(&self, passeport: &[u8]) -> PossessionProuvee {
        let pv = verifier_passeport(passeport, &self.politique(), T0).unwrap();
        let (defi, liaison) = ([7u8; 32], [9u8; 32]);
        let sig = SigningKey::from_bytes(&self.appareil.graine())
            .sign(&miyauth::message_possession(&defi, "visa", &liaison))
            .to_bytes();
        prouver_possession(pv, &defi, "visa", &liaison, &sig).unwrap()
    }

    fn visa_signe_par(&self, passeport: &[u8], cle: &CleSignature) -> Vec<u8> {
        let demande = DemandeVisa {
            audience: AUDIENCE.to_string(),
            portees: Portees::new(["jaykoa:read"]).unwrap(),
            pas_avant: T0,
            pas_apres: T0 + 15 * 60,
        };
        emettre_visa(&self.cog, cle, &self.possession(passeport), &demande).unwrap()
    }
}

#[test]
fn indices_conformes_a_miyuracine() {
    assert_eq!(INDICE_USAGE_EMISSION, Usage::Emission.indice());
    assert_eq!(INDICE_USAGE_VISAS, Usage::Visas.indice());
}

#[test]
fn delegation_valide_acceptee() {
    let m = Monde::new();
    let p = m.passeport();
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Visas));
    let acces = verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap();
    assert_eq!(acces.role(), IdentityRole::Citizen);
    assert_eq!(acces.passeport().niveau(), NiveauConfiance::Souverain);
    // Liens du visa : passeport, clé d'appareil, audience (service), possession.
    assert_eq!(acces.visa().passeport, *acces.passeport().empreinte());
    assert_eq!(acces.visa().cle_appareil, m.appareil.cle_publique());
    assert_eq!(
        verifier_visa(&visa, &p, "autre", &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::AudienceIncorrecte
    );
}

#[test]
fn delegation_presentee_avec_le_titre_acceptee() {
    let mut m = Monde::sans_delegation();
    m.deleguer(Usage::Emission, 0, T0 - JOUR, T0 + 89 * JOUR)
        .unwrap();
    let p = m.passeport();
    let d_visas = m.delegation(Usage::Visas, 0, T0 - JOUR, T0 + 89 * JOUR);
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Visas));
    // Sans la délégation : clé inconnue.
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::SignataireInconnu)
    );
    // Présentée avec le visa : acceptée, l'annuaire d'origine inchangé.
    let augmente = m.annuaire.avec_delegations(&[&d_visas], T0).unwrap();
    assert!(verifier_visa(&visa, &p, AUDIENCE, &m.politique_avec(&augmente), T0 + 60).is_ok());
    assert!(verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).is_err());
}

#[test]
fn delegation_signee_par_une_autre_identite_refusee() {
    let mut m = Monde::sans_delegation();
    let autre = Racine::depuis_secret(&[0x52; 32]);
    // Autre racine, son propre cog_id (non épinglé) : refus.
    let d = autre.deleguer(Usage::Visas, 0, T0, T0 + JOUR).unwrap();
    assert_eq!(
        m.annuaire.ajouter_delegation(&d, T0).unwrap_err(),
        RefusIdentite::DelegationIncoherente
    );
    // Autre identité signant pour le cog_id épinglé : signataire inconnu.
    let forgee = DelegationCle::nouvelle(
        m.cog.as_str(),
        5,
        0,
        m.cle(Usage::Visas).cle_publique(),
        T0,
        T0 + JOUR,
    )
    .unwrap();
    let octets = sceller(&forgee, &autre.cle(Usage::Identite, 0).unwrap()).unwrap();
    assert_eq!(
        m.annuaire.ajouter_delegation(&octets, T0).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::SignataireInconnu)
    );
    // Une clé d'usage (même du bon COG) ne délègue pas : seule l'identité.
    let octets = sceller(&forgee, &m.cle(Usage::Emission)).unwrap();
    assert!(m.annuaire.ajouter_delegation(&octets, T0).is_err());
}

#[test]
fn indice_faux_refuse() {
    let mut m = Monde::new();
    let p = m.passeport();
    // Visa signé par la clé d'indice 1 (emission) : refus, sans transition.
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Emission));
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::UsageIncorrect {
            attendu: INDICE_USAGE_VISAS,
            delegue: INDICE_USAGE_EMISSION
        }
    );
    // Passeport signé par la clé de visas : refus.
    let p_visas = m.passeport_signe_par(&m.cle(Usage::Visas));
    assert_eq!(
        verifier_passeport(&p_visas, &m.politique(), T0).unwrap_err(),
        RefusIdentite::UsageIncorrect {
            attendu: INDICE_USAGE_EMISSION,
            delegue: INDICE_USAGE_VISAS
        }
    );
    // Visa signé par une autre clé d'usage déléguée (journal, indice 2) : refus.
    m.deleguer(Usage::Journal, 0, T0 - JOUR, T0 + 89 * JOUR)
        .unwrap();
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Journal));
    assert!(matches!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::UsageIncorrect {
            attendu: 5,
            delegue: 2
        }
    ));
    // L'identité elle-même ne signe pas de titre.
    let p_id = m.passeport_signe_par(&m.identite());
    assert_eq!(
        verifier_passeport(&p_id, &m.politique(), T0).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::SignataireInconnu)
    );
}

#[test]
fn hors_fenetre_refuse() {
    let mut m = Monde::sans_delegation();
    // Délégation expirée à l'enregistrement.
    let d = m.delegation(Usage::Visas, 0, T0 - 30 * JOUR, T0 - 2 * JOUR);
    assert_eq!(
        m.annuaire.ajouter_delegation(&d, T0).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::Expire)
    );
    // Délégation d'émission plus courte que le passeport : refus du passeport.
    let d = m.delegation(Usage::Emission, 0, T0 - JOUR, T0 + JOUR);
    m.annuaire.ajouter_delegation(&d, T0).unwrap();
    assert_eq!(
        verifier_passeport(&m.passeport(), &m.politique(), T0).unwrap_err(),
        RefusIdentite::DelegationHorsFenetre
    );
    // Délégation de visas commençant après l'instant de vérification.
    let mut m = Monde::sans_delegation();
    m.deleguer(Usage::Emission, 0, T0 - JOUR, T0 + 89 * JOUR)
        .unwrap();
    let d = m.delegation(Usage::Visas, 0, T0 + 3_600, T0 + 30 * JOUR);
    m.annuaire.ajouter_delegation(&d, T0 + 3_600).unwrap();
    let p = m.passeport();
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Visas));
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::DelegationHorsFenetre
    );
}

fn forger_delegation(m: &Monde, modifier: impl FnOnce(&mut Vec<(u8, Vec<u8>)>)) -> Vec<u8> {
    let mut c = champs(&m.delegation(Usage::Visas, 0, T0, T0 + JOUR));
    modifier(&mut c);
    sceller(&CertificatBrut(c), &m.identite()).unwrap()
}

#[test]
fn plus_de_90_jours_refuse() {
    let mut m = Monde::sans_delegation();
    let octets = forger_delegation(&m, |c| {
        c[6] = (7, (T0 + 90 * JOUR + 1).to_be_bytes().to_vec());
    });
    assert_eq!(
        m.annuaire.ajouter_delegation(&octets, T0).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::ValeurInvalide(7))
    );
    assert!(m
        .racine
        .deleguer(Usage::Visas, 0, T0, T0 + 90 * JOUR + 1)
        .is_err());
}

#[test]
fn champ_inconnu_refuse() {
    let mut m = Monde::sans_delegation();
    let octets = forger_delegation(&m, |c| c.push((8, vec![1])));
    assert_eq!(
        m.annuaire.ajouter_delegation(&octets, T0).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::ChampInconnu(8))
    );
}

#[test]
fn generation_incoherente_refusee() {
    let mut m = Monde::new();
    let id = m.identite();
    // Même (cog, indice 5, génération 0), autre clé : refus.
    let intruse = CleSignature::depuis_graine(&[0x77; 32]);
    let d = DelegationCle::nouvelle(m.cog.as_str(), 5, 0, intruse.cle_publique(), T0, T0 + JOUR)
        .unwrap();
    assert_eq!(
        m.annuaire
            .ajouter_delegation(&d.sceller(&id).unwrap(), T0)
            .unwrap_err(),
        RefusIdentite::DelegationIncoherente
    );
    // Clé de visas g0 re-déléguée sous une autre génération ou un autre indice : refus.
    let cle_visas = m.cle(Usage::Visas).cle_publique();
    for (indice, generation) in [(5, 1), (1, 7)] {
        let d =
            DelegationCle::nouvelle(m.cog.as_str(), indice, generation, cle_visas, T0, T0 + JOUR)
                .unwrap();
        assert_eq!(
            m.annuaire
                .ajouter_delegation(&d.sceller(&id).unwrap(), T0)
                .unwrap_err(),
            RefusIdentite::DelegationIncoherente,
            "indice {indice}, génération {generation}"
        );
    }
    // Rotation : génération 1 (autre clé dérivée) acceptée à côté de la 0.
    m.deleguer(Usage::Visas, 1, T0 - JOUR, T0 + 89 * JOUR)
        .unwrap();
    let p = m.passeport();
    let visa = m.visa_signe_par(&p, &m.racine.cle(Usage::Visas, 1).unwrap());
    assert!(verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).is_ok());
    // Renouvellement de la même délégation (même clé, même génération) : accepté.
    m.deleguer(Usage::Visas, 0, T0 - JOUR, T0 + 80 * JOUR)
        .unwrap();
}

#[test]
fn visa_sans_delegation_refuse() {
    let m = Monde::sans_delegation();
    let mut annuaire = m.annuaire.clone();
    // Le COG a une racine épinglée : aucune ancre directe admise.
    assert!(annuaire
        .ajouter(
            m.cog.clone(),
            &m.cle(Usage::Visas).cle_publique(),
            NiveauConfiance::Souverain
        )
        .is_err());
    // Visa d'une clé non déléguée : signataire inconnu.
    let mut m2 = Monde::sans_delegation();
    m2.deleguer(Usage::Emission, 0, T0 - JOUR, T0 + 89 * JOUR)
        .unwrap();
    let p = m2.passeport();
    let visa = m2.visa_signe_par(&p, &m2.cle(Usage::Visas));
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &m2.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::SignataireInconnu)
    );
}

#[test]
fn transition_ancres_directes_puis_mode_strict() {
    // COG sans racine épinglée : l'ancre directe (mode antérieur) signe encore passeports et visas.
    let cle = CleSignature::depuis_graine(&[0x81; 32]);
    let cog = CogId::new(&cog_id_depuis_cle(&cle.cle_publique())).unwrap();
    let mut annuaire = AnnuaireEmetteurs::new();
    annuaire
        .ajouter(cog.clone(), &cle.cle_publique(), NiveauConfiance::Souverain)
        .unwrap();
    let mut revocations = RevocationsFederees::new();
    revocations.inserer(ListeRevocation::new(cog.clone(), T0, 900));
    let appareil = CleSignature::depuis_graine(&[0x82; 32]);
    let secret = SecretTitulaire::generer();
    let p = emettre_passeport(
        &cog,
        &cle,
        &DemandePasseport {
            secret: &secret,
            destination: cog.clone(),
            cle_appareil: appareil.cle_publique(),
            portees: Portees::new(["jaykoa:read"]).unwrap(),
            pas_avant: T0 - JOUR,
            pas_apres: T0 + JOUR,
        },
    )
    .unwrap();
    let politique = PolitiqueIdentite {
        cog_local: cog.clone(),
        annuaire: &annuaire,
        revocations: &revocations,
    };
    assert!(verifier_passeport(&p, &politique, T0).is_ok());
    // Mode strict : l'ancre directe est refusée.
    let mut strict = annuaire.clone();
    strict.exiger_delegations();
    let politique = PolitiqueIdentite {
        cog_local: cog.clone(),
        annuaire: &strict,
        revocations: &revocations,
    };
    assert_eq!(
        verifier_passeport(&p, &politique, T0).unwrap_err(),
        RefusIdentite::SansDelegation
    );
    assert!(strict
        .ajouter(
            cog.clone(),
            &appareil.cle_publique(),
            NiveauConfiance::Souverain
        )
        .is_err());
    // Racine épinglée après une ancre directe du même COG : refus (pas de mélange).
    let racine = Racine::depuis_secret(&[0x83; 32]);
    let cog_r = CogId::new(&racine.cog_id()).unwrap();
    let mut a = AnnuaireEmetteurs::new();
    a.ajouter(
        cog_r.clone(),
        &cle.cle_publique(),
        NiveauConfiance::Souverain,
    )
    .unwrap();
    assert_eq!(
        a.epingler_racine(
            cog_r,
            &racine.cle_publique_identite(),
            NiveauConfiance::Souverain
        )
        .unwrap_err(),
        RefusIdentite::RacineIncoherente
    );
    // cog_id qui n'est pas l'empreinte de la clé d'identité : refus.
    assert_eq!(
        AnnuaireEmetteurs::new()
            .epingler_racine(
                cog,
                &racine.cle_publique_identite(),
                NiveauConfiance::Souverain
            )
            .unwrap_err(),
        RefusIdentite::RacineIncoherente
    );
}

#[test]
fn visa_de_plus_de_15_minutes_refuse() {
    let m = Monde::new();
    let p = m.passeport();
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Visas));
    // Émission refusée au-delà de 15 min.
    let demande = DemandeVisa {
        audience: AUDIENCE.to_string(),
        portees: Portees::new(["jaykoa:read"]).unwrap(),
        pas_avant: T0,
        pas_apres: T0 + 15 * 60 + 1,
    };
    assert!(emettre_visa(&m.cog, &m.cle(Usage::Visas), &m.possession(&p), &demande).is_err());
    // Visa forgé de 15 min + 1 s, signé par la clé de visas déléguée : refusé.
    let charge = Sceau::decoder(&visa).unwrap().charge;
    let mut c = miyusceau::decoder_titre::<VisaBrut>(&charge).unwrap().0;
    let fin = c.iter_mut().find(|(t, _)| *t == 10).unwrap();
    fin.1 = (T0 + 15 * 60 + 1).to_be_bytes().to_vec();
    let forge = sceller(&VisaBrut(c), &m.cle(Usage::Visas)).unwrap();
    assert_eq!(
        verifier_visa(&forge, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::Sceau(ErreurSceau::ValeurInvalide(10))
    );
}

#[test]
fn cog_reconnu_a_racine_epinglee() {
    // COG d'origine reconnu (racine Reconnu + délégation d'émission), visa du COG local.
    let mut m = Monde::new();
    let origine = Racine::depuis_secret(&[0x91; 32]);
    let cog_o = CogId::new(&origine.cog_id()).unwrap();
    m.annuaire
        .epingler_racine(
            cog_o.clone(),
            &origine.cle_publique_identite(),
            NiveauConfiance::Reconnu,
        )
        .unwrap();
    m.annuaire
        .ajouter_delegation(
            &origine
                .deleguer(Usage::Emission, 0, T0 - JOUR, T0 + 89 * JOUR)
                .unwrap(),
            T0,
        )
        .unwrap();
    m.revocations
        .inserer(ListeRevocation::new(cog_o.clone(), T0, 900));
    let p = emettre_passeport(
        &cog_o,
        &origine.cle(Usage::Emission, 0).unwrap(),
        &DemandePasseport {
            secret: &m.secret,
            destination: m.cog.clone(),
            cle_appareil: m.appareil.cle_publique(),
            portees: Portees::new(["jaykoa:read"]).unwrap(),
            pas_avant: T0 - JOUR,
            pas_apres: T0 + 20 * JOUR,
        },
    )
    .unwrap();
    let visa = m.visa_signe_par(&p, &m.cle(Usage::Visas));
    let acces = verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap();
    assert_eq!(acces.role(), IdentityRole::Visitor);
    assert_eq!(acces.passeport().niveau(), NiveauConfiance::Reconnu);
    // La clé de visas du COG reconnu ne signe pas de visa pour le COG local.
    m.annuaire
        .ajouter_delegation(
            &origine
                .deleguer(Usage::Visas, 0, T0 - JOUR, T0 + 89 * JOUR)
                .unwrap(),
            T0,
        )
        .unwrap();
    let visa = m.visa_signe_par(&p, &origine.cle(Usage::Visas, 0).unwrap());
    assert_eq!(
        verifier_visa(&visa, &p, AUDIENCE, &m.politique(), T0 + 60).unwrap_err(),
        RefusIdentite::VisaEtranger
    );
}
