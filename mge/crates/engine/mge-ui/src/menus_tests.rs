// @id: MGE-UI-MenusTests @do: menu-state-tests @role: front-end @layer: 3 @human: miyuk
//! Unit tests for menu state models and loot filter / stash grid.
//!
//! All tests are pure Rust -- no egui `Context`, no wgpu device required.

#[cfg(test)]
mod menus_tests {
    use crate::inventory::{LootFilter, StashGrid, StashSlot, STASH_HEIGHT, STASH_WIDTH};
    use crate::menus::char_select::{CharSelectState, CharacterSlot, CreateCharState};
    use crate::menus::main_menu::{MainMenuAction, MainMenuState};
    use crate::menus::pause_menu::PauseMenuState;

    // -----------------------------------------------------------------------
    // Main menu tests
    // -----------------------------------------------------------------------

    /// MainMenuState::new() must produce exactly 5 buttons.
    #[test]
    fn main_menu_buttons() {
        let state = MainMenuState::new();
        assert_eq!(
            state.buttons.len(),
            5,
            "main menu must have exactly 5 buttons, got {}",
            state.buttons.len()
        );

        // Verify the expected order.
        assert_eq!(state.buttons[0], MainMenuAction::SinglePlayer);
        assert_eq!(state.buttons[1], MainMenuAction::Multiplayer);
        assert_eq!(state.buttons[2], MainMenuAction::Options);
        assert_eq!(state.buttons[3], MainMenuAction::Credits);
        assert_eq!(state.buttons[4], MainMenuAction::Exit);
    }

    // -----------------------------------------------------------------------
    // Character select tests
    // -----------------------------------------------------------------------

    /// can_create() must return false when 8 slots are filled.
    #[test]
    fn char_select_max_8() {
        let mut state = CharSelectState::new();
        assert!(
            state.can_create(),
            "empty state must allow creation"
        );

        for i in 0..8 {
            state.slots.push(CharacterSlot {
                name: format!("Hero-{i}"),
                class: String::from("Necromancer"),
                level: 1,
            });
        }

        assert_eq!(state.slots.len(), 8);
        assert!(
            !state.can_create(),
            "can_create() must return false when 8 slots are occupied"
        );
    }

    // -----------------------------------------------------------------------
    // Pause menu tests
    // -----------------------------------------------------------------------

    /// toggle() must flip game_paused from false to true.
    #[test]
    fn pause_menu_stops_game() {
        let mut state = PauseMenuState::new();
        assert!(
            !state.game_paused,
            "fresh PauseMenuState must have game_paused = false"
        );

        state.toggle();
        assert!(
            state.game_paused,
            "first toggle must set game_paused = true"
        );

        state.toggle();
        assert!(
            !state.game_paused,
            "second toggle must set game_paused = false"
        );
    }

    // -----------------------------------------------------------------------
    // Character creation tests
    // -----------------------------------------------------------------------

    /// Names shorter than 2 chars must be rejected.
    /// Names longer than 24 chars must be rejected.
    /// Names containing HTML angle brackets must be rejected.
    #[test]
    fn create_char_name_validation() {
        let mut state = CreateCharState::new();

        // Too short.
        state.name = String::from("A");
        assert!(
            state.validate_name().is_err(),
            "single-char name must be rejected"
        );

        // Too long (25 chars).
        state.name = "A".repeat(25);
        assert!(
            state.validate_name().is_err(),
            "25-char name must be rejected"
        );

        // HTML injection attempt.
        state.name = String::from("<script>alert(1)</script>");
        assert!(
            state.validate_name().is_err(),
            "HTML angle brackets must be rejected"
        );

        // Valid name.
        state.name = String::from("Necro_King");
        assert!(
            state.validate_name().is_ok(),
            "10-char alphanumeric name must be accepted"
        );
    }

    /// Season 3 only allows the Necromancer class.
    #[test]
    fn create_char_necro_only_s3() {
        let state = CreateCharState::new();
        assert_eq!(
            state.available_classes.len(),
            1,
            "S3 must have exactly 1 available class"
        );
        assert_eq!(
            state.available_classes[0], "Necromancer",
            "S3 only class must be Necromancer"
        );
        assert_eq!(
            state.selected_class, "Necromancer",
            "default selected class must be Necromancer"
        );
    }

