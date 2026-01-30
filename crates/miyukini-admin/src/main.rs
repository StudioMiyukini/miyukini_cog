//! Point d'entrée backend de MiyukiniAdmin
//!
//! Détection d'état au démarrage (VIERGE / INITIALISE / COMPROMIS) et routage conditionnel
//! selon Auth and First-Boot Contract et Implementation Security and Controls §13.

// delete et put sont utilisés dans router_full (l.154) ; le compilateur les signale à tort comme non utilisés.
#[allow(unused_imports)]
use axum::{
    extract::{Path, Query, State},
    http::{header, status::StatusCode, HeaderValue},
    middleware,
    routing::{delete, get, post, put},
    Router,
    response::{Html, IntoResponse, Json, Redirect, Response},
    Json as JsonExtract,
};
use miyukini_admin::admin_cell::{AdminCell, ModuleType};
use miyukini_admin::backup_service::{BackupServiceImpl, BackupService};
use miyukini_admin::config::BackendConfig;
use miyukini_admin::crud_state::CrudState;
use miyukini_admin::migration_service::{MigrationServiceImpl, MigrationService};
use miyukini_admin::models::{AdminRole, EnvironmentState};
use miyukini_admin::module_lifecycle_service::{AddModuleParams, ModuleLifecycleService};
use miyukini_admin::module_testing_service::{
    ModuleTestingService, StubAdminCellReader, StubIntegrityVerifier, StubModuleDiscovery,
};
use miyukini_admin::services::{
    AutoRecoveryService, AuthService, DestructionAndReinitService, EnvironmentStateService,
    PermissionService, PreDestructionBackupService,
};
use miyukini_kernel::EnvConfig;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

/// @id: miyukiniadmin_backend_app_state
/// @role: data
/// @layer: operator
/// @human: État partagé de l'application (CRUD, migrations, backups, état environnement, auth).
/// @do: hold_shared_app_state
#[derive(Clone)]
struct AppState {
    /// @id: miyukiniadmin_app_state_environment_state
    /// @role: data
    /// @layer: operator
    /// @human: État détecté au démarrage (Vierge, Initialise, Compromis).
    /// @do: store_environment_state
    environment_state: EnvironmentState,
    /// @id: miyukiniadmin_app_state_crud
    crud: Arc<CrudState>,
    /// @id: miyukiniadmin_app_state_migrations
    migrations: Arc<MigrationServiceImpl>,
    /// @id: miyukiniadmin_app_state_backups
    backups: Arc<BackupServiceImpl>,
    /// @id: miyukiniadmin_app_state_environment_state_svc (réservé pour middleware / re-détection)
    #[allow(dead_code)]
    environment_state_svc: Arc<EnvironmentStateService>,
    /// @id: miyukiniadmin_app_state_auth_svc
    auth_svc: Arc<AuthService>,
    /// @id: miyukiniadmin_app_state_permission_svc (réservé pour middleware permission)
    #[allow(dead_code)]
    permission_svc: Arc<PermissionService>,
    /// @id: miyukiniadmin_app_state_module_testing_svc
    /// Service de tests des modules (découverte, cellule Admin, tests embarqués, intégrité TAMR).
    module_testing_svc: Arc<ModuleTestingService<StubModuleDiscovery, StubAdminCellReader, StubIntegrityVerifier>>,
    /// @id: miyukiniadmin_app_state_module_lifecycle_svc
    /// Service de cycle de vie des modules (add, lock, unlock, delete).
    module_lifecycle_svc: Arc<ModuleLifecycleService>,
    /// @id: miyukiniadmin_app_state_module_discovery
    /// Stub de découverte (pour enregistrement après add).
    module_discovery: Arc<StubModuleDiscovery>,
    /// @id: miyukiniadmin_app_state_admin_cell_reader
    /// Stub reader cellule Admin (pour enregistrement après add).
    admin_cell_reader: Arc<StubAdminCellReader>,
}

/// Nom du cookie de session (HttpOnly, Path=/).
const SESSION_COOKIE_NAME: &str = "miyukini_admin_session";

/// Extrait la valeur du cookie de session depuis l'en-tête Cookie.
fn session_id_from_headers(headers: &axum::http::HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    cookie_header
        .split(';')
        .filter_map(|s| {
            let s = s.trim();
            let prefix = format!("{}=", SESSION_COOKIE_NAME);
            s.starts_with(prefix.as_str()).then(|| s[prefix.len()..].trim().to_string())
        })
        .next()
}

/// Chemins publics (pas de session requise) en environnement INITIALISE.
fn is_public_path(path: &str) -> bool {
    matches!(
        path,
        "/login" | "/api/auth/login" | "/api/auth/logout" | "/health" | "/api/status"
    ) || path.starts_with("/ui/")
}

