<!-- @id: SD-Impl-06 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-06 -- Reseau : Listen Server, Protocol & Lobby

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-Architecture.md section 7/8 (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

---

## Table des matieres

1. [Architecture reseau MVP](#1-architecture-reseau-mvp)
2. [Decoupe en crates](#2-decoupe-en-crates)
3. [Crate mge-net (Couche 2 Engine)](#3-crate-mge-net-couche-2-engine)
4. [Crate sd-lobby (Couche 4 Game)](#4-crate-sd-lobby-couche-4-game)
5. [Protocole de messages -- mge-net](#5-protocole-de-messages--mge-net)
6. [ServerState et gestion des sessions](#6-serverstate-et-gestion-des-sessions)
7. [Handlers REST -- Auth & Lobby (axum)](#7-handlers-rest--auth--lobby-axum)
8. [Transport gameplay -- TCP bincode](#8-transport-gameplay--tcp-bincode)
9. [Game loop serveur (25 TPS)](#9-game-loop-serveur-25-tps)
10. [Anti-cheat basique](#10-anti-cheat-basique)
11. [Integration Listen Server dans sodomight-game](#11-integration-listen-server-dans-sodomight-game)
12. [Delta compression](#12-delta-compression)
13. [Tests](#13-tests)
14. [Checklist integration](#14-checklist-integration)
15. [Migration Phase 2 -- Serveur dedie](#15-migration-phase-2--serveur-dedie)

---

## 1. Architecture reseau MVP

Sodomight utilise un modele **Listen Server** pour la Phase 1 : le joueur hote execute
a la fois le client et le serveur dans le meme processus. Pas de serveur dedie pour le MVP.

```
   ┌───────────────────────────────────────────────────────────┐
   │                sodomight-game (Hote)                      │
   │  ┌──────────────┐    ┌────────────────────────────────┐   │
   │  │  Game Loop    │<-->│  Listen Server                 │   │
   │  │  ECS World    │    │  ┌──────────┐  ┌────────────┐ │   │
   │  │  Render + UI  │    │  │ sd-lobby │  │  mge-net   │ │   │
   │  │  Audio        │    │  │ axum REST│  │  TCP 25 Hz │ │   │
   │  └──────────────┘    │  └──────────┘  └────────────┘ │   │
   │                       └────────────────────────────────┘   │
   │           |                      |                         │
   │     loopback TCP            TCP accept                     │
   │     (127.0.0.1)          (0.0.0.0:7777)                   │
   └───────────────────────────────────────────────────────────┘
          |                           |
     sodomight-client           sodomight-client
       (Invite 1)                (Invite 2-7)
```

**Protocole dual :**

| Canal          | Transport     | Serialisation | Usage                              |
|----------------|---------------|---------------|------------------------------------|
| REST API       | HTTP (axum)   | JSON          | Auth, lobby, liste parties, meta   |
| Gameplay temps reel | TCP (tokio) | bincode       | Inputs, snapshots, combat, loot    |

**Justification du dual protocol :**
- axum/JSON pour le lobby : standard REST, debug facile, pas de contrainte de latence.
- TCP/bincode pour le gameplay : compact (~10x plus petit que JSON), rapide a encoder/decoder, suffisant pour un ARPG a 25 Hz (pas de FPS).

**Contraintes (Denis, SD-Tech-Architecture) :**
- Max 8 joueurs par partie
- Modele client-serveur autoritaire (host = serveur)
- Loot partage dans le monde, timer priorite 30s pour le killer
- Tick rate 25 Hz (40ms, standard D2)
- Network tick synchronise au fixed tick (25 Hz)

---

## 2. Decoupe en crates

L'architecture reseau se repartit sur deux couches du workspace :

```
Couche 2 (Engine) :
  mge-net           -- transport TCP, serialisation bincode, frame protocol
                       generique et reutilisable par tout jeu MGE

Couche 4 (Game) :
  sd-lobby          -- nouveau crate : auth REST, gestion lobbies axum
                       specifique Sodomight
  sodomight-game    -- integration listen server (init, game loop)
  sodomight-server  -- binaire serveur dedie Phase 2
  sodomight-client  -- binaire client Phase 2
```

La crate `mge-net` ne connait PAS le contenu Sodomight. Elle fournit :
- Un frame protocol TCP (length-prefixed frames)
- La serialisation/deserialisation bincode generique
- Un `ConnectionPool` pour gerer N clients

La crate `sd-lobby` contient la logique specifique Sodomight :
- Auth (bcrypt, tokens session)
- Gestion lobbies (creation, join, liste)
- API REST axum

Les types de messages (`ClientMessage`, `ServerMessage`) sont definis dans `mge-arpg-entity`
ou directement dans `sodomight-game` car ils referencent des types specifiques au jeu
(`SkillId`, `EntityId`, `EquipSlot`...).

---

## 3. Crate mge-net (Couche 2 Engine)

### Cargo.toml

```toml
# crates/engine/mge-net/Cargo.toml
[package]
name = "mge-net"
version.workspace = true
edition.workspace = true
description = "MGE networking: TCP listen server, frame protocol, bincode serialization"

[dependencies]
mge-core = { path = "../../kernel/mge-core" }
tokio = { workspace = true }
bincode = { workspace = true }
serde = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
tokio = { workspace = true, features = ["test-util"] }

[lints]
workspace = true
```

### Structure

```
crates/engine/mge-net/src/
├── lib.rs           -- pub use, NetError
├── frame.rs         -- length-prefixed frame codec (read/write)
├── server.rs        -- TcpListener accept loop, ConnectionPool
├── client.rs        -- TcpStream connect, send/recv
├── connection.rs    -- Connection struct (reader + writer halves)
└── types.rs         -- NetworkMode, ConnectionId, MAX_FRAME_SIZE
```

### NetError

```rust
// crates/engine/mge-net/src/lib.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Bincode(#[from] bincode::Error),
    #[error("Frame too large: {size} > {max}")]
    FrameTooLarge { size: u32, max: u32 },
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Connection timeout")]
    Timeout,
    #[error("Max clients reached: {0}")]
    MaxClients(usize),
}

pub type NetResult<T> = Result<T, NetError>;

pub mod frame;
pub mod server;
pub mod client;
pub mod connection;
pub mod types;

pub use connection::Connection;
pub use server::NetServer;
pub use client::NetClient;
pub use types::{ConnectionId, NetworkMode};
```

### Frame protocol

Chaque message TCP est precede d'un header 4 octets (u32 little-endian) indiquant la
taille du payload bincode. Taille max : 64 KiB par frame.

```rust
// crates/engine/mge-net/src/frame.rs

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::{NetError, NetResult};

/// Taille maximale d'une frame (64 KiB)
pub const MAX_FRAME_SIZE: u32 = 65_536;

/// Ecrit une frame : [u32 LE length][payload]
pub async fn write_frame<W: AsyncWriteExt + Unpin>(
    writer: &mut W,
    payload: &[u8],
) -> NetResult<()> {
    let len = u32::try_from(payload.len())
        .map_err(|_| NetError::FrameTooLarge { size: 0, max: MAX_FRAME_SIZE })?;
    if len > MAX_FRAME_SIZE {
        return Err(NetError::FrameTooLarge { size: len, max: MAX_FRAME_SIZE });
    }
    writer.write_all(&len.to_le_bytes()).await?;
    writer.write_all(payload).await?;
    writer.flush().await?;
    Ok(())
}

/// Lit une frame. Retourne le payload deserialise.
pub async fn read_frame<R: AsyncReadExt + Unpin>(
    reader: &mut R,
) -> NetResult<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                NetError::ConnectionClosed
            } else {
                NetError::Io(e)
            }
        })?;
    let len = u32::from_le_bytes(len_buf);
    if len > MAX_FRAME_SIZE {
        return Err(NetError::FrameTooLarge { size: len, max: MAX_FRAME_SIZE });
    }
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf).await?;
    Ok(buf)
}
```

### Connection

```rust
// crates/engine/mge-net/src/connection.rs

use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use serde::{Serialize, de::DeserializeOwned};
use crate::{frame, NetResult};
use crate::types::ConnectionId;

/// Une connexion TCP bidirectionnelle vers un client.
pub struct Connection {
    pub id: ConnectionId,
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
}

impl Connection {
    pub fn new(id: ConnectionId, reader: OwnedReadHalf, writer: OwnedWriteHalf) -> Self {
        Self { id, reader, writer }
    }

    /// Envoie un message serialise en bincode.
    pub async fn send<T: Serialize>(&mut self, msg: &T) -> NetResult<()> {
        let payload = bincode::serialize(msg)?;
        frame::write_frame(&mut self.writer, &payload).await
    }

    /// Recoit et deserialise un message bincode.
    pub async fn recv<T: DeserializeOwned>(&mut self) -> NetResult<T> {
        let payload = frame::read_frame(&mut self.reader).await?;
        let msg = bincode::deserialize(&payload)?;
        Ok(msg)
    }

    /// Separe la connexion en lecteur et ecrivain pour tokio::select!
    pub fn split(self) -> (ConnectionReader, ConnectionWriter) {
        (
            ConnectionReader { id: self.id, reader: self.reader },
            ConnectionWriter { id: self.id, writer: self.writer },
        )
    }
}

pub struct ConnectionReader {
    pub id: ConnectionId,
    reader: OwnedReadHalf,
}

impl ConnectionReader {
    pub async fn recv<T: DeserializeOwned>(&mut self) -> NetResult<T> {
        let payload = frame::read_frame(&mut self.reader).await?;
        let msg = bincode::deserialize(&payload)?;
        Ok(msg)
    }
}

pub struct ConnectionWriter {
    pub id: ConnectionId,
    writer: OwnedWriteHalf,
}

impl ConnectionWriter {
    pub async fn send<T: Serialize>(&mut self, msg: &T) -> NetResult<()> {
        let payload = bincode::serialize(msg)?;
        frame::write_frame(&mut self.writer, &payload).await
    }
}
```

### NetServer

```rust
// crates/engine/mge-net/src/server.rs

use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::{NetError, NetResult, Connection};
use crate::types::ConnectionId;

/// Serveur TCP pour le listen server.
pub struct NetServer {
    listener: TcpListener,
    max_clients: usize,
    next_id: u32,
}

impl NetServer {
    /// Bind le serveur sur l'adresse donnee.
    pub async fn bind(addr: &str, max_clients: usize) -> NetResult<Self> {
        let listener = TcpListener::bind(addr).await?;
        tracing::info!("mge-net server listening on {}", addr);
        Ok(Self {
            listener,
            max_clients,
            next_id: 1,
        })
    }

    /// Accepte une nouvelle connexion.
    /// Retourne None si le serveur est plein.
    pub async fn accept(&mut self, current_count: usize) -> NetResult<Connection> {
        if current_count >= self.max_clients {
            return Err(NetError::MaxClients(self.max_clients));
        }
        let (stream, addr) = self.listener.accept().await?;
        stream.set_nodelay(true)?;
        let id = ConnectionId(self.next_id);
        self.next_id += 1;
        tracing::info!("Client {} connected from {}", id.0, addr);
        let (reader, writer) = stream.into_split();
        Ok(Connection::new(id, reader, writer))
    }

    pub fn local_addr(&self) -> NetResult<SocketAddr> {
        self.listener.local_addr().map_err(NetError::Io)
    }
}
```

### NetClient

```rust
// crates/engine/mge-net/src/client.rs

use tokio::net::TcpStream;
use crate::{NetResult, Connection};
use crate::types::ConnectionId;

/// Client TCP pour se connecter a un listen server.
pub struct NetClient;

impl NetClient {
    /// Se connecte a un serveur.
    pub async fn connect(addr: &str) -> NetResult<Connection> {
        let stream = TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;
        tracing::info!("Connected to server at {}", addr);
        let (reader, writer) = stream.into_split();
        Ok(Connection::new(ConnectionId(0), reader, writer))
    }
}
```

### Types

```rust
// crates/engine/mge-net/src/types.rs

/// Identifiant de connexion attribue par le serveur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(pub u32);

/// Mode reseau pour le lancement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMode {
    /// Pas de reseau, partie solo
    Offline,
    /// Le joueur hote est a la fois client et serveur
    ListenServer,
    /// Connexion a un serveur distant
    Client,
    /// Serveur dedie sans rendu (Phase 2)
    DedicatedServer,
}
```

---

## 4. Crate sd-lobby (Couche 4 Game)

### Cargo.toml

```toml
# games/sd-lobby/Cargo.toml
[package]
name = "sd-lobby"
version.workspace = true
edition.workspace = true
description = "Sodomight lobby server: REST API for auth, lobby management"

[dependencies]
sd-persistence = { path = "../sd-persistence" }
tokio = { workspace = true }
axum = { version = "0.8", features = ["ws"] }
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
bcrypt = "0.16"
tower-http = { version = "0.6", features = ["cors"] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
axum-test = "16"

[lints]
workspace = true
```

### Structure

```
games/sd-lobby/src/
├── lib.rs           -- pub use, LobbyError, build_router()
├── state.rs         -- LobbyState (DbPool, sessions, lobbies)
├── auth.rs          -- POST /auth/register, POST /auth/login
├── lobby.rs         -- GET /lobbies, POST /lobbies, POST /lobbies/{id}/join
├── character.rs     -- GET /characters, POST /characters
└── extractors.rs    -- AuthToken extractor middleware
```

---

## 5. Protocole de messages -- mge-net

Les types de messages sont definis dans `sodomight-game` car ils referencent des types
specifiques au jeu. Ils sont serialises avec bincode pour le transport TCP.

### ClientMessage

```rust
// games/sodomight/src/net/messages.rs

use serde::{Deserialize, Serialize};

/// ID d'entite dans le monde ECS (u32 compact pour le reseau)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetEntityId(pub u32);

/// ID de skill (reference la definition TOML)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillId(pub String);

/// ID d'item (UUID v4 persistant)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetItemId(pub String);

/// ID de zone
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ZoneId(pub String);

/// Cible d'un skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillTarget {
    Entity(NetEntityId),
    Position { x: f32, y: f32 },
    Self_,
    None,
}

/// Slot d'equipement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipSlot {
    Head,
    Chest,
    Gloves,
    Belt,
    Boots,
    MainHand,
    OffHand,
    Amulet,
    RingLeft,
    RingRight,
}

/// Messages envoyes par le CLIENT vers le SERVEUR (host).
/// Serialises en bincode via mge-net frame protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // --- Mouvement ---
    /// Le joueur clique pour se deplacer vers (x, y)
    MoveToPosition { x: f32, y: f32 },
    /// Le joueur clique sur une entite pour s'en approcher
    MoveToEntity { target: NetEntityId },
    /// Arret du mouvement
    StopMoving,

    // --- Combat ---
    /// Utilise une competence sur une cible
    UseSkill { skill_id: SkillId, target: SkillTarget },
    /// Attaque normale sur une cible
    NormalAttack { target: NetEntityId },
    /// Swap weapon set (I/II)
    SwapWeaponSet,

    // --- Items ---
    /// Ramasse un item au sol
    PickupItem { entity_id: NetEntityId },
    /// Depose un item au sol
    DropItem { item_id: NetItemId, x: f32, y: f32 },
    /// Equipe un item
    EquipItem { item_id: NetItemId, slot: EquipSlot },
    /// Desequipe un slot
    UnequipItem { slot: EquipSlot },
    /// Utilise une potion du belt
    UsePotion { belt_slot: u8 },
    /// Identifie un item
    IdentifyItem { item_id: NetItemId },

    // --- Trade ---
    /// Ouvre le trade avec un joueur
    OpenTrade { target_player: NetEntityId },
    /// Ajoute un item au trade
    AddToTrade { item_id: NetItemId },
    /// Retire un item du trade
    RemoveFromTrade { item_id: NetItemId },
    /// Definit le montant d'or dans le trade
    SetTradeGold { amount: u32 },
    /// Accepte le trade
    AcceptTrade,
    /// Annule le trade
    CancelTrade,

    // --- Cube Alchimique ---
    TransmuteCube,

    // --- World ---
    /// Utilise un waypoint
    UseWaypoint { zone_id: ZoneId },
    /// Entre dans un portail
    EnterPortal { portal_id: NetEntityId },
    /// Cast Town Portal
    CastTownPortal,
    /// Interagit avec un NPC
    InteractNpc { npc_id: NetEntityId },

    // --- Quetes ---
    AcceptQuest { quest_id: String },
    CompleteQuest { quest_id: String },

    // --- Chat ---
    ChatMessage { channel: ChatChannel, text: String },

    // --- Keepalive ---
    Ping { seq: u32 },

    // --- Session ---
    SaveAndQuit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatChannel {
    Global,
    Party,
    Whisper { target: String },
}
```

### ServerMessage

```rust
// games/sodomight/src/net/messages.rs (suite)

/// Type de degat pour l'affichage (couleur du nombre)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Magic,
}

/// Snapshot d'une entite pour le FullStateSync ou EntitySpawned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub net_id: NetEntityId,
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub vel_y: f32,
    pub current_life: i32,
    pub max_life: i32,
    pub level: i32,
    pub class_or_kind: String,     // "necromancer" | "zombie" | "fallen" | "andariel"
    pub animation_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Monster,
    Npc,
    Mercenary,
    Projectile,
    Shrine,
    Portal,
}

/// Snapshot d'item au sol pour l'affichage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSnapshot {
    pub item_id: NetItemId,
    pub base_item_id: String,
    pub quality: String,           // "normal" | "magic" | "rare" | "unique" | "set" | "rune_word"
    pub display_name: String,
    pub item_level: u32,
}

/// Snapshot complet du monde pour la connexion initiale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub zone_id: ZoneId,
    pub entities: Vec<EntitySnapshot>,
    pub ground_items: Vec<GroundItemSnapshot>,
    pub difficulty: String,
    pub act: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundItemSnapshot {
    pub item: ItemSnapshot,
    pub x: f32,
    pub y: f32,
    pub priority_player: Option<NetEntityId>,
    pub timer_remaining_ms: u32,
}

/// Messages envoyes par le SERVEUR vers les CLIENTS.
/// Serialises en bincode via mge-net frame protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // --- Connexion ---
    /// Envoye au client juste apres connexion
    Welcome {
        your_entity_id: NetEntityId,
        world: WorldSnapshot,
    },
    /// Synchronisation complete (resync apres zone change)
    FullStateSync { world: WorldSnapshot },

    // --- Entites ---
    EntitySpawned { entity: EntitySnapshot },
    EntityDespawned { id: NetEntityId },
    EntityMoved { id: NetEntityId, x: f32, y: f32, vel_x: f32, vel_y: f32 },
    EntityDied { id: NetEntityId, killer: Option<NetEntityId> },
    EntityHealthChanged { id: NetEntityId, current: i32, max: i32 },
    EntityManaChanged { id: NetEntityId, current: i32, max: i32 },
    EntityAnimationChanged { id: NetEntityId, animation_id: String },

    // --- Combat ---
    DamageDealt {
        source: NetEntityId,
        target: NetEntityId,
        amount: i32,
        damage_type: DamageType,
        is_critical: bool,
    },
    SkillActivated {
        source: NetEntityId,
        skill_id: SkillId,
        target: SkillTarget,
    },
    ProjectileSpawned {
        id: NetEntityId,
        source: NetEntityId,
        skill_id: SkillId,
        x: f32,
        y: f32,
        vel_x: f32,
        vel_y: f32,
    },
    StatusApplied {
        target: NetEntityId,
        status_type: String,
        duration_ms: u32,
    },
    StatusRemoved {
        target: NetEntityId,
        status_type: String,
    },

    // --- Items ---
    ItemDropped {
        item: ItemSnapshot,
        x: f32,
        y: f32,
        priority_player: Option<NetEntityId>,
        timer_ms: u32,
    },
    ItemPickedUp { item_id: NetItemId, by_player: NetEntityId },
    ItemEquipped { player: NetEntityId, slot: EquipSlot, item: ItemSnapshot },
    ItemUnequipped { player: NetEntityId, slot: EquipSlot },
    InventoryUpdate {
        player: NetEntityId,
        items: Vec<InventorySlotInfo>,
    },

    // --- Player stats ---
    StatChanged { player: NetEntityId, stat_name: String, value: i32 },
    ExperienceGained { player: NetEntityId, amount: u64, current: u64 },
    LevelUp { player: NetEntityId, new_level: i32 },
    PlayerDied { player: NetEntityId, is_hardcore: bool },
    GoldChanged { player: NetEntityId, amount: i64 },

    // --- World ---
    ZoneLoaded { zone_id: ZoneId, zone_name: String },
    WaypointActivated { player: NetEntityId, zone_id: ZoneId },
    PortalSpawned { id: NetEntityId, x: f32, y: f32, owner: NetEntityId },
    ShrineActivated { shrine_type: String, player: NetEntityId },

    // --- Quetes ---
    QuestUpdated { player: NetEntityId, quest_id: String, state: String },
    QuestCompleted { player: NetEntityId, quest_id: String },

    // --- Trade ---
    TradeOpened { with_player: NetEntityId },
    TradeItemAdded { by_player: NetEntityId, item: ItemSnapshot },
    TradeItemRemoved { by_player: NetEntityId, item_id: NetItemId },
    TradeGoldChanged { by_player: NetEntityId, amount: u32 },
    TradeAccepted { by_player: NetEntityId },
    TradeCompleted,
    TradeCancelled,

    // --- Network ---
    PlayerJoined { entity: EntitySnapshot, account_name: String },
    PlayerLeft { id: NetEntityId, account_name: String },

    // --- Chat ---
    ChatMessage { from: String, channel: ChatChannel, text: String },

    // --- Erreur ---
    Error { code: String, message: String },

    // --- Keepalive ---
    Pong { seq: u32 },

    // --- Save ---
    SaveConfirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlotInfo {
    pub item: ItemSnapshot,
    pub grid_x: i32,
    pub grid_y: i32,
}
```

---

## 6. ServerState et gestion des sessions

```rust
// games/sd-lobby/src/state.rs

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use sd_persistence::DbPool;

/// Information d'un lobby (partie en attente ou en cours)
#[derive(Debug)]
pub struct LobbyInfo {
    pub id: String,
    pub host_account_id: String,
    pub host_name: String,
    pub difficulty: String,         // "normal" | "nightmare" | "hell"
    pub act: i32,
    pub players: Vec<String>,      // account_ids des joueurs presents
    pub max_players: usize,        // 1-8
    pub password_hash: Option<String>,
    pub host_addr: String,         // adresse TCP du listen server pour mge-net
    pub created_at: String,
}

/// Etat partage du serveur lobby.
/// Encapsule dans Arc pour le partage entre handlers axum.
pub struct LobbyState {
    pub db: Arc<DbPool>,
    /// Tokens de session : token -> account_id
    pub session_tokens: RwLock<HashMap<String, SessionInfo>>,
    /// Lobbies actifs : lobby_id -> LobbyInfo
    pub lobbies: RwLock<HashMap<String, LobbyInfo>>,
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub account_id: String,
    pub username: String,
    pub created_at: String,
}

impl LobbyState {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self {
            db,
            session_tokens: RwLock::new(HashMap::new()),
            lobbies: RwLock::new(HashMap::new()),
        }
    }

    /// Valide un token de session. Retourne le SessionInfo si valide.
    pub async fn validate_token(&self, token: &str) -> Option<SessionInfo> {
        self.session_tokens.read().await.get(token).cloned()
    }

    /// Enregistre un token de session.
    pub async fn register_token(&self, token: String, info: SessionInfo) {
        self.session_tokens.write().await.insert(token, info);
    }

    /// Revoque un token de session.
    pub async fn revoke_token(&self, token: &str) {
        self.session_tokens.write().await.remove(token);
    }
}
```

---

## 7. Handlers REST -- Auth & Lobby (axum)

### Auth handlers

```rust
// games/sd-lobby/src/auth.rs

use axum::{extract::State, response::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use sd_persistence::accounts::{AccountDal, CreateAccountParams};
use crate::state::{LobbyState, SessionInfo};

// --- Register ---

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub account_id: String,
}

pub async fn register(
    State(state): State<Arc<LobbyState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, StatusCode> {
    // Validation entrees
    if body.username.len() < 3 || body.username.len() > 16 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.password.len() < 8 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let dal = AccountDal(&state.db);
    let account = dal.create(CreateAccountParams {
        username: &body.username,
        password_hash: &hash,
        email: &body.email,
    }).map_err(|e| {
        tracing::warn!("Register failed for {}: {}", body.username, e);
        StatusCode::CONFLICT
    })?;

    Ok(Json(RegisterResponse { account_id: account.id }))
}

// --- Login ---

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub account_id: String,
    pub username: String,
}

pub async fn login(
    State(state): State<Arc<LobbyState>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let dal = AccountDal(&state.db);
    let account = dal.find_by_username(&body.username)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if account.is_banned {
        tracing::warn!("Banned account login attempt: {}", body.username);
        return Err(StatusCode::FORBIDDEN);
    }

    let valid = bcrypt::verify(&body.password, &account.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Token session = UUID v4 simple (suffisant pour LAN/listen server)
    let token = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    state.register_token(token.clone(), SessionInfo {
        account_id: account.id.clone(),
        username: account.username.clone(),
        created_at: now,
    }).await;

    // Mettre a jour last_login
    dal.update_last_login(&account.id)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        account_id: account.id,
        username: account.username,
    }))
}

// --- Logout ---

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub token: String,
}

pub async fn logout(
    State(state): State<Arc<LobbyState>>,
    Json(body): Json<LogoutRequest>,
) -> StatusCode {
    state.revoke_token(&body.token).await;
    StatusCode::OK
}
```

### Lobby handlers

```rust
// games/sd-lobby/src/lobby.rs

use axum::{
    extract::{State, Path},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::state::{LobbyState, LobbyInfo};

// --- Create Lobby ---

#[derive(Deserialize)]
pub struct CreateLobbyRequest {
    pub token: String,
    pub difficulty: String,
    pub act: i32,
    pub max_players: usize,
    pub password: Option<String>,
    pub host_addr: String,          // l'adresse TCP du mge-net server
}

#[derive(Serialize)]
pub struct CreateLobbyResponse {
    pub lobby_id: String,
}

pub async fn create_lobby(
    State(state): State<Arc<LobbyState>>,
    Json(body): Json<CreateLobbyRequest>,
) -> Result<Json<CreateLobbyResponse>, StatusCode> {
    let session = state.validate_token(&body.token).await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Validation difficulte
    if !["normal", "nightmare", "hell"].contains(&body.difficulty.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }
    if !(1..=5).contains(&body.act) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let lobby_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let password_hash = body.password.as_ref().map(|p| {
        bcrypt::hash(p, 4).unwrap_or_default()  // cost 4 suffisant pour un mot de passe lobby
    });

    let lobby = LobbyInfo {
        id: lobby_id.clone(),
        host_account_id: session.account_id.clone(),
        host_name: session.username.clone(),
        difficulty: body.difficulty,
        act: body.act,
        players: vec![session.account_id],
        max_players: body.max_players.clamp(1, 8),
        password_hash,
        host_addr: body.host_addr,
        created_at: now,
    };

    state.lobbies.write().await.insert(lobby_id.clone(), lobby);
    Ok(Json(CreateLobbyResponse { lobby_id }))
}

// --- List Lobbies ---

#[derive(Serialize)]
pub struct LobbyListEntry {
    pub id: String,
    pub host_name: String,
    pub difficulty: String,
    pub act: i32,
    pub players: usize,
    pub max_players: usize,
    pub has_password: bool,
}

pub async fn list_lobbies(
    State(state): State<Arc<LobbyState>>,
) -> Json<Vec<LobbyListEntry>> {
    let lobbies = state.lobbies.read().await;
    let entries = lobbies.values().map(|l| LobbyListEntry {
        id: l.id.clone(),
        host_name: l.host_name.clone(),
        difficulty: l.difficulty.clone(),
        act: l.act,
        players: l.players.len(),
        max_players: l.max_players,
        has_password: l.password_hash.is_some(),
    }).collect();
    Json(entries)
}

// --- Join Lobby ---

#[derive(Deserialize)]
pub struct JoinLobbyRequest {
    pub token: String,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct JoinLobbyResponse {
    pub host_addr: String,          // le client se connecte en TCP a cette adresse
    pub lobby_id: String,
}

pub async fn join_lobby(
    State(state): State<Arc<LobbyState>>,
    Path(lobby_id): Path<String>,
    Json(body): Json<JoinLobbyRequest>,
) -> Result<Json<JoinLobbyResponse>, StatusCode> {
    let session = state.validate_token(&body.token).await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut lobbies = state.lobbies.write().await;
    let lobby = lobbies.get_mut(&lobby_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // Verification mot de passe
    if let Some(ref hash) = lobby.password_hash {
        let pwd = body.password.as_deref().unwrap_or("");
        let valid = bcrypt::verify(pwd, hash).unwrap_or(false);
        if !valid {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Verification capacite
    if lobby.players.len() >= lobby.max_players {
        return Err(StatusCode::CONFLICT); // lobby plein
    }

    // Verification doublon
    if lobby.players.contains(&session.account_id) {
        return Err(StatusCode::CONFLICT);
    }

    lobby.players.push(session.account_id);
    let host_addr = lobby.host_addr.clone();

    Ok(Json(JoinLobbyResponse { host_addr, lobby_id }))
}

// --- Leave Lobby ---

#[derive(Deserialize)]
pub struct LeaveLobbyRequest {
    pub token: String,
}

pub async fn leave_lobby(
    State(state): State<Arc<LobbyState>>,
    Path(lobby_id): Path<String>,
    Json(body): Json<LeaveLobbyRequest>,
) -> StatusCode {
    let session = match state.validate_token(&body.token).await {
        Some(s) => s,
        None => return StatusCode::UNAUTHORIZED,
    };

    let mut lobbies = state.lobbies.write().await;
    if let Some(lobby) = lobbies.get_mut(&lobby_id) {
        lobby.players.retain(|id| id != &session.account_id);
        // Si le host quitte ou le lobby est vide, on le supprime
        if lobby.players.is_empty() || lobby.host_account_id == session.account_id {
            lobbies.remove(&lobby_id);
        }
    }
    StatusCode::OK
}
```

### Router principal

```rust
// games/sd-lobby/src/lib.rs

use axum::{routing::{get, post, delete}, Router};
use std::sync::Arc;
use thiserror::Error;

pub mod state;
pub mod auth;
pub mod lobby;
pub mod character;

pub use state::LobbyState;

#[derive(Debug, Error)]
pub enum LobbyError {
    #[error("Persistence: {0}")]
    Persistence(#[from] sd_persistence::PersistenceError),
    #[error("Auth failed: {0}")]
    AuthFailed(String),
    #[error("Lobby not found: {0}")]
    LobbyNotFound(String),
    #[error("Lobby full")]
    LobbyFull,
}

pub fn build_router(state: Arc<LobbyState>) -> Router {
    Router::new()
        // Auth
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/logout", post(auth::logout))
        // Lobbies
        .route("/lobbies", get(lobby::list_lobbies).post(lobby::create_lobby))
        .route("/lobbies/{lobby_id}/join", post(lobby::join_lobby))
        .route("/lobbies/{lobby_id}/leave", post(lobby::leave_lobby))
        // Characters (pour la selection avant de jouer)
        // .route("/characters", get(character::list).post(character::create))
        .with_state(state)
}

/// Demarre le serveur REST lobby sur l'adresse donnee.
pub async fn start_lobby_server(state: Arc<LobbyState>, addr: &str) {
    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind(addr).await
        .expect("Failed to bind lobby server");
    tracing::info!("sd-lobby REST server listening on {}", addr);
    axum::serve(listener, app).await.expect("Lobby server crashed");
}
```

### Endpoints REST -- Resume

| Methode | Route                      | Corps requete          | Reponse                  | Description                     |
|---------|----------------------------|------------------------|--------------------------|---------------------------------|
| POST    | `/auth/register`           | `RegisterRequest`      | `RegisterResponse`       | Cree un compte                  |
| POST    | `/auth/login`              | `LoginRequest`         | `LoginResponse`          | Authentifie, retourne token     |
| POST    | `/auth/logout`             | `LogoutRequest`        | 200 OK                   | Revoque le token                |
| GET     | `/lobbies`                 | -                      | `Vec<LobbyListEntry>`   | Liste les parties ouvertes      |
| POST    | `/lobbies`                 | `CreateLobbyRequest`   | `CreateLobbyResponse`    | Cree un lobby                   |
| POST    | `/lobbies/{id}/join`       | `JoinLobbyRequest`     | `JoinLobbyResponse`      | Rejoint un lobby                |
| POST    | `/lobbies/{id}/leave`      | `LeaveLobbyRequest`    | 200 OK                   | Quitte un lobby                 |

---

## 8. Transport gameplay -- TCP bincode

Une fois le lobby rejoint via REST, le client ouvre une connexion TCP directe
vers le listen server du host via `mge-net`.

### Flux de connexion

```
1. Client → REST POST /auth/login         → token
2. Client → REST GET  /lobbies             → liste des parties
3. Client → REST POST /lobbies/{id}/join   → host_addr (ex: "192.168.1.42:7777")
4. Client → TCP  connect(host_addr)        → mge-net Connection
5. Client → TCP  send(Handshake { token, character_id })
6. Server → TCP  send(Welcome { your_entity_id, world_snapshot })
7. [boucle gameplay 25 Hz]
   Client → TCP  send(ClientMessage)
   Server → TCP  send(ServerMessage)
```

### Handshake TCP

```rust
// games/sodomight/src/net/handshake.rs

use serde::{Deserialize, Serialize};

/// Premier message envoye par le client apres connexion TCP.
/// Le serveur verifie le token et charge le personnage.
#[derive(Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub token: String,          // token obtenu via REST /auth/login
    pub character_id: String,   // UUID du personnage selectionne
}

/// Reponse du serveur au handshake.
#[derive(Debug, Serialize, Deserialize)]
pub enum HandshakeResult {
    /// Connexion acceptee, voici ton entite et l'etat du monde
    Accepted {
        your_entity_id: u32,
        world_snapshot: Vec<u8>,    // WorldSnapshot serialise bincode
    },
    /// Connexion refusee
    Rejected { reason: String },
}
```

### Boucle de reception serveur (par client)

```rust
// games/sodomight/src/net/server_session.rs

use mge_net::connection::{ConnectionReader, ConnectionWriter};
use tokio::sync::mpsc;
use crate::net::messages::{ClientMessage, ServerMessage};

/// Gere la session d'un client connecte en TCP.
pub async fn run_client_session(
    mut reader: ConnectionReader,
    mut writer: ConnectionWriter,
    // Canal pour envoyer les messages du client vers la game loop
    to_game_loop: mpsc::Sender<(u32, ClientMessage)>,
    // Canal pour recevoir les messages du serveur vers ce client
    mut from_game_loop: mpsc::Receiver<ServerMessage>,
) {
    let conn_id = reader.id.0;

    // Tache : recevoir les messages client -> game loop
    let recv_handle = tokio::spawn(async move {
        loop {
            match reader.recv::<ClientMessage>().await {
                Ok(msg) => {
                    if to_game_loop.send((conn_id, msg)).await.is_err() {
                        break; // game loop fermee
                    }
                }
                Err(mge_net::NetError::ConnectionClosed) => break,
                Err(e) => {
                    tracing::warn!("Client {} recv error: {}", conn_id, e);
                    break;
                }
            }
        }
    });

    // Tache : envoyer les messages game loop -> client
    let send_handle = tokio::spawn(async move {
        while let Some(msg) = from_game_loop.recv().await {
            if writer.send(&msg).await.is_err() {
                break; // connexion perdue
            }
        }
    });

    // Attendre que l'une ou l'autre tache termine
    tokio::select! {
        _ = recv_handle => {}
        _ = send_handle => {}
    }

    tracing::info!("Client session {} ended", conn_id);
}
```

---

## 9. Game loop serveur (25 TPS)

La game loop serveur tourne a 25 ticks/seconde (40ms par tick), synchronisee
avec le FixedUpdate de l'ECS (SD-Tech-Architecture section 6.3).

```rust
// games/sodomight/src/net/game_loop_server.rs

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use crate::net::messages::{ClientMessage, ServerMessage};

/// Contexte d'un joueur connecte
struct ConnectedPlayer {
    conn_id: u32,
    account_id: String,
    character_id: String,
    net_entity_id: u32,
    // Canal pour envoyer des messages a ce client
    tx: mpsc::Sender<ServerMessage>,
    // Position connue du serveur (pour anti-cheat)
    last_x: f32,
    last_y: f32,
}

/// Boucle serveur autoritaire a 25 Hz.
/// Recoit les inputs clients, met a jour le monde ECS, broadcast les changements.
pub async fn run_server_game_loop(
    // Canal pour recevoir les messages de tous les clients
    mut client_msgs: mpsc::Receiver<(u32, ClientMessage)>,
    // Registre des joueurs connectes (gere par le accept loop)
    players: Arc<tokio::sync::RwLock<HashMap<u32, ConnectedPlayer>>>,
) {
    let mut ticker = interval(Duration::from_millis(40)); // 25 Hz
    let mut tick: u64 = 0;

    loop {
        ticker.tick().await;
        tick += 1;

        // 1. Drain tous les messages clients recus depuis le dernier tick
        let mut incoming = Vec::new();
        while let Ok(msg) = client_msgs.try_recv() {
            incoming.push(msg);
        }

        // 2. Traiter les inputs clients
        for (conn_id, msg) in &incoming {
            process_client_input(*conn_id, msg, &players).await;
        }

        // 3. Mettre a jour les systemes ECS (FixedUpdate)
        //    - AI monstres (10 Hz = tous les 2-3 ticks)
        //    - Combat resolution
        //    - Status effects tick
        //    - Projectile mouvement
        //    - Loot timers
        if tick % 2 == 0 {
            // AI update a ~12.5 Hz (tous les 2 ticks)
            // tick_monster_ai(&world).await;
        }
        // tick_combat(&world).await;
        // tick_status_effects(&world).await;
        // tick_projectiles(&world).await;
        // tick_loot_timers(&world).await;

        // 4. Respawn monstres morts
        if tick % 125 == 0 {
            // Toutes les 5 secondes
            // tick_monster_respawn(&world).await;
        }

        // 5. Autosave toutes les 30 secondes (750 ticks)
        if tick % 750 == 0 {
            tracing::debug!("Autosave at tick {}", tick);
            // save_all_characters(&state).await;
        }

        // 6. Nettoyage des connexions mortes toutes les 10 secondes
        if tick % 250 == 0 {
            cleanup_disconnected_players(&players).await;
        }

        // 7. Broadcast des changements (delta compression)
        //    - EntityMoved pour les entites qui ont bouge
        //    - EntityHealthChanged pour les HP modifies
        //    - etc.
        // broadcast_delta(&world, &players).await;
    }
}

async fn process_client_input(
    conn_id: u32,
    msg: &ClientMessage,
    players: &tokio::sync::RwLock<HashMap<u32, ConnectedPlayer>>,
) {
    match msg {
        ClientMessage::MoveToPosition { x, y } => {
            // Validation anti-cheat : position raisonnable ?
            // Mettre a jour la position cible dans le monde ECS
            // Le pathfinding calculera le chemin cote serveur
            tracing::trace!("Player conn={} move to ({}, {})", conn_id, x, y);
        }
        ClientMessage::UseSkill { skill_id, target } => {
            // Validation : skill appris ? cooldown ok ? mana suffisant ?
            // Ajouter l'action au pipeline de combat
            tracing::trace!("Player conn={} use skill {:?}", conn_id, skill_id);
        }
        ClientMessage::NormalAttack { target } => {
            // Validation : cible a portee ? pas de cooldown auto-attack ?
            tracing::trace!("Player conn={} attack {:?}", conn_id, target);
        }
        ClientMessage::PickupItem { entity_id } => {
            // Verification : item existe ? a portee ? loot timer expire ou priorite ?
            tracing::trace!("Player conn={} pickup {:?}", conn_id, entity_id);
        }
        ClientMessage::ChatMessage { channel, text } => {
            // Broadcast le chat a tous les joueurs
            let players_read = players.read().await;
            let from_name = players_read.get(&conn_id)
                .map(|p| p.account_id.clone())
                .unwrap_or_default();
            let broadcast = ServerMessage::ChatMessage {
                from: from_name,
                channel: channel.clone(),
                text: text.clone(),
            };
            for player in players_read.values() {
                let _ = player.tx.send(broadcast.clone()).await;
            }
        }
        ClientMessage::Ping { seq } => {
            let players_read = players.read().await;
            if let Some(player) = players_read.get(&conn_id) {
                let _ = player.tx.send(ServerMessage::Pong { seq: *seq }).await;
            }
        }
        ClientMessage::SaveAndQuit => {
            tracing::info!("Player conn={} requests save & quit", conn_id);
            let players_read = players.read().await;
            if let Some(player) = players_read.get(&conn_id) {
                // Sauvegarder le personnage
                // save_character(&state.db, &player.character_id).await;
                let _ = player.tx.send(ServerMessage::SaveConfirmed).await;
            }
        }
        // Autres messages : deleger au systeme concerne
        _ => {
            tracing::debug!("Unhandled message from conn={}: {:?}", conn_id, msg);
        }
    }
}

async fn cleanup_disconnected_players(
    players: &tokio::sync::RwLock<HashMap<u32, ConnectedPlayer>>,
) {
    let mut write = players.write().await;
    write.retain(|_, p| !p.tx.is_closed());
}
```

### Scaling HP monstres multijoueur

Conformement a SD-Monsters-AI.md, le HP des monstres scale avec le nombre de joueurs :

```
HP_effectif = HP_base * (N + 1) / 2
```

Ou `N` est le nombre de joueurs dans la partie.

| Joueurs | Multiplicateur HP |
|---------|-------------------|
| 1       | 1.0x              |
| 2       | 1.5x              |
| 3       | 2.0x              |
| 4       | 2.5x              |
| 5       | 3.0x              |
| 6       | 3.5x              |
| 7       | 4.0x              |
| 8       | 4.5x              |

```rust
/// Calcule le multiplicateur HP pour N joueurs
pub fn hp_multiplier(player_count: u32) -> f32 {
    (player_count as f32 + 1.0) / 2.0
}

#[cfg(test)]
#[test]
fn test_hp_multiplier() {
    assert!((hp_multiplier(1) - 1.0).abs() < f32::EPSILON);
    assert!((hp_multiplier(2) - 1.5).abs() < f32::EPSILON);
    assert!((hp_multiplier(4) - 2.5).abs() < f32::EPSILON);
    assert!((hp_multiplier(8) - 4.5).abs() < f32::EPSILON);
}
```

---

## 10. Anti-cheat basique

Le modele autoritaire signifie que le serveur (host) est la seule source de verite.
Les clients envoient des **intentions** (inputs), jamais des **resultats**.

```rust
// games/sodomight/src/net/anti_cheat.rs

/// Vitesse maximale en unites de carte par tick (40ms).
/// Un personnage a ~10 tiles/sec en sprint, soit 0.4 tiles/tick.
const MAX_SPEED_PER_TICK: f32 = 0.4;

/// Tolerance pour la latence (x3, couvre ~120ms de lag)
const SPEED_TOLERANCE: f32 = 3.0;

/// Valide un deplacement. Retourne false si suspect.
pub fn validate_move(
    prev_x: f32,
    prev_y: f32,
    new_x: f32,
    new_y: f32,
    ticks_elapsed: u32,
) -> bool {
    let dx = new_x - prev_x;
    let dy = new_y - prev_y;
    let dist = (dx * dx + dy * dy).sqrt();
    let max_dist = MAX_SPEED_PER_TICK * SPEED_TOLERANCE * ticks_elapsed as f32;
    dist <= max_dist
}

/// Verifie qu'un skill est dans la liste des skills appris du personnage.
pub fn validate_skill_use(skill_id: &str, learned_skills: &[(String, i32)]) -> bool {
    learned_skills.iter().any(|(s, points)| s == skill_id && *points > 0)
}

/// Verifie qu'un item appartient bien au joueur (owner_id en DB).
pub fn validate_item_owner(item_owner_id: &str, player_character_id: &str) -> bool {
    item_owner_id == player_character_id
}

/// Verifie que la cible est a portee du joueur pour le ramassage.
/// Portee standard de pickup : 5 tiles.
const PICKUP_RANGE: f32 = 5.0;

pub fn validate_pickup_range(
    player_x: f32,
    player_y: f32,
    item_x: f32,
    item_y: f32,
) -> bool {
    let dx = item_x - player_x;
    let dy = item_y - player_y;
    (dx * dx + dy * dy).sqrt() <= PICKUP_RANGE
}

/// Verifie le loot priority timer.
/// Le killer a 30 secondes de priorite (750 ticks a 25 Hz).
pub fn validate_loot_priority(
    picker_entity_id: u32,
    priority_entity_id: Option<u32>,
    priority_remaining_ticks: u32,
) -> bool {
    match priority_entity_id {
        Some(priority_id) => {
            picker_entity_id == priority_id || priority_remaining_ticks == 0
        }
        None => true,
    }
}

/// Rate limiting : max messages par seconde par client.
/// Un client normal envoie ~25 msg/s (1 par tick). Au-dela de 50, suspect.
const MAX_MESSAGES_PER_SECOND: u32 = 50;

pub struct RateLimiter {
    count: u32,
    last_reset_tick: u64,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self { count: 0, last_reset_tick: 0 }
    }

    /// Retourne true si le message est autorise, false si rate-limite.
    pub fn check(&mut self, current_tick: u64) -> bool {
        // Reset toutes les 25 ticks (1 seconde)
        if current_tick - self.last_reset_tick >= 25 {
            self.count = 0;
            self.last_reset_tick = current_tick;
        }
        self.count += 1;
        self.count <= MAX_MESSAGES_PER_SECOND
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_move_ok() {
        // 0.3 tiles en 1 tick, bien sous le seuil
        assert!(validate_move(0.0, 0.0, 0.3, 0.0, 1));
    }

    #[test]
    fn test_validate_move_teleport_rejected() {
        // 50 tiles en 1 tick = teleportation
        assert!(!validate_move(0.0, 0.0, 50.0, 0.0, 1));
    }

    #[test]
    fn test_validate_move_multi_tick_ok() {
        // 3 tiles en 10 ticks = 0.3 tiles/tick, ok
        assert!(validate_move(0.0, 0.0, 3.0, 0.0, 10));
    }

    #[test]
    fn test_validate_skill_use() {
        let skills = vec![
            ("bone_spear".to_string(), 5),
            ("raise_skeleton".to_string(), 3),
        ];
        assert!(validate_skill_use("bone_spear", &skills));
        assert!(!validate_skill_use("teleport", &skills));
    }

    #[test]
    fn test_validate_pickup_range() {
        assert!(validate_pickup_range(0.0, 0.0, 3.0, 3.0));    // ~4.24, ok
        assert!(!validate_pickup_range(0.0, 0.0, 10.0, 10.0));  // ~14.14, trop loin
    }

    #[test]
    fn test_loot_priority() {
        // Le killer peut ramasser pendant le timer
        assert!(validate_loot_priority(1, Some(1), 100));
        // Un autre joueur ne peut pas pendant le timer
        assert!(!validate_loot_priority(2, Some(1), 100));
        // Un autre joueur peut quand le timer expire
        assert!(validate_loot_priority(2, Some(1), 0));
        // Pas de priorite = tout le monde peut
        assert!(validate_loot_priority(2, None, 0));
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new();
        for _ in 0..50 {
            assert!(limiter.check(0));
        }
        assert!(!limiter.check(0)); // 51eme message refuse
        assert!(limiter.check(25)); // reset apres 1 seconde
    }
}
```

---

## 11. Integration Listen Server dans sodomight-game

### Demarrage du Listen Server (host)

```rust
// games/sodomight/src/main.rs (extrait)

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use sd_persistence::DbPool;
use sd_lobby::{LobbyState, start_lobby_server};
use mge_net::NetServer;

const LISTEN_SERVER_ADDR: &str = "0.0.0.0:7777";
const LOBBY_REST_ADDR: &str = "0.0.0.0:7778";

/// Demarre le listen server complet (lobby REST + gameplay TCP).
async fn start_listen_server(db_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ouvrir la base de donnees
    let db = Arc::new(DbPool::open(db_path)?);

    // 2. Demarrer le lobby REST (auth, gestion parties)
    let lobby_state = Arc::new(LobbyState::new(Arc::clone(&db)));
    let lobby_state_clone = Arc::clone(&lobby_state);
    tokio::spawn(async move {
        start_lobby_server(lobby_state_clone, LOBBY_REST_ADDR).await;
    });

    // 3. Demarrer le serveur TCP gameplay (mge-net)
    let mut net_server = NetServer::bind(LISTEN_SERVER_ADDR, 8).await?;
    let (client_msg_tx, client_msg_rx) = mpsc::channel(1024);
    let players = Arc::new(RwLock::new(HashMap::new()));

    // 4. Lancer la game loop serveur
    let players_loop = Arc::clone(&players);
    tokio::spawn(async move {
        crate::net::game_loop_server::run_server_game_loop(
            client_msg_rx,
            players_loop,
        ).await;
    });

    // 5. Accept loop : accepter les connexions TCP des clients
    let players_accept = Arc::clone(&players);
    tokio::spawn(async move {
        loop {
            let current_count = players_accept.read().await.len();
            match net_server.accept(current_count).await {
                Ok(conn) => {
                    let conn_id = conn.id.0;
                    let (reader, writer) = conn.split();

                    // Canal pour les messages serveur -> ce client
                    let (tx, rx) = mpsc::channel::<crate::net::messages::ServerMessage>(256);

                    // TODO : handshake (recevoir token + character_id, valider, charger perso)

                    let client_msg_tx_clone = client_msg_tx.clone();
                    tokio::spawn(async move {
                        crate::net::server_session::run_client_session(
                            reader,
                            writer,
                            client_msg_tx_clone,
                            rx,
                        ).await;
                    });
                }
                Err(mge_net::NetError::MaxClients(_)) => {
                    tracing::warn!("Server full, rejecting connection");
                }
                Err(e) => {
                    tracing::error!("Accept error: {}", e);
                }
            }
        }
    });

    Ok(())
}
```

### Connexion client (invite)

```rust
// games/sodomight-client/src/connect.rs (extrait)

use mge_net::NetClient;
use crate::net::messages::{ClientMessage, ServerMessage};
use crate::net::handshake::Handshake;

/// Se connecte a un listen server distant.
pub async fn connect_to_server(
    host_addr: &str,
    token: &str,
    character_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = NetClient::connect(host_addr).await?;

    // Envoyer le handshake
    conn.send(&Handshake {
        token: token.to_string(),
        character_id: character_id.to_string(),
    }).await?;

    // Recevoir la reponse
    let result: crate::net::handshake::HandshakeResult = conn.recv().await?;
    match result {
        crate::net::handshake::HandshakeResult::Accepted { your_entity_id, .. } => {
            tracing::info!("Connected! Entity ID = {}", your_entity_id);
            // Demarrer la boucle client (recv ServerMessage, send ClientMessage)
        }
        crate::net::handshake::HandshakeResult::Rejected { reason } => {
            tracing::error!("Connection rejected: {}", reason);
            return Err(reason.into());
        }
    }

    Ok(())
}
```

### Architecture des ports

| Port  | Protocole | Usage                                |
|-------|-----------|--------------------------------------|
| 7777  | TCP       | mge-net gameplay (bincode, 25 Hz)    |
| 7778  | HTTP      | sd-lobby REST (JSON, auth/lobbies)   |

Le host ouvre les deux ports. Les clients invites :
1. Se connectent a `host:7778` pour l'auth REST et la liste des lobbies
2. Se connectent a `host:7777` pour le gameplay TCP en temps reel

---

## 12. Delta compression

Pour optimiser la bande passante, le serveur n'envoie que les changements
depuis le dernier tick. L'implementation suit les specs de Denis (SD-Tech-Architecture 8.4).

```rust
// games/sodomight/src/net/delta.rs

use std::collections::HashMap;
use crate::net::messages::{NetEntityId, ServerMessage, EntitySnapshot};

/// Compresseur delta : compare l'etat courant avec le dernier etat envoye
/// et ne genere que les messages pour les champs modifies.
pub struct DeltaCompressor {
    last_sent: HashMap<NetEntityId, EntitySnapshot>,
}

impl DeltaCompressor {
    pub fn new() -> Self {
        Self { last_sent: HashMap::new() }
    }

    /// Compare l'etat courant avec le cache et retourne les messages delta.
    pub fn compute_delta(
        &mut self,
        current_entities: &HashMap<NetEntityId, EntitySnapshot>,
    ) -> Vec<ServerMessage> {
        let mut messages = Vec::new();

        // Entites nouvelles ou modifiees
        for (id, entity) in current_entities {
            match self.last_sent.get(id) {
                Some(prev) => {
                    // Position changee ?
                    if (prev.x - entity.x).abs() > 0.01
                        || (prev.y - entity.y).abs() > 0.01
                    {
                        messages.push(ServerMessage::EntityMoved {
                            id: *id,
                            x: entity.x,
                            y: entity.y,
                            vel_x: entity.vel_x,
                            vel_y: entity.vel_y,
                        });
                    }
                    // HP change ?
                    if prev.current_life != entity.current_life
                        || prev.max_life != entity.max_life
                    {
                        messages.push(ServerMessage::EntityHealthChanged {
                            id: *id,
                            current: entity.current_life,
                            max: entity.max_life,
                        });
                    }
                    // Animation changee ?
                    if prev.animation_id != entity.animation_id {
                        if let Some(ref anim) = entity.animation_id {
                            messages.push(ServerMessage::EntityAnimationChanged {
                                id: *id,
                                animation_id: anim.clone(),
                            });
                        }
                    }
                }
                None => {
                    // Nouvelle entite
                    messages.push(ServerMessage::EntitySpawned {
                        entity: entity.clone(),
                    });
                }
            }
            self.last_sent.insert(*id, entity.clone());
        }

        // Entites supprimees (presentes dans last_sent mais pas dans current)
        let despawned: Vec<NetEntityId> = self.last_sent.keys()
            .filter(|id| !current_entities.contains_key(id))
            .copied()
            .collect();
        for id in &despawned {
            messages.push(ServerMessage::EntityDespawned { id: *id });
            self.last_sent.remove(id);
        }

        messages
    }

    /// Reset complet (changement de zone par exemple).
    pub fn reset(&mut self) {
        self.last_sent.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net::messages::EntityType;

    fn make_entity(id: u32, x: f32, y: f32, hp: i32) -> (NetEntityId, EntitySnapshot) {
        let net_id = NetEntityId(id);
        (net_id, EntitySnapshot {
            net_id,
            entity_type: EntityType::Monster,
            x,
            y,
            vel_x: 0.0,
            vel_y: 0.0,
            current_life: hp,
            max_life: 100,
            level: 5,
            class_or_kind: "zombie".to_string(),
            animation_id: None,
        })
    }

    #[test]
    fn test_delta_new_entity_spawned() {
        let mut comp = DeltaCompressor::new();
        let mut entities = HashMap::new();
        let (id, snap) = make_entity(1, 10.0, 20.0, 100);
        entities.insert(id, snap);

        let msgs = comp.compute_delta(&entities);
        assert_eq!(msgs.len(), 1);
        assert!(matches!(msgs[0], ServerMessage::EntitySpawned { .. }));
    }

    #[test]
    fn test_delta_no_change_no_message() {
        let mut comp = DeltaCompressor::new();
        let mut entities = HashMap::new();
        let (id, snap) = make_entity(1, 10.0, 20.0, 100);
        entities.insert(id, snap);

        comp.compute_delta(&entities); // premier appel = spawn
        let msgs = comp.compute_delta(&entities); // deuxieme = rien
        assert!(msgs.is_empty());
    }

    #[test]
    fn test_delta_position_changed() {
        let mut comp = DeltaCompressor::new();
        let mut entities = HashMap::new();
        let (id, snap) = make_entity(1, 10.0, 20.0, 100);
        entities.insert(id, snap);
        comp.compute_delta(&entities);

        // Deplacer l'entite
        entities.get_mut(&id).unwrap().x = 12.0;
        let msgs = comp.compute_delta(&entities);
        assert_eq!(msgs.len(), 1);
        assert!(matches!(msgs[0], ServerMessage::EntityMoved { .. }));
    }

    #[test]
    fn test_delta_entity_despawned() {
        let mut comp = DeltaCompressor::new();
        let mut entities = HashMap::new();
        let (id, snap) = make_entity(1, 10.0, 20.0, 100);
        entities.insert(id, snap);
        comp.compute_delta(&entities);

        // Retirer l'entite
        entities.remove(&id);
        let msgs = comp.compute_delta(&entities);
        assert_eq!(msgs.len(), 1);
        assert!(matches!(msgs[0], ServerMessage::EntityDespawned { .. }));
    }
}
```

### Estimation bande passante

| Scenario                        | Entites | Messages/tick | Taille/msg | Bande passante   |
|---------------------------------|---------|---------------|------------|------------------|
| 1 joueur seul (solo calm)       | ~50     | ~5            | ~40 B      | ~5 KB/s          |
| 4 joueurs, combat actif         | ~200    | ~40           | ~40 B      | ~40 KB/s         |
| 8 joueurs, boss fight chaos     | ~400    | ~100          | ~40 B      | ~100 KB/s        |

Largement dans les limites d'une connexion LAN ou domestique.

---

## 13. Tests

### Tests unitaires mge-net

```rust
// crates/engine/mge-net/src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_frame_roundtrip() {
        use tokio::io::duplex;
        use crate::frame::{write_frame, read_frame};

        let (mut client, mut server) = duplex(1024);
        let payload = b"hello world";

        write_frame(&mut client, payload).await.unwrap();
        let received = read_frame(&mut server).await.unwrap();
        assert_eq!(received, payload);
    }

    #[tokio::test]
    async fn test_frame_too_large() {
        use tokio::io::duplex;
        use crate::frame::{write_frame, MAX_FRAME_SIZE};

        let (mut client, _server) = duplex(1024);
        let payload = vec![0u8; (MAX_FRAME_SIZE + 1) as usize];

        let result = write_frame(&mut client, &payload).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connection_send_recv() {
        use tokio::net::TcpListener;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct TestMsg {
            value: u32,
            text: String,
        }

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let (reader, writer) = stream.into_split();
            let mut conn = Connection::new(ConnectionId(1), reader, writer);
            let msg: TestMsg = conn.recv().await.unwrap();
            conn.send(&msg).await.unwrap();
            msg
        });

        let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        let (reader, writer) = stream.into_split();
        let mut conn = Connection::new(ConnectionId(0), reader, writer);

        let original = TestMsg { value: 42, text: "test".to_string() };
        conn.send(&original).await.unwrap();
        let echoed: TestMsg = conn.recv().await.unwrap();

        assert_eq!(original, echoed);
        let server_msg = server_handle.await.unwrap();
        assert_eq!(original, server_msg);
    }
}
```

### Tests integration sd-lobby

```rust
// games/sd-lobby/src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;
    use sd_persistence::DbPool;
    use std::sync::Arc;

    async fn test_state() -> Arc<LobbyState> {
        let db = Arc::new(DbPool::in_memory().unwrap());
        Arc::new(LobbyState::new(db))
    }

    #[tokio::test]
    async fn test_register_and_login() {
        let state = test_state().await;
        let app = build_router(Arc::clone(&state));

        // Register
        let reg_body = serde_json::json!({
            "username": "testuser",
            "password": "password123",
            "email": "test@example.com"
        });
        let response = app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(reg_body.to_string()))
                    .unwrap()
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Login
        let login_body = serde_json::json!({
            "username": "testuser",
            "password": "password123"
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(login_body.to_string()))
                    .unwrap()
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let state = test_state().await;
        let app = build_router(Arc::clone(&state));

        // Register
        let reg_body = serde_json::json!({
            "username": "user2",
            "password": "correctpassword",
            "email": "user2@example.com"
        });
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(reg_body.to_string()))
                    .unwrap()
            )
            .await
            .unwrap();

        // Login avec mauvais mot de passe
        let login_body = serde_json::json!({
            "username": "user2",
            "password": "wrongpassword"
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(login_body.to_string()))
                    .unwrap()
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_list_lobbies_empty() {
        let state = test_state().await;
        let app = build_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/lobbies")
                    .body(Body::empty())
                    .unwrap()
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_lobby_requires_auth() {
        let state = test_state().await;
        let app = build_router(state);

        let body = serde_json::json!({
            "token": "invalid-token",
            "difficulty": "normal",
            "act": 1,
            "max_players": 4,
            "host_addr": "127.0.0.1:7777"
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/lobbies")
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap()
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
```

### Tests anti-cheat

Voir section 10 ci-dessus, les tests sont integres dans le module `anti_cheat`.

### Commandes de test

```bash
# Tests mge-net
cargo test -p mge-net -- --nocapture

# Tests sd-lobby
cargo test -p sd-lobby -- --nocapture

# Tests specifiques anti-cheat
cargo test -p sodomight -- anti_cheat --nocapture

# Tests specifiques delta compression
cargo test -p sodomight -- delta --nocapture

# Lint complet
cargo clippy -p mge-net -p sd-lobby -- -D warnings
```

---

## 14. Checklist integration

### Workspace

- [ ] `crates/engine/mge-net/` cree avec `Cargo.toml` conforme
- [ ] `games/sd-lobby/` cree avec `Cargo.toml` conforme
- [ ] Deux crates ajoutees dans `mge/Cargo.toml` workspace members
- [ ] `unsafe_code = "forbid"` dans les deux `Cargo.toml`
- [ ] `[lints] workspace = true` dans les deux `Cargo.toml`

### mge-net

- [ ] Frame protocol (read/write length-prefixed bincode) operationnel
- [ ] `NetServer::bind()` et `NetServer::accept()` fonctionnels
- [ ] `NetClient::connect()` fonctionnel
- [ ] `Connection::send()` et `Connection::recv()` avec generics serde
- [ ] Tests : roundtrip frame, frame trop large, connection send/recv
- [ ] `cargo test -p mge-net -- --nocapture` : tous les tests passent
- [ ] `cargo clippy -p mge-net -- -D warnings` : zero warning

### sd-lobby

- [ ] Routes REST `/auth/register`, `/auth/login`, `/auth/logout`
- [ ] Routes REST `/lobbies` (GET list, POST create)
- [ ] Routes REST `/lobbies/{id}/join`, `/lobbies/{id}/leave`
- [ ] Validation des entrees (longueur username, password, difficulte, acte)
- [ ] Token session UUID v4
- [ ] Tests : register, login, wrong password, list lobbies, auth required
- [ ] `cargo test -p sd-lobby -- --nocapture` : tous les tests passent
- [ ] `cargo clippy -p sd-lobby -- -D warnings` : zero warning

### Messages reseau

- [ ] `ClientMessage` enum complet (mouvement, combat, items, trade, world, chat)
- [ ] `ServerMessage` enum complet (entites, combat, items, stats, world, trade, chat)
- [ ] Tous les types serialisables avec `serde` + `bincode`
- [ ] Pas de `String` la ou un type fort existe (`NetEntityId`, `SkillId`, etc.)

### Game loop serveur

- [ ] Tick rate 25 Hz (40ms) synchronise avec le FixedUpdate ECS
- [ ] Input processing : drain des messages clients a chaque tick
- [ ] AI update a ~12.5 Hz (tous les 2 ticks)
- [ ] Autosave toutes les 30 secondes
- [ ] Cleanup des connexions mortes toutes les 10 secondes
- [ ] Delta compression pour le broadcast

### Anti-cheat

- [ ] Validation vitesse de deplacement
- [ ] Validation skill appris
- [ ] Validation ownership item
- [ ] Validation portee de pickup
- [ ] Validation loot priority timer (30s)
- [ ] Rate limiting (50 msg/s max)
- [ ] Tests unitaires pour chaque validation

### Integration listen server

- [ ] Port 7777 TCP (mge-net gameplay)
- [ ] Port 7778 HTTP (sd-lobby REST)
- [ ] Handshake TCP (token + character_id)
- [ ] Welcome message avec WorldSnapshot
- [ ] HP scaling monstres par nombre de joueurs

---

## 15. Migration Phase 2 -- Serveur dedie

La Phase 2 (Sprint 11 dans le plan de Denis) separe le listen server en
deux binaires distincts :

### sodomight-server (binaire dedie)

```
games/sodomight-server/
  Cargo.toml
  src/
    main.rs             -- Point d'entree serveur, tokio::main
    server.rs           -- Accept loop TCP (mge-net)
    session.rs          -- Session joueur (handshake, auth)
    world_host.rs       -- World ECS autoritaire sans rendu
    tick.rs             -- Fixed tick 25 Hz pur logique
    loot_authority.rs   -- Drop generation cote serveur uniquement
    save_authority.rs   -- Sauvegarde autoritaire KindMother
```

**Dependances :**
- `mge-ecs`, `mge-core`, `mge-math` (logique)
- `mge-arpg-*` (toutes les crates ARPG)
- `mge-net` (transport TCP)
- `mge-save` / `sd-persistence` (KindMother)
- `sd-lobby` (auth + lobby intergre)
- PAS de `mge-render`, `mge-audio`, `mge-ui`

### sodomight-client (binaire client)

```
games/sodomight-client/
  Cargo.toml
  src/
    main.rs             -- Point d'entree client
    client.rs           -- Connexion TCP au serveur (mge-net)
    prediction.rs       -- Client-side prediction (mouvement local)
    interpolation.rs    -- Interpolation entites distantes
    render_world.rs     -- World ECS local pour rendu uniquement
```

**Dependances :**
- `mge-ecs`, `mge-core`, `mge-math` (monde local)
- `mge-render`, `mge-audio`, `mge-ui` (rendu)
- `mge-net` (transport TCP)
- `mge-platform` (fenetre, input)
- PAS de `mge-arpg-combat`, `mge-arpg-loot`, `mge-arpg-ai` (pas de logique autorit.)

### Ce qui change entre Phase 1 et Phase 2

| Aspect                    | Phase 1 (Listen Server)        | Phase 2 (Dedie)                   |
|---------------------------|--------------------------------|-----------------------------------|
| Processus serveur         | Dans le meme binaire que host  | Binaire separe headless           |
| Rendu cote serveur        | Oui (le host joue)             | Non (pas de rendu)                |
| Persistence               | SQLite local (host)            | SQLite gouverne par le serveur    |
| Lobby                     | sd-lobby integre au host       | sd-lobby dans sodomight-server    |
| Client-side prediction    | Non (host joue en local)       | Oui (necessaire pour la latence)  |
| Interpolation             | Non                            | Oui                               |
| Scaling                   | 1-8 joueurs LAN                | N joueurs, infrastructure cloud   |

### Strategie de migration

1. Extraire la logique serveur de `sodomight-game` dans un module `server/` reutilisable
2. `sodomight-server` reutilise ce module sans les dependances de rendu
3. `sodomight-client` ajoute la prediction et l'interpolation
4. Les messages `ClientMessage`/`ServerMessage` ne changent PAS
5. Le frame protocol `mge-net` ne change PAS
6. La couche `sd-lobby` ne change PAS (meme API REST)
7. Seul le point d'entree (`main.rs`) differe

---

*Fin IMPL-06. Prochain document : IMPL-07 (Client-Side Prediction & Interpolation, Phase 2).*

*Document redige par Francois, Dev Back-End -- Miyukini AI Studio*
*Base technique : SD-Tech-Architecture.md (Denis)*
*Revision : 2026-02-28 v1.0*
