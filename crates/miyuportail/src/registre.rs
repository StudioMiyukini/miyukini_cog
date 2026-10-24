//! @id     miyuportail_registre
//! @do     inscrire_passages_portail
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — crochet d'inscription au registre Merkle (empreintes des passages, coupures) ; registre réel au lot 4.

use std::sync::Mutex;

use crate::traite::IdService;

/// Empreinte SHA-256.
pub type Empreinte = [u8; 32];

/// Motif d'une coupure de voie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotifCoupure {
    /// Anomalie détectée par WorrySentinel (signal des Vigies).
    Anomalie,
    /// Dépassement du contrat de la voie.
    HorsContrat,
    /// Décision humaine (TAMR).
    Decision,
}

/// Événement inscrit au registre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evenement {
    /// Déclaration admise en zone COG (empreinte de la trame reçue).
    Entree {
        /// Service.
        service: IdService,
        /// SHA-256 de la trame reçue.
        empreinte: Empreinte,
    },
    /// Publication émise vers la Franche (empreinte de la trame émise).
    Publication {
        /// Service.
        service: IdService,
        /// SHA-256 de la trame émise.
        empreinte: Empreinte,
    },
    /// Voie coupée (incident).
    Coupure {
        /// Service.
        service: IdService,
        /// Motif.
        motif: MotifCoupure,
    },
}

/// Échec d'inscription : le passage est refusé.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErreurInscription;

/// Inscription au registre (Merkle, lot 4).
pub trait Inscription: Send + Sync {
    /// Inscrit un événement ; un échec annule le passage (fail-closed).
    fn inscrire(&self, evenement: &Evenement) -> Result<(), ErreurInscription>;
}

/// Registre en mémoire (tests, intégration avant le lot 4).
#[derive(Debug, Default)]
pub struct RegistreMemoire {
    evenements: Mutex<Vec<Evenement>>,
}

impl RegistreMemoire {
    /// Registre vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Copie des événements inscrits.
    pub fn evenements(&self) -> Vec<Evenement> {
        self.evenements.lock().map(|e| e.clone()).unwrap_or_default()
    }
}

impl Inscription for RegistreMemoire {
    fn inscrire(&self, evenement: &Evenement) -> Result<(), ErreurInscription> {
        self.evenements.lock().map_err(|_| ErreurInscription)?.push(evenement.clone());
        Ok(())
    }
}