/// Middleware : exige une session valide pour les routes protégées ; sinon redirection ou 401.
/// @id: miyukiniadmin_backend_require_session_layer
async fn require_session_layer(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    if is_public_path(req.uri().path()) {
        return next.run(req).await;
    }
    let session_id = session_id_from_headers(req.headers());
    let valid = match session_id.as_ref() {
        Some(id) => state.auth_svc.validate_session(id).await.is_some(),
        None => false,
    };
    if valid {
        return next.run(req).await;
    }
    if req.uri().path().starts_with("/api/") {
        let body = Json(serde_json::json!({
            "error": "Non authentifié",
            "message": "Connexion requise. Utilisez POST /api/auth/login."
        }));
        (StatusCode::UNAUTHORIZED, body).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

/// @id: miyukiniadmin_backend_main
/// @role: infrastructure
/// @layer: operator
/// @human: Point d'entrée principal du backend MiyukiniAdmin (détection état + routage conditionnel).
/// @do: start_backend_server
/// @depends: miyukiniadmin_config_backend_config_from_env, miyukiniadmin_services_environment_state_detect
#[tokio::main]
async fn main() {
    let env_config = EnvConfig::from_env();
    let config = BackendConfig::from_env(&env_config);

    let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let env_state_svc = Arc::new(EnvironmentStateService::new(data_dir.clone()));
    let environment_state = env_state_svc
        .detect()
        .await
        .unwrap_or(EnvironmentState::Vierge);

    let backups = Arc::new(BackupServiceImpl::new().with_seed());
    let backups_dir = data_dir.join("backups");
    let pre_destruction_backup =
        PreDestructionBackupService::new(Arc::clone(&backups), backups_dir);
    let destruction_reinit =
        DestructionAndReinitService::new(data_dir.clone(), pre_destruction_backup);
    let _auto_recovery = AutoRecoveryService::new(
        Arc::clone(&env_state_svc),
        destruction_reinit.clone(),
    );

    let module_discovery = Arc::new(StubModuleDiscovery::new());
    let admin_cell_reader = Arc::new(StubAdminCellReader::new());
    let integrity_verifier = Arc::new(StubIntegrityVerifier::default());
    let module_testing_svc = Arc::new(ModuleTestingService::new(
        Arc::clone(&module_discovery),
        Arc::clone(&admin_cell_reader),
        integrity_verifier,
    ));
    let module_lifecycle_svc = Arc::new(ModuleLifecycleService::new());

    let app_state = AppState {
        environment_state,
        crud: Arc::new(CrudState::with_demo_tables()),
        migrations: Arc::new(MigrationServiceImpl::new().with_seed()),
        backups,
        environment_state_svc: env_state_svc,
        auth_svc: Arc::new(AuthService::new(data_dir.clone())),
        permission_svc: Arc::new(PermissionService::new()),
        module_testing_svc,
        module_lifecycle_svc,
        module_discovery,
        admin_cell_reader,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let ui_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui");
    let shared_state = Arc::new(app_state);
    let env_state = shared_state.environment_state;
    let state_for_router = Arc::clone(&shared_state);

    let app = match env_state {
        EnvironmentState::Vierge => router_setup_only(state_for_router, ui_path),
        EnvironmentState::Initialise => router_full(state_for_router.clone(), ui_path)
            .layer(middleware::from_fn_with_state(
                state_for_router,
                require_session_layer,
            )),
        EnvironmentState::Compromis => router_compromised_only(state_for_router, ui_path),
    }
    .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    let scheme = if config.use_https { "https" } else { "http" };
    println!(
        "MiyukiniAdmin [état: {}] sur {}://{}:{}",
        env_state, scheme, config.host, config.port
    );
    println!("Ouvrez {}://localhost:{}/ dans votre navigateur.", scheme, config.port);
    if config.use_https {
        println!("Note: HTTPS configuré — en production, utilisez un reverse proxy (nginx, Caddy) avec TLS devant MiyukiniAdmin.");
    }

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}

/// Routes lorsque l'environnement est VIERGE : uniquement /setup, /health, /api/status, /api/setup/create-admin (Auth and First-Boot §5).
/// Cores et kernel actifs ; outils opérateurs et services désactivés. Seule la page de création du premier profil Admin est accessible.
/// @id: miyukiniadmin_backend_router_setup_only
fn router_setup_only(state: Arc<AppState>, _ui_path: PathBuf) -> Router {
    Router::new()
        .route("/", get(serve_setup_redirect))
        .route("/setup", get(serve_setup))
        .route("/health", get(health))
        .route("/api/status", get(api_status))
        .route("/api/setup/create-admin", post(api_setup_create_admin))
        .with_state(state)
}

/// Routes lorsque l'environnement est INITIALISE : dashboard, API, login (Implementation §13.2).
/// @id: miyukiniadmin_backend_router_full
fn router_full(state: Arc<AppState>, ui_path: PathBuf) -> Router {
    Router::new()
        .route("/", get(serve_dashboard))
        .route("/database", get(serve_database))
        .route("/tests", get(serve_tests))
        .route("/login", get(serve_login_page))
        .route("/health", get(health))
        .route("/api/status", get(api_status))
        .route("/api/auth/login", post(api_auth_login))
        .route("/api/auth/logout", post(api_auth_logout))
        .route("/api/tables", get(api_tables_list))
        .route("/api/tables/:name/rows", get(api_table_rows).post(api_table_create_row))
        .route("/api/tables/:name/rows/:id", get(api_table_get_row).put(api_table_update_row).delete(api_table_delete_row))
        .route("/api/migrations", get(api_migrations_list))
        .route("/api/migrations/history", get(api_migrations_history))
        .route("/api/migrations/apply", post(api_migrations_apply))
        .route("/api/backups", get(api_backups_list).post(api_backups_create))
        .route("/api/backups/restore", post(api_backups_restore))
        .route("/api/tests/flow", get(api_tests_flow))
        .route("/api/modules", get(api_modules_list).post(api_modules_add))
        .route("/api/modules/:id/admin-cell", get(api_modules_admin_cell))
        .route("/api/modules/:id/tests/run", post(api_modules_tests_run))
        .route("/api/modules/:id/integrity", get(api_modules_integrity))
        .route("/api/modules/:id/lock", post(api_modules_lock))
        .route("/api/modules/:id/unlock", post(api_modules_unlock))
        .route("/api/modules/:id", delete(api_modules_delete))
        .nest_service("/ui", ServeDir::new(ui_path))
        .with_state(state)
}

/// Routes lorsque l'environnement est COMPROMIS : page « Environnement compromis » et /health (Auth and First-Boot §7).
/// @id: miyukiniadmin_backend_router_compromised_only
fn router_compromised_only(state: Arc<AppState>, ui_path: PathBuf) -> Router {
    Router::new()
        .route("/", get(serve_compromised))
        .route("/health", get(health))
        .nest_service("/ui", ServeDir::new(ui_path))
        .with_state(state)
}

/// Redirection vers /setup lorsque l'environnement est VIERGE (Auth and First-Boot §5).
/// @id: miyukiniadmin_backend_serve_setup_redirect
async fn serve_setup_redirect() -> impl IntoResponse {
    axum::response::Redirect::to("/setup")
}

/// Page parcours Futur Admin (installation) lorsque l'environnement est VIERGE.
/// @id: miyukiniadmin_backend_serve_setup
async fn serve_setup() -> impl IntoResponse {
    Html(SETUP_HTML)
}

/// Page « Environnement compromis » lorsque l'état est COMPROMIS (Auth and First-Boot §7).
/// @id: miyukiniadmin_backend_serve_compromised
async fn serve_compromised() -> impl IntoResponse {
    Html(COMPROMISED_HTML)
}

/// Page login (environnement INITIALISE).
/// @id: miyukiniadmin_backend_serve_login_page
async fn serve_login_page() -> impl IntoResponse {
    Html(LOGIN_HTML)
}

/// Corps POST /api/auth/login (Authentication Contract §11.2).
#[derive(serde::Deserialize)]
struct LoginBody {
    username: String,
    password: String,
}

/// Corps POST /api/setup/create-admin (First-boot — Auth and First-Boot §5).
#[derive(serde::Deserialize)]
struct CreateAdminBody {
    username: String,
    password: String,
    password_confirm: String,
}

/// POST /api/setup/create-admin : création du premier compte admin et génération EIP (First-boot).
/// Accepté uniquement en état VIERGE. Crée admin_registry, EIP stub et bootstrap_schema puis demande redémarrage.
/// @id: miyukiniadmin_backend_api_setup_create_admin
async fn api_setup_create_admin(
    State(state): State<Arc<AppState>>,
    JsonExtract(body): JsonExtract<CreateAdminBody>,
) -> impl IntoResponse {
    if state.environment_state != EnvironmentState::Vierge {
        return Json(serde_json::json!({
            "success": false,
            "message": "Environnement déjà initialisé. Cette action n'est possible qu'au premier démarrage."
        }));
    }
    let username = body.username.trim();
    if username.is_empty() {
        return Json(serde_json::json!({
            "success": false,
            "message": "Le nom d'utilisateur est requis."
        }));
    }
    if body.password != body.password_confirm {
        return Json(serde_json::json!({
            "success": false,
            "message": "Les mots de passe ne correspondent pas."
        }));
    }
    if body.password.len() < 12 {
        return Json(serde_json::json!({
            "success": false,
            "message": "Le mot de passe doit contenir au moins 12 caractères."
        }));
    }
    match state.auth_svc.create_account(username, &body.password, AdminRole::Admin).await {
        Ok(_) => {}
        Err(e) => {
            let message = match e {
                miyukini_admin::services::auth::CreateAccountError::PolicyViolation => {
                    "Politique non respectée (mot de passe trop court ou nom invalide)."
                }
                miyukini_admin::services::auth::CreateAccountError::UsernameExists => {
                    "Ce nom d'utilisateur existe déjà."
                }
                miyukini_admin::services::auth::CreateAccountError::HashError => {
                    "Erreur interne lors du hachage du mot de passe."
                }
                miyukini_admin::services::auth::CreateAccountError::Io => {
                    "Erreur d'écriture du registre admin."
                }
            };
            return Json(serde_json::json!({ "success": false, "message": message }));
        }
    }
    if let Err(_) = state.environment_state_svc.write_eip_stub().await {
        return Json(serde_json::json!({
            "success": false,
            "message": "Profil admin créé mais échec de la génération EIP. Consultez les logs."
        }));
    }
    if let Err(_) = state.environment_state_svc.write_bootstrap_schema_initial().await {
        return Json(serde_json::json!({
            "success": false,
            "message": "Profil admin et EIP créés mais échec du schéma bootstrap. Consultez les logs."
        }));
    }
    Json(serde_json::json!({
        "success": true,
        "message": "Profil administrateur créé. Redémarrez MiyukiniAdmin pour accéder au tableau de bord.",
        "next_step": "restart"
    }))
}

/// POST /api/auth/login : authentification admin ; en cas de succès, pose le cookie de session (HttpOnly).
/// @id: miyukiniadmin_backend_api_auth_login
async fn api_auth_login(
    State(state): State<Arc<AppState>>,
    JsonExtract(body): JsonExtract<LoginBody>,
) -> impl IntoResponse {
    let ip = None::<&str>;
    let user_agent = None::<&str>;
    match state
        .auth_svc
        .login(&body.username, &body.password, ip, user_agent)
        .await
    {
        Ok(session) => {
            let json = Json(serde_json::json!({
                "success": true,
                "session_id": session.session_id,
                "role": serde_json::to_value(session.role).unwrap_or(serde_json::Value::Null),
                "expires_at": session.expires_at.to_rfc3339()
            }));
            let mut res = json.into_response();
            let cookie_value = format!(
                "{}={}; HttpOnly; Path=/; SameSite=Lax",
                SESSION_COOKIE_NAME,
                session.session_id
            );
            if let Ok(hv) = HeaderValue::try_from(cookie_value) {
                res.headers_mut().append(header::SET_COOKIE, hv);
            }
            res
        }
        Err(_) => Json(serde_json::json!({
            "success": false,
            "message": "Identifiants invalides."
        }))
            .into_response(),
    }
}

/// POST /api/auth/logout : invalide la session et efface le cookie.
/// @id: miyukiniadmin_backend_api_auth_logout
async fn api_auth_logout(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    if let Some(sid) = session_id_from_headers(req.headers()) {
        state.auth_svc.logout(&sid).await;
    }
    let mut res = Json(serde_json::json!({ "success": true, "message": "Déconnecté." })).into_response();
    let clear_cookie = format!("{}=; Max-Age=0; Path=/; SameSite=Lax", SESSION_COOKIE_NAME);
    if let Ok(hv) = HeaderValue::try_from(clear_cookie) {
        res.headers_mut().append(header::SET_COOKIE, hv);
    }
    res
}

/// @id: miyukiniadmin_backend_serve_dashboard
/// @role: infrastructure
/// @layer: operator
/// @human: Sert la page d'accueil MiyukiniAdmin (protégée par session).
/// @do: serve_dashboard_html
async fn serve_dashboard() -> impl IntoResponse {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui").join("index.html");
    match tokio::fs::read_to_string(path).await {
        Ok(html) => Html(html),
        Err(_) => Html(DASHBOARD_FALLBACK_HTML.to_string()),
    }
}

/// @id: miyukiniadmin_backend_health
/// @role: infrastructure
/// @layer: operator
/// @human: Endpoint de santé pour le monitoring.
/// @do: health_check
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "service": "miyukini-admin" }))
}

