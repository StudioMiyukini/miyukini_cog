//! WebSocket ↔ TCP bridge for the browser client (`--features webclient`).
//!
//! The authoritative server speaks length-prefixed **bincode** over TCP
//! (port 7777). Browsers can only open **WebSocket** connections and JS only
//! conveniently speaks **JSON**. This bridge sits in the middle:
//!
//! ```text
//!   browser ──wss://host/ (JSON)──▶ ws_bridge ──tcp://127.0.0.1:7777 (bincode)──▶ server
//! ```
//!
//! * The bridge serves the static `web/` client **and** the browser WebSocket
//!   from a single port ([`WEB_PORT`]), telling them apart by the request's
//!   `Upgrade` header. A single port is what lets one Cloudflare tunnel hostname
//!   (`allumina.miyukini-home.org` → `http://localhost:7000`) reach both, so the
//!   browser connects to `wss://<same-host>/` with no extra port.
//! * One WebSocket connection ⇒ one dedicated TCP session to the server, so the
//!   server treats each browser exactly like a native client.
//! * Translation is *generic*: we round-trip through [`serde_json::Value`] and
//!   the real [`ClientMessage`]/[`ServerMessage`] types, so every protocol
//!   variant is supported without per-variant glue. Serde's external tagging
//!   (`{"Login": {...}}`) is flattened to the browser-friendly internal tag
//!   (`{"type": "Login", ...}`) and back.
//!
//! Everything is `std::net` + `std::thread`; the only extra dependency is
//! `tungstenite` for the WebSocket handshake/framing.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use serde_json::{Map, Value};
use tungstenite::Message;

use super::protocol::{self, ClientMessage, ServerMessage};

/// Default port the bridge serves **both** the static `web/` files and the
/// browser WebSocket on. A single port is required so one Cloudflare tunnel
/// hostname (`allumina.miyukini-home.org` → `http://localhost:7000`) reaches the
/// page *and* the realtime socket — HTTP and WebSocket are told apart per
/// connection by the request's `Upgrade` header.
pub const WEB_PORT: u16 = 7000;
/// How long a socket read blocks before we poll the other direction.
const POLL: Duration = Duration::from_millis(5);
/// Directory holding the browser client, resolved at compile time.
const WEB_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/web");

/// Launch the bridge on a single `web_port`, serving the static `web/` files and
/// browser WebSocket sessions from the same listener (so one Cloudflare tunnel
/// hostname covers both). Each accepted socket gets its own thread and is routed
/// by [`dispatch`] based on whether it's a WebSocket upgrade.
///
/// `tcp_port` is the authoritative server's TCP port (the bridge always dials
/// `127.0.0.1`, since it is meant to run on the same host as `--server`).
pub fn run(web_port: u16, tcp_port: u16) -> ! {
    let listener = TcpListener::bind(("0.0.0.0", web_port))
        .unwrap_or_else(|e| panic!("[ws-bridge] impossible d'écouter sur {web_port}: {e}"));
    println!("[ws-bridge] HTTP + WebSocket sur :{web_port}/ → tcp 127.0.0.1:{tcp_port} ({WEB_DIR})");

    for stream in listener.incoming() {
        match stream {
            Ok(sock) => {
                thread::spawn(move || dispatch(sock, tcp_port));
            }
            Err(e) => eprintln!("[ws-bridge] accept: {e}"),
        }
    }
    // `incoming()` only ends on listener teardown; keep the `!` contract.
    std::process::exit(0);
}

/// Route one freshly-accepted connection: peek the request head (without
/// consuming it, so the WebSocket handshake can re-read it) and send a
/// `Upgrade: websocket` request to the bridge, everything else to the static
/// file server.
fn dispatch(sock: TcpStream, tcp_port: u16) {
    // Bound the wait so a silent/half-open connection can't pin a thread.
    sock.set_read_timeout(Some(Duration::from_secs(10))).ok();
    let mut buf = [0u8; 4096];
    let n = match sock.peek(&mut buf) {
        Ok(0) | Err(_) => return,
        Ok(n) => n,
    };
    let head = String::from_utf8_lossy(&buf[..n]).to_ascii_lowercase();
    // Hand the socket back to a blocking state for the handshake / file read.
    sock.set_read_timeout(None).ok();

    if head.contains("upgrade: websocket") {
        if let Err(e) = bridge_connection(sock, tcp_port) {
            eprintln!("[ws-bridge] session terminée: {e}");
        }
    } else {
        handle_http(sock);
    }
}

