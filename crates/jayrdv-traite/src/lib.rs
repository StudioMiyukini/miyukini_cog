//! @id     jayrdv_traite_lib
//! @do     declarer_traite_jayrdv
//! @role   contrat
//! @layer  service
//! @human  jayrdv-traite — traité versionné JayRDV (H2) : demandes de créneaux et réservations entrantes, créneaux et confirmations sortants ; contrat de référence du manifeste.

//! # Traité JayRDV (version 1)
//!
//! Premier service hybride migré (PLAN-IMPLEMENTATION §7) : classe **H2**
//! (des données entrent depuis l'extérieur). Le moteur (zone COG, crate
//! `jayrdv`) et la façade (Franche, crate `jayrdv-facade`) compilent tous deux
//! ce crate : leurs codecs sont générés du même schéma (INV-H4).
//!
//! | Sens | Code | Message | Rôle |
//! |---|---|---|---|
//! | entrant | 1 | [`DemandeCreneaux`] | créneaux libres dans une fenêtre |
//! | entrant | 2 | [`Reservation`] | réserver un créneau |
//! | sortant | 101 | [`CreneauDisponible`] | un créneau libre (réponse à 1) |
//! | sortant | 102 | [`FinCreneaux`] | fin de liste (réponse à 1) |
//! | sortant | 103 | [`ReservationConfirmee`] | rendez-vous créé (réponse à 2) |
//! | sortant | 104 | [`ReservationRefusee`] | refus motivé par un code (réponse à 2) |
//!
//! Toute réponse porte `demande` = SHA-256 de la trame entrante à laquelle
//! elle répond (corrélation par la façade, sans autre canal retour).

use miyuhybride::{
    message_traite, ClasseExposition, ContratFacade, ContratService, ContratVoie, Declaration, ErreurTraite,
    MessageTraite, OrigineExterne, PointEntree, Ressources, Traite, Transport,
};

/// Identifiant du service (voie du Portail, audience des visas).
pub const SERVICE: &str = "jayrdv";
/// Version du traité.
pub const VERSION: u64 = 1;
/// Nombre maximal de créneaux dans une réponse (plus la fin de liste).
pub const MAX_CRENEAUX_REPONSE: usize = 30;
/// Taille d'une date ISO 8601 (bornée).
pub const MAX_DATE: u32 = 40;
/// Taille d'un identifiant (créneau, rendez-vous).
pub const MAX_ID: u32 = 64;

message_traite! {
    /// Demande des créneaux libres entre `debut` et `fin` (ISO 8601).
    pub struct DemandeCreneaux (Entrante, 1) {
        /// Début de la fenêtre.
        1 => pub debut: String, max MAX_DATE;
        /// Fin de la fenêtre.
        2 => pub fin: String, max MAX_DATE;
        /// Ressource (facultatif).
        3 => pub ressource: Option<String>, max MAX_ID;
    }
}

message_traite! {
    /// Réservation d'un créneau.
    pub struct Reservation (Entrante, 2) {
        /// Créneau visé.
        1 => pub creneau: String, max MAX_ID;
        /// Nom affiché du client.
        2 => pub nom: String, max 128;
        /// Contact (courriel), facultatif.
        3 => pub contact: Option<String>, max 254;
        /// Note libre, facultative.
        4 => pub note: Option<String>, max 512;
    }
}

message_traite! {
    /// Un créneau libre (réponse à [`DemandeCreneaux`]).
    pub struct CreneauDisponible (Sortante, 101) {
        /// Empreinte de la demande.
        1 => pub demande: Vec<u8>, max 32;
        /// Rang (à partir de 1).
        2 => pub rang: u64, max 0;
        /// Nombre total de créneaux de la réponse.
        3 => pub total: u64, max 0;
        /// Identifiant du créneau.
        4 => pub creneau: String, max MAX_ID;
        /// Début.
        5 => pub debut: String, max MAX_DATE;
        /// Fin.
        6 => pub fin: String, max MAX_DATE;
    }
}

message_traite! {
    /// Fin de liste des créneaux.
    pub struct FinCreneaux (Sortante, 102) {
        /// Empreinte de la demande.
        1 => pub demande: Vec<u8>, max 32;
        /// Nombre de créneaux envoyés.
        2 => pub total: u64, max 0;
    }
}

message_traite! {
    /// Rendez-vous créé.
    pub struct ReservationConfirmee (Sortante, 103) {
        /// Empreinte de la demande.
        1 => pub demande: Vec<u8>, max 32;
        /// Identifiant du rendez-vous.
        2 => pub rendez_vous: String, max MAX_ID;
        /// Créneau réservé.
        3 => pub creneau: String, max MAX_ID;
        /// Début.
        4 => pub debut: String, max MAX_DATE;
        /// Fin.
        5 => pub fin: String, max MAX_DATE;
    }
}

message_traite! {
    /// Réservation refusée (motif = [`Motif`]).
    pub struct ReservationRefusee (Sortante, 104) {
        /// Empreinte de la demande.
        1 => pub demande: Vec<u8>, max 32;
        /// Code de motif.
        2 => pub motif: u64, max 0;
    }
}

