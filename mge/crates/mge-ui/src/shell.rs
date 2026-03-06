use serde::{Deserialize, Serialize};

/// Top-level screens the game can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Screen {
    MainMenu,
    CharacterSelect,
    Loading,
    InGame,
    Death,
}

/// Overlay panels that can be opened on top of the game screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Overlay {
    Inventory,
    CharacterSheet,
    QuestJournal,
    Stash,
    Vendor,
    SystemMenu,
    Options,
    DebugPanel,
}

/// Where keyboard/mouse focus is directed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusTarget {
    /// The game world receives input (movement, skills, etc.).
    World,
    /// An overlay panel has focus (inventory drag, menu navigation).
    Panel(Overlay),
    /// A text input field has focus.
    TextInput,
    /// A modal dialog blocks all other input.
    Modal,
}

/// Manages the current screen, open overlays and input focus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiShell {
    screen: Screen,
    overlays: Vec<Overlay>,
    focus: FocusTarget,
}

impl Default for UiShell {
    fn default() -> Self {
        Self { screen: Screen::MainMenu, overlays: Vec::new(), focus: FocusTarget::World }
    }
}

impl UiShell {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.screen = screen;
        self.overlays.clear();
        self.focus = FocusTarget::World;
    }

    pub fn focus(&self) -> FocusTarget {
        self.focus
    }

    pub fn set_focus(&mut self, focus: FocusTarget) {
        self.focus = focus;
    }

    /// Open an overlay. If already open, brings it to front (focus).
    pub fn open_overlay(&mut self, overlay: Overlay) {
        self.overlays.retain(|o| *o != overlay);
        self.overlays.push(overlay);
        self.focus = FocusTarget::Panel(overlay);
    }

    /// Close an overlay. Focus returns to the next top overlay or world.
    pub fn close_overlay(&mut self, overlay: Overlay) {
        self.overlays.retain(|o| *o != overlay);
        self.focus = self.overlays.last().map_or(FocusTarget::World, |o| FocusTarget::Panel(*o));
    }

    /// Toggle an overlay open/closed.
    pub fn toggle_overlay(&mut self, overlay: Overlay) {
        if self.is_overlay_open(overlay) {
            self.close_overlay(overlay);
        } else {
            self.open_overlay(overlay);
        }
    }

    pub fn is_overlay_open(&self, overlay: Overlay) -> bool {
        self.overlays.contains(&overlay)
    }

    /// Close all overlays and return focus to world.
    pub fn close_all_overlays(&mut self) {
        self.overlays.clear();
        self.focus = FocusTarget::World;
    }

    pub fn open_overlays(&self) -> &[Overlay] {
        &self.overlays
    }

    /// Whether the game world should process gameplay input.
    pub fn world_has_focus(&self) -> bool {
        self.focus == FocusTarget::World
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_shell_starts_on_main_menu() {
        let shell = UiShell::new();
        assert_eq!(shell.screen(), Screen::MainMenu);
        assert!(shell.world_has_focus());
        assert!(shell.open_overlays().is_empty());
    }

    #[test]
    fn set_screen_clears_overlays() {
        let mut shell = UiShell::new();
        shell.set_screen(Screen::InGame);
        shell.open_overlay(Overlay::Inventory);
        shell.set_screen(Screen::Death);
        assert_eq!(shell.screen(), Screen::Death);
        assert!(shell.open_overlays().is_empty());
        assert!(shell.world_has_focus());
    }

    #[test]
    fn overlay_stack_and_focus() {
        let mut shell = UiShell::new();
        shell.set_screen(Screen::InGame);
        shell.open_overlay(Overlay::Inventory);
        assert_eq!(shell.focus(), FocusTarget::Panel(Overlay::Inventory));
        assert!(!shell.world_has_focus());

        shell.open_overlay(Overlay::CharacterSheet);
        assert_eq!(shell.focus(), FocusTarget::Panel(Overlay::CharacterSheet));

        shell.close_overlay(Overlay::CharacterSheet);
        assert_eq!(shell.focus(), FocusTarget::Panel(Overlay::Inventory));

        shell.close_overlay(Overlay::Inventory);
        assert!(shell.world_has_focus());
    }

    #[test]
    fn toggle_overlay() {
        let mut shell = UiShell::new();
        shell.set_screen(Screen::InGame);
        shell.toggle_overlay(Overlay::QuestJournal);
        assert!(shell.is_overlay_open(Overlay::QuestJournal));
        shell.toggle_overlay(Overlay::QuestJournal);
        assert!(!shell.is_overlay_open(Overlay::QuestJournal));
    }

    #[test]
    fn duplicate_open_brings_to_front() {
        let mut shell = UiShell::new();
        shell.open_overlay(Overlay::Inventory);
        shell.open_overlay(Overlay::CharacterSheet);
        shell.open_overlay(Overlay::Inventory);
        assert_eq!(shell.open_overlays().len(), 2);
        assert_eq!(*shell.open_overlays().last().expect("last"), Overlay::Inventory);
    }
}
