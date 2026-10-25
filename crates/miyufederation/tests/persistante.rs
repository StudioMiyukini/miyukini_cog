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
use miyufederation::{
    CompteurFichier, CompteurMemoire, ErreurEtat, ErreurTransition, FederationPersistante, Fraicheur, Statut,
    StockageFederal, SupportMemoire,
};

/// Dossier temporaire supprimé à la fin du test.
struct Temp(PathBuf);

impl Temp {
    fn nouveau(nom: &str) -> Self {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        Self(std::env::temp_dir().join(format!("miyufederation-persistante-{nom}-{}-{n}", std::process::id())))
    }

    /// Rouvre le stockage et son compteur de fraîcheur (même dossier ; les
    /// verrous de l'instance précédente doivent être relâchés).
    fn ouvrir(&self) -> (StockageFederal, CompteurFichier) {
        (StockageFederal::ouvrir(&self.0, &SECRET).unwrap(), CompteurFichier::ouvrir(&self.0, &SECRET).unwrap())
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
                    let c = CompteurMemoire::new();
                    scenarios::$nom(async || (m.clone(), c.clone())).await;
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
    (m, c): (&SupportMemoire, &CompteurMemoire),
    secret: &[u8; 32],
    reg: &scenarios::Registre,
    t: u64,
) -> Result<FederationPersistante<SupportMemoire>, ErreurEtat> {
    FederationPersistante::charger(m.clone(), c.clone(), secret, scenarios::parametres(), None, t, reg, true)
        .await
        .map(|r| r.0)
}

/// Un octet altéré n'importe où dans l'état stocké ⇒ refus au rechargement,
/// jamais de vue (donc jamais de promotion) ; état supprimé alors
/// qu'attendu ⇒ refus ; autre secret local ⇒ refus.
#[tokio::test]
async fn etat_altere_ou_supprime_refuse() {
    let reg = registre();
    let m = SupportMemoire::new();
    let c = CompteurMemoire::new();
    let f = fondateurs(4);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier((m.clone(), c.clone()), &reg).await;
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
        assert!(charger_memoire((&m, &c), &SECRET, &reg, T0 + 61).await.is_err(), "octet {i} altéré accepté");
    }
    m.remplacer(Some(bon[..bon.len() - 1].to_vec()));
    assert!(charger_memoire((&m, &c), &SECRET, &reg, T0 + 61).await.is_err());
    m.remplacer(None);
    assert!(matches!(charger_memoire((&m, &c), &SECRET, &reg, T0 + 61).await, Err(ErreurEtat::EtatAbsent)));
    m.remplacer(Some(bon));
    assert!(matches!(charger_memoire((&m, &c), &[1; 32], &reg, T0 + 61).await, Err(ErreurEtat::Altere)));
    let p = recharger((m.clone(), c.clone()), &reg, T0 + 61).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite));
}

/// Panne d'écriture : transition refusée, vue en mémoire = dernier état
/// écrit, instance hors service ; le rechargement rétablit le service.
#[tokio::test]
async fn panne_d_ecriture_fail_closed() {
    let reg = registre();
    let m = SupportMemoire::new();
    let c = CompteurMemoire::new();
    let f = fondateurs(3);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier((m.clone(), c.clone()), &reg).await;
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
    let mut p = recharger((m.clone(), c.clone()), &reg, T0 + 3).await;
    assert!(p.en_service());
    assert_eq!(p.federation().statut(&a.cog), None);
    p.recevoir_candidature(&reg, &a.candidature, T0 + 3).await.unwrap();
    // Opération refusée sans effet sur la vue : aucune écriture.
    let n = m.ecritures();
    assert!(p.recevoir_confirmation(&reg, b"pas un sceau", T0 + 3).await.is_err());
    assert_eq!(m.ecritures(), n);
    drop(p);
    let p = recharger((m.clone(), c.clone()), &reg, T0 + 4).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Candidat));
}

/// Registre local qui ne prolonge plus l'ancre écrite : transition refusée, hors service.
#[tokio::test]
async fn registre_remplace_en_cours_refuse() {
    let mut reg = registre();
    let m = SupportMemoire::new();
    let mut p = premier((m.clone(), CompteurMemoire::new()), &reg).await;
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
    let c = CompteurMemoire::new();
    let p = premier((m.clone(), c.clone()), &reg).await;
    assert!(m.octets().is_some());
    assert_eq!(c.valeur().map(|f| f.generation), Some(1));
    drop(p);
    let (_, bilan) = scenarios::charger_avec((m.clone(), c.clone()), T0, &reg, true).await.unwrap();
    assert!(!bilan.premier_usage);
    // État supprimé, compteur présent : absence détectée même si l'appelant ne l'attendait pas.
    m.remplacer(None);
    assert!(matches!(scenarios::charger_avec((m.clone(), c.clone()), T0, &reg, false).await, Err(ErreurEtat::EtatAbsent)));
}


