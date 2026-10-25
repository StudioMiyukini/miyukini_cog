//! @id     central_canal_officiel_alertes
//! @do     remonter_refus_canal_worrysentinel
//! @role   securite
//! @layer  app
//! @human  central — module « alertes » du canal officiel (refus du canal → signaux WorrySentinel, journal).

//! Refus du canal officiel → signaux pour WorrySentinel (A-006 : « refus
//! signalé, événement à remonter à WorrySentinel »).
//!
//! WorrySentinel n'est pas redessiné : chaque refus devient un
//! [`SignalCanal`] portant un [`ThreatLevel`] ; un [`PuitsSignaux`] le reçoit.
//! [`PuitsJournal`] (défaut) journalise ; [`PuitsMemoire`] conserve les
//! signaux et en tire un `DefaultThreatDetector` (vue déclarative attendue par
//! WorrySentinel). Les événements ordinaires (synchronisation, quarantaine,
//! attente, installation) ne sont que journalisés.

use std::sync::{Arc, Mutex};

use miyucanal::{ErreurCanal, Evenement, Motif};
use worrysentinel::{DefaultThreatDetector, ThreatLevel};

/// Signal d'un refus du canal officiel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalCanal {
    /// Gravité pour WorrySentinel.
    pub niveau: ThreatLevel,
    /// Résumé lisible (journal, audit).
    pub motif: String,
}

/// Destinataire des signaux du canal (WorrySentinel, journal…).
pub trait PuitsSignaux: Send {
    /// Reçoit un signal.
    fn signaler(&self, signal: &SignalCanal);
}

/// Puits par défaut : journal (`warn` au-delà de `Low`).
#[derive(Debug, Default, Clone, Copy)]
pub struct PuitsJournal;

impl PuitsSignaux for PuitsJournal {
    fn signaler(&self, s: &SignalCanal) {
        if s.niveau >= ThreatLevel::Medium {
            tracing::warn!(niveau = ?s.niveau, "canal officiel : {}", s.motif);
        } else {
            tracing::info!(niveau = ?s.niveau, "canal officiel : {}", s.motif);
        }
    }
}

/// Puits en mémoire (partageable) : journalise et conserve les signaux.
#[derive(Debug, Default, Clone)]
pub struct PuitsMemoire {
    signaux: Arc<Mutex<Vec<SignalCanal>>>,
}

impl PuitsMemoire {
    /// Signaux reçus.
    pub fn signaux(&self) -> Vec<SignalCanal> {
        self.signaux.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    /// Vue WorrySentinel : détecteur dont le niveau est le maximum des signaux.
    pub fn detecteur(&self) -> DefaultThreatDetector {
        DefaultThreatDetector::with_signals(self.signaux().into_iter().map(|s| s.niveau))
    }
}

impl PuitsSignaux for PuitsMemoire {
    fn signaler(&self, s: &SignalCanal) {
        PuitsJournal.signaler(s);
        self.signaux.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(s.clone());
    }
}

fn niveau_motif(m: &Motif) -> ThreatLevel {
    match m {
        // Signature, rétrogradation, inclusion ou artefact altéré : tentative probable.
        Motif::Signature(_) | Motif::Retrogradation { .. } | Motif::Inclusion(_) | Motif::ArtefactAltere => {
            ThreatLevel::High
        }
        Motif::Invalide(_) | Motif::Revoquee | Motif::Installateur(_) => ThreatLevel::Medium,
        Motif::ArtefactIndisponible(_) => ThreatLevel::Low,
    }
}

/// Signal d'un événement de relevé ; `None` pour un événement ordinaire.
pub fn signal_evenement(e: &Evenement) -> Option<SignalCanal> {
    let (niveau, motif) = match e {
        Evenement::AnnonceRefusee { index, motif } => {
            (niveau_motif(motif), format!("annonce {index} refusée : {motif:?}"))
        }
        Evenement::InstallationRefusee { composant, version, motif } => {
            (niveau_motif(motif), format!("installation de {composant} v{version} refusée : {motif:?}"))
        }
        Evenement::QuarantaineAlteree { refuses, illisible } => (
            ThreatLevel::High,
            format!("état de quarantaine refusé ({refuses} enregistrement(s) altéré(s), illisible : {illisible}) : quarantaine relancée"),
        ),
        Evenement::Revocation { cible } => (ThreatLevel::Low, format!("révocation enregistrée : {cible:?}")),
        Evenement::Annulee { composant, version } => {
            (ThreatLevel::Low, format!("annonce {composant} v{version} annulée"))
        }
        Evenement::Synchronise { .. }
        | Evenement::EntreeIgnoree { .. }
        | Evenement::EnQuarantaine { .. }
        | Evenement::EnAttente { .. }
        | Evenement::Installee { .. } => return None,
    };
    Some(SignalCanal { niveau, motif })
}

/// Signal d'un relevé refusé en bloc (tête gelée, équivoque, horloge…).
pub fn signal_erreur(e: &ErreurCanal) -> SignalCanal {
    let niveau = match e {
        ErreurCanal::Transport(_) => ThreatLevel::Low,
        ErreurCanal::Registre(_) | ErreurCanal::Sceau(_) | ErreurCanal::Temps(_) | ErreurCanal::Revoquee => {
            ThreatLevel::High
        }
        ErreurCanal::Configuration(_) | ErreurCanal::ReponseInvalide | ErreurCanal::TropDEntrees(_) => {
            ThreatLevel::Medium
        }
    };
    SignalCanal { niveau, motif: format!("relevé refusé : {e}") }
}

/// Journalise un événement et remonte son signal éventuel.
pub fn remonter(puits: &dyn PuitsSignaux, e: &Evenement) {
    if let Some(s) = signal_evenement(e) {
        puits.signaler(&s);
    } else {
        tracing::info!("canal officiel : {e:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyucanal::ErreurTransport;
    use worrysentinel::ThreatDetector as _;

    #[test]
    fn refus_vers_niveaux_worrysentinel() {
        let puits = PuitsMemoire::default();
        for e in [
            Evenement::Installee { composant: "a".into(), version: 2 },
            Evenement::InstallationRefusee { composant: "a".into(), version: 2, motif: Motif::ArtefactAltere },
            Evenement::QuarantaineAlteree { refuses: 1, illisible: false },
            Evenement::AnnonceRefusee { index: 0, motif: Motif::ArtefactIndisponible(ErreurTransport::Absent) },
        ] {
            remonter(&puits, &e);
        }
        let s = puits.signaux();
        assert_eq!(s.iter().map(|s| s.niveau).collect::<Vec<_>>(), [ThreatLevel::High, ThreatLevel::High, ThreatLevel::Low]);
        assert_eq!(puits.detecteur().max_signal(), ThreatLevel::High);
        let _ = puits.detecteur().detect();
        assert_eq!(signal_erreur(&ErreurCanal::Transport(ErreurTransport::Absent)).niveau, ThreatLevel::Low);
        assert_eq!(signal_erreur(&ErreurCanal::ReponseInvalide).niveau, ThreatLevel::Medium);
    }
}
