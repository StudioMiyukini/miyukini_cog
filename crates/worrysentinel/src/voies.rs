//! @id     worrysentinel_voies
//! @do     decider_coupure_par_voie
//! @role   governance
//! @layer  core
//! @human  worrysentinel — module « voies » : dégradation et coupure par voie (EX-49), à côté de T0–T4.

//! Dégradation **par voie** du Portail et décisions de coupure (EX-49 :
//! « coupure de voie par WorrySentinel sans impact sur les autres services »),
//! à côté de l'état global T0–T4.
//!
//! Chaîne : Vigies → CaringNanny ([`RapportObservation`]) → [`SentinelleVoies::evaluer`]
//! → [`Decision`]s → [`SentinelleVoies::appliquer_coupures`] → consommateur
//! [`CoupureVoie`] (le Portail, câblé par le coordinateur). WorrySentinel
//! **décide** ; il n'exécute rien lui-même (INV-WS-2) : la coupure effective
//! (façade détruite et reconstruite, session close) est faite par le consommateur.
//!
//! # Règles par voie
//!
//! | observation | décision |
//! |---|---|
//! | signal **critique** attribué à la voie X | couper X (seule X change d'état) |
//! | signal critique sans voie | incident hors voie (à bloquer par la Douane), aucune voie touchée |
//! | signal moyen / élevé sur X | X surveillée |
//! | santé de X `Inconnu` / `Unhealthy` (voie aveugle) | politique : couper X (défaut, fail-closed) ou surveiller |
//! | santé de X `Degraded` | X surveillée |
//!
//! Une voie coupée ou surveillée le reste jusqu'à un **réarmement explicite**
//! ([`SentinelleVoies::rearmer`]), tracé au journal.
//!
//! # Règles globales (T0–T4)
//!
//! Conformes au contrat des états de confiance : progression **par étapes**
//! (un cran au plus par évaluation, aucun saut), **T4 terminal et jamais
//! automatique** (« confirmation de compromission » = décision humaine / TAMR).
//! Cible calculée (seuils : décision de ce lot, voir le rapport) :
//! - T1 (« détection d'anomalie ») : au moins une voie coupée pour incident ;
//! - T2 (« persistance ») : deux voies coupées pour incident, ou un incident hors voie ;
//! - T3 (« aggravation ») : trois voies coupées pour incident, ou incident hors voie
//!   **et** au moins une voie coupée pour incident ;
//! - plafond automatique : [`PolitiqueVoies::plafond_automatique`] (T3 au plus).
//!
//! Les voies aveugles ne comptent pas pour l'état global (défaillance de
//! capteur, pas incident). Aucune désescalade automatique.
//! À T4, plus aucune décision opérationnelle n'est émise.

use std::collections::{BTreeMap, HashSet};
use std::fmt;

use caringnanny::{HealthStatus, RapportObservation};
use miyuvigie::{Severite, Signal, VoieId};

use crate::degradation::{Degradation, DegradationManager, DegradationState};

/// État d'une voie vu par WorrySentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatVoie {
    /// Voie ouverte.
    Ouverte,
    /// Voie surveillée (journal renforcé), trafic maintenu.
    Surveillee,
    /// Voie coupée : décision émise, en attente de réarmement explicite.
    Coupee,
}

/// Réponse à une voie aveugle (aucun capteur sain ne la couvre).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReponseVoieAveugle {
    /// Couper la voie (défaut, fail-closed).
    Couper,
    /// Surveiller seulement.
    Surveiller,
}

/// Politique de décision par voie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolitiqueVoies {
    /// Réponse à une voie aveugle.
    pub voie_aveugle: ReponseVoieAveugle,
    /// État global maximal atteignable automatiquement (borné à T3).
    pub plafond_automatique: DegradationState,
}

impl Default for PolitiqueVoies {
    fn default() -> Self {
        Self {
            voie_aveugle: ReponseVoieAveugle::Couper,
            plafond_automatique: DegradationState::Restricted,
        }
    }
}

/// Motif d'une décision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motif {
    /// Signal critique (EX-12 : sortie de zone COG, mouvement latéral…).
    IncidentCritique,
    /// Signal moyen ou élevé.
    Anomalie,
    /// Aucun capteur sain ne couvre la voie.
    VoieAveugle,
    /// Capteur dégradé sur la voie.
    CapteurDegrade,
    /// Escalade de l'état global (règles T0–T4).
    Escalade,
    /// Réarmement explicite.
    Rearmement,
}

