//! @id     miyufederation_tests_scenarios_redemarrage
//! @do     verifier_redemarrage_par_transition
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — scénarios de redémarrage communs à tous les supports (mémoire, fichier, KindMother) : admission, sanction, recours, carnet, panne d'écriture.

//! Chaque scénario rejoue une transition de statut, **redémarre** (le support
//! est rouvert, la vue rechargée et revérifiée) et contrôle que le statut,
//! l'échéance des 72 h et les files survivent à l'identique de la vue d'un
//! « jumeau » qui n'a jamais redémarré.

// Vide / non vide : le message d'échec du test suffit.
#![allow(clippy::many_single_char_names, clippy::assert_is_empty)]

use miyufederation::{
    BilanReprise, CleEtatFederal, CompteurFraicheur, ErreurEtat, ErreurFederation, ErreurTransition, Federation,
    FederationPersistante, Observation, Parametres, Preuve, Statut, SupportEtat,
};
use miyuregistre::{Journal, StockageMemoire};
use miyusceau::{cosigner, CleSignature, TeteRegistre};

use super::{CogTest, DIFFICULTE_TEST, JOUR, T0};

/// Secret local du COG de test.
pub const SECRET: [u8; 32] = [0x5a; 32];

/// Registre local (journal Merkle) en mémoire.
pub type Registre = Journal<StockageMemoire>;

pub fn registre() -> Registre {
    Journal::ouvrir([0x11; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap()
}

pub fn parametres() -> Parametres {
    Parametres::default().avec_difficulte(DIFFICULTE_TEST).unwrap()
}

pub fn cle() -> CleEtatFederal {
    CleEtatFederal::deriver(&SECRET)
}

/// Clé de journal du COG local.
pub fn journal_local() -> CleSignature {
    CleSignature::depuis_graine(&[0xee; 32])
}

/// Charge la vue depuis un support et son compteur de fraîcheur.
pub async fn charger_avec<S: SupportEtat, K: CompteurFraicheur + 'static>(
    (support, compteur): (S, K),
    t: u64,
    reg: &Registre,
    etat_attendu: bool,
) -> Result<(FederationPersistante<S>, BilanReprise), ErreurEtat> {
    FederationPersistante::charger(support, compteur, &SECRET, parametres(), None, t, reg, etat_attendu).await
}

/// Recharge la vue (état attendu) : le « redémarrage ».
pub async fn recharger<S: SupportEtat, K: CompteurFraicheur + 'static>(
    ouvert: (S, K),
    reg: &Registre,
    t: u64,
) -> FederationPersistante<S> {
    charger_avec(ouvert, t, reg, true).await.unwrap().0
}

/// Premier démarrage : aucun état, vue vide écrite aussitôt.
pub async fn premier<S: SupportEtat, K: CompteurFraicheur + 'static>(
    ouvert: (S, K),
    reg: &Registre,
) -> FederationPersistante<S> {
    let (p, bilan) = charger_avec(ouvert, T0, reg, false).await.unwrap();
    assert!(bilan.premier_usage);
    p
}

pub fn sceau(v: &Federation) -> Vec<u8> {
    v.sceller_etat(&cle(), &miyufederation::AncreRegistre::de(&registre())).unwrap()
}

pub fn accusation(v: &Federation, f: &[CogTest], accuse: &CogTest, t: u64) -> (Vec<u8>, Preuve) {
    let preuve = Preuve::equivoque(accuse.tete(1, 5, [1; 32], t), accuse.tete(1, 5, [2; 32], t));
    let mut acc = v.accuser(&accuse.cog, &preuve, &f[0].identite, t).unwrap();
    for m in &f[1..6] {
        acc = v.contresigner_accusation(&acc, &preuve, &m.identite, t).unwrap();
    }
    (acc, preuve)
}

pub fn recours(v: &Federation, f: &[CogTest], accuse: &CogTest, t: u64) -> Vec<u8> {
    let mut r = v.ouvrir_recours(&accuse.cog, &f[0].identite, t).unwrap();
    for m in &f[1..6] {
        r = v.contresigner_recours(&r, &m.identite, t).unwrap();
    }
    r
}

pub fn fondateurs(n: u8) -> Vec<CogTest> {
    (1..=n).map(|g| CogTest::nouveau(g, T0)).collect()
}

