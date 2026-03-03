// @id: MGE-Platform-InputMap @do: input-mapping @role: back-end @layer: 1 @human: miyuk
//! Mapping from winit key/mouse codes to MGE `InputEvent` types.

use winit::keyboard::KeyCode as WinitKeyCode;

use crate::input::{KeyCode, MouseButton};

/// Map a winit physical key code to an MGE key code.
#[must_use]
pub fn map_key_code(code: WinitKeyCode) -> KeyCode {
    match code {
        WinitKeyCode::KeyW => KeyCode::W,
        WinitKeyCode::KeyA => KeyCode::A,
        WinitKeyCode::KeyS => KeyCode::S,
        WinitKeyCode::KeyB => KeyCode::B,
        WinitKeyCode::KeyC => KeyCode::C,
        WinitKeyCode::KeyD => KeyCode::D,
        WinitKeyCode::KeyE => KeyCode::E,
        WinitKeyCode::KeyF => KeyCode::F,
        WinitKeyCode::KeyG => KeyCode::G,
        WinitKeyCode::KeyH => KeyCode::H,
        WinitKeyCode::KeyI => KeyCode::I,
        WinitKeyCode::KeyJ => KeyCode::J,
        WinitKeyCode::KeyK => KeyCode::K,
        WinitKeyCode::KeyL => KeyCode::L,
        WinitKeyCode::KeyM => KeyCode::M,
        WinitKeyCode::KeyN => KeyCode::N,
        WinitKeyCode::KeyO => KeyCode::O,
        WinitKeyCode::KeyP => KeyCode::P,
        WinitKeyCode::KeyQ => KeyCode::Q,
        WinitKeyCode::KeyR => KeyCode::R,
        WinitKeyCode::KeyT => KeyCode::T,
        WinitKeyCode::KeyU => KeyCode::U,
        WinitKeyCode::KeyV => KeyCode::V,
        WinitKeyCode::KeyX => KeyCode::X,
        WinitKeyCode::KeyY => KeyCode::Y,
        WinitKeyCode::KeyZ => KeyCode::Z,
        WinitKeyCode::Space => KeyCode::Space,
        WinitKeyCode::ShiftLeft | WinitKeyCode::ShiftRight => KeyCode::Shift,
        WinitKeyCode::ControlLeft | WinitKeyCode::ControlRight => KeyCode::Ctrl,
        WinitKeyCode::AltLeft | WinitKeyCode::AltRight => KeyCode::Alt,
        WinitKeyCode::Escape => KeyCode::Escape,
        WinitKeyCode::Tab => KeyCode::Tab,
        WinitKeyCode::Enter => KeyCode::Enter,
        WinitKeyCode::ArrowUp => KeyCode::Up,
        WinitKeyCode::ArrowDown => KeyCode::Down,
        WinitKeyCode::ArrowLeft => KeyCode::Left,
        WinitKeyCode::ArrowRight => KeyCode::Right,
        WinitKeyCode::F1 => KeyCode::F1,
        WinitKeyCode::F2 => KeyCode::F2,
        WinitKeyCode::F3 => KeyCode::F3,
        WinitKeyCode::F4 => KeyCode::F4,
        WinitKeyCode::F5 => KeyCode::F5,
        WinitKeyCode::F6 => KeyCode::F6,
        WinitKeyCode::F7 => KeyCode::F7,
        WinitKeyCode::F8 => KeyCode::F8,
        WinitKeyCode::F9 => KeyCode::F9,
        WinitKeyCode::F10 => KeyCode::F10,
        WinitKeyCode::F11 => KeyCode::F11,
        WinitKeyCode::F12 => KeyCode::F12,
        WinitKeyCode::Digit0 => KeyCode::Num0,
        WinitKeyCode::Digit1 => KeyCode::Num1,
        WinitKeyCode::Digit2 => KeyCode::Num2,
        WinitKeyCode::Digit3 => KeyCode::Num3,
        WinitKeyCode::Digit4 => KeyCode::Num4,
        WinitKeyCode::Digit5 => KeyCode::Num5,
        WinitKeyCode::Digit6 => KeyCode::Num6,
        WinitKeyCode::Digit7 => KeyCode::Num7,
        WinitKeyCode::Digit8 => KeyCode::Num8,
        WinitKeyCode::Digit9 => KeyCode::Num9,
        _ => KeyCode::Unknown,
    }
}

