use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Network mode
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetMode {
    OfflineLocal,
    HostAuthoritative,
    DedicatedClient,
}

impl NetMode {
    pub fn is_authoritative(self) -> bool {
        matches!(self, Self::OfflineLocal | Self::HostAuthoritative)
    }

    pub fn is_online(self) -> bool {
        matches!(self, Self::HostAuthoritative | Self::DedicatedClient)
    }
}

// ---------------------------------------------------------------------------
// MWS Lobby bridge — connects MWS service to game server
// ---------------------------------------------------------------------------

/// Configuration for hosting a game lobby via MWS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyConfig {
    pub name: String,
    pub max_players: u32,
    pub is_public: bool,
    pub password: Option<String>,
    pub difficulty: u8,
    pub game_mode: GameMode,
}

impl Default for LobbyConfig {
    fn default() -> Self {
        Self {
            name: "Partie de Sodomight".into(),
            max_players: 8,
            is_public: true,
            password: None,
            difficulty: 0,
            game_mode: GameMode::Coop,
        }
    }
}

/// Game modes available for multiplayer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    /// Cooperative PvE.
    Coop,
    /// Player vs Player arena.
    Pvp,
    /// Hardcore — permanent death.
    Hardcore,
}

/// State of the game's MWS lobby integration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LobbyState {
    /// Not connected to MWS.
    Offline,
    /// Creating a lobby on the tracker.
    Creating,
    /// Lobby active, waiting for players.
    Hosting { lobby_id: String },
    /// Joined another player's lobby.
    Joined { lobby_id: String, host_address: String },
    /// Error state.
    Error { reason: String },
}

impl Default for LobbyState {
    fn default() -> Self {
        Self::Offline
    }
}

/// A remote player visible in the lobby browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyListing {
    pub lobby_id: String,
    pub name: String,
    pub host_name: String,
    pub current_players: u32,
    pub max_players: u32,
    pub game_mode: GameMode,
    pub password_required: bool,
}

/// Multiplayer session info exposed to the game UI.
#[derive(Debug, Clone, Default)]
pub struct MultiplayerInfo {
    pub net_mode: Option<NetMode>,
    pub lobby_state: LobbyState,
    pub lobby_config: LobbyConfig,
    pub remote_players: Vec<RemotePlayer>,
    pub available_lobbies: Vec<LobbyListing>,
}

/// A remote player in the current game session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemotePlayer {
    pub player_id: u64,
    pub name: String,
    pub level: u32,
    pub zone_id: u32,
    pub pos: [f32; 2],
    pub hp_frac: f32,
    pub facing_right: bool,
    pub running: bool,
}