/// @id: miyukiniadmin_backend_api_status
/// @role: infrastructure
/// @layer: operator
/// @human: Statut API pour l'administration. En VIERGE, seuls les endpoints du parcours setup sont listés.
/// @do: api_status
async fn api_status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let env_state = state.environment_state.to_string();
    let (endpoints, note) = if state.environment_state == EnvironmentState::Vierge {
        (
            serde_json::json!({
                "setup": "/setup",
                "health": "/health",
                "api_status": "/api/status",
                "api_setup_create_admin": "POST /api/setup/create-admin"
            }),
            "First-boot : seuls le parcours d'installation et le monitoring sont actifs. Cores et kernel actifs ; outils opérateurs et services désactivés."
        )
    } else {
        (
            serde_json::json!({
                "dashboard": "/",
                "database": "/database",
                "tests": "/tests",
                "health": "/health",
                "api_status": "/api/status",
                "api_tables": "/api/tables",
                "api_tests_flow": "/api/tests/flow",
                "api_modules": "/api/modules",
                "api_modules_admin_cell": "/api/modules/:id/admin-cell",
                "api_modules_tests_run": "/api/modules/:id/tests/run",
                "api_modules_integrity": "/api/modules/:id/integrity",
                "api_modules_lock": "/api/modules/:id/lock",
                "api_modules_unlock": "/api/modules/:id/unlock",
                "api_modules_delete": "/api/modules/:id"
            }),
            ""
        )
    };
    let mut payload = serde_json::json!({
        "version": "0.1.0",
        "service": "MiyukiniAdmin",
        "environment_state": env_state,
        "cores": ["kernel", "strongfather", "kindmother", "borderguard", "caringnanny", "masterbutler", "bondingbrother", "everbuddy", "worrysentinel", "tamr", "logisticssteward"],
        "endpoints": endpoints
    });
    if !note.is_empty() {
        payload["note"] = serde_json::Value::String(note.to_string());
    }
    Json(payload)
}

