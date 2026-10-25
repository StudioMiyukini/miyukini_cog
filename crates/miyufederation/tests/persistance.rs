//! @id     miyufederation_test_persistance
//! @do     verifier_persistance_etat_federal
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — tests de la persistance MFED : aller-retour, altération, troncature, lien symbolique, reprise pendant une suspension, accusation levée non rejouable.

//! Persistance de l'état fédéral local (`MFED`) : revérification des titres
//! signés, HMAC du secret local, reprise fail-closed, rattrapage de l'arrêt
//! (« comme si le processus avait tourné »).

#![allow(clippy::many_single_char_names)]

mod commun;

use std::path::PathBuf;

use commun::{CogTest, DIFFICULTE_TEST, JOUR, T0};
use miyufederation::{
    CleEtatFederal, ErreurEtat, ErreurFederation, Federation, Observation, Parametres, Preuve, Statut,
    StockageFederal, FICHIER_ETAT,
};
use miyusceau::{cosigner, CleSignature, TeteRegistre};

const SECRET: [u8; 32] = [0x5a; 32];
/// Accusé suspendu (jamais levé) et accusé levé par un recours.
const X: usize = 7;
const Y: usize = 6;

fn parametres() -> Parametres {
    Parametres::default().avec_difficulte(DIFFICULTE_TEST).unwrap()
}

fn cle() -> CleEtatFederal {
    CleEtatFederal::deriver(&SECRET)
}

/// Dossier temporaire supprimé à la fin du test.
struct Temp(PathBuf);

impl Temp {
    fn nouveau(nom: &str) -> Self {
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        Self(std::env::temp_dir().join(format!("miyufederation-{nom}-{}-{n}", std::process::id())))
    }

