//! @id     miyuhybride_quotas
//! @do     appliquer_quotas_contrat_service
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — application des quotas déclarés par le contrat de service (disque, sessions, tâches) : compteurs atomiques sans course, refus au-delà, libération par jeton (fin, inactivité, destruction de la façade).

//! # Quotas du contrat de service (tag 5, ressources)
//!
//! Le contrat déclare (SERVICES-HYBRIDES §4.1) : CPU %, mémoire, **disque**,
//! **tâches**, **sessions**. CPU et mémoire relèvent de la sandbox Franche
//! (rlimits / cgroups de `miyudouane`) ; les trois compteurs applicatifs sont
//! appliqués ici, **par service** (partagés par toutes ses façades et tâches) :
//!
//! - **disque** (Mio → octets) : tout stockage d'une façade réserve ses octets
//!   **avant** d'écrire ([`ReservationDisque`]) ; la DB secondaire y est
//!   adossée ([`crate::DbSecondaire::avec_quota`]) ;
//! - **sessions** : sessions clientes simultanées servies par les façades du
//!   service ([`SessionsClients`]) ; libérées à la fin explicite, après le
//!   délai d'inactivité, au retrait de l'appareil
//!   ([`crate::FacadeJetable::auteur_retire`], [`SessionsClients::retenir`])
//!   ou à la destruction de la façade ;
//! - **tâches** : traitements simultanés ([`JetonTache`]) **à l'échelle du
//!   service**. Une façade ne traite qu'une requête à la fois (`&mut self`) :
//!   le jeton qu'elle prend par requête ne borne aucun parallélisme interne,
//!   il compte la façade comme occupée ; le quota borne donc le nombre de
//!   façades (et de tâches de fond) du service actives **en même temps**
//!   (quota inter-façades).
//!
//! # Sans course
//!
//! Chaque compteur est un `AtomicU64` modifié par comparaison-échange : la
//! vérification de la limite et l'incrément sont **une seule** opération
//! atomique (aucun « vérifier puis agir ») ; la limite ne peut jamais être
//! dépassée, quel que soit le nombre de tâches concurrentes. La libération
//! passe par `Drop` des jetons : un jeton perdu (panique, façade détruite)
//! rend sa part, une part n'est jamais rendue deux fois.
//!
//! Fail-closed : quota épuisé ⇒ [`ErreurHybride::QuotaDepasse`], rien n'est
//! écrit ni ouvert.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::erreurs::ErreurHybride;
use crate::manifeste::Ressources;

const MIO: u64 = 1024 * 1024;

/// Délai d'inactivité par défaut d'une session cliente (choix fail-closed,
/// non fixé par la doc gelée : durée d'un visa, 15 min).
pub const DELAI_INACTIVITE_DEFAUT_SECS: u64 = 15 * 60;

/// Ressource comptée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ressource {
    /// Octets de stockage (DB secondaire comprise).
    Disque,
    /// Sessions clientes simultanées.
    Sessions,
    /// Tâches simultanées.
    Taches,
}

impl std::fmt::Display for Ressource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Disque => "disque",
            Self::Sessions => "sessions",
            Self::Taches => "tâches",
        })
    }
}

/// Compteur borné, atomique.
#[derive(Debug)]
pub struct Quota {
    ressource: Ressource,
    limite: u64,
    utilise: AtomicU64,
}

impl Quota {
    fn new(ressource: Ressource, limite: u64) -> Arc<Self> {
        Arc::new(Self { ressource, limite, utilise: AtomicU64::new(0) })
    }

    /// Ressource comptée.
    pub fn ressource(&self) -> Ressource {
        self.ressource
    }

    /// Limite déclarée par le contrat.
    pub fn limite(&self) -> u64 {
        self.limite
    }

    /// Part utilisée.
    pub fn utilise(&self) -> u64 {
        self.utilise.load(Ordering::Acquire)
    }

    /// Part disponible.
    pub fn disponible(&self) -> u64 {
        self.limite.saturating_sub(self.utilise())
    }

    /// Prend `n` en une seule opération atomique, ou refuse sans rien prendre.
    fn prendre(&self, n: u64) -> Result<(), ErreurHybride> {
        if n == 0 {
            return Ok(());
        }
        let mut u = self.utilise.load(Ordering::Acquire);
        loop {
            let total = u.checked_add(n).filter(|t| *t <= self.limite).ok_or(ErreurHybride::QuotaDepasse(self.ressource))?;
            match self.utilise.compare_exchange_weak(u, total, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => return Ok(()),
                Err(vu) => u = vu,
            }
        }
    }

    /// Rend `n` (jamais sous zéro).
    fn rendre(&self, n: u64) {
        if n == 0 {
            return;
        }
        let mut u = self.utilise.load(Ordering::Acquire);
        loop {
            match self.utilise.compare_exchange_weak(u, u.saturating_sub(n), Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => return,
                Err(vu) => u = vu,
            }
        }
    }
}

