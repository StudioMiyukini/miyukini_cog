//! Tests de sécurité K-04 (authentification des opérateurs) et K-05 (arbitrage SQL).
//!
//! @id     kindmother_service_tests_securite_k04_k05
//! @do     verifier_refus_authentification_et_contournements_sql
//! @role   test
//! @layer  core
//! @human  kindmother-service — tests négatifs (C-60) sur le serveur TCP réel.

use std::net::SocketAddr;
use std::path::PathBuf;

use ed25519_dalek::{Signer, SigningKey};
use kindmother_client::{ClientError, KindMotherClient, OperatorCredential};
use kindmother_service::{auth_message, KindMotherServer, Request, Response, NONCE_LEN};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Démarre un serveur sur un port éphémère de la boucle locale.
async fn start_server() -> (SocketAddr, PathBuf) {
    let dir = std::env::temp_dir().join(format!("km-sec-{}", uuid::Uuid::new_v4()));
    let server = KindMotherServer::new(&dir).await.expect("server");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(server.serve_listener(listener));
    (addr, dir)
}

fn credential(dir: &std::path::Path, operator: &str) -> OperatorCredential {
    OperatorCredential::from_file(operator, dir.join("credentials").join(format!("{operator}.key")))
        .expect("credential")
}

fn seed_of(dir: &std::path::Path, operator: &str) -> [u8; 32] {
    let text = std::fs::read_to_string(dir.join("credentials").join(format!("{operator}.key")))
        .expect("seed");
    let text = text.trim();
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex");
    }
    out
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

/// Ouvre une connexion brute et retourne le nonce du défi.
async fn raw_connect(addr: SocketAddr) -> (TcpStream, [u8; NONCE_LEN]) {
    let mut stream = TcpStream::connect(addr).await.expect("connect");
    let Some(Response::Challenge { protocol, nonce }) = read_response(&mut stream).await else {
        panic!("défi attendu");
    };
    assert_eq!(protocol, "kindmother-auth-v1");
    let mut out = [0u8; NONCE_LEN];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&nonce[2 * i..2 * i + 2], 16).expect("hex");
    }
    (stream, out)
}

/// Authentifie une connexion brute avec une graine donnée ; retourne la réponse.
async fn raw_auth(
    stream: &mut TcpStream,
    nonce: &[u8; NONCE_LEN],
    operator: &str,
    seed: &[u8; 32],
) -> Option<Response> {
    let sk = SigningKey::from_bytes(seed);
    let sig = sk.sign(&auth_message(operator, nonce));
    write_json(
        stream,
        &json!({"type": "Authenticate", "operator_id": operator, "signature": hex(&sig.to_bytes())}),
    )
    .await;
    read_response(stream).await
}

fn is_error(resp: &Option<Response>, expected: &str) -> bool {
    matches!(resp, Some(Response::Error { code, .. }) if code == expected)
}

// ─── K-04 : authentification ────────────────────────────────────────────────

#[tokio::test]
async fn requete_non_authentifiee_refusee() {
    let (addr, dir) = start_server().await;
    let (mut stream, _nonce) = raw_connect(addr).await;

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
    let (mut stream, nonce) = raw_connect(addr).await;
    let resp = raw_auth(&mut stream, &nonce, "jaykonta", &[42u8; 32]).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    assert!(read_frame(&mut stream).await.is_none());

    // Même refus via le client
    let faux = OperatorCredential::from_seed("jaykonta", &[42u8; 32]).expect("cred");
    let err = KindMotherClient::connect(&addr.to_string(), &faux, "jaykonta")
        .await
        .err();
    assert!(matches!(err, Some(ClientError::Authentication(_))), "{err:?}");
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn operateur_inconnu_refuse() {
    let (addr, dir) = start_server().await;
    let (mut stream, nonce) = raw_connect(addr).await;
    let resp = raw_auth(&mut stream, &nonce, "intrus", &[1u8; 32]).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn signature_rejouee_refusee() {
    let (addr, dir) = start_server().await;
    let seed = seed_of(&dir, "jaykonta");
    let (mut s1, n1) = raw_connect(addr).await;
    let sig = SigningKey::from_bytes(&seed).sign(&auth_message("jaykonta", &n1));
    let auth = json!({"type": "Authenticate", "operator_id": "jaykonta", "signature": hex(&sig.to_bytes())});
    write_json(&mut s1, &auth).await;
    assert!(matches!(read_response(&mut s1).await, Some(Response::Authenticated { .. })));

    // Rejeu de la même signature sur une nouvelle connexion (nouveau nonce).
    let (mut s2, _n2) = raw_connect(addr).await;
    write_json(&mut s2, &auth).await;
    assert!(is_error(&read_response(&mut s2).await, "AUTH_FAILED"));
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn operateur_a_ne_peut_agir_comme_b() {
    let (addr, dir) = start_server().await;

    // A = miyukini-central (Read sur jaykonta) tente d'écrire en se déclarant jaykonta (Admin).
    let seed_a = seed_of(&dir, "miyukini-central");
    let (mut stream, nonce) = raw_connect(addr).await;
    let resp = raw_auth(&mut stream, &nonce, "miyukini-central", &seed_a).await;
    assert!(matches!(resp, Some(Response::Authenticated { ref operator_id }) if operator_id == "miyukini-central"));

    let req = Request::Execute {
        operator_id: "jaykonta".to_string(),
        database: "jaykonta".to_string(),
        sql: "CREATE TABLE vol (x)".to_string(),
        params: vec![],
        intent: "usurpation".to_string(),
    };
    write_json(&mut stream, &serde_json::to_value(&req).expect("req")).await;
    let resp = read_response(&mut stream).await;
    assert!(is_error(&resp, "PERMISSION_DENIED"), "{resp:?}");

    // Avec sa propre identité, l'écriture sur jaykonta reste interdite (Read).
    let req = Request::Execute {
        operator_id: "miyukini-central".to_string(),
        database: "jaykonta".to_string(),
        sql: "CREATE TABLE vol (x)".to_string(),
        params: vec![],
        intent: "ecriture".to_string(),
    };
    write_json(&mut stream, &serde_json::to_value(&req).expect("req")).await;
    let resp = read_response(&mut stream).await;
    assert!(is_error(&resp, "PERMISSION_DENIED"), "{resp:?}");

    // A signe avec sa clé mais se présente comme B : refus.
    let (mut s2, n2) = raw_connect(addr).await;
    let resp = raw_auth(&mut s2, &n2, "jaykonta", &seed_a).await;
    assert!(is_error(&resp, "AUTH_FAILED"), "{resp:?}");

    // Seconde authentification sur une connexion déjà liée : refus.
    let resp = raw_auth(&mut stream, &nonce, "jaykonta", &seed_of(&dir, "jaykonta")).await;
    assert!(is_error(&resp, "INVALID_REQUEST"), "{resp:?}");
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
        .query_db("jaykonta", "SELECT COUNT(*) AS n FROM t", Vec::<String>::new())
        .await
        .expect("lecture autorisée");
    assert_eq!(rows[0]["n"], json!(1));

    // Écriture interdite au lecteur
    assert!(reader
        .execute_db("jaykonta", "INSERT INTO t (v) VALUES ('x')", Vec::<String>::new(), "w")
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
