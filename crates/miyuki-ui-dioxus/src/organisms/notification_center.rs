// @id: MUID-NotificationCenter @do: notification-center-organism @role: component @layer: 6 @human: miyuk

//! Notification center organism for managing notifications.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     NotificationCenter {
//!         notifications: vec![
//!             Notification { id: "1".into(), message: "Build succeeded".into(), variant: NotifVariant::Success, timestamp: "2m ago".into() },
//!         ],
//!         on_dismiss: move |id| dismiss_notif(id),
//!         on_clear_all: move |_| clear_all(),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Notification variant (reusing semantics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotifVariant {
    /// Success notification.
    Success,
    /// Warning notification.
    Warning,
    /// Error notification.
    Error,
    /// Informational notification (default).
    #[default]
    Info,
}

/// A single notification.
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    /// Unique identifier.
    pub id: String,
    /// Notification message.
    pub message: String,
    /// Notification variant.
    pub variant: NotifVariant,
    /// Timestamp display string.
    pub timestamp: String,
}

/// Notification center organism.
#[component]
pub fn NotificationCenter(
    /// List of notifications.
    #[props(default)]
    notifications: Vec<Notification>,
    /// Dismiss single notification (receives id).
    #[props(default)]
    on_dismiss: EventHandler<String>,
    /// Clear all notifications.
    #[props(default)]
    on_clear_all: EventHandler<()>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_surface.to_css();
    let border_color = p.border_default.to_css();
    let title_color = p.text_high.to_css();
    let text_color = p.text_primary.to_css();
    let muted_color = p.text_muted.to_css();
    let accent_color = p.accent_primary.to_css();
    let radius = theme.radius.lg;

    let container_style = format!(
        "display: flex; flex-direction: column; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {radius}px; overflow: hidden; width: 360px; max-height: 480px;"
    );

    let header_style = format!(
        "display: flex; align-items: center; justify-content: space-between; \
         padding: 12px 16px; border-bottom: 1px solid {border_color};"
    );

    let count = notifications.len();
    let count_str = format!("{count}");
    let has_notifications = !notifications.is_empty();

    let clear_style = format!(
        "background: none; border: none; color: {accent_color}; \
         cursor: pointer; font-size: 12px; padding: 0;"
    );

    rsx! {
        div { style: "{container_style}",
            div { style: "{header_style}",
                div { style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "font-size: 15px; font-weight: 600; color: {title_color};", "Notifications" }
                    if has_notifications {
                        span { style: "font-size: 11px; background: {accent_color}; color: white; padding: 1px 6px; border-radius: 8px;", "{count_str}" }
                    }
                }
                if has_notifications {
                    button {
                        style: "{clear_style}",
                        onclick: move |_| on_clear_all.call(()),
                        "Clear all"
                    }
                }
            }
            div { style: "flex: 1; overflow-y: auto;",
                if has_notifications {
                    for notif in notifications.iter() {
                        {
                            let accent = match notif.variant {
                                NotifVariant::Success => p.success.to_css(),
                                NotifVariant::Warning => p.warning.to_css(),
                                NotifVariant::Error => p.error.to_css(),
                                NotifVariant::Info => p.info.to_css(),
                            };
                            let notif_id = notif.id.clone();
                            let notif_style = format!(
                                "display: flex; align-items: flex-start; gap: 8px; \
                                 padding: 10px 16px; border-left: 3px solid {accent}; \
                                 border-bottom: 1px solid {border_color};"
                            );
                            rsx! {
                                div { style: "{notif_style}",
                                    div { style: "flex: 1;",
                                        div { style: "font-size: 13px; color: {text_color};", "{notif.message}" }
                                        div { style: "font-size: 11px; color: {muted_color}; margin-top: 2px;", "{notif.timestamp}" }
                                    }
                                    button {
                                        style: "background: none; border: none; color: {muted_color}; cursor: pointer; font-size: 14px; padding: 0; line-height: 1;",
                                        onclick: move |_| on_dismiss.call(notif_id.clone()),
                                        "x"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { style: "padding: 32px; text-align: center; color: {muted_color}; font-size: 13px;",
                        "No notifications"
                    }
                }
            }
        }
    }
}
