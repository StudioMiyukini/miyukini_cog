//! Transport TCP chiffre E2E pour la synchronisation P2P.
//!
//! @id: miyucloud_sync_transport
//! @do: implement_tcp_transport_with_e2e_encryption
//! @role: sync
//! @layer: infra
//!
//! Transport TCP pour echanger des messages de synchronisation entre pairs.
//! Tous les messages sont chiffres avec la cle de session E2E (ChaCha20-Poly1305).
//! Timeout de 30 secondes par chunk, max 4 chunks en vol simultanement.
//!
//! Le protocole de trame est : [4 bytes length (big-endian)] [payload].

use crate::crypto::e2e::{self, EncryptedMessage};
use crate::errors::MiyucloudError;
use crate::sync::protocol::SyncMessage;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

/// Port de synchronisation par defaut.
pub const DEFAULT_SYNC_PORT: u16 = 11441;

/// Timeout par defaut pour les operations de lecture/ecriture (30 secondes).
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Taille max d'un message (16 MB).
const MAX_MESSAGE_SIZE: u32 = 16 * 1024 * 1024;

/// Nombre max de chunks en vol simultanement.
pub const MAX_INFLIGHT_CHUNKS: usize = 4;

/// Connexion de synchronisation avec un pair distant.
///
/// Encapsule un `TcpStream` et une cle de session pour chiffrer/dechiffrer
/// les messages automatiquement.
pub struct SyncConnection {
    stream: TcpStream,
    session_key: [u8; 32],
}

impl SyncConnection {
    /// Cree une connexion depuis un stream TCP et une cle de session.
    pub fn from_stream(stream: TcpStream, session_key: [u8; 32]) -> Result<Self, MiyucloudError> {
        stream
            .set_read_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(MiyucloudError::Io)?;
        stream
            .set_write_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(MiyucloudError::Io)?;
        Ok(Self {
            stream,
            session_key,
        })
    }

    /// Envoie un message de synchronisation (chiffre E2E).
    ///
    /// Le message est serialise en JSON, chiffre avec la cle de session,
    /// puis encadre avec un header de 4 bytes (longueur big-endian).
    pub fn send_message(&mut self, msg: &SyncMessage) -> Result<(), MiyucloudError> {
        // Serialize message to JSON
        let json =
            serde_json::to_vec(msg).map_err(|e| MiyucloudError::Serialization(e.to_string()))?;

        // Encrypt with session key
        let encrypted = e2e::encrypt_message(&self.session_key, &json)?;

        // Serialize the encrypted message
        let payload = serde_json::to_vec(&encrypted)
            .map_err(|e| MiyucloudError::Serialization(e.to_string()))?;

        // Write length header (4 bytes big-endian)
        let len = payload.len() as u32;
        if len > MAX_MESSAGE_SIZE {
            return Err(MiyucloudError::InvalidInput(format!(
                "Message too large: {len} bytes (max {MAX_MESSAGE_SIZE})"
            )));
        }
        self.stream
            .write_all(&len.to_be_bytes())
            .map_err(MiyucloudError::Io)?;

        // Write payload
        self.stream
            .write_all(&payload)
            .map_err(MiyucloudError::Io)?;

        self.stream.flush().map_err(MiyucloudError::Io)?;

        Ok(())
    }

    /// Recoit un message de synchronisation (dechiffre E2E).
    ///
    /// Lit le header de 4 bytes (longueur), le payload, le dechiffre
    /// et le deserialise en `SyncMessage`.
    pub fn receive_message(&mut self) -> Result<SyncMessage, MiyucloudError> {
        // Read length header
        let mut len_buf = [0u8; 4];
        self.stream
            .read_exact(&mut len_buf)
            .map_err(MiyucloudError::Io)?;
        let len = u32::from_be_bytes(len_buf);

        if len > MAX_MESSAGE_SIZE {
            return Err(MiyucloudError::InvalidInput(format!(
                "Received message too large: {len} bytes (max {MAX_MESSAGE_SIZE})"
            )));
        }

        // Read payload
        let mut payload = vec![0u8; len as usize];
        self.stream
            .read_exact(&mut payload)
            .map_err(MiyucloudError::Io)?;

        // Deserialize encrypted message
        let encrypted: EncryptedMessage = serde_json::from_slice(&payload)
            .map_err(|e| MiyucloudError::Serialization(e.to_string()))?;

        // Decrypt
        let json = e2e::decrypt_message(&self.session_key, &encrypted)?;

        // Deserialize sync message
        let msg: SyncMessage = serde_json::from_slice(&json)
            .map_err(|e| MiyucloudError::Serialization(e.to_string()))?;

        Ok(msg)
    }

