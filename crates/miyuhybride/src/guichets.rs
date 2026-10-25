//! @id     miyuhybride_guichets
//! @do     separer_acces_franche_et_zone_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — guichets du Portail : la façade (Franche) dépose et lit, le moteur (zone COG) relève et répond ; aucun guichet ne donne l'autre côté.

//! # Guichets
//!
//! Le [`miyuportail::PortailInterne`] expose les deux côtés d'une voie. Les
//! guichets les **séparent par les types** :
//!
//! - [`GuichetFranche`] (façade) : `deposer` une déclaration entrante,
//!   `lire_publication`, état et compteurs — rien d'autre. Il porte la clé
//!   de session **éphémère** de la façade, émise par la zone COG (INV-H6) ;
//! - [`GuichetMoteur`] (zone COG) : `relever` les déclarations admises,
//!   `repondre` à l'une d'elles — rien d'autre ; il ne voit ni trame ni octet
//!   reçu, seulement des structures typées (INV-H1).
//!
//! Aucun guichet ne contient d'adresse ni de poignée de l'autre côté, de
//! KindMother ou du moteur : la façade ne peut atteindre la zone COG que
//! par une trame MSCE déposée en quarantaine.
//!
//! ## Réponse à une action de l'utilisateur
//!
//! Le Portail n'admet en sortie que les publications déclenchées par une
//! **action explicite de l'utilisateur** (§3, EX-47). Le moteur d'un H2 ne
//! publie donc qu'en **réponse** à une déclaration admise, signée par son
//! auteur (le participant) : [`GuichetMoteur::relever`] remet avec chaque
//! déclaration un [`JetonReponse`] à usage unique, sans lequel
//! [`GuichetMoteur::repondre`] n'existe pas. La réponse porte l'étiquette
//! `session` (visible des participants de la session, jamais `publiable`),
//! et le jeton d'une session close est refusé.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use ed25519_dalek::{Signer, SigningKey};
use miyuportail::msce::{conforme_au_traite, decoder_trame, encoder_charge, octets_signes, sceller_cosigne, Contexte};
use miyuportail::{
    Ancres, Champ, CompteursVoie, Declaration, Declencheur, ErreurPortail, EtatVoie, IdService, PortailInterne,
    Publication, Sens, Sensibilite, Traite,
};
use miyusceau::CleSignature;
use sha2::{Digest, Sha256};

use crate::erreurs::{ErreurHybride, ErreurTraite};
use crate::message::MessageTraite;

/// Nombre maximal de messages dans une réponse.
pub const MAX_MESSAGES_REPONSE: usize = 32;

/// Empreinte SHA-256 d'une trame déposée (corrélation demande → réponse).
pub type EmpreinteDemande = [u8; 32];

/// Signature d'auteur : `(clé publique, signature Ed25519 des octets signés)`.
pub type SignatureAuteur = miyuportail::msce::Cosignature;

/// Déclaration entrante non signée d'un message (côté appareil ou façade).
pub fn declaration_entrante<M: MessageTraite>(
    service: &IdService,
    version_traite: u64,
    message: &M,
    sequence: u64,
    emise_le: u64,
) -> Result<Declaration, ErreurTraite> {
    if M::SENS != Sens::Entrante {
        return Err(ErreurTraite::Sens);
    }
    Ok(Declaration {
        service: service.clone(),
        sens: Sens::Entrante,
        version_traite,
        type_code: M::CODE,
        sequence,
        emise_le,
        champs: message.en_champs(),
        signataires: Vec::new(),
    })
}

/// Signature de l'**auteur** sur une déclaration (sur son appareil) : octets
/// signés de la trame MSCE (domaine Sceau v1.1, type `0x08`).
pub fn signer_auteur(cle: &CleSignature, d: &Declaration) -> Result<SignatureAuteur, ErreurHybride> {
    let charge = encoder_charge(d)?;
    let sk = SigningKey::from_bytes(&cle.graine());
    Ok((sk.verifying_key().to_bytes(), sk.sign(&octets_signes(&charge)).to_bytes()))
}

// ---------------------------------------------------------------- Franche

/// Guichet de la façade (Franche) : dépôt en quarantaine et lecture des publications.
pub struct GuichetFranche {
    portail: Arc<PortailInterne>,
    service: IdService,
    traite: Traite,
    cle_session: SigningKey,
    cle_publication: [u8; 32],
    auteurs: Vec<[u8; 32]>,
}

impl std::fmt::Debug for GuichetFranche {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuichetFranche").field("service", &self.service).finish_non_exhaustive()
    }
}

impl GuichetFranche {
    pub(crate) fn new(
        portail: Arc<PortailInterne>,
        service: IdService,
        traite: Traite,
        cle_session: SigningKey,
        auteurs: Vec<[u8; 32]>,
    ) -> Result<Self, ErreurHybride> {
        let cle_publication = portail.cle_publication(&service)?;
        Ok(Self { portail, service, traite, cle_session, cle_publication, auteurs })
    }

    /// Service de la voie.
    pub fn service(&self) -> &IdService {
        &self.service
    }

    /// Version du traité.
    pub fn version_traite(&self) -> u64 {
        self.traite.version
    }

    /// Clé publique de session de la façade (épinglée par la zone COG).
    pub fn cle_session(&self) -> [u8; 32] {
        self.cle_session.verifying_key().to_bytes()
    }

    /// Vrai si `cle` est un auteur épinglé de la session.
    pub fn est_auteur(&self, cle: &[u8; 32]) -> bool {
        self.auteurs.contains(cle)
    }

