//! @id     caringnanny_vigies
//! @do     observer_signaux_vigies
//! @role   observation
//! @layer  core
//! @human  caringnanny — module « vigies » : ingestion des signaux des Vigies, santé par capteur et par voie.

//! Maillon CaringNanny de la chaîne Vigies → CaringNanny → WorrySentinel
//! (`COG-VUE-GLOBALE.md` §8, décision D1 : « signaux propagés par CaringNanny »).
//!
//! CaringNanny **observe** (INV-CN-1) : elle ne qualifie pas les signaux, ne
//! décide rien, ne bloque rien (INV-CN-2, INV-CN-6). Elle :
//! - ingère les [`LotVigie`] et écarte les signaux déjà vus (rejeu, doublon) ;
//! - tient la santé de chaque capteur et de chaque voie couverte ;
//! - transmet à WorrySentinel un [`RapportObservation`] : les signaux nouveaux
//!   **inchangés** (INV-CN-7) et la santé observée ;
//! - garde la trace de chaque ingestion (INV-CN-5) via [`ObservatoireVigies::journal`].
//!
//! Santé d'un capteur (le temps est fourni par l'appelant, aucune horloge lue) :
//! - non enregistré, jamais de pouls, pouls périmé (> délai) ou daté du futur → `Inconnu` ;
//! - dernière relève en échec → `Unhealthy` ;
//! - dernier lot dégradé (entrées rejetées) → `Degraded` ;
//! - sinon `Healthy`.
//!
//! Santé d'une voie : le pire des capteurs enregistrés qui la couvrent ;
//! aucun capteur → `Inconnu` (jamais `Healthy` par défaut, C-12).

use std::collections::{BTreeMap, HashSet, VecDeque};

use miyuvigie::{CapteurId, Compteurs, EtatCapteur, LotVigie, Signal, VoieId};

use crate::health::HealthStatus;
use crate::observer::{DefaultObserver, EventType, Observer, SystemEvent};

/// Capacité par défaut de la mémoire des signaux déjà vus.
pub const CAPACITE_SIGNAUX_VUS: usize = 65_536;

/// Capteur attendu (déclaré à l'installation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapteurAttendu {
    /// Capteur.
    pub capteur: CapteurId,
    /// Voies qu'il couvre.
    pub voies: Vec<VoieId>,
    /// Délai maximal entre deux pouls avant que le capteur soit `Inconnu` (ms).
    pub delai_pouls_max_ms: u64,
}

#[derive(Debug, Clone)]
struct SuiviCapteur {
    attendu: CapteurAttendu,
    dernier_pouls_ms: Option<u64>,
    dernier_etat: Option<EtatCapteur>,
    cumul: Compteurs,
}

/// Accusé d'ingestion d'un lot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecuIngestion {
    /// Signaux nouveaux mis en attente de transmission.
    pub nouveaux: usize,
    /// Signaux déjà vus, écartés.
    pub doublons: usize,
    /// Vrai si le capteur n'est pas enregistré (pouls ignoré, signaux transmis).
    pub capteur_inconnu: bool,
}

/// Rapport transmis à WorrySentinel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RapportObservation {
    /// Instant d'établissement (ms, fourni par l'appelant).
    pub horodatage_ms: u64,
    /// Signaux nouveaux depuis le précédent rapport, inchangés (INV-CN-7).
    pub signaux: Vec<Signal>,
    /// Santé de chaque voie connue.
    pub sante_voies: BTreeMap<VoieId, HealthStatus>,
    /// Santé de chaque capteur enregistré.
    pub sante_capteurs: BTreeMap<CapteurId, HealthStatus>,
}

/// Observatoire des Vigies.
#[derive(Debug)]
pub struct ObservatoireVigies {
    capteurs: BTreeMap<CapteurId, SuiviCapteur>,
    vus: HashSet<String>,
    ordre_vus: VecDeque<String>,
    capacite_vus: usize,
    en_attente: Vec<Signal>,
    lots_capteur_inconnu: u64,
    journal: DefaultObserver,
}

