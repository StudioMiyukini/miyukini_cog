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
    b64_decoder, b64_encoder, formater_autorisation_visa, formater_pop, liaison_requete, signer_pop, IdDefi,
};
use miyuhybride::{declaration_entrante, signer_auteur, ErreurHybride, IdService, MessageTraite};
use miyusceau::CleSignature;

use crate::api::{DefiJson, DemandeCreneauxJson, RequeteHttp, ReservationJson};

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

/// Client d'un appareil fédéré (détient la clé d'appareil, le visa, le passeport).
#[derive(Debug)]
pub struct ClientFedere {
    cle: CleSignature,
    visa: Vec<u8>,
    passeport: Vec<u8>,
    sequence: u64,
}

impl ClientFedere {
    /// Client avec ses titres.
    pub fn new(cle: CleSignature, visa: Vec<u8>, passeport: Vec<u8>) -> Self {
        Self { cle, visa, passeport, sequence: 0 }
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
        RequeteHttp {
            methode: methode.into(),
            chemin: chemin.into(),
            entetes: vec![
                ("authorization".into(), formater_autorisation_visa(&self.visa, &self.passeport)),
                ("x-miyu-pop".into(), formater_pop(&defi.id, &pop)),
            ],
            corps,
        }
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

    /// `GET /v1/creneaux` (cache de la façade).
    pub fn lire_cache(&self, defi: &DefiRecu) -> RequeteHttp {
        self.authentifiee(defi, "GET", "/v1/creneaux", Vec::new())
    }
}
