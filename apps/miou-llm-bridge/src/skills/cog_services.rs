//! @id     miou_llm_bridge_skills_cog_services
//! @do     appeler_services_cog_par_liste_blanche
//! @role   orchestration
//! @layer  app
//! @human  MAIA — skill « services COG » : appels en loopback restreints à une liste blanche (K-01).

//! Skill : Interaction avec les services COG.
//!
//! Permet aux agents d'interagir avec les services Miyukini installés
//! (JayKonta, JayKoa, MiyukiniWatch, etc.) via leurs API locales.
//!
//! # Sécurité (constat K-01, C-50)
//!
//! - L'`endpoint` est un **chemin relatif** validé par une grammaire stricte
//!   ([`valider_chemin`]) puis comparé **exactement** à la liste blanche du
//!   service ([`ServiceConnu::endpoints`]) : aucune commande libre.
//! - L'URL est construite par un constructeur d'URL sur `127.0.0.1` et le
//!   port fixe du service, puis re-vérifiée après analyse (hôte, port,
//!   absence d'identifiants) — [`construire_url`].
//! - La méthode HTTP est imposée par l'endpoint (GET pour les lectures).
//! - Les écritures exigent une [`Autorisation::EcritureConfirmee`], accordée
//!   uniquement après confirmation humaine (K-02).
//! - Ni proxy, ni redirection, réponse bornée en taille.

use super::{Autorisation, SkillInfo, SkillResult};
use crate::garde_etranger::{EffetOutil, Provenance};
use std::collections::HashMap;
use std::time::Duration;

/// Port de base des services COG (convention Miyukini).
const COG_BASE_PORT: u16 = 11400;

/// Hôte unique autorisé pour les appels de services.
const HOTE_LOOPBACK: &str = "127.0.0.1";
/// Longueur maximale d'un endpoint (octets).
const LONGUEUR_MAX_ENDPOINT: usize = 256;
/// Nombre maximal de paramètres de requête.
const MAX_PARAMS_REQUETE: usize = 8;
/// Longueur maximale d'une valeur de paramètre de requête.
const LONGUEUR_MAX_VALEUR: usize = 256;
/// Taille maximale du corps JSON envoyé (octets).
const TAILLE_MAX_CORPS: usize = 16 * 1024;
/// Taille maximale de la réponse lue (octets).
const TAILLE_MAX_REPONSE: usize = 256 * 1024;

/// Méthode HTTP autorisée pour un endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Methode {
    /// Lecture.
    Get,
    /// Écriture (intention à effet).
    Post,
}

impl Methode {
    fn nom(self) -> &'static str {
        match self {
            Methode::Get => "GET",
            Methode::Post => "POST",
        }
    }
}

/// Endpoint déclaré dans la liste blanche d'un service.
#[derive(Debug, Clone, Copy)]
pub struct EndpointAutorise {
    /// Chemin exact (forme canonique décodée).
    pub chemin: &'static str,
    /// Seule méthode acceptée.
    pub methode: Methode,
    /// Effet (lecture ou écriture).
    pub effet: EffetOutil,
}

/// Service COG connu, avec son port fixe et ses endpoints autorisés.
#[derive(Debug, Clone, Copy)]
pub struct ServiceConnu {
    /// Identifiant (manifeste).
    pub id: &'static str,
    /// Nom affiché.
    pub nom: &'static str,
    /// Port loopback fixe.
    pub port: u16,
    /// Provenance des réponses : `Etranger` pour les services qui relaient
    /// du contenu de tiers (inter-COG, surface web).
    pub provenance: Provenance,
    /// Liste blanche des endpoints.
    pub endpoints: &'static [EndpointAutorise],
}

const SANTE: EndpointAutorise = EndpointAutorise {
    chemin: "/health",
    methode: Methode::Get,
    effet: EffetOutil::Lecture,
};

