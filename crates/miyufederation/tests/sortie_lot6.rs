//! @id     miyufederation_test_sortie_lot6
//! @do     verifier_sortie_lot6
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — test de sortie du lot 6 : trois COG de test deviennent membres, un COG fautif est suspendu puis révoqué après 72 h, sans Origin.

//! Sortie du lot 6 (PLAN-IMPLEMENTATION §8) : « trois COG de test atteignent
//! le statut membre ; un COG fautif est révoqué ». Aucune clé d'Origin n'est
//! configurée : chaque COG tient sa propre vue fédérale (EX-30). Délai de
//! contestation (A-007, EX-32) : suspension immédiate, révocation définitive
//! après 72 h sans recours accepté.

mod commun;

use commun::{federation, CogTest, JOUR, T0};
use miyauth::{
    emettre_passeport, verifier_passeport, AnnuaireEmetteurs, CibleRevocation, DemandePasseport, ErreurRevocation,
    ListeRevocation, PolitiqueIdentite, Portees, RefusIdentite, RevocationsFederees, SecretTitulaire,
};
use miyufederation::{
    CarnetPairs, ErreurFederation, Federation, Preuve, RevocationsFederation, Sanction, Statut,
    DELAI_CONTESTATION_SECS, QUORUM_BLACKLIST,
};
use miyuregistre::{Cible, Corps, EntreeRegistre};
use miyusceau::{empreinte_cle, verifier, AncresDeConfiance, CleSignature, NiveauConfiance};

/// Indices : 0..4 fondateurs, 4 = A, 5 = B, 6 = C (fautif).
const A: usize = 4;
const B: usize = 5;
const C: usize = 6;

fn diffuser<F>(vues: &mut [Federation], mut f: F)
where
    F: FnMut(&mut Federation) -> Result<(), ErreurFederation>,
{
    for v in vues.iter_mut() {
        f(v).unwrap();
    }
}

fn passeport(emetteur: &CogTest, destination: &CogTest, t: u64) -> Vec<u8> {
    let appareil = CleSignature::generer();
    emettre_passeport(
        &emetteur.cog,
        &emetteur.emission,
        &DemandePasseport {
            secret: &SecretTitulaire::depuis_octets([7; 32]),
            destination: destination.cog.clone(),
            cle_appareil: appareil.cle_publique(),
            portees: Portees::new(["jayrdv.lecture"]).unwrap(),
            pas_avant: t,
            pas_apres: t + JOUR,
        },
    )
    .unwrap()
}

