// @id: MGE-Platform-Input @do: input-mapping @role: back-end @layer: 1 @human: miyuk
//! Input event types for platform-agnostic input handling.

/// Platform-agnostic keyboard key codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum KeyCode {
    // Movement
    /// W key.
    W,
    /// A key.
    A,
    /// S key.
    S,
    /// D key.
    D,
    // Action
    /// Space bar.
    Space,
    /// Shift key.
    Shift,
    /// Control key.
    Ctrl,
    /// Alt key.
    Alt,
    // Navigation
    /// Escape key.
    Escape,
    /// Tab key.
    Tab,
    /// Enter / Return key.
    Enter,
    // Function keys
    /// F1 key.
    F1,
    /// F2 key.
    F2,
    /// F3 key.
    F3,
    /// F4 key.
    F4,
    /// F5 key.
    F5,
    /// F6 key.
    F6,
    /// F7 key.
    F7,
    /// F8 key.
    F8,
    /// F9 key.
    F9,
    /// F10 key.
    F10,
    /// F11 key.
    F11,
    /// F12 key.
    F12,
    // Number row
    /// 0 key.
    Num0,
    /// 1 key.
    Num1,
    /// 2 key.
    Num2,
    /// 3 key.
    Num3,
    /// 4 key.
    Num4,
    /// 5 key.
    Num5,
    /// 6 key.
    Num6,
    /// 7 key.
    Num7,
    /// 8 key.
    Num8,
    /// 9 key.
    Num9,
    // Letters
    /// B key.
    B,
    /// C key.
    C,
    /// E key.
    E,
    /// F key.
    F,
    /// G key.
    G,
    /// H key.
    H,
    /// I key.
    I,
    /// J key.
    J,
    /// K key.
    K,
    /// L key.
    L,
    /// M key.
    M,
    /// N key.
    N,
    /// O key.
    O,
    /// P key.
    P,
    /// Q key.
    Q,
    /// R key.
    R,
    /// T key.
    T,
    /// U key.
    U,
    /// V key.
    V,
    /// X key.
    X,
    /// Y key.
    Y,
    /// Z key.
    Z,
    // Arrows
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    // Other
    /// Unknown or unmapped key.
    Unknown,
}

/// Mouse buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button (scroll wheel click).
    Middle,
    /// Other mouse button identified by index.
    Other(u8),
}

/// Platform-agnostic input events.
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// A keyboard key was pressed.
    KeyDown {
        /// The key that was pressed.
        key: KeyCode,
    },
    /// A keyboard key was released.
    KeyUp {
        /// The key that was released.
        key: KeyCode,
    },
    /// Mouse cursor moved to position (pixels from top-left).
    MouseMove {
        /// Horizontal position in pixels.
        x: f64,
        /// Vertical position in pixels.
        y: f64,
    },
    /// Mouse button state changed.
    MouseButtonEvent {
        /// The mouse button involved.
        button: MouseButton,
        /// Whether the button is pressed (`true`) or released (`false`).
        pressed: bool,
    },
    /// Mouse wheel scrolled (positive = up).
    MouseWheel {
        /// Scroll delta (positive is up/forward).
        delta: f32,
    },
    /// Window close requested.
    WindowClose,
    /// Window resized.
    WindowResize {
        /// New window width in pixels.
        width: u32,
        /// New window height in pixels.
        height: u32,
    },
    /// Window gained or lost focus.
    WindowFocus {
        /// Whether the window is now focused.
        focused: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keycode_eq() {
        assert_eq!(KeyCode::W, KeyCode::W);
        assert_ne!(KeyCode::W, KeyCode::A);
    }

    #[test]
    fn test_mouse_button_eq() {
        assert_ne!(MouseButton::Left, MouseButton::Right);
        assert_eq!(MouseButton::Left, MouseButton::Left);
    }

    #[test]
    fn test_input_event_clone() {
        let event = InputEvent::KeyDown { key: KeyCode::Space };
        let cloned = event.clone();
        match cloned {
            InputEvent::KeyDown { key } => assert_eq!(key, KeyCode::Space),
            _ => panic!("Expected KeyDown event after clone"),
        }
    }

    #[test]
    fn test_window_close_event() {
        let event = InputEvent::WindowClose;
        let cloned = event.clone();
        assert!(matches!(cloned, InputEvent::WindowClose));
    }

    #[test]
    fn test_mouse_wheel_event() {
        let event = InputEvent::MouseWheel { delta: 1.5 };
        let cloned = event.clone();
        match cloned {
            InputEvent::MouseWheel { delta } => {
                assert!((delta - 1.5).abs() < f32::EPSILON);
            }
            _ => panic!("Expected MouseWheel event"),
        }
    }
}
