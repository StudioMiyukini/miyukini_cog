//! Tests de sécurité K-04 (authentification mutuelle des opérateurs) et K-05 (arbitrage SQL).
//!
//! @id     kindmother_service_tests_securite_k04_k05
//! @do     verifier_refus_authentification_et_contournements_sql
//! @role   test
//! @layer  core
//! @human  kindmother-service — tests négatifs (C-60) sur le serveur TCP réel : relais, pin, v1, inondation, inactivité.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ed25519_dalek::{Signer, SigningKey};
use kindmother_client::{ClientError, KindMotherClient, OperatorCredential};
use kindmother_service::{
    client_transcript, server_transcript, KindMotherServer, Request, Response, ServerLimits,
    NONCE_LEN,
};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Démarre un serveur sur un port éphémère de la boucle locale.
async fn start_server() -> (SocketAddr, PathBuf) {
    start_server_with(ServerLimits::default()).await
}

/// Démarre un serveur avec des bornes données.
async fn start_server_with(limits: ServerLimits) -> (SocketAddr, PathBuf) {
    let dir = std::env::temp_dir().join(format!("km-sec-{}", uuid::Uuid::new_v4()));
    let server = KindMotherServer::new(&dir)
        .await
        .expect("server")
        .with_limits(limits);
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(server.serve_listener(listener));
    (addr, dir)
}

/// Justificatif complet (clé de l'opérateur + pin `server.pub`).
fn credential(dir: &Path, operator: &str) -> OperatorCredential {
    OperatorCredential::load_from_dir(operator, dir.join("credentials")).expect("credential")
}

fn unhex<const N: usize>(text: &str) -> [u8; N] {
    let mut out = [0u8; N];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex");
    }
    out
}

fn seed_of(dir: &Path, operator: &str) -> [u8; 32] {
    let text = std::fs::read_to_string(dir.join("credentials").join(format!("{operator}.key")))
        .expect("seed");
    unhex::<32>(text.trim())
}