/// Liste blanche par service.
///
/// Aucun service ne sert encore d'API HTTP : seuls `/health` et les
/// intentions déclarées dans `crates/<service>/service.manifest.json`
/// (`alicia_capabilities`) sont admis, sous `/intents/<intent>`.
/// Lectures en GET, écritures (`writes: true`) en POST avec confirmation.
const KNOWN_SERVICES: &[ServiceConnu] = &[
    ServiceConnu {
        id: "jaykonta",
        nom: "JayKonta — Comptabilité",
        port: 11441,
        provenance: Provenance::Locale,
        endpoints: &[
            SANTE,
            EndpointAutorise {
                chemin: "/intents/get_balance",
                methode: Methode::Get,
                effet: EffetOutil::Lecture,
            },
            EndpointAutorise {
                chemin: "/intents/list_recent_movements",
                methode: Methode::Get,
                effet: EffetOutil::Lecture,
            },
            EndpointAutorise {
                chemin: "/intents/add_movement",
                methode: Methode::Post,
                effet: EffetOutil::Ecriture,
            },
        ],
    },
    ServiceConnu {
        id: "jaykoa",
        nom: "JayKoa — Calendrier",
        port: 11442,
        provenance: Provenance::Locale,
        endpoints: &[
            SANTE,
            EndpointAutorise {
                chemin: "/intents/list_events",
                methode: Methode::Get,
                effet: EffetOutil::Lecture,
            },
            EndpointAutorise {
                chemin: "/intents/next_event",
                methode: Methode::Get,
                effet: EffetOutil::Lecture,
            },
        ],
    },
    ServiceConnu {
        id: "miyukiniwatch",
        nom: "MiyukiniWatch — Habitudes & Mesures",
        port: 11445,
        provenance: Provenance::Locale,
        endpoints: &[SANTE],
    },
    ServiceConnu {
        id: "jay1tribu",
        nom: "Jay1Tribu — Social & Chat",
        port: 11446,
        // Service inter-COG : relaie des messages de tiers.
        provenance: Provenance::Etranger,
        endpoints: &[SANTE],
    },
    ServiceConnu {
        id: "jaymanga",
        nom: "JayManga — Manga",
        port: 11447,
        // Surface web : contenu publié par des tiers.
        provenance: Provenance::Etranger,
        endpoints: &[SANTE],
    },
];

/// Refus d'un appel de service (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusEndpoint {
    #[error("service inconnu : {0}")]
    ServiceInconnu(String),
    #[error("endpoint vide ou trop long (max {LONGUEUR_MAX_ENDPOINT} octets)")]
    Longueur,
    #[error("l'endpoint doit être un chemin relatif commençant par un seul '/'")]
    PasUnCheminRelatif,
    #[error("caractère interdit dans l'endpoint : {0:?}")]
    CaractereInterdit(char),
    #[error("séquence '//' interdite dans l'endpoint")]
    DoubleBarre,
    #[error("encodage pourcent invalide ou interdit dans l'endpoint")]
    EncodageInterdit,
    #[error("segment '.' ou '..' interdit dans l'endpoint")]
    SegmentPoint,
    #[error("endpoint non déclaré pour le service {service} : {chemin}")]
    EndpointNonDeclare { service: String, chemin: String },
    #[error("méthode {demandee} interdite pour {chemin} (seule {attendue} est admise)")]
    MethodeNonAutorisee {
        demandee: String,
        attendue: &'static str,
        chemin: String,
    },
    #[error("paramètres de requête invalides")]
    RequeteInvalide,
    #[error("corps interdit pour une lecture, ou corps invalide / trop gros (max {TAILLE_MAX_CORPS} octets)")]
    CorpsInvalide,
    #[error("URL construite non conforme (hôte ou port inattendu)")]
    UrlNonConforme,
    #[error("écriture vers un service COG sans confirmation humaine")]
    ConfirmationRequise,
}

/// Appel validé, prêt à être émis.
#[derive(Debug, Clone)]
pub struct AppelPrepare {
    /// Service visé.
    pub service: &'static ServiceConnu,
    /// Endpoint de la liste blanche.
    pub endpoint: EndpointAutorise,
    /// URL finale (loopback, port fixe).
    pub url: reqwest::Url,
    /// Corps JSON (écritures uniquement).
    pub corps: Option<serde_json::Value>,
}

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert("action".into(), "list | call | status".into());
    params.insert("service".into(), "ID du service COG (ex: jaykonta)".into());
    params.insert(
        "endpoint".into(),
        "Chemin déclaré par le service (voir action list, ex: /intents/get_balance)".into(),
    );
    params.insert(
        "method".into(),
        "Méthode HTTP : doit correspondre à l'endpoint (GET lecture, POST écriture)".into(),
    );
    params.insert(
        "query".into(),
        "Paramètres de requête (objet clé-valeur, lectures)".into(),
    );
    params.insert(
        "body".into(),
        "Corps JSON (écritures uniquement, soumises à confirmation)".into(),
    );

    SkillInfo {
        id: "cog_services".into(),
        name: "Services COG".into(),
        description: "Interagir avec les services Miyukini COG installés (JayKonta, JayKoa, MiyukiniWatch, etc.) via leurs endpoints déclarés. Les écritures demandent une confirmation de l'utilisateur.".into(),
        parameters: params,
        min_security_level: "sandboxed",
    }
}