/// Retour arrière d'un état authentique (restauré dans le support) : refusé
/// par le compteur de fraîcheur ; même génération d'empreinte différente :
/// refusée ; compteur en retard d'une génération (interruption) : accepté.
#[tokio::test]
async fn retour_arriere_refuse_par_le_compteur() {
    let reg = registre();
    let m = SupportMemoire::new();
    let c = CompteurMemoire::new();
    let f = fondateurs(4);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier((m.clone(), c.clone()), &reg).await;
    for x in &f {
        p.ajouter_fondateur(&reg, &x.candidature, T0).await.unwrap();
    }
    p.recevoir_candidature(&reg, &a.candidature, T0).await.unwrap();
    let candidat = (m.octets().unwrap(), c.valeur().unwrap());
    for x in &f[..2] {
        p.recevoir_confirmation(&reg, &x.confirmer(&a, T0 + 60), T0 + 60).await.unwrap();
    }
    p.actualiser(&reg, T0 + 60).await.unwrap();
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite));
    let g = p.generation();
    assert_eq!(c.valeur().unwrap().generation, g);
    assert!(g > candidat.1.generation);
    drop(p);
    let invite = m.octets().unwrap();

    // Ancien état authentique restauré : refus (le pair ne redevient pas candidat).
    m.remplacer(Some(candidat.0.clone()));
    let r = charger_memoire((&m, &c), &SECRET, &reg, T0 + 61).await;
    assert!(matches!(r, Err(ErreurEtat::EtatPerime { etat, compteur }) if etat == candidat.1.generation && compteur == g), "{r:?}");

    // Même génération, contenu différent : autre branche écrite depuis l'état candidat.
    let c2 = CompteurMemoire::new();
    c2.remplacer(Some(candidat.1));
    let mut q = recharger((m.clone(), c2.clone()), &reg, T0 + 61).await;
    for k in 0..g {
        if q.generation() == g {
            break;
        }
        q.actualiser(&reg, T0 + 61 + k).await.unwrap();
    }
    assert_eq!(q.generation(), g);
    drop(q);
    let branche = m.octets().unwrap();
    assert_ne!(branche, invite);
    m.remplacer(Some(invite.clone()));
    let r = charger_memoire((&m, &c2), &SECRET, &reg, T0 + 70).await;
    assert!(matches!(r, Err(ErreurEtat::EtatDivergent(x)) if x == g), "{r:?}");
    m.remplacer(Some(branche));
    assert!(charger_memoire((&m, &c), &SECRET, &reg, T0 + 70).await.is_err(), "branche refusée par le compteur d'origine");

    // Compteur en retard d'une génération (état écrit, compteur interrompu) : accepté, compteur avancé.
    m.remplacer(Some(invite.clone()));
    let mut retard = c.valeur().unwrap();
    retard.generation -= 1;
    retard.empreinte = [0; 32];
    c.remplacer(Some(retard));
    let p = charger_memoire((&m, &c), &SECRET, &reg, T0 + 70).await.unwrap();
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite));
    assert_eq!(c.valeur().unwrap(), Fraicheur::de_etat(g, &invite));
    drop(p);
    // Deux générations de retard : refus.
    let mut loin = c.valeur().unwrap();
    loin.generation -= 2;
    c.remplacer(Some(loin));
    assert!(matches!(charger_memoire((&m, &c), &SECRET, &reg, T0 + 70).await, Err(ErreurEtat::CompteurIncoherent(_))));
}

/// Compteur local perdu : rechargement refusé (pas de reprise silencieuse)
/// jusqu'au réamorçage explicite ; réamorcer est refusé tant que le
/// compteur existe.
#[tokio::test]
async fn compteur_absent_refus_puis_reamorcage_explicite() {
    let reg = registre();
    let m = SupportMemoire::new();
    let c = CompteurMemoire::new();
    let f = fondateurs(2);
    let mut p = premier((m.clone(), c.clone()), &reg).await;
    for x in &f {
        p.ajouter_fondateur(&reg, &x.candidature, T0).await.unwrap();
    }
    let g = p.generation();
    drop(p);
    let reamorcer = async |c: &CompteurMemoire| {
        FederationPersistante::reamorcer_compteur(&m, c, &SECRET, scenarios::parametres(), None, T0 + 1, &reg).await
    };
    assert!(matches!(reamorcer(&c).await, Err(ErreurEtat::CompteurIncoherent(_))));
    c.remplacer(None);
    assert!(matches!(charger_memoire((&m, &c), &SECRET, &reg, T0 + 1).await, Err(ErreurEtat::CompteurAbsent)));
    // Premier usage simulé (état non attendu) : toujours refusé, l'état existe.
    assert!(matches!(
        scenarios::charger_avec((m.clone(), c.clone()), T0 + 1, &reg, false).await,
        Err(ErreurEtat::CompteurAbsent)
    ));
    assert_eq!(reamorcer(&c).await.unwrap(), g);
    let p = recharger((m.clone(), c.clone()), &reg, T0 + 2).await;
    assert_eq!(p.federation().membres().len(), 2);
}

/// Support fichier : compteur dans le dossier de l'état, plusieurs
/// redémarrages, fichier du compteur supprimé ⇒ refus explicite.
#[tokio::test]
async fn fichier_redemarrages_et_compteur_supprime() {
    let d = Temp::nouveau("compteur");
    let reg = registre();
    let f = fondateurs(3);
    let mut p = premier(d.ouvrir(), &reg).await;
    for (i, x) in f.iter().enumerate() {
        let t = T0 + i as u64;
        p.ajouter_fondateur(&reg, &x.candidature, t).await.unwrap();
        drop(p);
        p = recharger(d.ouvrir(), &reg, t).await;
        assert_eq!(p.federation().membres().len(), i + 1);
        assert_eq!(p.generation(), 2 + i as u64);
    }
    let chemin = p.support().chemin().with_file_name(miyufederation::FICHIER_COMPTEUR);
    drop(p);
    std::fs::remove_file(chemin).unwrap();
    let (s, c) = d.ouvrir();
    let r = FederationPersistante::charger(s, c, &SECRET, scenarios::parametres(), None, T0 + 9, &reg, true).await;
    assert!(matches!(r, Err(ErreurEtat::CompteurAbsent)), "{:?}", r.err());
}

/// Les transitions sont des futurs `Send` (utilisables depuis `tokio::spawn`).
#[test]
fn transitions_send() {
    fn send<T: Send>(_: &T) {}
    let reg = registre();
    let mut p = futures_pret(FederationPersistante::charger(
        SupportMemoire::new(),
        CompteurMemoire::new(),
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
