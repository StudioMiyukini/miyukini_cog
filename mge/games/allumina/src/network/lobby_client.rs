//! Non-blocking TCP client used by the renderer to talk to the lobby server.
//!
//! Unlike [`super::client::run_client`] (which runs a scripted blocking demo
//! loop), this version is built for the interactive GUI:
//! * `connect` performs the TCP dial + Login handshake on the calling thread.
//! * A background reader thread parses framed [`ServerMessage`]s and pushes
//!   them onto an mpsc the renderer drains each frame.
//! * Outbound [`ClientMessage`]s go through `send`, which writes a framed
//!   payload synchronously to the socket.

use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::{Context, Result};

use crate::components::player::ClassType;
use crate::save::SaveData;

use super::protocol::{self, ClientMessage, ServerMessage};
use super::{DEFAULT_PORT, DEFAULT_SERVER};

/// Connection timeout for the initial dial.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(8);

/// Non-blocking lobby/game network client.
///
/// The TCP socket survives across the lobby → in-game transition — the same
/// connection carries `JoinGame` then in-game intents.
pub struct LobbyNetClient {
    /// Writer half of the socket; reader half is owned by the background
    /// thread.
    writer: TcpStream,
    /// Inbound message queue (drained from the renderer thread).
    inbox: Receiver<ServerMessage>,
    /// Joined when the client is dropped to keep the worker tidy.
    reader_handle: Option<JoinHandle<()>>,
}

impl LobbyNetClient {
    /// Dial `host:port` and perform the Login handshake. Returns a ready
    /// client whose inbox already contains the server's first messages
    /// (`LoginAccepted` + an initial `GameList`).
    pub fn connect(
        host: &str,
        port: u16,
        player_id: String,
        character_name: String,
        class: ClassType,
        save_data: Option<SaveData>,
    ) -> Result<Self> {
        let host = if host.trim().is_empty() { DEFAULT_SERVER } else { host };
        let port = if port == 0 { DEFAULT_PORT } else { port };

        let addr = (host, port)
            .to_socket_addrs()
            .with_context(|| format!("resolve {host}:{port}"))?
            .next()
            .ok_or_else(|| anyhow::anyhow!("no address resolved for {host}:{port}"))?;

        let stream = TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT)
            .with_context(|| format!("connect {addr}"))?;
        stream.set_nodelay(true).ok();

        let (tx_in, inbox): (Sender<ServerMessage>, Receiver<ServerMessage>) = mpsc::channel();
        let reader_stream = stream.try_clone().context("clone tcp stream")?;
        let reader_handle = std::thread::spawn(move || reader_thread(reader_stream, tx_in));
        let mut writer = stream;

        // Send the Login frame inline so the renderer can show "connected"
        // immediately. Failures here close the socket and bubble up.
        let login = ClientMessage::Login { player_id, character_name, class, save_data };
        protocol::write_message(&mut writer, &login).context("send login")?;

        Ok(Self {
            writer,
            inbox,
            reader_handle: Some(reader_handle),
        })
    }

    /// Write one frame to the server. Returns Err on a closed socket.
    pub fn send(&mut self, msg: &ClientMessage) -> std::io::Result<()> {
        protocol::write_message(&mut self.writer, msg)
    }

    /// Drain every message the reader has queued. Returns Err if the
    /// reader thread has exited (TCP closed); the caller should drop the
    /// client in that case.
    pub fn drain(&mut self) -> std::result::Result<Vec<ServerMessage>, ()> {
        let mut out = Vec::new();
        loop {
            match self.inbox.try_recv() {
                Ok(msg) => out.push(msg),
                Err(TryRecvError::Empty) => return Ok(out),
                Err(TryRecvError::Disconnected) => {
                    if out.is_empty() {
                        return Err(());
                    }
                    return Ok(out);
                }
            }
        }
    }
}

impl Drop for LobbyNetClient {
    fn drop(&mut self) {
        // Best-effort goodbye + close.
        let _ = protocol::write_message(&mut self.writer, &ClientMessage::Disconnect);
        let _ = self.writer.shutdown(std::net::Shutdown::Both);
        if let Some(h) = self.reader_handle.take() {
            let _ = h.join();
        }
    }
}

fn reader_thread(mut stream: TcpStream, tx: Sender<ServerMessage>) {
    let mut buf: Vec<u8> = Vec::with_capacity(8192);
    let mut chunk = [0u8; 4096];
    loop {
        loop {
            match protocol::try_unframe::<ServerMessage>(&mut buf) {
                Ok(Some(msg)) => {
                    if tx.send(msg).is_err() {
                        return;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!("[lobby-client] decode error: {e}");
                    return;
                }
            }
        }
        match stream.read(&mut chunk) {
            Ok(0) => return,
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
            Err(_) => return,
        }
    }
}
