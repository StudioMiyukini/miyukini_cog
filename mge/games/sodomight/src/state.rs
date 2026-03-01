// @id: Sodomight-Game-State @do: game-state @role: back-end @layer: 4 @human: miyuk
//! High-level game state machine.

/// The game's current high-level state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GameState {
    /// Loading assets and initializing systems.
    #[default]
    Loading,
    /// Main menu is displayed.
    MainMenu,
    /// Character selection screen.
    CharacterSelect,
    /// Active gameplay.
    InGame,
    /// Game is paused.
    Paused,
    /// Game over screen.
    GameOver,
}

impl GameState {
    /// Returns true if the player can interact with the world.
    pub fn is_playing(&self) -> bool {
        matches!(self, Self::InGame)
    }

    /// Returns true if the game is in a menu state.
    pub fn is_menu(&self) -> bool {
        matches!(self, Self::MainMenu | Self::CharacterSelect | Self::GameOver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_playing_in_game() {
        assert!(GameState::InGame.is_playing());
    }

    #[test]
    fn test_is_playing_menu() {
        assert!(!GameState::MainMenu.is_playing());
    }

    #[test]
    fn test_is_menu_states() {
        assert!(GameState::MainMenu.is_menu());
        assert!(GameState::CharacterSelect.is_menu());
        assert!(GameState::GameOver.is_menu());
        assert!(!GameState::InGame.is_menu());
        assert!(!GameState::Paused.is_menu());
        assert!(!GameState::Loading.is_menu());
    }

    #[test]
    fn test_default_state_loading() {
        assert_eq!(GameState::default(), GameState::Loading);
    }
}