fn verifier_chez(
    vue: &Federation,
    hote: &CogTest,
    emetteur: &CogTest,
    octets: &[u8],
    t: u64,
) -> Result<(), RefusIdentite> {
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
fn trois_cog_deviennent_membres_puis_un_fautif_est_revoque() {
    let cogs: Vec<CogTest> = (1..=7u8).map(|g| CogTest::nouveau(g, T0)).collect();
    // Chaque COG a sa vue ; pas d'Origin.
    let mut vues: Vec<Federation> = (0..cogs.len()).map(|_| federation()).collect();
    diffuser(&mut vues, |v| {
        for f in &cogs[..4] {
            v.ajouter_fondateur(&f.candidature, T0)?;
        }
        for x in &cogs[A..] {
            v.recevoir_candidature(&x.candidature, T0)?;
        }
        v.actualiser(T0).map(|_| ())
    });
    for v in &vues {
        assert_eq!(v.statut(&cogs[A].cog), Some(Statut::Candidat));
        assert_eq!(v.statut(&cogs[0].cog), Some(Statut::Membre));
    }

    // ── Confirmations quotidiennes des 4 fondateurs pendant 7 jours ──
    for jour in 0..=7u64 {
        let t = T0 + jour * JOUR + 60;
        for f in &cogs[..4] {
            for x in &cogs[A..] {
                let c = f.confirmer(x, t);
                diffuser(&mut vues, |v| v.recevoir_confirmation(&c, t));
            }
        }
        diffuser(&mut vues, |v| v.actualiser(t).map(|_| ()));
        let attendu = if jour < 7 { Statut::Invite } else { Statut::Membre };
        for v in &vues {
            for x in &cogs[A..] {
                assert_eq!(v.statut(&x.cog), Some(attendu), "jour {jour}");
            }
        }
        if jour < 7 {
            // Un invité ne confirme pas.
            let c = cogs[A].confirmer(&cogs[B], t);
            assert!(matches!(vues[0].recevoir_confirmation(&c, t), Err(ErreurFederation::NonMembre)));
        }
    }
    let t1 = T0 + 7 * JOUR + 600;
    assert_eq!(vues[A].membres().len(), 7);

    // ── C équivoque : deux têtes de même séquence, racines différentes ──
    let h1 = cogs[C].tete(1, 5, [0x11; 32], t1);
    let mut h1_temoignee = h1.clone();
    for w in 0..3 {
        h1_temoignee = vues[w].temoigner(&cogs[C].cog, &h1_temoignee, None, &cogs[w].journal, t1).unwrap();
    }
    // Sans témoins (éclipse) : refusée.
    assert!(matches!(
        vues[A].recevoir_tete(&cogs[C].cog, &h1, None, t1),
        Err(ErreurFederation::TemoinsInsuffisants { requis: 3, obtenu: 0 })
    ));
    diffuser(&mut vues, |v| v.recevoir_tete(&cogs[C].cog, &h1_temoignee, None, t1).map(|_| ()));

    // Avant la sanction, un passeport émis par C est accepté chez A.
    let p = passeport(&cogs[C], &cogs[A], t1);
    verifier_chez(&vues[A], &cogs[A], &cogs[C], &p, t1 + 20).unwrap();

    let t2 = t1 + 30;
    let h2 = cogs[C].tete(1, 5, [0x22; 32], t2);
    // Un témoin qui a vu H1 refuse de contresigner H2 et obtient la preuve.
    assert!(matches!(
        vues[0].temoigner(&cogs[C].cog, &h2, None, &cogs[0].journal, t2),
        Err(ErreurFederation::Equivoque(_))
    ));
    let mut h2_temoignee = h2.clone();
    for w in [3, A, B] {
        h2_temoignee = vues[w].temoigner(&cogs[C].cog, &h2_temoignee, None, &cogs[w].journal, t2).unwrap();
    }
    let preuve = match vues[A].recevoir_tete(&cogs[C].cog, &h2_temoignee, None, t2) {
        Err(ErreurFederation::Equivoque(p)) => *p,
        autre => panic!("équivoque attendue : {autre:?}"),
    };
    assert_eq!(preuve, Preuve::equivoque(h1_temoignee.clone(), h2_temoignee.clone()));
    // Note locale : C banni chez A (faute grave observée), indépendamment de la blacklist.
    assert!(vues[A].note().est_banni(&cogs[C].identite.kid(), t2));

    // ── Accusation : A ouvre, 5 membres vérifient la preuve et cosignent ──
    let t3 = t2 + 60;
    let mut accusation = vues[A].accuser(&cogs[C].cog, &preuve, &cogs[A].identite, t3).unwrap();
    for (i, m) in [0usize, 1, 2, 3, B].iter().enumerate() {
        if i == 4 {
            // 5 signatures : quorum non atteint, aucun effet.
            let mut essai = vues[B].clone();
            assert!(matches!(
                essai.recevoir_accusation(&accusation, &preuve, t3),
                Err(ErreurFederation::QuorumNonAtteint { requis: 6, obtenu: 5 })
            ));
        }
        accusation = vues[*m].contresigner_accusation(&accusation, &preuve, &cogs[*m].identite, t3).unwrap();
    }
    // Le fautif ne peut pas cosigner sa propre accusation.
    assert!(vues[C].contresigner_accusation(&accusation, &preuve, &cogs[C].identite, t3).is_err());

    let mut suspensions = Vec::new();
    for v in &mut vues {
        suspensions.push(v.recevoir_accusation(&accusation, &preuve, t3).unwrap());
    }
    let suspension = suspensions[A].clone();
    assert_eq!(suspension.cog, cogs[C].cog);
    assert_eq!(suspension.echeance(), t3 + DELAI_CONTESTATION_SECS);
    assert_eq!(QUORUM_BLACKLIST, 6);
    for v in &vues {
        // Suspension immédiate (A-007) : pas encore de révocation.
        assert_eq!(
            v.statut(&cogs[C].cog),
            Some(Statut::Suspendu { depuis: t3, accusation: suspension.accusation })
        );
        assert_eq!(v.revocations().count(), 0);
        // Les autres restent membres.
        assert_eq!(v.statut(&cogs[A].cog), Some(Statut::Membre));
        assert_eq!(v.statut(&cogs[B].cog), Some(Statut::Membre));
    }

    // ── Pendant la suspension, les autres ne font plus confiance à C ──
    // Passeport émis par C : refusé chez A via la source de révocation.
    let t4 = t3 + 30;
    assert!(matches!(
        verifier_chez(&vues[A], &cogs[A], &cogs[C], &p, t4),
        Err(RefusIdentite::Revoque(CibleRevocation::Serie(_)))
    ));
    assert!(matches!(vues[B].acces_federe(&cogs[C].cog, t4), Err(ErreurFederation::Suspendu)));
    let h3 = cogs[C].tete(2, 5, [0x11; 32], t4);
    assert!(matches!(vues[B].recevoir_tete(&cogs[C].cog, &h3, None, t4), Err(ErreurFederation::Suspendu)));
    let c = cogs[C].confirmer(&cogs[A], t4);
    assert!(matches!(vues[B].recevoir_confirmation(&c, t4), Err(ErreurFederation::Suspendu)));
    let mut carnet = CarnetPairs::new();
    for (i, x) in cogs.iter().enumerate() {
        carnet.noter_contact(&x.cog, if i % 2 == 0 { "reseau-pair" } else { "reseau-impair" }, T0);
    }
    let choisis = carnet.choisir(10, &vues[A], t4);
    assert_eq!(choisis.len(), 6);
    assert!(!choisis.contains(&cogs[C].cog));

    // ── Fraîcheur (EX-35) : sans tête témoignée récente, accès fédéré refusé ──
    let pb = passeport(&cogs[B], &cogs[A], t4);
    verifier_chez(&vues[A], &cogs[A], &cogs[B], &pb, t4).unwrap();
    let tard = t1 + 16 * 60;
    assert!(matches!(
        verifier_chez(&vues[A], &cogs[A], &cogs[B], &pb, tard),
        Err(RefusIdentite::Revocation(ErreurRevocation::Perimee { .. }))
    ));
    assert!(matches!(vues[A].acces_federe(&cogs[B].cog, tard), Err(ErreurFederation::VuePerimee)));
    // Une tête témoignée fraîche de B rétablit la vue.
    let mut hb = cogs[B].tete(1, 3, [0x33; 32], tard);
    for w in 0..3 {
        hb = vues[w].temoigner(&cogs[B].cog, &hb, None, &cogs[w].journal, tard).unwrap();
    }
    vues[A].recevoir_tete(&cogs[B].cog, &hb, None, tard).unwrap();
    assert!(matches!(vues[A].acces_federe(&cogs[B].cog, tard), Ok(Statut::Membre)));

    // ── 72 h sans recours accepté : révocation définitive ──
    let t5 = suspension.echeance();
    // Les fondateurs renouvellent leurs confirmations de A et B (24 h).
    for f in &cogs[..4] {
        for x in [&cogs[A], &cogs[B]] {
            let c = f.confirmer(x, t5 - 60);
            diffuser(&mut vues, |v| v.recevoir_confirmation(&c, t5 - 60));
        }
    }
    let mut revocations = Vec::new();
    for v in &mut vues {
        assert_eq!(v.actualiser(t5 - 1).unwrap().len(), 0);
        let mut d = v.actualiser(t5).unwrap();
        assert_eq!(d.len(), 1);
        revocations.push(d.remove(0));
    }
    let rev = &revocations[A];
    assert_eq!(rev.cog, cogs[C].cog);
    assert_eq!(rev.accusation, suspension.accusation);
    assert_eq!(rev.sanction, Sanction::RevocationIdentite);
    for v in &vues {
        assert_eq!(v.statut(&cogs[C].cog), Some(Statut::Revoque));
        assert_eq!(v.statut(&cogs[A].cog), Some(Statut::Membre));
        assert_eq!(v.statut(&cogs[B].cog), Some(Statut::Membre));
    }
    // Confirmateurs responsables : pénalisés, sans bannissement.
    assert_eq!(vues[A].note().points(&cogs[0].identite.kid(), t5), 25);
    assert!(!vues[A].note().est_banni(&cogs[0].identite.kid(), t5));

    // ── Entrée de registre produite : Revocation { cible: Cle(kid) } ──
    let entree = rev.sceller_entree(&cogs[A].journal).unwrap();
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cogs[A].journal.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let lue = verifier::<EntreeRegistre>(&entree, &ancres, t5).unwrap().into_titre();
    match lue.corps {
        Corps::Revocation { cible: Cible::Cle(kid), motif } => {
            assert_eq!(kid, empreinte_cle(&cogs[C].identite.cle_publique()));
            assert!(motif.starts_with("blacklist federale "));
        }
        autre => panic!("révocation attendue : {autre:?}"),
    }

    // ── Révoqué : refus définitif ──
    let t6 = t5 + 30;
    let p2 = passeport(&cogs[C], &cogs[A], t6);
    assert!(matches!(
        verifier_chez(&vues[A], &cogs[A], &cogs[C], &p2, t6),
        Err(RefusIdentite::Revoque(CibleRevocation::Serie(_)))
    ));
    assert!(matches!(vues[B].acces_federe(&cogs[C].cog, t6), Err(ErreurFederation::Revoque)));
    let h4 = cogs[C].tete(3, 5, [0x11; 32], t6);
    assert!(matches!(vues[B].recevoir_tete(&cogs[C].cog, &h4, None, t6), Err(ErreurFederation::Revoque)));
    let c = cogs[C].confirmer(&cogs[A], t6);
    assert!(matches!(vues[B].recevoir_confirmation(&c, t6), Err(ErreurFederation::Revoque)));
    let c = cogs[0].confirmer(&cogs[C], t6);
    assert!(matches!(vues[B].recevoir_confirmation(&c, t6), Err(ErreurFederation::Revoque)));
    // Même racine : retour seulement par nouvelle identité.
    let recandidature = CogTest::nouveau(7, t6).candidature;
    assert!(matches!(vues[B].recevoir_candidature(&recandidature, t6), Err(ErreurFederation::Revoque)));
    assert!(matches!(vues[B].recevoir_accusation(&accusation, &preuve, t6), Err(ErreurFederation::Revoque)));
}

#[test]
fn aucune_destruction_a_distance() {
    // Une seule sanction possible : la révocation d'identité.
    assert_eq!(Sanction::TOUTES, [Sanction::RevocationIdentite]);
    // Aucune opération de destruction, d'effacement ou de réinitialisation
    // n'existe dans l'API (contrôle du source).
    let interdits = [
        "detrui", "destruct", "destroy", "wipe", "efface", "erase", "reinitialis", "reset", "kill", "purge",
        "supprimer_donnees", "autodestruction",
    ];
    let dossier = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut lus = 0;
    for e in std::fs::read_dir(dossier).unwrap() {
        let chemin = e.unwrap().path();
        let texte = std::fs::read_to_string(&chemin).unwrap();
        lus += 1;
        for ligne in texte.lines() {
            let l = ligne.trim_start();
            if l.starts_with("//") {
                continue;
            }
            let bas = l.to_lowercase();
            if bas.contains("fn ") || bas.contains("pub ") || bas.contains("enum ") {
                for mot in interdits {
                    assert!(!bas.contains(mot), "{}: « {mot} » dans « {l} »", chemin.display());
                }
            }
        }
    }
    assert!(lus >= 8);
}