async fn avec_fondateurs<S: SupportEtat>(p: &mut FederationPersistante<S>, reg: &Registre, f: &[CogTest]) {
    for c in f {
        p.ajouter_fondateur(reg, &c.candidature, T0).await.unwrap();
    }
}

/// candidat → invité → membre, un redémarrage après chaque transition et
/// pendant le passage à membre (arrêt de part et d'autre des 7 jours).
pub async fn admission<S: SupportEtat, K: CompteurFraicheur + 'static>(ouvrir: impl AsyncFn() -> (S, K)) {
    let reg = registre();
    let f = fondateurs(6);
    let a = CogTest::nouveau(30, T0);
    let mut p = premier(ouvrir().await, &reg).await;
    avec_fondateurs(&mut p, &reg, &f).await;
    p.recevoir_candidature(&reg, &a.candidature, T0).await.unwrap();
    drop(p);

    let mut p = recharger(ouvrir().await, &reg, T0 + 10).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Candidat));
    assert_eq!(p.federation().membres().len(), 6);
    let mut jumeau = p.federation().clone();

    // Invité : 4 confirmations de membres (≥ 2).
    let t = T0 + 60;
    for m in &f[..4] {
        let c = m.confirmer(&a, t);
        p.recevoir_confirmation(&reg, &c, t).await.unwrap();
        jumeau.recevoir_confirmation(&c, t).unwrap();
    }
    p.actualiser(&reg, t).await.unwrap();
    jumeau.actualiser(t).unwrap();
    drop(p);
    let mut p = recharger(ouvrir().await, &reg, t).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite));
    assert_eq!(sceau(p.federation()), sceau(&jumeau));

    // Six jours de confirmations quotidiennes, redémarrage chaque jour.
    for jour in 1..=6u64 {
        let t = T0 + jour * JOUR + 60;
        for m in &f[..4] {
            let c = m.confirmer(&a, t);
            p.recevoir_confirmation(&reg, &c, t).await.unwrap();
            jumeau.recevoir_confirmation(&c, t).unwrap();
        }
        p.actualiser(&reg, t).await.unwrap();
        jumeau.actualiser(t).unwrap();
        drop(p);
        p = recharger(ouvrir().await, &reg, t).await;
        assert_eq!(p.federation().statut(&a.cog), Some(Statut::Invite), "jour {jour}");
    }
    // Deux confirmations couvrant la fin des 7 jours, puis arrêt.
    let t = T0 + 6 * JOUR + 3_600;
    for m in &f[4..6] {
        let c = m.confirmer(&a, t);
        p.recevoir_confirmation(&reg, &c, t).await.unwrap();
        jumeau.recevoir_confirmation(&c, t).unwrap();
    }
    drop(p);
    // Reprise après le passage à membre (T0 + 7 j + 60 s) : membre acquis pendant l'arrêt.
    let reprise = T0 + 7 * JOUR + 1_000;
    let p = recharger(ouvrir().await, &reg, reprise).await;
    jumeau.actualiser(T0 + 7 * JOUR + 60).unwrap();
    jumeau.actualiser(reprise).unwrap();
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Membre));
    assert_eq!(sceau(p.federation()), sceau(&jumeau));
    drop(p);
    let p = recharger(ouvrir().await, &reg, reprise + 1).await;
    assert_eq!(p.federation().statut(&a.cog), Some(Statut::Membre));
    assert!(p.federation().membres().contains(&a.cog));
}

