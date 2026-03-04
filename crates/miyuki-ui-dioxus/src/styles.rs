// @id: MUID-Styles @do: css-conversion @role: util @layer: 6 @human: miyuk

//! CSS inline style helpers converting design tokens to CSS strings.
//!
//! These functions produce CSS property declarations ready for Dioxus `style`
//! attributes. All values come from [`miyuki_ui_tokens`] types.

use miyuki_ui_tokens::{FontFamily, FontSize, FontWeight, RadiusScale, Rgba, Shadow, SpacingScale};

/// Build an inline CSS background-color from an Rgba token.
pub fn bg(color: &Rgba) -> String {
    format!("background: {};", color.to_css())
}

/// Build an inline CSS color from an Rgba token.
pub fn fg(color: &Rgba) -> String {
    format!("color: {};", color.to_css())
}

/// Build border CSS.
pub fn border(width: f32, color: &Rgba) -> String {
    format!("border: {width}px solid {};", color.to_css())
}

/// Build border-radius CSS.
pub fn border_radius(value: f32) -> String {
    format!("border-radius: {};", RadiusScale::to_css(value))
}

/// Build padding CSS (all sides).
pub fn padding(value: f32) -> String {
    format!("padding: {};", SpacingScale::to_css_px(value))
}

/// Build padding CSS (vertical, horizontal).
pub fn padding_vh(v: f32, h: f32) -> String {
    format!(
        "padding: {} {};",
        SpacingScale::to_css_px(v),
        SpacingScale::to_css_px(h)
    )
}

/// Build gap CSS.
pub fn gap(value: f32) -> String {
    format!("gap: {};", SpacingScale::to_css_px(value))
}

/// Build font-family CSS.
pub fn font_family(family: &FontFamily) -> String {
    format!("font-family: {};", family.to_css())
}

/// Build font-size CSS with line-height.
pub fn font_size(size: &FontSize) -> String {
    format!(
        "font-size: {}; line-height: {};",
        size.to_css(),
        size.line_height_css()
    )
}

/// Build font-weight CSS.
pub fn font_weight(weight: FontWeight) -> String {
    format!("font-weight: {};", weight.value())
}

/// Build box-shadow CSS.
pub fn box_shadow(shadow: &Shadow) -> String {
    format!("box-shadow: {};", shadow.to_css())
}

/// Build a transition CSS string.
pub fn transition(property: &str, duration_ms: u32) -> String {
    format!("transition: {property} {duration_ms}ms;")
}

/// Combine multiple CSS inline declarations.
pub fn css(parts: &[&str]) -> String {
    let mut result = String::new();
    for part in parts {
        if !part.is_empty() {
            result.push_str(part);
            if !part.ends_with(';') {
                result.push(';');
            }
            result.push(' ');
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bg_produces_background() {
        let c = Rgba::new(10, 20, 30);
        let result = bg(&c);
        assert!(result.contains("background:"));
        assert!(result.contains("rgb(10, 20, 30)"));
    }

    #[test]
    fn test_fg_produces_color() {
        let c = Rgba::new(255, 128, 0);
        let result = fg(&c);
        assert!(result.contains("color:"));
    }

    #[test]
    fn test_css_combines_parts() {
        let result = css(&["color: red", "padding: 4px"]);
        assert!(result.contains("color: red;"));
        assert!(result.contains("padding: 4px;"));
    }

    #[test]
    fn test_css_does_not_double_semicolons() {
        let result = css(&["color: red;", "padding: 4px;"]);
        assert!(!result.contains(";;"));
    }

    #[test]
    fn test_border_radius_zero() {
        let result = border_radius(0.0);
        assert_eq!(result, "border-radius: 0;");
    }

    #[test]
    fn test_padding_vh() {
        let result = padding_vh(8.0, 16.0);
        assert!(result.contains("8px"));
        assert!(result.contains("16px"));
    }

    #[test]
    fn test_transition_format() {
        let result = transition("all", 200);
        assert_eq!(result, "transition: all 200ms;");
    }
}
