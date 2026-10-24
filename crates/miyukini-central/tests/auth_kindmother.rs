//! @id     miyukini_central_tests_auth_kindmother
//! @do     verifier_auth_central_via_kindmother
//! @role   test
//! @layer  infra
//! @human  miyukini-central — tests d'intégration de l'auth Central sur un KindMother éphémère (K-13).

//! Tests d'intégration du chemin `kindmother-only` (défaut) : un serveur KindMother
//! éphémère est démarré en processus, Central s'y connecte comme opérateur
//! `miyukini-central` (Admin de la base `central`).

#![cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]

use kindmother_client::{KindMotherClient, OperatorCredential, TypedSelect, TypedWriteIntent};
use miyukini_central::auth::{CentralAuthDb, CentralProfile};
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::path::PathBuf;

const PW: &str = "Str0ng!Passw0rd#2026";

struct Env {
    addr: SocketAddr,
    creds: PathBuf,
    db: CentralAuthDb,
}

fn env(tag: &str) -> Env {
    let tmp = std::env::temp_dir().join(format!("miyu-central-km-{tag}-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&tmp).unwrap();
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(&tmp).expect("serveur éphémère");
    let cred = OperatorCredential::load_from_dir("miyukini-central", &creds).expect("justificatif");
    let db = CentralAuthDb::connect_with(&addr.to_string(), &cred).expect("connexion");
    Env { addr, creds, db }
}

/// Client brut (même opérateur) pour inspecter/préparer la base dans les tests.
fn raw<T>(
    env: &Env,
    f: impl FnOnce(KindMotherClient) -> std::pin::Pin<Box<dyn std::future::Future<Output = T>>>,
) -> T {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let cred = OperatorCredential::load_from_dir("miyukini-central", &env.creds).unwrap();
        let client = KindMotherClient::connect(&env.addr.to_string(), &cred, "central")
            .await
            .unwrap();
        f(client).await
    })
}

#[test]
fn migrations_idempotentes() {
    let e = env("migr");
    e.db.init_schema_sync().expect("réapplication");
    let cred = OperatorCredential::load_from_dir("miyukini-central", &e.creds).unwrap();
    CentralAuthDb::connect_with(&e.addr.to_string(), &cred).expect("seconde connexion");
    assert!(e.db.health_check().unwrap());
}

#[test]
fn inscription_connexion_et_premier_admin() {
    let e = env("signup");
    assert!(e.db.is_cog_virgin().unwrap());
    let a =
        e.db.sign_up("  a@example.com ", PW, Some("  Alice "))
            .unwrap();
    assert!(a.is_admin);
    assert_eq!(a.email, "a@example.com");
    assert_eq!(a.pseudonyme.as_deref(), Some("Alice"));
    assert!(!e.db.is_cog_virgin().unwrap());
    let b = e.db.sign_up("b@example.com", PW, Some("   ")).unwrap();
    assert!(!b.is_admin);
    assert_eq!(b.pseudonyme, None);

    // Règles d'entrée.
    assert!(e.db.sign_up("", PW, None).is_err());
    assert!(e.db.sign_up("c@example.com", "faible", None).is_err());
    assert!(
        e.db.sign_up("a@example.com", PW, None).is_err(),
        "email unique"
    );

    // Connexion.
    let p =
        e.db.sign_in("a@example.com", PW)
            .unwrap()
            .expect("connexion");
    assert_eq!(p.id, a.id);
    assert!(p.is_admin);
    assert!(e
        .db
        .sign_in("a@example.com", "Mauvais!Passw0rd")
        .unwrap()
        .is_none());
    assert!(e.db.sign_in("inconnu@example.com", PW).unwrap().is_none());
    assert!(e.db.sign_in("   ", PW).unwrap().is_none());

    let admins =
        e.db.list_profiles()
            .unwrap()
            .iter()
            .filter(|p| p.is_admin)
            .count();
    assert_eq!(admins, 1);
}

#[test]
fn premier_admin_unique_concurrent() {
    let e = env("conc");
    let db = std::sync::Arc::new(e.db.clone());
    let handles: Vec<_> = (0..6)
        .map(|i| {
            let db = db.clone();
            std::thread::spawn(move || db.sign_up(&format!("u{i}@example.com"), PW, None).unwrap())
        })
        .collect();
    let admins = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|p| p.is_admin)
        .count();
    assert_eq!(admins, 1);
    assert_eq!(e.db.list_profiles().unwrap().len(), 6);
}

