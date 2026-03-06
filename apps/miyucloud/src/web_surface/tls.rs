//! Configuration TLS pour la surface web.
//!
//! @id: miyucloud_web_tls
//! @do: generate_self_signed_cert_and_configure_rustls
//! @role: security
//! @layer: app
//!
//! Au premier demarrage, genere un certificat auto-signe via `rcgen`.
//! Aux demarrages suivants, charge le certificat existant.
//! Supporte aussi le chargement d'un certificat Let's Encrypt fourni par l'utilisateur.
//!
//! INVARIANT : Seul HTTPS est accepte sur la surface web. Pas de HTTP.

use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn ensure_rustls_crypto_provider() {
    if rustls::crypto::CryptoProvider::get_default().is_none() {
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    }
}

/// Configuration TLS pour la surface web.
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Chemin vers le certificat PEM.
    pub cert_path: PathBuf,
    /// Chemin vers la cle privee PEM.
    pub key_path: PathBuf,
    /// `true` = generer un certificat auto-signe au premier demarrage.
    pub auto_generate: bool,
}

impl TlsConfig {
    /// Cree une configuration TLS avec les chemins par defaut.
    pub fn default_paths(base_dir: &Path) -> Self {
        let tls_dir = base_dir.join("tls");
        Self {
            cert_path: tls_dir.join("cert.pem"),
            key_path: tls_dir.join("key.pem"),
            auto_generate: true,
        }
    }

    /// S'assure que le certificat et la cle existent.
    /// Si `auto_generate` est vrai et qu'ils n'existent pas, genere un cert auto-signe.
    pub fn ensure_cert(&self) -> Result<(), String> {
        if self.cert_path.exists() && self.key_path.exists() {
            return Ok(());
        }

        if !self.auto_generate {
            return Err(format!(
                "TLS certificate not found at {} and auto-generation is disabled",
                self.cert_path.display()
            ));
        }

        generate_self_signed_cert(&self.cert_path, &self.key_path)
    }

    /// Charge la configuration rustls.
    pub fn load_rustls_config(&self) -> Result<Arc<rustls::ServerConfig>, String> {
        self.ensure_cert()?;
        load_rustls_config(&self.cert_path, &self.key_path)
    }
}

/// Genere un certificat auto-signe RSA 2048 valide 365 jours.
pub fn generate_self_signed_cert(cert_path: &Path, key_path: &Path) -> Result<(), String> {
    // Ensure parent directories exist
    if let Some(parent) = cert_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create TLS directory: {e}"))?;
    }

    // Generate certificate
    let mut params =
        rcgen::CertificateParams::new(vec!["localhost".to_string(), "127.0.0.1".to_string()])
            .map_err(|e| format!("Failed to create cert params: {e}"))?;

    params.not_after = rcgen::date_time_ymd(2027, 3, 1);
    params.not_before = rcgen::date_time_ymd(2026, 3, 1);
    params
        .subject_alt_names
        .push(rcgen::SanType::IpAddress(std::net::IpAddr::V4(
            std::net::Ipv4Addr::LOCALHOST,
        )));
    params
        .subject_alt_names
        .push(rcgen::SanType::IpAddress(std::net::IpAddr::V4(
            std::net::Ipv4Addr::UNSPECIFIED,
        )));

    let key_pair = rcgen::KeyPair::generate().map_err(|e| format!("Key generation failed: {e}"))?;
    let cert = params
        .self_signed(&key_pair)
        .map_err(|e| format!("Certificate generation failed: {e}"))?;

    // Write PEM files
    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    fs::write(cert_path, cert_pem.as_bytes()).map_err(|e| format!("Failed to write cert: {e}"))?;
    fs::write(key_path, key_pem.as_bytes()).map_err(|e| format!("Failed to write key: {e}"))?;

    Ok(())
}

/// Charge un certificat et une cle privee PEM et construit un `rustls::ServerConfig`.
pub fn load_rustls_config(
    cert_path: &Path,
    key_path: &Path,
) -> Result<Arc<rustls::ServerConfig>, String> {
    ensure_rustls_crypto_provider();

    // Load certificate chain
    let cert_file = fs::File::open(cert_path).map_err(|e| format!("Failed to open cert: {e}"))?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs: Vec<_> = rustls_pemfile::certs(&mut cert_reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to parse cert PEM: {e}"))?;

    if certs.is_empty() {
        return Err("No certificates found in PEM file".to_string());
    }

    // Load private key
    let key_file = fs::File::open(key_path).map_err(|e| format!("Failed to open key: {e}"))?;
    let mut key_reader = BufReader::new(key_file);
    let key = rustls_pemfile::private_key(&mut key_reader)
        .map_err(|e| format!("Failed to parse key PEM: {e}"))?
        .ok_or_else(|| "No private key found in PEM file".to_string())?;

    // Build rustls config
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|e| format!("Failed to build TLS config: {e}"))?;

    Ok(Arc::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_self_signed_cert() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("cert.pem");
        let key_path = dir.path().join("key.pem");

        generate_self_signed_cert(&cert_path, &key_path).unwrap();

        assert!(cert_path.exists());
        assert!(key_path.exists());

        // Verify files are valid PEM
        let cert_content = fs::read_to_string(&cert_path).unwrap();
        assert!(cert_content.contains("BEGIN CERTIFICATE"));

        let key_content = fs::read_to_string(&key_path).unwrap();
        assert!(key_content.contains("BEGIN PRIVATE KEY"));
    }

    #[test]
    fn test_load_existing_cert() {
        let dir = tempfile::tempdir().unwrap();
        let cert_path = dir.path().join("cert.pem");
        let key_path = dir.path().join("key.pem");

        generate_self_signed_cert(&cert_path, &key_path).unwrap();

        let config = load_rustls_config(&cert_path, &key_path);
        assert!(config.is_ok());
    }

    #[test]
    fn test_tls_config_auto_generate() {
        let dir = tempfile::tempdir().unwrap();
        let tls_config = TlsConfig::default_paths(dir.path());

        assert!(!tls_config.cert_path.exists());
        tls_config.ensure_cert().unwrap();
        assert!(tls_config.cert_path.exists());
        assert!(tls_config.key_path.exists());
    }

    #[test]
    fn test_tls_config_no_auto_generate() {
        let dir = tempfile::tempdir().unwrap();
        let tls_config = TlsConfig {
            cert_path: dir.path().join("nonexistent.pem"),
            key_path: dir.path().join("nonexistent.key"),
            auto_generate: false,
        };

        let result = tls_config.ensure_cert();
        assert!(result.is_err());
    }
}
