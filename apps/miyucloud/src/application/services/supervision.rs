//! @id     miyucloud_application_services_supervision
//! @do     superviser_taches_periodiques
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — supervision des tâches de fond périodiques : passage régulier, relance journalisée si la tâche s'arrête ou panique.

//! Supervision des tâches de fond périodiques.
//!
//! Une tâche lancée par `tokio::spawn` dont le `JoinHandle` est jeté meurt
//! en silence si elle panique. [`superviser_tache_periodique`] l'exécute
//! dans une sous-tâche surveillée : tout arrêt (panique comprise) est
//! journalisé en erreur et la tâche relancée après un délai.

use std::future::Future;
use std::time::Duration;

/// Exécute `passage` immédiatement puis toutes les `periode`, dans une
/// sous-tâche relancée après `delai_relance` si elle s'arrête (panique
/// comprise). Le handle rendu ne se termine qu'à l'annulation (arrêt du
/// moteur d'exécution ou `abort`).
pub fn superviser_tache_periodique<F, Fut>(
    nom: &'static str,
    periode: Duration,
    delai_relance: Duration,
    passage: F,
) -> tokio::task::JoinHandle<()>
where
    F: Fn() -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        let mut relances: u64 = 0;
        loop {
            let passage = passage.clone();
            let travail = tokio::spawn(async move {
                let mut intervalle = tokio::time::interval(periode);
                intervalle.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
                loop {
                    intervalle.tick().await;
                    passage().await;
                }
            });
            match travail.await {
                Ok(()) => tracing::error!("{nom} : tâche terminée de façon inattendue"),
                Err(e) if e.is_panic() => tracing::error!("{nom} : la tâche a paniqué ({e})"),
                Err(e) => {
                    tracing::error!("{nom} : tâche annulée ({e}), supervision arrêtée");
                    return;
                }
            }
            relances += 1;
            tracing::error!(
                "{nom} : relance n° {relances} dans {} ms",
                delai_relance.as_millis()
            );
            tokio::time::sleep(delai_relance).await;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn relance_apres_panique_puis_continue() {
        let appels = Arc::new(AtomicU32::new(0));
        let a = appels.clone();
        let handle = superviser_tache_periodique(
            "test",
            Duration::from_millis(5),
            Duration::from_millis(5),
            move || {
                let a = a.clone();
                async move {
                    if a.fetch_add(1, Ordering::SeqCst) < 2 {
                        panic!("panique simulée");
                    }
                }
            },
        );
        let debut = std::time::Instant::now();
        while appels.load(Ordering::SeqCst) < 6 {
            assert!(debut.elapsed() < Duration::from_secs(10), "pas de relance");
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        assert!(!handle.is_finished());
        handle.abort();
    }
}
