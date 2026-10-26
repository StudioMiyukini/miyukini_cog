//! Tests d'API des liaisons d'identité et de clés d'appareil (I6) : routeur réel
//! (`miyu_public_routes` + `miyu_protected_routes`), CSRF et même origine
//! réels ; seule l'authentification est simulée (en-tête `x-test-auth`).

use super::*;
use crate::infrastructure::services::miyu_auth_service::tests::Monde;
use crate::infrastructure::services::miyu_auth_service::{AncreCog, MemoireIdentites};
use crate::infrastructure::services::miyu_liaison_appareil::tests::demande as demande_mla2;
use crate::interfaces::middleware::auth::{CookieAuthenticated, CurrentUser};
use crate::interfaces::middleware::csrf::{csrf_middleware, same_origin_middleware};
use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use miyauth::http::{formater_autorisation_visa, formater_pop, liaison_requete, signer_pop};
use miyusceau::CleSignature;
use tower::ServiceExt;

const CSRF: &str = "jeton-csrf-de-test";

#[derive(Clone)]
struct Contexte {
    user: Uuid,
    store: Arc<MemoireIdentites>,
}

/// Authentification simulée : `session` (cookie du navigateur), `bearer`
/// (session sans cookie), `appareil:<clé b64>` (requête signée par cette clé).
async fn faux_auth(State(ctx): State<Contexte>, mut req: Request, next: Next) -> Response {
    let mode = req
        .headers()
        .get("x-test-auth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let user = CurrentUser {
        id: ctx.user,
        username: "alice".into(),
        email: "a@x".into(),
        role: "user".into(),
    };
    match mode.as_str() {
        "session" => {
            req.extensions_mut().insert(Arc::new(user));
            req.extensions_mut().insert(AuthMethod::Session);
            req.extensions_mut().insert(CookieAuthenticated);
        }
        "bearer" => {
            req.extensions_mut().insert(Arc::new(user));
            req.extensions_mut().insert(AuthMethod::Session);
        }
        m if m.starts_with("appareil:") => {
            let cle = miyauth::http::b64_decoder(&m["appareil:".len()..])
                .and_then(|v| <[u8; 32]>::try_from(v).ok())
                .unwrap();
            let fiche = ctx.store.active_device_key(&cle).await.unwrap().unwrap();
            req.extensions_mut().insert(Arc::new(user));
            req.extensions_mut().insert(AuthMethod::Appareil);
            req.extensions_mut().insert(Arc::new(fiche));
        }
        _ => {}
    }
    next.run(req).await
}

struct Banc {
    monde: Monde,
    user: Uuid,
    svc: Arc<MiyuAuthService<MemoireIdentites>>,
    app: Router,
}

impl Banc {
    fn new() -> Self {
        let monde = Monde::new();
        let store = Arc::new(MemoireIdentites::default());
        let svc = Arc::new(
            MiyuAuthService::new(
                store.clone(),
                Some(AncreCog {
                    cog_id: monde.cog.clone(),
                    cle_emission: Some(monde.cle_cog.cle_publique()),
                    racine: None,
                    delegations: Vec::new(),
                }),
            )
            .unwrap()
            .avec_revocations(monde.revocations.clone()),
        );
        let user = Uuid::new_v4();
        let ctx = Contexte { user, store };
        let routes = miyu_public_routes::<MemoireIdentites>()
            .merge(miyu_protected_routes::<MemoireIdentites>())
            // Même empilement que `main.rs` : auth → CSRF → même origine.
            .layer(axum::middleware::from_fn(same_origin_middleware))
            .layer(axum::middleware::from_fn(csrf_middleware))
            .layer(axum::middleware::from_fn_with_state(ctx, faux_auth))
            .with_state(svc.clone());
        let app = Router::new().nest("/api/auth/miyu", routes);
        Self {
            monde,
            user,
            svc,
            app,
        }
    }

    async fn appel(&self, req: axum::http::Request<Body>) -> (StatusCode, serde_json::Value) {
        let r = self.app.clone().oneshot(req).await.unwrap();
        let statut = r.status();
        let octets = axum::body::to_bytes(r.into_body(), 1 << 20).await.unwrap();
        let json = serde_json::from_slice(&octets).unwrap_or(serde_json::Value::Null);
        (statut, json)
    }
}

