//! @id     miyucloud_application_services_reconciliation_references_service
//! @do     reconcilier_compteurs_references_blobs
//! @role   donnee
//! @layer  app
//! @human  miyucloud — réconciliation périodique et bornée de `storage.blobs.ref_count` depuis `storage.files`, puis ramassage des blobs sans référence.

//! Réconciliation des compteurs de références des blobs.
//!
//! Toute modification de `storage.files.blob_hash` ajuste
//! `storage.blobs.ref_count` dans la même transaction ; un arrêt brutal ne
//! laisse donc pas de compteur faux. Restent les compteurs hérités
//! (versions antérieures : compensation hors transaction, références prises
//! sans ligne de fichier) : cette tâche les recalcule depuis
//! `storage.files`, par lots verrouillés, puis le ramasse-miettes supprime
//! les blobs devenus sans référence.
//!
//! **Bornée** : au plus `lots_max` lots de `taille_lot` blobs par passage ;
//! le curseur est conservé et le balayage reprend au passage suivant.
//! **Journalisée** : chaque compteur corrigé (dépôt) et le bilan du passage.

use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::application::services::supervision::superviser_tache_periodique;
use crate::common::errors::DomainError;

/// Bilan d'un lot de réconciliation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LotReconcilie {
    /// Blobs examinés (verrouillés) dans le lot.
    pub examines: u64,
    /// Blobs dont `ref_count` a été corrigé.
    pub corriges: u64,
    /// Dernier hachage du lot (`None` : plus rien après le curseur).
    pub dernier: Option<String>,
}

/// Accès au magasin de blobs (PostgreSQL en production, mémoire en test).
pub trait DepotReferences: Send + Sync + 'static {
    /// Recalcule `ref_count` des au plus `limite` blobs de hachage > `apres`.
    fn reconcilier_lot(
        &self,
        apres: Option<&str>,
        limite: i64,
    ) -> impl Future<Output = Result<LotReconcilie, DomainError>> + Send;

    /// Supprime les blobs sans référence ; rend (blobs, octets).
    fn ramasser_orphelins(&self) -> impl Future<Output = Result<(u64, u64), DomainError>> + Send;
}

/// Bilan d'un passage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BilanReconciliation {
    /// Blobs examinés.
    pub examines: u64,
    /// Compteurs corrigés.
    pub corriges: u64,
    /// Vrai si le balayage complet s'est achevé pendant ce passage.
    pub balayage_termine: bool,
    /// Blobs supprimés par le ramasse-miettes.
    pub blobs_supprimes: u64,
    /// Octets libérés.
    pub octets_liberes: u64,
}

/// Tâche de réconciliation des références.
pub struct ReconciliationReferencesService<D: DepotReferences> {
    depot: Arc<D>,
    taille_lot: i64,
    lots_max: u32,
    curseur: Mutex<Option<String>>,
}

impl<D: DepotReferences> ReconciliationReferencesService<D> {
    /// Tâche bornée à `lots_max` lots de `taille_lot` blobs par passage.
    pub fn new(depot: Arc<D>, taille_lot: i64, lots_max: u32) -> Self {
        Self {
            depot,
            taille_lot: taille_lot.max(1),
            lots_max: lots_max.max(1),
            curseur: Mutex::new(None),
        }
    }

    fn lire_curseur(&self) -> Option<String> {
        self.curseur.lock().map(|c| c.clone()).unwrap_or(None)
    }

    fn ecrire_curseur(&self, valeur: Option<String>) {
        if let Ok(mut c) = self.curseur.lock() {
            *c = valeur;
        }
    }