/// Exécute une interaction avec un service COG.
pub async fn execute(
    params: &HashMap<String, serde_json::Value>,
    autorisation: Autorisation,
) -> SkillResult {
    let action = params
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("list");

    match action {
        "list" => list_services(),
        "status" => {
            let service = params.get("service").and_then(|v| v.as_str()).unwrap_or("");
            check_service_status(service).await
        }
        "call" => match preparer_appel(params) {
            Ok(appel) => {
                if appel.endpoint.effet != EffetOutil::Lecture
                    && autorisation != Autorisation::EcritureConfirmee
                {
                    return refus(&RefusEndpoint::ConfirmationRequise);
                }
                emettre(&appel).await
            }
            Err(e) => refus(&e),
        },
        _ => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!(
                "Action inconnue : {action}. Actions : list, status, call"
            )),
            duration_ms: 0,
        },
    }
}

/// Effet d'un appel (pour la garde). Appel invalide : `Ecriture` (fail-closed).
pub fn effet_appel(params: &HashMap<String, serde_json::Value>) -> EffetOutil {
    match params.get("action").and_then(|v| v.as_str()).unwrap_or("list") {
        "list" | "status" => EffetOutil::Lecture,
        "call" => preparer_appel(params).map_or(EffetOutil::Ecriture, |a| a.endpoint.effet),
        _ => EffetOutil::Ecriture,
    }
}

/// Provenance du résultat d'un appel. Service inconnu : `Etranger` (prudence).
pub fn provenance_appel(params: &HashMap<String, serde_json::Value>) -> Provenance {
    match params.get("action").and_then(|v| v.as_str()).unwrap_or("list") {
        "list" => Provenance::Locale,
        _ => params
            .get("service")
            .and_then(|v| v.as_str())
            .and_then(trouver_service)
            .map_or(Provenance::Etranger, |s| s.provenance),
    }
}

fn refus(e: &RefusEndpoint) -> SkillResult {
    SkillResult {
        success: false,
        output: serde_json::Value::Null,
        error: Some(format!("Appel refusé : {e}")),
        duration_ms: 0,
    }
}

fn list_services() -> SkillResult {
    let services: Vec<serde_json::Value> = KNOWN_SERVICES
        .iter()
        .map(|s| {
            let endpoints: Vec<serde_json::Value> = s
                .endpoints
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "endpoint": e.chemin,
                        "method": e.methode.nom(),
                        "writes": e.effet != EffetOutil::Lecture,
                    })
                })
                .collect();
            serde_json::json!({
                "id": s.id,
                "name": s.nom,
                "port": s.port,
                "url": format!("http://{HOTE_LOOPBACK}:{}", s.port),
                "endpoints": endpoints,
            })
        })
        .collect();

    SkillResult {
        success: true,
        output: serde_json::json!({
            "services": services,
            "count": services.len(),
        }),
        error: None,
        duration_ms: 0,
    }
}

/// Client HTTP des appels de services : sans proxy, sans redirection.
fn client(delai: Duration) -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(delai)
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| format!("client HTTP indisponible : {e}"))
}

async fn check_service_status(service_id: &str) -> SkillResult {
    let Some(service) = trouver_service(service_id) else {
        return refus(&RefusEndpoint::ServiceInconnu(service_id.into()));
    };
    let url = match construire_url(service.port, SANTE.chemin, &[]) {
        Ok(u) => u,
        Err(e) => return refus(&e),
    };
    let client = match client(Duration::from_secs(3)) {
        Ok(c) => c,
        Err(e) => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(e),
                duration_ms: 0,
            }
        }
    };

    let running = matches!(client.get(url).send().await, Ok(resp) if resp.status().is_success());
    SkillResult {
        success: true,
        output: serde_json::json!({
            "service": service_id,
            "status": if running { "running" } else { "unreachable" },
            "port": service.port,
        }),
        error: None,
        duration_ms: 0,
    }
}