/// Action décidée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Couper la voie (EX-49).
    CouperVoie(VoieId),
    /// Mettre la voie sous surveillance renforcée.
    SurveillerVoie(VoieId),
    /// Rouvrir la voie (réarmement explicite).
    RearmerVoie(VoieId),
    /// Incident critique non attribuable à une voie : à bloquer par la Douane / BorderGuard.
    IncidentHorsVoie,
    /// Changement d'état global.
    ChangerEtatGlobal {
        /// État précédent.
        de: DegradationState,
        /// Nouvel état.
        vers: DegradationState,
    },
}

/// Décision explicable : motif, explication, preuves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    /// Identifiant (séquentiel, unique dans la sentinelle).
    pub id: u64,
    /// Instant (ms) du rapport ayant conduit à la décision.
    pub horodatage_ms: u64,
    /// Action.
    pub action: Action,
    /// Motif.
    pub motif: Motif,
    /// Explication lisible.
    pub explication: String,
    /// Preuves : identifiants de signaux puis d'événements sources.
    pub preuves: Vec<String>,
}

/// Ordre transmis au consommateur de coupure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrdreCoupure {
    /// Décision d'origine.
    pub decision: u64,
    /// Voie à couper.
    pub voie: VoieId,
    /// Motif.
    pub motif: Motif,
    /// Explication.
    pub explication: String,
    /// Preuves.
    pub preuves: Vec<String>,
}

/// Échec d'exécution d'une coupure par le consommateur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurCoupure(pub String);

impl fmt::Display for ErreurCoupure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "coupure non exécutée : {}", self.0)
    }
}

impl std::error::Error for ErreurCoupure {}

/// Consommateur des décisions de coupure (implémenté par le Portail).
///
/// `couper` doit couper **uniquement** la voie désignée (EX-49) et être
/// idempotent (un même ordre peut être rejoué après un échec).
pub trait CoupureVoie {
    /// Exécute la coupure de `ordre.voie`.
    fn couper(&mut self, ordre: &OrdreCoupure) -> Result<(), ErreurCoupure>;
}

/// Nature d'un événement d'audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NatureAudit {
    /// Décision émise.
    Decision(Decision),
    /// Preuve supplémentaire sur une voie déjà coupée (aucune nouvelle décision).
    PreuveSupplementaire {
        /// Voie.
        voie: VoieId,
        /// Signal.
        signal: String,
    },
    /// Signal déjà traité, écarté.
    SignalDejaTraite {
        /// Signal.
        signal: String,
    },
    /// Coupure confirmée par le consommateur.
    CoupureConfirmee {
        /// Décision.
        decision: u64,
        /// Voie.
        voie: VoieId,
    },
    /// Coupure non confirmée : la décision tient, l'ordre est à rejouer.
    CoupureNonConfirmee {
        /// Décision.
        decision: u64,
        /// Voie.
        voie: VoieId,
        /// Erreur rapportée.
        erreur: String,
    },
    /// Opération refusée.
    Refus {
        /// Motif.
        motif: String,
    },
}

/// Événement d'audit (journal append-only ; inscription au registre Merkle : lot 4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvenementAudit {
    /// Numéro de séquence (strictement croissant, sans trou).
    pub sequence: u64,
    /// Instant (ms) fourni par l'appelant.
    pub horodatage_ms: u64,
    /// Nature.
    pub nature: NatureAudit,
}

#[derive(Debug, Clone)]
struct SuiviVoie {
    etat: EtatVoie,
    coupee_pour_incident: bool,
    coupure_confirmee: bool,
    decision_coupure: Option<u64>,
}

impl SuiviVoie {
    fn ouverte() -> Self {
        Self {
            etat: EtatVoie::Ouverte,
            coupee_pour_incident: false,
            coupure_confirmee: false,
            decision_coupure: None,
        }
    }
}

/// Refus d'une opération manuelle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refus(pub &'static str);

impl fmt::Display for Refus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for Refus {}

/// Sentinelle des voies : état par voie, état global, journal d'audit.
#[derive(Debug)]
pub struct SentinelleVoies {
    politique: PolitiqueVoies,
    global: DegradationState,
    voies: BTreeMap<VoieId, SuiviVoie>,
    signaux_traites: HashSet<String>,
    incident_hors_voie: bool,
    prochaine_decision: u64,
    journal: Vec<EvenementAudit>,
}

impl Default for SentinelleVoies {
    fn default() -> Self {
        Self::new(PolitiqueVoies::default())
    }
}