impl Default for ObservatoireVigies {
    fn default() -> Self {
        Self::new(CAPACITE_SIGNAUX_VUS)
    }
}

impl ObservatoireVigies {
    /// Crée un observatoire (mémoire des signaux vus bornée à `capacite_vus`).
    pub fn new(capacite_vus: usize) -> Self {
        Self {
            capteurs: BTreeMap::new(),
            vus: HashSet::new(),
            ordre_vus: VecDeque::new(),
            capacite_vus: capacite_vus.max(1),
            en_attente: Vec::new(),
            lots_capteur_inconnu: 0,
            journal: DefaultObserver::new(),
        }
    }

    /// Enregistre un capteur attendu (remplace une déclaration précédente,
    /// en conservant l'historique de pouls).
    pub fn enregistrer_capteur(&mut self, attendu: CapteurAttendu) {
        match self.capteurs.get_mut(&attendu.capteur) {
            Some(s) => s.attendu = attendu,
            None => {
                self.capteurs.insert(
                    attendu.capteur.clone(),
                    SuiviCapteur {
                        attendu,
                        dernier_pouls_ms: None,
                        dernier_etat: None,
                        cumul: Compteurs::default(),
                    },
                );
            }
        }
    }

    /// Ingère un lot d'une Vigie.
    pub fn ingerer(&mut self, lot: LotVigie) -> RecuIngestion {
        let mut recu = RecuIngestion::default();
        if let Some(suivi) = self.capteurs.get_mut(&lot.capteur) {
            if let Some(p) = lot.pouls.as_ref().filter(|p| p.capteur == lot.capteur) {
                suivi.dernier_pouls_ms = Some(
                    suivi
                        .dernier_pouls_ms
                        .map_or(p.horodatage_ms, |d| d.max(p.horodatage_ms)),
                );
            }
            suivi.dernier_etat = Some(lot.etat);
            suivi.cumul.cumuler(&lot.compteurs);
        } else {
            recu.capteur_inconnu = true;
            self.lots_capteur_inconnu += 1;
        }
        if lot.etat != EtatCapteur::Nominal {
            self.journal.observe(SystemEvent {
                event_type: EventType::HealthChange,
                component: format!("vigie:{}", lot.capteur),
                message: format!("capteur {:?}", lot.etat),
            });
        }
        for s in lot.signaux {
            if self.vus.contains(&s.id) {
                recu.doublons += 1;
                continue;
            }
            self.memoriser(s.id.clone());
            self.journal.observe(SystemEvent {
                event_type: EventType::Anomaly,
                component: format!("vigie:{}", s.capteur),
                message: format!("signal {} ({:?}, {})", s.id, s.severite, s.regle),
            });
            self.en_attente.push(s);
            recu.nouveaux += 1;
        }
        recu
    }

    /// Santé d'un capteur à l'instant donné.
    pub fn sante_capteur(&self, capteur: &CapteurId, maintenant_ms: u64) -> HealthStatus {
        let Some(s) = self.capteurs.get(capteur) else {
            return HealthStatus::Inconnu;
        };
        if s.dernier_etat == Some(EtatCapteur::EnEchec) {
            return HealthStatus::Unhealthy;
        }
        let frais = s.dernier_pouls_ms.is_some_and(|p| {
            p <= maintenant_ms && maintenant_ms - p <= s.attendu.delai_pouls_max_ms
        });
        if !frais {
            return HealthStatus::Inconnu;
        }
        match s.dernier_etat {
            Some(EtatCapteur::Nominal) => HealthStatus::Healthy,
            Some(EtatCapteur::Degrade) => HealthStatus::Degraded,
            // Pouls sans état (impossible par construction) : fail-closed.
            Some(EtatCapteur::EnEchec) | None => HealthStatus::Inconnu,
        }
    }