/// Valide entièrement un appel `call` sans l'émettre.
pub fn preparer_appel(
    params: &HashMap<String, serde_json::Value>,
) -> Result<AppelPrepare, RefusEndpoint> {
    let service_id = params.get("service").and_then(|v| v.as_str()).unwrap_or("");
    let service =
        trouver_service(service_id).ok_or_else(|| RefusEndpoint::ServiceInconnu(service_id.into()))?;

    let brut = params
        .get("endpoint")
        .and_then(|v| v.as_str())
        .unwrap_or("/health");
    let chemin = valider_chemin(brut)?;

    let endpoint = *service
        .endpoints
        .iter()
        .find(|e| e.chemin == chemin)
        .ok_or_else(|| RefusEndpoint::EndpointNonDeclare {
            service: service.id.into(),
            chemin: chemin.clone(),
        })?;

    if let Some(m) = params.get("method") {
        let demandee = m.as_str().unwrap_or("").to_ascii_uppercase();
        if demandee != endpoint.methode.nom() {
            return Err(RefusEndpoint::MethodeNonAutorisee {
                demandee,
                attendue: endpoint.methode.nom(),
                chemin,
            });
        }
    }

    let requete = valider_requete(params.get("query"))?;

    let corps_objet =
        objet_json(params.get("body")).map_err(|()| RefusEndpoint::CorpsInvalide)?;
    let corps = match (endpoint.methode, corps_objet) {
        (Methode::Get, None) => None,
        (Methode::Get, Some(_)) => return Err(RefusEndpoint::CorpsInvalide),
        (Methode::Post, objet) => {
            let v = serde_json::Value::Object(objet.unwrap_or_default());
            let taille = serde_json::to_vec(&v)
                .map_err(|_| RefusEndpoint::CorpsInvalide)?
                .len();
            if taille > TAILLE_MAX_CORPS {
                return Err(RefusEndpoint::CorpsInvalide);
            }
            Some(v)
        }
    };

    let url = construire_url(service.port, endpoint.chemin, &requete)?;
    Ok(AppelPrepare {
        service,
        endpoint,
        url,
        corps,
    })
}

/// Grammaire stricte d'un endpoint. Retourne la forme canonique décodée.
///
/// Refuse : chaîne vide ou trop longue, absence de `/` initial, `//`,
/// tout caractère hors `[A-Za-z0-9/_.~-]` et `%XX` (donc `@`, `\`, `:`,
/// `?`, `#`, espaces, contrôles, schéma), tout `%XX` qui ne décode pas
/// vers un caractère non réservé, et les segments `.` / `..`.
pub fn valider_chemin(endpoint: &str) -> Result<String, RefusEndpoint> {
    if endpoint.is_empty() || endpoint.len() > LONGUEUR_MAX_ENDPOINT {
        return Err(RefusEndpoint::Longueur);
    }
    if !endpoint.starts_with('/') {
        return Err(RefusEndpoint::PasUnCheminRelatif);
    }
    if endpoint.contains("//") {
        return Err(RefusEndpoint::DoubleBarre);
    }

    let octets = endpoint.as_bytes();
    let mut decode = String::with_capacity(endpoint.len());
    let mut i = 0;
    while i < octets.len() {
        let c = octets[i];
        if c == b'%' {
            let hex = endpoint
                .get(i + 1..i + 3)
                .ok_or(RefusEndpoint::EncodageInterdit)?;
            let valeur =
                u8::from_str_radix(hex, 16).map_err(|_| RefusEndpoint::EncodageInterdit)?;
            // Seuls les caractères non réservés peuvent être encodés : un
            // `/`, `.`, `\`, `@` ou `%` encodé est refusé.
            if !(valeur.is_ascii_alphanumeric() || valeur == b'-' || valeur == b'_' || valeur == b'~')
            {
                return Err(RefusEndpoint::EncodageInterdit);
            }
            decode.push(char::from(valeur));
            i += 3;
            continue;
        }
        if c.is_ascii_alphanumeric() || matches!(c, b'/' | b'_' | b'.' | b'~' | b'-') {
            decode.push(char::from(c));
            i += 1;
            continue;
        }
        let interdit = endpoint[i..].chars().next().unwrap_or('\u{FFFD}');
        return Err(RefusEndpoint::CaractereInterdit(interdit));
    }

    if decode.split('/').any(|seg| seg == "." || seg == "..") {
        return Err(RefusEndpoint::SegmentPoint);
    }
    Ok(decode)
}

