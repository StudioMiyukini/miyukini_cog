//! @id     jayrdv_facade_facade
//! @do     terminer_http_et_deposer_traite
//! @role   service
//! @layer  service
//! @human  jayrdv-facade — façade jetable JayRDV : termine HTTP, vérifie visa + PoP, traduit en messages du traité, dépose via le Portail, sert les réponses publiées depuis sa DB secondaire.

//! # Façade JayRDV (Franche)
//!
//! - **INV-H1** côté moteur : c'est ici, et seulement ici, que HTTP et JSON
//!   sont analysés ; le moteur ne reçoit que des messages du traité ;
//! - **INV-H2** : aucune logique métier (aucune décision de réservation), aucun
//!   secret durable — la clé de session est éphémère, émise par la zone COG,
//!   les ancres d'identité sont publiques ;
//! - **INV-H3** : seuls les messages du traité sont déposés ; aucun octet
//!   reçu n'est recopié (les champs typés sont réencodés) ;
//! - **INV-H5** : une coupure, une désactivation ou la fin de session
//!   **condamnent** la façade : DB purgée, tout est refusé (503), la zone COG
//!   la détruit et en reconstruit une neuve ([`miyuhybride::Superviseur`]) ;
//! - DB secondaire : réponses publiées et cache des créneaux publiés, **jamais
//!   autoritaire** (le moteur décide en zone COG à chaque réservation).
//!
//! La façade ne connaît ni l'adresse ni une poignée du moteur ou de
//! KindMother : son seul lien est le [`GuichetFranche`] de sa voie.

use jayrdv_traite::{CreneauDisponible, DemandeCreneaux, Motif, Reservation, Sortant};
use miyauth::http::{b64_decoder, b64_encoder, verifier_requete_visa, MagasinDefis, MagasinDefisMemoire};
use miyauth::{AnnuaireEmetteurs, CogId, PolitiqueIdentite, RevocationsFederees};
use miyuhybride::{
    declaration_entrante, DbSecondaire, ErreurHybride, ErreurPortail, EtatVoie, FacadeJetable, GuichetFranche,
    LancementFacade, MessageTraite, RefusDecodage,
};

use crate::api::{
    AccepteJson, CreneauJson, DefiJson, DemandeCreneauxJson, EnAttenteJson, ReponseHttp, ReponseJson, RequeteHttp,
    ReservationJson, MAX_CORPS,
};

/// Portée exigée dans le visa (choix fail-closed : la doc gelée n'en définit pas pour JayRDV).
pub const PORTEE_RESERVER: &str = "jayrdv:reserver";
/// Nombre maximal de publications lues par requête.
const MAX_LECTURES: usize = 256;
/// Capacité du magasin de défis d'une façade.
const CAPACITE_DEFIS: usize = 1024;

const T_ATTENTE: &str = "attente";
const T_REPONSES: &str = "reponses";
const T_PARTIEL: &str = "partiel";
const T_CRENEAUX: &str = "creneaux";

/// Configuration publique de la façade (aucun secret).
#[derive(Debug, Clone)]
pub struct ConfigurationFacade {
    /// COG hôte (émetteur des visas).
    pub cog_local: CogId,
    /// Clés d'émission épinglées (COG hôte souverain, COG reconnus).
    pub annuaire: AnnuaireEmetteurs,
    /// Instantané des révocations, remis à chaque lancement.
    pub revocations: RevocationsFederees,
}

/// Façade JayRDV : une instance par session, jetable.
#[derive(Debug)]
pub struct FacadeJayRdv {
    guichet: GuichetFranche,
    db: DbSecondaire,
    audience: String,
    duree_max_session_secs: u64,
    config: ConfigurationFacade,
    defis: MagasinDefisMemoire,
    derniere_sequence: Option<u64>,
    ouverte_le: Option<u64>,
    condamnee: bool,
}

impl FacadeJetable for FacadeJayRdv {
    fn condamnee(&self) -> bool {
        self.condamnee
    }
}

fn b64_taille<const N: usize>(texte: &str) -> Option<[u8; N]> {
    b64_decoder(texte)?.try_into().ok()
}