    // -----------------------------------------------------------------------
    // Loot filter tests (TASK-137)
    // -----------------------------------------------------------------------

    /// Default filter shows all qualities.
    #[test]
    fn loot_filter_show_all() {
        let filter = LootFilter::default();
        for quality in &["Normal", "Magic", "Rare", "Set", "Unique", "Gold"] {
            assert!(
                filter.should_show(quality),
                "default filter must show '{quality}'"
            );
        }
    }

    /// Toggling "Normal" off hides Normal items.
    #[test]
    fn loot_filter_hide_normal() {
        let mut filter = LootFilter::default();
        assert!(
            filter.should_show("Normal"),
            "Normal must be visible before toggle"
        );

        filter.toggle("Normal");
        assert!(
            !filter.should_show("Normal"),
            "Normal must be hidden after toggle"
        );

        // Other qualities remain visible.
        assert!(filter.should_show("Magic"), "Magic must still be visible");
        assert!(filter.should_show("Rare"), "Rare must still be visible");
    }

    // -----------------------------------------------------------------------
    // Stash grid tests (TASK-148)
    // -----------------------------------------------------------------------

    /// Stash grid dimensions must be 10x10 (100 cells).
    #[test]
    fn stash_10x10() {
        let grid = StashGrid::new();
        assert_eq!(grid.width, STASH_WIDTH, "stash width must be {STASH_WIDTH}");
        assert_eq!(grid.height, STASH_HEIGHT, "stash height must be {STASH_HEIGHT}");
        let capacity = grid.width * grid.height;
        assert_eq!(
            capacity, 100,
            "stash capacity must be 100 (10*10), got {capacity}"
        );
    }

    /// Placing an item outside of town must return an error.
    #[test]
    fn stash_town_only() {
        let mut grid = StashGrid::new();
        // in_town defaults to false.
        let slot = StashSlot {
            item_id: String::from("item-01"),
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        };
        let result = grid.place_item(slot);
        assert!(
            result.is_err(),
            "place_item outside town must fail"
        );
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("Not in town"),
            "error must mention 'Not in town', got '{err_msg}'"
        );
    }

    /// Placing an item in town must succeed, and removing it must return it.
    #[test]
    fn stash_place_and_remove() {
        let mut grid = StashGrid::new();
        grid.in_town = true;

        let slot = StashSlot {
            item_id: String::from("gem-01"),
            x: 3,
            y: 5,
            width: 2,
            height: 2,
        };
        grid.place_item(slot).expect("in-town placement must succeed");
        assert_eq!(grid.items.len(), 1);

        let removed = grid.remove_item(3, 5).expect("remove must return the slot");
        assert_eq!(removed.item_id, "gem-01");
        assert_eq!(grid.items.len(), 0);
    }

    /// Overlapping placements must be rejected.
    #[test]
    fn stash_overlap_rejected() {
        let mut grid = StashGrid::new();
        grid.in_town = true;

        let slot1 = StashSlot {
            item_id: String::from("sword-01"),
            x: 0,
            y: 0,
            width: 2,
            height: 3,
        };
        grid.place_item(slot1).expect("first placement must succeed");

        let slot2 = StashSlot {
            item_id: String::from("shield-01"),
            x: 1,
            y: 1,
            width: 2,
            height: 2,
        };
        let result = grid.place_item(slot2);
        assert!(
            result.is_err(),
            "overlapping placement must fail"
        );
    }

    /// Out-of-bounds placement must be rejected.
    #[test]
    fn stash_out_of_bounds() {
        let mut grid = StashGrid::new();
        grid.in_town = true;

        let slot = StashSlot {
            item_id: String::from("bow-01"),
            x: 9,
            y: 9,
            width: 2,
            height: 2,
        };
        let result = grid.place_item(slot);
        assert!(
            result.is_err(),
            "out-of-bounds placement must fail"
        );
    }
}