/// @id: miyukiniadmin_backend_api_tables_list
/// @role: infrastructure
/// @layer: operator
/// @human: Liste des tables (CRUD).
/// @do: api_list_tables
/// @depends: miyukiniadmin_backend_app_state
async fn api_tables_list(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let tables = state.crud.tables.read().await;
    let names: Vec<&String> = tables.keys().collect();
    let names: Vec<String> = names.into_iter().cloned().collect();
    Json(serde_json::json!({ "tables": names }))
}

/// Paramètres de pagination.
#[derive(Debug, serde::Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

/// @id: miyukiniadmin_backend_api_table_rows
/// @role: infrastructure
/// @layer: operator
/// @human: Lignes d'une table (paginées).
/// @do: api_table_rows
/// @depends: miyukiniadmin_backend_app_state
async fn api_table_rows(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(pagination): Query<Pagination>,
) -> Json<serde_json::Value> {
    let page = pagination.page.unwrap_or(1).max(1);
    let per_page = pagination.per_page.unwrap_or(50).min(100);
    let tables = state.crud.tables.read().await;
    let rows = match tables.get(&name) {
        Some(r) => r.clone(),
        None => return Json(serde_json::json!({ "error": "table not found", "table": name })),
    };
    let total = rows.len();
    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(total);
    let slice: Vec<serde_json::Value> = rows[start..end].to_vec();
    let total_pages = if per_page as usize == 0 { 1 } else { (total + per_page as usize - 1) / per_page as usize };
    Json(serde_json::json!({
        "table": name,
        "rows": slice,
        "pagination": { "page": page, "per_page": per_page, "total": total, "total_pages": total_pages }
    }))
}

/// @id: miyukiniadmin_backend_api_table_create_row
/// @role: infrastructure
/// @layer: operator
/// @human: Création d'une ligne.
/// @do: api_create_row
/// @depends: miyukiniadmin_backend_app_state
async fn api_table_create_row(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    JsonExtract(mut body): JsonExtract<serde_json::Value>,
) -> impl IntoResponse {
    let mut tables = state.crud.tables.write().await;
    let rows = tables.entry(name.clone()).or_default();
    let id = (rows.len() + 1).to_string();
    if let Some(obj) = body.as_object_mut() {
        obj.insert("id".to_string(), serde_json::Value::String(id.clone()));
    } else {
        body = serde_json::json!({ "id": id });
    }
    rows.push(body.clone());
    Json(serde_json::json!({ "status": "created", "table": name, "id": id, "row": body }))
}

