//! @id     miyufederation_test_persistante
//! @do     verifier_vue_persistante_par_transition
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — tests de la vue persistante sur supports mémoire et fichier : redémarrage après chaque transition, altération refusée, panne d'écriture fail-closed.

//! [`FederationPersistante`] : chaque transition écrite atomiquement,
//! rechargement revérifié (statuts recalculés, jamais lus), état altéré ⇒
//! refus, panne d'écriture ⇒ vue inchangée et hors service.

#![allow(clippy::many_single_char_names)]

mod commun;

use std::path::PathBuf;

use commun::scenarios::{self, fondateurs, premier, recharger, registre, SECRET};
use commun::{CogTest, T0};
use miyufederation::{ErreurEtat, ErreurTransition, FederationPersistante, Statut, StockageFederal, SupportMemoire};

/// Dossier temporaire supprimé à la fin du test.
struct Temp(PathBuf);

impl Temp {
    fn nouveau(nom: &str) -> Self {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        Self(std::env::temp_dir().join(format!("miyufederation-persistante-{nom}-{}-{n}", std::process::id())))
    }

    /// Rouvre le stockage (le verrou de l'instance précédente doit être relâché).
    fn ouvrir(&self) -> StockageFederal {
        StockageFederal::ouvrir(&self.0, &SECRET).unwrap()
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

macro_rules! sur_les_supports {
    ($($nom:ident),* $(,)?) => {
        mod memoire {
            use super::*;
            $(
                #[tokio::test]
                async fn $nom() {
                    let m = SupportMemoire::new();
                    scenarios::$nom(async || m.clone()).await;
                }
            )*
        }
        mod fichier {
            use super::*;
            $(
                #[tokio::test]
                async fn $nom() {
                    let d = Temp::nouveau(stringify!($nom));
                    scenarios::$nom(async || d.ouvrir()).await;
                }
            )*
        }
    };
}

sur_les_supports!(admission, sanction, contestation, vue_note_carnet, horloge_en_recul);

async fn charger_memoire(
    m: &SupportMemoire,
    secret: &[u8; 32],
    reg: &scenarios::Registre,
    t: u64,
) -> Result<FederationPersistante<SupportMemoire>, ErreurEtat> {
    FederationPersistante::charger(m.clone(), secret, scenarios::parametres(), None, t, reg, true).await.map(|r| r.0)
}

/// Un octet altéré n'importe où dans l'état stocké ⇒ refus au rechargement,
/// jamais de vue (donc jamais de promotion) ; état supprimé alors
/// qu'attendu ⇒ refus ; autre secret local ⇒ refus.
#[tokio::test]
async fn etat_altere_ou_supprime_refuse() {
    let reg = registre();
    let m = SupportMemoire::new();
    let f = fondateurs(4);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier(m.clone(), &reg).await;
    for c in &f {
        p.ajouter_fondateur(&reg, &c.candidature, T0).await.unwrap();
    }
    p.recevoir_candidature(&reg, &a.candidature, T0).await.unwrap();
    for c in &f[..2] {
        p.recevoir_confirmation(&reg, &c.confirmer(&a, T0 + 60), T0 + 60).await.unwrap();
    }
    p.actualiser(&reg, T0 + 60).await.unwrap();
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite));
    drop(p);
    let bon = m.octets().unwrap();
    for i in (0..bon.len()).step_by(11).chain([bon.len() - 1]) {
        let mut x = bon.clone();
        x[i] ^= 0x20;
        m.remplacer(Some(x));
        assert!(charger_memoire(&m, &SECRET, &reg, T0 + 61).await.is_err(), "octet {i} altéré accepté");
    }
    m.remplacer(Some(bon[..bon.len() - 1].to_vec()));
    assert!(charger_memoire(&m, &SECRET, &reg, T0 + 61).await.is_err());
    m.remplacer(None);
    assert!(matches!(charger_memoire(&m, &SECRET, &reg, T0 + 61).await, Err(ErreurEtat::EtatAbsent)));
    m.remplacer(Some(bon));
    assert!(matches!(charger_memoire(&m, &[1; 32], &reg, T0 + 61).await, Err(ErreurEtat::Altere)));
    let p = recharger(m.clone(), &reg, T0 + 61).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite));
}