fn server_pin(dir: &Path) -> [u8; 32] {
    let text = std::fs::read_to_string(dir.join("credentials").join("server.pub")).expect("pin");
    unhex::<32>(text.trim())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

async fn read_frame(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut len = [0u8; 4];
    stream.read_exact(&mut len).await.ok()?;
    let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
    stream.read_exact(&mut buf).await.ok()?;
    Some(buf)
}

async fn write_json(stream: &mut TcpStream, value: &serde_json::Value) {
    let payload = serde_json::to_vec(value).expect("json");
    stream
        .write_all(&(payload.len() as u32).to_be_bytes())
        .await
        .expect("write");
    stream.write_all(&payload).await.expect("write");
}

async fn read_response(stream: &mut TcpStream) -> Option<Response> {
    let frame = read_frame(stream).await?;
    Some(serde_json::from_slice(&frame).expect("response"))
}

/// Défi reçu sur une connexion brute.
struct Handshake {
    stream: TcpStream,
    client_nonce: [u8; NONCE_LEN],
    server_nonce: [u8; NONCE_LEN],
    server_pub: [u8; 32],
}

/// Ouvre une connexion brute, envoie `ClientHello` et retourne le défi (signature vérifiée).
async fn raw_hello(addr: SocketAddr, client_nonce: [u8; NONCE_LEN]) -> Handshake {
    let mut stream = TcpStream::connect(addr).await.expect("connect");
    write_json(
        &mut stream,
        &json!({"type": "ClientHello", "protocol": "kindmother-auth-v2", "client_nonce": hex(&client_nonce)}),
    )
    .await;
    let Some(Response::Challenge {
        protocol,
        server_nonce,
        server_pub,
        server_sig,
    }) = read_response(&mut stream).await
    else {
        panic!("défi attendu");
    };
    assert_eq!(protocol, "kindmother-auth-v2");
    let server_nonce = unhex::<NONCE_LEN>(&server_nonce);
    let server_pub = unhex::<32>(&server_pub);
    let vk = ed25519_dalek::VerifyingKey::from_bytes(&server_pub).expect("vk");
    let sig = ed25519_dalek::Signature::from_bytes(&unhex::<64>(&server_sig));
    vk.verify_strict(
        &server_transcript(&client_nonce, &server_nonce, &server_pub),
        &sig,
    )
    .expect("signature du serveur");
    Handshake {
        stream,
        client_nonce,
        server_nonce,
        server_pub,
    }
}

/// Envoie `Authenticate` avec une signature donnée ; retourne la réponse.
async fn send_auth(stream: &mut TcpStream, operator: &str, signature: &[u8]) -> Option<Response> {
    write_json(
        stream,
        &json!({"type": "Authenticate", "operator_id": operator, "signature": hex(signature)}),
    )
    .await;
    read_response(stream).await
}

/// Signe correctement la transcription de `hs` avec `seed` et s'authentifie.
async fn raw_auth(hs: &mut Handshake, operator: &str, seed: &[u8; 32]) -> Option<Response> {
    let sig = SigningKey::from_bytes(seed).sign(&client_transcript(
        &hs.client_nonce,
        &hs.server_nonce,
        &hs.server_pub,
        operator,
    ));
    send_auth(&mut hs.stream, operator, &sig.to_bytes()).await
}

fn is_error(resp: &Option<Response>, expected: &str) -> bool {
    matches!(resp, Some(Response::Error { code, .. }) if code == expected)
}

fn nonce(b: u8) -> [u8; NONCE_LEN] {
    [b; NONCE_LEN]
}

/// Faux serveur : lit le `ClientHello` et répond avec la trame produite par `challenge`
/// (à partir du nonce client). Retourne son adresse.
async fn fake_server<F>(challenge: F) -> SocketAddr
where
    F: Fn([u8; NONCE_LEN]) -> serde_json::Value + Send + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(async move {
        while let Ok((mut stream, _)) = listener.accept().await {
            let Some(frame) = read_frame(&mut stream).await else {
                continue;
            };
            let hello: serde_json::Value = serde_json::from_slice(&frame).expect("hello");
            let cn = unhex::<NONCE_LEN>(hello["client_nonce"].as_str().expect("nonce"));
            write_json(&mut stream, &challenge(cn)).await;
            // Garde la connexion ouverte : si le client signait, on le verrait ici.
            let _ = read_frame(&mut stream).await;
        }
    });
    addr
}

// ─── K-04 : authentification ────────────────────────────────────────────────