/// Requête de session du navigateur, CSRF correct, même origine.
fn navigateur(methode: &str, chemin: &str) -> axum::http::request::Builder {
    axum::http::Request::builder()
        .method(methode)
        .uri(chemin)
        .header("x-test-auth", "session")
        .header("cookie", format!("miyucloud_csrf={CSRF}"))
        .header("x-csrf-token", CSRF)
        .header("host", "cloud.local")
        .header("origin", "https://cloud.local")
        .header("sec-fetch-site", "same-origin")
        .header("content-type", "application/json")
}

fn json(b: axum::http::request::Builder, v: &serde_json::Value) -> axum::http::Request<Body> {
    b.body(Body::from(v.to_string())).unwrap()
}

fn vide(b: axum::http::request::Builder) -> axum::http::Request<Body> {
    b.body(Body::empty()).unwrap()
}

fn corps_liaison(banc: &Banc, cle: &CleSignature) -> serde_json::Value {
    let d = demande_mla2(&banc.monde.cog, &cle.graine(), None, maintenant() + 120);
    serde_json::json!({ "demande": d.encoder(), "nom": "Téléphone", "portees": "api,webdav" })
}

#[tokio::test]
async fn public_link_context_exposes_pinned_cog() {
    let b = Banc::new();
    let req = axum::http::Request::get("/api/auth/miyu/liaison-appareil")
        .body(Body::empty())
        .unwrap();
    let (s, j) = b.appel(req).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(j["cog_id"], b.monde.cog.as_str());
    assert_eq!(j["format"], "MLA2");
}

