// @id: MGE-Net @do: engine @role: back-end @layer: 2 @human: miyuk
//! # mge-net
//!
//! Networking primitives for the Miyukini Game Engine (Sodomight multiplayer).
//!
//! This crate provides:
//! - [`ClientId`] -- unique client identity (UUID v4)
//! - [`Packet`] -- generic envelope wrapping any serializable payload
//! - [`GameMessage`] -- enum of all client/server game messages
//! - [`FrameCodec`] -- length-prefixed (u32 LE) frame encoder/decoder
//! - [`ServerConfig`] / [`ConnectionInfo`] -- server and connection metadata
//! - [`NetError`] / [`NetResult`] -- error types

pub mod client_id;
pub mod codec;
pub mod config;
pub mod packet;

mod tests;

// Re-exports for ergonomic usage.
pub use client_id::ClientId;
pub use codec::FrameCodec;
pub use config::{ConnectionInfo, ServerConfig};
pub use packet::{GameMessage, Packet};

/// Network error type for mge-net.
#[derive(Debug, thiserror::Error)]
pub enum NetError {
    /// An I/O operation failed.
    #[error("IO error: {message}")]
    Io {
        /// Human-readable description of the I/O failure.
        message: String,
    },

    /// Serialization or deserialization failed.
    #[error("Serialization error: {message}")]
    Serialization {
        /// Human-readable description of the serialization failure.
        message: String,
    },

    /// The remote end closed the connection.
    #[error("Connection closed")]
    ConnectionClosed,

    /// The declared message size exceeds the configured maximum.
    #[error("Message too large: {size} bytes (max {max})")]
    MessageTooLarge {
        /// Declared payload size in bytes.
        size: usize,
        /// Configured maximum in bytes.
        max: usize,
    },

    /// The packet data is malformed.
    #[error("Invalid packet: {message}")]
    InvalidPacket {
        /// Human-readable description of what is wrong.
        message: String,
    },
}

/// Convenience type alias.
pub type NetResult<T> = Result<T, NetError>;
