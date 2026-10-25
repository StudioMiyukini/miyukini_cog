//! @id     miyufederation_test_regles
//! @do     verifier_regles_federation
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — tests des règles : seuils, remise à zéro, quotas, preuve de calcul, faux signalement, témoins, Origin facultatif.

mod commun;

use commun::{federation, CogTest, DIFFICULTE_TEST, JOUR, T0};
use miyufederation::{
    Accusation, CarnetPairs, Candidature, Confirmation, ErreurFederation, Federation, Observation, Parametres, Preuve, Statut,
    QUOTA_CONFIRMATIONS,
};
use miyusceau::{cosigner, ErreurSceau, TeteRegistre};

/// Vue avec `n` fondateurs et les candidats donnés.
fn vue(fondateurs: &[CogTest], candidats: &[&CogTest]) -> Federation {
    let mut v = federation();
    for f in fondateurs {
        v.ajouter_fondateur(&f.candidature, T0).unwrap();
    }
    for c in candidats {
        v.recevoir_candidature(&c.candidature, T0).unwrap();
    }
    v
}

fn fondateurs(n: u8) -> Vec<CogTest> {
    (1..=n).map(|g| CogTest::nouveau(g, T0)).collect()
}

fn confirmer_par(v: &mut Federation, membres: &[&CogTest], x: &CogTest, t: u64) {
    for m in membres {
        v.recevoir_confirmation(&m.confirmer(x, t), t).unwrap();
    }
    v.actualiser(t).unwrap();
}

#[test]
fn seuils_invite_et_candidat() {
    let f = fondateurs(4);
    let x = CogTest::nouveau(50, T0);
    let mut v = vue(&f, &[&x]);
    confirmer_par(&mut v, &[&f[0]], &x, T0);
    assert_eq!(v.statut(&x.cog), Some(Statut::Candidat));
    confirmer_par(&mut v, &[&f[1]], &x, T0 + 1);
    assert_eq!(v.statut(&x.cog), Some(Statut::Invite));
    // Expiration des confirmations (non renouvelées) : retour candidat.
    v.actualiser(T0 + JOUR + 10).unwrap();
    assert_eq!(v.statut(&x.cog), Some(Statut::Candidat));
    // Auto-confirmation impossible.
    assert!(Confirmation::nouvelle(x.identite.cle_publique(), x.identite.cle_publique(), T0).sceller(&x.identite).is_err());
}

#[test]
fn compteur_remis_a_zero_sous_quatre() {
    let f = fondateurs(4);
    let x = CogTest::nouveau(50, T0);
    let mut v = vue(&f, &[&x]);
    let tous: Vec<&CogTest> = f.iter().collect();
    for jour in 0..4u64 {
        confirmer_par(&mut v, &tous, &x, T0 + jour * JOUR);
    }
    // Jour 4 : un seul membre ne renouvelle pas (3 confirmations fraîches après expiration).
    let t4 = T0 + 4 * JOUR;
    confirmer_par(&mut v, &tous[..3], &x, t4);
    v.actualiser(t4 + 10).unwrap();
    assert_eq!(v.statut(&x.cog), Some(Statut::Invite));
    for jour in 5..=11u64 {
        confirmer_par(&mut v, &tous, &x, T0 + jour * JOUR);
        assert_eq!(v.statut(&x.cog), Some(Statut::Invite), "jour {jour}");
    }
    confirmer_par(&mut v, &tous, &x, T0 + 12 * JOUR);
    assert_eq!(v.statut(&x.cog), Some(Statut::Membre));
    // Membre : reste membre à 2 confirmations, perd le statut sous 2 (statut
    // alors fixé par le compte : 0–1 = candidat ; lecture fail-closed de
    // « membre durablement sous 2 → invité », à amender).
    let t13 = T0 + 13 * JOUR;
    confirmer_par(&mut v, &tous[..2], &x, t13);
    v.actualiser(t13 + 10).unwrap();
    assert_eq!(v.statut(&x.cog), Some(Statut::Membre));
    let t14 = T0 + 14 * JOUR;
    confirmer_par(&mut v, &tous[..1], &x, t14);
    v.actualiser(t14 + 10).unwrap();
    assert_eq!(v.statut(&x.cog), Some(Statut::Candidat));
}

#[test]
fn confirmation_compte_a_partir_de_la_reception() {
    let f = fondateurs(4);
    let x = CogTest::nouveau(50, T0);
    let mut v = vue(&f, &[&x]);
    let tous: Vec<&CogTest> = f.iter().collect();
    // Confirmations datées de T0 mais reçues 23 h plus tard.
    let recu = T0 + 23 * 3600;
    for m in &tous {
        v.recevoir_confirmation(&m.confirmer(&x, T0), recu).unwrap();
    }
    for jour in 1..=7u64 {
        confirmer_par(&mut v, &tous, &x, T0 + jour * JOUR);
    }
    // 7 j depuis T0 mais pas depuis la réception : encore invité.
    assert_eq!(v.statut(&x.cog), Some(Statut::Invite));
    v.actualiser(recu + 7 * JOUR).unwrap();
    assert_eq!(v.statut(&x.cog), Some(Statut::Membre));
}