    /// Un passage : réconciliation bornée depuis le curseur, puis ramassage.
    pub async fn passage(&self) -> Result<BilanReconciliation, DomainError> {
        let mut bilan = BilanReconciliation::default();
        let mut curseur = self.lire_curseur();
        for _ in 0..self.lots_max {
            let lot = match self
                .depot
                .reconcilier_lot(curseur.as_deref(), self.taille_lot)
                .await
            {
                Ok(lot) => lot,
                Err(e) => {
                    // Curseur conservé : reprise au même endroit.
                    self.ecrire_curseur(curseur);
                    return Err(e);
                }
            };
            bilan.examines += lot.examines;
            bilan.corriges += lot.corriges;
            // Lot incomplet : fin du balayage (les blobs verrouillés, donc
            // sautés, sont repris au balayage suivant).
            let complet = i64::try_from(lot.examines).unwrap_or(i64::MAX) >= self.taille_lot;
            match lot.dernier {
                Some(dernier) if complet => curseur = Some(dernier),
                _ => {
                    curseur = None;
                    bilan.balayage_termine = true;
                    break;
                }
            }
        }
        self.ecrire_curseur(curseur);
        let (blobs, octets) = self.depot.ramasser_orphelins().await?;
        bilan.blobs_supprimes = blobs;
        bilan.octets_liberes = octets;
        Ok(bilan)
    }
}

/// Journalise le bilan d'un passage.
pub fn journaliser_bilan(b: &BilanReconciliation) {
    if b.corriges > 0 {
        tracing::warn!(
            "Réconciliation des références : {} compteur(s) corrigé(s) sur {} blobs examinés",
            b.corriges,
            b.examines
        );
    } else {
        tracing::debug!(
            "Réconciliation des références : {} blobs examinés, aucun écart",
            b.examines
        );
    }
    if b.blobs_supprimes > 0 {
        tracing::info!(
            "Ramasse-miettes : {} blob(s) sans référence supprimé(s) ({} octets)",
            b.blobs_supprimes,
            b.octets_liberes
        );
    }
}

