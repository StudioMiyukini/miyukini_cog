//! @id     miou_llm_bridge_skills_web_fetch
//! @do     recuperer_contenu_web_public
//! @role   orchestration
//! @layer  app
//! @human  MAIA — skill « web fetch » : récupération web publique avec anti-SSRF par résolution (K-03).

//! Skill : Récupération web.
//!
//! Permet aux agents de récupérer du contenu web public (pages, API publiques).
//!
//! # Sécurité (constat K-03)
//!
//! - URL **analysée** (WHATWG) : schéma `http`/`https`, pas d'identifiants,
//!   ports 80/443/8080/8443, longueur bornée.
//! - Hôte **résolu** puis chaque adresse vérifiée ([`ip_publique`]) : refus
//!   de loopback, privé, lien-local, ULA, CGNAT, multicast, non spécifiée,
//!   IPv4 mappée/compatible, NAT64, 6to4, Teredo, plages réservées.
//! - Connexion **épinglée** sur l'adresse vérifiée (`resolve`), sans proxy :
//!   pas de seconde résolution (anti DNS rebinding).
//! - Redirections **non suivies** automatiquement : chaque saut est
//!   re-validé, au plus [`MAX_REDIRECTIONS`].
//! - Méthodes GET/HEAD seulement, en-têtes sur liste blanche, aucun corps.
//! - Délai global borné, taille de réponse bornée, types de contenu textuels.
//! - Le résultat est marqué `etranger` par la garde de la boucle d'outils.

use super::{SkillInfo, SkillResult};
use std::collections::HashMap;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::pin::Pin;
use std::time::{Duration, Instant};

/// Timeout par défaut (secondes).
const DEFAULT_TIMEOUT_SECS: u64 = 15;
/// Timeout maximal accepté (secondes).
const MAX_TIMEOUT_SECS: u64 = 30;
/// Timeout de connexion par saut (secondes).
const CONNECT_TIMEOUT_SECS: u64 = 5;
/// Taille max de réponse lue (512 KB).
const MAX_RESPONSE_SIZE: usize = 524_288;
/// Taille max du texte renvoyé au modèle (caractères).
const MAX_TEXTE_MODELE: usize = 4096;
/// Nombre maximal de redirections re-validées.
pub const MAX_REDIRECTIONS: usize = 3;
/// Longueur maximale d'une URL.
const LONGUEUR_MAX_URL: usize = 2048;
/// Ports autorisés.
const PORTS_AUTORISES: &[u16] = &[80, 443, 8080, 8443];
/// En-têtes que le modèle peut fixer.
const ENTETES_AUTORISES: &[&str] = &["accept", "accept-language"];
/// Types de contenu `application/*` acceptés (en plus de `text/*`).
const TYPES_APPLICATION: &[&str] = &[
    "application/json",
    "application/ld+json",
    "application/xml",
    "application/xhtml+xml",
    "application/rss+xml",
    "application/atom+xml",
];

/// Refus de la récupération web (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusWeb {
    #[error("URL invalide")]
    UrlInvalide,
    #[error("URL trop longue (max {LONGUEUR_MAX_URL})")]
    UrlTropLongue,
    #[error("seuls les schémas http et https sont autorisés")]
    SchemaInterdit,
    #[error("identifiants dans l'URL interdits")]
    IdentifiantsInterdits,
    #[error("port {0} non autorisé")]
    PortInterdit(u16),
    #[error("hôte absent")]
    HoteAbsent,
    #[error("hôte non public refusé : {0}")]
    HoteNonPublic(String),
    #[error("adresse non publique refusée : {0}")]
    AdresseNonPublique(IpAddr),
    #[error("résolution DNS impossible")]
    ResolutionImpossible,
    #[error("méthode {0} interdite (GET ou HEAD uniquement)")]
    MethodeInterdite(String),
    #[error("en-tête {0} non autorisé")]
    EnteteInterdit(String),
    #[error("corps de requête interdit")]
    CorpsInterdit,
    #[error("redirection sans en-tête Location valide")]
    RedirectionInvalide,
    #[error("trop de redirections (max {MAX_REDIRECTIONS})")]
    TropDeRedirections,
    #[error("type de contenu non textuel refusé : {0}")]
    TypeContenuInterdit(String),
    #[error("délai dépassé")]
    DelaiDepasse,
    #[error("erreur réseau : {0}")]
    Reseau(String),
}