/// Motif d'un refus (aucun texte libre ne sort de la zone COG).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum Motif {
    /// Créneau déjà pris, verrouillé ou bloqué.
    Indisponible = 1,
    /// Créneau inconnu.
    Inconnu = 2,
    /// Demande invalide (dates, champs).
    Invalide = 3,
    /// Erreur interne du moteur.
    Interne = 4,
}

impl Motif {
    /// Motif depuis son code.
    pub const fn depuis_code(code: u64) -> Option<Self> {
        match code {
            1 => Some(Self::Indisponible),
            2 => Some(Self::Inconnu),
            3 => Some(Self::Invalide),
            4 => Some(Self::Interne),
            _ => None,
        }
    }
}

/// Traité versionné (inscrit dans le manifeste signé).
pub fn traite() -> Traite {
    miyuhybride::traite(
        VERSION,
        vec![
            DemandeCreneaux::type_declaration(),
            Reservation::type_declaration(),
            CreneauDisponible::type_declaration(),
            FinCreneaux::type_declaration(),
            ReservationConfirmee::type_declaration(),
            ReservationRefusee::type_declaration(),
        ],
    )
}

/// Message entrant décodé (côté moteur).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entrant {
    /// Demande de créneaux.
    DemandeCreneaux(DemandeCreneaux),
    /// Réservation.
    Reservation(Reservation),
}

impl Entrant {
    /// Décodage strict d'une déclaration entrante.
    pub fn depuis(d: &Declaration) -> Result<Self, ErreurTraite> {
        match d.type_code {
            DemandeCreneaux::CODE => DemandeCreneaux::depuis_declaration(d).map(Self::DemandeCreneaux),
            Reservation::CODE => Reservation::depuis_declaration(d).map(Self::Reservation),
            _ => Err(ErreurTraite::Type),
        }
    }
}

/// Message sortant décodé (côté façade).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sortant {
    /// Créneau libre.
    CreneauDisponible(CreneauDisponible),
    /// Fin de liste.
    FinCreneaux(FinCreneaux),
    /// Réservation confirmée.
    ReservationConfirmee(ReservationConfirmee),
    /// Réservation refusée.
    ReservationRefusee(ReservationRefusee),
}

impl Sortant {
    /// Décodage strict d'une déclaration sortante.
    pub fn depuis(d: &Declaration) -> Result<Self, ErreurTraite> {
        match d.type_code {
            CreneauDisponible::CODE => CreneauDisponible::depuis_declaration(d).map(Self::CreneauDisponible),
            FinCreneaux::CODE => FinCreneaux::depuis_declaration(d).map(Self::FinCreneaux),
            ReservationConfirmee::CODE => ReservationConfirmee::depuis_declaration(d).map(Self::ReservationConfirmee),
            ReservationRefusee::CODE => ReservationRefusee::depuis_declaration(d).map(Self::ReservationRefusee),
            _ => Err(ErreurTraite::Type),
        }
    }

    /// Empreinte de la demande à laquelle répond le message.
    pub fn demande(&self) -> &[u8] {
        match self {
            Self::CreneauDisponible(m) => &m.demande,
            Self::FinCreneaux(m) => &m.demande,
            Self::ReservationConfirmee(m) => &m.demande,
            Self::ReservationRefusee(m) => &m.demande,
        }
    }
}

/// Contrat de référence de JayRDV (H2) pour le manifeste signé ; `image` =
/// SHA-256 de l'image signée de la façade.
pub fn contrat_reference(image: [u8; 32]) -> ContratService {
    ContratService {
        classe: ClasseExposition::H2,
        ressources: Ressources { cpu_pourcent: 50, memoire_mio: 256, disque_mio: 64, taches_max: 64, sessions_max: 16 },
        voie: Some(ContratVoie { taille_max_trame: 8192, debit_max_par_minute: 600, quota_file: 256 }),
        traite: Some(traite()),
        facade: Some(ContratFacade { image, db_secondaire_max_mio: 8, duree_max_session_secs: 3600 }),
        entrees: vec![PointEntree {
            origine: OrigineExterne::Federation,
            transport: Transport::Tcp,
            port: 443,
            debit_max_kio_s: 1024,
            taille_max_paquet: 1500,
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuhybride::{IdService, Sens};

    #[test]
    fn traite_valide_et_classe_h2() {
        let t = traite();
        t.valider().unwrap();
        assert_eq!(t.types.iter().filter(|t| t.sens == Sens::Entrante).count(), 2);
        contrat_reference([7; 32]).valider(&IdService::new(SERVICE).unwrap()).unwrap();
    }

    #[test]
    fn aller_retour_des_messages() {
        let r = Reservation { creneau: "c1".into(), nom: "Ada".into(), contact: None, note: Some("n".into()) };
        let d = miyuhybride::declaration_entrante(&IdService::new(SERVICE).unwrap(), VERSION, &r, 1, 0).unwrap();
        assert_eq!(Entrant::depuis(&d).unwrap(), Entrant::Reservation(r));
    }
}