/// Valide les paramètres de requête : clés `[a-z0-9_]{1,32}`, valeurs
/// scalaires bornées, au plus [`MAX_PARAMS_REQUETE`] paires.
fn valider_requete(
    query: Option<&serde_json::Value>,
) -> Result<Vec<(String, String)>, RefusEndpoint> {
    let Some(objet) = objet_json(query).map_err(|()| RefusEndpoint::RequeteInvalide)? else {
        return Ok(Vec::new());
    };
    if objet.len() > MAX_PARAMS_REQUETE {
        return Err(RefusEndpoint::RequeteInvalide);
    }
    let mut paires = Vec::with_capacity(objet.len());
    for (cle, valeur) in &objet {
        let cle_ok = !cle.is_empty()
            && cle.len() <= 32
            && cle
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_');
        if !cle_ok {
            return Err(RefusEndpoint::RequeteInvalide);
        }
        let texte = match valeur {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            _ => return Err(RefusEndpoint::RequeteInvalide),
        };
        if texte.len() > LONGUEUR_MAX_VALEUR || texte.chars().any(char::is_control) {
            return Err(RefusEndpoint::RequeteInvalide);
        }
        paires.push((cle.clone(), texte));
    }
    Ok(paires)
}

/// Construit l'URL d'appel sur `127.0.0.1:port` puis vérifie, après
/// analyse, que l'hôte, le port et le chemin sont exactement ceux attendus.
pub fn construire_url(
    port: u16,
    chemin: &str,
    requete: &[(String, String)],
) -> Result<reqwest::Url, RefusEndpoint> {
    let mut url = reqwest::Url::parse(&format!("http://{HOTE_LOOPBACK}/"))
        .map_err(|_| RefusEndpoint::UrlNonConforme)?;
    url.set_port(Some(port))
        .map_err(|()| RefusEndpoint::UrlNonConforme)?;
    url.set_path(chemin);
    if !requete.is_empty() {
        url.query_pairs_mut().extend_pairs(requete.iter());
    }

    // Re-vérification après analyse (défense en profondeur).
    let reanalysee =
        reqwest::Url::parse(url.as_str()).map_err(|_| RefusEndpoint::UrlNonConforme)?;
    let conforme = reanalysee.scheme() == "http"
        && reanalysee.host_str() == Some(HOTE_LOOPBACK)
        && reanalysee.port() == Some(port)
        && reanalysee.username().is_empty()
        && reanalysee.password().is_none()
        && reanalysee.path() == chemin
        && reanalysee.fragment().is_none();
    if !conforme {
        return Err(RefusEndpoint::UrlNonConforme);
    }
    Ok(reanalysee)
}

async fn emettre(appel: &AppelPrepare) -> SkillResult {
    let service_id = appel.service.id;
    let endpoint = appel.endpoint.chemin;
    let client = match client(Duration::from_secs(15)) {
        Ok(c) => c,
        Err(e) => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(e),
                duration_ms: 0,
            }
        }
    };

    let mut req = match appel.endpoint.methode {
        Methode::Get => client.get(appel.url.clone()),
        Methode::Post => client.post(appel.url.clone()),
    };
    if let Some(corps) = &appel.corps {
        req = req.json(corps);
    }

    match req.send().await {
        Ok(mut resp) => {
            let status = resp.status().as_u16();
            let mut octets: Vec<u8> = Vec::new();
            let mut tronque = false;
            loop {
                match resp.chunk().await {
                    Ok(Some(morceau)) => {
                        let reste = TAILLE_MAX_REPONSE.saturating_sub(octets.len());
                        if morceau.len() > reste {
                            octets.extend_from_slice(&morceau[..reste]);
                            tronque = true;
                            break;
                        }
                        octets.extend_from_slice(&morceau);
                    }
                    Ok(None) => break,
                    Err(e) => {
                        return SkillResult {
                            success: false,
                            output: serde_json::json!({ "service": service_id, "endpoint": endpoint }),
                            error: Some(format!("Erreur lecture réponse : {e}")),
                            duration_ms: 0,
                        }
                    }
                }
            }
            let body_text = String::from_utf8_lossy(&octets).into_owned();
            let body_value = serde_json::from_str::<serde_json::Value>(&body_text)
                .unwrap_or(serde_json::Value::String(body_text));

            SkillResult {
                success: (200..300).contains(&status),
                output: serde_json::json!({
                    "service": service_id,
                    "endpoint": endpoint,
                    "status": status,
                    "body": body_value,
                    "truncated": tronque,
                }),
                error: if status >= 400 {
                    Some(format!("HTTP {status}"))
                } else {
                    None
                },
                duration_ms: 0,
            }
        }
        Err(e) => SkillResult {
            success: false,
            output: serde_json::json!({
                "service": service_id,
                "endpoint": endpoint,
            }),
            error: Some(format!("Service {service_id} injoignable : {e}")),
            duration_ms: 0,
        },
    }
}