/// Résolveur DNS injectable (tests : résolveur factice).
pub trait Resolveur: Send + Sync {
    /// Résout un nom d'hôte en adresses IP.
    fn resoudre<'a>(
        &'a self,
        hote: &'a str,
        port: u16,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<Vec<IpAddr>>> + Send + 'a>>;
}

/// Résolveur système (tokio).
pub struct ResolveurSysteme;

impl Resolveur for ResolveurSysteme {
    fn resoudre<'a>(
        &'a self,
        hote: &'a str,
        port: u16,
    ) -> Pin<Box<dyn Future<Output = std::io::Result<Vec<IpAddr>>> + Send + 'a>> {
        Box::pin(async move {
            let adresses = tokio::net::lookup_host((hote, port)).await?;
            Ok(adresses.map(|a| a.ip()).collect())
        })
    }
}

/// Cible validée : URL, hôte et adresse épinglée.
#[derive(Debug, Clone)]
pub struct CibleValidee {
    /// URL analysée.
    pub url: reqwest::Url,
    /// Nom d'hôte (sans crochets).
    pub hote: String,
    /// Adresse vérifiée sur laquelle se connecter.
    pub adresse: SocketAddr,
    /// Vrai si l'hôte est une IP littérale.
    pub hote_est_ip: bool,
}

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert(
        "url".into(),
        "URL publique http(s) à récupérer (obligatoire)".into(),
    );
    params.insert("method".into(), "GET | HEAD".into());
    params.insert(
        "headers".into(),
        "En-têtes autorisés : accept, accept-language (objet clé-valeur)".into(),
    );
    params.insert(
        "timeout".into(),
        format!("Timeout en secondes (défaut {DEFAULT_TIMEOUT_SECS}, max {MAX_TIMEOUT_SECS})"),
    );

    SkillInfo {
        id: "web_fetch".into(),
        name: "Recherche Web".into(),
        description:
            "Récupère du contenu web public (pages, API publiques), en lecture seule. Le contenu obtenu est marqué étranger (non fiable)."
                .into(),
        parameters: params,
        min_security_level: "sandboxed",
    }
}

/// Exécute une requête web.
pub async fn execute(params: &HashMap<String, serde_json::Value>) -> SkillResult {
    executer_avec(params, &ResolveurSysteme).await
}

fn refus(e: &RefusWeb) -> SkillResult {
    SkillResult {
        success: false,
        output: serde_json::Value::Null,
        error: Some(format!("Requête web refusée : {e}")),
        duration_ms: 0,
    }
}

/// Exécution avec résolveur injecté.
pub async fn executer_avec(
    params: &HashMap<String, serde_json::Value>,
    resolveur: &dyn Resolveur,
) -> SkillResult {
    let Some(url) = params.get("url").and_then(|v| v.as_str()) else {
        return SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some("Paramètre 'url' requis".into()),
            duration_ms: 0,
        };
    };

    let methode = params
        .get("method")
        .and_then(|v| v.as_str())
        .unwrap_or("GET")
        .to_ascii_uppercase();
    let methode = match methode.as_str() {
        "GET" => reqwest::Method::GET,
        "HEAD" => reqwest::Method::HEAD,
        _ => return refus(&RefusWeb::MethodeInterdite(methode)),
    };
    if params.get("body").is_some_and(|b| !b.is_null()) {
        return refus(&RefusWeb::CorpsInterdit);
    }
    let entetes = match valider_entetes(params.get("headers")) {
        Ok(h) => h,
        Err(e) => return refus(&e),
    };
    let delai = Duration::from_secs(
        params
            .get("timeout")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(DEFAULT_TIMEOUT_SECS)
            .clamp(1, MAX_TIMEOUT_SECS),
    );

    match tokio::time::timeout(delai, recuperer(url, methode, &entetes, delai, resolveur)).await {
        Ok(Ok(sortie)) => sortie,
        Ok(Err(e)) => refus(&e),
        Err(_) => refus(&RefusWeb::DelaiDepasse),
    }
}