    fn stockage(&self) -> StockageFederal {
        StockageFederal::ouvrir(&self.0, &SECRET).unwrap()
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

struct Scenario {
    f: Vec<CogTest>,
    a: CogTest,
    b: CogTest,
    v: Federation,
    acc_y: (Vec<u8>, Preuve),
    recours_y: Vec<u8>,
    echeance_x: u64,
    t: u64,
}

fn accusation(v: &Federation, f: &[CogTest], accuse: &CogTest, t: u64) -> (Vec<u8>, Preuve) {
    let preuve = Preuve::equivoque(accuse.tete(1, 5, [1; 32], t), accuse.tete(1, 5, [2; 32], t));
    let mut acc = v.accuser(&accuse.cog, &preuve, &f[0].identite, t).unwrap();
    for m in &f[1..6] {
        acc = v.contresigner_accusation(&acc, &preuve, &m.identite, t).unwrap();
    }
    (acc, preuve)
}

/// Vue complète : 8 fondateurs, A invité, B candidat, tête témoignée,
/// témoignage local, note locale, X suspendu, Y suspendu puis levé.
fn scenario() -> Scenario {
    let f: Vec<CogTest> = (1..=8u8).map(|g| CogTest::nouveau(g, T0)).collect();
    let a = CogTest::nouveau(20, T0);
    let b = CogTest::nouveau(21, T0);
    let mut v = Federation::new(parametres(), None);
    for c in &f {
        v.ajouter_fondateur(&c.candidature, T0).unwrap();
    }
    v.recevoir_candidature(&a.candidature, T0).unwrap();
    v.recevoir_candidature(&b.candidature, T0).unwrap();
    v.actualiser(T0).unwrap();
    for m in &f[..4] {
        v.recevoir_confirmation(&m.confirmer(&a, T0 + 60), T0 + 60).unwrap();
    }
    v.recevoir_confirmation(&f[0].confirmer(&b, T0 + 60), T0 + 60).unwrap();
    v.actualiser(T0 + 60).unwrap();
    // Tête de f[0] témoignée par trois membres ; témoignage local d'une tête de f[1].
    let mut h = f[0].tete(1, 2, [5; 32], T0 + 70);
    for m in &f[1..4] {
        h = cosigner::<TeteRegistre>(&h, &m.journal).unwrap();
    }
    v.recevoir_tete(&f[0].cog, &h, None, T0 + 70).unwrap();
    let journal_local = CleSignature::depuis_graine(&[0xee; 32]);
    v.temoigner(&f[1].cog, &f[1].tete(1, 3, [6; 32], T0 + 70), None, &journal_local, T0 + 70).unwrap();
    v.observer(&f[2].cog, Observation::Indisponible, T0 + 80).unwrap();
    // X suspendu, Y suspendu puis levé par six membres.
    let t = T0 + 100;
    let (acc_x, preuve_x) = accusation(&v, &f, &f[X], t);
    let echeance_x = v.recevoir_accusation(&acc_x, &preuve_x, t).unwrap().echeance();
    let acc_y = accusation(&v, &f, &f[Y], t);
    v.recevoir_accusation(&acc_y.0, &acc_y.1, t).unwrap();
    let mut r = v.ouvrir_recours(&f[Y].cog, &f[0].identite, t + 100).unwrap();
    for m in &f[1..6] {
        r = v.contresigner_recours(&r, &m.identite, t + 100).unwrap();
    }
    v.recevoir_recours(&r, t + 100).unwrap();
    let t = t + 100;
    v.actualiser(t).unwrap();
    Scenario { f, a, b, v, acc_y, recours_y: r, echeance_x, t }
}

fn statuts(v: &Federation, s: &Scenario) -> Vec<Option<Statut>> {
    s.f.iter().chain([&s.a, &s.b]).map(|c| v.statut(&c.cog)).collect()
}

#[test]
fn aller_retour() {
    let s = scenario();
    let d = Temp::nouveau("aller-retour");
    let st = d.stockage();
    st.enregistrer(&s.v).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(std::fs::metadata(&d.0).unwrap().permissions().mode() & 0o777, 0o700);
        assert_eq!(std::fs::metadata(st.chemin()).unwrap().permissions().mode() & 0o777, 0o600);
    }
    let r = st.charger(parametres(), None, s.t).unwrap();
    assert!(!r.premier_usage);
    assert_eq!(r.revocations.len(), 0);
    let g = r.federation;
    // Re-scellé à l'identique : rien n'est perdu ni ajouté.
    assert_eq!(g.sceller_etat(&cle()).unwrap(), s.v.sceller_etat(&cle()).unwrap());
    assert_eq!(statuts(&g, &s), statuts(&s.v, &s));
    assert_eq!(g.statut(&s.a.cog), Some(Statut::Invite));
    assert_eq!(g.statut(&s.b.cog), Some(Statut::Candidat));
    assert!(matches!(g.statut(&s.f[X].cog), Some(Statut::Suspendu { .. })));
    assert_eq!(g.statut(&s.f[Y].cog), Some(Statut::Membre));
    assert_eq!(g.membres(), s.v.membres());
    assert_eq!(g.suspensions().collect::<Vec<_>>(), s.v.suspensions().collect::<Vec<_>>());
    assert_eq!(g.vue_a_jour_au(), s.v.vue_a_jour_au());
    let kid = miyusceau::empreinte_cle(&s.f[2].identite.cle_publique());
    assert_eq!(g.note().points(&kid, s.t), 2);
    // Fichier absent : premier usage, aucun membre hors fondateurs à réinscrire.
    let vide = Temp::nouveau("premier-usage");
    let r = vide.stockage().charger(parametres(), None, s.t).unwrap();
    assert!(r.premier_usage);
    assert_eq!(r.federation.membres().len(), 0);
}

#[test]
fn alteration_d_un_octet_refusee() {
    let s = scenario();
    let o = s.v.sceller_etat(&cle()).unwrap();
    let reprendre = |o: &[u8]| Federation::reprendre(o, &cle(), parametres(), None, s.t);
    assert!(reprendre(&o).is_ok());
    let mut positions: Vec<usize> = (0..o.len()).step_by(7).collect();
    positions.push(o.len() - 1);
    for i in positions {
        let mut m = o.clone();
        m[i] ^= 0x01;
        let e = reprendre(&m).expect_err("octet altéré accepté");
        match i {
            4 => assert!(matches!(e, ErreurEtat::VersionInconnue(0x00)), "{i} : {e:?}"),
            0..=8 => assert!(matches!(e, ErreurEtat::Illisible(_)), "{i} : {e:?}"),
            _ => assert!(matches!(e, ErreurEtat::Altere), "{i} : {e:?}"),
        }
    }
    // Version inconnue, autre secret local.
    let mut v2 = o.clone();
    v2[4] = 0x02;
    assert!(matches!(reprendre(&v2), Err(ErreurEtat::VersionInconnue(0x02))));
    assert!(matches!(
        Federation::reprendre(&o, &CleEtatFederal::deriver(&[1; 32]), parametres(), None, s.t),
        Err(ErreurEtat::Altere)
    ));
    // Par le stockage : altéré sur disque ⇒ refus, fédération désactivée.
    let d = Temp::nouveau("altere");
    let st = d.stockage();
    let mut m = o;
    let milieu = m.len() / 2;
    m[milieu] ^= 0x80;
    st.ecrire(&m).unwrap();
    assert!(matches!(st.charger(parametres(), None, s.t), Err(ErreurEtat::Altere)));
}

#[test]
fn troncature_refusee() {
    let s = scenario();
    let o = s.v.sceller_etat(&cle()).unwrap();
    for n in [0, 1, 4, 5, 8, 9, 40, o.len() / 2, o.len() - 33, o.len() - 32, o.len() - 1] {
        assert!(
            matches!(Federation::reprendre(&o[..n], &cle(), parametres(), None, s.t), Err(ErreurEtat::Illisible(_))),
            "troncature à {n} acceptée"
        );
    }
    // Octet en trop.
    let plus = [o.as_slice(), &[0]].concat();
    assert!(matches!(Federation::reprendre(&plus, &cle(), parametres(), None, s.t), Err(ErreurEtat::Illisible(_))));
    let d = Temp::nouveau("tronque");
    let st = d.stockage();
    st.ecrire(&o[..o.len() - 1]).unwrap();
    assert!(st.charger(parametres(), None, s.t).is_err());
}

#[cfg(unix)]
#[test]
fn lien_symbolique_et_droits_refuses() {
    use std::os::unix::fs::PermissionsExt;
    let s = scenario();
    let o = s.v.sceller_etat(&cle()).unwrap();
    let d = Temp::nouveau("lien");
    let st = d.stockage();
    // État valide ailleurs, lien symbolique à la place du fichier d'état.
    let ailleurs = d.0.join("ailleurs");
    std::fs::write(&ailleurs, &o).unwrap();
    std::fs::set_permissions(&ailleurs, std::fs::Permissions::from_mode(0o600)).unwrap();
    std::os::unix::fs::symlink(&ailleurs, st.chemin()).unwrap();
    assert!(matches!(st.charger(parametres(), None, s.t), Err(ErreurEtat::LienSymbolique)));
    assert!(matches!(st.enregistrer(&s.v), Err(ErreurEtat::LienSymbolique)));
    std::fs::remove_file(st.chemin()).unwrap();
    // Dossier lien symbolique.
    let lien = d.0.join("dossier-lien");
    std::os::unix::fs::symlink(&d.0, &lien).unwrap();
    assert!(matches!(StockageFederal::ouvrir(&lien, &SECRET), Err(ErreurEtat::LienSymbolique)));
    // Droits ouverts.
    st.enregistrer(&s.v).unwrap();
    std::fs::set_permissions(st.chemin(), std::fs::Permissions::from_mode(0o644)).unwrap();
    assert!(matches!(st.charger(parametres(), None, s.t), Err(ErreurEtat::DroitsOuverts)));
    std::fs::set_permissions(st.chemin(), std::fs::Permissions::from_mode(0o600)).unwrap();
    assert!(st.charger(parametres(), None, s.t).is_ok());
    assert_eq!(st.chemin(), d.0.join(FICHIER_ETAT));
}

#[test]
fn reprise_pendant_la_suspension_avant_72h() {
    let s = scenario();
    let d = Temp::nouveau("avant-72h");
    let st = d.stockage();
    st.enregistrer(&s.v).unwrap();
    let t = s.echeance_x - 1;
    let r = st.charger(parametres(), None, t).unwrap();
    assert_eq!(r.revocations.len(), 0);
    let mut g = r.federation;
    assert!(matches!(g.statut(&s.f[X].cog), Some(Statut::Suspendu { .. })));
    assert!(matches!(g.acces_federe(&s.f[X].cog, t), Err(ErreurFederation::Suspendu)));
    // Jumeau qui a tourné : même état.
    let mut jumeau = s.v.clone();
    jumeau.actualiser(t).unwrap();
    assert_eq!(g.sceller_etat(&cle()).unwrap(), jumeau.sceller_etat(&cle()).unwrap());
    // L'échéance arrive après la reprise : révocation une seule fois.
    let rev = g.actualiser(s.echeance_x).unwrap();
    assert_eq!(rev, jumeau.actualiser(s.echeance_x).unwrap());
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0].cog, s.f[X].cog);
    assert_eq!(g.actualiser(s.echeance_x + 10).unwrap().len(), 0);
}

