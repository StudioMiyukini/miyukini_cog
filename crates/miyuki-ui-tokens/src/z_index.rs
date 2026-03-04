// @id: MUIT-ZIndex @do: z-index-scale @role: token @layer: 5 @human: miyuk

/// Z-index scale for stacking context management.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ZIndexScale {
    /// 0 -- normal content.
    pub base: i32,
    /// 100 -- dropdowns, popovers.
    pub dropdown: i32,
    /// 200 -- sticky headers, nav bars.
    pub sticky: i32,
    /// 300 -- modal backdrop.
    pub overlay: i32,
    /// 400 -- modal content.
    pub modal: i32,
    /// 500 -- toast notifications.
    pub notification: i32,
    /// 600 -- tooltips (always on top).
    pub tooltip: i32,
}

impl ZIndexScale {
    /// Standard z-index scale.
    pub const fn standard() -> Self {
        Self {
            base: 0,
            dropdown: 100,
            sticky: 200,
            overlay: 300,
            modal: 400,
            notification: 500,
            tooltip: 600,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_index_order() {
        let z = ZIndexScale::standard();
        assert!(z.base < z.dropdown);
        assert!(z.dropdown < z.sticky);
        assert!(z.sticky < z.overlay);
        assert!(z.overlay < z.modal);
        assert!(z.modal < z.notification);
        assert!(z.notification < z.tooltip);
    }

    #[test]
    fn test_z_index_values() {
        let z = ZIndexScale::standard();
        assert_eq!(z.base, 0);
        assert_eq!(z.dropdown, 100);
        assert_eq!(z.sticky, 200);
        assert_eq!(z.overlay, 300);
        assert_eq!(z.modal, 400);
        assert_eq!(z.notification, 500);
        assert_eq!(z.tooltip, 600);
    }
}