/// Normalise un paramètre objet : objet JSON, ou chaîne contenant un objet
/// JSON (le schéma d'outil transmis au modèle ne type que des chaînes).
/// `None` / `null` / chaîne vide : absent. Autre forme : `Err(())`.
pub(crate) fn objet_json(
    valeur: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Map<String, serde_json::Value>>, ()> {
    match valeur {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Object(o)) => Ok(Some(o.clone())),
        Some(serde_json::Value::String(s)) if s.trim().is_empty() => Ok(None),
        Some(serde_json::Value::String(s)) if s.len() <= TAILLE_MAX_CORPS => {
            match serde_json::from_str::<serde_json::Value>(s) {
                Ok(serde_json::Value::Object(o)) => Ok(Some(o)),
                _ => Err(()),
            }
        }
        Some(_) => Err(()),
    }
}

fn trouver_service(service_id: &str) -> Option<&'static ServiceConnu> {
    KNOWN_SERVICES.iter().find(|s| s.id == service_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn appel(service: &str, endpoint: &str) -> HashMap<String, serde_json::Value> {
        let mut p = HashMap::new();
        p.insert("action".into(), "call".into());
        p.insert("service".into(), service.into());
        p.insert("endpoint".into(), endpoint.into());
        p
    }

    /// Table SSRF (K-01) : tous ces endpoints doivent être refusés.
    type Attendu = fn(&RefusEndpoint) -> bool;

    #[test]
    fn table_ssrf_refusee() {
        let cas: &[(&str, Attendu)] = &[
            ("@evil.com", |e| matches!(e, RefusEndpoint::PasUnCheminRelatif)),
            ("@127.0.0.1:11435/v1/agents", |e| {
                matches!(e, RefusEndpoint::PasUnCheminRelatif)
            }),
            ("/@evil.com", |e| matches!(e, RefusEndpoint::CaractereInterdit('@'))),
            ("//evil", |e| matches!(e, RefusEndpoint::DoubleBarre)),
            ("//evil.com/x", |e| matches!(e, RefusEndpoint::DoubleBarre)),
            ("/a//b", |e| matches!(e, RefusEndpoint::DoubleBarre)),
            ("/..%2f", |e| matches!(e, RefusEndpoint::EncodageInterdit)),
            ("/..%2F..%2Fetc", |e| matches!(e, RefusEndpoint::EncodageInterdit)),
            ("/%2e%2e/admin", |e| matches!(e, RefusEndpoint::EncodageInterdit)),
            ("/%252e", |e| matches!(e, RefusEndpoint::EncodageInterdit)),
            ("/%4", |e| matches!(e, RefusEndpoint::EncodageInterdit)),
            ("/%zz", |e| matches!(e, RefusEndpoint::EncodageInterdit)),
            ("/../v1/agents", |e| matches!(e, RefusEndpoint::SegmentPoint)),
            ("/intents/./get_balance", |e| matches!(e, RefusEndpoint::SegmentPoint)),
            ("http://evil.com/", |e| matches!(e, RefusEndpoint::PasUnCheminRelatif)),
            ("HTTP://127.0.0.1:11435/", |e| {
                matches!(e, RefusEndpoint::PasUnCheminRelatif)
            }),
            ("/x:y", |e| matches!(e, RefusEndpoint::CaractereInterdit(':'))),
            ("/\\evil.com", |e| matches!(e, RefusEndpoint::CaractereInterdit('\\'))),
            ("/health?x=@evil", |e| matches!(e, RefusEndpoint::CaractereInterdit('?'))),
            ("/health#frag", |e| matches!(e, RefusEndpoint::CaractereInterdit('#'))),
            ("/he alth", |e| matches!(e, RefusEndpoint::CaractereInterdit(' '))),
            ("/health\r\nHost: evil", |e| {
                matches!(e, RefusEndpoint::CaractereInterdit('\r'))
            }),
            ("/é", |e| matches!(e, RefusEndpoint::CaractereInterdit('é'))),
            ("", |e| matches!(e, RefusEndpoint::Longueur)),
        ];
        for (endpoint, attendu) in cas {
            let err = preparer_appel(&appel("jaykonta", endpoint))
                .expect_err(&format!("{endpoint:?} aurait dû être refusé"));
            assert!(attendu(&err), "{endpoint:?} → refus inattendu : {err:?}");
        }
        let long = format!("/{}", "a".repeat(LONGUEUR_MAX_ENDPOINT));
        assert_eq!(
            preparer_appel(&appel("jaykonta", &long)).unwrap_err(),
            RefusEndpoint::Longueur
        );
    }

    #[test]
    fn endpoint_inconnu_refuse() {
        for endpoint in ["/api/balance", "/v1/agents", "/intents/delete_all", "/health/"] {
            assert!(
                matches!(
                    preparer_appel(&appel("jaykonta", endpoint)),
                    Err(RefusEndpoint::EndpointNonDeclare { .. })
                ),
                "{endpoint}"
            );
        }
        // Endpoint déclaré pour un autre service seulement.
        assert!(matches!(
            preparer_appel(&appel("jaykoa", "/intents/add_movement")),
            Err(RefusEndpoint::EndpointNonDeclare { .. })
        ));
    }

    #[test]
    fn service_inconnu_refuse() {
        assert!(matches!(
            preparer_appel(&appel("maia", "/health")),
            Err(RefusEndpoint::ServiceInconnu(_))
        ));
    }

    #[test]
    fn endpoint_autorise_construit_une_url_loopback() {
        let a = preparer_appel(&appel("jaykonta", "/intents/get_balance")).unwrap();
        assert_eq!(a.url.as_str(), "http://127.0.0.1:11441/intents/get_balance");
        assert_eq!(a.url.host_str(), Some("127.0.0.1"));
        assert_eq!(a.url.port(), Some(11441));
        assert_eq!(a.endpoint.methode, Methode::Get);
        assert!(a.corps.is_none());

        // Encodage pourcent d'un caractère non réservé : forme canonique.
        let b = preparer_appel(&appel("jaykonta", "/intents/get%5Fbalance")).unwrap();
        assert_eq!(b.url.path(), "/intents/get_balance");
    }

    #[test]
    fn requete_encodee_sans_effet_sur_hote() {
        let mut p = appel("jaykoa", "/intents/list_events");
        p.insert(
            "query".into(),
            serde_json::json!({ "from": "2026-10-24@evil.com/#x", "to": 3 }),
        );
        let a = preparer_appel(&p).unwrap();
        assert_eq!(a.url.host_str(), Some("127.0.0.1"));
        assert_eq!(a.url.port(), Some(11442));
        assert_eq!(a.url.path(), "/intents/list_events");
        assert!(a.url.fragment().is_none());

        let mut mauvais = appel("jaykoa", "/intents/list_events");
        mauvais.insert("query".into(), serde_json::json!({ "Host": "x" }));
        assert_eq!(
            preparer_appel(&mauvais).unwrap_err(),
            RefusEndpoint::RequeteInvalide
        );
    }

    #[test]
    fn objets_encodes_en_chaine_acceptes() {
        let mut p = appel("jaykonta", "/intents/add_movement");
        p.insert("body".into(), r#"{"amount": 3, "kind": "income"}"#.into());
        let a = preparer_appel(&p).unwrap();
        assert_eq!(a.corps, Some(serde_json::json!({ "amount": 3, "kind": "income" })));

        let mut q = appel("jaykoa", "/intents/list_events");
        q.insert("query".into(), r#"{"from": "2026-10-24"}"#.into());
        let b = preparer_appel(&q).unwrap();
        assert_eq!(b.url.query(), Some("from=2026-10-24"));

        let mut mauvais = appel("jaykonta", "/intents/add_movement");
        mauvais.insert("body".into(), "[1,2]".into());
        assert_eq!(preparer_appel(&mauvais).unwrap_err(), RefusEndpoint::CorpsInvalide);
    }

    #[test]
    fn methode_imposee_par_endpoint() {
        for m in ["POST", "DELETE", "PUT", "get "] {
            let mut p = appel("jaykonta", "/intents/get_balance");
            p.insert("method".into(), m.into());
            assert!(
                matches!(
                    preparer_appel(&p),
                    Err(RefusEndpoint::MethodeNonAutorisee { .. })
                ),
                "{m}"
            );
        }
        let mut p = appel("jaykonta", "/intents/get_balance");
        p.insert("method".into(), "get".into());
        assert!(preparer_appel(&p).is_ok());

        let mut w = appel("jaykonta", "/intents/add_movement");
        w.insert("method".into(), "GET".into());
        assert!(matches!(
            preparer_appel(&w),
            Err(RefusEndpoint::MethodeNonAutorisee { .. })
        ));
    }

    #[test]
    fn corps_interdit_en_lecture_et_borne_en_ecriture() {
        let mut p = appel("jaykonta", "/intents/get_balance");
        p.insert("body".into(), serde_json::json!({ "x": 1 }));
        assert_eq!(preparer_appel(&p).unwrap_err(), RefusEndpoint::CorpsInvalide);

        let mut w = appel("jaykonta", "/intents/add_movement");
        w.insert(
            "body".into(),
            serde_json::json!({ "x": "a".repeat(TAILLE_MAX_CORPS) }),
        );
        assert_eq!(preparer_appel(&w).unwrap_err(), RefusEndpoint::CorpsInvalide);

        let mut ok = appel("jaykonta", "/intents/add_movement");
        ok.insert("body".into(), serde_json::json!({ "amount": 12.0, "kind": "expense" }));
        let a = preparer_appel(&ok).unwrap();
        assert_eq!(a.endpoint.methode, Methode::Post);
        assert_eq!(a.endpoint.effet, EffetOutil::Ecriture);
    }

    #[test]
    fn construire_url_verifie_hote_et_port() {
        let u = construire_url(11441, "/health", &[]).unwrap();
        assert_eq!(u.as_str(), "http://127.0.0.1:11441/health");
        // Un chemin qui ne survit pas tel quel à l'analyse est refusé.
        assert_eq!(
            construire_url(11441, "/a/../b", &[]).unwrap_err(),
            RefusEndpoint::UrlNonConforme
        );
    }

    #[test]
    fn effets_et_provenances() {
        assert_eq!(
            effet_appel(&appel("jaykonta", "/intents/get_balance")),
            EffetOutil::Lecture
        );
        assert_eq!(
            effet_appel(&appel("jaykonta", "/intents/add_movement")),
            EffetOutil::Ecriture
        );
        // Appel invalide : traité comme une écriture (fail-closed).
        assert_eq!(effet_appel(&appel("jaykonta", "@evil.com")), EffetOutil::Ecriture);
        assert_eq!(
            provenance_appel(&appel("jaykonta", "/health")),
            Provenance::Locale
        );
        assert_eq!(
            provenance_appel(&appel("jay1tribu", "/health")),
            Provenance::Etranger
        );
        assert_eq!(provenance_appel(&appel("x", "/health")), Provenance::Etranger);
    }

    #[tokio::test]
    async fn ecriture_sans_confirmation_refusee_avant_tout_reseau() {
        let p = appel("jaykonta", "/intents/add_movement");
        let r = execute(&p, Autorisation::Standard).await;
        assert!(!r.success);
        assert!(r
            .error
            .unwrap_or_default()
            .contains("sans confirmation humaine"));
    }

    #[tokio::test]
    async fn refus_ssrf_par_execute() {
        let r = execute(&appel("jaykonta", "@evil.com/x"), Autorisation::EcritureConfirmee).await;
        assert!(!r.success);
        assert!(r.error.unwrap_or_default().starts_with("Appel refusé"));
    }

    #[test]
    fn liste_expose_endpoints_et_loopback() {
        let r = list_services();
        let s = r.output.to_string();
        assert!(s.contains("http://127.0.0.1:11441"));
        assert!(s.contains("/intents/add_movement"));
        assert!(!s.contains("localhost"));
    }
}
