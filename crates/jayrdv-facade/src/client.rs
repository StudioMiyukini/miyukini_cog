//! @id     jayrdv_facade_client
//! @do     construire_requetes_appareil_federe
//! @role   service
//! @layer  service
//! @human  jayrdv-facade — client de l'API publique pour un appareil fédéré : visa + PoP, déclarations du traité signées par la clé d'appareil.

//! # Client d'appareil fédéré
//!
//! L'appareil détient un passeport de son COG (reconnu par le COG hôte) et un
//! visa du COG hôte pour l'audience du service. Pour chaque demande il :
//!
//! 1. obtient un défi (`POST /v1/defi`) ;
//! 2. construit la **déclaration du traité** (mêmes codecs que la façade et
//!    le moteur) et la **signe** avec sa clé d'appareil (auteur, §3 contrôle 2) ;
//! 3. envoie les champs typés + la signature, avec `Authorization: MiyuVisa`
//!    et `X-Miyu-PoP` liés à la requête.

use jayrdv_traite::{DemandeCreneaux, Reservation, SERVICE, VERSION};
use miyauth::http::{
    b64_decoder, b64_encoder, formater_autorisation_visa, formater_delegations, formater_pop, liaison_requete,
    signer_pop, IdDefi, ENTETE_DELEGATIONS,
};
use miyuhybride::{
    declaration_entrante, liaison_demande_visa, portees_texte, signer_auteur, ErreurHybride, IdService, MessageTraite,
};
use miyusceau::CleSignature;

use crate::api::{DefiJson, DemandeCreneauxJson, DemandeVisaJson, RequeteHttp, ReservationJson, VisaJson};

/// Défi reçu de la façade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefiRecu {
    id: IdDefi,
    defi: [u8; 32],
}

impl DefiRecu {
    /// Décode la réponse de `POST /v1/defi`.
    pub fn depuis(corps: &[u8]) -> Option<Self> {
        let d: DefiJson = serde_json::from_slice(corps).ok()?;
        Some(Self { id: b64_decoder(&d.id)?.try_into().ok()?, defi: b64_decoder(&d.defi)?.try_into().ok()? })
    }
}

/// Visa reçu du guichet (`GET /v1/visa/<demande>`, 200).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisaRecu {
    /// Visa scellé.
    pub visa: Vec<u8>,
    /// Délégation `0x45` de la clé `visas`.
    pub delegation: Vec<u8>,
    /// Fin de validité.
    pub pas_apres: u64,
}

impl VisaRecu {
    /// Décode la réponse 200 de `GET /v1/visa/<demande>`.
    pub fn depuis(corps: &[u8]) -> Option<Self> {
        let v: VisaJson = serde_json::from_slice(corps).ok()?;
        Some(Self { visa: b64_decoder(&v.visa)?, delegation: b64_decoder(&v.delegation)?, pas_apres: v.pas_apres })
    }
}

/// Paramètres d'une demande de visa.
#[derive(Debug, Clone)]
pub struct ParametresDemandeVisa<'a> {
    /// Passeport.
    pub passeport: &'a [u8],
    /// Délégation de la clé `emission` du COG d'origine (si sa racine est épinglée).
    pub delegation_passeport: Option<&'a [u8]>,
    /// Portées demandées.
    pub portees: &'a [&'a str],
    /// Durée demandée (secondes).
    pub duree_secs: u64,
    /// Date d'émission.
    pub emise_le: u64,
}

/// `POST /v1/visa` : demande de visa signée par la clé d'appareil (preuve de
/// possession sur tout le contenu, défi `defi` de la façade).
pub fn requete_demande_visa(cle: &CleSignature, defi: &DefiRecu, p: &ParametresDemandeVisa<'_>) -> RequeteHttp {
    let portees = portees_texte(p.portees);
    let cle_appareil = cle.cle_publique();
    let liaison = liaison_demande_visa(p.passeport, &cle_appareil, SERVICE, &portees, p.duree_secs, p.emise_le);
    let preuve = signer_pop(cle, &defi.defi, SERVICE, &liaison);
    let corps = DemandeVisaJson {
        passeport: b64_encoder(p.passeport),
        delegation_passeport: p.delegation_passeport.map(b64_encoder),
        cle_appareil: b64_encoder(&cle_appareil),
        portees: p.portees.iter().map(ToString::to_string).collect(),
        duree_secs: p.duree_secs,
        emise_le: p.emise_le,
        defi: b64_encoder(&defi.defi),
        preuve: b64_encoder(&preuve),
    };
    RequeteHttp {
        methode: "POST".into(),
        chemin: "/v1/visa".into(),
        entetes: Vec::new(),
        corps: serde_json::to_vec(&corps).unwrap_or_default(),
    }
}

/// `GET /v1/visa/<demande>`.
pub fn requete_lire_visa(demande: &str) -> RequeteHttp {
    RequeteHttp { methode: "GET".into(), chemin: format!("/v1/visa/{demande}"), entetes: Vec::new(), corps: Vec::new() }
}