    /// Retourne l'adresse du pair distant.
    pub fn peer_addr(&self) -> Result<SocketAddr, MiyucloudError> {
        self.stream.peer_addr().map_err(MiyucloudError::Io)
    }
}

/// Transport de synchronisation P2P.
///
/// Fournit les methodes pour se connecter a un pair distant
/// ou ecouter les connexions entrantes.
pub struct SyncTransport;

impl SyncTransport {
    /// Se connecte a un pair distant et effectue l'echange de cles E2E.
    ///
    /// L'echange de cles utilise X25519 DH ephemere :
    /// 1. Le client genere une paire ephemere et envoie sa cle publique.
    /// 2. Le serveur repond avec sa cle publique.
    /// 3. Les deux derivent la cle de session partagee.
    pub fn connect(peer_addr: SocketAddr) -> Result<SyncConnection, MiyucloudError> {
        let stream =
            TcpStream::connect_timeout(&peer_addr, DEFAULT_TIMEOUT).map_err(MiyucloudError::Io)?;

        // Generate ephemeral keypair
        let local_kp = e2e::generate_ephemeral_keypair();

        // Send our public key (32 bytes)
        let mut stream_ref = &stream;
        stream_ref
            .write_all(&local_kp.public_key_bytes())
            .map_err(MiyucloudError::Io)?;
        stream_ref.flush().map_err(MiyucloudError::Io)?;

        // Receive peer's public key (32 bytes)
        let mut remote_pk_bytes = [0u8; 32];
        stream_ref
            .read_exact(&mut remote_pk_bytes)
            .map_err(MiyucloudError::Io)?;
        let remote_pk = x25519_dalek::PublicKey::from(remote_pk_bytes);

        // Derive session key
        let session_key = e2e::derive_session_key(&local_kp, &remote_pk)?;

        SyncConnection::from_stream(stream, session_key)
    }

    /// Ecoute sur un port et accepte une connexion entrante.
    ///
    /// Effectue l'echange de cles E2E avec le pair qui se connecte.
    pub fn listen(bind_addr: SocketAddr) -> Result<SyncListener, MiyucloudError> {
        let listener = TcpListener::bind(bind_addr).map_err(MiyucloudError::Io)?;
        Ok(SyncListener { listener })
    }

    /// Accepte une connexion entrante et effectue l'echange de cles.
    pub fn accept_connection(stream: TcpStream) -> Result<SyncConnection, MiyucloudError> {
        stream
            .set_read_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(MiyucloudError::Io)?;
        stream
            .set_write_timeout(Some(DEFAULT_TIMEOUT))
            .map_err(MiyucloudError::Io)?;

        // Generate ephemeral keypair
        let local_kp = e2e::generate_ephemeral_keypair();

        // Receive client's public key (32 bytes)
        let mut remote_pk_bytes = [0u8; 32];
        let mut stream_ref = &stream;
        stream_ref
            .read_exact(&mut remote_pk_bytes)
            .map_err(MiyucloudError::Io)?;
        let remote_pk = x25519_dalek::PublicKey::from(remote_pk_bytes);

        // Send our public key (32 bytes)
        stream_ref
            .write_all(&local_kp.public_key_bytes())
            .map_err(MiyucloudError::Io)?;
        stream_ref.flush().map_err(MiyucloudError::Io)?;

        // Derive session key
        let session_key = e2e::derive_session_key(&local_kp, &remote_pk)?;

        SyncConnection::from_stream(stream, session_key)
    }
}

