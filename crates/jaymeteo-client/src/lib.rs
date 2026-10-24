//! # jaymeteo-client
//!
//! Adaptateur entre Alicia (via `miyualicia_dispatcher::ServiceClient`) et un
//! fournisseur de données météo (par défaut [Open-Meteo](https://open-meteo.com/),
//! gratuit et sans clé API).
//!
//! Capabilités exposées (cf. `crates/jaymeteo/service.manifest.json`) :
//! - `query_current_weather` (read, jamais de confirmation)
//! - `query_forecast` (read, jamais de confirmation)
//!
//! Le **backend** (`JayMeteoBackend`) est un trait async — l'implémentation
//! réelle (HTTP Open-Meteo, capteurs domotiques, in-process) sera branchée au
//! runtime. Ce crate livre un `FakeJayMeteoBackend` programmable pour tests.

#![doc(html_root_url = "https://docs.miyukini.com/jaymeteo-client")]

// @id service.jaymeteo.alicia_client
// @role alicia_to_jaymeteo_adapter
// @layer 7
// @do bridge_alicia_to_jaymeteo
// @human Adaptateur Alicia → JayMeteo : route les intents vers un backend pluggable.

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Conditions météo agrégées (mappées depuis les codes WMO d'Open-Meteo).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WeatherCondition {
    /// Ciel dégagé / ensoleillé.
    Clear,
    /// Nuageux / partiellement nuageux.
    Cloudy,
    /// Pluie (faible à forte) ou bruine.
    Rain,
    /// Neige.
    Snow,
    /// Orage.
    Storm,
    /// Brouillard / brume.
    Fog,
}

impl WeatherCondition {
    /// Convertit un code météo WMO (Open-Meteo) en condition agrégée.
    ///
    /// Référence : <https://open-meteo.com/en/docs#weathervariables>
    #[must_use]
    pub fn from_wmo_code(code: u16) -> Self {
        match code {
            0 | 1 => Self::Clear,
            2 | 3 => Self::Cloudy,
            45 | 48 => Self::Fog,
            51..=57 | 61..=67 | 80..=82 => Self::Rain,
            71..=77 | 85 | 86 => Self::Snow,
            95..=99 => Self::Storm,
            _ => Self::Cloudy,
        }
    }

    /// Étiquette française.
    #[must_use]
    pub fn label_fr(self) -> &'static str {
        match self {
            Self::Clear => "Ensoleillé",
            Self::Cloudy => "Nuageux",
            Self::Rain => "Pluie",
            Self::Snow => "Neige",
            Self::Storm => "Orage",
            Self::Fog => "Brouillard",
        }
    }
}

/// Météo courante d'une localité.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherNow {
    /// Nom de la localité (texte libre).
    pub city: String,
    /// Température en °C.
    pub temp_c: f32,
    /// Condition agrégée.
    pub condition: WeatherCondition,
    /// Humidité relative en pourcentage (0-100).
    pub humidity_pct: u8,
    /// Vitesse du vent en km/h.
    pub wind_kmh: u8,
}

/// Prévision pour un jour.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherForecastDay {
    /// Date au format `YYYY-MM-DD`.
    pub date_iso: String,
    /// Température maximale °C.
    pub max_c: f32,
    /// Température minimale °C.
    pub min_c: f32,
    /// Condition dominante de la journée.
    pub condition: WeatherCondition,
}

/// Erreurs renvoyées par le backend météo.
#[derive(Debug, thiserror::Error)]
pub enum JayMeteoError {
    /// Erreur côté backend (HTTP, parsing, capteur indisponible).
    #[error("backend météo : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
}

/// Trait abstrait pour un fournisseur de météo.
#[async_trait]
pub trait JayMeteoBackend: Send + Sync {
    /// Renvoie la météo courante pour `location` (ville ou alias `home`).
    async fn current_weather(&self, location: &str) -> Result<WeatherNow, JayMeteoError>;

