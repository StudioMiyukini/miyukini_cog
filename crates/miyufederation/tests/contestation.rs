//! @id     miyufederation_test_contestation
//! @do     verifier_delai_de_contestation
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — tests du délai de contestation (A-007, EX-32) : suspension immédiate, révocation après 72 h, recours accepté par 6 membres.

//! Délai de contestation (EX-32, décision du propriétaire A-007) :
//! accusation au quorum ⇒ suspension immédiate ; révocation définitive après
//! 72 h sauf « recours accepté » signé par 6 membres distincts (hors accusé)
//! sur la même empreinte de preuve.

#![allow(clippy::many_single_char_names)]

mod commun;

use commun::{federation, CogTest, JOUR, T0};
use miyauth::{
    emettre_passeport, verifier_passeport, AnnuaireEmetteurs, CibleRevocation, DemandePasseport, ListeRevocation,
    PolitiqueIdentite, Portees, RefusIdentite, RevocationsFederees, SecretTitulaire,
};
use miyufederation::{
    ErreurFederation, Federation, Preuve, Recours, RevocationsFederation, Statut, DELAI_CONTESTATION_SECS,
    QUORUM_RECOURS,
};
use miyuregistre::{Cible, Corps};
use miyusceau::{cosigner, empreinte_cle, NiveauConfiance};

/// Accusé (fondateur) : indice 7 ; membres non accusés : 0..7.
const X: usize = 7;

fn fondateurs() -> Vec<CogTest> {
    (1..=8u8).map(|g| CogTest::nouveau(g, T0)).collect()
}

fn vue(f: &[CogTest], candidats: &[&CogTest]) -> Federation {
    let mut v = federation();
    for c in f {
        v.ajouter_fondateur(&c.candidature, T0).unwrap();
    }
    for c in candidats {
        v.recevoir_candidature(&c.candidature, T0).unwrap();
    }
    v.actualiser(T0).unwrap();
    v
}

/// Accusation au quorum de 6 (signataires 0..6) sur une équivoque de `accuse`.
fn accusation(v: &Federation, f: &[CogTest], accuse: &CogTest, t: u64) -> (Vec<u8>, Preuve) {
    let preuve = Preuve::equivoque(accuse.tete(1, 5, [1; 32], t), accuse.tete(1, 5, [2; 32], t));
    let mut acc = v.accuser(&accuse.cog, &preuve, &f[0].identite, t).unwrap();
    for m in &f[1..6] {
        acc = v.contresigner_accusation(&acc, &preuve, &m.identite, t).unwrap();
    }
    (acc, preuve)
}

/// Recours signé par les membres `signataires` (premier : ouvreur).
fn recours(v: &Federation, f: &[CogTest], accuse: &CogTest, signataires: &[usize], t: u64) -> Vec<u8> {
    let mut r = v.ouvrir_recours(&accuse.cog, &f[signataires[0]].identite, t).unwrap();
    for &m in &signataires[1..] {
        r = v.contresigner_recours(&r, &f[m].identite, t).unwrap();
    }
    r
}

fn passeport(emetteur: &CogTest, destination: &CogTest, t: u64) -> Vec<u8> {
    emettre_passeport(
        &emetteur.cog,
        &emetteur.emission,
        &DemandePasseport {
            secret: &SecretTitulaire::depuis_octets([7; 32]),
            destination: destination.cog.clone(),
            cle_appareil: miyusceau::CleSignature::generer().cle_publique(),
            portees: Portees::new(["jayrdv.lecture"]).unwrap(),
            pas_avant: t,
            pas_apres: t + JOUR,
        },
    )
    .unwrap()
}

fn verifier_chez(vue: &Federation, hote: &CogTest, emetteur: &CogTest, octets: &[u8], t: u64) -> Result<(), RefusIdentite> {
    let mut annuaire = AnnuaireEmetteurs::new();
    annuaire.ajouter(emetteur.cog.clone(), &emetteur.emission.cle_publique(), NiveauConfiance::Reconnu).unwrap();
    let mut locales = RevocationsFederees::new();
    locales.inserer(ListeRevocation::new(emetteur.cog.clone(), t, 900));
    locales.inserer(ListeRevocation::new(hote.cog.clone(), t, 900));
    let source = RevocationsFederation::new(vue, &locales);
    let politique = PolitiqueIdentite { cog_local: hote.cog.clone(), annuaire: &annuaire, revocations: &source };
    verifier_passeport(octets, &politique, t).map(|_| ())
}

