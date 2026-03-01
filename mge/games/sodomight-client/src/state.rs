// @id: Sodomight-Client-State @do: client-state @role: back-end @layer: 4 @human: miyuk
//! Client connection state machine.

/// Current connection state of the game client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClientState {
    /// Not connected to any server.
    #[default]
    Disconnected,
    /// Connection attempt in progress.
    Connecting,
    /// Connected and authenticated.
    Connected,
    /// Actively playing in a game session.
    InGame,
}

impl ClientState {
    /// Returns true if the client has an active connection.
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected | Self::InGame)
    }

    /// Returns true if the client is in an active game session.
    pub fn can_play(&self) -> bool {
        matches!(self, Self::InGame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_connected_states() {
        assert!(ClientState::Connected.is_connected());
        assert!(ClientState::InGame.is_connected());
        assert!(!ClientState::Disconnected.is_connected());
        assert!(!ClientState::Connecting.is_connected());
    }

    #[test]
    fn test_can_play() {
        assert!(ClientState::InGame.can_play());
        assert!(!ClientState::Connected.can_play());
        assert!(!ClientState::Disconnected.can_play());
        assert!(!ClientState::Connecting.can_play());
    }

    #[test]
    fn test_default_state_disconnected() {
        assert_eq!(ClientState::default(), ClientState::Disconnected);
    }
}
