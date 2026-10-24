//! @id     central_cle_service
//! @do     signer_requetes_central_vers_maia
//! @role   securite
//! @layer  app
//! @human  central — clé de service de Central (graine 0600) et signature de ses appels à MAIA.

//! Clé de service de Central (sortie du lot 2).
//!
//! MAIA (`apps/miou-llm-bridge`) n'accepte plus que des requêtes **signées** par
//! un appelant épinglé (`[security] cles_appelants` de `bridge.toml`). Central
//! détient donc sa propre clé Ed25519 :
//!
//! - graine dans `<MIYUKINI_DATA_DIR>/keys/central.seed` (fichier `0600`, créé une
//!   seule fois par [`charger_ou_creer_cle`]) — même convention que
//!   `keys/maia.seed` côté MAIA. Sans `MIYUKINI_DATA_DIR`, le dossier de données
//!   de Central ([`miyukini_central::chemins::dossier_donnees`]) ;
//! - clé publique journalisée au démarrage (base64 URL et hexadécimal) pour être
//!   épinglée par l'opérateur ;
//! - transmise automatiquement à MAIA quand Central la lance
//!   ([`ENV_CLES_APPELANTS_MAIA`]).
//!
//! Clé illisible ⇒ Central continue sans signer : MAIA refusera ses appels
//! (fail-closed côté service), l'erreur est journalisée.

use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use miyauth::http::service::{charger_ou_creer_cle, encoder_cle_publique, entetes_signees};
use miyusceau::CleSignature;

/// Usage signé des appels à l'API protégée de MAIA (`signature::USAGE_API` du bridge).
pub const USAGE_MAIA_API: &str = "maia/api";

/// Variable d'environnement lue par MAIA au chargement de sa configuration :
/// clés publiques ajoutées à `[security] cles_appelants` (séparées par des virgules).
pub const ENV_CLES_APPELANTS_MAIA: &str = "MAIA_CLES_APPELANTS_SUPPLEMENTAIRES";

/// Identifiants de service sous lesquels MAIA est installée (ancien et nouveau nom).
pub const IDS_SERVICE_MAIA: &[&str] = &["miou-llm-bridge", "maia"];

/// Nom du fichier de graine de Central.
const NOM_GRAINE: &str = "central.seed";

static CLE: OnceLock<Option<Arc<CleSignature>>> = OnceLock::new();

/// Chemin de la graine : `$MIYUKINI_DATA_DIR/keys/central.seed`, sinon le dossier
/// de données de Central. `None` si aucun dossier n'est résolu (jamais le cwd en release).
pub fn chemin_cle() -> Option<PathBuf> {
    let dossier = match std::env::var_os("MIYUKINI_DATA_DIR").filter(|d| !d.is_empty()) {
        Some(d) => PathBuf::from(d),
        None => miyukini_central::chemins::dossier_donnees().ok()?,
    };
    Some(dossier.join("keys").join(NOM_GRAINE))
}

/// Clé de service de Central (chargée ou créée au premier appel, puis mémorisée).
pub fn cle() -> Option<Arc<CleSignature>> {
    CLE.get_or_init(|| {
        let Some(chemin) = chemin_cle() else {
            tracing::error!("Clé de service de Central : dossier de données introuvable — appels à MAIA non signés");
            return None;
        };
        match charger_ou_creer_cle(&chemin) {
            Ok(c) => Some(Arc::new(c)),
            Err(e) => {
                tracing::error!(
                    "Clé de service de Central illisible ({}) : {e} — appels à MAIA non signés (refusés)",
                    chemin.display()
                );
                None
            }
        }
    })
    .clone()
}

/// Clé publique de Central en base64 URL (forme à épingler), si la clé est disponible.
pub fn cle_publique_b64() -> Option<String> {
    cle().map(|c| encoder_cle_publique(&c.cle_publique()))
}

