//! Moteur JayRDV du service hybride H2 (lot 5) : zone COG.
//!
//! @id     jayrdv_hybride
//! @do     traiter_messages_traite_jayrdv
//! @role   service
//! @layer  domain
//! @human  Moteur JayRDV en zone COG : relève les messages du traité via le Portail, décide avec KindMother, répond par la voie de publication (INV-H1, EX-41).
//!
//! Le moteur ne parle qu'à deux interlocuteurs :
//!
//! - **KindMother**, par le store JayRDV (intentions typées) ;
//! - le **Portail**, par le [`GuichetMoteur`] de sa voie : il **relève** les
//!   déclarations admises (structures typées, aucune trame, aucun octet reçu)
//!   et **répond** à chacune avec son jeton (étiquette `session`, réponse à
//!   l'action de l'auteur).
//!
//! Il ne parse aucun protocole externe (INV-H1) : ni HTTP, ni JSON reçu.
//! Toute décision (créneau libre, réservation) est prise ici, en zone COG ;
//! la façade n'en prend aucune (INV-H2).

#![allow(clippy::doc_markdown)] // En-têtes MSCM.

use chrono::{DateTime, Utc};
use jayrdv_traite::{
    CreneauDisponible, DemandeCreneaux, Entrant, FinCreneaux, Motif, Reservation, ReservationConfirmee,
    ReservationRefusee, MAX_CRENEAUX_REPONSE,
};
use miyuhybride::{ErreurHybride, GuichetMoteur, MessageSortant};

use crate::data::{JayRdvStore, Slot, SlotStatus};
use crate::domain::appointment_create;

/// Nombre maximal de déclarations relevées par lot (borne du sas).
pub const MAX_LOT: usize = 64;

/// Bilan d'un lot relevé.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RapportLot {
    /// Messages du traité reçus et traités (dans l'ordre).
    pub recus: Vec<Entrant>,
    /// Déclarations refusées par le Portail (sas, validation, rejeu…).
    pub refusees_portail: usize,
    /// Déclarations admises mais hors traité JayRDV.
    pub hors_traite: usize,
    /// Messages publiés en réponse.
    pub publies: usize,
}

/// Moteur JayRDV (zone COG).
#[derive(Debug)]
pub struct MoteurJayRdv<'a> {
    store: &'a JayRdvStore,
    guichet: GuichetMoteur,
}

fn date(s: &str) -> Option<DateTime<Utc>> {
    s.parse().ok()
}

fn sortant<M: miyuhybride::MessageTraite>(m: &M) -> Result<MessageSortant, ErreurHybride> {
    Ok(MessageSortant::de(m)?)
}

impl<'a> MoteurJayRdv<'a> {
    /// Moteur sur le store KindMother et le guichet de sa voie.
    pub fn new(store: &'a JayRdvStore, guichet: GuichetMoteur) -> Self {
        Self { store, guichet }
    }

    /// Relève et traite au plus `max` déclarations ; une réponse par demande admise.
    ///
    /// # Errors
    /// Voie non relevable (désactivée, coupée, sans session) ou publication refusée.
    pub fn traiter_lot(&self, max: usize) -> Result<RapportLot, ErreurHybride> {
        let mut rapport = RapportLot::default();
        for resultat in self.guichet.relever(max.min(MAX_LOT))? {
            let Ok(demande) = resultat else {
                rapport.refusees_portail += 1;
                continue;
            };
            let Ok(entrant) = Entrant::depuis(&demande.declaration) else {
                rapport.hors_traite += 1;
                continue;
            };
            let empreinte = demande.jeton.empreinte().to_vec();
            let reponse = match &entrant {
                Entrant::DemandeCreneaux(d) => self.creneaux(d, &empreinte)?,
                Entrant::Reservation(r) => vec![self.reserver(r, &empreinte)?],
            };
            rapport.publies += self.guichet.repondre(demande.jeton, &reponse)?.len();
            rapport.recus.push(entrant);
        }
        Ok(rapport)
    }

    fn creneaux(&self, d: &DemandeCreneaux, demande: &[u8]) -> Result<Vec<MessageSortant>, ErreurHybride> {
        let libres: Vec<Slot> = match (date(&d.debut), date(&d.fin)) {
            (Some(debut), Some(fin)) if debut < fin => self
                .store
                .slot_list(d.ressource.as_deref(), Some(SlotStatus::Available))
                .unwrap_or_default()
                .into_iter()
                .filter(
                    |s| matches!((date(&s.start_at), date(&s.end_at)), (Some(a), Some(b)) if a >= debut && b <= fin),
                )
                .take(MAX_CRENEAUX_REPONSE)
                .collect(),
            _ => Vec::new(),
        };
        let total = libres.len() as u64;
        let mut messages = Vec::with_capacity(libres.len() + 1);
        for (i, s) in libres.into_iter().enumerate() {
            messages.push(sortant(&CreneauDisponible {
                demande: demande.to_vec(),
                rang: i as u64 + 1,
                total,
                creneau: s.id,
                debut: s.start_at,
                fin: s.end_at,
            })?);
        }
        messages.push(sortant(&FinCreneaux { demande: demande.to_vec(), total })?);
        Ok(messages)
    }

    fn reserver(&self, r: &Reservation, demande: &[u8]) -> Result<MessageSortant, ErreurHybride> {
        let refus = |motif: Motif| sortant(&ReservationRefusee { demande: demande.to_vec(), motif: motif as u64 });
        if r.nom.trim().is_empty() {
            return refus(Motif::Invalide);
        }
        let mut creneau = match self.store.slot_by_id(&r.creneau) {
            Ok(Some(s)) => s,
            Ok(None) => return refus(Motif::Inconnu),
            Err(_) => return refus(Motif::Interne),
        };
        if creneau.status != SlotStatus::Available {
            return refus(Motif::Indisponible);
        }
        // Créneau pris d'abord (pas de double réservation), rendu en cas d'échec.
        creneau.status = SlotStatus::Booked;
        creneau.updated_at = Utc::now().to_rfc3339();
        if self.store.slot_update(creneau.clone()).is_err() {
            return refus(Motif::Interne);
        }
        let rdv = appointment_create(
            self.store,
            format!("RDV {}", r.nom),
            creneau.start_at.clone(),
            creneau.end_at.clone(),
            Some(creneau.resource_id.clone()),
            Some(creneau.id.clone()),
            creneau.service_id.clone(),
            r.contact.clone(),
            Some(r.nom.clone()),
            None,
            r.note.clone(),
            None,
        );
        if let Ok(a) = rdv {
            sortant(&ReservationConfirmee {
                demande: demande.to_vec(),
                rendez_vous: a.id,
                creneau: creneau.id,
                debut: a.start_at,
                fin: a.end_at,
            })
        } else {
            creneau.status = SlotStatus::Available;
            let _ = self.store.slot_update(creneau);
            refus(Motif::Interne)
        }
    }
}
