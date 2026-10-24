//! Tests du schéma HTTP réutilisable (feature `http`) : passeport → visa → requête prouvée.
//!
//! @id     miyauth_http_visa_tests
//! @role   test
//! @layer  toolkit
//! @do     tester_schema_http_visa_pop
#![cfg(feature = "http")]

use miyauth::http::{
    formater_autorisation_passeport, formater_autorisation_visa, formater_pop, liaison_requete, signer_pop,
    verifier_requete_passeport, verifier_requete_visa, MagasinDefis, MagasinDefisMemoire, RefusHttp, RequeteHttp,
};
use miyauth::{
    emettre_passeport, emettre_visa, AnnuaireEmetteurs, CogId, DemandePasseport, DemandeVisa, IdentityRole,
    ListeRevocation, PolitiqueIdentite, Portees, RefusIdentite, RevocationsFederees, SecretTitulaire,
};
use miyusceau::{CleSignature, NiveauConfiance};

const T0: u64 = 1_800_000_000;
const AUD: &str = "forum";

fn cog(c: char) -> CogId {
    CogId::new(&c.to_string().repeat(26)).unwrap()
}

struct Monde {
    h: CogId,
    v: CogId,
    cle_h: CleSignature,
    cle_v: CleSignature,
    appareil: CleSignature,
    secret: SecretTitulaire,
    annuaire: AnnuaireEmetteurs,
    revocations: RevocationsFederees,
    defis: MagasinDefisMemoire,
}

impl Monde {
    fn new() -> Self {
        let (h, v) = (cog('h'), cog('v'));
        let (cle_h, cle_v) = (CleSignature::generer(), CleSignature::generer());
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire.ajouter(h.clone(), &cle_h.cle_publique(), NiveauConfiance::Reconnu).unwrap();
        annuaire.ajouter(v.clone(), &cle_v.cle_publique(), NiveauConfiance::Souverain).unwrap();
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
            annuaire,
            revocations,
            defis: MagasinDefisMemoire::default(),
        }
    }

    fn politique(&self) -> PolitiqueIdentite<'_> {
        PolitiqueIdentite { cog_local: self.v.clone(), annuaire: &self.annuaire, revocations: &self.revocations }
    }

    fn passeport(&self) -> Vec<u8> {
        let d = DemandePasseport {
            secret: &self.secret,
            destination: self.v.clone(),
            cle_appareil: self.appareil.cle_publique(),
            portees: Portees::new(["forum:ecrire"]).unwrap(),
            pas_avant: T0 - 60,
            pas_apres: T0 + 3600,
        };
        emettre_passeport(&self.h, &self.cle_h, &d).unwrap()
    }

    /// En-tête PoP pour la requête `(methode, chemin, corps)`.
    fn pop(&self, methode: &str, chemin: &str, corps: &[u8]) -> String {
        let d = self.defis.emettre(AUD, T0).unwrap();
        formater_pop(&d.id, &signer_pop(&self.appareil, &d.defi, AUD, &liaison_requete(methode, chemin, corps)))
    }
}

#[test]
fn passeport_puis_visa_puis_requete_prouvee() {
    let m = Monde::new();
    let p = m.passeport();
    let auth = formater_autorisation_passeport(&p);
    let pop = m.pop("POST", "/login", b"");
    let req = RequeteHttp { methode: "POST", chemin: "/login", corps: b"", autorisation: Some(&auth), pop: Some(&pop), liaison: None };
    let possession = verifier_requete_passeport(&req, AUD, &m.politique(), &m.defis, T0).unwrap();
    let demande = DemandeVisa { audience: AUD.into(), portees: Portees::new(["forum:ecrire"]).unwrap(), pas_avant: T0, pas_apres: T0 + 900 };
    let visa = emettre_visa(&m.v, &m.cle_v, &possession, &demande).unwrap();

    let auth = formater_autorisation_visa(&visa, &p);
    let corps = br#"{"message":"bonjour"}"#;
    let pop = m.pop("POST", "/poster", corps);
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps, autorisation: Some(&auth), pop: Some(&pop), liaison: None };
    let acces = verifier_requete_visa(&req, AUD, &m.politique(), &m.defis, T0 + 1).unwrap();
    assert_eq!(acces.role(), IdentityRole::Visitor);
    assert_eq!(acces.cog_origine(), &m.h);

    // Rejeu : défi consommé.
    assert_eq!(verifier_requete_visa(&req, AUD, &m.politique(), &m.defis, T0 + 1).err(), Some(RefusHttp::DefiInvalide));
}