#[test]
fn hash_sha256_legacy_migre_vers_argon2() {
    let e = env("upgrade");
    let p = e.db.sign_up("old@example.com", PW, None).unwrap();
    let legacy = format!("{:x}", Sha256::digest(PW.as_bytes()));
    let id = p.id.clone();
    raw(&e, move |c| {
        Box::pin(async move {
            c.submit(
                TypedWriteIntent::update("central_profiles")
                    .key("id", id.as_str())
                    .value("password_hash", legacy)
                    .purpose("test: hash SHA-256 hérité"),
            )
            .await
            .unwrap();
        })
    });
    assert!(e
        .db
        .sign_in("old@example.com", "Mauvais!Passw0rd")
        .unwrap()
        .is_none());
    assert!(e.db.sign_in("old@example.com", PW).unwrap().is_some());
    let id = p.id.clone();
    let stored = raw(&e, move |c| {
        Box::pin(async move {
            let rows = c
                .select(
                    TypedSelect::from("central_profiles")
                        .columns(["password_hash"])
                        .filter("id", id.as_str()),
                )
                .await
                .unwrap();
            rows[0]["password_hash"].as_str().unwrap().to_string()
        })
    });
    assert!(stored.starts_with("$argon2"), "hash migré : {stored}");
    assert!(e.db.sign_in("old@example.com", PW).unwrap().is_some());
}

#[test]
fn profil_lecture_et_mise_a_jour() {
    let e = env("profile");
    let p = e.db.sign_up("p@example.com", PW, None).unwrap();
    assert!(e.db.get_profile("inconnu").unwrap().is_none());
    let mut edit: CentralProfile = e.db.get_profile(&p.id).unwrap().unwrap();
    assert_eq!(edit.email, "p@example.com");
    edit.email = "p2@example.com".into();
    edit.nom = Some("Dupont".into());
    edit.prenom = Some(String::new());
    edit.ville = Some("Lyon".into());
    edit.genre = Some("Neutre".into());
    edit.enfants_nombre = Some(2);
    edit.enfants_noms = Some("Léa, Tom".into());
    edit.langue_maternelle = Some("fr".into());
    e.db.update_profile(&edit).unwrap();
    let got = e.db.get_profile(&p.id).unwrap().unwrap();
    assert_eq!(got.email, "p2@example.com");
    assert_eq!(got.nom.as_deref(), Some("Dupont"));
    assert_eq!(got.prenom.as_deref(), Some(""), "chaîne vide conservée");
    assert_eq!(got.ville.as_deref(), Some("Lyon"));
    assert_eq!(got.genre.as_deref(), Some("Neutre"));
    assert_eq!(got.enfants_nombre, Some(2));
    assert_eq!(got.enfants_noms.as_deref(), Some("Léa, Tom"));
    assert_eq!(got.langue_maternelle.as_deref(), Some("fr"));
    assert_eq!(got.telephone, None);
    assert!(got.is_admin);
    // Le login suit l'email modifié.
    assert!(e.db.sign_in("p2@example.com", PW).unwrap().is_some());
    assert!(e.db.sign_in("p@example.com", PW).unwrap().is_none());
    let list = e.db.list_profiles().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].email, "p2@example.com");
}