impl SentinelleVoies {
    /// Crée une sentinelle à T0, sans voie connue.
    pub fn new(politique: PolitiqueVoies) -> Self {
        let mut politique = politique;
        if politique.plafond_automatique > DegradationState::Restricted {
            // T4 n'est jamais automatique.
            politique.plafond_automatique = DegradationState::Restricted;
        }
        Self {
            politique,
            global: DegradationState::Normal,
            voies: BTreeMap::new(),
            signaux_traites: HashSet::new(),
            incident_hors_voie: false,
            prochaine_decision: 1,
            journal: Vec::new(),
        }
    }

    /// État global T0–T4.
    pub fn etat_global(&self) -> DegradationState {
        self.global
    }

    /// État d'une voie ; `None` si la voie n'a jamais été observée (à traiter
    /// comme fermée par l'appelant).
    pub fn etat_voie(&self, voie: &VoieId) -> Option<EtatVoie> {
        self.voies.get(voie).map(|s| s.etat)
    }

    /// Voies coupées dont la coupure n'a pas été confirmée par le consommateur.
    pub fn coupures_non_confirmees(&self) -> Vec<VoieId> {
        self.voies
            .iter()
            .filter(|(_, s)| s.etat == EtatVoie::Coupee && !s.coupure_confirmee)
            .map(|(v, _)| v.clone())
            .collect()
    }

    /// Journal d'audit.
    pub fn journal(&self) -> &[EvenementAudit] {
        &self.journal
    }

    /// Évalue un rapport de CaringNanny et rend les décisions nouvelles.
    pub fn evaluer(&mut self, rapport: &RapportObservation) -> Vec<Decision> {
        let t = rapport.horodatage_ms;
        let mut decisions = Vec::new();
        if self.global == DegradationState::Blocked {
            self.auditer(
                t,
                NatureAudit::Refus {
                    motif: "T4 : aucune décision opérationnelle".to_string(),
                },
            );
            return decisions;
        }
        for v in rapport.sante_voies.keys() {
            self.voies
                .entry(v.clone())
                .or_insert_with(SuiviVoie::ouverte);
        }

        for s in &rapport.signaux {
            if !self.signaux_traites.insert(s.id.clone()) {
                self.auditer(
                    t,
                    NatureAudit::SignalDejaTraite {
                        signal: s.id.clone(),
                    },
                );
                continue;
            }
            self.traiter_signal(s, t, &mut decisions);
        }

        for (voie, sante) in &rapport.sante_voies {
            self.traiter_sante(voie, *sante, t, &mut decisions);
        }

        self.escalader(t, &mut decisions);
        decisions
    }

    /// Transmet les décisions de coupure au consommateur et trace le résultat.
    pub fn appliquer_coupures(
        &mut self,
        decisions: &[Decision],
        consommateur: &mut dyn CoupureVoie,
    ) -> Vec<(u64, Result<(), ErreurCoupure>)> {
        let mut resultats = Vec::new();
        for d in decisions {
            let Action::CouperVoie(voie) = &d.action else {
                continue;
            };
            let ordre = OrdreCoupure {
                decision: d.id,
                voie: voie.clone(),
                motif: d.motif,
                explication: d.explication.clone(),
                preuves: d.preuves.clone(),
            };
            let r = consommateur.couper(&ordre);
            match &r {
                Ok(()) => {
                    if let Some(s) = self.voies.get_mut(voie) {
                        s.coupure_confirmee = true;
                    }
                    self.auditer(
                        d.horodatage_ms,
                        NatureAudit::CoupureConfirmee {
                            decision: d.id,
                            voie: voie.clone(),
                        },
                    );
                }
                Err(e) => self.auditer(
                    d.horodatage_ms,
                    NatureAudit::CoupureNonConfirmee {
                        decision: d.id,
                        voie: voie.clone(),
                        erreur: e.0.clone(),
                    },
                ),
            }
            resultats.push((d.id, r));
        }
        resultats
    }