#[tokio::test]
async fn browser_session_lists_links_and_removes_a_device() {
    let b = Banc::new();
    let cle = CleSignature::generer();
    let corps = corps_liaison(&b, &cle);
    let (s, j) = b
        .appel(json(navigateur("POST", "/api/auth/miyu/appareils"), &corps))
        .await;
    assert_eq!(s, StatusCode::CREATED, "{j}");
    assert_eq!(
        j["public_key"],
        miyauth::http::b64_encoder(&cle.cle_publique())
    );
    assert_eq!(j["scopes"], "api,webdav");
    let id = j["id"].as_str().unwrap().to_string();

    // Même demande rejouée : conflit.
    let (s, _) = b
        .appel(json(navigateur("POST", "/api/auth/miyu/appareils"), &corps))
        .await;
    assert_eq!(s, StatusCode::CONFLICT);

    let (s, j) = b
        .appel(vide(navigateur("GET", "/api/auth/miyu/appareils")))
        .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(j.as_array().unwrap().len(), 1);

    // Retrait : la clé n'authentifie plus rien (signature ni visa).
    let chemin = format!("/api/auth/miyu/appareils/{id}");
    let (s, _) = b.appel(vide(navigateur("DELETE", &chemin))).await;
    assert_eq!(s, StatusCode::NO_CONTENT);
    let (_, j) = b
        .appel(vide(navigateur("GET", "/api/auth/miyu/appareils")))
        .await;
    assert!(j.as_array().unwrap().is_empty());
    assert!(
        b.svc
            .store()
            .active_device_key(&cle.cle_publique())
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        b.svc
            .store()
            .device_key_revoked(&cle.cle_publique())
            .await
            .unwrap()
    );
    let (s, _) = b.appel(vide(navigateur("DELETE", &chemin))).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = b
        .appel(vide(navigateur(
            "DELETE",
            "/api/auth/miyu/appareils/pas-un-uuid",
        )))
        .await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn removing_a_device_cuts_its_visa_sessions() {
    let b = Banc::new();
    // L'appareil du monde (celui des visas) est lié par l'interface.
    let corps = corps_liaison(&b, &b.monde.appareil);
    let (s, j) = b
        .appel(json(navigateur("POST", "/api/auth/miyu/appareils"), &corps))
        .await;
    assert_eq!(s, StatusCode::CREATED);
    let id = j["id"].as_str().unwrap().to_string();
    // Son identité est liée au compte : le visa passe.
    let titres = b.monde.titres();
    let presenter = || {
        let d = b.svc.emettre_defi(maintenant()).unwrap();
        let liaison = liaison_requete("GET", "/x", b"");
        let sig = signer_pop(&b.monde.appareil, &d.defi, AUDIENCE_MIYUCLOUD, &liaison);
        (
            formater_autorisation_visa(&titres.0, &titres.1),
            formater_pop(&d.id, &sig),
        )
    };
    let requete = RequeteProuvee {
        methode: "GET",
        chemin: "/x",
        chemin_et_requete: "/x",
        corps: b"",
    };
    let (a, p) = presenter();
    let id_visa = b
        .svc
        .verifier_visa(requete, Some(&a), Some(&p), maintenant())
        .unwrap();
    b.svc
        .store()
        .link_identity(b.user, &id_visa.cog_id, &id_visa.sujet, "t")
        .await
        .unwrap();
    let (a, p) = presenter();
    assert!(
        b.svc
            .authentifier_visa(requete, Some(&a), Some(&p), None, maintenant())
            .await
            .is_ok()
    );
    // Retrait de l'appareil : ses sessions par visa sont coupées.
    let (s, _) = b
        .appel(vide(navigateur(
            "DELETE",
            &format!("/api/auth/miyu/appareils/{id}"),
        )))
        .await;
    assert_eq!(s, StatusCode::NO_CONTENT);
    let (a, p) = presenter();
    let r = b
        .svc
        .authentifier_visa(requete, Some(&a), Some(&p), None, maintenant())
        .await;
    assert!(matches!(r, Err(RefusMiyu::AppareilRetire)), "{r:?}");
}

#[tokio::test]
async fn csrf_and_origin_are_enforced_for_browser_sessions() {
    let b = Banc::new();
    let corps = corps_liaison(&b, &CleSignature::generer());
    // Sans jeton CSRF.
    let req = axum::http::Request::post("/api/auth/miyu/appareils")
        .header("x-test-auth", "session")
        .header("content-type", "application/json")
        .body(Body::from(corps.to_string()))
        .unwrap();
    assert_eq!(b.appel(req).await.0, StatusCode::FORBIDDEN);
    // Cookie et en-tête différents.
    let req = axum::http::Request::post("/api/auth/miyu/appareils")
        .header("x-test-auth", "session")
        .header("cookie", "miyucloud_csrf=autre")
        .header("x-csrf-token", CSRF)
        .header("content-type", "application/json")
        .body(Body::from(corps.to_string()))
        .unwrap();
    assert_eq!(b.appel(req).await.0, StatusCode::FORBIDDEN);
    // CSRF correct mais autre origine (Origin ou Sec-Fetch-Site).
    for (nom, valeur) in [
        ("origin", "https://evil.example"),
        ("sec-fetch-site", "cross-site"),
    ] {
        let req = axum::http::Request::post("/api/auth/miyu/appareils")
            .header("x-test-auth", "session")
            .header("cookie", format!("miyucloud_csrf={CSRF}"))
            .header("x-csrf-token", CSRF)
            .header("host", "cloud.local")
            .header(nom, valeur)
            .header("content-type", "application/json")
            .body(Body::from(corps.to_string()))
            .unwrap();
        assert_eq!(b.appel(req).await.0, StatusCode::FORBIDDEN, "{nom}");
    }
    // Retrait depuis une autre origine.
    let req = axum::http::Request::delete(format!("/api/auth/miyu/appareils/{}", Uuid::new_v4()))
        .header("x-test-auth", "session")
        .header("cookie", format!("miyucloud_csrf={CSRF}"))
        .header("x-csrf-token", CSRF)
        .header("sec-fetch-site", "cross-site")
        .body(Body::empty())
        .unwrap();
    assert_eq!(b.appel(req).await.0, StatusCode::FORBIDDEN);
    // Rien n'a été lié.
    assert!(
        b.svc
            .store()
            .list_device_keys(b.user)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn only_interactive_sessions_manage_devices() {
    let b = Banc::new();
    let cle = CleSignature::generer();
    b.svc
        .store()
        .register_device_key(b.user, &cle.cle_publique(), "pc", "api")
        .await
        .unwrap();
    let corps = corps_liaison(&b, &CleSignature::generer());
    let auth = format!(
        "appareil:{}",
        miyauth::http::b64_encoder(&cle.cle_publique())
    );
    for (methode, chemin) in [
        ("GET", "/api/auth/miyu/appareils".to_string()),
        ("POST", "/api/auth/miyu/appareils".to_string()),
        (
            "DELETE",
            format!("/api/auth/miyu/appareils/{}", Uuid::new_v4()),
        ),
    ] {
        let req = axum::http::Request::builder()
            .method(methode)
            .uri(&chemin)
            .header("x-test-auth", auth.as_str())
            .header("content-type", "application/json")
            .body(Body::from(corps.to_string()))
            .unwrap();
        assert_eq!(
            b.appel(req).await.0,
            StatusCode::FORBIDDEN,
            "{methode} {chemin}"
        );
    }
    // Sans authentification : refus.
    let req = axum::http::Request::get("/api/auth/miyu/appareils")
        .body(Body::empty())
        .unwrap();
    assert_eq!(b.appel(req).await.0, StatusCode::UNAUTHORIZED);
    // Bearer (session sans cookie) : pas de CSRF exigé.
    let req = axum::http::Request::post("/api/auth/miyu/appareils")
        .header("x-test-auth", "bearer")
        .header("content-type", "application/json")
        .body(Body::from(corps.to_string()))
        .unwrap();
    assert_eq!(b.appel(req).await.0, StatusCode::CREATED);
}

#[tokio::test]
async fn device_rotation_endpoint_requires_old_key_signature() {
    let b = Banc::new();
    let ancienne = CleSignature::generer();
    b.svc
        .store()
        .register_device_key(b.user, &ancienne.cle_publique(), "tel", "webdav")
        .await
        .unwrap();
    let nouvelle = CleSignature::generer();
    let d = demande_mla2(
        &b.monde.cog,
        &nouvelle.graine(),
        Some(&ancienne.graine()),
        maintenant() + 60,
    );
    let corps = serde_json::json!({ "demande": d.encoder() });
    // Session du navigateur : refus (seule l'ancienne clé fait tourner).
    let (s, _) = b
        .appel(json(
            navigateur("POST", "/api/auth/miyu/appareils/rotation"),
            &corps,
        ))
        .await;
    assert_eq!(s, StatusCode::FORBIDDEN);
    // Requête signée par l'ancienne clé.
    let req = axum::http::Request::post("/api/auth/miyu/appareils/rotation")
        .header(
            "x-test-auth",
            format!(
                "appareil:{}",
                miyauth::http::b64_encoder(&ancienne.cle_publique())
            ),
        )
        .header("content-type", "application/json")
        .body(Body::from(corps.to_string()))
        .unwrap();
    let (s, j) = b.appel(req).await;
    assert_eq!(s, StatusCode::CREATED, "{j}");
    assert_eq!(
        j["public_key"],
        miyauth::http::b64_encoder(&nouvelle.cle_publique())
    );
    assert_eq!(j["client_name"], "tel");
    let store = b.svc.store();
    assert!(
        store
            .active_device_key(&ancienne.cle_publique())
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        store
            .active_device_key(&nouvelle.cle_publique())
            .await
            .unwrap()
            .is_some()
    );
}

#[tokio::test]
async fn identity_link_from_pasted_proof() {
    let b = Banc::new();
    let titres = b.monde.titres();
    let chemin = "/api/auth/miyu/liaisons";
    let d = b.svc.emettre_defi(maintenant()).unwrap();
    let liaison = liaison_requete("POST", chemin, b"");
    let sig = signer_pop(&b.monde.appareil, &d.defi, AUDIENCE_MIYUCLOUD, &liaison);
    let req = vide(
        navigateur("POST", &format!("{chemin}?label=Mon%20t%C3%A9l%C3%A9phone"))
            .header(
                ENTETE_VISA_A_LIER,
                formater_autorisation_visa(&titres.0, &titres.1),
            )
            .header(miyauth::http::ENTETE_POP, formater_pop(&d.id, &sig)),
    );
    let (s, j) = b.appel(req).await;
    assert_eq!(s, StatusCode::CREATED, "{j}");
    assert_eq!(j["label"], "Mon téléphone");
    assert_eq!(j["cog_id"], b.monde.cog.as_str());
    // Preuve rejouée : défi consommé.
    let (s, _) = b
        .appel(vide(
            navigateur("POST", chemin)
                .header(
                    ENTETE_VISA_A_LIER,
                    formater_autorisation_visa(&titres.0, &titres.1),
                )
                .header(miyauth::http::ENTETE_POP, formater_pop(&d.id, &sig)),
        ))
        .await;
    assert_eq!(s, StatusCode::UNAUTHORIZED);
    let (_, j) = b.appel(vide(navigateur("GET", chemin))).await;
    let id = j[0]["id"].as_str().unwrap().to_string();
    let (s, _) = b
        .appel(vide(navigateur("DELETE", &format!("{chemin}/{id}"))))
        .await;
    assert_eq!(s, StatusCode::NO_CONTENT);
    let (_, j) = b.appel(vide(navigateur("GET", chemin))).await;
    assert!(j.as_array().unwrap().is_empty());
}
