//! @id     miyuhybride_tests_quotas
//! @do     tester_quotas_concurrents
//! @role   test
//! @layer  toolkit
//! @human  miyuhybride — quotas du contrat sous accès concurrents : plusieurs tâches ne dépassent jamais disque, sessions ni tâches ; tout est rendu à la fin.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use miyuhybride::{DbSecondaire, ErreurHybride, QuotasService, Ressource, Ressources, SessionsClients};

const TACHES: usize = 16;
const ESSAIS: usize = 500;

fn quotas(disque_mio: u32, sessions_max: u32, taches_max: u32) -> QuotasService {
    QuotasService::depuis(&Ressources { cpu_pourcent: 10, memoire_mio: 64, disque_mio, taches_max, sessions_max })
}

#[test]
fn sessions_et_taches_concurrentes_sans_depassement() {
    let q = quotas(1, 7, 3);
    let barriere = Arc::new(Barrier::new(TACHES));
    let pic_sessions = Arc::new(AtomicU64::new(0));
    let pic_taches = Arc::new(AtomicU64::new(0));
    let fils: Vec<_> = (0..TACHES)
        .map(|_| {
            let (q, barriere) = (q.clone(), Arc::clone(&barriere));
            let (ps, pt) = (Arc::clone(&pic_sessions), Arc::clone(&pic_taches));
            thread::spawn(move || {
                barriere.wait();
                let mut refus = 0usize;
                for _ in 0..ESSAIS {
                    match q.ouvrir_session() {
                        Ok(jeton) => {
                            let u = q.sessions().utilise();
                            assert!(u <= 7, "sessions dépassées : {u}");
                            ps.fetch_max(u, Ordering::Relaxed);
                            if let Ok(t) = q.lancer_tache() {
                                pt.fetch_max(q.taches().utilise(), Ordering::Relaxed);
                                drop(t);
                            }
                            drop(jeton);
                        }
                        Err(e) => {
                            assert_eq!(e, ErreurHybride::QuotaDepasse(Ressource::Sessions));
                            refus += 1;
                        }
                    }
                }
                refus
            })
        })
        .collect();
    for f in fils {
        f.join().unwrap();
    }
    assert!(pic_sessions.load(Ordering::Relaxed) <= 7);
    assert!(pic_taches.load(Ordering::Relaxed) <= 3);
    assert_eq!(q.sessions().utilise(), 0, "toutes les sessions rendues");
    assert_eq!(q.taches().utilise(), 0, "toutes les tâches rendues");
}

#[test]
fn ouvertures_simultanees_exactement_a_la_limite() {
    // Chaque tâche garde ses sessions : le total accordé vaut exactement la limite.
    let q = quotas(1, 10, 1);
    let barriere = Arc::new(Barrier::new(TACHES));
    let fils: Vec<_> = (0..TACHES)
        .map(|i| {
            let (q, barriere) = (q.clone(), Arc::clone(&barriere));
            thread::spawn(move || {
                let mut s = SessionsClients::new(q, 60);
                barriere.wait();
                let accordees = (0..4u32).filter(|k| s.toucher(&(i as u32 * 100 + k), 1_000).is_ok()).count();
                (accordees, s)
            })
        })
        .collect();
    let resultats: Vec<_> = fils.into_iter().map(|f| f.join().unwrap()).collect();
    let total: usize = resultats.iter().map(|(n, _)| n).sum();
    assert_eq!(total, 10);
    assert_eq!(q.sessions().utilise(), 10);
    // Inactivité : à l'expiration, les sessions sont libérées.
    let mut sessions: Vec<_> = resultats.into_iter().map(|(_, s)| s).collect();
    for s in &mut sessions {
        s.expirer(1_000 + 61);
    }
    assert_eq!(q.sessions().utilise(), 0, "sessions expirées libérées");
}

#[test]
fn ecritures_concurrentes_sous_quota_disque() {
    // 16 bases (une par tâche) de 1 Mio chacune sous un disque de service de 2 Mio.
    let q = quotas(2, 1, 1);
    let limite = q.disque().limite();
    let barriere = Arc::new(Barrier::new(TACHES));
    let fils: Vec<_> = (0..TACHES)
        .map(|_| {
            let (q, barriere) = (q.clone(), Arc::clone(&barriere));
            thread::spawn(move || {
                let mut db = DbSecondaire::avec_quota(1024 * 1024, &q).unwrap();
                barriere.wait();
                for i in 0..ESSAIS as u32 {
                    let cle = (i % 64).to_be_bytes();
                    if let Err(e) = db.ecrire("t", &cle, vec![0; 4096]) {
                        assert!(
                            matches!(e, ErreurHybride::QuotaDepasse(Ressource::Disque) | ErreurHybride::BaseSecondairePleine),
                            "refus inattendu : {e}"
                        );
                    }
                    assert!(q.disque().utilise() <= q.disque().limite());
                    if i % 7 == 0 {
                        db.retirer("t", &cle);
                    }
                }
                db
            })
        })
        .collect();
    let bases: Vec<DbSecondaire> = fils.into_iter().map(|f| f.join().unwrap()).collect();
    let somme: u64 = bases.iter().map(|b| b.utilise() as u64).sum();
    assert_eq!(q.disque().utilise(), somme, "compteur = octets réellement écrits");
    assert!(somme <= limite);
    drop(bases);
    assert_eq!(q.disque().utilise(), 0, "destruction des façades : disque rendu");
}