#[test]
fn requete_visa_refusee_si_alteree_expiree_ou_sans_preuve() {
    let m = Monde::new();
    let p = m.passeport();
    let auth = formater_autorisation_passeport(&p);
    let pop = m.pop("POST", "/login", b"");
    let req = RequeteHttp { methode: "POST", chemin: "/login", corps: b"", autorisation: Some(&auth), pop: Some(&pop), liaison: None };
    let possession = verifier_requete_passeport(&req, AUD, &m.politique(), &m.defis, T0).unwrap();
    let demande = DemandeVisa { audience: AUD.into(), portees: Portees::default(), pas_avant: T0, pas_apres: T0 + 600 };
    let visa = emettre_visa(&m.v, &m.cle_v, &possession, &demande).unwrap();
    let auth = formater_autorisation_visa(&visa, &p);
    let politique = m.politique();

    // Corps différent de celui lié par la preuve.
    let pop = m.pop("POST", "/poster", b"a");
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps: b"b", autorisation: Some(&auth), pop: Some(&pop), liaison: None };
    assert_eq!(verifier_requete_visa(&req, AUD, &politique, &m.defis, T0).err(), Some(RefusHttp::Identite(RefusIdentite::PossessionInvalide)));

    // Sans PoP / sans Authorization.
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps: b"", autorisation: Some(&auth), pop: None, liaison: None };
    assert!(matches!(verifier_requete_visa(&req, AUD, &politique, &m.defis, T0), Err(RefusHttp::EnteteAbsente(_))));
    let pop = m.pop("POST", "/poster", b"");
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps: b"", autorisation: None, pop: Some(&pop), liaison: None };
    assert!(matches!(verifier_requete_visa(&req, AUD, &politique, &m.defis, T0), Err(RefusHttp::EnteteAbsente(_))));

    // Visa expiré (au-delà de la tolérance d'horloge).
    let pop = m.pop("POST", "/poster", b"");
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps: b"", autorisation: Some(&auth), pop: Some(&pop), liaison: None };
    assert!(matches!(verifier_requete_visa(&req, AUD, &politique, &m.defis, T0 + 600 + 121), Err(RefusHttp::DefiInvalide | RefusHttp::Identite(_))));

    // Visa forgé (signé par une clé non épinglée).
    let faux = emettre_visa(&m.v, &CleSignature::generer(), &possession, &demande).unwrap();
    let auth_faux = formater_autorisation_visa(&faux, &p);
    let pop = m.pop("POST", "/poster", b"");
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps: b"", autorisation: Some(&auth_faux), pop: Some(&pop), liaison: None };
    assert!(matches!(verifier_requete_visa(&req, AUD, &politique, &m.defis, T0), Err(RefusHttp::Identite(_))));

    // Jeton au porteur à l'ancienne.
    let pop = m.pop("POST", "/poster", b"");
    let req = RequeteHttp { methode: "POST", chemin: "/poster", corps: b"", autorisation: Some("Bearer abc"), pop: Some(&pop), liaison: None };
    assert!(matches!(verifier_requete_visa(&req, AUD, &politique, &m.defis, T0), Err(RefusHttp::EnteteMalFormee(_))));
}

#[test]
fn liaison_de_canal_fournie_par_le_service() {
    let m = Monde::new();
    let p = m.passeport();
    let auth = formater_autorisation_passeport(&p);
    let exporteur = [42u8; 32];
    let d = m.defis.emettre(AUD, T0).unwrap();
    let pop = formater_pop(&d.id, &signer_pop(&m.appareil, &d.defi, AUD, &exporteur));
    let req = RequeteHttp { methode: "POST", chemin: "/login", corps: b"", autorisation: Some(&auth), pop: Some(&pop), liaison: Some(exporteur) };
    assert!(verifier_requete_passeport(&req, AUD, &m.politique(), &m.defis, T0).is_ok());
    // Même preuve, autre canal : refus.
    let d = m.defis.emettre(AUD, T0).unwrap();
    let pop = formater_pop(&d.id, &signer_pop(&m.appareil, &d.defi, AUD, &exporteur));
    let req = RequeteHttp { liaison: Some([43u8; 32]), pop: Some(&pop), ..req };
    assert!(verifier_requete_passeport(&req, AUD, &m.politique(), &m.defis, T0).is_err());
}
