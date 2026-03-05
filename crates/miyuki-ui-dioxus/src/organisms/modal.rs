// @id: MUID-Modal @do: modal-organism @role: component @layer: 6 @human: miyuk

//! Modal dialog organism with backdrop and configurable size.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Modal {
//!         title: "Confirm Action".to_string(),
//!         on_close: move |_| set_modal_open(false),
//!         p { "Are you sure you want to proceed?" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Modal dialog size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    /// Small modal (400px).
    Sm,
    /// Medium modal (560px, default).
    #[default]
    Md,
    /// Large modal (720px).
    Lg,
    /// Fullscreen modal.
    Fullscreen,
}

/// Modal dialog organism.
#[component]
pub fn Modal(
    /// Dialog title.
    title: String,
    /// Size variant.
    #[props(default)]
    size: ModalSize,
    /// Close handler.
    #[props(default)]
    on_close: EventHandler<()>,
    /// Optional action buttons (footer).
    #[props(default)]
    actions: Option<Element>,
    /// Modal body content.
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let z = theme.z_index;

    let backdrop_z = z.overlay;
    let modal_z = z.modal;
    let bg_color = p.bg_surface.to_css();
    let border_color = p.border_default.to_css();
    let title_color = p.text_high.to_css();
    let muted_color = p.text_muted.to_css();
    let text_color = p.text_primary.to_css();
    let radius = theme.radius.lg;
    let shadow = theme.shadow.xl.to_css();

    let max_w = match size {
        ModalSize::Sm => "400px",
        ModalSize::Md => "560px",
        ModalSize::Lg => "720px",
        ModalSize::Fullscreen => "100vw",
    };

    let max_h = match size {
        ModalSize::Fullscreen => "100vh",
        _ => "85vh",
    };

    let border_radius = match size {
        ModalSize::Fullscreen => "0",
        _ => &format!("{radius}px"),
    };

    let backdrop_style = format!(
        "position: fixed; inset: 0; \
         background: rgba(0, 0, 0, 0.6); \
         display: flex; align-items: center; justify-content: center; \
         z-index: {backdrop_z}; padding: 24px;"
    );

    let modal_style = format!(
        "display: flex; flex-direction: column; \
         width: 100%; max-width: {max_w}; max-height: {max_h}; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {border_radius}; box-shadow: {shadow}; \
         z-index: {modal_z}; overflow: hidden;"
    );

    let header_style = format!(
        "display: flex; align-items: center; justify-content: space-between; \
         padding: 16px 20px; border-bottom: 1px solid {border_color}; flex-shrink: 0;"
    );

    let body_style = format!("flex: 1; padding: 20px; overflow-y: auto; color: {text_color};");

    let has_actions = actions.is_some();
    let footer_style = format!(
        "display: flex; justify-content: flex-end; gap: 8px; \
         padding: 12px 20px; border-top: 1px solid {border_color}; flex-shrink: 0;"
    );

    let close_btn_style = format!(
        "background: none; border: none; color: {muted_color}; \
         cursor: pointer; font-size: 18px; padding: 4px; line-height: 1;"
    );

    rsx! {
        div {
            style: "{backdrop_style}",
            onclick: move |_| on_close.call(()),
            div {
                style: "{modal_style}",
                role: "dialog",
                aria_modal: "true",
                onclick: move |evt| evt.stop_propagation(),
                div { style: "{header_style}",
                    div { style: "font-size: 18px; font-weight: 600; color: {title_color};", "{title}" }
                    button {
                        style: "{close_btn_style}",
                        onclick: move |_| on_close.call(()),
                        aria_label: "Close",
                        "x"
                    }
                }
                div { style: "{body_style}",
                    {children}
                }
                if has_actions {
                    div { style: "{footer_style}",
                        {actions}
                    }
                }
            }
        }
    }
}