/// Drive a single browser session: WebSocket handshake, open the backing TCP
/// session, then pump both directions until either side closes.
fn bridge_connection(sock: TcpStream, tcp_port: u16) -> Result<(), String> {
    let peer = sock.peer_addr().map(|a| a.to_string()).unwrap_or_default();
    let mut ws = tungstenite::accept(sock).map_err(|e| format!("handshake {peer}: {e}"))?;

    // Backing TCP session to the authoritative server.
    let mut tcp = TcpStream::connect(("127.0.0.1", tcp_port))
        .map_err(|e| format!("connexion serveur tcp:{tcp_port}: {e}"))?;
    tcp.set_read_timeout(Some(POLL)).ok();
    tcp.set_nodelay(true).ok();
    println!("[ws-bridge] {peer} ↔ serveur");

    // WebSocket reads must not block the TCP→browser pump, so make the WS side
    // non-blocking and poll it. Writes stay buffered by tungstenite.
    ws.get_ref().set_nonblocking(true).ok();

    let mut tcp_buf: Vec<u8> = Vec::with_capacity(8192);
    let mut scratch = [0u8; 8192];

    loop {
        let mut idle = true;

        // ── browser → server ────────────────────────────────────────────
        match ws.read() {
            Ok(Message::Text(text)) => {
                idle = false;
                if let Some(msg) = json_to_client(&text) {
                    let frame = protocol::frame(&msg).map_err(|e| format!("encode: {e}"))?;
                    tcp.write_all(&frame).map_err(|e| format!("write tcp: {e}"))?;
                } else {
                    eprintln!("[ws-bridge] message client ignoré: {text}");
                }
            }
            Ok(Message::Binary(_)) | Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => idle = false,
            Ok(Message::Close(_)) => return Ok(()),
            Ok(Message::Frame(_)) => {}
            Err(tungstenite::Error::Io(e))
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(tungstenite::Error::ConnectionClosed)
            | Err(tungstenite::Error::AlreadyClosed) => return Ok(()),
            Err(e) => return Err(format!("ws read: {e}")),
        }

        // ── server → browser ────────────────────────────────────────────
        match tcp.read(&mut scratch) {
            Ok(0) => return Ok(()), // server closed
            Ok(n) => {
                idle = false;
                tcp_buf.extend_from_slice(&scratch[..n]);
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(e) => return Err(format!("read tcp: {e}")),
        }
        loop {
            match protocol::try_unframe::<ServerMessage>(&mut tcp_buf) {
                Ok(Some(msg)) => {
                    idle = false;
                    let json = server_to_json(&msg);
                    ws_send(&mut ws, Message::Text(json))?;
                }
                Ok(None) => break,
                Err(e) => return Err(format!("decode tcp: {e}")),
            }
        }

        // Flush any queued WS writes (also drains auto-pong replies).
        match ws.flush() {
            Ok(()) => {}
            Err(tungstenite::Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(tungstenite::Error::ConnectionClosed)
            | Err(tungstenite::Error::AlreadyClosed) => return Ok(()),
            Err(e) => return Err(format!("ws flush: {e}")),
        }

        if idle {
            thread::sleep(Duration::from_millis(2));
        }
    }
}

/// Enqueue a WebSocket message and flush it, tolerating non-blocking back-pressure.
fn ws_send(ws: &mut tungstenite::WebSocket<TcpStream>, msg: Message) -> Result<(), String> {
    match ws.write(msg) {
        Ok(()) => {}
        Err(tungstenite::Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {}
        Err(tungstenite::Error::ConnectionClosed) | Err(tungstenite::Error::AlreadyClosed) => {
            return Err("ws closed".into())
        }
        Err(e) => return Err(format!("ws write: {e}")),
    }
    Ok(())
}

// ── JSON ⇆ wire-type translation ─────────────────────────────────────────────

/// Browser JSON (`{"type":"Foo", ...}`) → [`ClientMessage`].
///
/// We rebuild serde's external tagging (`{"Foo": {...}}`) — or the bare variant
/// string for unit variants — then let serde do the real validation.
fn json_to_client(text: &str) -> Option<ClientMessage> {
    let value: Value = serde_json::from_str(text).ok()?;
    let obj = value.as_object()?;
    let tag = obj.get("type")?.as_str()?.to_string();

    let mut rest = obj.clone();
    rest.remove("type");

    // The browser sends `difficulty` as a 0/1/2 number; serde wants the variant
    // name. Map it here so the JS stays close to the documented protocol.
    if tag == "CreateGame" {
        if let Some(d) = rest.get("difficulty").and_then(Value::as_u64) {
            let name = match d {
                1 => "Cauchemar",
                2 => "Enfer",
                _ => "Normal",
            };
            rest.insert("difficulty".to_string(), Value::String(name.to_string()));
        }
    }

    let tagged = if rest.is_empty() {
        Value::String(tag)
    } else {
        let mut m = Map::new();
        m.insert(tag, Value::Object(rest));
        Value::Object(m)
    };
    serde_json::from_value(tagged).ok()
}

/// [`ServerMessage`] → browser JSON string (`{"type":"Foo", ...}`).
fn server_to_json(msg: &ServerMessage) -> String {
    let tagged = serde_json::to_value(msg).unwrap_or(Value::Null);
    serde_json::to_string(&flatten_tagged(tagged)).unwrap_or_else(|_| "{}".to_string())
}

/// Flatten serde's external enum tagging into an internally-tagged object:
/// `{"Foo": {a,b}}` → `{"type":"Foo", a, b}`; bare `"Foo"` → `{"type":"Foo"}`.
fn flatten_tagged(v: Value) -> Value {
    match v {
        Value::String(s) => {
            let mut m = Map::new();
            m.insert("type".to_string(), Value::String(s));
            Value::Object(m)
        }
        Value::Object(map) if map.len() == 1 => {
            let (tag, inner) = map.into_iter().next().expect("len == 1");
            match inner {
                Value::Object(mut fields) => {
                    fields.insert("type".to_string(), Value::String(tag));
                    Value::Object(fields)
                }
                other => {
                    let mut m = Map::new();
                    m.insert("type".to_string(), Value::String(tag));
                    m.insert("value".to_string(), other);
                    Value::Object(m)
                }
            }
        }
        other => other,
    }
}

// ── Minimal static HTTP server (bonus) ───────────────────────────────────────

/// Serve files from [`WEB_DIR`] forever. One thread per request.
/// Serve one static-file request from `web/` (the connection was already
/// routed here by [`dispatch`] as a non-WebSocket request).
fn handle_http(mut stream: TcpStream) {
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
    let mut buf = [0u8; 2048];
    let Ok(n) = stream.read(&mut buf) else { return };
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    // Strip query string and resolve to a file under WEB_DIR, refusing escapes.
    let rel = path.split('?').next().unwrap_or("/");
    let rel = if rel == "/" { "/index.html" } else { rel };
    if rel.contains("..") {
        let _ = http_respond(&mut stream, 400, "text/plain", b"bad request");
        return;
    }
    let full = format!("{WEB_DIR}{rel}");

    match std::fs::read(&full) {
        Ok(body) => {
            let _ = http_respond(&mut stream, 200, content_type(rel), &body);
        }
        Err(_) => {
            let _ = http_respond(&mut stream, 404, "text/plain", b"404 not found");
        }
    }
}

fn http_respond(stream: &mut TcpStream, code: u16, ctype: &str, body: &[u8]) -> std::io::Result<()> {
    let reason = if code == 200 { "OK" } else { "ERR" };
    let header = format!(
        "HTTP/1.1 {code} {reason}\r\nContent-Type: {ctype}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-cache\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body)
}

fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;

    #[test]
    fn login_json_to_client() {
        let text =
            r#"{"type":"Login","player_id":"u1","character_name":"Hero","class":"Soldat"}"#;
        let msg = json_to_client(text).expect("parsed");
        assert_eq!(
            msg,
            ClientMessage::Login {
                player_id: "u1".into(),
                character_name: "Hero".into(),
                class: ClassType::Soldat,
                save_data: None,
            }
        );
    }

    #[test]
    fn unit_variant_json_to_client() {
        assert_eq!(json_to_client(r#"{"type":"ListGames"}"#), Some(ClientMessage::ListGames));
    }

    #[test]
    fn create_game_difficulty_number_is_mapped() {
        let text = r#"{"type":"CreateGame","name":"X","max_players":4,"difficulty":2}"#;
        match json_to_client(text) {
            Some(ClientMessage::CreateGame { difficulty, .. }) => {
                assert_eq!(difficulty, protocol::Difficulty::Enfer);
            }
            other => panic!("expected CreateGame, got {other:?}"),
        }
    }

    #[test]
    fn move_intent_round_trip() {
        let text = r#"{"type":"MoveIntent","target_x":3.5,"target_y":-2.0}"#;
        assert_eq!(
            json_to_client(text),
            Some(ClientMessage::MoveIntent { target_x: 3.5, target_y: -2.0 })
        );
    }

    #[test]
    fn server_struct_variant_is_flattened() {
        let json = server_to_json(&ServerMessage::LoginAccepted { your_entity_id: 7, tick: 3 });
        let v: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["type"], "LoginAccepted");
        assert_eq!(v["your_entity_id"], 7);
        assert_eq!(v["tick"], 3);
    }

    #[test]
    fn server_unit_variant_is_flattened() {
        let json = server_to_json(&ServerMessage::ReturnedToLobby);
        let v: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["type"], "ReturnedToLobby");
    }

    #[test]
    fn damage_dealt_flattened() {
        let json = server_to_json(&ServerMessage::DamageDealt { target: 5, amount: 42.5, crit: true });
        let v: Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["type"], "DamageDealt");
        assert_eq!(v["target"], 5);
        assert_eq!(v["crit"], true);
    }
}
