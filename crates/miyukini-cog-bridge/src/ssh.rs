//! Mode SSH — tunnel sécurisé vers le COG Host avec port forwarding.
//!
//! Utilise `russh` (client SSH pur Rust) pour établir un tunnel chiffré.
//! Ports forwardés :
//! - 50051 → KindMother gRPC
//! - 8080  → Origin/services web
//! - 8081  → Admin
//! - 3030  → CentralRemote WebSocket

use crate::{BridgeError, BridgeResult};

/// Ports à forwarder via le tunnel SSH.
pub const FORWARDED_PORTS: &[(u16, u16)] = &[
    (50051, 50051), // KindMother gRPC
    (8080, 8080),   // Origin web
    (8081, 8081),   // Admin
    (3030, 3030),   // CentralRemote WebSocket
];

/// Établit un tunnel SSH vers le COG Host.
pub async fn connect(
    host: &str,
    port: u16,
    username: &str,
    _private_key_pem: &str,
) -> BridgeResult<()> {
    tracing::info!("SSH: connexion à {username}@{host}:{port}");

    // TODO Phase 0 : implémenter le tunnel SSH avec russh
    // 1. Parser la clé privée ed25519
    // 2. Établir la connexion SSH
    // 3. Ouvrir les port forwardings (FORWARDED_PORTS)
    // 4. Maintenir la session en arrière-plan

    // Pour l'instant, vérifier que le host est joignable
    let addr = format!("{host}:{port}");
    tokio::net::TcpStream::connect(&addr)
        .await
        .map_err(|e| BridgeError::SshError(format!("TCP connect failed: {e}")))?;

    tracing::info!("SSH: connexion TCP OK vers {addr}");
    Ok(())
}

/// Génère une paire de clés ed25519 pour l'appairage SSH.
pub fn generate_keypair() -> (String, String) {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();

    // Format OpenSSH pour la clé publique
    let pub_key_bytes = verifying_key.to_bytes();
    let pub_key_b64 = base64_encode(&pub_key_bytes);
    let public_key = format!("ssh-ed25519 {pub_key_b64} central-mobile");

    // Clé privée en hex (sera convertie en PEM dans une version future)
    let private_key = hex::encode(signing_key.to_bytes());

    (private_key, public_key)
}

fn base64_encode(data: &[u8]) -> String {
    // Construction manuelle du blob SSH ed25519
    let key_type = b"ssh-ed25519";
    let mut blob = Vec::new();
    // longueur du type (4 bytes big-endian) + type
    blob.extend_from_slice(&(key_type.len() as u32).to_be_bytes());
    blob.extend_from_slice(key_type);
    // longueur de la clé (4 bytes big-endian) + clé
    blob.extend_from_slice(&(data.len() as u32).to_be_bytes());
    blob.extend_from_slice(data);

    use std::io::Write;
    let mut buf = Vec::new();
    {
        let mut encoder = Base64Encoder::new(&mut buf);
        encoder.write_all(&blob).unwrap();
        encoder.finish().unwrap();
    }
    String::from_utf8(buf).unwrap()
}

/// Encodeur base64 minimal (pas de dépendance externe).
struct Base64Encoder<W: std::io::Write> {
    writer: W,
    buf: Vec<u8>,
}

impl<W: std::io::Write> Base64Encoder<W> {
    fn new(writer: W) -> Self {
        Self {
            writer,
            buf: Vec::new(),
        }
    }

    fn finish(mut self) -> std::io::Result<()> {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = Vec::new();
        for chunk in self.buf.chunks(3) {
            match chunk.len() {
                3 => {
                    let n = (u32::from(chunk[0]) << 16) | (u32::from(chunk[1]) << 8) | u32::from(chunk[2]);
                    out.push(CHARS[((n >> 18) & 63) as usize]);
                    out.push(CHARS[((n >> 12) & 63) as usize]);
                    out.push(CHARS[((n >> 6) & 63) as usize]);
                    out.push(CHARS[(n & 63) as usize]);
                }
                2 => {
                    let n = (u32::from(chunk[0]) << 16) | (u32::from(chunk[1]) << 8);
                    out.push(CHARS[((n >> 18) & 63) as usize]);
                    out.push(CHARS[((n >> 12) & 63) as usize]);
                    out.push(CHARS[((n >> 6) & 63) as usize]);
                    out.push(b'=');
                }
                1 => {
                    let n = u32::from(chunk[0]) << 16;
                    out.push(CHARS[((n >> 18) & 63) as usize]);
                    out.push(CHARS[((n >> 12) & 63) as usize]);
                    out.push(b'=');
                    out.push(b'=');
                }
                _ => {}
            }
        }
        self.writer.write_all(&out)
    }
}

impl<W: std::io::Write> std::io::Write for Base64Encoder<W> {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
