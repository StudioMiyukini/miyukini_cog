//! @id     miyuportail_cdr
//! @do     desarmer_documents_traversant_voie
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — crochet de désarmement et reconstruction (CDR) : tout document traversant une voie y passe (EX-45).

//! Le CDR proprement dit est fourni par une autre crate ; le Portail ne le
//! connaît que par le trait [`Desarmement`]. L'implémentation par défaut
//! [`RefusParDefaut`] **refuse tout** (fail-closed) tant qu'aucun CDR n'est
//! branché : aucun document ne traverse sans désarmement.

use crate::traite::{IdService, NatureDocument, Sens};

/// Document à désarmer (octets non fiables, jamais transmis tels quels).
#[derive(Debug, Clone, Copy)]
pub struct ObjetADesarmer<'a> {
    /// Service de la voie.
    pub service: &'a IdService,
    /// Sens de la traversée.
    pub sens: Sens,
    /// Type de déclaration.
    pub type_code: u64,
    /// Tag du champ.
    pub tag: u8,
    /// Nature déclarée par le traité.
    pub nature: NatureDocument,
    /// Taille maximale admise après reconstruction.
    pub taille_max: usize,
    /// Octets reçus.
    pub octets: &'a [u8],
}

/// Document reconstruit dans un format interne sûr.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjetDesarme {
    /// Octets reconstruits (aucun octet brut d'origine n'est garanti conservé).
    pub octets: Vec<u8>,
}

/// Refus du CDR (motif opaque : aucun détail ne remonte vers la façade).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefusDesarmement;

/// Désarmement et reconstruction (CDR).
pub trait Desarmement: Send + Sync {
    /// Désarme un document ; tout échec est un refus du message entier.
    fn desarmer(&self, objet: &ObjetADesarmer<'_>) -> Result<ObjetDesarme, RefusDesarmement>;
}

/// CDR par défaut : refuse tout document (fail-closed, en attendant la crate CDR).
#[derive(Debug, Clone, Copy, Default)]
pub struct RefusParDefaut;

impl Desarmement for RefusParDefaut {
    fn desarmer(&self, _objet: &ObjetADesarmer<'_>) -> Result<ObjetDesarme, RefusDesarmement> {
        Err(RefusDesarmement)
    }
}
