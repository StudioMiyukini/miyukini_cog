//! Client HTTP haut niveau pour les dispositifs domotiques locaux.
//!
//! Timeout 2 secondes par defaut, 1 retry automatique sur echec temporaire.
//! Support de l'authentification Basic et Bearer.

// @id: toolkit.alicia.http.client
// @role: http_device_client
// @layer: 6
// @human: Client HTTP pour dispositifs locaux Alicia ; timeout, retry, auth Basic/Bearer.
// @do: provide_http_client_for_local_devices

use std::time::Duration;

use crate::errors::HttpError;

/// Configuration d'authentification pour un dispositif HTTP.
#[derive(Debug, Clone)]
pub enum AuthConfig {
    /// Pas d'authentification.
    None,
    /// Authentification HTTP Basic (utilisateur:mot_de_passe).
    Basic {
        /// Nom d'utilisateur.
        username: String,
        /// Mot de passe.
        password: String,
    },
    /// Authentification Bearer Token.
    Bearer {
        /// Token d'acces.
        token: String,
    },
}

/// Client HTTP pour les dispositifs domotiques locaux.
///
/// Utilise `reqwest` avec un timeout de 2 secondes et 1 retry automatique
/// sur les erreurs temporaires (timeout, connexion refusee).
#[derive(Debug, Clone)]
pub struct HttpDeviceClient {
    client: reqwest::Client,
    timeout: Duration,
    max_retries: u32,
}

impl Default for HttpDeviceClient {
    fn default() -> Self {
        Self::new(Duration::from_secs(2), 1)
    }
}

impl HttpDeviceClient {
    /// Cree un nouveau client HTTP avec le timeout et le nombre de retries specifies.
    pub fn new(timeout: Duration, max_retries: u32) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_default();
        Self {
            client,
            timeout,
            max_retries,
        }
    }

    /// Execute un GET sur l'URL specifiee et retourne la reponse JSON.
    ///
    /// Applique l'authentification si fournie.
    /// Retente automatiquement en cas de timeout ou d'erreur de connexion.
    pub async fn get_json(
        &self,
        url: &str,
        auth: &AuthConfig,
    ) -> Result<serde_json::Value, HttpError> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                tracing::debug!("Retry {attempt}/{} pour GET {url}", self.max_retries);
            }

            let mut req = self.client.get(url);
            req = apply_auth(req, auth);

            match req.send().await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return response.json::<serde_json::Value>().await.map_err(|e| {
                            HttpError::ParseError {
                                url: url.to_string(),
                                message: e.to_string(),
                            }
                        });
                    }

                    let body = response
                        .text()
                        .await
                        .unwrap_or_default();
                    // Tronquer a 512 caracteres
                    let body_truncated = if body.len() > 512 {
                        format!("{}...", &body[..512])
                    } else {
                        body
                    };

                    if status.as_u16() == 401 || status.as_u16() == 403 {
                        return Err(HttpError::AuthenticationFailed {
                            url: url.to_string(),
                            message: body_truncated,
                        });
                    }

                    return Err(HttpError::HttpStatus {
                        url: url.to_string(),
                        status: status.as_u16(),
                        body: body_truncated,
                    });
                }
                Err(e) => {
                    if e.is_timeout() {
                        last_error = Some(HttpError::Timeout {
                            url: url.to_string(),
                            timeout_ms: self.timeout.as_millis() as u64,
                        });
                    } else if e.is_connect() {
                        last_error = Some(HttpError::ConnectionFailed {
                            url: url.to_string(),
                            message: e.to_string(),
                        });
                    } else {
                        return Err(HttpError::ReqwestError(e));
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| HttpError::ConnectionFailed {
            url: url.to_string(),
            message: "toutes les tentatives ont echoue".to_string(),
        }))
    }

    /// Execute un POST JSON sur l'URL specifiee et retourne la reponse JSON.
    ///
    /// Applique l'authentification si fournie.
    /// Retente automatiquement en cas de timeout ou d'erreur de connexion.
    pub async fn post_json(
        &self,
        url: &str,
        body: &serde_json::Value,
        auth: &AuthConfig,
    ) -> Result<serde_json::Value, HttpError> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                tracing::debug!("Retry {attempt}/{} pour POST {url}", self.max_retries);
            }

            let mut req = self.client.post(url).json(body);
            req = apply_auth(req, auth);

            match req.send().await {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return response.json::<serde_json::Value>().await.map_err(|e| {
                            HttpError::ParseError {
                                url: url.to_string(),
                                message: e.to_string(),
                            }
                        });
                    }

                    let body_text = response
                        .text()
                        .await
                        .unwrap_or_default();
                    let body_truncated = if body_text.len() > 512 {
                        format!("{}...", &body_text[..512])
                    } else {
                        body_text
                    };

                    if status.as_u16() == 401 || status.as_u16() == 403 {
                        return Err(HttpError::AuthenticationFailed {
                            url: url.to_string(),
                            message: body_truncated,
                        });
                    }

                    return Err(HttpError::HttpStatus {
                        url: url.to_string(),
                        status: status.as_u16(),
                        body: body_truncated,
                    });
                }
                Err(e) => {
                    if e.is_timeout() {
                        last_error = Some(HttpError::Timeout {
                            url: url.to_string(),
                            timeout_ms: self.timeout.as_millis() as u64,
                        });
                    } else if e.is_connect() {
                        last_error = Some(HttpError::ConnectionFailed {
                            url: url.to_string(),
                            message: e.to_string(),
                        });
                    } else {
                        return Err(HttpError::ReqwestError(e));
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| HttpError::ConnectionFailed {
            url: url.to_string(),
            message: "toutes les tentatives ont echoue".to_string(),
        }))
    }

    /// Retourne le timeout configure.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Retourne le nombre maximum de retries.
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

/// Applique la configuration d'authentification a un `RequestBuilder`.
fn apply_auth(
    req: reqwest::RequestBuilder,
    auth: &AuthConfig,
) -> reqwest::RequestBuilder {
    match auth {
        AuthConfig::None => req,
        AuthConfig::Basic { username, password } => req.basic_auth(username, Some(password)),
        AuthConfig::Bearer { token } => req.bearer_auth(token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_default() {
        let client = HttpDeviceClient::default();
        assert_eq!(client.timeout(), Duration::from_secs(2));
        assert_eq!(client.max_retries(), 1);
    }

    #[test]
    fn test_client_custom_config() {
        let client = HttpDeviceClient::new(Duration::from_millis(500), 3);
        assert_eq!(client.timeout(), Duration::from_millis(500));
        assert_eq!(client.max_retries(), 3);
    }

    #[tokio::test]
    async fn test_get_json_connection_refused() {
        let client = HttpDeviceClient::new(Duration::from_millis(100), 0);
        // Adresse locale qui ne repond pas
        let result = client
            .get_json("http://127.0.0.1:1/test", &AuthConfig::None)
            .await;
        assert!(result.is_err());
    }
}
