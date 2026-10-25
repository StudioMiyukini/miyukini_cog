//! @id     miyuhybride_lib
//! @do     declarer_modules_services_hybrides
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — services hybrides (lot 5) : manifeste signé H0–H3, patron moteur / façade / traité, façades jetables, DB secondaire.

//! # MiyuHybride — services hybrides (lot 5)
//!
//! Spécification : `docs/architecture/SERVICES-HYBRIDES-ET-PORTAIL.md` §1, §2,
//! §4 ; `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §9, §10.
//!
//! - **EX-16 / EX-40** manifeste de service signé : classe d'exposition,
//!   contrat de ressources et de flux, traité, image de façade
//!   ([`manifeste`]) ; dérivation des entrées Douane ([`douane`], feature
//!   `douane`, génération seulement) ;
//! - **EX-41** le moteur ne reçoit que des structures typées par le
//!   [`GuichetMoteur`] : aucun protocole externe ne l'atteint ;
//! - **EX-42** traité versionné, codecs générés des deux côtés depuis une
//!   seule déclaration ([`message_traite!`]) ;
//! - **EX-43** façades jetables : clé de session éphémère émise par la zone
//!   COG, DB secondaire vierge, destruction et reconstruction
//!   ([`ServiceHybride`], [`Superviseur`]) ;
//! - **quotas** du contrat (disque, sessions, tâches) appliqués par
//!   compteurs atomiques, refus au-delà ([`quotas`]) ;
//! - **EX-48** exposition désactivée par défaut (manifeste qui la demande :
//!   refusé ; voie installée désactivée).
//!
//! ## Qui tourne où
//!
//! | Zone | Composant | Accès |
//! |---|---|---|
//! | zone COG | moteur du service | KindMother + [`GuichetMoteur`] (relever / répondre) |
//! | zone COG (Cores) | [`ServiceHybride`], [`Superviseur`] | Portail (déclarer, activer, sessions) |
//! | Franche | façade | [`GuichetFranche`] (déposer / lire) + [`DbSecondaire`] |

pub mod db_secondaire;
#[cfg(feature = "douane")]
pub mod douane;
pub mod erreurs;
pub mod guichets;
pub mod manifeste;
pub mod message;
pub mod quotas;
#[cfg(feature = "revocations")]
pub mod revocations;
pub mod service;

pub use db_secondaire::DbSecondaire;
pub use erreurs::{ErreurHybride, ErreurManifeste, ErreurTraite};
pub use guichets::{
    declaration_entrante, signer_auteur, DemandeAdmise, EmpreinteDemande, GuichetFranche, GuichetMoteur, JetonReponse,
    MessageSortant, SignatureAuteur, MAX_MESSAGES_REPONSE,
};
pub use manifeste::{
    sceller_manifeste, verifier_manifeste, ContratFacade, ContratService, ContratVoie, ManifesteService,
    ManifesteVerifie, OrigineExterne, PointEntree, Ressources, Transport, GENRE_CONTRAT,
};
pub use message::{traite, ChampScalaire, ChampType, MessageTraite};
pub use quotas::{
    JetonSession, JetonTache, Quota, QuotasService, ReservationDisque, Ressource, SessionsClients,
    DELAI_INACTIVITE_DEFAUT_SECS,
};
#[cfg(feature = "revocations")]
pub use revocations::{
    controler_fraicheur_revocations, RefusRemise, RevocationsFacade, FRAICHEUR_REVOCATIONS_FACADE_SECS,
};
pub use service::{FacadeJetable, LancementFacade, ServiceHybride, Superviseur};

// Réexportations nécessaires à la macro et aux services.
pub use miyuportail::{
    Champ, ClasseExposition, Declaration, ErreurPortail, EtatVoie, IdService, RefusDecodage, Sens, Traite,
    TypeDeclaration, Valeur,
};
