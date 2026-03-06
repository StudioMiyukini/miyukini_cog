//! Configuration du serveur MiyuCloud.
//!
//! @id: miyucloud_server_config
//! @do: define_server_configuration
//! @role: config
//! @layer: app
//!
//! La configuration est lue depuis les variables d'environnement avec
//! des valeurs par defaut sensibles.

use std::fmt::Write;
use std::path::PathBuf;

/// Configuration du serveur MiyuCloud.
#[derive(Debug, Clone)]
pub struct MiyucloudConfig {
    /// Port de l'API interne (defaut 11440).
    pub api_port: u16,
    /// Chemin racine du stockage (defaut `~/.miyucloud/storage`).
    pub storage_path: PathBuf,
    /// Chemin de la base de donnees (defaut `~/.miyucloud/miyucloud.db`).
    pub db_path: PathBuf,
    /// Token COG pour l'authentification interne.
    pub cog_token: String,
    /// Passphrase pour le chiffrement (variable d'env `MIYUCLOUD_PASSPHRASE`).
    pub passphrase: Option<String>,
    /// Owner ID par defaut (pour le MVP mono-utilisateur).
    pub owner_id: String,
    /// Port de la surface web publique HTTPS (defaut 11442).
    pub web_port: u16,
    /// Active ou desactive la surface web (defaut true).
    pub web_enabled: bool,
    /// Fait confiance aux en-tetes proxy (X-Forwarded-For/X-Real-IP) pour l'IP cliente.
    pub trust_proxy: bool,
    /// Quota par defaut impose au proprietaire courant.
    pub default_quota_bytes: u64,
    /// Active les liens de partage publics.
    pub public_share_links_enabled: bool,
    /// Active le partage interne profil/tribu.
    pub internal_sharing_enabled: bool,
    /// Permission de partage par defaut.
    pub default_share_permission: String,
    /// Active le profil de connexion SSH persistant.
    pub ssh_enabled: bool,
    /// Hote SSH cible.
    pub ssh_host: Option<String>,
    /// Port SSH.
    pub ssh_port: u16,
    /// Utilisateur SSH.
    pub ssh_username: Option<String>,
    /// Racine distante SSH.
    pub ssh_root_path: Option<String>,
    /// Cle privee SSH optionnelle.
    pub ssh_private_key_path: Option<PathBuf>,
    /// Demande des keepalive pour la session SSH.
    pub ssh_keepalive: bool,
    /// Chemin vers la base Central pour Miyukini Connect.
    pub central_db_path: Option<PathBuf>,
    /// Chemin vers le certificat TLS PEM (si fourni, desactive auto-generation).
    pub tls_cert_path: Option<PathBuf>,
    /// Chemin vers la cle privee TLS PEM.
    pub tls_key_path: Option<PathBuf>,
    /// Chemin de la base de donnees CalDAV/CardDAV (defaut `~/.miyucloud/dav.db`).
    pub dav_db_path: PathBuf,
}

impl MiyucloudConfig {
    /// Charge la configuration depuis les variables d'environnement.
    pub fn from_env() -> Self {
        let home = dirs_home();
        let base = home.join(".miyucloud");

        let api_port = std::env::var("MIYUCLOUD_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(11440);

        let storage_path = std::env::var("MIYUCLOUD_STORAGE_PATH")
            .map_or_else(|_| base.join("storage"), PathBuf::from);

        let db_path = std::env::var("MIYUCLOUD_DB_PATH")
            .map_or_else(|_| base.join("miyucloud.db"), PathBuf::from);

        let cog_token = std::env::var("MIYUCLOUD_COG_TOKEN").unwrap_or_else(|_| {
            // Generate a random token if not set
            generate_random_token()
        });

        let passphrase = std::env::var("MIYUCLOUD_PASSPHRASE").ok();

        let owner_id =
            std::env::var("MIYUCLOUD_OWNER_ID").unwrap_or_else(|_| "default-owner".to_string());

        let web_port = std::env::var("MIYUCLOUD_WEB_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(11442);

        let web_enabled = std::env::var("MIYUCLOUD_WEB_ENABLED")
            .map_or(true, |s| s != "0" && s.to_lowercase() != "false");

        let trust_proxy = std::env::var("MIYUCLOUD_TRUST_PROXY")
            .map_or(false, |s| s == "1" || s.to_lowercase() == "true");

        let default_quota_bytes = std::env::var("MIYUCLOUD_DEFAULT_QUOTA_BYTES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10 * 1024 * 1024 * 1024);

        let public_share_links_enabled = std::env::var("MIYUCLOUD_PUBLIC_SHARES_ENABLED")
            .map_or(true, |s| s == "1" || s.to_lowercase() == "true");

        let internal_sharing_enabled =
            std::env::var("MIYUCLOUD_INTERNAL_SHARING_ENABLED")
                .map_or(true, |s| s == "1" || s.to_lowercase() == "true");

        let default_share_permission =
            std::env::var("MIYUCLOUD_DEFAULT_SHARE_PERMISSION")
                .unwrap_or_else(|_| "read".to_string());

        let ssh_enabled = std::env::var("MIYUCLOUD_SSH_ENABLED")
            .map_or(false, |s| s == "1" || s.to_lowercase() == "true");

        let ssh_host = std::env::var("MIYUCLOUD_SSH_HOST")
            .ok()
            .filter(|s| !s.trim().is_empty());

        let ssh_port = std::env::var("MIYUCLOUD_SSH_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(22);

        let ssh_username = std::env::var("MIYUCLOUD_SSH_USERNAME")
            .ok()
            .filter(|s| !s.trim().is_empty());

        let ssh_root_path = std::env::var("MIYUCLOUD_SSH_ROOT_PATH")
            .ok()
            .filter(|s| !s.trim().is_empty());

        let ssh_private_key_path = std::env::var("MIYUCLOUD_SSH_PRIVATE_KEY_PATH")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from);

        let ssh_keepalive = std::env::var("MIYUCLOUD_SSH_KEEPALIVE")
            .map_or(true, |s| s == "1" || s.to_lowercase() == "true");

        let central_db_path = std::env::var("MIYUCLOUD_CENTRAL_DB_PATH")
            .ok()
            .map(PathBuf::from);

        let tls_cert_path = std::env::var("MIYUCLOUD_TLS_CERT").ok().map(PathBuf::from);

        let tls_key_path = std::env::var("MIYUCLOUD_TLS_KEY").ok().map(PathBuf::from);

        let dav_db_path = std::env::var("MIYUCLOUD_DAV_DB_PATH")
            .map_or_else(|_| base.join("dav.db"), PathBuf::from);

        Self {
            api_port,
            storage_path,
            db_path,
            cog_token,
            passphrase,
            owner_id,
            web_port,
            web_enabled,
            trust_proxy,
            default_quota_bytes,
            public_share_links_enabled,
            internal_sharing_enabled,
            default_share_permission,
            ssh_enabled,
            ssh_host,
            ssh_port,
            ssh_username,
            ssh_root_path,
            ssh_private_key_path,
            ssh_keepalive,
            central_db_path,
            tls_cert_path,
            tls_key_path,
            dav_db_path,
        }
    }
}

/// Retourne le repertoire home de l'utilisateur.
fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_or_else(|_| PathBuf::from("."), PathBuf::from)
}

/// Genere un token aleatoire de 32 bytes en hexadecimal.
fn generate_random_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    bytes.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}