/// accusation → suspension → révocation à 72 h, redémarrages avant et
/// après l'échéance ; file à publier jusqu'à l'accusé d'inscription.
pub async fn sanction<S: SupportEtat, K: CompteurFraicheur + 'static>(ouvrir: impl AsyncFn() -> (S, K)) {
    let mut reg = registre();
    let f = fondateurs(8);
    let x = &f[7];
    let mut p = premier(ouvrir().await, &reg).await;
    avec_fondateurs(&mut p, &reg, &f).await;
    let t = T0 + 100;
    let (acc, preuve) = accusation(p.federation(), &f, x, t);
    let echeance = p.recevoir_accusation(&reg, &acc, &preuve, t).await.unwrap().echeance();
    assert_eq!(echeance, t + 72 * 3_600);
    drop(p);

    // Suspendu après redémarrage, même échéance.
    let p = recharger(ouvrir().await, &reg, t + 1).await;
    assert!(matches!(p.federation().statut(&x.cog), Some(Statut::Suspendu { depuis, .. }) if depuis == t));
    assert_eq!(p.federation().suspensions().map(miyufederation::SuspensionFederale::echeance).collect::<Vec<_>>(), vec![echeance]);
    drop(p);
    // Juste avant l'échéance : toujours suspendu, aucune révocation.
    let (p, bilan) =
        charger_avec(ouvrir().await, echeance - 1, &reg, true)
            .await
            .unwrap();
    assert!(bilan.revocations.is_empty());
    assert!(matches!(p.federation().statut(&x.cog), Some(Statut::Suspendu { .. })));
    drop(p);
    // Arrêt à cheval sur l'échéance : révocation datée de l'échéance, une fois.
    let (p, bilan) =
        charger_avec(ouvrir().await, echeance + JOUR, &reg, true)
            .await
            .unwrap();
    assert_eq!(bilan.revocations.len(), 1);
    assert_eq!(bilan.revocations[0].cog, x.cog);
    assert_eq!(bilan.revocations[0].revoque_a, echeance);
    assert_eq!(p.federation().statut(&x.cog), Some(Statut::Revoque));
    drop(p);
    // Non acquittée : rendue encore à chaque reprise.
    let (mut p, bilan) =
        charger_avec(ouvrir().await, echeance + JOUR + 1, &reg, true)
            .await
            .unwrap();
    assert_eq!(bilan.revocations.len(), 1);
    let rev = bilan.revocations[0].clone();
    // Accusé refusé sans inscription ; inscription puis accusé.
    assert!(matches!(p.acquitter(&rev.kid, 0, &reg).await, Err(ErreurTransition::Etat(ErreurEtat::Incoherent(_)))));
    let index = reg.ajouter(&rev.sceller_entree(&journal_local()).unwrap()).unwrap();
    assert!(p.acquitter(&rev.kid, index, &reg).await.unwrap());
    drop(p);
    let (p, bilan) =
        charger_avec(ouvrir().await, echeance + 2 * JOUR, &reg, true)
            .await
            .unwrap();
    assert!(bilan.revocations.is_empty());
    assert_eq!(bilan.reappliquees, 0);
    assert_eq!(p.federation().statut(&x.cog), Some(Statut::Revoque));
    assert!(matches!(
        p.federation().ouvrir_recours(&x.cog, &f[0].identite, echeance + 2 * JOUR),
        Err(ErreurFederation::Revoque)
    ));
    // Candidature de la racine révoquée : refus définitif, persisté.
    drop(p);
    let mut p = recharger(ouvrir().await, &reg, echeance + 3 * JOUR).await;
    assert!(matches!(
        p.recevoir_candidature(&reg, &CogTest::nouveau(8, echeance + 3 * JOUR).candidature, echeance + 3 * JOUR).await,
        Err(ErreurTransition::Federation(ErreurFederation::Revoque))
    ));
}

/// accusation → suspension → recours accepté → statut rétabli ; redémarrages
/// entre chaque étape ; l'accusation levée n'est jamais rejouable.
pub async fn contestation<S: SupportEtat, K: CompteurFraicheur + 'static>(ouvrir: impl AsyncFn() -> (S, K)) {
    let reg = registre();
    let f = fondateurs(8);
    let y = &f[6];
    let mut p = premier(ouvrir().await, &reg).await;
    avec_fondateurs(&mut p, &reg, &f).await;
    let t = T0 + 100;
    let (acc, preuve) = accusation(p.federation(), &f, y, t);
    let echeance = p.recevoir_accusation(&reg, &acc, &preuve, t).await.unwrap().echeance();
    drop(p);
    let mut p = recharger(ouvrir().await, &reg, t + 50).await;
    assert!(matches!(p.federation().statut(&y.cog), Some(Statut::Suspendu { .. })));
    let r = recours(p.federation(), &f, y, t + 100);
    p.recevoir_recours(&reg, &r, t + 100).await.unwrap();
    assert_eq!(p.federation().statut(&y.cog), Some(Statut::Membre));
    drop(p);

    let mut p = recharger(ouvrir().await, &reg, t + 200).await;
    assert_eq!(p.federation().statut(&y.cog), Some(Statut::Membre));
    assert!(matches!(
        p.recevoir_accusation(&reg, &acc, &preuve, t + 200).await,
        Err(ErreurTransition::Federation(ErreurFederation::AccusationLevee))
    ));
    assert!(matches!(
        p.recevoir_recours(&reg, &r, t + 200).await,
        Err(ErreurTransition::Federation(ErreurFederation::AucuneSuspension))
    ));
    drop(p);
    // Après l'échéance initiale : aucune révocation.
    let (p, bilan) =
        charger_avec(ouvrir().await, echeance + JOUR, &reg, true)
            .await
            .unwrap();
    assert!(bilan.revocations.is_empty());
    assert_eq!(p.federation().statut(&y.cog), Some(Statut::Membre));
}

