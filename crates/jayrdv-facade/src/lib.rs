//! @id     jayrdv_facade_lib
//! @do     exposer_facade_jayrdv
//! @role   service
//! @layer  service
//! @human  jayrdv-facade — façade Franche du premier service hybride H2 : API de réservation pour appareil fédéré, sans route vers la zone COG.

//! # Façade JayRDV (lot 5, premier service H2)
//!
//! S'exécute en **Franche** (sandbox `miyudouane`, DB secondaire seule
//! écriture permise). Termine l'API externe ([`api`]), authentifie l'appareil
//! fédéré (visa du COG hôte + preuve de possession, `miyauth::http`, audience
//! `jayrdv`), traduit chaque requête en message du traité (`jayrdv-traite`)
//! et le dépose dans la quarantaine de sa voie. Le moteur relève en zone COG
//! et répond par la voie de publication ; la façade range les réponses dans
//! sa DB secondaire et les sert à l'auteur de la demande.
//!
//! **Aucun chemin direct** : ce crate ne dépend ni du moteur `jayrdv`, ni de
//! KindMother, ni du Central (règle vérifiée par `tests/architecture.rs`) ; il
//! ne reçoit de la zone COG qu'un [`miyuhybride::LancementFacade`].

pub mod api;
pub mod client;
pub mod facade;

pub use api::{ReponseHttp, ReponseJson, RequeteHttp};
pub use client::{ClientFedere, DefiRecu};
pub use facade::{ConfigurationFacade, FacadeJayRdv, PORTEE_RESERVER};

/// Empreinte de l'image de façade de cette version (INV-H5) : en production,
/// SHA-256 de l'image signée de la sandbox ; en processus, SHA-256 de
/// l'identité du crate (nom et version).
pub fn empreinte_image() -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(concat!("jayrdv-facade/", env!("CARGO_PKG_VERSION")).as_bytes()).into()
}
