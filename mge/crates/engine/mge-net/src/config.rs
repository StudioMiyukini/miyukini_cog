// @id: MGE-Net-Config @do: configuration @role: back-end @layer: 2 @human: miyuk
//! Server configuration and connection metadata.

use crate::client_id::ClientId;

/// Network server configuration.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Address to bind, e.g. `"0.0.0.0:7777"`.
    pub bind_addr: String,
    /// Maximum number of simultaneous clients.
    pub max_clients: usize,
    /// Maximum size of a single message payload in bytes.
    pub max_message_size: usize,
    /// Server tick rate in ticks per second.
    pub tick_rate: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:7777".to_owned(),
            max_clients: 200,
            max_message_size: 1_048_576,
            tick_rate: 25,
        }
    }
}

/// Metadata about an active connection.
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Unique client identifier.
    pub client_id: ClientId,
    /// Remote peer address as a string.
    pub remote_addr: String,
    /// Unix timestamp (milliseconds) when the connection was established.
    pub connected_at_ms: u64,
}