/// Valide les en-têtes fournis (liste blanche, valeurs bornées).
fn valider_entetes(
    headers: Option<&serde_json::Value>,
) -> Result<reqwest::header::HeaderMap, RefusWeb> {
    let mut carte = reqwest::header::HeaderMap::new();
    let Some(objet) = super::cog_services::objet_json(headers)
        .map_err(|()| RefusWeb::EnteteInterdit("(format)".into()))?
    else {
        return Ok(carte);
    };
    for (cle, valeur) in &objet {
        let nom = cle.to_ascii_lowercase();
        if !ENTETES_AUTORISES.contains(&nom.as_str()) {
            return Err(RefusWeb::EnteteInterdit(cle.clone()));
        }
        let texte = valeur
            .as_str()
            .filter(|v| v.len() <= 256)
            .ok_or_else(|| RefusWeb::EnteteInterdit(cle.clone()))?;
        let nom_h = reqwest::header::HeaderName::from_bytes(nom.as_bytes())
            .map_err(|_| RefusWeb::EnteteInterdit(cle.clone()))?;
        let val_h = reqwest::header::HeaderValue::from_str(texte)
            .map_err(|_| RefusWeb::EnteteInterdit(cle.clone()))?;
        carte.insert(nom_h, val_h);
    }
    Ok(carte)
}

/// Vrai si l'adresse IPv4 est publique (routable sur Internet).
fn ipv4_publique(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    !(o[0] == 0                                   // 0.0.0.0/8 (non spécifiée, « ce réseau »)
        || o[0] == 10                             // privé
        || o[0] == 127                            // loopback
        || (o[0] == 100 && (o[1] & 0xC0) == 64)   // CGNAT 100.64.0.0/10
        || (o[0] == 169 && o[1] == 254)           // lien-local (métadonnées cloud)
        || (o[0] == 172 && (o[1] & 0xF0) == 16)   // privé 172.16.0.0/12
        || (o[0] == 192 && o[1] == 0 && o[2] == 0) // 192.0.0.0/24 (IETF)
        || (o[0] == 192 && o[1] == 0 && o[2] == 2) // documentation
        || (o[0] == 192 && o[1] == 88 && o[2] == 99) // relais 6to4
        || (o[0] == 192 && o[1] == 168)           // privé
        || (o[0] == 198 && (o[1] & 0xFE) == 18)   // bancs d'essai 198.18.0.0/15
        || (o[0] == 198 && o[1] == 51 && o[2] == 100) // documentation
        || (o[0] == 203 && o[1] == 0 && o[2] == 113) // documentation
        || o[0] >= 224) // multicast 224/4, réservé 240/4, diffusion
}

/// Vrai si l'adresse est publique. Toute IPv6 qui embarque une IPv4 est refusée.
pub fn ip_publique(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => ipv4_publique(v4),
        IpAddr::V6(v6) => ipv6_publique(v6),
    }
}

fn ipv6_publique(v6: Ipv6Addr) -> bool {
    let s = v6.segments();
    let refusee = v6.is_unspecified()
        || v6.is_loopback()
        || v6.is_multicast()
        || v6.to_ipv4_mapped().is_some()            // ::ffff:a.b.c.d
        || s[..6] == [0, 0, 0, 0, 0, 0]              // ::a.b.c.d (compatible, obsolète)
        || (s[0] & 0xFE00) == 0xFC00                 // ULA fc00::/7
        || (s[0] & 0xFFC0) == 0xFE80                 // lien-local fe80::/10
        || (s[0] & 0xFFC0) == 0xFEC0                 // site-local fec0::/10 (obsolète)
        || (s[0] == 0x0064 && s[1] == 0xFF9B)        // NAT64 64:ff9b::/96 et 64:ff9b:1::/48
        || (s[0] == 0x0100 && s[1..4] == [0, 0, 0])  // discard 100::/64
        || (s[0] == 0x2001 && s[1] == 0x0000)        // Teredo 2001::/32
        || (s[0] == 0x2001 && s[1] == 0x0DB8)        // documentation
        || s[0] == 0x2002                            // 6to4 (embarque une IPv4)
        || (s[0] == 0x3FFF && s[1] < 0x1000); // documentation 3fff::/20
    !refusee
}