fn motif_texte(code: u64) -> &'static str {
    match Motif::depuis_code(code) {
        Some(Motif::Indisponible) => "indisponible",
        Some(Motif::Inconnu) => "inconnu",
        Some(Motif::Invalide) => "invalide",
        Some(Motif::Interne) | None => "interne",
    }
}

fn cle_partiel(demande: &[u8], rang: u64) -> Vec<u8> {
    let mut k = demande.to_vec();
    k.extend_from_slice(&rang.to_be_bytes());
    k
}

impl FacadeJayRdv {
    /// Façade neuve depuis ce que la zone COG lui remet.
    pub fn new(lancement: LancementFacade, config: ConfigurationFacade) -> Self {
        Self {
            guichet: lancement.guichet,
            db: lancement.db,
            audience: lancement.audience,
            duree_max_session_secs: lancement.duree_max_session_secs,
            config,
            defis: MagasinDefisMemoire::new(CAPACITE_DEFIS, miyauth::http::DUREE_DEFI_SECS),
            derniere_sequence: None,
            ouverte_le: None,
            condamnee: false,
        }
    }

    /// DB secondaire (inspection : cache non autoritaire).
    pub fn db(&self) -> &DbSecondaire {
        &self.db
    }

    /// Clé publique de session (épinglée par la zone COG).
    pub fn cle_session(&self) -> [u8; 32] {
        self.guichet.cle_session()
    }

    fn condamner(&mut self) {
        self.condamnee = true;
        self.db.purger();
    }

    /// Une façade ne sert que sur une voie active, dans sa durée de session.
    fn en_vie(&mut self, maintenant: u64) -> bool {
        if self.condamnee {
            return false;
        }
        let ouverte = *self.ouverte_le.get_or_insert(maintenant);
        let active = matches!(self.guichet.etat(), Ok(EtatVoie::Active));
        if !active || maintenant > ouverte.saturating_add(self.duree_max_session_secs) {
            self.condamner();
            return false;
        }
        true
    }

    /// Traite une requête HTTP (toute erreur = refus).
    pub fn traiter(&mut self, r: &RequeteHttp, maintenant: u64) -> ReponseHttp {
        if !self.en_vie(maintenant) {
            return ReponseHttp::refus(503, "facade_condamnee");
        }
        if r.corps.len() > MAX_CORPS {
            return ReponseHttp::refus(413, "corps_trop_grand");
        }
        match (r.methode.as_str(), r.chemin.as_str()) {
            ("POST", "/v1/defi") if r.corps.is_empty() => self.defi(maintenant),
            ("POST", "/v1/creneaux") => self.demander_creneaux(r, maintenant),
            ("POST", "/v1/reservations") => self.reserver(r, maintenant),
            ("GET", "/v1/creneaux") if r.corps.is_empty() => self.cache_creneaux(r, maintenant),
            ("GET", chemin) if r.corps.is_empty() && chemin.starts_with("/v1/reponses/") => {
                self.reponse(r, &chemin["/v1/reponses/".len()..], maintenant)
            }
            _ => ReponseHttp::refus(404, "inconnu"),
        }
    }

    fn defi(&self, maintenant: u64) -> ReponseHttp {
        match self.defis.emettre(&self.audience, maintenant) {
            Ok(d) => ReponseHttp::json(200, &DefiJson { id: d.id_b64(), defi: d.defi_b64(), expire_a: d.expire_a }),
            Err(e) => ReponseHttp::refus(e.statut(), "defi"),
        }
    }

    /// Visa + PoP pour l'audience du service ; rend la clé d'appareil (= auteur).
    fn authentifier(&self, r: &RequeteHttp, maintenant: u64) -> Result<[u8; 32], ReponseHttp> {
        let requete = miyauth::http::RequeteHttp {
            methode: &r.methode,
            chemin: &r.chemin,
            corps: &r.corps,
            autorisation: r.entete("authorization"),
            pop: r.entete("x-miyu-pop"),
            liaison: None,
        };
        let politique = PolitiqueIdentite {
            cog_local: self.config.cog_local.clone(),
            annuaire: &self.config.annuaire,
            revocations: &self.config.revocations,
        };
        let acces = verifier_requete_visa(&requete, &self.audience, &politique, &self.defis, maintenant)
            .map_err(|e| ReponseHttp::refus(e.statut(), "identite"))?;
        let visa = acces.acces().visa();
        if !visa.portees.contient(PORTEE_RESERVER) {
            return Err(ReponseHttp::refus(403, "portee"));
        }
        if !self.guichet.est_auteur(&visa.cle_appareil) {
            return Err(ReponseHttp::refus(403, "auteur_hors_session"));
        }
        Ok(visa.cle_appareil)
    }

