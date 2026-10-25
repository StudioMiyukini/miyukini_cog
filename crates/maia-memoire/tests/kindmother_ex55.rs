//! @id     maia_memoire_tests_kindmother
//! @do     verifier_memoire_kindmother_ex55
//! @role   test
//! @layer  8
//! @human  maia-memoire — EX-55 sur un KindMother réel : une base chiffrée par utilisateur, étiquette `nationale`, cloisonnement, persistance, oubli.

// Vide / non vide : le message d'échec du test suffit.
#![allow(clippy::assert_is_empty)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use maia_memoire::{
    BorneOctets, Branche, Ecrit, EntrepotKindMother, ErreurMemoire, Issue, MemoireMaia, Profil,
    Proposition, Provenance, TypeUnite, Validation, ETIQUETTE,
};

async fn serveur(utilisateurs: &str) -> (String, PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("maia-memoire-km-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).expect("dir");
    std::fs::write(dir.join("maia-utilisateurs"), utilisateurs).expect("utilisateurs");
    let (addr, creds) = kindmother_service::spawn_ephemeral(&dir).await.expect("serveur");
    (addr.to_string(), creds, dir)
}

async fn ouvrir(addr: &str, creds: &Path, utilisateur: &str) -> MemoireMaia<EntrepotKindMother> {
    let e = EntrepotKindMother::connecter(addr, utilisateur, Some(creds)).await.expect("connexion");
    MemoireMaia::ouvrir(e, Arc::new(BorneOctets)).await.expect("ouvrir")
}

fn b(s: &str) -> Branche {
    Branche::parse(s).expect("branche")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ex55_une_memoire_chiffree_par_utilisateur_etiquetee_nationale() {
    let (addr, creds, dir) = serveur("# mémoires MAIA\nalice\nbob\n").await;
    let alice = ouvrir(&addr, &creds, "alice").await;
    let bob = ouvrir(&addr, &creds, "bob").await;

    let Issue::Ecrite(f) = alice
        .utilisateur()
        .ecrire(Ecrit::new(b("travail.cog"), TypeUnite::Fait, "Racine de confiance sur USB", 8))
        .await
        .expect("écrire")
    else {
        panic!("écrite")
    };
    alice.utilisateur().poser_regle(b("style"), "Répondre en français", 9).await.expect("règle");

    // Cloisonnement : rien chez Bob.
    assert!(bob.utilisateur().consulter().await.expect("bob").is_empty());
    // L'opérateur d'Alice ne peut pas ouvrir la base de Bob.
    let cred = kindmother_client::OperatorCredential::load_from_dir("maia-alice", &creds).expect("cred");
    let intrus = kindmother_client::KindMotherClient::connect(&addr, &cred, "maia-bob").await.expect("connexion");
    assert!(intrus.select(kindmother_client::TypedSelect::from("versions")).await.is_err());
    // Utilisateur non déclaré : aucun justificatif, aucune base.
    assert!(EntrepotKindMother::connecter(&addr, "carol", Some(&creds)).await.is_err());
    assert!(EntrepotKindMother::connecter(&addr, "../bob", Some(&creds)).await.is_err());

    // Chiffrement : fichier sans en-tête SQLite en clair.
    let fichier = dir.join("maia-alice.db");
    assert!(fichier.exists(), "base par utilisateur : {fichier:?}");
    assert!(!kindmother_service::is_plaintext_sqlite(&fichier), "base chiffrée");
    let octets = std::fs::read(&fichier).expect("lecture");
    assert!(!octets.windows(10).any(|w| w == b"Racine de "), "aucun contenu en clair");

    // Étiquette.
    let rows = intrus_meta(&addr, &creds).await;
    assert_eq!(rows, Some(ETIQUETTE.as_str().to_string()));
    assert_eq!(alice.utilisateur().exporter().await.expect("export").etiquette, ETIQUETTE);
    let bloc = alice.assembler("usb", Profil::Petit.budget()).await.expect("bloc");
    assert_eq!(bloc.etiquette, ETIQUETTE);
    assert!(bloc.texte.contains("N0 Répondre en français."));
    assert!(bloc.texte.contains("travail.cog.f1 Racine de confiance sur USB"));

    // Persistance et versions après reconnexion ; validation, oubli.
    drop(alice);
    let alice = ouvrir(&addr, &creds, "alice").await;
    alice.utilisateur().corriger(&f, "Racine de confiance sur deux clés USB", None).await.expect("corriger");
    assert_eq!(alice.utilisateur().historique(&f).await.expect("h").len(), 2);
    let k = alice
        .proposer(Proposition {
            cible: None,
            branche: b("travail.veille"),
            type_unite: TypeUnite::Fait,
            texte: "Fiche fédérée".into(),
            importance: 3,
            confiance: 9,
            provenance: Provenance::Etranger,
            liens: vec![],
            sources: vec![],
        })
        .await
        .expect("proposer");
    assert_eq!(alice.utilisateur().consulter().await.expect("c").len(), 2);
    let v = alice.utilisateur().valider(k, Validation::Utilisateur).await.expect("valider");
    assert_eq!(v.as_str(), "travail.veille.f1");
    let tombes = alice.utilisateur().oublier("travail.cog", "demande").await.expect("oublier");
    assert_eq!(tombes[0].versions_effacees, 2);
    assert!(matches!(alice.utilisateur().historique(&f).await, Err(ErreurMemoire::NoteOubliee(_))));
    let notes = alice.utilisateur().consulter().await.expect("c");
    assert_eq!(notes.len(), 2);
    assert!(notes.iter().all(|n| n.id.domaine() != "travail" || n.id.as_str() == "travail.veille.f1"));
    let _ = std::fs::remove_dir_all(&dir);
}

/// Étiquette lue directement dans la base d'Alice (inspection).
async fn intrus_meta(addr: &str, creds: &Path) -> Option<String> {
    let cred = kindmother_client::OperatorCredential::load_from_dir("maia-alice", creds).expect("cred");
    let c = kindmother_client::KindMotherClient::connect(addr, &cred, "maia-alice").await.expect("connexion");
    let rows = c
        .select(kindmother_client::TypedSelect::from("meta").filter("cle", "etiquette"))
        .await
        .expect("meta");
    rows.first().and_then(|r| r.get("valeur")).and_then(|v| v.as_str()).map(str::to_string)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ex55_etiquette_autre_que_nationale_refusee() {
    let (addr, creds, dir) = serveur("dave\n").await;
    let cred = kindmother_client::OperatorCredential::load_from_dir("maia-dave", &creds).expect("cred");
    let c = kindmother_client::KindMotherClient::connect(&addr, &cred, "maia-dave").await.expect("connexion");
    // Base préparée une première fois, puis étiquette altérée par un opérateur fautif.
    drop(ouvrir(&addr, &creds, "dave").await);
    c.submit(
        kindmother_client::TypedWriteIntent::update("meta")
            .key("cle", "etiquette")
            .value("valeur", "publiable"),
    )
    .await
    .expect("altération");
    let e = EntrepotKindMother::connecter(&addr, "dave", Some(&creds)).await.expect("connexion");
    let r = MemoireMaia::ouvrir(e, Arc::new(BorneOctets)).await;
    assert!(matches!(r, Err(ErreurMemoire::EtiquetteInvalide(v)) if v == "publiable"));
    let _ = std::fs::remove_dir_all(&dir);
}