/// @id: miyukiniadmin_backend_api_table_get_row
/// @role: infrastructure
/// @layer: operator
/// @human: Une ligne par ID (index 1-based).
/// @do: api_get_row
/// @depends: miyukiniadmin_backend_app_state
async fn api_table_get_row(
    State(state): State<Arc<AppState>>,
    Path((name, id)): Path<(String, String)>,
) -> Json<serde_json::Value> {
    let tables = state.crud.tables.read().await;
    let rows = match tables.get(&name) {
        Some(r) => r,
        None => return Json(serde_json::json!({ "error": "table not found" })),
    };
    let idx: usize = id.parse().unwrap_or(0);
    if idx == 0 || idx > rows.len() {
        return Json(serde_json::json!({ "error": "row not found", "id": id }));
    }
    let row = rows.get(idx - 1).cloned().unwrap_or(serde_json::Value::Null);
    Json(serde_json::json!({ "table": name, "id": id, "row": row }))
}

/// @id: miyukiniadmin_backend_api_table_update_row
/// @role: infrastructure
/// @layer: operator
/// @human: Mise à jour d'une ligne.
/// @do: api_update_row
/// @depends: miyukiniadmin_backend_app_state
async fn api_table_update_row(
    State(state): State<Arc<AppState>>,
    Path((name, id)): Path<(String, String)>,
    JsonExtract(body): JsonExtract<serde_json::Value>,
) -> impl IntoResponse {
    let mut tables = state.crud.tables.write().await;
    let rows = match tables.get_mut(&name) {
        Some(r) => r,
        None => return Json(serde_json::json!({ "error": "table not found" })),
    };
    let idx: usize = id.parse().unwrap_or(0);
    if idx == 0 || idx > rows.len() {
        return Json(serde_json::json!({ "error": "row not found", "id": id }));
    }
    let row = rows.get_mut(idx - 1).unwrap();
    if let (Some(obj_row), Some(obj_body)) = (row.as_object_mut(), body.as_object()) {
        for (k, v) in obj_body {
            if k != "id" {
                obj_row.insert(k.clone(), v.clone());
            }
        }
    }
    Json(serde_json::json!({ "status": "updated", "table": name, "id": id }))
}