/// Hexadécimal minuscule (forme alternative acceptée par `cles_appelants`).
fn hex(octets: &[u8]) -> String {
    use std::fmt::Write;
    octets.iter().fold(String::with_capacity(octets.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Charge la clé au démarrage et journalise la clé publique à épingler.
pub fn initialiser() {
    if let Some(c) = cle() {
        let publique = c.cle_publique();
        tracing::info!(
            "Clé publique de Central (à épingler dans bridge.toml `[security] cles_appelants`) : {} (hex {})",
            encoder_cle_publique(&publique),
            hex(&publique)
        );
    }
}

/// Chemin + requête d'une URL, tels que vérifiés par MAIA. Vide si l'URL est
/// invalide (la signature sera alors refusée).
pub fn chemin_url(url: &str) -> String {
    reqwest::Url::parse(url).map_or_else(
        |_| String::new(),
        |u| match u.query() {
            Some(q) => format!("{}?{q}", u.path()),
            None => u.path().to_string(),
        },
    )
}

/// Ajoute les en-têtes `X-Miyu-*` signés (usage [`USAGE_MAIA_API`]) à une requête
/// vers MAIA. Sans clé : requête inchangée (donc refusée par MAIA).
pub fn signer_maia(
    rb: reqwest::RequestBuilder,
    cle: Option<&CleSignature>,
    methode: &str,
    url: &str,
    corps: &[u8],
) -> reqwest::RequestBuilder {
    let Some(cle) = cle else {
        return rb;
    };
    entetes_signees(cle, USAGE_MAIA_API, methode, &chemin_url(url), corps)
        .into_iter()
        .fold(rb, |rb, (nom, valeur)| rb.header(nom, valeur))
}

/// Variables d'environnement à transmettre au lancement d'un service : la clé
/// publique de Central pour MAIA (pas un secret), rien pour les autres.
pub fn env_lancement(service_id: &str, cle_publique: Option<&str>) -> Option<(&'static str, String)> {
    if !IDS_SERVICE_MAIA.contains(&service_id) {
        return None;
    }
    cle_publique.map(|c| (ENV_CLES_APPELANTS_MAIA, c.to_string()))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use miyauth::http::service::{maintenant_unix, GardeSignee};
    use miyauth::http::{ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};

    #[test]
    fn chemin_url_avec_requete() {
        assert_eq!(chemin_url("http://localhost:11435/v1/models?x=1"), "/v1/models?x=1");
        assert_eq!(chemin_url("http://localhost:11435/v1/chat/completions"), "/v1/chat/completions");
        assert_eq!(chemin_url("pas une url"), "");
    }

    #[test]
    fn env_lancement_reserve_a_maia() {
        assert_eq!(
            env_lancement("miou-llm-bridge", Some("k")),
            Some((ENV_CLES_APPELANTS_MAIA, "k".to_string()))
        );
        assert!(env_lancement("maia", Some("k")).is_some());
        assert!(env_lancement("miyucloud", Some("k")).is_none());
        assert!(env_lancement("maia", None).is_none());
    }

    #[test]
    fn hex_minuscule() {
        assert_eq!(hex(&[0x00, 0xab, 0x10]), "00ab10");
    }

    /// Vérifie une requête construite comme le fait MAIA (`signature::verifier`) :
    /// en-têtes, chemin + requête de l'URI, corps tel qu'envoyé.
    pub(crate) fn verifier_comme_maia(garde: &GardeSignee, req: &reqwest::Request) -> Result<(), String> {
        let h = req.headers();
        let v = |n: &str| h.get(n).and_then(|x| x.to_str().ok());
        let t = maintenant_unix();
        let entetes = garde
            .controler_entetes(v(ENTETE_SIGNATAIRE), v(ENTETE_HORODATAGE), v(ENTETE_NONCE), v(ENTETE_SIGNATURE), t)
            .map_err(|e| format!("{e:?}"))?;
        let corps = req.body().and_then(reqwest::Body::as_bytes).unwrap_or_default();
        let chemin = match req.url().query() {
            Some(q) => format!("{}?{q}", req.url().path()),
            None => req.url().path().to_string(),
        };
        garde
            .verifier(&entetes, USAGE_MAIA_API, req.method().as_str(), &chemin, corps, t)
            .map_err(|e| format!("{e:?}"))
    }

    #[test]
    fn requete_signee_acceptee_non_signee_refusee() {
        let cle = CleSignature::generer();
        let garde = GardeSignee::new([cle.cle_publique()]);
        let client = reqwest::Client::new();
        let url = "http://localhost:11435/v1/models?filtre=a";
        let req = signer_maia(client.get(url), Some(&cle), "GET", url, b"").build().unwrap();
        assert_eq!(verifier_comme_maia(&garde, &req), Ok(()));
        // Rejeu refusé.
        assert!(verifier_comme_maia(&garde, &req).is_err());
        // Sans clé : refusé.
        let req = signer_maia(client.get(url), None, "GET", url, b"").build().unwrap();
        assert!(verifier_comme_maia(&garde, &req).is_err());
        // Clé non épinglée : refusé.
        let autre = CleSignature::generer();
        let req = signer_maia(client.get(url), Some(&autre), "GET", url, b"").build().unwrap();
        assert!(verifier_comme_maia(&garde, &req).is_err());
    }
}