#[test]
fn reattribution_d_id_deplace_les_lignes_liees() {
    let e = env("reid");
    let p = e.db.sign_up("r@example.com", PW, Some("Rémi")).unwrap();
    let save =
        e.db.insert_profile_save(&p.id, "jaykoa", 0, b"etat")
            .unwrap();
    e.db.set_profile_service_ref(&p.id, "jaykoa", Some(&save))
        .unwrap();
    let hash = "$argon2id$v=19$m=19456,t=2,p=1$c2VsMTIzNDU2Nzg$aGFzaGhhc2hoYXNoaGFzaA";
    e.db.set_connect_password_hash(&p.id, hash).unwrap();
    e.db.set_current_profile_id(Some(&p.id)).unwrap();

    let new_id = uuid::Uuid::new_v4().to_string();
    e.db.update_profile_id(&p.id, &new_id).unwrap();

    assert!(e.db.get_profile(&p.id).unwrap().is_none());
    let moved = e.db.get_profile(&new_id).unwrap().unwrap();
    assert_eq!(moved.email, "r@example.com");
    assert_eq!(moved.pseudonyme.as_deref(), Some("Rémi"));
    assert!(moved.is_admin);
    assert_eq!(
        e.db.sign_in("r@example.com", PW).unwrap().unwrap().id,
        new_id
    );
    assert_eq!(
        e.db.get_profile_service_ref(&new_id, "jaykoa").unwrap(),
        Some(save.clone())
    );
    assert_eq!(e.db.get_profile_service_ref(&p.id, "jaykoa").unwrap(), None);
    assert_eq!(
        e.db.get_profile_save(&save).unwrap().unwrap().profile_id,
        new_id
    );
    assert_eq!(e.db.list_profile_saves(&new_id, "jaykoa").unwrap().len(), 1);
    assert_eq!(
        e.db.get_connect_password_hash(&new_id).unwrap().as_deref(),
        Some(hash)
    );
    assert_eq!(e.db.get_connect_password_hash(&p.id).unwrap(), None);
    assert_eq!(e.db.get_current_profile_id().unwrap(), Some(new_id.clone()));
    assert_eq!(e.db.list_profiles().unwrap().len(), 1);

    assert!(e.db.update_profile_id("inconnu", "autre").is_err());
    e.db.update_profile_id(&new_id, &new_id)
        .expect("ID inchangé");
}

#[test]
fn etat_vierge_session_et_reinitialisation() {
    let e = env("reset");
    assert!(e.db.is_cog_virgin().unwrap());
    e.db.set_cog_virgin(false).unwrap();
    assert!(!e.db.is_cog_virgin().unwrap());
    e.db.set_cog_virgin_sync(true).unwrap();
    assert!(e.db.is_cog_virgin().unwrap());

    let admin = e.db.sign_up("admin@example.com", PW, None).unwrap();
    let user = e.db.sign_up("user@example.com", PW, None).unwrap();
    assert!(admin.is_admin && !user.is_admin);
    assert_eq!(e.db.get_current_profile_id().unwrap(), None);
    e.db.set_current_profile_id(Some(&user.id)).unwrap();
    e.db.set_current_profile_id(Some(&admin.id)).unwrap();
    assert_eq!(
        e.db.get_current_profile_id().unwrap(),
        Some(admin.id.clone())
    );
    e.db.insert_profile_save(&user.id, "jaykoa", 0, b"x")
        .unwrap();
    e.db.set_profile_service_ref(&user.id, "jaykonta", Some("ref"))
        .unwrap();
    e.db.set_connect_password_hash(&user.id, "$argon2id$v=19$x")
        .unwrap();

    // Refus : non admin, mauvais mot de passe, inconnu — rien n'est supprimé.
    assert!(e.db.reset_cog_to_virgin("user@example.com", PW).is_err());
    assert!(e
        .db
        .reset_cog_to_virgin("admin@example.com", "mauvais")
        .is_err());
    assert!(e.db.reset_cog_to_virgin("inconnu@example.com", PW).is_err());
    assert_eq!(
        e.db.list_profiles().unwrap().len(),
        2,
        "rien ne doit être supprimé"
    );
    assert!(!e.db.is_cog_virgin().unwrap());

    e.db.reset_cog_to_virgin("admin@example.com", PW).unwrap();
    assert!(e.db.list_profiles().unwrap().is_empty());
    assert!(e.db.is_cog_virgin().unwrap());
    assert_eq!(e.db.get_current_profile_id().unwrap(), None);
    assert!(e
        .db
        .list_profile_saves(&user.id, "jaykoa")
        .unwrap()
        .is_empty());
    assert_eq!(
        e.db.get_profile_service_ref(&user.id, "jaykonta").unwrap(),
        None
    );
    assert_eq!(e.db.get_connect_password_hash(&user.id).unwrap(), None);

    // Après réinitialisation, le premier compte redevient admin.
    let again = e.db.sign_up("user@example.com", PW, None).unwrap();
    assert!(again.is_admin);
    assert!(!e.db.is_cog_virgin().unwrap());
    e.db.set_current_profile_id(None).unwrap();
    assert_eq!(e.db.get_current_profile_id().unwrap(), None);
}