#[test]
fn quota_de_confirmations() {
    let f = fondateurs(1);
    let candidats: Vec<CogTest> = (0..=QUOTA_CONFIRMATIONS as u8).map(|i| CogTest::nouveau(100 + i, T0)).collect();
    let refs: Vec<&CogTest> = candidats.iter().collect();
    let mut v = vue(&f, &refs);
    for (i, c) in candidats.iter().enumerate() {
        let r = v.recevoir_confirmation(&f[0].confirmer(c, T0), T0);
        if i < QUOTA_CONFIRMATIONS {
            r.unwrap();
        } else {
            assert!(matches!(r, Err(ErreurFederation::QuotaDepasse)));
        }
    }
    // Le renouvellement d'une confirmation existante ne consomme pas de quota.
    v.recevoir_confirmation(&f[0].confirmer(&candidats[0], T0 + JOUR), T0 + JOUR).unwrap();
    // Une nouvelle période rouvre le quota.
    let t = T0 + 31 * JOUR;
    v.recevoir_confirmation(&f[0].confirmer(&candidats[QUOTA_CONFIRMATIONS], t), t).unwrap();
}

#[test]
fn preuve_de_calcul_et_candidature() {
    let mut v = federation();
    let x = CogTest::nouveau(60, T0);
    // Difficulté exigée supérieure à celle fournie.
    let mut stricte = Federation::new(Parametres::default().avec_difficulte(20).unwrap(), None);
    assert!(matches!(stricte.recevoir_candidature(&x.candidature, T0), Err(ErreurFederation::CalculInsuffisant)));
    // Nonce faux, correctement signé.
    let mut c = Candidature::preparer(x.identite.cle_publique(), x.journal.cle_publique(), x.registre, T0, DIFFICULTE_TEST)
        .unwrap();
    while c.preuve_valide() {
        c.nonce += 1;
    }
    let faux = c.sceller(&x.identite).unwrap();
    assert!(matches!(v.recevoir_candidature(&faux, T0), Err(ErreurFederation::CalculInsuffisant)));
    // Candidature trop ancienne (> 48 h) : refusée.
    assert!(matches!(
        v.recevoir_candidature(&x.candidature, T0 + 3 * JOUR),
        Err(ErreurFederation::Sceau(ErreurSceau::Expire))
    ));
    // Signée par une autre clé que la racine annoncée.
    let autre = CogTest::nouveau(61, T0);
    let c = Candidature::preparer(x.identite.cle_publique(), x.journal.cle_publique(), x.registre, T0, DIFFICULTE_TEST)
        .unwrap();
    assert!(c.sceller(&autre.identite).is_err());
    let usurpee = miyusceau::sceller(&c, &autre.identite).unwrap();
    let mut v2 = federation();
    assert!(v2.recevoir_candidature(&usurpee, T0).is_err());
    // Politique : difficulté sous le plancher refusée.
    assert!(Parametres::default().avec_difficulte(4).is_err());
    // Candidature valide puis contradictoire (autre clé de journal).
    let mut v3 = federation();
    v3.recevoir_candidature(&x.candidature, T0).unwrap();
    v3.recevoir_candidature(&x.candidature, T0).unwrap();
    let c = Candidature::preparer(x.identite.cle_publique(), autre.journal.cle_publique(), x.registre, T0, DIFFICULTE_TEST)
        .unwrap()
        .sceller(&x.identite)
        .unwrap();
    assert!(matches!(v3.recevoir_candidature(&c, T0), Err(ErreurFederation::Contradictoire)));
    // Horloge en recul.
    assert!(matches!(v3.actualiser(T0 - 1), Err(ErreurFederation::RetourHorloge)));
}

fn temoigner(v: &mut [Federation], temoins: &[(usize, &CogTest)], cog: &CogTest, mut tete: Vec<u8>, t: u64) -> Vec<u8> {
    for (i, w) in temoins {
        tete = v[*i].temoigner(&cog.cog, &tete, None, &w.journal, t).unwrap();
    }
    tete
}

