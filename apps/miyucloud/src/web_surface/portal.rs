//! Portail web authentifie MiyuCloud.

use crate::connect_auth::{new_pending_qr_challenge, PendingQrChallenge, PortalIdentity, WebPortalSession};
use crate::security_headers::CspNonce;
use crate::web_surface::access_log;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Form, Multipart, Path, Query, State};
use axum::http::{header, HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::{Extension, Json};
use miyucloud::domain::FileOps;
use miyucloud::utils::sanitize::{sanitize_mime_type, validate_uuid};
use qrcode::render::svg;
use qrcode::QrCode;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PortalQuery {
    pub folder: Option<String>,
}

#[derive(Deserialize)]
pub struct PasswordLoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateFolderForm {
    pub name: String,
    pub parent_id: Option<String>,
}

pub async fn portal_home(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PortalQuery>,
    headers: HeaderMap,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> axum::response::Response {
    let Some(session) = resolve_session(&state, &headers) else {
        return Redirect::to("/login").into_response();
    };

    let folder_id = query
        .folder
        .as_deref()
        .filter(|id| validate_uuid(id))
        .map(ToString::to_string);
    let contents = match FileOps::list_contents(&state.db, folder_id.as_deref(), &state.config.owner_id) {
        Ok(contents) => contents,
        Err(err) => {
            return Html(render_error_page("MiyuCloud", &format!("Erreur de lecture: {err}"), &nonce))
                .into_response();
        }
    };

    let parent_id = contents
        .folder
        .as_ref()
        .and_then(|folder| folder.parent_id.clone());
    let current_title = contents
        .folder
        .as_ref()
        .map(|folder| folder.name.clone())
        .unwrap_or_else(|| "Racine".to_string());

    let mut folders_html = String::new();
    if let Some(parent) = parent_id {
        folders_html.push_str(&format!(
            "<li><a href=\"/?folder={}\">.. Remonter</a></li>",
            escape_attr(&parent)
        ));
    }
    for folder in &contents.subfolders {
        folders_html.push_str(&format!(
            "<li><a href=\"/?folder={id}\">📁 {name}</a></li>",
            id = escape_attr(&folder.id),
            name = escape_html(&folder.name)
        ));
    }
    if folders_html.is_empty() {
        folders_html.push_str("<li class=\"empty\">Aucun dossier.</li>");
    }

    let mut files_html = String::new();
    for file in &contents.files {
        files_html.push_str(&format!(
            "<tr><td>{name}</td><td>{size}</td><td><a href=\"/files/{id}/download\">Télécharger</a></td></tr>",
            id = escape_attr(&file.id),
            name = escape_html(&file.name),
            size = file.size_bytes
        ));
    }
    if files_html.is_empty() {
        files_html.push_str("<tr><td colspan=\"3\" class=\"empty\">Aucun fichier.</td></tr>");
    }

    let folder_field = folder_id.clone().unwrap_or_default();
    let html = format!(
        r#"<!doctype html>
<html lang="fr">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>MiyuCloud</title>
  <style nonce="{nonce}">
    body {{ font-family: 'Segoe UI', sans-serif; background: #0d1524; color: #e8edf5; margin: 0; }}
    main {{ max-width: 1100px; margin: 0 auto; padding: 32px 20px 48px; }}
    .hero {{ display: flex; justify-content: space-between; align-items: flex-start; gap: 20px; margin-bottom: 24px; }}
    .card {{ background: #152038; border: 1px solid #253252; border-radius: 16px; padding: 18px; }}
    .grid {{ display: grid; grid-template-columns: 280px 1fr; gap: 20px; }}
    h1, h2 {{ margin: 0 0 12px; }}
    p {{ color: #aab7d3; }}
    form {{ display: flex; gap: 10px; flex-wrap: wrap; margin-top: 12px; }}
    input {{ background: #0e172a; color: #e8edf5; border: 1px solid #334263; border-radius: 10px; padding: 10px 12px; }}
    button {{ background: #2b7cff; color: white; border: none; border-radius: 10px; padding: 10px 14px; cursor: pointer; font-weight: 600; }}
    a {{ color: #7cc4ff; text-decoration: none; }}
    ul {{ list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 10px; }}
    li {{ background: #0e172a; border: 1px solid #253252; border-radius: 12px; padding: 12px; }}
    table {{ width: 100%; border-collapse: collapse; }}
    th, td {{ text-align: left; padding: 12px 8px; border-bottom: 1px solid #253252; }}
    .empty {{ color: #8190ae; }}
    .muted {{ color: #98a6c3; font-size: 14px; }}
    .mb-20 {{ margin-bottom: 20px; }}
  </style>
</head>
<body>
  <main>
    <section class="hero">
      <div>
        <p class="muted">Miyukini Cloud</p>
        <h1>Bonjour {display_name}</h1>
        <p>Exploration, televersement et telechargement de ton cloud depuis le web du COG.</p>
      </div>
      <form method="post" action="/logout">
        <button type="submit">Se deconnecter</button>
      </form>
    </section>
    <section class="card mb-20">
      <h2>Dossier courant: {current_title}</h2>
      <p>Compte Connect: {email}</p>
      <form method="post" action="/folders">
        <input type="hidden" name="parent_id" value="{folder_field}">
        <input type="text" name="name" placeholder="Nouveau dossier" required>
        <button type="submit">Creer un dossier</button>
      </form>
      <form method="post" action="/upload" enctype="multipart/form-data">
        <input type="hidden" name="parent_id" value="{folder_field}">
        <input type="file" name="file" required>
        <button type="submit">Televerser</button>
      </form>
    </section>
    <section class="grid">
      <aside class="card">
        <h2>Dossiers</h2>
        <ul>{folders_html}</ul>
      </aside>
      <section class="card">
        <h2>Fichiers</h2>
        <table>
          <thead>
            <tr><th>Nom</th><th>Taille (octets)</th><th>Action</th></tr>
          </thead>
          <tbody>{files_html}</tbody>
        </table>
      </section>
    </section>
  </main>
</body>
</html>"#,
        nonce = nonce,
        display_name = escape_html(&session.display_name),
        current_title = escape_html(&current_title),
        email = escape_html(&session.email),
        folder_field = escape_attr(&folder_field),
        folders_html = folders_html,
        files_html = files_html,
    );

    Html(html).into_response()
}

pub async fn login_page(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> axum::response::Response {
    if resolve_session(&state, &headers).is_some() {
        return Redirect::to("/").into_response();
    }

    Html(render_login_page(&nonce)).into_response()
}

pub async fn password_login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PasswordLoginBody>,
) -> axum::response::Response {
    match state
        .connect_auth
        .authenticate_password(&body.email, &body.password, &state.config.owner_id)
    {
        Ok(identity) => json_with_session_cookie(
            &state,
            identity,
            serde_json::json!({ "authenticated": true }),
        ),
        Err(err) => {
            tracing::warn!(
                email = %body.email,
                error = %err,
                "security: portal password auth failed"
            );
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "authenticated": false, "error": err })),
            )
                .into_response()
        }
    }
}

pub async fn create_qr_challenge(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> axum::response::Response {
    let browser_label = build_browser_label(&state, &headers);
    let challenge = new_pending_qr_challenge(browser_label.clone());
    let challenge_id = challenge.id.clone();
    let expires_at = challenge.expires_at.clone();
    let qr_svg = build_qr_svg(&challenge);

    state.qr_challenges
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(challenge_id.clone(), challenge);

    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "challenge_id": challenge_id,
            "browser_label": browser_label,
            "expires_at": expires_at,
            "qr_svg": qr_svg
        })),
    )
        .into_response()
}

pub async fn poll_qr_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> axum::response::Response {
    let mut challenges = state
        .qr_challenges
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let Some(challenge) = challenges.get_mut(&challenge_id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "status": "missing" })),
        )
            .into_response();
    };
    if challenge.is_expired() {
        challenges.remove(&challenge_id);
        return (
            StatusCode::GONE,
            Json(serde_json::json!({ "status": "expired" })),
        )
            .into_response();
    }
    if challenge.rejected {
        challenges.remove(&challenge_id);
        return (
            StatusCode::OK,
            Json(serde_json::json!({ "status": "rejected" })),
        )
            .into_response();
    }
    if let Some(identity) = challenge.approved_identity.clone() {
        challenges.remove(&challenge_id);
        drop(challenges);
        return json_with_session_cookie(
            &state,
            identity,
            serde_json::json!({ "status": "approved" }),
        );
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({ "status": "pending" })),
    )
        .into_response()
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> axum::response::Response {
    if let Some(token) = read_session_cookie(&headers) {
        state.web_sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(&token);
    }

    let mut response = Redirect::to("/login").into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&expired_session_cookie()).unwrap_or_else(|_| HeaderValue::from_static("")),
    );
    response
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
    mut multipart: Multipart,
) -> axum::response::Response {
    if resolve_session(&state, &headers).is_none() {
        return Redirect::to("/login").into_response();
    }

    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name = String::from("unnamed");
    let mut mime_type = String::from("application/octet-stream");
    let mut parent_id: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name().unwrap_or("") {
            "file" => {
                if let Some(fname) = field.file_name() {
                    file_name = fname.to_string();
                }
                if let Some(ct) = field.content_type() {
                    mime_type = sanitize_mime_type(ct);
                }
                match field.bytes().await {
                    Ok(bytes) => file_data = Some(bytes.to_vec()),
                    Err(err) => {
                        return Html(render_error_page("MiyuCloud", &format!("Upload invalide: {err}"), &nonce))
                            .into_response();
                    }
                }
            }
            "parent_id" => {
                if let Ok(text) = field.text().await {
                    if validate_uuid(&text) {
                        parent_id = Some(text);
                    }
                }
            }
            _ => {}
        }
    }

    let Some(data) = file_data else {
        return Html(render_error_page("MiyuCloud", "Aucun fichier fourni.", &nonce)).into_response();
    };

    match FileOps::upload_file(
        &state.db,
        &state.storage,
        &state.key_manager,
        &state.config.owner_id,
        parent_id.as_deref(),
        &file_name,
        &mime_type,
        &data,
    ) {
        Ok(_) => redirect_to_folder(parent_id.as_deref()),
        Err(err) => Html(render_error_page("MiyuCloud", &format!("Upload impossible: {err}"), &nonce))
            .into_response(),
    }
}

pub async fn create_folder(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
    Form(form): Form<CreateFolderForm>,
) -> axum::response::Response {
    if resolve_session(&state, &headers).is_none() {
        return Redirect::to("/login").into_response();
    }
    if form.name.trim().is_empty() {
        return Html(render_error_page("MiyuCloud", "Nom de dossier vide.", &nonce)).into_response();
    }
    let parent_id = form.parent_id.as_deref().filter(|id| validate_uuid(id));
    match FileOps::create_folder(&state.db, &state.config.owner_id, parent_id, form.name.trim()) {
        Ok(_) => redirect_to_folder(parent_id),
        Err(err) => Html(render_error_page("MiyuCloud", &format!("Création impossible: {err}"), &nonce))
            .into_response(),
    }
}

pub async fn download_file(
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
    headers: HeaderMap,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> axum::response::Response {
    if resolve_session(&state, &headers).is_none() {
        return Redirect::to("/login").into_response();
    }
    if !validate_uuid(&file_id) {
        return Html(render_error_page("MiyuCloud", "Identifiant de fichier invalide.", &nonce)).into_response();
    }

    match FileOps::download_file(&state.db, &state.storage, &state.key_manager, &file_id) {
        Ok(data) => {
            let mime = state
                .db
                .file_by_id(&file_id)
                .ok()
                .flatten()
                .map_or_else(|| "application/octet-stream".to_string(), |entry| entry.mime_type);
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, sanitize_mime_type(&mime))
                .header(header::CONTENT_LENGTH, data.len().to_string())
                .body(Body::from(data))
                .unwrap_or_else(|_| Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("internal error")).expect("fallback response"))
                .into_response()
        }
        Err(err) => Html(render_error_page("MiyuCloud", &format!("Téléchargement impossible: {err}"), &nonce))
            .into_response(),
    }
}

fn resolve_session(state: &Arc<AppState>, headers: &HeaderMap) -> Option<WebPortalSession> {
    let token = read_session_cookie(headers)?;
    let mut sessions = state
        .web_sessions
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    sessions.retain(|_, session| !session.is_expired());
    sessions.get(&token).cloned()
}

fn json_with_session_cookie(
    state: &Arc<AppState>,
    identity: PortalIdentity,
    body: serde_json::Value,
) -> axum::response::Response {
    let token = Uuid::new_v4().simple().to_string();
    state.web_sessions
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(
            token.clone(),
            WebPortalSession {
                token: token.clone(),
                subject_id: identity.subject_id,
                email: identity.email,
                display_name: identity.display_name,
                expires_at_unix: identity.connect_session.absolute_expires_unix,
            },
        );

    let mut response = (StatusCode::OK, Json(body)).into_response();
    let cookie = session_cookie(&token);
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    response
}

fn build_browser_label(state: &Arc<AppState>, headers: &HeaderMap) -> String {
    let ip_fragment = access_log::extract_client_ip(headers, state.config.trust_proxy)
        .map(|ip| access_log::hash_ip(&ip, &state.ip_salt))
        .map(|hash| hash.chars().take(10).collect::<String>())
        .unwrap_or_else(|| "inconnu".to_string());
    let ua = access_log::extract_user_agent(headers)
        .map(|ua| access_log::truncate_user_agent(&ua))
        .unwrap_or_else(|| "Navigateur web".to_string());
    format!("{ua} • {ip_fragment}")
}

fn build_qr_svg(challenge: &PendingQrChallenge) -> String {
    let payload = format!(
        "miyukini-connect://miyucloud-web-login?challenge_id={}",
        challenge.id
    );
    QrCode::new(payload.into_bytes())
        .map(|code| code.render::<svg::Color>().min_dimensions(220, 220).build())
        .unwrap_or_else(|_| "<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_string())
}

fn read_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    cookie_header
        .split(';')
        .filter_map(|part| part.trim().split_once('='))
        .find_map(|(name, value)| {
            if name == "miyucloud_web_session" {
                Some(value.to_string())
            } else {
                None
            }
        })
}

fn session_cookie(token: &str) -> String {
    format!(
        "miyucloud_web_session={token}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=28800"
    )
}

fn expired_session_cookie() -> String {
    "miyucloud_web_session=deleted; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0"
        .to_string()
}

fn redirect_to_folder(parent_id: Option<&str>) -> axum::response::Response {
    let target = parent_id
        .map(|id| format!("/?folder={id}"))
        .unwrap_or_else(|| "/".to_string());
    Redirect::to(&target).into_response()
}

fn render_login_page(nonce: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="fr">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>MiyuCloud Login</title>
  <style nonce="{nonce}">
    body {{ margin: 0; font-family: 'Segoe UI', sans-serif; background: radial-gradient(circle at top, #182647, #08111f 65%); color: #eef3fb; min-height: 100vh; display: grid; place-items: center; }}
    main {{ width: min(920px, calc(100vw - 32px)); display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }}
    .card {{ background: rgba(15, 23, 40, 0.94); border: 1px solid #253252; border-radius: 22px; padding: 24px; box-shadow: 0 24px 60px rgba(0,0,0,0.35); }}
    h1, h2 {{ margin: 0 0 12px; }}
    p {{ color: #9db0d2; line-height: 1.5; }}
    input {{ width: 100%; box-sizing: border-box; margin-top: 10px; padding: 12px 14px; border-radius: 12px; border: 1px solid #334263; background: #0d172a; color: #eef3fb; }}
    button {{ margin-top: 12px; width: 100%; padding: 12px 14px; border: none; border-radius: 12px; font-weight: 700; cursor: pointer; background: #2b7cff; color: white; }}
    #qrbox svg {{ width: 220px; height: 220px; background: white; padding: 12px; border-radius: 16px; }}
    #qrbox {{ margin-top: 16px; }}
    .muted {{ color: #8fa1c3; font-size: 14px; }}
    @media (max-width: 820px) {{ main {{ grid-template-columns: 1fr; }} }}
  </style>
</head>
<body>
  <main>
    <section class="card">
      <p class="muted">Miyukini Connect</p>
      <h1>Connexion au Cloud du COG</h1>
      <p>Identifie-toi pour explorer, televerser et telecharger dans MiyuCloud depuis le navigateur.</p>
      <input id="email" type="email" placeholder="Adresse e-mail">
      <input id="password" type="password" placeholder="Mot de passe">
      <button id="btn-login">Se connecter</button>
      <p id="password-status" class="muted"></p>
    </section>
    <section class="card">
      <p class="muted">Connexion QR</p>
      <h2>Scanner puis approuver depuis le COG</h2>
      <p>Le QR initialise une demande Connect. Va ensuite sur MiyuCloud dans Central pour approuver la connexion.</p>
      <button id="btn-qr">Generer un QR</button>
      <div id="qrbox"></div>
      <p id="qr-status" class="muted"></p>
    </section>
  </main>
  <script nonce="{nonce}">
    let qrPoll = null;
    async function loginPassword() {{
      const response = await fetch('/auth/password', {{
        method: 'POST',
        headers: {{ 'Content-Type': 'application/json' }},
        body: JSON.stringify({{
          email: document.getElementById('email').value,
          password: document.getElementById('password').value
        }})
      }});
      const body = await response.json();
      if (response.ok && body.authenticated) {{
        window.location.href = '/';
        return;
      }}
      document.getElementById('password-status').textContent = body.error || 'Connexion impossible';
    }}

    async function startQr() {{
      const response = await fetch('/auth/qr', {{ method: 'POST' }});
      const body = await response.json();
      document.getElementById('qrbox').innerHTML = body.qr_svg || '';
      document.getElementById('qr-status').textContent = 'QR cree pour ' + body.browser_label + '. Expire le ' + body.expires_at;
      if (qrPoll) clearInterval(qrPoll);
      qrPoll = setInterval(async () => {{
        const pollResponse = await fetch('/auth/qr/' + body.challenge_id);
        const pollBody = await pollResponse.json();
        if (pollBody.status === 'approved') {{
          clearInterval(qrPoll);
          window.location.href = '/';
        }}
        if (pollBody.status === 'rejected' || pollBody.status === 'expired' || pollBody.status === 'missing') {{
          clearInterval(qrPoll);
          document.getElementById('qr-status').textContent = 'Demande QR ' + pollBody.status + '.';
        }}
      }}, 2000);
    }}

    document.getElementById('btn-login').addEventListener('click', loginPassword);
    document.getElementById('btn-qr').addEventListener('click', startQr);
  </script>
</body>
</html>"#,
        nonce = nonce
    )
}

fn render_error_page(title: &str, message: &str, nonce: &str) -> String {
    format!(
        "<!doctype html><html lang=\"fr\"><head><meta charset=\"utf-8\"><title>{t}</title><style nonce=\"{n}\">body{{font-family:Segoe UI,sans-serif;background:#0d1524;color:#eef3fb;padding:32px}}a{{color:#7cc4ff}}</style></head><body><h1>{t}</h1><p>{m}</p><p><a href=\"/\">Retour</a></p></body></html>",
        t = escape_html(title),
        m = escape_html(message),
        n = nonce
    )
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_attr(value: &str) -> String {
    escape_html(value)
}