    fn demander_creneaux(&mut self, r: &RequeteHttp, maintenant: u64) -> ReponseHttp {
        let auteur = match self.authentifier(r, maintenant) {
            Ok(a) => a,
            Err(refus) => return refus,
        };
        let Ok(corps) = serde_json::from_slice::<DemandeCreneauxJson>(&r.corps) else {
            return ReponseHttp::refus(400, "corps");
        };
        let message = DemandeCreneaux { debut: corps.debut, fin: corps.fin, ressource: corps.ressource };
        self.deposer(auteur, corps.sequence, corps.emise_le, &message, &corps.signature)
    }

    fn reserver(&mut self, r: &RequeteHttp, maintenant: u64) -> ReponseHttp {
        let auteur = match self.authentifier(r, maintenant) {
            Ok(a) => a,
            Err(refus) => return refus,
        };
        let Ok(corps) = serde_json::from_slice::<ReservationJson>(&r.corps) else {
            return ReponseHttp::refus(400, "corps");
        };
        let message = Reservation { creneau: corps.creneau, nom: corps.nom, contact: corps.contact, note: corps.note };
        self.deposer(auteur, corps.sequence, corps.emise_le, &message, &corps.signature)
    }

    /// Traduit en déclaration du traité, cosigne, dépose en quarantaine.
    fn deposer<M: MessageTraite>(
        &mut self,
        auteur: [u8; 32],
        sequence: u64,
        emise_le: u64,
        message: &M,
        signature: &str,
    ) -> ReponseHttp {
        let Some(signature) = b64_taille::<64>(signature) else {
            return ReponseHttp::refus(400, "signature");
        };
        if self.derniere_sequence.is_some_and(|s| sequence <= s) {
            return ReponseHttp::refus(409, "sequence");
        }
        let Ok(d) =
            declaration_entrante(self.guichet.service(), self.guichet.version_traite(), message, sequence, emise_le)
        else {
            return ReponseHttp::refus(400, "traite");
        };
        match self.guichet.deposer(&d, &(auteur, signature)) {
            Ok(demande) => {
                self.derniere_sequence = Some(sequence);
                if self.db.ecrire(T_ATTENTE, &demande, auteur.to_vec()).is_err() {
                    return ReponseHttp::refus(503, "db_secondaire_pleine");
                }
                ReponseHttp::json(202, &AccepteJson { demande: b64_encoder(&demande) })
            }
            Err(e) => self.refus_portail(&e),
        }
    }

    fn refus_portail(&mut self, e: &ErreurHybride) -> ReponseHttp {
        match e {
            ErreurHybride::Portail(
                ErreurPortail::VoieCoupee
                | ErreurPortail::VoieDesactivee
                | ErreurPortail::SessionAbsente
                | ErreurPortail::VoieNonDeclaree
                | ErreurPortail::EtatIndisponible,
            ) => {
                self.condamner();
                ReponseHttp::refus(503, "facade_condamnee")
            }
            ErreurHybride::Portail(ErreurPortail::FilePleine | ErreurPortail::DebitDepasse) => {
                ReponseHttp::refus(429, "contre_pression")
            }
            ErreurHybride::Portail(ErreurPortail::CleInvalide) => ReponseHttp::refus(403, "auteur"),
            ErreurHybride::Decodage(RefusDecodage::Signature | RefusDecodage::Signataires) => {
                ReponseHttp::refus(403, "signature")
            }
            _ => ReponseHttp::refus(400, "traite"),
        }
    }

    /// Lit les publications de la voie et les range dans la DB secondaire.
    fn relever_publications(&mut self) {
        for _ in 0..MAX_LECTURES {
            match self.guichet.lire_publication() {
                Ok(Some(d)) => {
                    if let Ok(m) = Sortant::depuis(&d) {
                        self.ranger(m);
                    }
                }
                Ok(None) => return,
                Err(ErreurHybride::Portail(_)) => {
                    self.condamner();
                    return;
                }
                // Publication illisible : écartée (la voie est signée par la zone COG ; signalé par les compteurs).
                Err(_) => {}
            }
        }
    }