/// Vue fédérale (tête témoignée), témoignage, note locale et carnet de
/// pairs survivent au redémarrage ; un témoignage contradictoire reste
/// refusé après redémarrage.
pub async fn vue_note_carnet<S: SupportEtat, K: CompteurFraicheur + 'static>(ouvrir: impl AsyncFn() -> (S, K)) {
    let reg = registre();
    let f = fondateurs(5);
    let mut p = premier(ouvrir().await, &reg).await;
    avec_fondateurs(&mut p, &reg, &f).await;
    let t = T0 + 70;
    let mut h = f[0].tete(1, 2, [5; 32], t);
    for m in &f[1..4] {
        h = cosigner::<TeteRegistre>(&h, &m.journal).unwrap();
    }
    p.recevoir_tete(&reg, &f[0].cog, &h, None, t).await.unwrap();
    p.temoigner(&reg, &f[1].cog, &f[1].tete(1, 3, [6; 32], t), None, &journal_local(), t).await.unwrap();
    p.observer(&reg, &f[2].cog, Observation::Indisponible, t).await.unwrap();
    assert!(p.noter_contact(&reg, &f[3].cog, "reseau-a", t).await.unwrap());
    assert!(p.noter_contact(&reg, &f[4].cog, "reseau-b", t + 1).await.unwrap());
    let avant = p.federation().clone();
    drop(p);

    let mut p = recharger(ouvrir().await, &reg, t + 2).await;
    assert_eq!(p.federation().vue_a_jour_au(), avant.vue_a_jour_au());
    assert!(p.federation().controler_vue(t + 2).is_ok());
    let kid = miyusceau::empreinte_cle(&f[2].identite.cle_publique());
    assert_eq!(p.federation().note().points(&kid, t + 2), avant.note().points(&kid, t + 2));
    assert_eq!(p.federation().carnet().reseau(&f[4].cog), Some("reseau-b"));
    assert_eq!(p.federation().choisir_pairs(10, t + 2), avant.choisir_pairs(10, t + 2));
    // Témoignage contradictoire (même séquence, autre racine) : refusé après redémarrage.
    assert!(p
        .temoigner(&reg, &f[1].cog, &f[1].tete(1, 3, [7; 32], t + 2), None, &journal_local(), t + 2)
        .await
        .is_err());
    // Premier contact conservé : un nouveau réseau ne rajeunit pas le pair.
    assert!(p.noter_contact(&reg, &f[4].cog, "reseau-c", t + 3).await.unwrap());
    drop(p);
    let p = recharger(ouvrir().await, &reg, t + 4).await;
    assert_eq!(p.federation().carnet().reseau(&f[4].cog), Some("reseau-b"));
    assert_eq!(p.federation().carnet().premier_contact(&f[4].cog), Some(t + 1));
}

/// Heure en recul au rechargement : refus, aucune vue rendue.
pub async fn horloge_en_recul<S: SupportEtat, K: CompteurFraicheur + 'static>(ouvrir: impl AsyncFn() -> (S, K)) {
    let reg = registre();
    let f = fondateurs(2);
    let mut p = premier(ouvrir().await, &reg).await;
    avec_fondateurs(&mut p, &reg, &f).await;
    p.actualiser(&reg, T0 + 500).await.unwrap();
    drop(p);
    let r = charger_avec(ouvrir().await, T0 + 499, &reg, true).await;
    assert!(matches!(r, Err(ErreurEtat::RetourHorloge)));
    // État attendu mais absent ailleurs : refus (fail-closed).
}