    /// Réarme (rouvre) une voie coupée ou surveillée. `autorisation` décrit la
    /// décision humaine / TAMR qui l'autorise (non vide).
    pub fn rearmer(
        &mut self,
        voie: &VoieId,
        autorisation: &str,
        horodatage_ms: u64,
    ) -> Result<Decision, Refus> {
        let refus = |s: &mut Self, m: &'static str| {
            s.auditer(
                horodatage_ms,
                NatureAudit::Refus {
                    motif: format!("réarmement de {voie} : {m}"),
                },
            );
            Err(Refus(m))
        };
        if self.global == DegradationState::Blocked {
            return refus(self, "T4 : aucune décision opérationnelle");
        }
        if autorisation.trim().is_empty() {
            return refus(self, "autorisation absente");
        }
        match self.voies.get(voie).map(|s| s.etat) {
            None => return refus(self, "voie inconnue"),
            Some(EtatVoie::Ouverte) => return refus(self, "voie déjà ouverte"),
            Some(_) => {}
        }
        self.voies.insert(voie.clone(), SuiviVoie::ouverte());
        let d = self.decider(
            horodatage_ms,
            Action::RearmerVoie(voie.clone()),
            Motif::Rearmement,
            format!("réarmement autorisé : {autorisation}"),
            Vec::new(),
        );
        Ok(d)
    }

    /// Abaisse l'état global d'un cran (jamais depuis T4 ; T3→T2 = « confirmation
    /// de sécurité »). Efface le drapeau d'incident hors voie au retour à T0.
    pub fn abaisser_global(
        &mut self,
        autorisation: &str,
        horodatage_ms: u64,
    ) -> Result<Decision, Refus> {
        let vers = match self.global {
            DegradationState::Blocked => return Err(Refus("T4 est terminal")),
            DegradationState::Normal => return Err(Refus("déjà en T0")),
            DegradationState::Unstable => DegradationState::Normal,
            DegradationState::Degraded => DegradationState::Unstable,
            DegradationState::Restricted => DegradationState::Degraded,
        };
        if autorisation.trim().is_empty() {
            return Err(Refus("autorisation absente"));
        }
        let de = self.global;
        self.global = vers;
        if vers == DegradationState::Normal {
            self.incident_hors_voie = false;
        }
        Ok(self.decider(
            horodatage_ms,
            Action::ChangerEtatGlobal { de, vers },
            Motif::Rearmement,
            format!("désescalade autorisée : {autorisation}"),
            Vec::new(),
        ))
    }

    fn traiter_signal(&mut self, s: &Signal, t: u64, decisions: &mut Vec<Decision>) {
        let mut preuves = vec![s.id.clone()];
        preuves.extend(s.preuves.iter().map(|p| p.as_str().to_string()));
        match (&s.voie, s.severite) {
            (Some(voie), Severite::Critique) => {
                let suivi = self
                    .voies
                    .entry(voie.clone())
                    .or_insert_with(SuiviVoie::ouverte);
                suivi.coupee_pour_incident = true;
                if suivi.etat == EtatVoie::Coupee {
                    self.auditer(
                        t,
                        NatureAudit::PreuveSupplementaire {
                            voie: voie.clone(),
                            signal: s.id.clone(),
                        },
                    );
                    return;
                }
                suivi.etat = EtatVoie::Coupee;
                suivi.coupure_confirmee = false;
                let d = self.decider(
                    t,
                    Action::CouperVoie(voie.clone()),
                    Motif::IncidentCritique,
                    format!("{} ({}) : {}", s.regle, s.categorie.nom(), s.detail),
                    preuves,
                );
                if let Some(sv) = self.voies.get_mut(voie) {
                    sv.decision_coupure = Some(d.id);
                }
                decisions.push(d);
            }
            (None, Severite::Critique) => {
                self.incident_hors_voie = true;
                decisions.push(self.decider(
                    t,
                    Action::IncidentHorsVoie,
                    Motif::IncidentCritique,
                    format!(
                        "{} ({}) non attribuable à une voie : blocage à la frontière requis — {}",
                        s.regle,
                        s.categorie.nom(),
                        s.detail
                    ),
                    preuves,
                ));
            }
            (Some(voie), Severite::Moyenne | Severite::Elevee) => {
                self.surveiller(
                    voie,
                    Motif::Anomalie,
                    format!("{} : {}", s.regle, s.detail),
                    preuves,
                    t,
                    decisions,
                );
            }
            // Signaux sans voie non critiques (ex. capteur dégradé) : portés par
            // la santé des voies ; info / faible : aucune réaction.
            _ => {}
        }
    }

    fn traiter_sante(
        &mut self,
        voie: &VoieId,
        sante: HealthStatus,
        t: u64,
        decisions: &mut Vec<Decision>,
    ) {
        match sante {
            HealthStatus::Healthy => {}
            HealthStatus::Degraded => self.surveiller(
                voie,
                Motif::CapteurDegrade,
                "capteur dégradé sur la voie (entrées rejetées)".to_string(),
                Vec::new(),
                t,
                decisions,
            ),
            HealthStatus::Inconnu | HealthStatus::Unhealthy => {
                let explication =
                    format!("voie aveugle : santé observée {sante:?} (aucun capteur sain)");
                match self.politique.voie_aveugle {
                    ReponseVoieAveugle::Surveiller => {
                        self.surveiller(
                            voie,
                            Motif::VoieAveugle,
                            explication,
                            Vec::new(),
                            t,
                            decisions,
                        );
                    }
                    ReponseVoieAveugle::Couper => {
                        let suivi = self
                            .voies
                            .entry(voie.clone())
                            .or_insert_with(SuiviVoie::ouverte);
                        if suivi.etat == EtatVoie::Coupee {
                            return;
                        }
                        suivi.etat = EtatVoie::Coupee;
                        suivi.coupure_confirmee = false;
                        let d = self.decider(
                            t,
                            Action::CouperVoie(voie.clone()),
                            Motif::VoieAveugle,
                            explication,
                            Vec::new(),
                        );
                        if let Some(sv) = self.voies.get_mut(voie) {
                            sv.decision_coupure = Some(d.id);
                        }
                        decisions.push(d);
                    }
                }
            }
        }
    }

    fn surveiller(
        &mut self,
        voie: &VoieId,
        motif: Motif,
        explication: String,
        preuves: Vec<String>,
        t: u64,
        decisions: &mut Vec<Decision>,
    ) {
        let suivi = self
            .voies
            .entry(voie.clone())
            .or_insert_with(SuiviVoie::ouverte);
        if suivi.etat != EtatVoie::Ouverte {
            return;
        }
        suivi.etat = EtatVoie::Surveillee;
        decisions.push(self.decider(
            t,
            Action::SurveillerVoie(voie.clone()),
            motif,
            explication,
            preuves,
        ));
    }

    fn cible_globale(&self) -> DegradationState {
        let coupees = self
            .voies
            .values()
            .filter(|s| s.coupee_pour_incident)
            .count();
        let hors_voie = self.incident_hors_voie;
        let cible = if coupees >= 3 || (hors_voie && coupees >= 1) {
            DegradationState::Restricted
        } else if coupees == 2 || hors_voie {
            DegradationState::Degraded
        } else if coupees == 1 {
            DegradationState::Unstable
        } else {
            DegradationState::Normal
        };
        cible.min(self.politique.plafond_automatique)
    }

    fn escalader(&mut self, t: u64, decisions: &mut Vec<Decision>) {
        let cible = self.cible_globale();
        if cible <= self.global {
            return;
        }
        let de = self.global;
        let (vers, regle) = match de {
            DegradationState::Normal => {
                (DegradationState::Unstable, "T0 → T1 : détection d'anomalie")
            }
            DegradationState::Unstable => (
                DegradationState::Degraded,
                "T1 → T2 : persistance d'anomalie",
            ),
            DegradationState::Degraded => (DegradationState::Restricted, "T2 → T3 : aggravation"),
            // T3 → T4 : jamais automatique.
            DegradationState::Restricted | DegradationState::Blocked => return,
        };
        self.global = vers;
        let coupees: Vec<String> = self
            .voies
            .iter()
            .filter(|(_, s)| s.coupee_pour_incident)
            .map(|(v, _)| v.to_string())
            .collect();
        let preuves = self
            .voies
            .values()
            .filter(|s| s.coupee_pour_incident)
            .filter_map(|s| s.decision_coupure.map(|d| format!("decision:{d}")))
            .collect();
        decisions.push(self.decider(
            t,
            Action::ChangerEtatGlobal { de, vers },
            Motif::Escalade,
            format!(
                "{regle} (voies coupées pour incident : [{}] ; incident hors voie : {})",
                coupees.join(", "),
                self.incident_hors_voie
            ),
            preuves,
        ));
    }

    fn decider(
        &mut self,
        t: u64,
        action: Action,
        motif: Motif,
        explication: String,
        preuves: Vec<String>,
    ) -> Decision {
        let d = Decision {
            id: self.prochaine_decision,
            horodatage_ms: t,
            action,
            motif,
            explication,
            preuves,
        };
        self.prochaine_decision += 1;
        self.auditer(t, NatureAudit::Decision(d.clone()));
        d
    }

    fn auditer(&mut self, horodatage_ms: u64, nature: NatureAudit) {
        let sequence = self.journal.len() as u64 + 1;
        self.journal.push(EvenementAudit {
            sequence,
            horodatage_ms,
            nature,
        });
    }
}

impl DegradationManager for SentinelleVoies {
    fn get_current(&self) -> Degradation {
        Degradation { state: self.global }
    }
}