/// Map a winit mouse button to an MGE mouse button.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn map_mouse_button(button: winit::event::MouseButton) -> MouseButton {
    match button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Other(id) => MouseButton::Other(id as u8),
        _ => MouseButton::Other(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_movement_keys() {
        assert_eq!(map_key_code(WinitKeyCode::KeyW), KeyCode::W);
        assert_eq!(map_key_code(WinitKeyCode::KeyA), KeyCode::A);
        assert_eq!(map_key_code(WinitKeyCode::KeyS), KeyCode::S);
        assert_eq!(map_key_code(WinitKeyCode::KeyD), KeyCode::D);
    }

    #[test]
    fn test_map_modifier_keys() {
        assert_eq!(map_key_code(WinitKeyCode::ShiftLeft), KeyCode::Shift);
        assert_eq!(map_key_code(WinitKeyCode::ShiftRight), KeyCode::Shift);
        assert_eq!(map_key_code(WinitKeyCode::ControlLeft), KeyCode::Ctrl);
        assert_eq!(map_key_code(WinitKeyCode::ControlRight), KeyCode::Ctrl);
        assert_eq!(map_key_code(WinitKeyCode::AltLeft), KeyCode::Alt);
        assert_eq!(map_key_code(WinitKeyCode::AltRight), KeyCode::Alt);
    }

    #[test]
    fn test_map_arrow_keys() {
        assert_eq!(map_key_code(WinitKeyCode::ArrowUp), KeyCode::Up);
        assert_eq!(map_key_code(WinitKeyCode::ArrowDown), KeyCode::Down);
        assert_eq!(map_key_code(WinitKeyCode::ArrowLeft), KeyCode::Left);
        assert_eq!(map_key_code(WinitKeyCode::ArrowRight), KeyCode::Right);
    }

    #[test]
    fn test_map_function_keys() {
        assert_eq!(map_key_code(WinitKeyCode::F1), KeyCode::F1);
        assert_eq!(map_key_code(WinitKeyCode::F12), KeyCode::F12);
    }

    #[test]
    fn test_map_digit_keys() {
        assert_eq!(map_key_code(WinitKeyCode::Digit0), KeyCode::Num0);
        assert_eq!(map_key_code(WinitKeyCode::Digit9), KeyCode::Num9);
    }

    #[test]
    fn test_map_special_keys() {
        assert_eq!(map_key_code(WinitKeyCode::Space), KeyCode::Space);
        assert_eq!(map_key_code(WinitKeyCode::Escape), KeyCode::Escape);
        assert_eq!(map_key_code(WinitKeyCode::Tab), KeyCode::Tab);
        assert_eq!(map_key_code(WinitKeyCode::Enter), KeyCode::Enter);
    }

    #[test]
    fn test_map_unknown_key() {
        assert_eq!(map_key_code(WinitKeyCode::Backquote), KeyCode::Unknown);
    }

    #[test]
    fn test_map_mouse_buttons() {
        assert_eq!(
            map_mouse_button(winit::event::MouseButton::Left),
            MouseButton::Left
        );
        assert_eq!(
            map_mouse_button(winit::event::MouseButton::Right),
            MouseButton::Right
        );
        assert_eq!(
            map_mouse_button(winit::event::MouseButton::Middle),
            MouseButton::Middle
        );
    }

    #[test]
    fn test_map_mouse_button_other() {
        assert_eq!(
            map_mouse_button(winit::event::MouseButton::Other(4)),
            MouseButton::Other(4)
        );
    }

    #[test]
    fn test_map_all_letter_keys() {
        assert_eq!(map_key_code(WinitKeyCode::KeyE), KeyCode::E);
        assert_eq!(map_key_code(WinitKeyCode::KeyF), KeyCode::F);
        assert_eq!(map_key_code(WinitKeyCode::KeyG), KeyCode::G);
        assert_eq!(map_key_code(WinitKeyCode::KeyH), KeyCode::H);
        assert_eq!(map_key_code(WinitKeyCode::KeyI), KeyCode::I);
        assert_eq!(map_key_code(WinitKeyCode::KeyJ), KeyCode::J);
        assert_eq!(map_key_code(WinitKeyCode::KeyK), KeyCode::K);
        assert_eq!(map_key_code(WinitKeyCode::KeyL), KeyCode::L);
        assert_eq!(map_key_code(WinitKeyCode::KeyM), KeyCode::M);
        assert_eq!(map_key_code(WinitKeyCode::KeyN), KeyCode::N);
        assert_eq!(map_key_code(WinitKeyCode::KeyO), KeyCode::O);
        assert_eq!(map_key_code(WinitKeyCode::KeyP), KeyCode::P);
        assert_eq!(map_key_code(WinitKeyCode::KeyQ), KeyCode::Q);
        assert_eq!(map_key_code(WinitKeyCode::KeyR), KeyCode::R);
        assert_eq!(map_key_code(WinitKeyCode::KeyT), KeyCode::T);
        assert_eq!(map_key_code(WinitKeyCode::KeyU), KeyCode::U);
        assert_eq!(map_key_code(WinitKeyCode::KeyV), KeyCode::V);
        assert_eq!(map_key_code(WinitKeyCode::KeyX), KeyCode::X);
        assert_eq!(map_key_code(WinitKeyCode::KeyY), KeyCode::Y);
        assert_eq!(map_key_code(WinitKeyCode::KeyZ), KeyCode::Z);
    }
}