/// Lance la réconciliation supervisée (passage immédiat puis toutes les
/// `periode`, relance journalisée en cas d'arrêt).
pub fn superviser_reconciliation<D: DepotReferences>(
    service: Arc<ReconciliationReferencesService<D>>,
    periode: Duration,
    delai_relance: Duration,
) -> tokio::task::JoinHandle<()> {
    superviser_tache_periodique(
        "Réconciliation des références",
        periode,
        delai_relance,
        move || {
            let service = service.clone();
            async move {
                match service.passage().await {
                    Ok(b) => journaliser_bilan(&b),
                    Err(e) => tracing::warn!(
                        "Réconciliation des références interrompue (reprise au prochain passage) : {e}"
                    ),
                }
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Dépôt mémoire : blobs (hachage → ref_count) et lignes de fichiers
    /// (hachage référencé), même sémantique que PostgreSQL.
    #[derive(Default)]
    struct DepotMemoire {
        blobs: Mutex<BTreeMap<String, i64>>,
        fichiers: Mutex<Vec<String>>,
        lots: Mutex<u32>,
        panne: Mutex<bool>,
    }

    impl DepotMemoire {
        fn blob(&self, hash: &str, refs: i64, fichiers: usize) {
            self.blobs.lock().unwrap().insert(hash.to_string(), refs);
            for _ in 0..fichiers {
                self.fichiers.lock().unwrap().push(hash.to_string());
            }
        }
        fn refs(&self, hash: &str) -> Option<i64> {
            self.blobs.lock().unwrap().get(hash).copied()
        }
    }

    impl DepotReferences for DepotMemoire {
        async fn reconcilier_lot(
            &self,
            apres: Option<&str>,
            limite: i64,
        ) -> Result<LotReconcilie, DomainError> {
            if *self.panne.lock().unwrap() {
                return Err(DomainError::internal_error("Test", "base indisponible"));
            }
            *self.lots.lock().unwrap() += 1;
            let fichiers = self.fichiers.lock().unwrap().clone();
            let mut blobs = self.blobs.lock().unwrap();
            let lot: Vec<String> = blobs
                .keys()
                .filter(|h| apres.is_none_or(|a| h.as_str() > a))
                .take(limite as usize)
                .cloned()
                .collect();
            let mut corriges = 0;
            for h in &lot {
                let n = fichiers.iter().filter(|f| *f == h).count() as i64;
                if let Some(r) = blobs.get_mut(h)
                    && *r != n
                {
                    *r = n;
                    corriges += 1;
                }
            }
            Ok(LotReconcilie {
                examines: lot.len() as u64,
                corriges,
                dernier: lot.last().cloned(),
            })
        }

        async fn ramasser_orphelins(&self) -> Result<(u64, u64), DomainError> {
            let mut blobs = self.blobs.lock().unwrap();
            let avant = blobs.len();
            blobs.retain(|_, r| *r > 0);
            Ok(((avant - blobs.len()) as u64, 0))
        }
    }

    #[tokio::test]
    async fn fuites_et_deficits_corriges_puis_orphelins_ramasses() {
        let depot = Arc::new(DepotMemoire::default());
        depot.blob("a1", 3, 1); // fuite (arrêt brutal après compensation)
        depot.blob("b2", 1, 2); // déficit : deux fichiers, un compté
        depot.blob("c3", 2, 2); // juste
        depot.blob("d4", 1, 0); // référence sans fichier (dépôt direct)
        let svc = ReconciliationReferencesService::new(depot.clone(), 100, 10);

        let bilan = svc.passage().await.unwrap();
        assert_eq!(bilan.examines, 4);
        assert_eq!(bilan.corriges, 3);
        assert!(bilan.balayage_termine);
        assert_eq!(bilan.blobs_supprimes, 1);
        assert_eq!(depot.refs("a1"), Some(1));
        assert_eq!(depot.refs("b2"), Some(2), "jamais sous le nombre réel");
        assert_eq!(depot.refs("c3"), Some(2));
        assert_eq!(depot.refs("d4"), None, "blob sans fichier ramassé");

        // Second passage : plus aucun écart.
        let bilan = svc.passage().await.unwrap();
        assert_eq!((bilan.corriges, bilan.blobs_supprimes), (0, 0));
    }

    #[tokio::test]
    async fn passage_borne_et_reprise_au_curseur() {
        let depot = Arc::new(DepotMemoire::default());
        for i in 0..10 {
            depot.blob(&format!("h{i:02}"), 5, 1);
        }
        // 2 lots de 3 blobs au plus par passage.
        let svc = ReconciliationReferencesService::new(depot.clone(), 3, 2);

        let b1 = svc.passage().await.unwrap();
        assert_eq!(
            (b1.examines, b1.corriges, b1.balayage_termine),
            (6, 6, false)
        );
        assert_eq!(*depot.lots.lock().unwrap(), 2, "borné à lots_max");
        assert_eq!(depot.refs("h06"), Some(5), "pas encore atteint");

        let b2 = svc.passage().await.unwrap();
        assert_eq!(
            (b2.examines, b2.corriges, b2.balayage_termine),
            (4, 4, true)
        );
        assert!((0..10).all(|i| depot.refs(&format!("h{i:02}")) == Some(1)));

        // Balayage terminé : le suivant repart du début.
        let b3 = svc.passage().await.unwrap();
        assert_eq!((b3.examines, b3.corriges), (6, 0));
    }

    #[tokio::test]
    async fn erreur_conserve_le_curseur() {
        let depot = Arc::new(DepotMemoire::default());
        for i in 0..6 {
            depot.blob(&format!("h{i}"), 2, 1);
        }
        let svc = ReconciliationReferencesService::new(depot.clone(), 2, 1);
        svc.passage().await.unwrap();
        *depot.panne.lock().unwrap() = true;
        assert!(svc.passage().await.is_err());
        *depot.panne.lock().unwrap() = false;
        let b = svc.passage().await.unwrap();
        assert_eq!(b.corriges, 2, "reprise après h1, pas au début");
        assert_eq!(depot.refs("h2"), Some(1));
        assert_eq!(depot.refs("h4"), Some(2));
    }
}