#[test]
fn reprise_apres_72h_revoque_une_seule_fois() {
    let s = scenario();
    let d = Temp::nouveau("apres-72h");
    let st = d.stockage();
    st.enregistrer(&s.v).unwrap();
    let t = s.echeance_x + JOUR;
    let r = st.charger(parametres(), None, t).unwrap();
    // Exactement comme si le processus avait tourné (actualisé à l'échéance).
    let mut jumeau = s.v.clone();
    let attendu = jumeau.actualiser(s.echeance_x).unwrap();
    jumeau.actualiser(t).unwrap();
    assert_eq!(r.revocations, attendu);
    assert_eq!(r.revocations.len(), 1);
    assert_eq!(r.revocations[0].revoque_a, s.echeance_x);
    assert_eq!(r.revocations[0].cog, s.f[X].cog);
    let g = r.federation;
    assert_eq!(g.statut(&s.f[X].cog), Some(Statut::Revoque));
    assert_eq!(g.sceller_etat(&cle()).unwrap(), jumeau.sceller_etat(&cle()).unwrap());
    // L'état a été réécrit : une nouvelle reprise ne la renvoie plus.
    let r2 = st.charger(parametres(), None, t + JOUR).unwrap();
    assert_eq!(r2.revocations.len(), 0);
    assert_eq!(r2.federation.statut(&s.f[X].cog), Some(Statut::Revoque));
    assert_eq!(r2.federation.revocations().count(), 1);
    // Révocation définitive : plus de recours possible après reprise.
    assert!(matches!(
        r2.federation.ouvrir_recours(&s.f[X].cog, &s.f[0].identite, t + JOUR),
        Err(ErreurFederation::Revoque)
    ));
}