#[tokio::test]
async fn requete_non_authentifiee_refusee() {
    let (addr, dir) = start_server().await;
    let mut stream = TcpStream::connect(addr).await.expect("connect");

    // Requête directe sans s'authentifier, en se déclarant Admin d'une base.
    let req = Request::Query {
        operator_id: "jaykonta".to_string(),
        database: "jaykonta".to_string(),
        sql: "SELECT 1".to_string(),
        params: vec![],
    };
    write_json(&mut stream, &serde_json::to_value(&req).expect("req")).await;
    let resp = read_response(&mut stream).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    // Le serveur ferme la connexion.
    assert!(read_frame(&mut stream).await.is_none());
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn mauvais_secret_refuse() {
    let (addr, dir) = start_server().await;
    let mut hs = raw_hello(addr, nonce(1)).await;
    let resp = raw_auth(&mut hs, "jaykonta", &[42u8; 32]).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    assert!(read_frame(&mut hs.stream).await.is_none());

    // Même refus via le client (pin correct, mauvaise clé d'opérateur)
    let faux = OperatorCredential::from_seed("jaykonta", &[42u8; 32])
        .expect("cred")
        .with_server_pin(&server_pin(&dir))
        .expect("pin");
    let err = KindMotherClient::connect(&addr.to_string(), &faux, "jaykonta")
        .await
        .err();
    assert!(
        matches!(err, Some(ClientError::Authentication(_))),
        "{err:?}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn operateur_inconnu_refuse() {
    let (addr, dir) = start_server().await;
    let mut hs = raw_hello(addr, nonce(2)).await;
    let resp = raw_auth(&mut hs, "intrus", &[1u8; 32]).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn signature_rejouee_refusee() {
    let (addr, dir) = start_server().await;
    let seed = seed_of(&dir, "jaykonta");
    let mut h1 = raw_hello(addr, nonce(3)).await;
    let sig = SigningKey::from_bytes(&seed).sign(&client_transcript(
        &h1.client_nonce,
        &h1.server_nonce,
        &h1.server_pub,
        "jaykonta",
    ));
    let resp = send_auth(&mut h1.stream, "jaykonta", &sig.to_bytes()).await;
    assert!(
        matches!(resp, Some(Response::Authenticated { .. })),
        "{resp:?}"
    );

    // Rejeu sur une nouvelle connexion avec le même nonce client : nouveau nonce serveur.
    let mut h2 = raw_hello(addr, nonce(3)).await;
    let resp = send_auth(&mut h2.stream, "jaykonta", &sig.to_bytes()).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn operateur_a_ne_peut_agir_comme_b() {
    let (addr, dir) = start_server().await;

    // A = miyukini-central (Read sur jaykonta) tente d'écrire en se déclarant jaykonta (Admin).
    let seed_a = seed_of(&dir, "miyukini-central");
    let mut hs = raw_hello(addr, nonce(4)).await;
    let resp = raw_auth(&mut hs, "miyukini-central", &seed_a).await;
    assert!(
        matches!(resp, Some(Response::Authenticated { ref operator_id }) if operator_id == "miyukini-central")
    );

    let req = Request::Execute {
        operator_id: "jaykonta".to_string(),
        database: "jaykonta".to_string(),
        sql: "CREATE TABLE vol (x)".to_string(),
        params: vec![],
        intent: "usurpation".to_string(),
    };
    write_json(&mut hs.stream, &serde_json::to_value(&req).expect("req")).await;
    let resp = read_response(&mut hs.stream).await;
    assert!(is_error(&resp, "PERMISSION_DENIED"), "{resp:?}");

    // Avec sa propre identité, l'écriture sur jaykonta reste interdite (Read).
    let req = Request::Execute {
        operator_id: "miyukini-central".to_string(),
        database: "jaykonta".to_string(),
        sql: "CREATE TABLE vol (x)".to_string(),
        params: vec![],
        intent: "ecriture".to_string(),
    };
    write_json(&mut hs.stream, &serde_json::to_value(&req).expect("req")).await;
    let resp = read_response(&mut hs.stream).await;
    assert!(is_error(&resp, "PERMISSION_DENIED"), "{resp:?}");

    // A signe avec sa clé mais se présente comme B : refus.
    let mut h2 = raw_hello(addr, nonce(5)).await;
    let resp = raw_auth(&mut h2, "jaykonta", &seed_a).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");

    // Seconde poignée de main sur une connexion déjà liée : refus.
    write_json(
        &mut hs.stream,
        &json!({"type": "ClientHello", "protocol": "kindmother-auth-v2", "client_nonce": hex(&nonce(6))}),
    )
    .await;
    assert!(is_error(
        &read_response(&mut hs.stream).await,
        "INVALID_REQUEST"
    ));
    let resp = raw_auth(&mut hs, "jaykonta", &seed_of(&dir, "jaykonta")).await;
    assert!(is_error(&resp, "INVALID_REQUEST"), "{resp:?}");
    let _ = std::fs::remove_dir_all(dir);
}

// ─── K-04 : authentification mutuelle, anti-relais ─────────────────────────

/// Un relais (faux serveur) présentant sa propre clé est refusé par le client épinglé,
/// qui ne signe rien.
#[tokio::test]
async fn relais_avec_autre_cle_refuse_par_le_client() {
    let (_addr, dir) = start_server().await;
    let mitm = SigningKey::from_bytes(&[0x66; 32]);
    let mitm_pub = mitm.verifying_key().to_bytes();
    let relay = fake_server(move |cn| {
        let sn = [0x77u8; NONCE_LEN];
        let sig = mitm.sign(&server_transcript(&cn, &sn, &mitm_pub));
        json!({"type": "Challenge", "protocol": "kindmother-auth-v2",
               "server_nonce": hex(&sn), "server_pub": hex(&mitm_pub), "server_sig": hex(&sig.to_bytes())})
    })
    .await;

    let err = KindMotherClient::connect(
        &relay.to_string(),
        &credential(&dir, "jaykonta"),
        "jaykonta",
    )
    .await
    .err();
    assert!(
        matches!(&err, Some(ClientError::Authentication(m)) if m.contains("pinned")),
        "{err:?}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

/// Une signature d'opérateur obtenue pour une autre transcription (clé du relais,
/// autre nonce client) est refusée par le vrai serveur.
#[tokio::test]
async fn signature_relayee_refusee_par_le_serveur() {
    let (addr, dir) = start_server().await;
    let seed = seed_of(&dir, "jaykonta");
    let sk = SigningKey::from_bytes(&seed);
    let mitm_pub = SigningKey::from_bytes(&[0x66; 32])
        .verifying_key()
        .to_bytes();

    // Le relais ouvre une session avec le vrai serveur en réutilisant le nonce client
    // de la victime, puis présente la signature obtenue pour sa propre clé.
    let mut hs = raw_hello(addr, nonce(7)).await;
    let volee = sk.sign(&client_transcript(
        &hs.client_nonce,
        &hs.server_nonce,
        &mitm_pub,
        "jaykonta",
    ));
    let resp = send_auth(&mut hs.stream, "jaykonta", &volee.to_bytes()).await;
    assert!(
        is_error(&resp, "AUTH_FAILED"),
        "server_pub différent : {resp:?}"
    );

    // Signature pour un autre nonce client (la victime parlait au relais).
    let mut hs = raw_hello(addr, nonce(8)).await;
    let volee = sk.sign(&client_transcript(
        &nonce(9),
        &hs.server_nonce,
        &hs.server_pub,
        "jaykonta",
    ));
    let resp = send_auth(&mut hs.stream, "jaykonta", &volee.to_bytes()).await;
    assert!(
        is_error(&resp, "AUTH_FAILED"),
        "client_nonce différent : {resp:?}"
    );

    // Contrôle : la transcription exacte est acceptée.
    let mut hs = raw_hello(addr, nonce(10)).await;
    let resp = raw_auth(&mut hs, "jaykonta", &seed).await;
    assert!(
        matches!(resp, Some(Response::Authenticated { .. })),
        "{resp:?}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

/// Sans pin du serveur, le client refuse de se connecter.
#[tokio::test]
async fn pin_serveur_absent_refuse() {
    let (addr, dir) = start_server().await;
    let sans_pin =
        OperatorCredential::from_file("jaykonta", dir.join("credentials").join("jaykonta.key"))
            .expect("cred");
    let err = KindMotherClient::connect(&addr.to_string(), &sans_pin, "jaykonta")
        .await
        .err();
    assert!(
        matches!(&err, Some(ClientError::Authentication(m)) if m.contains("server.pub")),
        "{err:?}"
    );

    // Répertoire sans server.pub : le chargement échoue.
    let copie = std::env::temp_dir().join(format!("km-nopin-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&copie).expect("dir");
    let key = copie.join("jaykonta.key");
    std::fs::copy(dir.join("credentials").join("jaykonta.key"), &key).expect("copy");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o600)).expect("perm");
    }
    assert!(OperatorCredential::load_from_dir("jaykonta", &copie).is_err());
    let _ = std::fs::remove_dir_all(copie);
    let _ = std::fs::remove_dir_all(dir);
}

/// Faux serveur présentant la clé épinglée sans pouvoir la signer : refus.
#[tokio::test]
async fn mauvaise_signature_serveur_refusee() {
    let (addr, dir) = start_server().await;
    let pin = server_pin(&dir);
    let faussaire = SigningKey::from_bytes(&[0x55; 32]);
    let relay = fake_server(move |cn| {
        let sn = [0x44u8; NONCE_LEN];
        // Signe la bonne transcription (clé épinglée) avec une autre clé.
        let sig = faussaire.sign(&server_transcript(&cn, &sn, &pin));
        json!({"type": "Challenge", "protocol": "kindmother-auth-v2",
               "server_nonce": hex(&sn), "server_pub": hex(&pin), "server_sig": hex(&sig.to_bytes())})
    })
    .await;
    let err = KindMotherClient::connect(
        &relay.to_string(),
        &credential(&dir, "jaykonta"),
        "jaykonta",
    )
    .await
    .err();
    assert!(
        matches!(&err, Some(ClientError::Authentication(m)) if m.contains("signature")),
        "{err:?}"
    );

    // Signature du vrai serveur rejouée pour un autre nonce client : refus.
    let mut stream = TcpStream::connect(addr).await.expect("connect");
    write_json(
        &mut stream,
        &json!({"type": "ClientHello", "protocol": "kindmother-auth-v2", "client_nonce": hex(&nonce(0x12))}),
    )
    .await;
    let authentique: serde_json::Value =
        serde_json::from_slice(&read_frame(&mut stream).await.expect("défi")).expect("json");
    let relay = fake_server(move |_cn| authentique.clone()).await;
    let err = KindMotherClient::connect(
        &relay.to_string(),
        &credential(&dir, "jaykonta"),
        "jaykonta",
    )
    .await
    .err();
    assert!(
        matches!(err, Some(ClientError::Authentication(_))),
        "{err:?}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

/// Le protocole v1 est refusé explicitement, côté serveur comme côté client.
#[tokio::test]
async fn protocole_v1_refuse() {
    let (addr, dir) = start_server().await;

    // ClientHello annonçant v1 : refus explicite et fermeture.
    let mut stream = TcpStream::connect(addr).await.expect("connect");
    write_json(
        &mut stream,
        &json!({"type": "ClientHello", "protocol": "kindmother-auth-v1", "client_nonce": hex(&nonce(1))}),
    )
    .await;
    let resp = read_response(&mut stream).await;
    assert!(
        matches!(&resp, Some(Response::Error { code, message }) if code == "AUTH_FAILED" && message.contains("unsupported")),
        "{resp:?}"
    );
    assert!(read_frame(&mut stream).await.is_none());

    // Client v1 : envoie directement Authenticate (signature v1) : refus.
    let mut stream = TcpStream::connect(addr).await.expect("connect");
    let resp = send_auth(&mut stream, "jaykonta", &[0u8; 64]).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    assert!(read_frame(&mut stream).await.is_none());

    // Serveur v1 (défi non signé) : le client v2 refuse.
    let v1 = fake_server(|_cn| {
        json!({"type": "Challenge", "protocol": "kindmother-auth-v1", "nonce": hex(&[0xab; 32])})
    })
    .await;
    let err = KindMotherClient::connect(&v1.to_string(), &credential(&dir, "jaykonta"), "jaykonta")
        .await
        .err();
    assert!(
        matches!(&err, Some(ClientError::Authentication(m)) if m.contains("Unsupported")),
        "{err:?}"
    );
    let _ = std::fs::remove_dir_all(dir);
}

// ─── K-04 : résistance à l'épuisement ───────────────────────────────────────

/// Une inondation de connexions muettes (au-delà de la borne de pré-authentification)
/// n'empêche pas un client légitime de s'authentifier ; l'emplacement principal n'est
/// pris qu'après authentification.
#[tokio::test]
async fn inondation_preauth_ne_bloque_pas_le_client() {
    let limits = ServerLimits {
        max_connections: 1,
        max_preauth: 4,
        max_preauth_per_peer: 2,
        ..ServerLimits::default()
    };
    let (addr, dir) = start_server_with(limits).await;

    let mut flood = Vec::new();
    for _ in 0..40 {
        flood.push(TcpStream::connect(addr).await.expect("flood"));
    }
    // Laisse le serveur accepter toutes les connexions muettes.
    tokio::time::sleep(Duration::from_millis(200)).await;

    let client = tokio::time::timeout(
        Duration::from_secs(5),
        KindMotherClient::connect(&addr.to_string(), &credential(&dir, "jaykonta"), "jaykonta"),
    )
    .await
    .expect("pas de blocage")
    .expect("client légitime authentifié malgré l'inondation");
    // Connexion utilisable (réponse reçue, base éventuellement encore vide).
    assert!(client.health_check().await.is_ok());

    // L'unique emplacement principal est occupé : un second opérateur authentifié est
    // refusé (BUSY), les connexions muettes n'en ont consommé aucun.
    let err = KindMotherClient::connect(&addr.to_string(), &credential(&dir, "jaykoa"), "jaykoa")
        .await
        .err();
    assert!(
        matches!(&err, Some(ClientError::Authentication(m)) if m.contains("connection limit")),
        "{err:?}"
    );

    // Les connexions évincées ont été fermées par le serveur (refus puis fin de flux).
    let mut fermees = 0;
    for s in &mut flood {
        let mut buf = Vec::new();
        if let Ok(Ok(_)) =
            tokio::time::timeout(Duration::from_millis(50), s.read_to_end(&mut buf)).await
        {
            fermees += 1;
        }
    }
    assert!(fermees >= 30, "connexions muettes évincées : {fermees}");
    drop(flood);
    let _ = std::fs::remove_dir_all(dir);
}

/// Une connexion authentifiée inactive est fermée après le délai d'inactivité.
#[tokio::test]
async fn connexion_inactive_fermee() {
    let limits = ServerLimits {
        idle_timeout: Duration::from_millis(300),
        ..ServerLimits::default()
    };
    let (addr, dir) = start_server_with(limits).await;

    let client =
        KindMotherClient::connect(&addr.to_string(), &credential(&dir, "jaykonta"), "jaykonta")
            .await
            .expect("client");
    // Active : la connexion répond
    assert!(client.health_check().await.is_ok());
    tokio::time::sleep(Duration::from_millis(900)).await;
    // Inactive depuis plus que le délai : connexion fermée par le serveur.
    assert!(client.health_check().await.is_err());

    // Même constat sur une connexion brute : fermeture sans réponse.
    let seed = seed_of(&dir, "jaykonta");
    let mut hs = raw_hello(addr, nonce(11)).await;
    assert!(matches!(
        raw_auth(&mut hs, "jaykonta", &seed).await,
        Some(Response::Authenticated { .. })
    ));
    let closed = tokio::time::timeout(Duration::from_secs(3), read_frame(&mut hs.stream))
        .await
        .expect("fermeture avant 3 s");
    assert!(closed.is_none());
    let _ = std::fs::remove_dir_all(dir);
}

// ─── K-05 : arbitrage SQL de bout en bout ───────────────────────────────────

const CORPUS: &[&str] = &[
    "ATTACH 'x' AS y",
    "attach database 'x' as y",
    "ATTACH  DATABASE  'x'  AS  y",
    "WITH x AS (SELECT 1) DELETE FROM t",
    "WITH x AS (SELECT 1) INSERT INTO t (v) SELECT 'b' FROM x",
    "PRAGMA writable_schema=1",
    "SELECT load_extension('x')",
    "SELECT 1; DROP TABLE t",
    "SELECT 1 /* ; */ ; DROP TABLE t",
    "/* c */ DELETE FROM t",
    "-- c\nDELETE FROM t",
    "DELETE/**/FROM t",
    "DROP TABLE t",
    "INSERT INTO t (v) VALUES ('b')",
    "CREATE TEMP TRIGGER tr AFTER INSERT ON t BEGIN DELETE FROM t; END",
    "BEGIN",
];

#[tokio::test]
async fn lecture_ecriture_et_contournements() {
    let (addr, dir) = start_server().await;
    let addr = addr.to_string();

    // jaykonta (Admin sur jaykonta) : écriture légitime.
    let admin = KindMotherClient::connect(&addr, &credential(&dir, "jaykonta"), "jaykonta")
        .await
        .expect("admin");
    admin
        .execute(
            "CREATE TABLE IF NOT EXISTS t (id INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT)",
            Vec::<String>::new(),
            "schema",
        )
        .await
        .expect("create");
    admin
        .execute("INSERT INTO t (v) VALUES (?)", vec!["a"], "insert")
        .await
        .expect("insert");

    // SELECT légitime
    let rows = admin
        .query("SELECT v FROM t WHERE id = ?", vec!["1"])
        .await
        .expect("select");
    assert_eq!(rows.len(), 1);

    // miyukini-central : Read sur jaykonta
    let reader = KindMotherClient::connect(&addr, &credential(&dir, "miyukini-central"), "central")
        .await
        .expect("reader");
    let rows = reader
        .query_db(
            "jaykonta",
            "SELECT COUNT(*) AS n FROM t",
            Vec::<String>::new(),
        )
        .await
        .expect("lecture autorisée");
    assert_eq!(rows[0]["n"], json!(1));

    // Écriture interdite au lecteur
    assert!(reader
        .execute_db(
            "jaykonta",
            "INSERT INTO t (v) VALUES ('x')",
            Vec::<String>::new(),
            "w"
        )
        .await
        .is_err());

    // Corpus : refusé sur le chemin Query, pour le lecteur comme pour l'Admin.
    for sql in CORPUS {
        assert!(
            reader
                .query_db("jaykonta", sql, Vec::<String>::new())
                .await
                .is_err(),
            "lecteur : doit être refusé {sql:?}"
        );
        assert!(
            admin.query(sql, Vec::<String>::new()).await.is_err(),
            "admin/query : doit être refusé {sql:?}"
        );
    }

    // Instructions dangereuses refusées même en écriture Admin.
    for sql in [
        "ATTACH 'x' AS y",
        "PRAGMA writable_schema=1",
        "SELECT load_extension('x')",
        "INSERT INTO t (v) VALUES ('b'); DROP TABLE t",
        "INSERT INTO _kindmother_audit (operator_id, operation, success) VALUES ('x','y',1)",
        "BEGIN",
    ] {
        assert!(
            admin.execute(sql, Vec::<String>::new(), "x").await.is_err(),
            "admin/execute : doit être refusé {sql:?}"
        );
    }

    // Rien n'a été modifié ni détruit.
    let rows = admin
        .query("SELECT COUNT(*) AS n FROM t", Vec::<String>::new())
        .await
        .expect("count");
    assert_eq!(rows[0]["n"], json!(1));

    // Transaction : une instruction interdite annule tout.
    assert!(admin
        .transaction(
            vec![
                ("INSERT INTO t (v) VALUES ('tx')", Vec::<String>::new()),
                ("ATTACH 'x' AS y", Vec::<String>::new()),
            ],
            "tx"
        )
        .await
        .is_err());
    let rows = admin
        .query("SELECT COUNT(*) AS n FROM t", Vec::<String>::new())
        .await
        .expect("count");
    assert_eq!(rows[0]["n"], json!(1));

    // Le lien d'identité tient : le client reste utilisable après des refus.
    assert!(admin.health_check().await.expect("health"));
    let _ = std::fs::remove_dir_all(dir);
}