/// Analyse et valide une URL, résout l'hôte et vérifie chaque adresse.
pub async fn valider_cible(url: &str, resolveur: &dyn Resolveur) -> Result<CibleValidee, RefusWeb> {
    if url.len() > LONGUEUR_MAX_URL {
        return Err(RefusWeb::UrlTropLongue);
    }
    let url = reqwest::Url::parse(url).map_err(|_| RefusWeb::UrlInvalide)?;
    valider_url(url, resolveur).await
}

async fn valider_url(
    url: reqwest::Url,
    resolveur: &dyn Resolveur,
) -> Result<CibleValidee, RefusWeb> {
    if url.as_str().len() > LONGUEUR_MAX_URL {
        return Err(RefusWeb::UrlTropLongue);
    }
    if !matches!(url.scheme(), "http" | "https") {
        return Err(RefusWeb::SchemaInterdit);
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(RefusWeb::IdentifiantsInterdits);
    }
    let port = url.port_or_known_default().ok_or(RefusWeb::UrlInvalide)?;
    if !PORTS_AUTORISES.contains(&port) {
        return Err(RefusWeb::PortInterdit(port));
    }
    let hote_brut = url.host_str().ok_or(RefusWeb::HoteAbsent)?;
    // IPv6 littérale : host_str() renvoie « [::1] ».
    let hote = hote_brut
        .strip_prefix('[')
        .and_then(|h| h.strip_suffix(']'))
        .unwrap_or(hote_brut)
        .to_string();

    // L'analyse WHATWG normalise déjà 127.1, 2130706433, 0x7f000001… en IPv4.
    if let Ok(ip) = hote.parse::<IpAddr>() {
        if !ip_publique(ip) {
            return Err(RefusWeb::AdresseNonPublique(ip));
        }
        return Ok(CibleValidee {
            url,
            hote,
            adresse: SocketAddr::new(ip, port),
            hote_est_ip: true,
        });
    }

    let minuscule = hote.to_ascii_lowercase();
    let minuscule = minuscule.trim_end_matches('.');
    // Suffixes de noms non publics (le nom est déjà en minuscules).
    let suffixe_interne = ["localhost", "local", "internal"]
        .iter()
        .any(|tld| minuscule.rsplit('.').next() == Some(*tld));
    if suffixe_interne || !minuscule.contains('.') {
        return Err(RefusWeb::HoteNonPublic(hote));
    }

    let adresses = resolveur
        .resoudre(&hote, port)
        .await
        .map_err(|_| RefusWeb::ResolutionImpossible)?;
    if adresses.is_empty() {
        return Err(RefusWeb::ResolutionImpossible);
    }
    // Toutes les adresses doivent être publiques (pas de mélange exploitable).
    if let Some(ip) = adresses.iter().find(|ip| !ip_publique(**ip)) {
        return Err(RefusWeb::AdresseNonPublique(*ip));
    }
    Ok(CibleValidee {
        url,
        hote,
        adresse: SocketAddr::new(adresses[0], port),
        hote_est_ip: false,
    })
}

/// Valide la cible d'une redirection relative à l'URL courante.
pub async fn valider_redirection(
    courante: &reqwest::Url,
    location: &str,
    resolveur: &dyn Resolveur,
) -> Result<CibleValidee, RefusWeb> {
    if location.len() > LONGUEUR_MAX_URL {
        return Err(RefusWeb::UrlTropLongue);
    }
    let suivante = courante
        .join(location)
        .map_err(|_| RefusWeb::RedirectionInvalide)?;
    valider_url(suivante, resolveur).await
}

/// Vrai si le type de contenu est textuel et accepté.
pub fn type_contenu_accepte(content_type: &str) -> bool {
    let essence = content_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    essence.starts_with("text/")
        || TYPES_APPLICATION.contains(&essence.as_str())
        || (essence.starts_with("application/") && essence.ends_with("+json"))
}

/// Tronque une chaîne sur une frontière de caractère.
fn tronquer(texte: &mut String, max: usize) -> bool {
    if texte.len() <= max {
        return false;
    }
    let mut fin = max;
    while !texte.is_char_boundary(fin) {
        fin -= 1;
    }
    texte.truncate(fin);
    true
}

