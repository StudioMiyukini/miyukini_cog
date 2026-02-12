//! Module JayShop — commerce, point de vente et vente en ligne.
//!
//! Architecture : sidebar + contenu dynamique par section (JSH-A01..JSH-A24).
//! Données réelles via JayShopDb (conns.read().jayshop).
//!
//! @id: central.services.jayshop
//! @do: render_jayshop_service_views
//! @role: ui
//! @layer: ui
//! @human: Vue JayShop — Commerce et PoS

mod sidebar;
mod dashboard;

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

use sidebar::JayShopSidebar;
use dashboard::Dashboard;

// ── State ──────────────────────────────────────────────────────────────

/// Section active dans JayShop (écrans JSH-A01 à JSH-A24).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JayShopSection {
    /// Tableau de bord (JSH-A01).
    #[default]
    Dashboard,
    /// Liste produits (JSH-A02).
    Products,
    /// Formulaire produit (JSH-A03).
    ProductForm,
    /// Catégories (JSH-A04).
    Categories,
    /// Écran principal PoS (JSH-A05).
    Pos,
    /// Écran de paiement (JSH-A06).
    PosPayment,
    /// Configuration PoS (JSH-A07).
    PosConfig,
    /// Ouverture caisse (JSH-A08).
    CashOpen,
    /// Clôture caisse (JSH-A09).
    CashClose,
    /// Historique tickets (JSH-A10).
    Tickets,
    /// Détail ticket (JSH-A11).
    TicketDetail,
    /// Commandes en ligne (JSH-A12).
    Orders,
    /// Paramètres boutique (JSH-A13).
    Settings,
    /// Remises (JSH-A16).
    Discounts,
    /// Fichier client (JSH-A17).
    Customers,
    /// Taxes (JSH-A18).
    Taxes,
    /// Rapports détaillés (JSH-A19).
    Reports,
    /// Liste événements (JSH-A20).
    Events,
    /// Formulaire événement (JSH-A21).
    EventForm,
}

/// État local complet de la vue JayShop.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JayShopState {
    /// Section active.
    pub section: JayShopSection,
    /// ID du vendeur courant.
    pub seller_id: Option<String>,
    /// ID du ticket en cours de consultation.
    pub viewing_ticket_id: Option<String>,
    /// ID du produit en cours d'édition.
    pub editing_product_id: Option<String>,
    /// ID de l'événement en cours d'édition.
    pub editing_event_id: Option<String>,
}

// ── Composant racine ───────────────────────────────────────────────────

/// Vue racine du service JayShop.
#[component]
pub fn JayShopView() -> Element {
    let _app_state = use_app_state();
    let _conns = use_service_connections();
    let state = use_signal(JayShopState::default);

    rsx! {
        div {
            style: "display: flex; height: 100%;",

            // Sidebar
            JayShopSidebar { state: state }

            // Contenu principal
            main {
                style: "flex: 1; padding: 24px; overflow-y: auto;",

                match state.read().section {
                    JayShopSection::Dashboard => rsx! {
                        Dashboard { state: state }
                    },
                    JayShopSection::Products => rsx! {
                        PlaceholderView { title: "Produits", icon: "📦", description: "Liste des produits (catalogue JayXpose)" }
                    },
                    JayShopSection::ProductForm => rsx! {
                        PlaceholderView { title: "Formulaire produit", icon: "✏️", description: "Création / édition d'un produit" }
                    },
                    JayShopSection::Categories => rsx! {
                        PlaceholderView { title: "Catégories", icon: "🏷️", description: "Gestion des catégories de produits" }
                    },
                    JayShopSection::Pos => rsx! {
                        PlaceholderView { title: "Point de vente", icon: "🖥️", description: "Écran principal PoS avec grille de boutons" }
                    },
                    JayShopSection::PosPayment => rsx! {
                        PlaceholderView { title: "Paiement", icon: "💳", description: "Écran de paiement et rendu monnaie" }
                    },
                    JayShopSection::PosConfig => rsx! {
                        PlaceholderView { title: "Configuration PoS", icon: "⚙️", description: "Onglets, boutons, modes de paiement" }
                    },
                    JayShopSection::CashOpen => rsx! {
                        PlaceholderView { title: "Ouverture de caisse", icon: "🔓", description: "Saisie du fond de caisse initial" }
                    },
                    JayShopSection::CashClose => rsx! {
                        PlaceholderView { title: "Clôture de caisse", icon: "🔒", description: "Comptage espèces et synthèse" }
                    },
                    JayShopSection::Tickets => rsx! {
                        PlaceholderView { title: "Historique des tickets", icon: "🧾", description: "Liste des ventes et remboursements" }
                    },
                    JayShopSection::TicketDetail => rsx! {
                        PlaceholderView { title: "Détail ticket", icon: "📋", description: "Lignes, paiements, actions" }
                    },
                    JayShopSection::Orders => rsx! {
                        PlaceholderView { title: "Commandes en ligne", icon: "📬", description: "Commandes reçues depuis la boutique en ligne" }
                    },
                    JayShopSection::Settings => rsx! {
                        PlaceholderView { title: "Paramètres", icon: "⚙️", description: "Configuration de la boutique" }
                    },
                    JayShopSection::Discounts => rsx! {
                        PlaceholderView { title: "Remises", icon: "🏷️", description: "Remises pré-définies (% ou montant)" }
                    },
                    JayShopSection::Customers => rsx! {
                        PlaceholderView { title: "Fichier client", icon: "👥", description: "Gestion des clients" }
                    },
                    JayShopSection::Taxes => rsx! {
                        PlaceholderView { title: "Taxes", icon: "📊", description: "Taux de taxe (incluse/ajoutée)" }
                    },
                    JayShopSection::Reports => rsx! {
                        PlaceholderView { title: "Rapports", icon: "📈", description: "Rapports de vente détaillés" }
                    },
                    JayShopSection::Events => rsx! {
                        PlaceholderView { title: "Événements", icon: "🎪", description: "Participations aux événements et festivals" }
                    },
                    JayShopSection::EventForm => rsx! {
                        PlaceholderView { title: "Fiche événement", icon: "📝", description: "Création / édition d'un événement" }
                    },
                }
            }
        }
    }
}

// ── Placeholder ────────────────────────────────────────────────────────

/// Vue placeholder pour les sections non encore implémentées.
#[component]
fn PlaceholderView(title: &'static str, icon: &'static str, description: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100%; gap: 16px;",

            span {
                style: "font-size: 48px;",
                "{icon}"
            }
            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "{title}"
            }
            p {
                style: "font-size: 14px; color: {c.text_secondary}; text-align: center; max-width: 400px;",
                "{description}"
            }
            div {
                style: "padding: 8px 16px; background: {c.bg_hover}; border-radius: 8px; font-size: 12px; color: {c.text_muted};",
                "Phase suivante d'implémentation"
            }
        }
    }
}