    /// Santé d'une voie : le pire des capteurs qui la couvrent ; aucun → `Inconnu`.
    pub fn sante_voie(&self, voie: &VoieId, maintenant_ms: u64) -> HealthStatus {
        self.capteurs
            .values()
            .filter(|s| s.attendu.voies.contains(voie))
            .map(|s| self.sante_capteur(&s.attendu.capteur, maintenant_ms))
            .reduce(HealthStatus::pire)
            .unwrap_or(HealthStatus::Inconnu)
    }

    /// Établit le rapport pour WorrySentinel et vide la file des signaux.
    ///
    /// Les voies rapportées sont celles des capteurs enregistrés et celles
    /// citées par les signaux transmis.
    pub fn transmettre(&mut self, maintenant_ms: u64) -> RapportObservation {
        let signaux = std::mem::take(&mut self.en_attente);
        let mut voies: Vec<VoieId> = self
            .capteurs
            .values()
            .flat_map(|s| s.attendu.voies.iter().cloned())
            .collect();
        voies.extend(signaux.iter().filter_map(|s| s.voie.clone()));
        let sante_voies = voies
            .into_iter()
            .map(|v| {
                let h = self.sante_voie(&v, maintenant_ms);
                (v, h)
            })
            .collect();
        let sante_capteurs = self
            .capteurs
            .keys()
            .map(|c| (c.clone(), self.sante_capteur(c, maintenant_ms)))
            .collect();
        RapportObservation {
            horodatage_ms: maintenant_ms,
            signaux,
            sante_voies,
            sante_capteurs,
        }
    }

    /// Compteurs cumulés d'un capteur enregistré.
    pub fn compteurs(&self, capteur: &CapteurId) -> Option<Compteurs> {
        self.capteurs.get(capteur).map(|s| s.cumul)
    }

    /// Lots reçus de capteurs non enregistrés.
    pub fn lots_capteur_inconnu(&self) -> u64 {
        self.lots_capteur_inconnu
    }

    /// Trace des observations (INV-CN-5).
    pub fn journal(&self) -> &[SystemEvent] {
        self.journal.events()
    }

