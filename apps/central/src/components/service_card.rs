//! Carte d'un service (affichage dans la grille).

use dioxus::prelude::*;
use crate::state::{use_app_state, ServiceInfo};
use crate::theme::styles;

#[derive(Props, Clone, PartialEq)]
pub struct ServiceCardProps {
    pub service: ServiceInfo,
    #[props(default = false)]
    pub compact: bool,
}

#[component]
pub fn ServiceCard(props: ServiceCardProps) -> Element {
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();
    let service = props.service.clone();
    let service_for_click = service.clone();

    rsx! {
        div {
            style: "{styles::service_card(theme)}",
            onclick: move |_| {
                state.write().open_service(&service_for_click);
            },
            onmouseenter: move |_| {},

            div {
                style: "{styles::service_icon_large(theme)}",
                span { style: "filter: drop-shadow(0 4px 8px rgba(0,0,0,0.3));", "{service.icon}" }
            }

            div {
                style: "{styles::service_card_content(theme)}",

                h3 { style: "{styles::service_title(theme)}", "{service.name}" }

                if !props.compact {
                    p {
                        style: "color: {c.text_secondary}; font-size: 12px; line-height: 1.4; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden;",
                        "{service.description}"
                    }
                }

                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin-top: 8px;",

                    span {
                        style: "{styles::type_badge(theme, service.service_type)}",
                        span { style: "width: 6px; height: 6px; border-radius: 50%; background: {service.service_type.color()};" }
                        "{service.service_type.label()}"
                    }

                    div {
                        style: "display: flex; gap: 4px;",
                        if service.is_favorite { span { style: "font-size: 12px;", "⭐" } }
                        if service.is_installed {
                            span { style: "{styles::price_badge(theme, true)}", "Installé" }
                        } else {
                            span { style: "{styles::price_badge(theme, false)} color: {c.accent_blue};", "Installer" }
                        }
                    }
                }
            }
        }
    }
}
