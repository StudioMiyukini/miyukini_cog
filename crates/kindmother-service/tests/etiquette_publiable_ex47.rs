//! Étiquette `publiable` (EX-47) de bout en bout sur le serveur TCP réel : posée et retirée
//! seulement par une intention signée par l'utilisateur, refusée sur une donnée `nationale`,
//! tracée dans le journal chaîné, lue par une requête typée.
//!
//! @id     kindmother_service_tests_etiquette_publiable
//! @do     verifier_etiquette_publiable_ex47
//! @role   test
//! @layer  core
//! @human  kindmother-service — tests EX-47 : étiquette publiable posée / retirée par l'utilisateur seul.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use ed25519_dalek::{Signer, SigningKey};
use kindmother::etiquette::{
    encode_hex, ActionEtiquette, AudiencePublication, IntentionEtiquette, ParametresPublication, RefDonnee, Sensibilite,
    TypePublication, DOMAINE_CONTENU, VERSION_INTENTION,
};
use kindmother::intent::{SchemaMigration, TypedWriteIntent};
use kindmother_client::{ClientError, KindMotherClient, OperatorCredential};
use kindmother_service::audit::{AuditJournal, AuditKind};
use kindmother_service::KindMotherServer;
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;

const TEXTE: &str = "Paris est la capitale de la France.";

fn alice() -> SigningKey {
    SigningKey::from_bytes(&[7; 32])
}

fn preparer_dossier(cle_utilisateur: &[u8; 32]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("km-ex47-{}", uuid::Uuid::new_v4()));
    let utilisateurs = dir.join("credentials").join("utilisateurs");
    std::fs::create_dir_all(&utilisateurs).expect("dossier");
    std::fs::write(utilisateurs.join("alice.pub"), format!("{}\n", encode_hex(cle_utilisateur))).expect("clé");
    // Bases dont Alice est propriétaire (M5) ; jay1tribu n'en fait pas partie.
    std::fs::write(utilisateurs.join("alice.bases"), "# bases d'Alice\njaykonta\njaykoa\nmaia-alice\n").expect("bases");
    std::fs::write(dir.join("maia-utilisateurs"), "alice\n").expect("maia");
    std::fs::write(dir.join("bases-nationales"), "# déclarée nationale\njaykoa\n").expect("nationales");
    dir
}

async fn demarrer(dir: &Path) -> (SocketAddr, [u8; 32]) {
    let server = KindMotherServer::new(dir).await.expect("server");
    let cle_serveur = server.server_public_key();
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(server.serve_listener(listener));
    (addr, cle_serveur)
}

async fn client(addr: SocketAddr, dir: &Path, operateur: &str) -> KindMotherClient {
    let cred = OperatorCredential::load_from_dir(operateur, dir.join("credentials")).expect("cred");
    KindMotherClient::connect(&addr.to_string(), &cred, operateur).await.expect("connect")
}

fn empreinte(texte: &str) -> String {
    let mut h = Sha256::new();
    h.update(DOMAINE_CONTENU);
    h.update(texte.as_bytes());
    encode_hex(&h.finalize())
}

fn maintenant() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("horloge").as_secs()
}

fn donnee(base: &str, id: &str) -> RefDonnee {
    RefDonnee::new(base, "notes", "body").cle("id", id)
}

/// Intention signée par `cle` (l'utilisateur légitime ou un imposteur).
fn signee(
    cle: &SigningKey,
    cle_serveur: &[u8; 32],
    action: ActionEtiquette,
    donnee: RefDonnee,
    empreinte_contenu: Option<String>,
    horodatage: u64,
) -> IntentionEtiquette {
    let parametres = (action == ActionEtiquette::Poser).then(|| ParametresPublication {
        type_fiche: TypePublication::Fait,
        langue: "fr".into(),
        licence: "CC-BY-4.0".into(),
        audience: AudiencePublication::Membres,
    });
    let mut i = IntentionEtiquette {
        version: VERSION_INTENTION,
        parametres,
        action,
        donnee,
        utilisateur: "alice".into(),
        empreinte_contenu,
        horodatage,
        cle_publique: encode_hex(cle.verifying_key().as_bytes()),
        signature: "00".repeat(64),
    };
    let m = i.message_signe(cle_serveur).expect("message");
    i.signature = encode_hex(&cle.sign(&m).to_bytes());
    i
}

async fn table_notes(c: &KindMotherClient, base: &str) {
    let m = vec![SchemaMigration::new(1, "notes", &["CREATE TABLE IF NOT EXISTS notes (id TEXT PRIMARY KEY, body TEXT NOT NULL)"])];
    c.apply_migrations(m).await.expect("migration");
    for (id, body) in [("n1", TEXTE), ("n2", "Note privée : code 1234.")] {
        c.submit(TypedWriteIntent::insert("notes").database(base).value("id", id).value("body", body)).await.expect("insert");
    }
}

fn refus(r: Result<kindmother::etiquette::EtatEtiquette, ClientError>) -> String {
    match r {
        Err(ClientError::PermissionDenied(m) | ClientError::InvalidRequest(m)) => m,
        autre => panic!("refus attendu, obtenu {autre:?}"),
    }
}