    fn ranger(&mut self, m: Sortant) {
        let demande = m.demande().to_vec();
        if self.db.lire(T_ATTENTE, &demande).is_none() {
            return; // Réponse orpheline (session précédente) : écartée.
        }
        let ecrit = match m {
            Sortant::CreneauDisponible(CreneauDisponible { rang, creneau, debut, fin, .. }) => {
                let c = CreneauJson { creneau, debut, fin };
                self.db.ecrire(T_PARTIEL, &cle_partiel(&demande, rang), serde_json::to_vec(&c).unwrap_or_default())
            }
            Sortant::FinCreneaux(f) => {
                let partiels: Vec<(Vec<u8>, CreneauJson)> = self
                    .db
                    .table(T_PARTIEL)
                    .filter(|(k, _)| k.len() == demande.len() + 8 && k.starts_with(&demande))
                    .filter_map(|(k, v)| Some((k.to_vec(), serde_json::from_slice(v).ok()?)))
                    .collect();
                for (k, _) in &partiels {
                    self.db.retirer(T_PARTIEL, k);
                }
                if partiels.len() as u64 != f.total {
                    return; // Liste incomplète : aucune réponse (le client redemande).
                }
                let creneaux: Vec<CreneauJson> = partiels.into_iter().map(|(_, c)| c).collect();
                for c in &creneaux {
                    let _ = self.db.ecrire(T_CRENEAUX, c.creneau.as_bytes(), serde_json::to_vec(c).unwrap_or_default());
                }
                self.ecrire_reponse(&demande, &ReponseJson::Creneaux { creneaux })
            }
            Sortant::ReservationConfirmee(c) => {
                self.db.retirer(T_CRENEAUX, c.creneau.as_bytes());
                let reponse = ReponseJson::Confirmee {
                    rendez_vous: c.rendez_vous,
                    creneau: CreneauJson { creneau: c.creneau, debut: c.debut, fin: c.fin },
                };
                self.ecrire_reponse(&demande, &reponse)
            }
            Sortant::ReservationRefusee(r) => {
                self.ecrire_reponse(&demande, &ReponseJson::Refusee { motif: motif_texte(r.motif).to_string() })
            }
        };
        let _ = ecrit; // DB pleine : la réponse est perdue, le client redemande (rien d'autoritaire ici).
    }

    fn ecrire_reponse(&mut self, demande: &[u8], reponse: &ReponseJson) -> Result<(), ErreurHybride> {
        self.db.ecrire(T_REPONSES, demande, serde_json::to_vec(reponse).unwrap_or_default())
    }

    fn reponse(&mut self, r: &RequeteHttp, demande_b64: &str, maintenant: u64) -> ReponseHttp {
        let auteur = match self.authentifier(r, maintenant) {
            Ok(a) => a,
            Err(refus) => return refus,
        };
        let Some(demande) = b64_taille::<32>(demande_b64) else {
            return ReponseHttp::refus(400, "demande");
        };
        // Seul l'auteur de la demande lit sa réponse (sinon : inconnue, sans fuite).
        if self.db.lire(T_ATTENTE, &demande) != Some(&auteur[..]) {
            return ReponseHttp::refus(404, "inconnu");
        }
        self.relever_publications();
        if self.condamnee {
            return ReponseHttp::refus(503, "facade_condamnee");
        }
        match self.db.retirer(T_REPONSES, &demande) {
            Some(corps) => {
                self.db.retirer(T_ATTENTE, &demande);
                ReponseHttp { statut: 200, corps }
            }
            None => ReponseHttp::json(202, &EnAttenteJson { etat: "en_attente".into() }),
        }
    }

    fn cache_creneaux(&mut self, r: &RequeteHttp, maintenant: u64) -> ReponseHttp {
        if let Err(refus) = self.authentifier(r, maintenant) {
            return refus;
        }
        self.relever_publications();
        let creneaux: Vec<CreneauJson> =
            self.db.table(T_CRENEAUX).filter_map(|(_, v)| serde_json::from_slice(v).ok()).collect();
        ReponseHttp::json(200, &ReponseJson::Creneaux { creneaux })
    }
}
