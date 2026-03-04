// @id: MUIE-TmplIndex @do: template-exports @role: exports @layer: 6 @human: miyuk

//! Template-level layouts -- full-screen compositions of organisms.
//!
//! Templates define the overall screen structure for different game states
//! (gameplay, menus, loading, dialog).

pub mod gameplay;
pub mod menu;
pub mod character_select;
pub mod loading;
pub mod lobby;
pub mod dialog;

// Re-exports
pub use gameplay::GameplayLayout;
pub use menu::MenuLayout;
pub use character_select::CharSelectLayout;
pub use loading::LoadingLayout;
pub use lobby::LobbyLayout;
pub use dialog::DialogLayout;