    /// Renvoie les prévisions pour les `days` prochains jours (1-7).
    async fn forecast(
        &self,
        location: &str,
        days: u8,
    ) -> Result<Vec<WeatherForecastDay>, JayMeteoError>;
}

/// Le client Alicia ↔ JayMeteo.
pub struct JayMeteoClient {
    backend: Arc<dyn JayMeteoBackend>,
    default_location: String,
}

impl JayMeteoClient {
    /// Construit un client autour d'un backend, localité par défaut `Paris`.
    #[must_use]
    pub fn new(backend: Arc<dyn JayMeteoBackend>) -> Self {
        Self::with_default_location(backend, "Paris")
    }

    /// Construit un client en précisant la localité par défaut.
    #[must_use]
    pub fn with_default_location(
        backend: Arc<dyn JayMeteoBackend>,
        location: impl Into<String>,
    ) -> Self {
        Self {
            backend,
            default_location: location.into(),
        }
    }

    fn parse_string_opt<'a>(
        params: &'a Value,
        key: &str,
    ) -> Result<Option<&'a str>, JayMeteoError> {
        match params.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayMeteoError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_days(params: &Value) -> Result<u8, JayMeteoError> {
        match params.get("days") {
            None | Some(Value::Null) => Ok(3),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| (1..=7).contains(v))
                .map(|v| v as u8)
                .ok_or_else(|| {
                    JayMeteoError::InvalidParam(format!(
                        "days doit être un entier entre 1 et 7, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayMeteoError::InvalidParam(format!(
                "days doit être un entier, reçu {other}"
            ))),
        }
    }

    fn resolve_location(&self, params: &Value) -> Result<String, JayMeteoError> {
        match Self::parse_string_opt(params, "location")? {
            Some(s) if !s.trim().is_empty() => Ok(s.to_string()),
            _ => Ok(self.default_location.clone()),
        }
    }

    async fn handle_current(&self, params: &Value) -> ServiceCallResult {
        let location = match self.resolve_location(params) {
            Ok(l) => l,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.current_weather(&location).await {
            Ok(now) => {
                let temp = now.temp_c;
                let cond = now.condition.label_fr();
                let city = now.city.clone();
                ServiceCallResult {
                    success: true,
                    data: json!({ "weather": now }),
                    message: Some(format!("{city} : {temp:.1} °C, {cond}")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_forecast(&self, params: &Value) -> ServiceCallResult {
        let location = match self.resolve_location(params) {
            Ok(l) => l,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let days = match Self::parse_days(params) {
            Ok(d) => d,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.forecast(&location, days).await {
            Ok(forecast) => {
                let count = forecast.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "forecast": forecast,
                        "location": location,
                        "days": days,
                        "count": count,
                    }),
                    message: Some(format!(
                        "{count} jour(s) de prévision pour {location}"
                    )),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayMeteoClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaymeteo" {
                    return ServiceCallResult::fail(format!(
                        "JayMeteoClient appelé avec un service '{service}' inattendu"
                    ));
                }
                intent
            }
            Err(_) => {
                return ServiceCallResult::fail(format!(
                    "qualified_id invalide : '{qualified_id}'"
                ));
            }
        };
        match intent.as_str() {
            "query_current_weather" => self.handle_current(params).await,
            "query_forecast" => self.handle_forecast(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayMeteo non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests E2E du dispatcher.
#[derive(Default)]
pub struct FakeJayMeteoBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    now: Option<WeatherNow>,
    forecast: Vec<WeatherForecastDay>,
    fail_with: Option<String>,
}

impl FakeJayMeteoBackend {
    /// Backend vide ; toute lecture renverra une erreur tant que les données ne sont pas posées.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pose la météo courante renvoyée pour toute localité.
    pub fn set_current(&self, weather: WeatherNow) {
        self.inner.write().unwrap().now = Some(weather);
    }

    /// Pose les prévisions renvoyées pour toute localité.
    pub fn set_forecast(&self, days: Vec<WeatherForecastDay>) {
        self.inner.write().unwrap().forecast = days;
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap().fail_with = Some(message.into());
    }
}

#[async_trait]
impl JayMeteoBackend for FakeJayMeteoBackend {
    async fn current_weather(&self, _location: &str) -> Result<WeatherNow, JayMeteoError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayMeteoError::Backend(msg.clone()));
        }
        state
            .now
            .clone()
            .ok_or_else(|| JayMeteoError::Backend("aucune météo posée".into()))
    }

    async fn forecast(
        &self,
        _location: &str,
        days: u8,
    ) -> Result<Vec<WeatherForecastDay>, JayMeteoError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayMeteoError::Backend(msg.clone()));
        }
        Ok(state
            .forecast
            .iter()
            .take(days as usize)
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_now() -> WeatherNow {
        WeatherNow {
            city: "Paris".into(),
            temp_c: 12.5,
            condition: WeatherCondition::Cloudy,
            humidity_pct: 70,
            wind_kmh: 12,
        }
    }

    fn make_day(date: &str, max: f32, min: f32, c: WeatherCondition) -> WeatherForecastDay {
        WeatherForecastDay {
            date_iso: date.into(),
            max_c: max,
            min_c: min,
            condition: c,
        }
    }

    #[tokio::test]
    async fn current_weather_returns_summary() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        backend.set_current(make_now());
        let client = JayMeteoClient::new(backend);
        let r = client
            .call("jaymeteo.query_current_weather", &json!({}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["weather"]["city"], "Paris");
        assert!(r.message.unwrap().contains("Paris"));
    }

    #[tokio::test]
    async fn forecast_default_days_is_three() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        backend.set_forecast(vec![
            make_day("2026-05-11", 18.0, 9.0, WeatherCondition::Clear),
            make_day("2026-05-12", 19.0, 10.0, WeatherCondition::Cloudy),
            make_day("2026-05-13", 17.0, 8.0, WeatherCondition::Rain),
            make_day("2026-05-14", 16.0, 7.0, WeatherCondition::Rain),
        ]);
        let client = JayMeteoClient::new(backend);
        let r = client.call("jaymeteo.query_forecast", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 3);
    }

    #[tokio::test]
    async fn forecast_explicit_days() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        backend.set_forecast(vec![
            make_day("2026-05-11", 18.0, 9.0, WeatherCondition::Clear),
            make_day("2026-05-12", 19.0, 10.0, WeatherCondition::Cloudy),
        ]);
        let client = JayMeteoClient::new(backend);
        let r = client
            .call("jaymeteo.query_forecast", &json!({"days": 5}))
            .await;
        assert!(r.success);
        // Limité par les données disponibles (2)
        assert_eq!(r.data["count"].as_u64().unwrap(), 2);
    }

    #[tokio::test]
    async fn forecast_invalid_days_fails() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        let client = JayMeteoClient::new(backend);
        let r = client
            .call("jaymeteo.query_forecast", &json!({"days": 99}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        let client = JayMeteoClient::new(backend);
        let r = client.call("jaymeteo.fake", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        let client = JayMeteoClient::new(backend);
        let r = client.call("jaykonta.query_forecast", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayMeteoBackend::new());
        backend.force_failure("Open-Meteo indisponible");
        let client = JayMeteoClient::new(backend);
        let r = client
            .call("jaymeteo.query_current_weather", &json!({}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("Open-Meteo"));
    }

    #[test]
    fn wmo_codes_map_to_conditions() {
        assert_eq!(WeatherCondition::from_wmo_code(0), WeatherCondition::Clear);
        assert_eq!(WeatherCondition::from_wmo_code(2), WeatherCondition::Cloudy);
        assert_eq!(WeatherCondition::from_wmo_code(63), WeatherCondition::Rain);
        assert_eq!(WeatherCondition::from_wmo_code(73), WeatherCondition::Snow);
        assert_eq!(WeatherCondition::from_wmo_code(95), WeatherCondition::Storm);
        assert_eq!(WeatherCondition::from_wmo_code(45), WeatherCondition::Fog);
    }
}