#[test]
fn suspension_immediate_puis_revocation_apres_72h() {
    let f = fondateurs();
    let mut v = vue(&f, &[]);
    let t = T0 + 100;
    let (acc, preuve) = accusation(&v, &f, &f[X], t);
    let s = v.recevoir_accusation(&acc, &preuve, t).unwrap();
    assert_eq!(s.depuis, t);
    assert_eq!(s.echeance(), t + DELAI_CONTESTATION_SECS);
    assert_eq!(DELAI_CONTESTATION_SECS, 72 * 3_600);
    // Idempotent pendant la suspension.
    assert_eq!(v.recevoir_accusation(&acc, &preuve, t + 1).unwrap(), s);

    // Suspension effective immédiatement : titres de l'accusé refusés.
    assert_eq!(v.statut(&f[X].cog), Some(Statut::Suspendu { depuis: t, accusation: s.accusation }));
    assert!(v.cog_suspendu(&f[X].cog));
    assert!(!v.cog_revoque(&f[X].cog));
    assert_eq!(v.revocations().count(), 0);
    let p = passeport(&f[X], &f[0], t + 5);
    assert!(matches!(
        verifier_chez(&v, &f[0], &f[X], &p, t + 10),
        Err(RefusIdentite::Revoque(CibleRevocation::Serie(_)))
    ));
    assert!(matches!(v.acces_federe(&f[X].cog, t + 10), Err(ErreurFederation::Suspendu)));
    assert!(!v.pair_eligible(&f[X].cog, t + 10));
    assert!(!v.membres().contains(&f[X].cog));
    // L'accusé ne confirme plus, ses têtes ne sont plus reçues.
    let c = f[X].confirmer(&f[0], t + 10);
    assert!(matches!(v.recevoir_confirmation(&c, t + 10), Err(ErreurFederation::Suspendu)));
    let h = f[X].tete(2, 6, [3; 32], t + 10);
    assert!(matches!(v.recevoir_tete(&f[X].cog, &h, None, t + 10), Err(ErreurFederation::Suspendu)));

    // Juste avant 72 h : toujours suspendu, aucune révocation.
    let echeance = s.echeance();
    assert_eq!(v.actualiser(echeance - 1).unwrap().len(), 0);
    assert!(matches!(v.statut(&f[X].cog), Some(Statut::Suspendu { .. })));

    // 72 h écoulées sans recours : révocation définitive, entrée de registre produite.
    let definitives = v.actualiser(echeance).unwrap();
    assert_eq!(definitives.len(), 1);
    let rev = &definitives[0];
    assert_eq!(rev.cog, f[X].cog);
    assert_eq!(rev.accusation, s.accusation);
    assert_eq!(rev.revoque_a, echeance);
    match rev.entree().corps {
        Corps::Revocation { cible: Cible::Cle(kid), .. } => assert_eq!(kid, empreinte_cle(&f[X].identite.cle_publique())),
        autre => panic!("révocation attendue : {autre:?}"),
    }
    assert_eq!(v.statut(&f[X].cog), Some(Statut::Revoque));
    assert!(!v.cog_suspendu(&f[X].cog));
    assert!(v.cog_revoque(&f[X].cog));
    assert_eq!(v.suspensions().count(), 0);
    // Une seule fois.
    assert_eq!(v.actualiser(echeance + 10).unwrap().len(), 0);
    // Plus de recours possible, ni de nouvelle accusation.
    assert!(matches!(v.ouvrir_recours(&f[X].cog, &f[0].identite, echeance + 20), Err(ErreurFederation::Revoque)));
    assert!(matches!(v.recevoir_accusation(&acc, &preuve, echeance + 20), Err(ErreurFederation::Revoque)));
}