async fn recuperer(
    url: &str,
    methode: reqwest::Method,
    entetes: &reqwest::header::HeaderMap,
    delai: Duration,
    resolveur: &dyn Resolveur,
) -> Result<SkillResult, RefusWeb> {
    let debut = Instant::now();
    let mut cible = valider_cible(url, resolveur).await?;
    let mut sauts = 0usize;

    loop {
        let restant = delai
            .checked_sub(debut.elapsed())
            .ok_or(RefusWeb::DelaiDepasse)?;
        let mut constructeur = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .timeout(restant)
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
            .user_agent("MiyukiniMAIA/0.1");
        if !cible.hote_est_ip {
            // Épinglage : la connexion se fait sur l'adresse vérifiée.
            constructeur = constructeur.resolve(&cible.hote, cible.adresse);
        }
        let client = constructeur
            .build()
            .map_err(|e| RefusWeb::Reseau(e.to_string()))?;

        let mut reponse = client
            .request(methode.clone(), cible.url.clone())
            .headers(entetes.clone())
            .send()
            .await
            .map_err(|e| RefusWeb::Reseau(e.without_url().to_string()))?;

        let status = reponse.status();
        if status.is_redirection() {
            if sauts >= MAX_REDIRECTIONS {
                return Err(RefusWeb::TropDeRedirections);
            }
            let location = reponse
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|v| v.to_str().ok())
                .ok_or(RefusWeb::RedirectionInvalide)?
                .to_string();
            cible = valider_redirection(&cible.url, &location, resolveur).await?;
            sauts += 1;
            continue;
        }

        let content_type = reponse
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if methode == reqwest::Method::HEAD {
            return Ok(SkillResult {
                success: status.is_success(),
                output: serde_json::json!({
                    "status": status.as_u16(),
                    "content_type": content_type,
                    "url": cible.url.as_str(),
                    "redirections": sauts,
                }),
                error: (!status.is_success()).then(|| format!("HTTP {}", status.as_u16())),
                duration_ms: 0,
            });
        }

        if !type_contenu_accepte(&content_type) {
            return Err(RefusWeb::TypeContenuInterdit(if content_type.is_empty() {
                "(absent)".into()
            } else {
                content_type
            }));
        }

        let mut octets: Vec<u8> = Vec::new();
        let mut tronque = false;
        while let Some(morceau) = reponse
            .chunk()
            .await
            .map_err(|e| RefusWeb::Reseau(e.without_url().to_string()))?
        {
            let reste = MAX_RESPONSE_SIZE.saturating_sub(octets.len());
            if morceau.len() > reste {
                octets.extend_from_slice(&morceau[..reste]);
                tronque = true;
                break;
            }
            octets.extend_from_slice(&morceau);
        }
        let taille = octets.len();
        let mut texte = String::from_utf8_lossy(&octets).into_owned();

        let corps = if content_type.contains("json") && !tronque {
            serde_json::from_str::<serde_json::Value>(&texte)
                .unwrap_or(serde_json::Value::String(texte))
        } else {
            if tronquer(&mut texte, MAX_TEXTE_MODELE) {
                texte.push_str("\n... [tronqué à 4KB pour économiser les tokens]");
            }
            serde_json::Value::String(texte)
        };

        let code = status.as_u16();
        return Ok(SkillResult {
            success: status.is_success(),
            output: serde_json::json!({
                "status": code,
                "content_type": content_type,
                "url": cible.url.as_str(),
                "redirections": sauts,
                "body": corps,
                "size": taille,
                "truncated": tronque,
            }),
            error: (code >= 400).then(|| format!("HTTP {code}")),
            duration_ms: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Résolveur factice : table nom → adresses.
    struct ResolveurFactice(Vec<(&'static str, Vec<IpAddr>)>);

    impl Resolveur for ResolveurFactice {
        fn resoudre<'a>(
            &'a self,
            hote: &'a str,
            _port: u16,
        ) -> Pin<Box<dyn Future<Output = std::io::Result<Vec<IpAddr>>> + Send + 'a>> {
            let r = self
                .0
                .iter()
                .find(|(n, _)| *n == hote)
                .map(|(_, a)| a.clone())
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "nx"));
            Box::pin(async move { r })
        }
    }

    fn ip(s: &str) -> IpAddr {
        s.parse().unwrap()
    }

    fn resolveur() -> ResolveurFactice {
        ResolveurFactice(vec![
            ("exemple.org", vec![ip("93.184.216.34")]),
            ("localtest.me", vec![ip("127.0.0.1")]),
            ("interne.exemple.org", vec![ip("10.0.0.5")]),
            ("meta.exemple.org", vec![ip("169.254.169.254")]),
            (
                "mixte.exemple.org",
                vec![ip("93.184.216.34"), ip("192.168.1.1")],
            ),
            ("v6.exemple.org", vec![ip("::ffff:127.0.0.1")]),
            ("ula.exemple.org", vec![ip("fd00::1")]),
            ("vide.exemple.org", vec![]),
        ])
    }

    #[test]
    fn plages_non_publiques_refusees() {
        for a in [
            "127.0.0.1",
            "127.0.0.2",
            "10.1.2.3",
            "172.16.0.1",
            "172.31.255.255",
            "192.168.0.1",
            "169.254.169.254",
            "100.64.0.1",
            "100.127.255.255",
            "0.0.0.0",
            "224.0.0.1",
            "255.255.255.255",
            "240.0.0.1",
            "198.18.0.1",
            "192.0.2.1",
            "::1",
            "::",
            "fe80::1",
            "fc00::1",
            "fd12:3456::1",
            "ff02::1",
            "::ffff:127.0.0.1",
            "::ffff:10.0.0.1",
            "::127.0.0.1",
            "64:ff9b::7f00:1",
            "2002:7f00:1::",
            "2001:0:4136:e378::1",
            "2001:db8::1",
        ] {
            assert!(!ip_publique(ip(a)), "{a} devrait être refusée");
        }
        for a in [
            "93.184.216.34",
            "1.1.1.1",
            "172.32.0.1",
            "100.128.0.1",
            "2606:4700::1111",
        ] {
            assert!(ip_publique(ip(a)), "{a} devrait être publique");
        }
    }

    #[tokio::test]
    async fn urls_litterales_non_publiques_refusees() {
        let r = resolveur();
        for u in [
            "http://127.0.0.1/",
            "http://127.1/",
            "http://2130706433/",
            "http://0x7f000001/",
            "http://017700000001/",
            "http://[::1]/",
            "http://[::ffff:127.0.0.1]/",
            "http://169.254.169.254/latest/meta-data/",
            "http://172.17.0.1/",
            "http://100.64.1.1/",
            "http://0.0.0.0/",
            "http://192.168.1.1:8080/",
        ] {
            let e = valider_cible(u, &r).await.unwrap_err();
            assert!(matches!(e, RefusWeb::AdresseNonPublique(_)), "{u} → {e:?}");
        }
    }

    #[tokio::test]
    async fn noms_resolus_vers_prive_refuses() {
        let r = resolveur();
        for u in [
            "http://localtest.me/",
            "https://interne.exemple.org/",
            "http://meta.exemple.org/latest/meta-data/",
            "https://mixte.exemple.org/",
            "https://v6.exemple.org/",
            "https://ula.exemple.org/",
        ] {
            let e = valider_cible(u, &r).await.unwrap_err();
            assert!(matches!(e, RefusWeb::AdresseNonPublique(_)), "{u} → {e:?}");
        }
        assert_eq!(
            valider_cible("https://vide.exemple.org/", &r)
                .await
                .unwrap_err(),
            RefusWeb::ResolutionImpossible
        );
        assert_eq!(
            valider_cible("https://inconnu.exemple.org/", &r)
                .await
                .unwrap_err(),
            RefusWeb::ResolutionImpossible
        );
    }

    #[tokio::test]
    async fn formes_d_url_refusees() {
        let r = resolveur();
        let cas: &[(&str, RefusWeb)] = &[
            ("ftp://exemple.org/", RefusWeb::SchemaInterdit),
            ("file:///etc/passwd", RefusWeb::SchemaInterdit),
            (
                "http://user:pw@exemple.org/",
                RefusWeb::IdentifiantsInterdits,
            ),
            (
                "http://exemple.org@127.0.0.1/",
                RefusWeb::IdentifiantsInterdits,
            ),
            ("http://exemple.org:11435/", RefusWeb::PortInterdit(11435)),
            (
                "http://localhost/",
                RefusWeb::HoteNonPublic("localhost".into()),
            ),
            (
                "http://a.localhost/",
                RefusWeb::HoteNonPublic("a.localhost".into()),
            ),
            (
                "http://intranet/",
                RefusWeb::HoteNonPublic("intranet".into()),
            ),
            ("pas une url", RefusWeb::UrlInvalide),
        ];
        for (u, attendu) in cas {
            assert_eq!(&valider_cible(u, &r).await.unwrap_err(), attendu, "{u}");
        }
        let long = format!("https://exemple.org/{}", "a".repeat(LONGUEUR_MAX_URL));
        assert_eq!(
            valider_cible(&long, &r).await.unwrap_err(),
            RefusWeb::UrlTropLongue
        );
    }

    #[tokio::test]
    async fn cible_publique_acceptee_et_epinglee() {
        let r = resolveur();
        let c = valider_cible("https://exemple.org/page", &r).await.unwrap();
        assert_eq!(c.hote, "exemple.org");
        assert_eq!(c.adresse, SocketAddr::new(ip("93.184.216.34"), 443));
        assert!(!c.hote_est_ip);
    }

    #[tokio::test]
    async fn redirection_vers_prive_refusee() {
        let r = resolveur();
        let base = reqwest::Url::parse("https://exemple.org/a").unwrap();
        for loc in [
            "http://127.0.0.1:8080/v1/agents",
            "http://[::1]/",
            "https://interne.exemple.org/x",
            "http://meta.exemple.org/latest/meta-data/",
            "//localtest.me/",
        ] {
            let e = valider_redirection(&base, loc, &r).await.unwrap_err();
            assert!(
                matches!(e, RefusWeb::AdresseNonPublique(_)),
                "{loc} → {e:?}"
            );
        }
        assert_eq!(
            valider_redirection(&base, "http://127.0.0.1:11435/v1/agents", &r)
                .await
                .unwrap_err(),
            RefusWeb::PortInterdit(11435)
        );
        assert_eq!(
            valider_redirection(&base, "file:///etc/passwd", &r)
                .await
                .unwrap_err(),
            RefusWeb::SchemaInterdit
        );
        let ok = valider_redirection(&base, "/b?c=1", &r).await.unwrap();
        assert_eq!(ok.url.as_str(), "https://exemple.org/b?c=1");
    }

    #[tokio::test]
    async fn execute_refuse_methodes_entetes_et_corps() {
        let r = resolveur();
        let mut p = HashMap::new();
        p.insert("url".into(), serde_json::json!("https://exemple.org/"));
        p.insert("method".into(), serde_json::json!("POST"));
        let s = executer_avec(&p, &r).await;
        assert!(s
            .error
            .unwrap_or_default()
            .contains("méthode POST interdite"));

        p.insert("method".into(), serde_json::json!("GET"));
        p.insert(
            "headers".into(),
            serde_json::json!({ "Authorization": "Bearer x" }),
        );
        let s = executer_avec(&p, &r).await;
        assert!(s
            .error
            .unwrap_or_default()
            .contains("en-tête Authorization"));

        p.remove("headers");
        p.insert("body".into(), serde_json::json!({ "a": 1 }));
        let s = executer_avec(&p, &r).await;
        assert!(s
            .error
            .unwrap_or_default()
            .contains("corps de requête interdit"));
    }

    #[tokio::test]
    async fn execute_refuse_metadonnees_cloud_sans_reseau() {
        let r = resolveur();
        let mut p = HashMap::new();
        p.insert(
            "url".into(),
            serde_json::json!("http://169.254.169.254/latest/meta-data/"),
        );
        let s = executer_avec(&p, &r).await;
        assert!(!s.success);
        assert!(s.error.unwrap_or_default().contains("adresse non publique"));
    }

    #[test]
    fn types_de_contenu() {
        for t in [
            "text/html; charset=utf-8",
            "text/plain",
            "application/json",
            "application/problem+json",
            "application/xhtml+xml",
        ] {
            assert!(type_contenu_accepte(t), "{t}");
        }
        for t in [
            "",
            "application/octet-stream",
            "image/png",
            "application/pdf",
            "video/mp4",
        ] {
            assert!(!type_contenu_accepte(t), "{t}");
        }
    }

    #[test]
    fn troncature_sur_frontiere_utf8() {
        let mut s = "é".repeat(10);
        assert!(tronquer(&mut s, 5));
        assert_eq!(s, "éé");
    }
}