#[test]
fn accusation_levee_non_rejouable_apres_rechargement() {
    let s = scenario();
    let d = Temp::nouveau("levee");
    let st = d.stockage();
    st.enregistrer(&s.v).unwrap();
    let t = s.t + 10;
    let mut g = st.charger(parametres(), None, t).unwrap().federation;
    assert_eq!(g.statut(&s.f[Y].cog), Some(Statut::Membre));
    assert!(matches!(g.recevoir_accusation(&s.acc_y.0, &s.acc_y.1, t), Err(ErreurFederation::AccusationLevee)));
    assert!(matches!(g.recevoir_recours(&s.recours_y, t), Err(ErreurFederation::AucuneSuspension)));
    assert_eq!(g.statut(&s.f[Y].cog), Some(Statut::Membre));
    // Après 72 h, toujours aucune révocation de Y.
    let r = st.charger(parametres(), None, s.echeance_x + JOUR).unwrap();
    assert!(r.revocations.iter().all(|x| x.cog != s.f[Y].cog));
    assert_eq!(r.federation.statut(&s.f[Y].cog), Some(Statut::Membre));
    // Une nouvelle accusation (quorum neuf) peut suspendre de nouveau.
    let (acc2, preuve2) = accusation(&g, &s.f, &s.f[Y], t + 20);
    assert!(g.recevoir_accusation(&acc2, &preuve2, t + 20).is_ok());
}

#[test]
fn membre_acquis_pendant_l_arret() {
    let f: Vec<CogTest> = (1..=6u8).map(|g| CogTest::nouveau(g, T0)).collect();
    let a = CogTest::nouveau(30, T0);
    let mut v = Federation::new(parametres(), None);
    for c in &f {
        v.ajouter_fondateur(&c.candidature, T0).unwrap();
    }
    v.recevoir_candidature(&a.candidature, T0).unwrap();
    for jour in 0..=6u64 {
        let t = T0 + jour * JOUR + 60;
        for m in &f[..4] {
            v.recevoir_confirmation(&m.confirmer(&a, t), t).unwrap();
        }
        v.actualiser(t).unwrap();
    }
    // Deux confirmations qui couvrent la fin des 7 jours (membre acquis conservé ≥ 2).
    let t = T0 + 6 * JOUR + 3_600;
    for m in &f[4..6] {
        v.recevoir_confirmation(&m.confirmer(&a, t), t).unwrap();
    }
    v.actualiser(t).unwrap();
    assert_eq!(v.statut(&a.cog), Some(Statut::Invite));
    let d = Temp::nouveau("membre");
    let st = d.stockage();
    st.enregistrer(&v).unwrap();
    // Arrêt pendant le passage à membre (T0 + 7 j + 60 s) : reprise plus tard.
    let reprise = T0 + 7 * JOUR + 1_000;
    let g = st.charger(parametres(), None, reprise).unwrap().federation;
    let mut jumeau = v.clone();
    jumeau.actualiser(T0 + 7 * JOUR + 60).unwrap();
    jumeau.actualiser(reprise).unwrap();
    assert_eq!(jumeau.statut(&a.cog), Some(Statut::Membre));
    assert_eq!(g.statut(&a.cog), Some(Statut::Membre));
    assert_eq!(g.sceller_etat(&cle()).unwrap(), jumeau.sceller_etat(&cle()).unwrap());
    // Sous 2 confirmations, le statut membre est perdu, comme en fonctionnement.
    let mut g = g;
    g.actualiser(T0 + 8 * JOUR).unwrap();
    assert_eq!(g.statut(&a.cog), Some(Statut::Candidat));
}

#[test]
fn heure_en_recul_refusee_et_rien_n_est_rendu() {
    let s = scenario();
    let d = Temp::nouveau("recul");
    let st = d.stockage();
    st.enregistrer(&s.v).unwrap();
    assert!(matches!(st.charger(parametres(), None, s.t - 1), Err(ErreurEtat::RetourHorloge)));
}