/// Octets de disque réservés ; rendus à la destruction.
#[derive(Debug)]
pub struct ReservationDisque {
    quota: Arc<Quota>,
    octets: u64,
}

impl ReservationDisque {
    /// Octets réservés.
    pub fn octets(&self) -> u64 {
        self.octets
    }

    /// Réserve `n` octets de plus **avant** l'écriture ; refus : rien n'est pris.
    pub fn agrandir(&mut self, n: u64) -> Result<(), ErreurHybride> {
        self.quota.prendre(n)?;
        self.octets += n;
        Ok(())
    }

    /// Rend jusqu'à `n` octets (après effacement).
    pub fn reduire(&mut self, n: u64) {
        let n = n.min(self.octets);
        self.quota.rendre(n);
        self.octets -= n;
    }
}

impl Drop for ReservationDisque {
    fn drop(&mut self) {
        self.quota.rendre(self.octets);
    }
}

/// Une session cliente ouverte ; rendue à la destruction.
#[derive(Debug)]
pub struct JetonSession {
    quota: Arc<Quota>,
}

impl Drop for JetonSession {
    fn drop(&mut self) {
        self.quota.rendre(1);
    }
}

/// Une tâche en cours ; rendue à la destruction.
#[derive(Debug)]
pub struct JetonTache {
    quota: Arc<Quota>,
}

impl Drop for JetonTache {
    fn drop(&mut self) {
        self.quota.rendre(1);
    }
}

/// Quotas d'un service, tirés de son contrat signé ; clonable (partagé).
#[derive(Debug, Clone)]
pub struct QuotasService {
    disque: Arc<Quota>,
    sessions: Arc<Quota>,
    taches: Arc<Quota>,
}

impl QuotasService {
    /// Quotas déclarés par les ressources du contrat (disque en Mio).
    pub fn depuis(r: &Ressources) -> Self {
        Self {
            disque: Quota::new(Ressource::Disque, u64::from(r.disque_mio).saturating_mul(MIO)),
            sessions: Quota::new(Ressource::Sessions, u64::from(r.sessions_max)),
            taches: Quota::new(Ressource::Taches, u64::from(r.taches_max)),
        }
    }

    /// Compteur du disque (octets).
    pub fn disque(&self) -> &Quota {
        &self.disque
    }

    /// Compteur des sessions.
    pub fn sessions(&self) -> &Quota {
        &self.sessions
    }

    /// Compteur des tâches.
    pub fn taches(&self) -> &Quota {
        &self.taches
    }

    /// Réserve `octets` de disque avant écriture.
    pub fn reserver_disque(&self, octets: u64) -> Result<ReservationDisque, ErreurHybride> {
        self.disque.prendre(octets)?;
        Ok(ReservationDisque { quota: Arc::clone(&self.disque), octets })
    }

    /// Ouvre une session (refus au-delà de `sessions_max`).
    pub fn ouvrir_session(&self) -> Result<JetonSession, ErreurHybride> {
        self.sessions.prendre(1)?;
        Ok(JetonSession { quota: Arc::clone(&self.sessions) })
    }

    /// Démarre une tâche (refus au-delà de `taches_max`).
    pub fn lancer_tache(&self) -> Result<JetonTache, ErreurHybride> {
        self.taches.prendre(1)?;
        Ok(JetonTache { quota: Arc::clone(&self.taches) })
    }
}

#[derive(Debug)]
struct SessionOuverte {
    _jeton: JetonSession,
    derniere_activite: u64,
}

/// Sessions clientes d'une façade, une par clé (ex. clé d'appareil), sous le
/// quota de sessions du service.
///
/// - [`SessionsClients::toucher`] expire d'abord les sessions inactives, puis
///   prolonge la session de la clé ou en ouvre une (quota) ;
/// - [`SessionsClients::fermer`] : fin explicite ;
/// - [`SessionsClients::vider`] ou destruction : tout est rendu.
#[derive(Debug)]
pub struct SessionsClients<K: Ord> {
    quotas: QuotasService,
    delai_inactivite_secs: u64,
    sessions: BTreeMap<K, SessionOuverte>,
}

impl<K: Ord + Clone> SessionsClients<K> {
    /// Aucune session ; `delai_inactivite_secs` ≥ 1.
    pub fn new(quotas: QuotasService, delai_inactivite_secs: u64) -> Self {
        Self { quotas, delai_inactivite_secs: delai_inactivite_secs.max(1), sessions: BTreeMap::new() }
    }

    /// Délai d'inactivité.
    pub fn delai_inactivite_secs(&self) -> u64 {
        self.delai_inactivite_secs
    }