/// Panne d'écriture : transition refusée, vue en mémoire = dernier état
/// écrit, instance hors service ; le rechargement rétablit le service.
#[tokio::test]
async fn panne_d_ecriture_fail_closed() {
    let reg = registre();
    let m = SupportMemoire::new();
    let f = fondateurs(3);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier(m.clone(), &reg).await;
    for c in &f {
        p.ajouter_fondateur(&reg, &c.candidature, T0).await.unwrap();
    }
    let ecritures = m.ecritures();
    m.echouer_ecritures(1);
    let r = p.recevoir_candidature(&reg, &a.candidature, T0 + 1).await;
    assert!(matches!(r, Err(ErreurTransition::Etat(ErreurEtat::Support(_)))), "{r:?}");
    assert!(!p.en_service());
    assert_eq!(p.federation().statut(&a.cog), None, "vue en mémoire = dernier état écrit");
    assert!(matches!(p.actualiser(&reg, T0 + 2).await, Err(ErreurTransition::HorsService)));
    assert_eq!(m.ecritures(), ecritures);
    drop(p);
    let mut p = recharger(m.clone(), &reg, T0 + 3).await;
    assert!(p.en_service());
    assert_eq!(p.federation().statut(&a.cog), None);
    p.recevoir_candidature(&reg, &a.candidature, T0 + 3).await.unwrap();
    // Opération refusée sans effet sur la vue : aucune écriture.
    let n = m.ecritures();
    assert!(p.recevoir_confirmation(&reg, b"pas un sceau", T0 + 3).await.is_err());
    assert_eq!(m.ecritures(), n);
    drop(p);
    let p = recharger(m.clone(), &reg, T0 + 4).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Candidat));
}

/// Registre local qui ne prolonge plus l'ancre écrite : transition refusée, hors service.
#[tokio::test]
async fn registre_remplace_en_cours_refuse() {
    let mut reg = registre();
    let m = SupportMemoire::new();
    let mut p = premier(m.clone(), &reg).await;
    let f = fondateurs(8);
    for c in &f {
        p.ajouter_fondateur(&reg, &c.candidature, T0).await.unwrap();
    }
    let (acc, preuve) = scenarios::accusation(p.federation(), &f, &f[7], T0 + 1);
    let e = p.recevoir_accusation(&reg, &acc, &preuve, T0 + 1).await.unwrap().echeance();
    let rev = p.actualiser(&reg, e).await.unwrap();
    reg.ajouter(&rev[0].sceller_entree(&scenarios::journal_local()).unwrap()).unwrap();
    p.actualiser(&reg, e + 1).await.unwrap();
    assert!(matches!(
        p.actualiser(&registre(), e + 2).await,
        Err(ErreurTransition::Etat(ErreurEtat::RegistreIncoherent))
    ));
    assert!(!p.en_service());
}

/// Premier usage : la vue vide est écrite, l'absence devient ensuite une anomalie.
#[tokio::test]
async fn premier_usage_ecrit_l_etat() {
    let reg = registre();
    let m = SupportMemoire::new();
    let p = premier(m.clone(), &reg).await;
    assert!(m.octets().is_some());
    drop(p);
    let (_, bilan) =
        FederationPersistante::charger(m.clone(), &SECRET, scenarios::parametres(), None, T0, &reg, true).await.unwrap();
    assert!(!bilan.premier_usage);
}

/// Les transitions sont des futurs `Send` (utilisables depuis `tokio::spawn`).
#[test]
fn transitions_send() {
    fn send<T: Send>(_: &T) {}
    let reg = registre();
    let mut p = futures_pret(FederationPersistante::charger(
        SupportMemoire::new(),
        &SECRET,
        scenarios::parametres(),
        None,
        T0,
        &reg,
        false,
    ))
    .unwrap()
    .0;
    let f = p.actualiser(&reg, T0);
    send(&f);
    drop(f);
}

/// Exécute un futur qui ne se suspend pas (support mémoire).
fn futures_pret<F: std::future::Future>(f: F) -> F::Output {
    let mut f = std::pin::pin!(f);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}