/// Client d'un appareil fédéré (détient la clé d'appareil, le visa, le passeport).
#[derive(Debug)]
pub struct ClientFedere {
    cle: CleSignature,
    visa: Vec<u8>,
    passeport: Vec<u8>,
    delegations: Vec<Vec<u8>>,
    sequence: u64,
}

impl ClientFedere {
    /// Client avec ses titres.
    pub fn new(cle: CleSignature, visa: Vec<u8>, passeport: Vec<u8>) -> Self {
        Self { cle, visa, passeport, delegations: Vec::new(), sequence: 0 }
    }

    /// Délégations `0x45` présentées avec le visa (`X-Miyu-Delegations`).
    pub fn avec_delegations(mut self, delegations: Vec<Vec<u8>>) -> Self {
        self.delegations = delegations;
        self
    }

    /// Clé publique d'appareil (auteur épinglé par les Cores).
    pub fn cle_appareil(&self) -> [u8; 32] {
        self.cle.cle_publique()
    }

    /// Remplace le visa (renouvellement).
    pub fn changer_visa(&mut self, visa: Vec<u8>) {
        self.visa = visa;
    }

    /// `POST /v1/defi`.
    pub fn requete_defi() -> RequeteHttp {
        RequeteHttp { methode: "POST".into(), chemin: "/v1/defi".into(), entetes: Vec::new(), corps: Vec::new() }
    }

    fn authentifiee(&self, defi: &DefiRecu, methode: &str, chemin: &str, corps: Vec<u8>) -> RequeteHttp {
        let pop = signer_pop(&self.cle, &defi.defi, SERVICE, &liaison_requete(methode, chemin, &corps));
        let mut entetes = vec![
            ("authorization".into(), formater_autorisation_visa(&self.visa, &self.passeport)),
            ("x-miyu-pop".into(), formater_pop(&defi.id, &pop)),
        ];
        if !self.delegations.is_empty() {
            let d: Vec<&[u8]> = self.delegations.iter().map(Vec::as_slice).collect();
            entetes.push((ENTETE_DELEGATIONS.into(), formater_delegations(&d)));
        }
        RequeteHttp { methode: methode.into(), chemin: chemin.into(), entetes, corps }
    }

    fn signer<M: MessageTraite>(&mut self, m: &M, emise_le: u64) -> Result<(u64, String), ErreurHybride> {
        self.sequence += 1;
        let service = IdService::new(SERVICE)?;
        let d = declaration_entrante(&service, VERSION, m, self.sequence, emise_le)?;
        let (_, signature) = signer_auteur(&self.cle, &d)?;
        Ok((self.sequence, b64_encoder(&signature)))
    }

    /// `POST /v1/creneaux`.
    pub fn demander_creneaux(
        &mut self,
        defi: &DefiRecu,
        debut: &str,
        fin: &str,
        emise_le: u64,
    ) -> Result<RequeteHttp, ErreurHybride> {
        let m = DemandeCreneaux { debut: debut.into(), fin: fin.into(), ressource: None };
        let (sequence, signature) = self.signer(&m, emise_le)?;
        let corps = DemandeCreneauxJson { sequence, emise_le, debut: m.debut, fin: m.fin, ressource: None, signature };
        Ok(self.authentifiee(defi, "POST", "/v1/creneaux", serde_json::to_vec(&corps).unwrap_or_default()))
    }

    /// `POST /v1/reservations`.
    pub fn reserver(
        &mut self,
        defi: &DefiRecu,
        creneau: &str,
        nom: &str,
        contact: Option<&str>,
        emise_le: u64,
    ) -> Result<RequeteHttp, ErreurHybride> {
        let m = Reservation { creneau: creneau.into(), nom: nom.into(), contact: contact.map(Into::into), note: None };
        let (sequence, signature) = self.signer(&m, emise_le)?;
        let corps = ReservationJson {
            sequence,
            emise_le,
            creneau: m.creneau,
            nom: m.nom,
            contact: m.contact,
            note: None,
            signature,
        };
        Ok(self.authentifiee(defi, "POST", "/v1/reservations", serde_json::to_vec(&corps).unwrap_or_default()))
    }

    /// `GET /v1/reponses/<demande>`.
    pub fn lire_reponse(&self, defi: &DefiRecu, demande: &str) -> RequeteHttp {
        self.authentifiee(defi, "GET", &format!("/v1/reponses/{demande}"), Vec::new())
    }

    /// `DELETE /v1/session` : fin explicite de la session cliente.
    pub fn fermer_session(&self, defi: &DefiRecu) -> RequeteHttp {
        self.authentifiee(defi, "DELETE", "/v1/session", Vec::new())
    }

    /// `GET /v1/creneaux` (cache de la façade).
    pub fn lire_cache(&self, defi: &DefiRecu) -> RequeteHttp {
        self.authentifiee(defi, "GET", "/v1/creneaux", Vec::new())
    }
}