#[test]
#[allow(clippy::many_single_char_names)]
fn faux_signalement_et_quorum() {
    let f = fondateurs(7);
    let x = &f[6];
    let mut v = vue(&f, &[]);
    v.actualiser(T0).unwrap();
    // Deux têtes compatibles (séquence et taille croissantes) : pas une faute.
    let a = x.tete(1, 5, [1; 32], T0);
    let b = x.tete(2, 5, [1; 32], T0 + 10);
    let preuve = Preuve::equivoque(a, b);
    assert!(matches!(v.accuser(&x.cog, &preuve, &f[0].identite, T0 + 20), Err(ErreurFederation::PreuveInvalide)));
    // Accusation forgée sans vérification, signée par 6 membres : refusée, auteurs notés.
    let mut acc = Accusation::depuis_preuve(x.identite.cle_publique(), &preuve, T0 + 20).sceller(&f[0].identite).unwrap();
    for m in &f[1..6] {
        acc = cosigner::<Accusation>(&acc, &m.identite).unwrap();
    }
    assert!(matches!(v.recevoir_accusation(&acc, &preuve, T0 + 30), Err(ErreurFederation::PreuveInvalide)));
    assert_eq!(v.note().points(&f[0].identite.kid(), T0 + 30), Observation::FauxSignalement.points());
    assert_eq!(v.statut(&x.cog), Some(Statut::Membre));
    // Preuve ne correspondant pas à l'empreinte signée.
    let vraie = Preuve::equivoque(x.tete(3, 5, [1; 32], T0), x.tete(3, 5, [2; 32], T0));
    assert!(matches!(v.recevoir_accusation(&acc, &vraie, T0 + 40), Err(ErreurFederation::PreuveNonConcordante)));
    // Signée par des non-membres seulement.
    let etranger = CogTest::nouveau(90, T0);
    let acc2 = Accusation::depuis_preuve(x.identite.cle_publique(), &vraie, T0).sceller(&etranger.identite).unwrap();
    assert!(matches!(v.recevoir_accusation(&acc2, &vraie, T0 + 50), Err(ErreurFederation::NonMembre)));
    // Encodage canonique de la preuve.
    assert_eq!(Preuve::decoder(&vraie.encoder()).unwrap(), vraie);
    let Preuve::Equivoque { premiere, seconde } = &vraie;
    let mut desordre = b"MFPV\x01\x01".to_vec();
    for p in [seconde, premiere] {
        desordre.extend_from_slice(&(p.len() as u32).to_be_bytes());
        desordre.extend_from_slice(p);
    }
    assert!(Preuve::decoder(&desordre).is_err());
}

#[test]
fn temoins_sans_origin_et_origin_facultatif() {
    let f = fondateurs(4);
    let x = &f[3];
    let origin = CogTest::nouveau(200, T0);
    let mut vues: Vec<Federation> = (0..3).map(|_| vue(&f, &[])).collect();
    let h = x.tete(1, 2, [5; 32], T0);
    let deux = temoigner(&mut vues, &[(0, &f[0]), (1, &f[1])], x, h.clone(), T0);
    // Sans Origin : deux témoins ne suffisent pas, trois oui.
    let mut sans = vue(&f, &[]);
    assert!(matches!(
        sans.recevoir_tete(&x.cog, &deux, None, T0),
        Err(ErreurFederation::TemoinsInsuffisants { requis: 3, obtenu: 2 })
    ));
    let trois = temoigner(&mut vues, &[(2, &f[2])], x, deux.clone(), T0);
    sans.recevoir_tete(&x.cog, &trois, None, T0).unwrap();
    // Avec Origin configuré : il compte comme un témoin, rien de plus.
    let mut avec = Federation::new(Parametres::default().avec_difficulte(DIFFICULTE_TEST).unwrap(), Some(origin.journal.cle_publique()));
    for c in &f {
        avec.ajouter_fondateur(&c.candidature, T0).unwrap();
    }
    let par_origin = cosigner::<TeteRegistre>(&deux, &origin.journal).unwrap();
    avec.recevoir_tete(&x.cog, &par_origin, None, T0).unwrap();
    let mut avec2 = avec.clone();
    let h2 = x.tete(2, 2, [5; 32], T0 + 5);
    let seul2 = cosigner::<TeteRegistre>(&h2, &origin.journal).unwrap();
    assert!(matches!(
        avec2.recevoir_tete(&x.cog, &seul2, None, T0 + 5),
        Err(ErreurFederation::TemoinsInsuffisants { requis: 3, obtenu: 1 })
    ));
    // Le propriétaire ne se témoigne pas lui-même.
    assert!(matches!(
        vues[0].temoigner(&x.cog, &h, None, &x.journal, T0),
        Err(ErreurFederation::AutoConfirmation)
    ));
}

#[test]
fn carnet_diversifie_et_anciens_conserves() {
    let f = fondateurs(6);
    let v = vue(&f, &[]);
    let mut carnet = CarnetPairs::new();
    // Réseau « r1 » : quatre pairs ; « r2 » et « r3 » : un chacun.
    for (i, c) in f[..4].iter().enumerate() {
        assert!(carnet.noter_contact(&c.cog, "r1", T0 + i as u64));
    }
    carnet.noter_contact(&f[4].cog, "r2", T0 + 10);
    carnet.noter_contact(&f[5].cog, "r3", T0 + 20);
    // Un pair ne change pas de réseau ni ne rajeunit.
    carnet.noter_contact(&f[0].cog, "r9", T0 + 100);
    assert_eq!(carnet.reseau(&f[0].cog), Some("r1"));
    assert!(!carnet.noter_contact(&f[0].cog, "", T0));
    let choix = carnet.choisir(3, &v, T0 + 200);
    assert_eq!(choix, vec![f[0].cog.clone(), f[4].cog.clone(), f[5].cog.clone()]);
    let choix = carnet.choisir(4, &v, T0 + 200);
    assert_eq!(choix[3], f[1].cog);
}
