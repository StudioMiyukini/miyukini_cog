// @id: MGE-Platform @do: platform-init @role: back-end @layer: 1 @human: miyuk
//! MGE platform: window creation, GPU context, input mapping, app runner.
#![deny(unsafe_code)]

pub mod app;
pub mod config;
pub mod error;
pub mod gpu;
pub mod input;
pub mod input_map;
pub mod window;

pub use app::{run, GameApp};
pub use config::PlatformConfig;
pub use error::PlatformError;
pub use gpu::{GpuContext, GpuFrame};
pub use input::{InputEvent, KeyCode, MouseButton};
pub use input_map::{map_key_code, map_mouse_button};
pub use window::WindowConfig;