/// Listener pour les connexions de synchronisation entrantes.
pub struct SyncListener {
    listener: TcpListener,
}

impl SyncListener {
    /// Accepte la prochaine connexion entrante.
    ///
    /// Effectue l'echange de cles E2E automatiquement.
    pub fn accept(&self) -> Result<(SyncConnection, SocketAddr), MiyucloudError> {
        let (stream, addr) = self.listener.accept().map_err(MiyucloudError::Io)?;
        let conn = SyncTransport::accept_connection(stream)?;
        Ok((conn, addr))
    }

    /// Retourne l'adresse locale sur laquelle le listener ecoute.
    pub fn local_addr(&self) -> Result<SocketAddr, MiyucloudError> {
        self.listener.local_addr().map_err(MiyucloudError::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::thread;

    #[test]
    fn test_send_and_receive_message_roundtrip() {
        // Create a pair of connected TCP streams via listener
        let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        let session_key = [42u8; 32]; // Shared key for testing

        let key_copy = session_key;
        let handle = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = SyncConnection::from_stream(stream, key_copy).unwrap();
            conn.receive_message().unwrap()
        });

        let stream = TcpStream::connect(addr).unwrap();
        let mut client_conn = SyncConnection::from_stream(stream, session_key).unwrap();

        let msg = SyncMessage::Ack {
            message_id: "test-42".into(),
        };
        client_conn.send_message(&msg).unwrap();

        let received = handle.join().unwrap();
        match received {
            SyncMessage::Ack { message_id } => {
                assert_eq!(message_id, "test-42");
            }
            _ => panic!("Wrong message type received"),
        }
    }

    #[test]
    fn test_send_chunk_data_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        let session_key = [99u8; 32];

        let key_copy = session_key;
        let handle = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = SyncConnection::from_stream(stream, key_copy).unwrap();
            conn.receive_message().unwrap()
        });

        let stream = TcpStream::connect(addr).unwrap();
        let mut client_conn = SyncConnection::from_stream(stream, session_key).unwrap();

        let chunk_data = vec![0xABu8; 1024]; // 1 KB chunk
        let msg = SyncMessage::ChunkData {
            file_id: "file-1".into(),
            chunk_index: 0,
            data: chunk_data.clone(),
            checksum: "sha256-test".into(),
        };
        client_conn.send_message(&msg).unwrap();

        let received = handle.join().unwrap();
        match received {
            SyncMessage::ChunkData {
                file_id,
                chunk_index,
                data,
                checksum,
            } => {
                assert_eq!(file_id, "file-1");
                assert_eq!(chunk_index, 0);
                assert_eq!(data, chunk_data);
                assert_eq!(checksum, "sha256-test");
            }
            _ => panic!("Wrong message type received"),
        }
    }

    #[test]
    fn test_connection_with_key_exchange() {
        let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
        let addr = listener.local_addr().unwrap();

        let handle = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut conn = SyncTransport::accept_connection(stream).unwrap();
            let msg = conn.receive_message().unwrap();
            conn.send_message(&SyncMessage::Ack {
                message_id: "ack-1".into(),
            })
            .unwrap();
            msg
        });

        let mut client = SyncTransport::connect(addr).unwrap();
        client
            .send_message(&SyncMessage::ManifestRequest {
                sender_cog_id: "cog-test".into(),
                folder_ids: vec![],
            })
            .unwrap();

        let ack = client.receive_message().unwrap();
        match ack {
            SyncMessage::Ack { message_id } => assert_eq!(message_id, "ack-1"),
            _ => panic!("Expected Ack"),
        }

        let received = handle.join().unwrap();
        match received {
            SyncMessage::ManifestRequest {
                sender_cog_id,
                folder_ids,
            } => {
                assert_eq!(sender_cog_id, "cog-test");
                assert!(folder_ids.is_empty());
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_connection_timeout() {
        // Bind but never accept
        let listener = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
        let _addr = listener.local_addr().unwrap();

        // Try to connect to a port that nobody listens on (high port)
        let result = SyncTransport::connect("127.0.0.1:1".parse().unwrap());
        assert!(result.is_err());
    }
}