#[tokio::test]
async fn posee_par_l_utilisateur_lue_puis_retiree() {
    let dir = preparer_dossier(alice().verifying_key().as_bytes());
    let (addr, srv) = demarrer(&dir).await;
    let jk = client(addr, &dir, "jaykonta").await;
    table_notes(&jk, "jaykonta").await;
    let t = maintenant();

    // Non étiquetée : pas publiable.
    let etat = jk.lire_etiquette(donnee("jaykonta", "n1")).await.expect("lire");
    assert!(!etat.posee && !etat.publiable);
    assert_eq!(etat.sensibilite, Sensibilite::Ordinaire);
    assert_eq!(etat.empreinte_actuelle.as_deref(), Some(empreinte(TEXTE).as_str()));

    // Posée par l'utilisateur (relayée par l'opérateur) : publiable.
    let pose = signee(&alice(), &srv, ActionEtiquette::Poser, donnee("jaykonta", "n1"), Some(empreinte(TEXTE)), t);
    let etat = jk.etiqueter(pose.clone()).await.expect("poser");
    assert!(etat.posee && etat.publiable);
    assert_eq!(etat.utilisateur.as_deref(), Some("alice"));
    assert!(etat.seq.is_some());
    // Paramètres signés par l'utilisateur et identité de la donnée rendus (M1).
    assert_eq!(etat.parametres.as_ref().map(|p| p.licence.as_str()), Some("CC-BY-4.0"));
    assert_eq!(etat.empreinte_donnee.as_ref().map(String::len), Some(64));
    // Une autre donnée de la même table reste non publiable.
    assert!(!jk.lire_etiquette(donnee("jaykonta", "n2")).await.expect("lire").publiable);

    // Rejeu de la même intention : refusé.
    assert!(refus(jk.etiqueter(pose.clone()).await).contains("replay"));

    // Retrait : plus publiable ; la pose retenue ne se rejoue pas, ni une pose antidatée.
    let retrait = signee(&alice(), &srv, ActionEtiquette::Retirer, donnee("jaykonta", "n1"), None, t + 1);
    let etat = jk.etiqueter(retrait).await.expect("retirer");
    assert!(!etat.posee && !etat.publiable);
    assert!(jk.etiqueter(pose).await.is_err());
    let antidatee = signee(&alice(), &srv, ActionEtiquette::Poser, donnee("jaykonta", "n1"), Some(empreinte(TEXTE)), t - 1);
    assert!(refus(jk.etiqueter(antidatee).await).contains("timestamp"));
    assert!(!jk.lire_etiquette(donnee("jaykonta", "n1")).await.expect("lire").publiable);

    // Nouvelle pose explicite : publiable de nouveau ; contenu modifié ensuite ⇒ non publiable.
    let pose2 = signee(&alice(), &srv, ActionEtiquette::Poser, donnee("jaykonta", "n1"), Some(empreinte(TEXTE)), t + 2);
    assert!(jk.etiqueter(pose2).await.expect("poser").publiable);
    jk.submit(TypedWriteIntent::update("notes").key("id", "n1").value("body", "Texte remplacé.")).await.expect("update");
    let etat = jk.lire_etiquette(donnee("jaykonta", "n1")).await.expect("lire");
    assert!(etat.posee && !etat.publiable, "contenu changé depuis la pose");

    // Journal chaîné : poses, retrait et refus agrégés par opérateur (M6) ; le
    // premier refus est écrit aussitôt, les deux suivants juste après la pose suivante.
    // Les lectures autorisées ne sont pas journalisées.
    drop(jk);
    let journal = AuditJournal::open(&dir, journal_key(&dir).as_deref()).expect("journal");
    journal.verify().expect("chaîne");
    let entrees: Vec<_> = journal.entries().expect("entrées").into_iter().filter(|e| e.record.kind == AuditKind::Etiquette).collect();
    let verdicts: Vec<&str> = entrees.iter().map(|e| e.record.verdict.as_str()).collect();
    assert_eq!(verdicts, ["posee", "refusee", "retiree", "posee", "refusee"]);
    assert!(entrees[1].record.detail.starts_with("refus=1;"), "{}", entrees[1].record.detail);
    assert!(entrees[4].record.detail.starts_with("refus=2;"), "{}", entrees[4].record.detail);
    assert!(entrees.iter().all(|e| e.record.operation != "publiable.lire"));
    assert!(entrees[0].record.detail.starts_with("v2;"));
    assert!(entrees.iter().all(|e| e.record.operator_id == "jaykonta"));
}