#[test]
fn hash_connect() {
    let e = env("connect");
    let p = e.db.sign_up("c@example.com", PW, None).unwrap();
    assert_eq!(e.db.get_connect_password_hash(&p.id).unwrap(), None);
    assert!(e
        .db
        .set_connect_password_hash(&p.id, "sha256-pas-argon")
        .is_err());
    assert!(e
        .db
        .set_connect_password_hash(&p.id, "$argon2i$v=19$x")
        .is_err());
    assert_eq!(e.db.get_connect_password_hash(&p.id).unwrap(), None);
    e.db.set_connect_password_hash(&p.id, "$argon2id$v=19$un")
        .unwrap();
    e.db.set_connect_password_hash(&p.id, "$argon2id$v=19$deux")
        .unwrap();
    assert_eq!(
        e.db.get_connect_password_hash(&p.id).unwrap().as_deref(),
        Some("$argon2id$v=19$deux")
    );
}

#[test]
fn sauvegardes_et_references_de_service() {
    let e = env("saves");
    let p = e.db.sign_up("s@example.com", PW, None).unwrap();
    let binaire: Vec<u8> = (0..=255).collect();
    let s1 =
        e.db.insert_profile_save(&p.id, "jaykoa", 1, &binaire)
            .unwrap();
    let s0 = e.db.insert_profile_save(&p.id, "jaykoa", 0, b"").unwrap();
    e.db.insert_profile_save(&p.id, "jaykonta", 0, b"autre")
        .unwrap();
    assert!(
        e.db.insert_profile_save(&p.id, "jaykoa", 1, b"doublon")
            .is_err(),
        "slot unique"
    );

    let got = e.db.get_profile_save(&s1).unwrap().unwrap();
    assert_eq!(got.data, binaire);
    assert_eq!(got.slot, 1);
    assert_eq!(got.service_key, "jaykoa");
    assert_eq!(got.profile_id, p.id);
    assert_eq!(
        e.db.get_profile_save(&s0).unwrap().unwrap().data,
        Vec::<u8>::new()
    );
    assert!(e.db.get_profile_save("inconnu").unwrap().is_none());
    let by_slot =
        e.db.get_profile_save_by_slot(&p.id, "jaykoa", 0)
            .unwrap()
            .unwrap();
    assert_eq!(by_slot.id, s0);
    assert!(e
        .db
        .get_profile_save_by_slot(&p.id, "jaykoa", 9)
        .unwrap()
        .is_none());

    let list = e.db.list_profile_saves(&p.id, "jaykoa").unwrap();
    assert_eq!(list.iter().map(|r| r.slot).collect::<Vec<_>>(), vec![0, 1]);
    assert_eq!(list[0].id, s0);

    e.db.update_profile_save(&s0, b"nouveau").unwrap();
    assert_eq!(
        e.db.get_profile_save(&s0).unwrap().unwrap().data,
        b"nouveau"
    );
    assert!(e.db.update_profile_save("inconnu", b"x").is_err());

    e.db.delete_profile_save(&s1).unwrap();
    assert!(e.db.get_profile_save(&s1).unwrap().is_none());
    assert!(e.db.delete_profile_save(&s1).is_err(), "déjà supprimée");
    assert_eq!(e.db.list_profile_saves(&p.id, "jaykoa").unwrap().len(), 1);

    // Références de service.
    assert_eq!(e.db.get_profile_service_ref(&p.id, "jaykoa").unwrap(), None);
    e.db.set_profile_service_ref(&p.id, "jaykoa", Some(&s0))
        .unwrap();
    e.db.set_profile_service_ref(&p.id, "jaykoa", Some("autre"))
        .unwrap();
    assert_eq!(
        e.db.get_profile_service_ref(&p.id, "jaykoa")
            .unwrap()
            .as_deref(),
        Some("autre")
    );
    e.db.set_profile_service_ref(&p.id, "jaykoa", None).unwrap();
    assert_eq!(e.db.get_profile_service_ref(&p.id, "jaykoa").unwrap(), None);
}

#[test]
fn appel_depuis_un_runtime_current_thread() {
    let e = env("rt");
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let virgin = rt.block_on(async { e.db.is_cog_virgin() }).unwrap();
    assert!(virgin);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();
    let p = rt
        .block_on(async { e.db.sign_up("rt@example.com", PW, None) })
        .unwrap();
    assert!(p.is_admin);
}