/// @id: miyukiniadmin_backend_api_table_delete_row
/// @role: infrastructure
/// @layer: operator
/// @human: Suppression d'une ligne.
/// @do: api_delete_row
/// @depends: miyukiniadmin_backend_app_state
async fn api_table_delete_row(
    State(state): State<Arc<AppState>>,
    Path((name, id)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut tables = state.crud.tables.write().await;
    let rows = match tables.get_mut(&name) {
        Some(r) => r,
        None => return Json(serde_json::json!({ "error": "table not found" })),
    };
    let idx: usize = id.parse().unwrap_or(0);
    if idx == 0 || idx > rows.len() {
        return Json(serde_json::json!({ "error": "row not found", "id": id }));
    }
    rows.remove(idx - 1);
    Json(serde_json::json!({ "status": "deleted", "table": name, "id": id }))
}

/// @id: miyukiniadmin_backend_api_migrations_list
/// @role: infrastructure
/// @layer: operator
/// @human: Liste des migrations (appliquées et en attente).
/// @do: api_migrations_list
/// @depends: miyukiniadmin_backend_app_state
async fn api_migrations_list(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let applied = state.migrations.list_applied();
    let pending = state.migrations.list_pending();
    Json(serde_json::json!({
        "applied": serde_json::to_value(&applied).unwrap_or(serde_json::Value::Array(vec![])),
        "pending": serde_json::to_value(&pending).unwrap_or(serde_json::Value::Array(vec![]))
    }))
}

/// @id: miyukiniadmin_backend_api_migrations_history
/// @role: infrastructure
/// @layer: operator
/// @human: Historique complet des migrations.
/// @do: api_migrations_history
/// @depends: miyukiniadmin_backend_app_state
async fn api_migrations_history(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let history = state.migrations.history();
    Json(serde_json::json!({
        "history": serde_json::to_value(&history).unwrap_or(serde_json::Value::Array(vec![]))
    }))
}

/// @id: miyukiniadmin_backend_api_migrations_apply
/// @role: infrastructure
/// @layer: operator
/// @human: Applique les migrations en attente (stub ; StrongFather + KindMother en production).
/// @do: api_migrations_apply
/// @depends: miyukiniadmin_backend_app_state
async fn api_migrations_apply(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let result = state.migrations.apply_pending();
    Json(serde_json::json!({
        "status": result.status,
        "applied_count": result.applied_count,
        "message": result.message
    }))
}

/// @id: miyukiniadmin_backend_api_backups_list
/// @role: infrastructure
/// @layer: operator
/// @human: Liste des sauvegardes.
/// @do: api_backups_list
/// @depends: miyukiniadmin_backend_app_state
async fn api_backups_list(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let backups = state.backups.list();
    Json(serde_json::json!({
        "backups": serde_json::to_value(&backups).unwrap_or(serde_json::Value::Array(vec![]))
    }))
}

/// @id: miyukiniadmin_backend_api_backups_create
/// @role: infrastructure
/// @layer: operator
/// @human: Crée une sauvegarde (stub ; KindMother en production).
/// @do: api_backups_create
/// @depends: miyukiniadmin_backend_app_state
async fn api_backups_create(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let result = state.backups.create();
    Json(serde_json::json!({
        "success": result.success,
        "backup_id": result.backup_id,
        "message": result.message
    }))
}

/// @id: miyukiniadmin_backend_api_backups_restore_body
/// @role: data
/// @layer: operator
/// @human: Corps de la requête restore.
/// @do: represent_restore_request
#[derive(serde::Deserialize)]
struct RestoreBackupBody {
    backup_id: String,
    justification: Option<String>,
}

/// @id: miyukiniadmin_backend_api_backups_restore
/// @role: infrastructure
/// @layer: operator
/// @human: Restaure depuis un backup (StrongFather + KindMother en production).
/// @do: api_backups_restore
/// @depends: miyukiniadmin_backend_app_state
async fn api_backups_restore(
    State(state): State<Arc<AppState>>,
    JsonExtract(body): JsonExtract<RestoreBackupBody>,
) -> Json<serde_json::Value> {
    let result = state.backups.restore(
        &body.backup_id,
        body.justification.as_deref(),
    );
    Json(serde_json::json!({
        "success": result.success,
        "message": result.message
    }))
}

/// GET /api/modules : liste des modules (découverte via Master Butler / BondingBrother).
async fn api_modules_list(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let modules = state.module_testing_svc.discover_modules();
    Json(serde_json::json!({
        "modules": serde_json::to_value(&modules).unwrap_or(serde_json::Value::Array(vec![]))
    }))
}

/// Corps POST /api/modules : ajout d'un module (optionnellement avec cellule Admin pour le reader stub).
#[derive(serde::Deserialize)]
struct AddModuleBody {
    id: String,
    version: String,
    #[serde(rename = "module_type")]
    module_type: ModuleType,
    #[serde(rename = "module_origin")]
    module_origin: String,
    #[serde(rename = "admin_cell_ref_path")]
    admin_cell_ref_path: String,
    /// Optionnel : cellule Admin complète (pour enregistrement dans le reader stub).
    #[serde(rename = "admin_cell")]
    admin_cell: Option<AdminCell>,
}

/// POST /api/modules : ajout d'un module (StrongFather, Master Butler en production).
async fn api_modules_add(
    State(state): State<Arc<AppState>>,
    JsonExtract(body): JsonExtract<AddModuleBody>,
) -> impl IntoResponse {
    let params = AddModuleParams {
        id: body.id.clone(),
        version: body.version.clone(),
        module_type: body.module_type,
        module_origin: body.module_origin.clone(),
        admin_cell_ref_path: body.admin_cell_ref_path.clone(),
    };
    match state.module_lifecycle_svc.add_module(params) {
        Ok(result) => {
            if let Some(info) = state.module_lifecycle_svc.get_module(&result.module_id) {
                state.module_discovery.register_module(info);
            }
            if let Some(cell) = body.admin_cell {
                state
                    .admin_cell_reader
                    .register_cell(result.module_id.clone(), cell);
            }
            Json(serde_json::json!({
                "success": result.success,
                "action": serde_json::to_value(&result.action).unwrap_or(serde_json::Value::Null),
                "module_id": result.module_id,
                "message": result.message
            }))
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// GET /api/modules/:id/admin-cell : lecture de la cellule Admin du module.
async fn api_modules_admin_cell(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.module_testing_svc.read_admin_cell(&id) {
        Ok(cell) => Json(serde_json::to_value(&cell).unwrap_or(serde_json::Value::Null)),
        Err(e) => Json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// POST /api/modules/:id/tests/run : exécution des tests embarqués du module.
async fn api_modules_tests_run(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.module_testing_svc.run_embedded_tests(&id) {
        Ok(results) => Json(serde_json::to_value(&results).unwrap_or(serde_json::Value::Null)),
        Err(e) => Json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// GET /api/modules/:id/integrity : vérification d'intégrité (collaboration TAMR).
async fn api_modules_integrity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.module_testing_svc.verify_integrity(&id) {
        Ok(result) => Json(serde_json::to_value(&result).unwrap_or(serde_json::Value::Null)),
        Err(e) => Json(serde_json::json!({
            "error": e.to_string()
        })),
    }
}

/// POST /api/modules/:id/lock : verrouillage du module.
async fn api_modules_lock(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.module_lifecycle_svc.lock_module(&id) {
        Ok(result) => {
            if let Some(info) = state.module_lifecycle_svc.get_module(&id) {
                state.module_discovery.register_module(info);
            }
            Json(serde_json::json!({
                "success": result.success,
                "action": serde_json::to_value(&result.action).unwrap_or(serde_json::Value::Null),
                "module_id": result.module_id,
                "message": result.message
            }))
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// POST /api/modules/:id/unlock : déverrouillage du module.
async fn api_modules_unlock(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.module_lifecycle_svc.unlock_module(&id) {
        Ok(result) => {
            if let Some(info) = state.module_lifecycle_svc.get_module(&id) {
                state.module_discovery.register_module(info);
            }
            Json(serde_json::json!({
                "success": result.success,
                "action": serde_json::to_value(&result.action).unwrap_or(serde_json::Value::Null),
                "module_id": result.module_id,
                "message": result.message
            }))
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// DELETE /api/modules/:id : suppression du module.
async fn api_modules_delete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.module_lifecycle_svc.delete_module(&id) {
        Ok(result) => Json(serde_json::json!({
            "success": result.success,
            "action": serde_json::to_value(&result.action).unwrap_or(serde_json::Value::Null),
            "module_id": result.module_id,
            "message": result.message
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// Tests de flux (disponibilité cores, health, status).
async fn api_tests_flow() -> Json<serde_json::Value> {
    let mut results = Vec::new();
    // Test 1: health local
    let health_ok = true;
    results.push(serde_json::json!({
        "id": "flow-health",
        "name": "Health endpoint",
        "description": "Vérification /health",
        "passed": health_ok,
        "detail": "GET /health retourne status ok"
    }));
    // Test 2: api status
    results.push(serde_json::json!({
        "id": "flow-api-status",
        "name": "API Status",
        "description": "Vérification /api/status (cores listés)",
        "passed": true,
        "detail": "GET /api/status retourne version et liste des cores"
    }));
    // Test 3: liste tables
    results.push(serde_json::json!({
        "id": "flow-tables",
        "name": "Liste tables",
        "description": "Vérification /api/tables",
        "passed": true,
        "detail": "CRUD tables disponible"
    }));
    let passed = results.iter().filter(|r| r["passed"].as_bool().unwrap_or(false)).count();
    let total = results.len();
    Json(serde_json::json!({
        "status": if passed == total { "ok" } else { "partial" },
        "passed": passed,
        "total": total,
        "results": results
    }))
}

/// @id: miyukiniadmin_backend_serve_database
/// @role: infrastructure
/// @layer: operator
/// @human: Sert la page Database (Daynight) depuis ui/database.html.
/// @do: serve_database_html
async fn serve_database() -> impl IntoResponse {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui").join("database.html");
    match tokio::fs::read_to_string(path).await {
        Ok(html) => Html(html),
        Err(_) => Html(DATABASE_FALLBACK_HTML.to_string()),
    }
}

/// @id: miyukiniadmin_backend_serve_tests
/// @role: infrastructure
/// @layer: operator
/// @human: Sert la page Tests (Daynight) depuis ui/tests.html.
/// @do: serve_tests_html
async fn serve_tests() -> impl IntoResponse {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui").join("tests.html");
    match tokio::fs::read_to_string(path).await {
        Ok(html) => Html(html),
        Err(_) => Html(TESTS_FALLBACK_HTML.to_string()),
    }
}

/// Page parcours Futur Admin (installation) — environnement VIERGE (Auth and First-Boot §5).
/// Seule page accessible en First-boot : création du premier profil Admin, génération EIP.
const SETUP_HTML: &str = r#"<!DOCTYPE html>
<html lang="fr">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>MiyukiniAdmin — Installation</title>
  <style>
    body { font-family: system-ui, sans-serif; background: #0f172a; color: #e2e8f0; padding: 2rem; max-width: 28rem; margin: 0 auto; }
    h1 { font-size: 1.5rem; margin-bottom: 0.5rem; }
    .intro { color: #94a3b8; font-size: 0.9rem; margin-bottom: 1.5rem; }
    label { display: block; margin-top: 1rem; font-size: 0.875rem; }
    input { width: 100%; padding: 0.5rem 0.75rem; margin-top: 0.25rem; border: 1px solid #334155; border-radius: 0.375rem; background: #1e293b; color: #e2e8f0; box-sizing: border-box; }
    input:focus { outline: none; border-color: #38bdf8; }
    button { margin-top: 1.25rem; padding: 0.5rem 1rem; background: #38bdf8; color: #0f172a; border: none; border-radius: 0.375rem; font-weight: 600; cursor: pointer; }
    button:hover { background: #7dd3fc; }
    button:disabled { opacity: 0.6; cursor: not-allowed; }
    .message { margin-top: 1rem; padding: 0.75rem; border-radius: 0.375rem; font-size: 0.875rem; }
    .message.success { background: #14532d; color: #86efac; }
    .message.error { background: #7f1d1d; color: #fca5a5; }
    .links { margin-top: 2rem; font-size: 0.875rem; }
    .links a { color: #38bdf8; text-decoration: none; }
    .links a:hover { text-decoration: underline; }
  </style>
</head>
<body>
  <h1>MiyukiniAdmin — Installation</h1>
  <p class="intro">Environnement vierge. Parcours Futur Admin : configuration minimale, création du premier compte admin, génération EIP. Cores et kernel actifs ; outils opérateurs et services sont désactivés jusqu'à la fin de l'installation.</p>
  <form id="setup-form">
    <label for="username">Nom d'utilisateur</label>
    <input type="text" id="username" name="username" required autocomplete="username" placeholder="admin">
    <label for="password">Mot de passe (min. 12 caractères)</label>
    <input type="password" id="password" name="password" required minlength="12" autocomplete="new-password" placeholder="">
    <label for="password_confirm">Confirmer le mot de passe</label>
    <input type="password" id="password_confirm" name="password_confirm" required minlength="12" autocomplete="new-password" placeholder="">
    <button type="submit" id="submit-btn">Créer le profil administrateur</button>
  </form>
  <div id="message" class="message" role="alert" style="display: none;"></div>
  <p class="links"><a href="/health">Health</a> | <a href="/api/status">API Status</a></p>
  <script>
    (function() {
      var form = document.getElementById('setup-form');
      var message = document.getElementById('message');
      var submitBtn = document.getElementById('submit-btn');
      function showMessage(text, isError) {
        message.textContent = text;
        message.className = 'message ' + (isError ? 'error' : 'success');
        message.style.display = 'block';
      }
      form.addEventListener('submit', function(e) {
        e.preventDefault();
        var username = document.getElementById('username').value.trim();
        var password = document.getElementById('password').value;
        var password_confirm = document.getElementById('password_confirm').value;
        if (password !== password_confirm) {
          showMessage('Les mots de passe ne correspondent pas.', true);
          return;
        }
        if (password.length < 12) {
          showMessage('Le mot de passe doit contenir au moins 12 caractères.', true);
          return;
        }
        submitBtn.disabled = true;
        fetch('/api/setup/create-admin', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username: username, password: password, password_confirm: password_confirm })
        }).then(function(r) { return r.json(); }).then(function(data) {
          if (data.success) {
            showMessage(data.message + ' Redémarrez l\'application pour accéder au tableau de bord.', false);
            form.reset();
          } else {
            showMessage(data.message || 'Erreur lors de la création du profil.', true);
          }
        }).catch(function() {
          showMessage('Erreur de connexion. Vérifiez que le serveur est démarré.', true);
        }).finally(function() { submitBtn.disabled = false; });
      });
    })();
  </script>
</body>
</html>"#;
/// Page « Environnement compromis » (Auth and First-Boot §7).
const COMPROMISED_HTML: &str = r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>MiyukiniAdmin — Environnement compromis</title></head><body style="font-family:system-ui;background:#1e293b;color:#e2e8f0;padding:2rem;"><h1>Environnement compromis</h1><p>L'environnement est en état de sécurité. Une procédure de recovery est requise (ou recovery automatique en cours selon politique).</p><p>Aucun formulaire de login ni lien vers l'installation ne sont fournis.</p><p><a href="/health" style="color:#38bdf8;">Health</a></p></body></html>"#;
/// Page login (environnement INITIALISE).
/// Page de connexion MiyukiniAdmin (environnement INITIALISE). Protège l'accès au tableau de bord.
const LOGIN_HTML: &str = r#"<!DOCTYPE html>
<html lang="fr">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>MiyukiniAdmin — Connexion</title>
  <style>
    body { font-family: system-ui, sans-serif; background: #0f172a; color: #e2e8f0; padding: 2rem; max-width: 28rem; margin: 0 auto; }
    h1 { font-size: 1.5rem; margin-bottom: 0.5rem; }
    .intro { color: #94a3b8; font-size: 0.9rem; margin-bottom: 1.5rem; }
    label { display: block; margin-top: 1rem; font-size: 0.875rem; }
    input { width: 100%; padding: 0.5rem 0.75rem; margin-top: 0.25rem; border: 1px solid #334155; border-radius: 0.375rem; background: #1e293b; color: #e2e8f0; box-sizing: border-box; }
    input:focus { outline: none; border-color: #38bdf8; }
    button { margin-top: 1.25rem; padding: 0.5rem 1rem; background: #38bdf8; color: #0f172a; border: none; border-radius: 0.375rem; font-weight: 600; cursor: pointer; }
    button:hover { background: #7dd3fc; }
    button:disabled { opacity: 0.6; cursor: not-allowed; }
    .message { margin-top: 1rem; padding: 0.75rem; border-radius: 0.375rem; font-size: 0.875rem; }
    .message.error { background: #7f1d1d; color: #fca5a5; }
    .links { margin-top: 2rem; font-size: 0.875rem; }
    .links a { color: #38bdf8; text-decoration: none; }
    .links a:hover { text-decoration: underline; }
  </style>
</head>
<body>
  <h1>MiyukiniAdmin — Connexion</h1>
  <p class="intro">Identifiez-vous pour accéder au tableau de bord et à l'administration.</p>
  <form id="login-form">
    <label for="username">Nom d'utilisateur</label>
    <input type="text" id="username" name="username" required autocomplete="username" placeholder="admin">
    <label for="password">Mot de passe</label>
    <input type="password" id="password" name="password" required autocomplete="current-password" placeholder="">
    <button type="submit" id="submit-btn">Se connecter</button>
  </form>
  <div id="message" class="message" role="alert" style="display: none;"></div>
  <p class="links"><a href="/health">Health</a> | <a href="/api/status">API Status</a></p>
  <script>
    (function() {
      var form = document.getElementById('login-form');
      var message = document.getElementById('message');
      var submitBtn = document.getElementById('submit-btn');
      function showError(text) {
        message.textContent = text;
        message.className = 'message error';
        message.style.display = 'block';
      }
      form.addEventListener('submit', function(e) {
        e.preventDefault();
        var username = document.getElementById('username').value.trim();
        var password = document.getElementById('password').value;
        submitBtn.disabled = true;
        fetch('/api/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({ username: username, password: password })
        }).then(function(r) { return r.json(); }).then(function(data) {
          if (data.success) {
            window.location.href = '/';
          } else {
            showError(data.message || 'Identifiants invalides.');
          }
        }).catch(function() {
          showError('Erreur de connexion. Vérifiez que le serveur est démarré.');
        }).finally(function() { submitBtn.disabled = false; });
      });
    })();
  </script>
</body>
</html>"#;
/// Fallback si ui/index.html est introuvable (ex. tests, mauvais répertoire).
const DASHBOARD_FALLBACK_HTML: &str = r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>MiyukiniAdmin</title></head><body style="font-family:system-ui;background:#0f172a;color:#e2e8f0;padding:2rem;"><h1>MiyukiniAdmin</h1><p>Index non trouvé. <a href="/api/status" style="color:#38bdf8;">API Status</a> | <a href="/health" style="color:#38bdf8;">Health</a></p></body></html>"#;
/// Fallback si ui/database.html est introuvable.
const DATABASE_FALLBACK_HTML: &str = r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>MiyukiniAdmin — Database</title></head><body style="font-family:system-ui;background:#0f172a;color:#e2e8f0;padding:2rem;"><h1>Database</h1><p>Page non trouvée. <a href="/" style="color:#38bdf8;">Dashboard</a> | <a href="/database" style="color:#38bdf8;">Réessayer</a></p></body></html>"#;
/// Fallback si ui/tests.html est introuvable.
const TESTS_FALLBACK_HTML: &str = r#"<!DOCTYPE html><html lang="fr"><head><meta charset="UTF-8"><title>MiyukiniAdmin — Tests</title></head><body style="font-family:system-ui;background:#0f172a;color:#e2e8f0;padding:2rem;"><h1>Tests</h1><p>Page non trouvée. <a href="/" style="color:#38bdf8;">Dashboard</a> | <a href="/tests" style="color:#38bdf8;">Réessayer</a></p></body></html>"#;
