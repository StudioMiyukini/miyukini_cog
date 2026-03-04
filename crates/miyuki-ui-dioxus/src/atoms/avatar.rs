// @id: MUID-Avatar @do: avatar-atom @role: component @layer: 6 @human: miyuk

//! Avatar atom displaying a user image or initials fallback.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Avatar {
//!         src: Some("/img/user.png".to_string()),
//!         size: AvatarSize::Md,
//!         shape: AvatarShape::Circle,
//!     }
//!     Avatar {
//!         initials: Some("JD".to_string()),
//!         size: AvatarSize::Lg,
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Avatar shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    /// Circular avatar.
    #[default]
    Circle,
    /// Rounded square avatar.
    Square,
}

/// Avatar size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    /// 24px.
    Xs,
    /// 32px.
    Sm,
    /// 40px (default).
    #[default]
    Md,
    /// 56px.
    Lg,
    /// 80px.
    Xl,
}

impl AvatarSize {
    /// Size in logical pixels.
    fn px(self) -> f32 {
        match self {
            Self::Xs => 24.0,
            Self::Sm => 32.0,
            Self::Md => 40.0,
            Self::Lg => 56.0,
            Self::Xl => 80.0,
        }
    }
}

/// Avatar component.
#[component]
pub fn Avatar(
    /// Image source URL.
    #[props(default)]
    src: Option<String>,
    /// Fallback initials (shown when no src).
    #[props(default)]
    initials: Option<String>,
    /// Shape variant.
    #[props(default)]
    shape: AvatarShape,
    /// Size variant.
    #[props(default)]
    size: AvatarSize,
    /// Alt text for accessibility.
    #[props(default)]
    alt: Option<String>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let px = size.px();
    let radius = match shape {
        AvatarShape::Circle => format!("{px}px"),
        AvatarShape::Square => format!("{}px", theme.radius.lg),
    };
    let bg_color = p.accent_primary_subtle.to_css();
    let text_color = p.accent_primary.to_css();
    let font_px = px * 0.4;

    let container_style = format!(
        "display: inline-flex; align-items: center; justify-content: center; \
         width: {px}px; height: {px}px; border-radius: {radius}; \
         background: {bg_color}; color: {text_color}; \
         font-size: {font_px}px; font-weight: 600; \
         overflow: hidden; flex-shrink: 0; user-select: none;"
    );

    let has_src = src.is_some();
    let src_val = src.unwrap_or_default();
    let alt_val = alt.unwrap_or_default();
    let initials_val = initials.unwrap_or_else(|| "?".to_string());

    rsx! {
        div {
            style: "{container_style}",
            role: "img",
            aria_label: "{alt_val}",
            if has_src {
                img {
                    src: "{src_val}",
                    alt: "{alt_val}",
                    style: "width: 100%; height: 100%; object-fit: cover;",
                }
            } else {
                "{initials_val}"
            }
        }
    }
}
