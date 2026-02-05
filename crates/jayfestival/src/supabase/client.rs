//! Client Supabase pour JayFestival (REST + Auth).
//!
//! @id: supabase_client_init
//! @do: initialise le client Supabase à partir de l'URL du projet et de la clé anon.
//! @layer: infra
//!
//! @id: supabase_client_request
//! @do: exécute une requête HTTP vers l'API REST ou Auth Supabase.
//! @layer: infra

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use std::sync::RwLock;

/// Client HTTP Supabase (REST + Auth).
/// Singleton ou instance partagée : URL projet, clé anon, token de session optionnel.
pub struct SupabaseClient {
    /// URL de base du projet (ex: `https://xxx.supabase.co`).
    base_url: String,
    /// Clé anon (publique) pour les appels API.
    anon_key: String,
    /// Client HTTP réutilisable.
    http: Client,
    /// Token d'accès courant (session). Rempli après sign_in, utilisé pour les requêtes authentifiées.
    access_token: RwLock<Option<String>>,
}

impl SupabaseClient {
    /// Crée un client Supabase (init).
    ///
    /// @id: supabase_client_init
    /// @do: initialise le client Supabase à partir de l'URL du projet et de la clé anon.
    /// @layer: infra
    pub fn new(base_url: impl Into<String>, anon_key: impl Into<String>) -> Self {
        let base_url = base_url.into();
        let anon_key = anon_key.into();
        let http = Client::builder()
            .build()
            .expect("reqwest Client build");
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            anon_key,
            http,
            access_token: RwLock::new(None),
        }
    }

    /// Enregistre le token d'accès après sign_in (utilisé en interne par le module auth).
    pub fn set_access_token(&self, token: Option<String>) {
        if let Ok(mut t) = self.access_token.write() {
            *t = token;
        }
    }

    /// Récupère le token d'accès courant (pour requêtes authentifiées).
    pub fn access_token(&self) -> Option<String> {
        self.access_token.read().ok().and_then(|t| t.clone())
    }

    /// Base URL du projet (sans slash final).
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// En-têtes communs REST : apikey, Authorization Bearer (anon ou access_token), Content-Type.
    fn rest_headers(&self, use_session: bool) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(
            HeaderName::from_static("apikey"),
            HeaderValue::from_str(&self.anon_key).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        let bearer = use_session
            .then(|| self.access_token.read().ok().and_then(|t| t.as_ref().cloned()))
            .flatten()
            .unwrap_or_else(|| self.anon_key.clone());
        h.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", bearer)).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h.insert(
            HeaderName::from_static("accept"),
            HeaderValue::from_static("application/json"),
        );
        h
    }

    /// Requête REST générique (GET/POST/PATCH/DELETE) vers `/rest/v1/{path}`.
    ///
    /// @id: supabase_client_request
    /// @do: exécute une requête HTTP vers l'API REST Supabase.
    /// @layer: infra
    pub fn rest_request(
        &self,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
        use_session: bool,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/rest/v1/{}", self.base_url, path.trim_start_matches('/'));
        let req = self
            .http
            .request(
                method.parse().unwrap_or(reqwest::Method::GET),
                &url,
            )
            .headers(self.rest_headers(use_session));
        let req = if let Some(b) = body {
            req.body(b.to_vec())
        } else {
            req
        };
        req.send()
    }

    /// GET REST (lecture).
    pub fn rest_get(&self, path: &str, use_session: bool) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.rest_request("GET", path, None, use_session)
    }

    /// POST REST (création).
    pub fn rest_post(
        &self,
        path: &str,
        body: &[u8],
        use_session: bool,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.rest_request("POST", path, Some(body), use_session)
    }

    /// PATCH REST (mise à jour partielle).
    pub fn rest_patch(
        &self,
        path: &str,
        body: &[u8],
        use_session: bool,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.rest_request("PATCH", path, Some(body), use_session)
    }

    /// DELETE REST.
    pub fn rest_delete(&self, path: &str, use_session: bool) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.rest_request("DELETE", path, None, use_session)
    }

    /// En-têtes pour l'API Auth Supabase (apikey + Content-Type).
    fn auth_headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(
            HeaderName::from_static("apikey"),
            HeaderValue::from_str(&self.anon_key).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h.insert(
            HeaderName::from_static("accept"),
            HeaderValue::from_static("application/json"),
        );
        h
    }

    /// Requête vers l'API Auth : POST /auth/v1/{path}.
    pub fn auth_post(&self, path: &str, body: &[u8]) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/auth/v1/{}", self.base_url, path.trim_start_matches('/'));
        self.http
            .post(&url)
            .headers(self.auth_headers())
            .body(body.to_vec())
            .send()
    }

    /// Requête GET Auth (ex: GET /auth/v1/user) avec Bearer token.
    pub fn auth_get(&self, path: &str, access_token: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/auth/v1/{}", self.base_url, path.trim_start_matches('/'));
        let mut h = self.auth_headers();
        h.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        self.http.get(&url).headers(h).send()
    }

    /// Requête POST Auth avec Bearer token (ex: logout).
    pub fn auth_post_with_token(
        &self,
        path: &str,
        body: &[u8],
        access_token: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = format!("{}/auth/v1/{}", self.base_url, path.trim_start_matches('/'));
        let mut h = self.auth_headers();
        h.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        self.http.post(&url).headers(h).body(body.to_vec()).send()
    }
}