    fn inactive(&self, s: &SessionOuverte, maintenant: u64) -> bool {
        maintenant > s.derniere_activite.saturating_add(self.delai_inactivite_secs)
    }

    /// Libère les sessions inactives ; rend leur nombre.
    pub fn expirer(&mut self, maintenant: u64) -> usize {
        let avant = self.sessions.len();
        let delai = self.delai_inactivite_secs;
        self.sessions.retain(|_, s| maintenant <= s.derniere_activite.saturating_add(delai));
        avant - self.sessions.len()
    }

    /// Activité de `cle` : session prolongée, ou ouverte sous le quota.
    pub fn toucher(&mut self, cle: &K, maintenant: u64) -> Result<(), ErreurHybride> {
        self.expirer(maintenant);
        if let Some(s) = self.sessions.get_mut(cle) {
            s.derniere_activite = s.derniere_activite.max(maintenant);
            return Ok(());
        }
        let jeton = self.quotas.ouvrir_session()?;
        self.sessions.insert(cle.clone(), SessionOuverte { _jeton: jeton, derniere_activite: maintenant });
        Ok(())
    }

    /// Vrai si `cle` a une session active à `maintenant`.
    pub fn est_ouverte(&self, cle: &K, maintenant: u64) -> bool {
        self.sessions.get(cle).is_some_and(|s| !self.inactive(s, maintenant))
    }

    /// Fin explicite de la session de `cle`.
    pub fn fermer(&mut self, cle: &K) -> bool {
        self.sessions.remove(cle).is_some()
    }

    /// Ne garde que les sessions dont la clé satisfait `garder` (ex. auteur retiré).
    pub fn retenir(&mut self, mut garder: impl FnMut(&K) -> bool) {
        self.sessions.retain(|k, _| garder(k));
    }

    /// Libère toutes les sessions (façade condamnée).
    pub fn vider(&mut self) {
        self.sessions.clear();
    }

    /// Sessions ouvertes (actives ou pas encore expirées).
    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    /// Vrai si aucune session.
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::*;

    fn ressources(disque_mio: u32, sessions_max: u32, taches_max: u32) -> Ressources {
        Ressources { cpu_pourcent: 10, memoire_mio: 64, disque_mio, taches_max, sessions_max }
    }

    #[test]
    fn quota_refuse_au_dela_et_rend_a_la_destruction() {
        let q = QuotasService::depuis(&ressources(1, 2, 1));
        assert_eq!(q.disque().limite(), MIO);
        let a = q.ouvrir_session().unwrap();
        let b = q.ouvrir_session().unwrap();
        assert_eq!(q.ouvrir_session().unwrap_err(), ErreurHybride::QuotaDepasse(Ressource::Sessions));
        drop(a);
        let _c = q.ouvrir_session().unwrap();
        drop(b);
        assert_eq!(q.sessions().utilise(), 1);

        let t = q.lancer_tache().unwrap();
        assert_eq!(q.lancer_tache().unwrap_err(), ErreurHybride::QuotaDepasse(Ressource::Taches));
        drop(t);
        assert!(q.lancer_tache().is_ok());

        let mut r = q.reserver_disque(MIO - 10).unwrap();
        assert_eq!(r.agrandir(11), Err(ErreurHybride::QuotaDepasse(Ressource::Disque)));
        assert_eq!(r.octets(), MIO - 10, "refus : rien n'est pris");
        r.agrandir(10).unwrap();
        assert!(q.reserver_disque(1).is_err());
        r.reduire(u64::MAX);
        assert_eq!(q.disque().utilise(), 0);
        r.agrandir(5).unwrap();
        drop(r);
        assert_eq!(q.disque().utilise(), 0);
    }

    #[test]
    fn sessions_expirent_apres_inactivite_et_se_liberent() {
        let q = QuotasService::depuis(&ressources(1, 2, 1));
        let mut s = SessionsClients::new(q.clone(), 60);
        s.toucher(&1u8, 100).unwrap();
        s.toucher(&2u8, 100).unwrap();
        assert_eq!(s.toucher(&3u8, 120), Err(ErreurHybride::QuotaDepasse(Ressource::Sessions)));
        // Activité de 1 : prolongée ; 2 inactive au-delà de 60 s.
        s.toucher(&1u8, 150).unwrap();
        assert!(s.est_ouverte(&2u8, 160));
        assert!(!s.est_ouverte(&2u8, 161));
        s.toucher(&3u8, 161).unwrap();
        assert_eq!(s.len(), 2);
        assert!(!s.est_ouverte(&2u8, 161));
        // Fin explicite, puis destruction : tout est rendu.
        assert!(s.fermer(&1u8));
        assert_eq!(q.sessions().utilise(), 1);
        drop(s);
        assert_eq!(q.sessions().utilise(), 0);
    }
}