#[tokio::test]
async fn refus_nationale_contenu_et_autres_chemins() {
    let dir = preparer_dossier(alice().verifying_key().as_bytes());
    let (addr, srv) = demarrer(&dir).await;
    let jk = client(addr, &dir, "jaykonta").await;
    table_notes(&jk, "jaykonta").await;
    let t = maintenant();
    let d = || donnee("jaykonta", "n1");

    // Empreinte qui ne correspond pas au contenu courant.
    let faux = signee(&alice(), &srv, ActionEtiquette::Poser, d(), Some(empreinte("Autre texte.")), t);
    assert!(refus(jk.etiqueter(faux).await).contains("digest"));

    // Autres chemins que l'utilisateur : clé d'opérateur, clé inconnue, intention altérée,
    // autre serveur, horodatage hors fenêtre.
    let graine_op = std::fs::read_to_string(dir.join("credentials").join("jaykonta.key")).expect("graine");
    let graine_op = kindmother::etiquette::decode_hex::<32>(graine_op.trim()).expect("hex");
    let operateur = SigningKey::from_bytes(&graine_op);
    let imposteur = SigningKey::from_bytes(&[9; 32]);
    let mut alteree = signee(&alice(), &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t);
    alteree.donnee = donnee("jaykonta", "n2");
    let mut cle_alice_signee_par_imposteur = signee(&imposteur, &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t);
    cle_alice_signee_par_imposteur.cle_publique = encode_hex(alice().verifying_key().as_bytes());
    for mauvaise in [
        signee(&operateur, &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t),
        signee(&imposteur, &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t),
        cle_alice_signee_par_imposteur,
        alteree,
        signee(&alice(), &[0; 32], ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t),
        signee(&alice(), &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t - 3_600),
    ] {
        jk.etiqueter(mauvaise).await.expect_err("chemin autre que l'utilisateur");
    }
    assert!(!jk.lire_etiquette(d()).await.expect("lire").posee);

    // Données `nationale` : mémoire de MAIA (maia-*) et base déclarée.
    let maia = client(addr, &dir, "maia-alice").await;
    table_notes(&maia, "maia-alice").await;
    let n = signee(&alice(), &srv, ActionEtiquette::Poser, donnee("maia-alice", "n1"), Some(empreinte(TEXTE)), t);
    assert!(refus(maia.etiqueter(n).await).contains("nationale"));
    let etat = maia.lire_etiquette(donnee("maia-alice", "n1")).await.expect("lire");
    assert_eq!(etat.sensibilite, Sensibilite::Nationale);
    assert!(!etat.publiable);
    let koa = client(addr, &dir, "jaykoa").await;
    table_notes(&koa, "jaykoa").await;
    let n = signee(&alice(), &srv, ActionEtiquette::Poser, donnee("jaykoa", "n1"), Some(empreinte(TEXTE)), t);
    assert!(refus(koa.etiqueter(n).await).contains("nationale"));

    // Un opérateur sans accès à la base ne pose ni ne lit l'étiquette.
    let pose = signee(&alice(), &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t + 1);
    assert!(matches!(maia.etiqueter(pose.clone()).await, Err(ClientError::PermissionDenied(_))));
    assert!(matches!(maia.lire_etiquette(d()).await, Err(ClientError::PermissionDenied(_))));
    // L'utilisateur, relayé par l'opérateur légitime : accepté.
    assert!(jk.etiqueter(pose).await.expect("poser").publiable);

    // M5 : base dont l'utilisateur n'est pas propriétaire (absente de alice.bases) : refus,
    // pour poser comme pour retirer, malgré une signature valide.
    let tribu = client(addr, &dir, "jay1tribu").await;
    table_notes(&tribu, "jay1tribu").await;
    let p = signee(&alice(), &srv, ActionEtiquette::Poser, donnee("jay1tribu", "n1"), Some(empreinte(TEXTE)), t + 2);
    assert!(refus(tribu.etiqueter(p).await).contains("not owned"));
    let r = signee(&alice(), &srv, ActionEtiquette::Retirer, donnee("jay1tribu", "n1"), None, t + 3);
    assert!(refus(tribu.etiqueter(r).await).contains("not owned"));

    // M1 : intention v1 refusée (version absente ou ≠ 2).
    let mut v1 = signee(&alice(), &srv, ActionEtiquette::Poser, d(), Some(empreinte(TEXTE)), t + 4);
    v1.version = 1;
    assert!(jk.etiqueter(v1).await.is_err());
}

#[tokio::test]
async fn cle_utilisateur_egale_a_une_cle_de_service_refus_de_demarrer() {
    // Démarrage initial : provisionne les clés des opérateurs.
    let dir = preparer_dossier(alice().verifying_key().as_bytes());
    drop(KindMotherServer::new(&dir).await.expect("server"));
    // L'administrateur dépose par erreur la clé de l'opérateur de MAIA comme clé d'utilisateur.
    let pub_maia = std::fs::read_to_string(dir.join("credentials").join("maia-alice.pub")).expect("pub");
    std::fs::write(dir.join("credentials").join("utilisateurs").join("alice.pub"), pub_maia).expect("écrire");
    assert!(KindMotherServer::new(&dir).await.is_err());
}

/// Clé du journal (le test joue le rôle d'un inspecteur disposant du magasin de clés).
fn journal_key(dir: &Path) -> Option<String> {
    #[cfg(feature = "db-encryption")]
    {
        let store = kindmother_service::DatabaseKeyStore::open(dir).expect("store");
        Some(store.pragma_key("_journal", true).expect("key").0.to_string())
    }
    #[cfg(not(feature = "db-encryption"))]
    {
        let _ = dir;
        None
    }
}
