// @id: MUIT-Animation @do: animation-tokens @role: token @layer: 5 @human: miyuk

/// Transition duration in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransitionDuration {
    /// Duration in milliseconds.
    pub ms: u32,
}

impl TransitionDuration {
    /// Create a new transition duration.
    pub const fn new(ms: u32) -> Self {
        Self { ms }
    }

    /// CSS duration string (e.g. "200ms").
    pub fn to_css(&self) -> String {
        format!("{}ms", self.ms)
    }

    /// Duration as seconds (for egui animations).
    pub fn to_secs(&self) -> f32 {
        #[allow(clippy::cast_precision_loss)]
        let secs = self.ms as f32 / 1000.0;
        secs
    }
}

/// Easing function specification (cubic bezier).
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Easing {
    /// First control point X.
    pub x1: f32,
    /// First control point Y.
    pub y1: f32,
    /// Second control point X.
    pub x2: f32,
    /// Second control point Y.
    pub y2: f32,
}

impl Easing {
    /// Create a new cubic-bezier easing.
    pub const fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Standard material motion easing.
    pub const STANDARD: Self = Self::new(0.4, 0.0, 0.2, 1.0);
    /// Entrance easing (deceleration).
    pub const ENTRANCE: Self = Self::new(0.0, 0.0, 0.2, 1.0);
    /// Exit easing (acceleration).
    pub const EXIT: Self = Self::new(0.4, 0.0, 1.0, 1.0);

    /// CSS cubic-bezier string.
    pub fn to_css(&self) -> String {
        format!(
            "cubic-bezier({}, {}, {}, {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

/// Transition scale for consistent motion.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransitionScale {
    /// 100ms -- color change, opacity.
    pub fast: TransitionDuration,
    /// 200ms -- hover, focus, toggle.
    pub normal: TransitionDuration,
    /// 300ms -- panel open, expand.
    pub slow: TransitionDuration,
    /// 400ms -- element appearance (fade-in, slide-in).
    pub entrance: TransitionDuration,
    /// Standard easing.
    pub easing_default: Easing,
    /// Entrance easing.
    pub easing_entrance: Easing,
    /// Exit easing.
    pub easing_exit: Easing,
}

impl TransitionScale {
    /// Standard Miyukini transition scale.
    pub const fn standard() -> Self {
        Self {
            fast: TransitionDuration::new(100),
            normal: TransitionDuration::new(200),
            slow: TransitionDuration::new(300),
            entrance: TransitionDuration::new(400),
            easing_default: Easing::STANDARD,
            easing_entrance: Easing::ENTRANCE,
            easing_exit: Easing::EXIT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_to_css() {
        let d = TransitionDuration::new(200);
        assert_eq!(d.to_css(), "200ms");
    }

    #[test]
    fn test_duration_to_secs() {
        let d = TransitionDuration::new(200);
        assert!((d.to_secs() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn test_easing_standard_to_css() {
        let css = Easing::STANDARD.to_css();
        assert_eq!(css, "cubic-bezier(0.4, 0, 0.2, 1)");
    }

    #[test]
    fn test_transition_scale_durations_increasing() {
        let ts = TransitionScale::standard();
        assert!(ts.fast.ms < ts.normal.ms);
        assert!(ts.normal.ms < ts.slow.ms);
        assert!(ts.slow.ms < ts.entrance.ms);
    }
}
