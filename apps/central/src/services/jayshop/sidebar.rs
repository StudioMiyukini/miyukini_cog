//! Sidebar JayShop — navigation par section (JSH-A01..JSH-A24).
//!
//! Utilise le composant générique ServiceSidebar.
//!
//! @id: central.services.jayshop.sidebar
//! @do: render_jayshop_sidebar_navigation
//! @role: ui
//! @layer: ui

use dioxus::prelude::*;
use crate::components::{ServiceSidebar, SidebarSection, SidebarFooter};
use crate::data::{use_service_connections, db_spawn};
use super::{JayShopSection, JayShopState};

/// Sidebar JayShop — navigation commerce et PoS.
#[component]
pub fn JayShopSidebar(state: Signal<JayShopState>) -> Element {
    let conns = use_service_connections();
    let seller_id = state.read().seller_id.clone();

    // Compteurs réels depuis la DB via use_resource + db_spawn
    let counts = use_resource(move || {
        let conns_arc = conns.read().clone();
        let sid = seller_id.clone();
        async move {
            if let Some(id) = sid {
                db_spawn(move || {
                    let db = &conns_arc.jayshop;
                    let tickets = db.tickets_by_seller(&id).map(|v| v.len()).unwrap_or(0);
                    let customers = 0_usize; // TODO: implémenter customers_by_seller
                    let events = db.events_by_seller(&id).map(|v| v.len()).unwrap_or(0);
                    (tickets, customers, events)
                }).await.unwrap_or((0, 0, 0))
            } else {
                (0, 0, 0)
            }
        }
    });

    let (ticket_count, customer_count, event_count) = match counts.read_unchecked().as_ref() {
        Some(c) => *c,
        None => (0usize, 0usize, 0usize),
    };

    // Section active courante
    let current_section = state.read().section;
    let active_id = match current_section {
        JayShopSection::Dashboard => "dashboard",
        JayShopSection::Products | JayShopSection::ProductForm | JayShopSection::Categories => "products",
        JayShopSection::Pos | JayShopSection::PosPayment => "pos",
        JayShopSection::PosConfig => "pos_config",
        JayShopSection::CashOpen | JayShopSection::CashClose => "cash",
        JayShopSection::Tickets | JayShopSection::TicketDetail => "tickets",
        JayShopSection::Orders => "orders",
        JayShopSection::Discounts => "discounts",
        JayShopSection::Taxes => "taxes",
        JayShopSection::Customers => "customers",
        JayShopSection::Reports => "reports",
        JayShopSection::Events | JayShopSection::EventForm => "events",
        JayShopSection::Settings => "settings",
    };

    // Sections principales
    let sections = vec![
        SidebarSection::new("dashboard", "📊", "Dashboard"),
        SidebarSection::new("pos", "🖥️", "Point de vente"),
        SidebarSection::new("tickets", "🧾", "Tickets").with_count(ticket_count),
        SidebarSection::new("products", "📦", "Produits"),
        SidebarSection::new("events", "🎪", "Événements").with_count(event_count),
        SidebarSection::new("customers", "👥", "Clients").with_count(customer_count),
        SidebarSection::new("reports", "📈", "Rapports"),
    ];

    // Sections secondaires (configuration)
    let secondary = vec![
        SidebarSection::new("cash", "💰", "Caisse"),
        SidebarSection::new("pos_config", "⚙️", "Config PoS"),
        SidebarSection::new("discounts", "🏷️", "Remises"),
        SidebarSection::new("taxes", "📊", "Taxes"),
        SidebarSection::new("settings", "🔧", "Paramètres"),
    ];

    rsx! {
        ServiceSidebar {
            title: "JayShop",
            subtitle: "Commerce & PoS",
            icon: "🛒",
            sections: sections,
            active_section: active_id.to_string(),
            on_section_change: move |section_id: String| {
                let new_section = match section_id.as_str() {
                    "dashboard" => JayShopSection::Dashboard,
                    "pos" => JayShopSection::Pos,
                    "tickets" => JayShopSection::Tickets,
                    "products" => JayShopSection::Products,
                    "events" => JayShopSection::Events,
                    "customers" => JayShopSection::Customers,
                    "reports" => JayShopSection::Reports,
                    "cash" => JayShopSection::CashOpen,
                    "pos_config" => JayShopSection::PosConfig,
                    "discounts" => JayShopSection::Discounts,
                    "taxes" => JayShopSection::Taxes,
                    "orders" => JayShopSection::Orders,
                    "settings" => JayShopSection::Settings,
                    _ => JayShopSection::Dashboard,
                };
                state.write().section = new_section;
            },
            secondary_sections: secondary,
            footer: SidebarFooter::None,
        }
    }
}