#[test]
fn recours_tardif_refuse_et_revocation_appliquee() {
    let f = fondateurs();
    let mut v = vue(&f, &[]);
    let t = T0 + 100;
    let (acc, preuve) = accusation(&v, &f, &f[X], t);
    let s = v.recevoir_accusation(&acc, &preuve, t).unwrap();
    let r = recours(&v, &f, &f[X], &[0, 1, 2, 3, 4, 5], t + 60);
    // Présenté après l'échéance (sans actualisation intermédiaire) : la
    // révocation est appliquée, le recours refusé.
    assert!(matches!(v.recevoir_recours(&r, s.echeance()), Err(ErreurFederation::Revoque)));
    assert_eq!(v.statut(&f[X].cog), Some(Statut::Revoque));
    assert_eq!(v.revocations().count(), 1);
}

#[test]
fn recours_de_six_membres_leve_la_suspension() {
    let f = fondateurs();
    let mut v = vue(&f, &[]);
    let t = T0 + 100;
    let (acc, preuve) = accusation(&v, &f, &f[X], t);
    let s = v.recevoir_accusation(&acc, &preuve, t).unwrap();
    // Six membres distincts (dont un qui n'avait pas accusé), avant 72 h.
    let r = recours(&v, &f, &f[X], &[6, 1, 2, 3, 4, 5], t + 3_600);
    let levee = v.recevoir_recours(&r, t + 3_600).unwrap();
    assert_eq!(levee, s);
    assert_eq!(QUORUM_RECOURS, 6);
    // Statut précédent rétabli (fondateur : membre), titres de nouveau acceptés.
    assert_eq!(v.statut(&f[X].cog), Some(Statut::Membre));
    assert!(!v.cog_suspendu(&f[X].cog));
    assert!(v.membres().contains(&f[X].cog));
    // Aucune révocation, même après 72 h.
    assert_eq!(v.actualiser(s.echeance() + JOUR).unwrap().len(), 0);
    assert_eq!(v.revocations().count(), 0);
    assert_eq!(v.statut(&f[X].cog), Some(Statut::Membre));
    // L'accusation levée ne peut pas être rejouée ; le recours non plus.
    let t2 = s.echeance() + JOUR + 10;
    assert!(matches!(v.recevoir_accusation(&acc, &preuve, t2), Err(ErreurFederation::AccusationLevee)));
    assert!(matches!(v.recevoir_recours(&r, t2), Err(ErreurFederation::AucuneSuspension)));
    // Une nouvelle accusation (nouveau quorum) peut suspendre de nouveau.
    let (acc2, preuve2) = accusation(&v, &f, &f[X], t2);
    let s2 = v.recevoir_accusation(&acc2, &preuve2, t2).unwrap();
    assert_ne!(s2.accusation, s.accusation);
    // L'ancien recours ne lève pas la nouvelle suspension.
    assert!(matches!(v.recevoir_recours(&r, t2 + 1), Err(ErreurFederation::PreuveNonConcordante)));
}

#[test]
fn recours_de_cinq_membres_insuffisant() {
    let f = fondateurs();
    let mut v = vue(&f, &[]);
    let t = T0 + 100;
    let (acc, preuve) = accusation(&v, &f, &f[X], t);
    v.recevoir_accusation(&acc, &preuve, t).unwrap();
    let r = recours(&v, &f, &f[X], &[0, 1, 2, 3, 4], t + 60);
    assert!(matches!(
        v.recevoir_recours(&r, t + 60),
        Err(ErreurFederation::QuorumNonAtteint { requis: 6, obtenu: 5 })
    ));
    assert!(matches!(v.statut(&f[X].cog), Some(Statut::Suspendu { .. })));
    // Signé par des non-membres seulement : refusé.
    let etranger = CogTest::nouveau(90, T0);
    let s = v.suspensions().next().unwrap().clone();
    let forge = Recours {
        accuse: s.racine,
        type_preuve: s.type_preuve,
        empreinte_preuve: s.empreinte_preuve,
        accusation: s.accusation,
        emise_a: t + 70,
    }
    .sceller(&etranger.identite)
    .unwrap();
    assert!(matches!(v.recevoir_recours(&forge, t + 70), Err(ErreurFederation::NonMembre)));
    // Autre empreinte de preuve : refusé.
    let autre = Recours { empreinte_preuve: [9; 32], ..decoder(&r) }.sceller(&f[0].identite).unwrap();
    assert!(matches!(v.recevoir_recours(&autre, t + 70), Err(ErreurFederation::PreuveNonConcordante)));
    assert!(matches!(v.contresigner_recours(&autre, &f[1].identite, t + 70), Err(ErreurFederation::PreuveNonConcordante)));
}

