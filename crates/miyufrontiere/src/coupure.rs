//! @id     miyufrontiere_coupure
//! @do     executer_coupure_voie_portail_douane
//! @role   integration
//! @layer  operateur
//! @human  miyufrontiere — adaptateur WorrySentinel → Portail + Douane : un ordre de coupure ferme la voie au Portail et à l'IPS (EX-49).

//! # WorrySentinel → Portail et Douane
//!
//! [`CoupureFrontiere`] implémente `worrysentinel::CoupureVoie` : chaque
//! [`OrdreCoupure`] est exécuté
//!
//! 1. au **Portail** : `miyuportail::CoupureVoie::couper(service, Anomalie)`
//!    (session close, files purgées, incident inscrit) ;
//! 2. sur chaque **IPS** de la Douane enregistré :
//!    `MoteurIps::appliquer(DecisionSentinelle::CouperVoie(voie))` (flux de
//!    la voie bloqués, connexions en cours oubliées ; les règles nftables
//!    rendues avec `MoteurIps::voies_coupees()` omettent la voie).
//!
//! La correspondance voie ↔ service est **explicite**
//! ([`CoupureFrontiere::declarer`]) : une voie absente de la table est une
//! erreur ; WorrySentinel trace alors `CoupureNonConfirmee` et l'ordre reste à
//! rejouer (fail-closed). L'identifiant de voie de la Douane est celui des
//! Vigies (une voie par service, identifiants des arrêtés).
//!
//! Un échec d'un exécutant n'arrête pas les autres : la coupure est tentée
//! partout, puis l'erreur est rendue. `couper` est idempotent.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use miyudouane::{DecisionSentinelle, MoteurIps};
use miyuportail::{IdService, MotifCoupure};
use miyuvigie::VoieId;
use worrysentinel::{CoupureVoie, ErreurCoupure, OrdreCoupure};

/// IPS partagé entre la boucle de la Douane et l'exécutant des coupures.
pub type IpsPartage = Arc<Mutex<MoteurIps>>;

/// Horloge (secondes Unix) des décisions appliquées à l'IPS.
pub type Horloge = Box<dyn Fn() -> u64 + Send>;

/// Exécutant des coupures de voie : Portail + IPS de la Douane.
pub struct CoupureFrontiere<P: miyuportail::CoupureVoie> {
    portail: P,
    voies: BTreeMap<VoieId, IdService>,
    ips: Vec<IpsPartage>,
    horloge: Horloge,
}

impl<P: miyuportail::CoupureVoie> std::fmt::Debug for CoupureFrontiere<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoupureFrontiere")
            .field("voies", &self.voies)
            .field("ips", &self.ips.len())
            .finish_non_exhaustive()
    }
}

fn horloge_systeme() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

impl<P: miyuportail::CoupureVoie> CoupureFrontiere<P> {
    /// Exécutant sur une poignée de coupure du Portail
    /// (`PortailInterne::poignee_coupure`), sans voie ni IPS déclarés.
    pub fn new(portail: P) -> Self {
        Self { portail, voies: BTreeMap::new(), ips: Vec::new(), horloge: Box::new(horloge_systeme) }
    }

    /// Remplace l'horloge (tests, horloge de confiance).
    #[must_use]
    pub fn avec_horloge(mut self, horloge: Horloge) -> Self {
        self.horloge = horloge;
        self
    }

    /// Déclare la voie `voie` (WorrySentinel, Vigies, Douane) ↔ service du Portail.
    /// Refus si la voie est déjà associée à un autre service.
    pub fn declarer(&mut self, voie: VoieId, service: IdService) -> Result<(), ErreurCoupure> {
        match self.voies.get(&voie) {
            Some(s) if *s != service => Err(ErreurCoupure(format!("voie {voie} déjà associée au service {s}"))),
            _ => {
                self.voies.insert(voie, service);
                Ok(())
            }
        }
    }

    /// Enregistre l'IPS d'une instance Douane (une par frontière).
    pub fn ajouter_ips(&mut self, ips: IpsPartage) {
        self.ips.push(ips);
    }

    /// Service du Portail associé à une voie.
    pub fn service(&self, voie: &VoieId) -> Option<&IdService> {
        self.voies.get(voie)
    }
}

impl<P: miyuportail::CoupureVoie> CoupureVoie for CoupureFrontiere<P> {
    fn couper(&mut self, ordre: &OrdreCoupure) -> Result<(), ErreurCoupure> {
        let Some(service) = self.voies.get(&ordre.voie) else {
            return Err(ErreurCoupure(format!("voie {} inconnue de la table voie ↔ service", ordre.voie)));
        };
        let mut erreurs = Vec::new();
        // Toute coupure de WorrySentinel est une anomalie détectée (signal ou voie aveugle).
        if let Err(e) = self.portail.couper(service, MotifCoupure::Anomalie) {
            erreurs.push(format!("Portail : {e}"));
        }
        let maintenant = (self.horloge)();
        for (i, ips) in self.ips.iter().enumerate() {
            match ips.lock() {
                Ok(mut m) => m.appliquer(DecisionSentinelle::CouperVoie(ordre.voie.as_str().to_owned()), maintenant),
                Err(_) => erreurs.push(format!("IPS n° {i} : verrou empoisonné")),
            }
        }
        if erreurs.is_empty() {
            Ok(())
        } else {
            Err(ErreurCoupure(erreurs.join(" ; ")))
        }
    }
}