    /// Dépose une déclaration entrante signée par son auteur : contrôle du
    /// traité (filtre côté Franche, la validation qui compte est en zone
    /// COG), cosignature par la clé de session, trame MSCE neuve. Rend
    /// l'empreinte de la trame déposée (corrélation de la réponse).
    pub fn deposer(&self, d: &Declaration, auteur: &SignatureAuteur) -> Result<EmpreinteDemande, ErreurHybride> {
        if d.sens != Sens::Entrante || d.service != self.service {
            return Err(ErreurTraite::Sens.into());
        }
        if !self.est_auteur(&auteur.0) {
            return Err(ErreurPortail::CleInvalide.into());
        }
        conforme_au_traite(d, &self.traite)?;
        let trame = sceller_cosigne(d, &[&self.cle_session], std::slice::from_ref(auteur))?;
        let empreinte = Sha256::digest(&trame).into();
        self.portail.deposer(&self.service, trame)?;
        Ok(empreinte)
    }

    /// Lit et vérifie la prochaine publication (signée par la clé de la voie,
    /// conforme au traité sortant).
    pub fn lire_publication(&self) -> Result<Option<Declaration>, ErreurHybride> {
        let Some(trame) = self.portail.lire_publication(&self.service)? else {
            return Ok(None);
        };
        let ancres = Ancres { emetteur: self.cle_publication, auteurs: Vec::new() };
        let ctx = Contexte { service: &self.service, sens: Sens::Sortante, traite: &self.traite, ancres: &ancres };
        Ok(Some(decoder_trame(&trame, &ctx)?))
    }

    /// État de la voie (une façade ne sert que si la voie est active).
    pub fn etat(&self) -> Result<EtatVoie, ErreurHybride> {
        Ok(self.portail.etat(&self.service)?)
    }

    /// Compteurs de la voie (seul retour du moteur vers la façade).
    pub fn compteurs(&self) -> Result<CompteursVoie, ErreurHybride> {
        Ok(self.portail.compteurs(&self.service)?)
    }
}

// ---------------------------------------------------------------- zone COG

/// Jeton de réponse : remis avec une déclaration admise, consommé par la réponse.
/// Ni `Clone` ni constructible hors de ce crate.
#[derive(Debug, PartialEq, Eq)]
pub struct JetonReponse {
    epoque: u64,
    empreinte: EmpreinteDemande,
}

impl JetonReponse {
    /// Empreinte de la trame à laquelle on répond (à recopier dans la réponse).
    pub fn empreinte(&self) -> &EmpreinteDemande {
        &self.empreinte
    }
}

/// Déclaration admise par le Portail, remise au moteur.
#[derive(Debug)]
pub struct DemandeAdmise {
    /// Structure typée (aucun octet reçu).
    pub declaration: Declaration,
    /// Jeton de réponse (usage unique).
    pub jeton: JetonReponse,
}

impl DemandeAdmise {
    /// Décodage strict vers un message du traité.
    pub fn message<M: MessageTraite>(&self) -> Result<M, ErreurTraite> {
        M::depuis_declaration(&self.declaration)
    }
}

/// Message sortant typé (une réponse peut en compter plusieurs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageSortant {
    type_code: u64,
    champs: Vec<Champ>,
}

impl MessageSortant {
    /// Message sortant depuis un message du traité.
    pub fn de<M: MessageTraite>(m: &M) -> Result<Self, ErreurTraite> {
        if M::SENS != Sens::Sortante {
            return Err(ErreurTraite::Sens);
        }
        Ok(Self { type_code: M::CODE, champs: m.en_champs() })
    }
}

/// Guichet du moteur (zone COG) : relève à son rythme, répond aux demandes.
#[derive(Clone)]
pub struct GuichetMoteur {
    portail: Arc<PortailInterne>,
    service: IdService,
    epoque: Arc<AtomicU64>,
}

impl std::fmt::Debug for GuichetMoteur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuichetMoteur").field("service", &self.service).finish_non_exhaustive()
    }
}

impl GuichetMoteur {
    pub(crate) fn new(portail: Arc<PortailInterne>, service: IdService, epoque: Arc<AtomicU64>) -> Self {
        Self { portail, service, epoque }
    }

    /// Service.
    pub fn service(&self) -> &IdService {
        &self.service
    }

    /// Relève jusqu'à `max` déclarations (sas, deuxième validation, CDR,
    /// anti-rejeu, registre : tout est fait par le Portail).
    pub fn relever(&self, max: usize) -> Result<Vec<Result<DemandeAdmise, ErreurPortail>>, ErreurHybride> {
        let epoque = self.epoque.load(Ordering::SeqCst);
        let lot = self.portail.relever(&self.service, max)?;
        Ok(lot
            .into_iter()
            .map(|r| {
                r.map(|admise| DemandeAdmise {
                    declaration: admise.declaration,
                    jeton: JetonReponse { epoque, empreinte: admise.empreinte },
                })
            })
            .collect())
    }

    /// Répond à une demande : 1 à [`MAX_MESSAGES_REPONSE`] messages sortants,
    /// étiquette `session`, déclenchés par l'action de l'auteur de la demande.
    /// Rend les numéros de publication.
    pub fn repondre(&self, jeton: JetonReponse, messages: &[MessageSortant]) -> Result<Vec<u64>, ErreurHybride> {
        if messages.is_empty() || messages.len() > MAX_MESSAGES_REPONSE {
            return Err(ErreurHybride::ReponseTropLongue);
        }
        if jeton.epoque != self.epoque.load(Ordering::SeqCst) {
            return Err(ErreurHybride::SessionPerimee);
        }
        let mut numeros = Vec::with_capacity(messages.len());
        for m in messages {
            let publication = Publication {
                type_code: m.type_code,
                champs: m.champs.clone(),
                etiquette: Sensibilite::Session,
                declencheur: Declencheur::Utilisateur,
            };
            numeros.push(self.portail.publier(&self.service, publication)?);
        }
        Ok(numeros)
    }
}