fn decoder(octets: &[u8]) -> Recours {
    miyusceau::decoder_titre::<Recours>(&miyusceau::Sceau::decoder(octets).unwrap().charge).unwrap()
}

#[test]
fn recours_signe_par_l_accuse_refuse() {
    let f = fondateurs();
    let mut v = vue(&f, &[]);
    let t = T0 + 100;
    let (acc, preuve) = accusation(&v, &f, &f[X], t);
    v.recevoir_accusation(&acc, &preuve, t).unwrap();
    // L'accusé ne peut ni ouvrir, ni cosigner son recours.
    assert!(matches!(v.ouvrir_recours(&f[X].cog, &f[X].identite, t + 60), Err(ErreurFederation::AutoConfirmation)));
    let r5 = recours(&v, &f, &f[X], &[0, 1, 2, 3, 4], t + 60);
    assert!(v.contresigner_recours(&r5, &f[X].identite, t + 60).is_err());
    assert!(Recours::cosigner(&r5, &f[X].identite).is_err());
    // Signature de l'accusé forcée au niveau du format : jamais comptée.
    let force = cosigner::<Recours>(&r5, &f[X].identite).unwrap();
    assert!(matches!(
        v.recevoir_recours(&force, t + 60),
        Err(ErreurFederation::QuorumNonAtteint { requis: 6, obtenu: 5 })
    ));
    assert!(matches!(v.statut(&f[X].cog), Some(Statut::Suspendu { .. })));
    // Un invité (non membre) ne compte pas non plus.
    let invite = CogTest::nouveau(60, T0);
    assert!(matches!(v.ouvrir_recours(&f[X].cog, &invite.identite, t + 60), Err(ErreurFederation::NonMembre)));
}

#[test]
fn levee_retablit_l_invite_et_reprend_le_compteur_des_7_jours() {
    let f = fondateurs();
    let y = CogTest::nouveau(70, T0);
    let mut v = vue(&f, &[&y]);
    let confirmer = |v: &mut Federation, t: u64| {
        for m in &f[..4] {
            v.recevoir_confirmation(&m.confirmer(&y, t), t).unwrap();
        }
        v.actualiser(t).unwrap();
    };
    for jour in 0..3 {
        confirmer(&mut v, T0 + jour * JOUR + 60);
    }
    assert_eq!(v.statut(&y.cog), Some(Statut::Invite));
    // Accusation au quorum à J2 : suspendu ; les membres continuent de le confirmer.
    let ts = T0 + 2 * JOUR + 120;
    let (acc, preuve) = accusation(&v, &f, &y, ts);
    v.recevoir_accusation(&acc, &preuve, ts).unwrap();
    confirmer(&mut v, T0 + 3 * JOUR + 60);
    assert!(matches!(v.statut(&y.cog), Some(Statut::Suspendu { .. })));
    // Recours accepté à J3 : retour au statut précédent (invité).
    let tl = T0 + 3 * JOUR + 120;
    let r = recours(&v, &f, &y, &[0, 1, 2, 3, 4, 5], tl);
    v.recevoir_recours(&r, tl).unwrap();
    assert_eq!(v.statut(&y.cog), Some(Statut::Invite));
    // Sans suspension, Y serait membre à J7 ; le compteur repart de la levée.
    for jour in 4..=10 {
        confirmer(&mut v, T0 + jour * JOUR + 60);
        assert_eq!(v.statut(&y.cog), Some(Statut::Invite), "jour {jour}");
    }
    v.actualiser(tl + 7 * JOUR).unwrap();
    assert_eq!(v.statut(&y.cog), Some(Statut::Membre));
}