    fn memoriser(&mut self, id: String) {
        if self.ordre_vus.len() >= self.capacite_vus {
            if let Some(ancien) = self.ordre_vus.pop_front() {
                self.vus.remove(&ancien);
            }
        }
        self.vus.insert(id.clone());
        self.ordre_vus.push_back(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuvigie::{Categorie, EvenementId, Pouls, RegleId, Severite, Zone};

    fn capteur(s: &str) -> CapteurId {
        CapteurId::new(s).unwrap()
    }
    fn voie(s: &str) -> VoieId {
        VoieId::new(s).unwrap()
    }

    fn lot(c: &str, pouls: Option<u64>, etat: EtatCapteur, signaux: Vec<Signal>) -> LotVigie {
        LotVigie {
            capteur: capteur(c),
            voies_couvertes: vec![voie("jaymail")],
            signaux,
            pouls: pouls.map(|h| Pouls {
                capteur: capteur(c),
                horodatage_ms: h,
            }),
            etat,
            compteurs: Compteurs::default(),
        }
    }

    fn signal(id: &str) -> Signal {
        Signal {
            id: id.to_string(),
            capteur: capteur("v1"),
            horodatage_ms: 1,
            voie: Some(voie("jaymail")),
            zone_src: Zone::Cog,
            zone_dst: Zone::Exterieur,
            severite: Severite::Critique,
            categorie: Categorie::SortieZoneCog,
            regle: RegleId::new("R-EX12-SORTIE").unwrap(),
            preuves: vec![EvenementId::new("e1").unwrap()],
            detail: String::new(),
        }
    }

    fn observatoire() -> ObservatoireVigies {
        let mut o = ObservatoireVigies::default();
        o.enregistrer_capteur(CapteurAttendu {
            capteur: capteur("v1"),
            voies: vec![voie("jaymail")],
            delai_pouls_max_ms: 1_000,
        });
        o
    }

    /// @id caringnanny_vigies_test_capteur_perime_inconnu
    /// @role test
    /// @layer core
    /// @do verify_stale_sensor_is_unknown
    //@human / @human Un capteur sans pouls ou au pouls périmé est `Inconnu`, jamais `Healthy`.
    /// @depends caringnanny_vigies
    #[test]
    fn capteur_sans_pouls_ou_perime_est_inconnu() {
        let mut o = observatoire();
        assert_eq!(o.sante_capteur(&capteur("v1"), 0), HealthStatus::Inconnu);
        assert_eq!(o.sante_voie(&voie("jaymail"), 0), HealthStatus::Inconnu);
        o.ingerer(lot("v1", Some(10_000), EtatCapteur::Nominal, vec![]));
        assert_eq!(
            o.sante_capteur(&capteur("v1"), 10_500),
            HealthStatus::Healthy
        );
        // Périmé.
        assert_eq!(
            o.sante_capteur(&capteur("v1"), 11_001),
            HealthStatus::Inconnu
        );
        assert_eq!(
            o.sante_voie(&voie("jaymail"), 11_001),
            HealthStatus::Inconnu
        );
        // Pouls daté du futur.
        assert_eq!(
            o.sante_capteur(&capteur("v1"), 9_000),
            HealthStatus::Inconnu
        );
        // Capteur non enregistré, voie non couverte.
        assert_eq!(
            o.sante_capteur(&capteur("x"), 10_000),
            HealthStatus::Inconnu
        );
        assert_eq!(o.sante_voie(&voie("autre"), 10_000), HealthStatus::Inconnu);
    }

    #[test]
    fn capteur_degrade_puis_en_echec() {
        let mut o = observatoire();
        o.ingerer(lot("v1", Some(10_000), EtatCapteur::Nominal, vec![]));
        // Lot dégradé : pas de pouls, état Degrade ; l'ancien pouls est encore frais.
        o.ingerer(lot("v1", None, EtatCapteur::Degrade, vec![]));
        assert_eq!(
            o.sante_capteur(&capteur("v1"), 10_100),
            HealthStatus::Degraded
        );
        // Plus de pouls frais : Inconnu (pas « dégradé mais fonctionnel »).
        assert_eq!(
            o.sante_capteur(&capteur("v1"), 20_000),
            HealthStatus::Inconnu
        );
        o.ingerer(lot("v1", None, EtatCapteur::EnEchec, vec![]));
        assert_eq!(
            o.sante_capteur(&capteur("v1"), 10_100),
            HealthStatus::Unhealthy
        );
        assert_eq!(o.journal().len(), 2);
    }

    #[test]
    fn signaux_dedoublonnes_et_transmis_inchanges() {
        let mut o = observatoire();
        let r = o.ingerer(lot(
            "v1",
            Some(1),
            EtatCapteur::Nominal,
            vec![signal("s1"), signal("s1")],
        ));
        assert_eq!((r.nouveaux, r.doublons), (1, 1));
        let r = o.ingerer(lot("v1", Some(2), EtatCapteur::Nominal, vec![signal("s1")]));
        assert_eq!((r.nouveaux, r.doublons), (0, 1));
        let rapport = o.transmettre(2);
        assert_eq!(rapport.signaux, vec![signal("s1")]);
        assert_eq!(
            rapport.sante_voies.get(&voie("jaymail")),
            Some(&HealthStatus::Healthy)
        );
        // File vidée.
        assert_eq!(o.transmettre(3).signaux, Vec::new());
    }

    #[test]
    fn capteur_non_enregistre() {
        let mut o = observatoire();
        let r = o.ingerer(lot(
            "intrus",
            Some(1),
            EtatCapteur::Nominal,
            vec![signal("s9")],
        ));
        assert!(r.capteur_inconnu);
        assert_eq!(o.lots_capteur_inconnu(), 1);
        assert_eq!(
            o.sante_capteur(&capteur("intrus"), 1),
            HealthStatus::Inconnu
        );
    }
}
