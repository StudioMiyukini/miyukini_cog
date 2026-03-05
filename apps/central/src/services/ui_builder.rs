//! Miyukini UI Builder — Vitrine des assets UI (fenêtres, conteneurs, boutons, barres, curseurs).
//!
//! Les panneaux et barres utilisent des générateurs SVG paramétriques :
//! les bordures gardent leur épaisseur absolue, seul le contenu s'adapte.
//! Les curseurs restent en PNG pixel-art à taille fixe.
//!
//! @id: central.services.ui_builder
//! @do: display_ui_assets_showcase_with_interactive_preview
//! @role: ui
//! @layer: ui
//! @human: Vue vitrine interactive des assets UI Miyukini (panneaux, boutons, barres, curseurs)

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::ui_assets;

/// Onglet interne du UI Builder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiBuilderTab {
    Fenetres,
    Conteneurs,
    Boutons,
    Barres,
    Curseurs,
    Maquettes,
}

#[allow(dead_code)]
impl UiBuilderTab {
    fn label(self) -> &'static str {
        match self {
            Self::Fenetres => "Fenêtres",
            Self::Conteneurs => "Conteneurs",
            Self::Boutons => "Boutons",
            Self::Barres => "Barres",
            Self::Curseurs => "Curseurs",
            Self::Maquettes => "Maquettes",
        }
    }

    fn all() -> &'static [UiBuilderTab] {
        &[
            UiBuilderTab::Fenetres,
            UiBuilderTab::Conteneurs,
            UiBuilderTab::Boutons,
            UiBuilderTab::Barres,
            UiBuilderTab::Curseurs,
            UiBuilderTab::Maquettes,
        ]
    }
}

fn tab_is_active(current: UiBuilderTab, tab: UiBuilderTab) -> bool {
    current == tab
}

#[derive(Props, Clone, PartialEq)]
struct TabButtonProps {
    label: &'static str,
    tab: UiBuilderTab,
    active_tab: Signal<UiBuilderTab>,
}

#[component]
fn TabButton(mut props: TabButtonProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let current = *props.active_tab.read();
    let is_active = tab_is_active(current, props.tab);
    let bg = if is_active { c.accent_blue } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border = if is_active { c.accent_blue } else { "transparent" };

    rsx! {
        button {
            style: "padding: 10px 18px; border: 2px solid {border}; border-radius: 6px; font-size: 13px; font-weight: 500; cursor: pointer; background: {bg}; color: {color}; transition: all 0.15s ease;",
            onclick: move |_| props.active_tab.set(props.tab),
            "{props.label}"
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  VUE PRINCIPALE
// ═══════════════════════════════════════════════════════════════════════

#[component]
pub fn UiBuilderView() -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let active_tab = use_signal(|| UiBuilderTab::Fenetres);

    rsx! {
        // Layout principal en colonne : en-tête fixe + onglets fixes + contenu défilable
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; overflow: hidden;",

            // Conteneur responsive avec max-width et centrage (partie fixe)
            div {
                style: "width: 100%; max-width: 1400px; margin: 0 auto; padding: 24px 24px 0 24px; flex-shrink: 0;",

                // En-tête
                div {
                    style: "margin-bottom: 16px; padding: 20px; border-radius: 12px; background: {c.bg_header}; border: 1px solid {c.border};",
                    h1 {
                        style: "font-size: 22px; font-weight: 600; color: {c.text_white}; margin: 0 0 6px 0;",
                        "Miyukini UI Builder"
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_secondary}; margin: 0;",
                        "Assets UI vectoriels : les bordures conservent leur épaisseur, seul le contenu s'adapte."
                    }
                }

                // Onglets internes
                div {
                    style: "display: flex; flex-wrap: wrap; gap: 4px; margin-bottom: 16px; padding: 8px; border-radius: 8px; background: {c.bg_secondary};",
                    TabButton { label: "Fenêtres", tab: UiBuilderTab::Fenetres, active_tab }
                    TabButton { label: "Conteneurs", tab: UiBuilderTab::Conteneurs, active_tab }
                    TabButton { label: "Boutons", tab: UiBuilderTab::Boutons, active_tab }
                    TabButton { label: "Barres", tab: UiBuilderTab::Barres, active_tab }
                    TabButton { label: "Curseurs", tab: UiBuilderTab::Curseurs, active_tab }
                    TabButton { label: "Maquettes", tab: UiBuilderTab::Maquettes, active_tab }
                }
            }

            // Contenu de l'onglet actif — le scroll est DANS le conteneur arrondi
            div {
                style: "flex: 1; min-height: 0; overflow: hidden; display: flex; flex-direction: column;",

                div {
                    style: "width: 100%; max-width: 1400px; margin: 0 auto; padding: 0 24px 24px 24px; flex: 1; min-height: 0; overflow: hidden; display: flex; flex-direction: column;",

                    div {
                        style: "flex: 1; min-height: 0; overflow-y: auto; overflow-x: hidden; padding: 24px; border-radius: 12px; background: {c.bg_secondary}; border: 1px solid {c.border};",
                        match *active_tab.read() {
                            UiBuilderTab::Fenetres => rsx! { TabFenetres {} },
                            UiBuilderTab::Conteneurs => rsx! { TabConteneurs {} },
                            UiBuilderTab::Boutons => rsx! { TabBoutons {} },
                            UiBuilderTab::Barres => rsx! { TabBarres {} },
                            UiBuilderTab::Curseurs => rsx! { TabCurseurs {} },
                            UiBuilderTab::Maquettes => rsx! { TabMaquettes {} },
                        }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — FENÊTRES
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabFenetres() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",

            // ── Adventure Pack (SVG paramétrique) ─────────────────────
            h2 { style: "font-size: 18px; color: {c.text_white};", "Fenêtres — Adventure Pack (SVG paramétrique)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 8px;",
                "Chaque fenêtre est générée en SVG à la taille demandée. Les bordures gardent 8px d'épaisseur."
            }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 24px; align-items: flex-start;",
                SvgPanel { title: "Panel Brown 200×120", svg: ui_assets::panel_brown(200, 120), w: 200, h: 120 }
                SvgPanel { title: "Panel Brown 500×200", svg: ui_assets::panel_brown(500, 200), w: 500, h: 200 }
            }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 24px; align-items: flex-start;",
                SvgPanel { title: "Panel Grey 350×160", svg: ui_assets::panel_grey(350, 160), w: 350, h: 160 }
                SvgPanel { title: "Panel Grey 500×250", svg: ui_assets::panel_grey(500, 250), w: 500, h: 250 }
            }

            div {
                style: "display: flex; flex-wrap: wrap; gap: 24px; align-items: flex-start;",
                SvgPanel { title: "Panel Brown Dark 300×120", svg: ui_assets::panel_brown_dark(300, 120), w: 300, h: 120 }
                SvgPanel { title: "Panel Grey Dark 300×120", svg: ui_assets::panel_grey_dark(300, 120), w: 300, h: 120 }
            }

            // ── Fantasy UI Borders ────────────────────────────────────
            hr { style: "border: none; border-top: 2px solid {c.border}; margin: 16px 0;" }
            h2 { style: "font-size: 18px; color: {c.text_white};", "Fenêtres — Fantasy UI Borders (Kenney)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 8px;",
                "Sprites 48×48 blancs sur transparent. Affichés en 2× (96px). Style RPG sombre / gothique."
            }

            // Panneaux (fond + bordure décorative)
            h3 { style: "font-size: 14px; color: {c.accent_blue}; border-bottom: 1px solid {c.border}; padding-bottom: 6px;", "Panels (fond + bordure)" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 12px;",
                FantasyTile { name: "000", src: ui_assets::fantasy_borders::PANEL_000.as_str() }
                FantasyTile { name: "001", src: ui_assets::fantasy_borders::PANEL_001.as_str() }
                FantasyTile { name: "002", src: ui_assets::fantasy_borders::PANEL_002.as_str() }
                FantasyTile { name: "003", src: ui_assets::fantasy_borders::PANEL_003.as_str() }
                FantasyTile { name: "004", src: ui_assets::fantasy_borders::PANEL_004.as_str() }
                FantasyTile { name: "005", src: ui_assets::fantasy_borders::PANEL_005.as_str() }
                FantasyTile { name: "006", src: ui_assets::fantasy_borders::PANEL_006.as_str() }
                FantasyTile { name: "007", src: ui_assets::fantasy_borders::PANEL_007.as_str() }
                FantasyTile { name: "008", src: ui_assets::fantasy_borders::PANEL_008.as_str() }
                FantasyTile { name: "010", src: ui_assets::fantasy_borders::PANEL_010.as_str() }
                FantasyTile { name: "015", src: ui_assets::fantasy_borders::PANEL_015.as_str() }
                FantasyTile { name: "016", src: ui_assets::fantasy_borders::PANEL_016.as_str() }
                FantasyTile { name: "020", src: ui_assets::fantasy_borders::PANEL_020.as_str() }
                FantasyTile { name: "024", src: ui_assets::fantasy_borders::PANEL_024.as_str() }
                FantasyTile { name: "028", src: ui_assets::fantasy_borders::PANEL_028.as_str() }
                FantasyTile { name: "031", src: ui_assets::fantasy_borders::PANEL_031.as_str() }
            }

            // Bordures (centre transparent)
            h3 { style: "font-size: 14px; color: {c.accent_blue}; border-bottom: 1px solid {c.border}; padding-bottom: 6px;", "Borders (centre transparent)" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 12px;",
                FantasyTile { name: "000", src: ui_assets::fantasy_borders::BORDER_000.as_str() }
                FantasyTile { name: "001", src: ui_assets::fantasy_borders::BORDER_001.as_str() }
                FantasyTile { name: "002", src: ui_assets::fantasy_borders::BORDER_002.as_str() }
                FantasyTile { name: "003", src: ui_assets::fantasy_borders::BORDER_003.as_str() }
                FantasyTile { name: "004", src: ui_assets::fantasy_borders::BORDER_004.as_str() }
                FantasyTile { name: "008", src: ui_assets::fantasy_borders::BORDER_008.as_str() }
                FantasyTile { name: "015", src: ui_assets::fantasy_borders::BORDER_015.as_str() }
                FantasyTile { name: "020", src: ui_assets::fantasy_borders::BORDER_020.as_str() }
                FantasyTile { name: "024", src: ui_assets::fantasy_borders::BORDER_024.as_str() }
                FantasyTile { name: "028", src: ui_assets::fantasy_borders::BORDER_028.as_str() }
                FantasyTile { name: "031", src: ui_assets::fantasy_borders::BORDER_031.as_str() }
            }

            // Bordures transparentes (fine)
            h3 { style: "font-size: 14px; color: {c.accent_blue}; border-bottom: 1px solid {c.border}; padding-bottom: 6px;", "Transparent Borders (fine)" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(120px, 1fr)); gap: 12px;",
                FantasyTile { name: "000", src: ui_assets::fantasy_borders::TBORDER_000.as_str() }
                FantasyTile { name: "004", src: ui_assets::fantasy_borders::TBORDER_004.as_str() }
                FantasyTile { name: "008", src: ui_assets::fantasy_borders::TBORDER_008.as_str() }
                FantasyTile { name: "015", src: ui_assets::fantasy_borders::TBORDER_015.as_str() }
                FantasyTile { name: "020", src: ui_assets::fantasy_borders::TBORDER_020.as_str() }
                FantasyTile { name: "024", src: ui_assets::fantasy_borders::TBORDER_024.as_str() }
                FantasyTile { name: "031", src: ui_assets::fantasy_borders::TBORDER_031.as_str() }
            }

            // Dividers
            h3 { style: "font-size: 14px; color: {c.accent_blue}; border-bottom: 1px solid {c.border}; padding-bottom: 6px;", "Dividers (séparateurs)" }
            div {
                style: "display: flex; flex-direction: column; gap: 16px; max-width: 600px;",
                FantasyDivider { name: "Divider 000", src: ui_assets::fantasy_borders::DIVIDER_000.as_str() }
                FantasyDivider { name: "Divider 001", src: ui_assets::fantasy_borders::DIVIDER_001.as_str() }
                FantasyDivider { name: "Divider 002", src: ui_assets::fantasy_borders::DIVIDER_002.as_str() }
                FantasyDivider { name: "Divider 003", src: ui_assets::fantasy_borders::DIVIDER_003.as_str() }
                FantasyDivider { name: "Divider 004", src: ui_assets::fantasy_borders::DIVIDER_004.as_str() }
                FantasyDivider { name: "Divider 005", src: ui_assets::fantasy_borders::DIVIDER_005.as_str() }
            }

            // Divider Fades
            h3 { style: "font-size: 14px; color: {c.accent_blue}; border-bottom: 1px solid {c.border}; padding-bottom: 6px;", "Divider Fades" }
            div {
                style: "display: flex; flex-direction: column; gap: 16px; max-width: 600px;",
                FantasyDivider { name: "Fade 000", src: ui_assets::fantasy_borders::DIVIDER_FADE_000.as_str() }
                FantasyDivider { name: "Fade 001", src: ui_assets::fantasy_borders::DIVIDER_FADE_001.as_str() }
                FantasyDivider { name: "Fade 002", src: ui_assets::fantasy_borders::DIVIDER_FADE_002.as_str() }
                FantasyDivider { name: "Fade 003", src: ui_assets::fantasy_borders::DIVIDER_FADE_003.as_str() }
                FantasyDivider { name: "Fade 004", src: ui_assets::fantasy_borders::DIVIDER_FADE_004.as_str() }
                FantasyDivider { name: "Fade 005", src: ui_assets::fantasy_borders::DIVIDER_FADE_005.as_str() }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — CONTENEURS (fenêtre avec contenu)
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabConteneurs() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            h2 { style: "font-size: 18px; color: {c.text_white};", "Conteneurs (fenêtre + contenu)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 8px;",
                "Exemples d'utilisation comme conteneurs de dialogue, inventaire ou info-bulle."
            }

            // Dialogue RPG style Sample.png
            div { style: "display: flex; flex-wrap: wrap; gap: 24px;",

                // Inventaire
                ContainerDemo {
                    panel_type: "brown",
                    width: 320, height: 200,
                    title: "Inventaire",
                    body: "You own a sword, a piece of string and five coins. Or buttons, probably buttons.",
                }

                // Warning / System
                ContainerDemo {
                    panel_type: "grey",
                    width: 320, height: 200,
                    title: "Warning",
                    body: "This is a sample system message with the grey panel style.",
                }

                // Info-bulle courte
                ContainerDemo {
                    panel_type: "brown",
                    width: 250, height: 120,
                    title: "Quête",
                    body: "Quest completed!",
                }

                // Cadre (pas de fond)
                div {
                    style: "position: relative; width: 300px; height: 180px;",
                    img {
                        src: "{ui_assets::svg_data_url(&ui_assets::border_grey(300, 180))}",
                        style: "position: absolute; top: 0; left: 0; width: 300px; height: 180px;",
                    }
                    div {
                        style: "position: relative; padding: 20px; color: {c.text_primary}; font-size: 13px; z-index: 1;",
                        h3 { style: "margin: 0 0 8px 0; color: {c.text_white};", "Cadre gris (border only)" }
                        p { "Ce cadre n'a pas de fond, on voit le background à travers." }
                    }
                }

                // Cadre marron
                div {
                    style: "position: relative; width: 300px; height: 180px;",
                    img {
                        src: "{ui_assets::svg_data_url(&ui_assets::border_brown(300, 180))}",
                        style: "position: absolute; top: 0; left: 0; width: 300px; height: 180px;",
                    }
                    div {
                        style: "position: relative; padding: 20px; color: {c.text_primary}; font-size: 13px; z-index: 1;",
                        h3 { style: "margin: 0 0 8px 0; color: {c.text_white};", "Cadre marron (border only)" }
                        p { "Bordure marron sans fond de remplissage." }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — BOUTONS
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabBoutons() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            h2 { style: "font-size: 18px; color: {c.text_white};", "Boutons (SVG vectoriels)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 8px;",
                "Boutons rectangulaires et ronds extraits du SVG original."
            }

            // Rectangulaires
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                h3 { style: "font-size: 14px; color: {c.text_white};", "Rectangulaires (48×24 natifs)" }
                div {
                    style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
                    SvgInline { svg: ui_assets::svg_fixed::BUTTON_BROWN, scale: 2.0, label: "Brown" }
                    SvgInline { svg: ui_assets::svg_fixed::BUTTON_GREY, scale: 2.0, label: "Grey" }
                    SvgInline { svg: ui_assets::svg_fixed::BUTTON_RED, scale: 2.0, label: "Red" }
                    SvgInline { svg: ui_assets::svg_fixed::BUTTON_BROWN_CLOSE, scale: 2.0, label: "Close Brown" }
                    SvgInline { svg: ui_assets::svg_fixed::BUTTON_GREY_CLOSE, scale: 2.0, label: "Close Grey" }
                    SvgInline { svg: ui_assets::svg_fixed::BUTTON_RED_CLOSE, scale: 2.0, label: "Close Red" }
                }
            }

            // Ronds
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                h3 { style: "font-size: 14px; color: {c.text_white};", "Ronds" }
                div {
                    style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
                    SvgInline { svg: ui_assets::svg_fixed::ROUND_BROWN, scale: 2.0, label: "Brown" }
                    SvgInline { svg: ui_assets::svg_fixed::ROUND_GREY, scale: 2.0, label: "Grey" }
                    SvgInline { svg: ui_assets::svg_fixed::ROUND_GREY_DETAILED_RED, scale: 2.0, label: "Red detail" }
                    SvgInline { svg: ui_assets::svg_fixed::ROUND_GREY_DETAILED_GREEN, scale: 2.0, label: "Green detail" }
                }
            }

            // Checkboxes
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                h3 { style: "font-size: 14px; color: {c.text_white};", "Checkboxes" }
                div {
                    style: "display: flex; align-items: center; gap: 16px; flex-wrap: wrap;",
                    SvgInline { svg: ui_assets::svg_fixed::CHECKBOX_BROWN_CHECKED, scale: 2.0, label: "Brown ✓" }
                    SvgInline { svg: ui_assets::svg_fixed::CHECKBOX_BROWN_EMPTY, scale: 2.0, label: "Brown ○" }
                    SvgInline { svg: ui_assets::svg_fixed::CHECKBOX_GREY_CHECKED, scale: 2.0, label: "Grey ✓" }
                    SvgInline { svg: ui_assets::svg_fixed::CHECKBOX_GREY_EMPTY, scale: 2.0, label: "Grey ○" }
                }
            }

            // Bannières
            div {
                style: "display: flex; flex-direction: column; gap: 16px;",
                h3 { style: "font-size: 14px; color: {c.text_white};", "Bannières" }
                div {
                    style: "display: flex; align-items: center; gap: 24px; flex-wrap: wrap;",
                    SvgInline { svg: ui_assets::svg_fixed::BANNER_HANGING, scale: 1.5, label: "Hanging" }
                    SvgInline { svg: ui_assets::svg_fixed::BANNER_MODERN, scale: 1.5, label: "Modern" }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — BARRES
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabBarres() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 32px;",
            h2 { style: "font-size: 18px; color: {c.text_white};", "Barres de progression (SVG paramétrique)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 8px;",
                "Barres pill-shape générées en SVG. Largeur et hauteur paramétrables."
            }

            // Exemple : Health / Mana / XP
            div {
                style: "display: flex; flex-direction: column; gap: 12px; max-width: 450px;",

                BarDemo { label: "Health", w: 300, h: 24, fill_pct: 75, color: "red" }
                BarDemo { label: "Mana", w: 300, h: 24, fill_pct: 50, color: "blue" }
                BarDemo { label: "XP", w: 300, h: 24, fill_pct: 90, color: "green" }
                BarDemo { label: "Stamina", w: 300, h: 24, fill_pct: 30, color: "orange" }
            }

            // Tailles variées
            div {
                style: "display: flex; flex-direction: column; gap: 12px; max-width: 450px; margin-top: 16px;",
                h3 { style: "font-size: 14px; color: {c.text_white};", "Tailles variées" }
                BarDemo { label: "Fin", w: 200, h: 12, fill_pct: 60, color: "blue" }
                BarDemo { label: "Normal", w: 300, h: 20, fill_pct: 45, color: "green" }
                BarDemo { label: "Épais", w: 400, h: 32, fill_pct: 80, color: "red" }
            }

            // Scrollbars originales
            div {
                style: "display: flex; flex-direction: column; gap: 16px; margin-top: 16px;",
                h3 { style: "font-size: 14px; color: {c.text_white};", "Scrollbars (SVG originaux)" }
                div {
                    style: "display: flex; gap: 24px; align-items: flex-start;",
                    SvgInline { svg: ui_assets::svg_fixed::SCROLLBAR_BROWN, scale: 2.0, label: "Brown" }
                    SvgInline { svg: ui_assets::svg_fixed::SCROLLBAR_GREY, scale: 2.0, label: "Grey" }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — CURSEURS
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabCurseurs() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 28px;",
            h2 { style: "font-size: 18px; color: {c.text_white};", "Curseurs — Cursor Pack (Basic)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 4px;",
                "Survolez chaque zone pour tester le curseur via CSS cursor:url(...)."
            }

            // ── Pointers ──────────────────────────────────────────
            CursorSection { title: "Pointers" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "A", src: ui_assets::cursors::POINTER_A.as_str() }
                CursorPreview { name: "B", src: ui_assets::cursors::POINTER_B.as_str() }
                CursorPreview { name: "C", src: ui_assets::cursors::POINTER_C.as_str() }
                CursorPreview { name: "D", src: ui_assets::cursors::POINTER_D.as_str() }
                CursorPreview { name: "E", src: ui_assets::cursors::POINTER_E.as_str() }
                CursorPreview { name: "F", src: ui_assets::cursors::POINTER_F.as_str() }
                CursorPreview { name: "G", src: ui_assets::cursors::POINTER_G.as_str() }
                CursorPreview { name: "H", src: ui_assets::cursors::POINTER_H.as_str() }
                CursorPreview { name: "I", src: ui_assets::cursors::POINTER_I.as_str() }
                CursorPreview { name: "J", src: ui_assets::cursors::POINTER_J.as_str() }
                CursorPreview { name: "K", src: ui_assets::cursors::POINTER_K.as_str() }
                CursorPreview { name: "L", src: ui_assets::cursors::POINTER_L.as_str() }
                CursorPreview { name: "Sci-Fi A", src: ui_assets::cursors::POINTER_SCIFI_A.as_str() }
                CursorPreview { name: "Sci-Fi B", src: ui_assets::cursors::POINTER_SCIFI_B.as_str() }
                CursorPreview { name: "Toon A", src: ui_assets::cursors::POINTER_TOON_A.as_str() }
                CursorPreview { name: "Toon B", src: ui_assets::cursors::POINTER_TOON_B.as_str() }
            }

            // ── Hands ─────────────────────────────────────────────
            CursorSection { title: "Hands" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Point", src: ui_assets::cursors::HAND_POINT.as_str() }
                CursorPreview { name: "Point E", src: ui_assets::cursors::HAND_POINT_E.as_str() }
                CursorPreview { name: "Point N", src: ui_assets::cursors::HAND_POINT_N.as_str() }
                CursorPreview { name: "Open", src: ui_assets::cursors::HAND_OPEN.as_str() }
                CursorPreview { name: "Closed", src: ui_assets::cursors::HAND_CLOSED.as_str() }
                CursorPreview { name: "Small", src: ui_assets::cursors::HAND_SMALL_POINT.as_str() }
                CursorPreview { name: "Thin", src: ui_assets::cursors::HAND_THIN_POINT.as_str() }
            }

            // ── Gauntlets ─────────────────────────────────────────
            CursorSection { title: "Gauntlets" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Default", src: ui_assets::cursors::GAUNTLET_DEFAULT.as_str() }
                CursorPreview { name: "Open", src: ui_assets::cursors::GAUNTLET_OPEN.as_str() }
                CursorPreview { name: "Point", src: ui_assets::cursors::GAUNTLET_POINT.as_str() }
            }

            // ── System ────────────────────────────────────────────
            CursorSection { title: "System" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Help", src: ui_assets::cursors::HELP.as_str() }
                CursorPreview { name: "Busy", src: ui_assets::cursors::BUSY.as_str() }
                CursorPreview { name: "Copy", src: ui_assets::cursors::COPY.as_str() }
                CursorPreview { name: "Disabled", src: ui_assets::cursors::DISABLED.as_str() }
                CursorPreview { name: "Alias", src: ui_assets::cursors::ALIAS.as_str() }
                CursorPreview { name: "Cogs", src: ui_assets::cursors::COGS.as_str() }
                CursorPreview { name: "Alert", src: ui_assets::cursors::EXCLAMATION.as_str() }
                CursorPreview { name: "Menu", src: ui_assets::cursors::MENU.as_str() }
                CursorPreview { name: "None", src: ui_assets::cursors::CURSOR_NONE.as_str() }
            }

            // ── Resize ────────────────────────────────────────────
            CursorSection { title: "Resize" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Horizontal", src: ui_assets::cursors::RESIZE_H.as_str() }
                CursorPreview { name: "Vertical", src: ui_assets::cursors::RESIZE_V.as_str() }
                CursorPreview { name: "Diagonal", src: ui_assets::cursors::RESIZE_DIAG.as_str() }
                CursorPreview { name: "Diag Mirror", src: ui_assets::cursors::RESIZE_DIAG_MIRROR.as_str() }
                CursorPreview { name: "Cross", src: ui_assets::cursors::RESIZE_CROSS.as_str() }
                CursorPreview { name: "Cross Diag", src: ui_assets::cursors::RESIZE_CROSS_DIAG.as_str() }
            }

            // ── Drawing ───────────────────────────────────────────
            CursorSection { title: "Drawing" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Pen", src: ui_assets::cursors::DRAWING_PEN.as_str() }
                CursorPreview { name: "Pencil", src: ui_assets::cursors::DRAWING_PENCIL.as_str() }
                CursorPreview { name: "Brush", src: ui_assets::cursors::DRAWING_BRUSH.as_str() }
                CursorPreview { name: "Bucket", src: ui_assets::cursors::DRAWING_BUCKET.as_str() }
                CursorPreview { name: "Eraser", src: ui_assets::cursors::DRAWING_ERASER.as_str() }
                CursorPreview { name: "Picker", src: ui_assets::cursors::DRAWING_PICKER.as_str() }
                CursorPreview { name: "Spray", src: ui_assets::cursors::DRAWING_SPRAY.as_str() }
            }

            // ── Game Tools ────────────────────────────────────────
            CursorSection { title: "Game Tools" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Sword A", src: ui_assets::cursors::TOOL_SWORD_A.as_str() }
                CursorPreview { name: "Sword B", src: ui_assets::cursors::TOOL_SWORD_B.as_str() }
                CursorPreview { name: "Axe", src: ui_assets::cursors::TOOL_AXE.as_str() }
                CursorPreview { name: "Hammer", src: ui_assets::cursors::TOOL_HAMMER.as_str() }
                CursorPreview { name: "Pickaxe", src: ui_assets::cursors::TOOL_PICKAXE.as_str() }
                CursorPreview { name: "Shovel", src: ui_assets::cursors::TOOL_SHOVEL.as_str() }
                CursorPreview { name: "Bow", src: ui_assets::cursors::TOOL_BOW.as_str() }
                CursorPreview { name: "Bomb", src: ui_assets::cursors::TOOL_BOMB.as_str() }
                CursorPreview { name: "Torch", src: ui_assets::cursors::TOOL_TORCH.as_str() }
                CursorPreview { name: "Wand", src: ui_assets::cursors::TOOL_WAND.as_str() }
                CursorPreview { name: "Wrench", src: ui_assets::cursors::TOOL_WRENCH.as_str() }
            }

            // ── Navigation ────────────────────────────────────────
            CursorSection { title: "Navigation" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "N", src: ui_assets::cursors::NAV_N.as_str() }
                CursorPreview { name: "S", src: ui_assets::cursors::NAV_S.as_str() }
                CursorPreview { name: "E", src: ui_assets::cursors::NAV_E.as_str() }
                CursorPreview { name: "W", src: ui_assets::cursors::NAV_W.as_str() }
                CursorPreview { name: "NE", src: ui_assets::cursors::NAV_NE.as_str() }
                CursorPreview { name: "NW", src: ui_assets::cursors::NAV_NW.as_str() }
                CursorPreview { name: "SE", src: ui_assets::cursors::NAV_SE.as_str() }
                CursorPreview { name: "SW", src: ui_assets::cursors::NAV_SW.as_str() }
            }

            // ── Zoom ──────────────────────────────────────────────
            CursorSection { title: "Zoom" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Zoom In", src: ui_assets::cursors::ZOOM_IN.as_str() }
                CursorPreview { name: "Zoom Out", src: ui_assets::cursors::ZOOM_OUT.as_str() }
                CursorPreview { name: "Reset", src: ui_assets::cursors::ZOOM_RESET.as_str() }
                CursorPreview { name: "Zoom", src: ui_assets::cursors::ZOOM.as_str() }
                CursorPreview { name: "Rotate CW", src: ui_assets::cursors::ROTATE_CW.as_str() }
                CursorPreview { name: "Rotate CCW", src: ui_assets::cursors::ROTATE_CCW.as_str() }
            }

            // ── Misc ──────────────────────────────────────────────
            CursorSection { title: "Miscellaneous" }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(110px, 1fr)); gap: 12px;",
                CursorPreview { name: "Look A", src: ui_assets::cursors::LOOK_A.as_str() }
                CursorPreview { name: "Look B", src: ui_assets::cursors::LOOK_B.as_str() }
                CursorPreview { name: "Look C", src: ui_assets::cursors::LOOK_C.as_str() }
                CursorPreview { name: "Look D", src: ui_assets::cursors::LOOK_D.as_str() }
                CursorPreview { name: "Lock", src: ui_assets::cursors::LOCK.as_str() }
                CursorPreview { name: "Unlocked", src: ui_assets::cursors::LOCK_UNLOCKED.as_str() }
                CursorPreview { name: "Target A", src: ui_assets::cursors::TARGET_A.as_str() }
                CursorPreview { name: "Target B", src: ui_assets::cursors::TARGET_B.as_str() }
                CursorPreview { name: "Boot", src: ui_assets::cursors::BOOT.as_str() }
                CursorPreview { name: "Door", src: ui_assets::cursors::DOOR.as_str() }
                CursorPreview { name: "Enter", src: ui_assets::cursors::DOOR_ENTER.as_str() }
                CursorPreview { name: "Exit", src: ui_assets::cursors::DOOR_EXIT.as_str() }
                CursorPreview { name: "Alert !", src: ui_assets::cursors::MARK_EXCLAMATION.as_str() }
                CursorPreview { name: "Question ?", src: ui_assets::cursors::MARK_QUESTION.as_str() }
                CursorPreview { name: "Stairs", src: ui_assets::cursors::STAIRS.as_str() }
                CursorPreview { name: "Steps", src: ui_assets::cursors::STEPS.as_str() }
                CursorPreview { name: "Cross L", src: ui_assets::cursors::CROSS_LARGE.as_str() }
                CursorPreview { name: "Cross S", src: ui_assets::cursors::CROSS_SMALL.as_str() }
            }

            // ═══════════════════════════════════════════════════════
            // Cursor Pixel Pack (16×16 pixel art)
            // ═══════════════════════════════════════════════════════
            hr { style: "border: none; border-top: 2px solid {c.border}; margin: 16px 0;" }
            h2 { style: "font-size: 18px; color: {c.text_white};", "Curseurs — Cursor Pixel Pack (16×16)" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-bottom: 4px;",
                "Curseurs pixel-art 16×16 (affichés en 2×). Survolez pour tester."
            }
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(100px, 1fr)); gap: 12px;",
                CursorPreview { name: "Pointer", src: ui_assets::cursors_pixel::POINTER.as_str() }
                CursorPreview { name: "Pointer Alt", src: ui_assets::cursors_pixel::POINTER_ALT.as_str() }
                CursorPreview { name: "Resize", src: ui_assets::cursors_pixel::RESIZE.as_str() }
                CursorPreview { name: "Hand", src: ui_assets::cursors_pixel::HAND.as_str() }
                CursorPreview { name: "Move", src: ui_assets::cursors_pixel::MOVE.as_str() }
                CursorPreview { name: "Crosshair", src: ui_assets::cursors_pixel::CROSSHAIR.as_str() }
                CursorPreview { name: "Zoom +", src: ui_assets::cursors_pixel::ZOOM_IN.as_str() }
                CursorPreview { name: "Zoom -", src: ui_assets::cursors_pixel::ZOOM_OUT.as_str() }
                CursorPreview { name: "Zoom Area", src: ui_assets::cursors_pixel::ZOOM_AREA.as_str() }
                CursorPreview { name: "Text", src: ui_assets::cursors_pixel::TEXT.as_str() }
                CursorPreview { name: "Grid", src: ui_assets::cursors_pixel::GRID.as_str() }
                CursorPreview { name: "Copy", src: ui_assets::cursors_pixel::COPY.as_str() }
                CursorPreview { name: "Disabled", src: ui_assets::cursors_pixel::DISABLED.as_str() }
                CursorPreview { name: "Cross", src: ui_assets::cursors_pixel::CROSS.as_str() }
                CursorPreview { name: "Lock", src: ui_assets::cursors_pixel::LOCK.as_str() }
                CursorPreview { name: "Arrow R", src: ui_assets::cursors_pixel::ARROW_RIGHT.as_str() }
                CursorPreview { name: "Arrow Up", src: ui_assets::cursors_pixel::ARROW_UP.as_str() }
                CursorPreview { name: "Flag", src: ui_assets::cursors_pixel::FLAG.as_str() }
                CursorPreview { name: "Pencil", src: ui_assets::cursors_pixel::PENCIL.as_str() }
                CursorPreview { name: "Bucket", src: ui_assets::cursors_pixel::BUCKET.as_str() }
                CursorPreview { name: "Eraser", src: ui_assets::cursors_pixel::ERASER.as_str() }
                CursorPreview { name: "Picker", src: ui_assets::cursors_pixel::PICKER.as_str() }
                CursorPreview { name: "Spray", src: ui_assets::cursors_pixel::SPRAY.as_str() }
                CursorPreview { name: "Rotate", src: ui_assets::cursors_pixel::ROTATE.as_str() }
                CursorPreview { name: "Nav N", src: ui_assets::cursors_pixel::NAV_N.as_str() }
                CursorPreview { name: "Nav E", src: ui_assets::cursors_pixel::NAV_E.as_str() }
                CursorPreview { name: "Diamond", src: ui_assets::cursors_pixel::DIAMOND.as_str() }
                CursorPreview { name: "Link", src: ui_assets::cursors_pixel::LINK.as_str() }
                CursorPreview { name: "Stairs", src: ui_assets::cursors_pixel::STAIRS.as_str() }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — MAQUETTES (GUI complètes)
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabMaquettes() -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 48px;",

            h2 { style: "font-size: 18px; color: {c.text_white}; margin: 0;", "Maquettes — GUI complètes" }
            p {
                style: "color: {c.text_secondary}; font-size: 13px; margin-top: -32px;",
                "Exemples complets d'interfaces construites avec les assets Miyukini UI."
            }

            // ── Mock 1 : Écran RPG Character ─────────────────────────
            MockRpgCharacter {}

            hr { style: "border: none; border-top: 2px solid {c.border}; margin: 0;" }

            // ── Mock 2 : Dialogue RPG ────────────────────────────────
            MockRpgDialogue {}

            hr { style: "border: none; border-top: 2px solid {c.border}; margin: 0;" }

            // ── Mock 3 : HUD de jeu ──────────────────────────────────
            MockGameHud {}

            hr { style: "border: none; border-top: 2px solid {c.border}; margin: 0;" }

            // ── Mock 4 : Menu principal ──────────────────────────────
            MockMainMenu {}

            hr { style: "border: none; border-top: 2px solid {c.border}; margin: 0;" }

            // ── Mock 5 : Boutique ────────────────────────────────────
            MockShop {}
        }
    }
}

// ─── Mock 1 : RPG Character Sheet (SVG paramétriques + Fantasy accents) ─

#[component]
fn MockRpgCharacter() -> Element {
    let c = use_app_state().read().current_theme.palette();

    // SVG paramétriques pour les cadres (eux s'adaptent à toute taille)
    let title_border = ui_assets::svg_data_url(&ui_assets::border_grey(420, 70));
    let inv_border = ui_assets::svg_data_url(&ui_assets::border_grey(380, 320));
    let equip_border = ui_assets::svg_data_url(&ui_assets::border_grey(380, 230));

    // Barres de stats
    let hp_track = ui_assets::svg_data_url(&ui_assets::bar_track(200, 16));
    let hp_fill = ui_assets::svg_data_url(&ui_assets::bar_red(160, 16));
    let mp_track = ui_assets::svg_data_url(&ui_assets::bar_track(200, 16));
    let mp_fill = ui_assets::svg_data_url(&ui_assets::bar_blue(120, 16));
    let xp_track = ui_assets::svg_data_url(&ui_assets::bar_track(200, 16));
    let xp_fill = ui_assets::svg_data_url(&ui_assets::bar_green(80, 16));
    let sta_track = ui_assets::svg_data_url(&ui_assets::bar_track(200, 16));
    let sta_fill = ui_assets::svg_data_url(&ui_assets::bar_orange(180, 16));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",
            h3 { style: "font-size: 16px; color: {c.accent_blue}; margin: 0;", "1. Fiche de personnage RPG" }
            p { style: "font-size: 12px; color: {c.text_muted}; margin: 0;", "SVG paramétriques pour les cadres, Fantasy borders en accents 48px natifs." }

            // Cadre principal de la maquette
            div {
                style: "background: #1a1e2e; border-radius: 12px; padding: 32px 24px 24px; display: flex; flex-direction: column; align-items: center; gap: 24px; border: 1px solid #2a3050;",

                // ── Bandeau titre centré (SVG border) + avatar en overlap ──
                div {
                    style: "position: relative; width: 420px; margin-bottom: 32px;",

                    // Cadre SVG paramétrique (s'adapte proprement)
                    div {
                        style: "position: relative; width: 420px; height: 70px;",
                        img {
                            src: "{title_border}",
                            style: "position: absolute; inset: 0; width: 420px; height: 70px;",
                        }
                        div {
                            style: "position: relative; z-index: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 70px;",
                            span { style: "font-size: 22px; font-weight: 700; color: #e8d5a0; text-shadow: 0 2px 8px rgba(0,0,0,0.6); letter-spacing: 3px;", "ALDRIC" }
                            span { style: "font-size: 11px; color: {c.text_muted}; letter-spacing: 1px;", "Paladin — Niveau 24" }
                        }
                    }

                    // Avatar en overlap (déborde sur le coin haut-gauche du cadre)
                    div {
                        style: "position: absolute; top: -24px; left: -24px; z-index: 2; width: 72px; height: 72px; border-radius: 50%; background: #222840; border: 3px solid #4a7aac; display: flex; align-items: center; justify-content: center; font-size: 32px; box-shadow: 0 4px 12px rgba(0,0,0,0.5);",
                        "🛡️"
                    }

                    // Coins décoratifs Fantasy (48px natifs, 2 coins)
                    img {
                        src: "{ui_assets::fantasy_borders::TBORDER_004.as_str()}",
                        style: "position: absolute; top: -8px; right: -8px; width: 48px; height: 48px; image-rendering: pixelated; opacity: 0.5;",
                    }
                    img {
                        src: "{ui_assets::fantasy_borders::TBORDER_004.as_str()}",
                        style: "position: absolute; bottom: -8px; left: 48px; width: 48px; height: 48px; image-rendering: pixelated; opacity: 0.5; transform: rotate(180deg);",
                    }
                }

                // ── Deux colonnes : stats | inventaire ──
                div {
                    style: "display: flex; gap: 24px; flex-wrap: wrap; width: 100%; justify-content: center;",

                    // ── Colonne gauche : stats + attributs ──
                    div {
                        style: "display: flex; flex-direction: column; gap: 16px; width: 300px;",

                        // Barres de stats
                        div {
                            style: "display: flex; flex-direction: column; gap: 6px; padding: 16px; background: #151828; border-radius: 8px; border: 1px solid #2a3050;",
                            MockStatBar { label: "HP", value: "160 / 200", track_url: hp_track, fill_url: hp_fill, fill_w: 160, total_w: 200, color: "#ff7366" }
                            MockStatBar { label: "MP", value: "120 / 200", track_url: mp_track, fill_url: mp_fill, fill_w: 120, total_w: 200, color: "#68b8ff" }
                            MockStatBar { label: "XP", value: "800 / 2000", track_url: xp_track, fill_url: xp_fill, fill_w: 80, total_w: 200, color: "#81d058" }
                            MockStatBar { label: "STA", value: "180 / 200", track_url: sta_track, fill_url: sta_fill, fill_w: 180, total_w: 200, color: "#ffbe5e" }
                        }

                        // Attributs
                        div {
                            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 4px; padding: 12px; background: #151828; border-radius: 8px; border: 1px solid #2a3050;",
                            MockAttr { name: "Force", value: "18" }
                            MockAttr { name: "Dextérité", value: "12" }
                            MockAttr { name: "Constitution", value: "16" }
                            MockAttr { name: "Intelligence", value: "10" }
                            MockAttr { name: "Sagesse", value: "14" }
                            MockAttr { name: "Charisme", value: "8" }
                        }

                        // Or
                        div {
                            style: "display: flex; align-items: center; justify-content: center; gap: 8px; padding: 8px 12px; background: #151828; border-radius: 6px; border: 1px solid #2a3050;",
                            span { style: "font-size: 18px;", "💰" }
                            span { style: "font-size: 16px; font-weight: 700; color: #ffbe5e;", "1,247 or" }
                        }
                    }

                    // ── Colonne droite : inventaire + équipement (dans des SVG borders) ──
                    div {
                        style: "display: flex; flex-direction: column; gap: 16px; width: 380px;",

                        // Inventaire dans un SVG border paramétrique
                        div {
                            style: "position: relative; width: 380px; min-height: 320px;",
                            img {
                                src: "{inv_border}",
                                style: "position: absolute; inset: 0; width: 380px; height: 320px;",
                            }
                            div {
                                style: "position: relative; z-index: 1; padding: 20px;",

                                // Titre avec petits accents Fantasy natifs (48px)
                                div {
                                    style: "display: flex; align-items: center; justify-content: center; gap: 10px; margin-bottom: 12px;",
                                    img {
                                        src: "{ui_assets::fantasy_borders::DIVIDER_FADE_002.as_str()}",
                                        style: "width: 48px; height: 16px; image-rendering: pixelated; opacity: 0.6;",
                                    }
                                    span { style: "font-size: 13px; font-weight: 700; color: #e8d5a0; letter-spacing: 2px;", "INVENTAIRE" }
                                    img {
                                        src: "{ui_assets::fantasy_borders::DIVIDER_FADE_002.as_str()}",
                                        style: "width: 48px; height: 16px; image-rendering: pixelated; opacity: 0.6; transform: scaleX(-1);",
                                    }
                                }

                                // Grille d'inventaire
                                div {
                                    style: "display: grid; grid-template-columns: repeat(5, 56px); gap: 6px; justify-content: center;",
                                    MockInvSlot { icon: "⚔️", count: "1" }
                                    MockInvSlot { icon: "🛡️", count: "1" }
                                    MockInvSlot { icon: "🧪", count: "5" }
                                    MockInvSlot { icon: "📜", count: "3" }
                                    MockInvSlot { icon: "💎", count: "12" }
                                    MockInvSlot { icon: "🍞", count: "8" }
                                    MockInvSlot { icon: "🗝️", count: "2" }
                                    MockInvSlot { icon: "🏹", count: "1" }
                                    MockInvSlot { icon: "💰", count: "247" }
                                    MockInvSlot { icon: "🔮", count: "1" }
                                    // Slots vides
                                    for _ in 0..5 {
                                        div {
                                            style: "width: 56px; height: 56px; background: #1a1e2e; border-radius: 6px; border: 1px dashed #2a3050;",
                                        }
                                    }
                                }
                            }
                        }

                        // Équipement dans un SVG border paramétrique
                        div {
                            style: "position: relative; width: 380px; min-height: 230px;",
                            img {
                                src: "{equip_border}",
                                style: "position: absolute; inset: 0; width: 380px; height: 230px;",
                            }
                            div {
                                style: "position: relative; z-index: 1; padding: 20px;",

                                // Titre avec petits accents Fantasy natifs (48px)
                                div {
                                    style: "display: flex; align-items: center; justify-content: center; gap: 10px; margin-bottom: 12px;",
                                    img {
                                        src: "{ui_assets::fantasy_borders::DIVIDER_FADE_003.as_str()}",
                                        style: "width: 48px; height: 16px; image-rendering: pixelated; opacity: 0.6;",
                                    }
                                    span { style: "font-size: 13px; font-weight: 700; color: #e8d5a0; letter-spacing: 2px;", "ÉQUIPEMENT" }
                                    img {
                                        src: "{ui_assets::fantasy_borders::DIVIDER_FADE_003.as_str()}",
                                        style: "width: 48px; height: 16px; image-rendering: pixelated; opacity: 0.6; transform: scaleX(-1);",
                                    }
                                }

                                div {
                                    style: "display: flex; flex-direction: column; gap: 6px;",
                                    MockEquipRow { slot: "Arme", item: "Épée de lumière +3", icon: "⚔️" }
                                    MockEquipRow { slot: "Armure", item: "Plate du gardien", icon: "🛡️" }
                                    MockEquipRow { slot: "Accessoire", item: "Amulette de sagesse", icon: "🔮" }
                                    MockEquipRow { slot: "Bottes", item: "Bottes de célérité", icon: "👢" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─── Mock 2 : Dialogue RPG (Adventure panels) ──────────────────────

#[component]
fn MockRpgDialogue() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let panel_bg = ui_assets::svg_data_url(&ui_assets::panel_brown(600, 340));
    let panel_choice = ui_assets::svg_data_url(&ui_assets::panel_grey(600, 220));
    let btn_brown_url = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN);
    let btn_red_url = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_RED);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",
            h3 { style: "font-size: 16px; color: {c.accent_blue}; margin: 0;", "2. Dialogue RPG" }
            p { style: "font-size: 12px; color: {c.text_muted}; margin: 0;", "Style Adventure Pack — panneaux marron/gris, boutons, curseurs main." }

            div {
                style: "background: #3a6a8c; border-radius: 12px; padding: 32px; display: flex; flex-direction: column; align-items: center; gap: 24px; border: 1px solid #4a8aac;",

                // NPC + dialogue
                div {
                    style: "display: flex; gap: 20px; align-items: flex-start; max-width: 650px; width: 100%;",

                    // Portrait NPC
                    div {
                        style: "flex-shrink: 0; width: 80px; height: 80px; border-radius: 50%; background: #2a4a6a; border: 3px solid #e8d5a0; display: flex; align-items: center; justify-content: center; font-size: 36px;",
                        "🧙"
                    }

                    // Bulle de dialogue (panneau brown)
                    div {
                        style: "position: relative; flex: 1; min-height: 160px;",
                        img {
                            src: "{panel_bg}",
                            style: "position: absolute; inset: 0; width: 100%; height: 100%;",
                        }
                        div {
                            style: "position: relative; padding: 20px 24px; z-index: 1;",
                            div {
                                style: "font-size: 13px; font-weight: 700; color: #e8d5a0; margin-bottom: 4px; letter-spacing: 1px;",
                                "ARCHIMAGE MAELIS"
                            }
                            // Séparateur : trait CSS simple (les dividers Fantasy sont des sprites, pas étirables)
                            div { style: "width: 160px; height: 1px; background: linear-gradient(90deg, #a3703a, transparent); margin-bottom: 8px;" }
                            p {
                                style: "font-size: 13px; color: #5a3a1a; line-height: 1.6; margin: 0;",
                                "Brave aventurier, le Cristal de Lune a été volé par les ombres du Val Obscur. Sans lui, notre cité perdra sa protection magique avant la prochaine pleine lune."
                            }
                            p {
                                style: "font-size: 13px; color: #5a3a1a; line-height: 1.6; margin: 8px 0 0 0; font-style: italic;",
                                "Accepterez-vous cette quête périlleuse ?"
                            }
                        }
                    }
                }

                // Choix du joueur (panneau gris)
                div {
                    style: "position: relative; max-width: 500px; width: 100%; min-height: 170px;",
                    img {
                        src: "{panel_choice}",
                        style: "position: absolute; inset: 0; width: 100%; height: 100%;",
                    }
                    div {
                        style: "position: relative; padding: 20px 24px; z-index: 1; display: flex; flex-direction: column; gap: 10px;",
                        span { style: "font-size: 12px; color: #1b2838; font-weight: 600; letter-spacing: 1px;", "VOTRE RÉPONSE" }

                        // Choix 1 — actif
                        div {
                            style: "display: flex; align-items: center; gap: 10px; padding: 8px 12px; background: rgba(26,159,255,0.15); border-radius: 6px; border: 1px solid #1a9fff; cursor: pointer;",
                            span { style: "font-size: 16px;", "▶" }
                            span { style: "font-size: 13px; color: #1b2838; font-weight: 500;", "J'accepte cette quête. Où dois-je aller ?" }
                        }
                        // Choix 2
                        div {
                            style: "display: flex; align-items: center; gap: 10px; padding: 8px 12px; border-radius: 6px; cursor: pointer; opacity: 0.7;",
                            span { style: "font-size: 16px; color: #888;", "▷" }
                            span { style: "font-size: 13px; color: #3a4a5a;", "Je ne suis pas encore prêt. Peut-être plus tard." }
                        }
                        // Choix 3
                        div {
                            style: "display: flex; align-items: center; gap: 10px; padding: 8px 12px; border-radius: 6px; cursor: pointer; opacity: 0.7;",
                            span { style: "font-size: 16px; color: #888;", "▷" }
                            span { style: "font-size: 13px; color: #3a4a5a;", "Quelle sera ma récompense ?" }
                        }
                    }
                }

                // Boutons d'action
                div {
                    style: "display: flex; gap: 16px;",
                    div {
                        style: "position: relative; width: 120px; height: 48px; cursor: pointer;",
                        img { src: "{btn_brown_url}", style: "width: 100%; height: 100%;" }
                        span { style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; color: #5a3a1a;", "Valider" }
                    }
                    div {
                        style: "position: relative; width: 120px; height: 48px; cursor: pointer; opacity: 0.6;",
                        img { src: "{btn_red_url}", style: "width: 100%; height: 100%;" }
                        span { style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; color: #5a2020;", "Refuser" }
                    }
                }
            }
        }
    }
}

// ─── Mock 3 : Game HUD (overlay style) ──────────────────────────────

#[component]
fn MockGameHud() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",
            h3 { style: "font-size: 16px; color: {c.accent_blue}; margin: 0;", "3. HUD de jeu (overlay)" }
            p { style: "font-size: 12px; color: {c.text_muted}; margin: 0;", "Affichage tête-haute typique : barres de vie, actions rapides, mini-carte, notifications." }

            // Zone de jeu simulée
            div {
                style: "position: relative; background: linear-gradient(135deg, #1a3a2a 0%, #0a1a15 50%, #1a2a1a 100%); border-radius: 12px; height: 480px; border: 1px solid #2a4a3a; overflow: hidden;",

                // Fond grille simulé
                div {
                    style: "position: absolute; inset: 0; background-image: radial-gradient(circle, #2a4a3a 1px, transparent 1px); background-size: 40px 40px; opacity: 0.3;",
                }

                // ── Top-left : barres de stats ──
                div {
                    style: "position: absolute; top: 16px; left: 16px; display: flex; flex-direction: column; gap: 6px;",
                    HudBar { label: "HP", fill_pct: 72, color: "red", w: 200, h: 16 }
                    HudBar { label: "MP", fill_pct: 45, color: "blue", w: 200, h: 16 }
                    HudBar { label: "XP", fill_pct: 88, color: "green", w: 160, h: 12 }
                }

                // ── Top-left : nom + niveau ──
                div {
                    style: "position: absolute; top: 82px; left: 16px; padding: 6px 12px; background: rgba(0,0,0,0.6); border-radius: 6px; border: 1px solid #2a4a3a;",
                    span { style: "font-size: 12px; color: #e8d5a0; font-weight: 600;", "Aldric" }
                    span { style: "font-size: 11px; color: {c.text_muted}; margin-left: 8px;", "Nv. 24" }
                }

                // ── Top-right : mini carte ──
                div {
                    style: "position: absolute; top: 16px; right: 16px; width: 140px; height: 140px; border-radius: 50%; background: rgba(0,0,0,0.5); border: 2px solid #4a6a5a; display: flex; align-items: center; justify-content: center; overflow: hidden;",
                    // Grille minimap
                    div {
                        style: "width: 120px; height: 120px; border-radius: 50%; background-image: radial-gradient(circle, #3a5a4a 1px, transparent 1px); background-size: 12px 12px; position: relative;",
                        // Joueur au centre
                        div {
                            style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: 8px; height: 8px; background: #1a9fff; border-radius: 50%; box-shadow: 0 0 8px #1a9fff;",
                        }
                        // Quelques POI
                        div { style: "position: absolute; top: 25%; left: 60%; width: 6px; height: 6px; background: #ff7366; border-radius: 50%;" }
                        div { style: "position: absolute; top: 70%; left: 35%; width: 6px; height: 6px; background: #ffbe5e; border-radius: 50%;" }
                        div { style: "position: absolute; top: 40%; left: 20%; width: 6px; height: 6px; background: #81d058; border-radius: 50%;" }
                    }
                }

                // ── Bottom-center : barre d'actions ──
                div {
                    style: "position: absolute; bottom: 16px; left: 50%; transform: translateX(-50%); display: flex; gap: 6px; padding: 8px 12px; background: rgba(0,0,0,0.7); border-radius: 10px; border: 1px solid #3a5a4a;",
                    HudAction { hotkey: "1", icon: "⚔️", active: false }
                    HudAction { hotkey: "2", icon: "🛡️", active: true }
                    HudAction { hotkey: "3", icon: "🧪", active: false }
                    HudAction { hotkey: "4", icon: "🔮", active: false }
                    HudAction { hotkey: "5", icon: "🏹", active: false }
                    // Séparateur
                    div { style: "width: 2px; background: #3a5a4a; margin: 0 4px;" }
                    HudAction { hotkey: "Q", icon: "🍞", active: false }
                    HudAction { hotkey: "E", icon: "💊", active: false }
                }

                // ── Bottom-right : notification ──
                div {
                    style: "position: absolute; bottom: 80px; right: 16px; padding: 10px 16px; background: rgba(0,0,0,0.75); border-radius: 8px; border-left: 3px solid #ffbe5e; max-width: 250px;",
                    div { style: "font-size: 11px; color: #ffbe5e; font-weight: 600; margin-bottom: 2px;", "Localisation découverte" }
                    div { style: "font-size: 14px; color: #e8d5a0; font-weight: 700;", "Mystic Vale" }
                }

                // ── Center : indicateur de quête ──
                div {
                    style: "position: absolute; top: 16px; left: 50%; transform: translateX(-50%); padding: 6px 20px; background: rgba(0,0,0,0.6); border-radius: 20px; border: 1px solid #4a6a5a;",
                    span { style: "font-size: 12px; color: {c.text_secondary};", "Quête : " }
                    span { style: "font-size: 12px; color: #e8d5a0; font-weight: 500;", "Retrouver le Cristal de Lune" }
                    span { style: "font-size: 11px; color: {c.text_muted}; margin-left: 8px;", "3/5 fragments" }
                }
            }
        }
    }
}

// ─── Mock 4 : Menu principal (Fantasy dark) ─────────────────────────

#[component]
fn MockMainMenu() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",
            h3 { style: "font-size: 16px; color: {c.accent_blue}; margin: 0;", "4. Menu principal" }
            p { style: "font-size: 12px; color: {c.text_muted}; margin: 0;", "Style Fantasy sombre — écran titre avec bordures décoratives, menu et sélection de personnage." }

            div {
                style: "background: linear-gradient(180deg, #0a0e1a 0%, #141828 40%, #1a2040 100%); border-radius: 12px; padding: 48px 32px; display: flex; flex-direction: column; align-items: center; gap: 32px; border: 1px solid #2a3050; min-height: 500px; position: relative; overflow: hidden;",

                // Particules décoratives (subtiles)
                div { style: "position: absolute; top: 20%; left: 15%; width: 4px; height: 4px; background: #e8d5a0; border-radius: 50%; opacity: 0.3; box-shadow: 0 0 12px #e8d5a0;" }
                div { style: "position: absolute; top: 40%; right: 20%; width: 3px; height: 3px; background: #68b8ff; border-radius: 50%; opacity: 0.4; box-shadow: 0 0 10px #68b8ff;" }
                div { style: "position: absolute; top: 65%; left: 25%; width: 3px; height: 3px; background: #81d058; border-radius: 50%; opacity: 0.3; box-shadow: 0 0 10px #81d058;" }
                div { style: "position: absolute; top: 30%; right: 35%; width: 2px; height: 2px; background: #ffbe5e; border-radius: 50%; opacity: 0.5; box-shadow: 0 0 8px #ffbe5e;" }

                // Logo / Titre
                div {
                    style: "text-align: center; position: relative; display: flex; flex-direction: column; align-items: center;",

                    // Ligne décorative haute : trait CSS + accent Fantasy centré (48px natif)
                    div {
                        style: "display: flex; align-items: center; gap: 12px; margin-bottom: 8px;",
                        div { style: "width: 100px; height: 1px; background: linear-gradient(90deg, transparent, #4a5a7a);" }
                        img {
                            src: "{ui_assets::fantasy_borders::DIVIDER_FADE_005.as_str()}",
                            style: "width: 48px; height: 16px; image-rendering: pixelated; opacity: 0.7;",
                        }
                        div { style: "width: 100px; height: 1px; background: linear-gradient(90deg, #4a5a7a, transparent);" }
                    }

                    h1 {
                        style: "font-size: 36px; font-weight: 800; color: #e8d5a0; letter-spacing: 6px; text-shadow: 0 4px 16px rgba(232,213,160,0.3); margin: 8px 0;",
                        "MIYUKINI"
                    }
                    p { style: "font-size: 14px; color: {c.text_muted}; letter-spacing: 4px; margin: 0;", "CHRONICLES" }

                    // Ligne décorative basse
                    div {
                        style: "display: flex; align-items: center; gap: 12px; margin-top: 8px;",
                        div { style: "width: 100px; height: 1px; background: linear-gradient(90deg, transparent, #4a5a7a);" }
                        img {
                            src: "{ui_assets::fantasy_borders::DIVIDER_FADE_005.as_str()}",
                            style: "width: 48px; height: 16px; image-rendering: pixelated; opacity: 0.7;",
                        }
                        div { style: "width: 100px; height: 1px; background: linear-gradient(90deg, #4a5a7a, transparent);" }
                    }
                }

                // Sélection de personnage
                div {
                    style: "display: flex; gap: 16px; margin-top: 8px;",
                    MenuCharSlot { name: "Aldric", class_name: "Paladin", icon: "🛡️", level: 24, selected: true }
                    MenuCharSlot { name: "Lyria", class_name: "Archère", icon: "🏹", level: 18, selected: false }
                    MenuCharSlot { name: "Vex", class_name: "Mage", icon: "🔮", level: 21, selected: false }
                }

                // Boutons du menu
                div {
                    style: "display: flex; flex-direction: column; gap: 8px; align-items: center; min-width: 220px;",
                    MenuButton { label: "Continuer", primary: true }
                    MenuButton { label: "Nouvelle partie", primary: false }
                    MenuButton { label: "Charger", primary: false }
                    MenuButton { label: "Options", primary: false }
                    MenuButton { label: "Quitter", primary: false }
                }

                // Version
                span { style: "position: absolute; bottom: 12px; right: 16px; font-size: 10px; color: {c.text_muted};", "v0.1.0-alpha" }
            }
        }
    }
}

// ─── Mock 5 : Boutique (Adventure panels) ───────────────────────────

#[component]
fn MockShop() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let panel_shop = ui_assets::svg_data_url(&ui_assets::panel_brown(700, 420));

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 12px;",
            h3 { style: "font-size: 16px; color: {c.accent_blue}; margin: 0;", "5. Boutique du marchand" }
            p { style: "font-size: 12px; color: {c.text_muted}; margin: 0;", "Style Adventure — panneau marron, grille d'articles avec prix et actions." }

            div {
                style: "background: #3a6a8c; border-radius: 12px; padding: 32px; display: flex; flex-direction: column; align-items: center; gap: 16px; border: 1px solid #4a8aac;",

                // Panneau principal
                div {
                    style: "position: relative; width: 100%; max-width: 700px; min-height: 420px;",
                    img {
                        src: "{panel_shop}",
                        style: "position: absolute; inset: 0; width: 100%; height: 100%;",
                    }
                    div {
                        style: "position: relative; padding: 24px; z-index: 1;",

                        // Header boutique
                        div {
                            style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;",
                            div {
                                style: "display: flex; align-items: center; gap: 8px;",
                                span { style: "font-size: 24px;", "🏪" }
                                span { style: "font-size: 16px; font-weight: 700; color: #5a3a1a;", "Boutique de Bertrand" }
                            }
                            div {
                                style: "display: flex; align-items: center; gap: 6px; padding: 4px 12px; background: rgba(90,58,26,0.15); border-radius: 12px;",
                                span { style: "font-size: 16px;", "💰" }
                                span { style: "font-size: 14px; font-weight: 700; color: #5a3a1a;", "1,247" }
                            }
                        }

                        // Grille d'articles
                        div {
                            style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(130px, 1fr)); gap: 10px;",
                            ShopItem { icon: "🧪", name: "Potion de soin", price: "25", rarity: "common" }
                            ShopItem { icon: "💊", name: "Antidote", price: "15", rarity: "common" }
                            ShopItem { icon: "⚔️", name: "Épée en acier", price: "180", rarity: "uncommon" }
                            ShopItem { icon: "🏹", name: "Arc long", price: "220", rarity: "uncommon" }
                            ShopItem { icon: "🔮", name: "Baguette arcane", price: "450", rarity: "rare" }
                            ShopItem { icon: "📜", name: "Parchemin feu", price: "75", rarity: "uncommon" }
                            ShopItem { icon: "💎", name: "Gemme de pouvoir", price: "800", rarity: "epic" }
                            ShopItem { icon: "🛡️", name: "Bouclier lourd", price: "320", rarity: "uncommon" }
                        }
                    }
                }

                // Tooltip sélection
                div {
                    style: "display: flex; gap: 16px; align-items: center; max-width: 700px; width: 100%;",

                    div {
                        style: "position: relative; flex: 1; min-height: 80px;",
                        img {
                            src: "{ui_assets::svg_data_url(&ui_assets::panel_grey(500, 80))}",
                            style: "position: absolute; inset: 0; width: 100%; height: 100%;",
                        }
                        div {
                            style: "position: relative; padding: 12px 16px; z-index: 1; display: flex; align-items: center; gap: 12px;",
                            span { style: "font-size: 28px;", "🔮" }
                            div {
                                span { style: "font-size: 13px; font-weight: 600; color: #1b2838;", "Baguette arcane" }
                                p { style: "font-size: 11px; color: #4a5a6a; margin: 2px 0 0 0;", "+12 Intelligence, +5% Mana regen. Requiert Nv. 15." }
                            }
                        }
                    }

                    // Bouton acheter
                    div {
                        style: "position: relative; width: 120px; height: 48px; cursor: pointer; flex-shrink: 0;",
                        img { src: "{ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN)}", style: "width: 100%; height: 100%;" }
                        span { style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; color: #5a3a1a;", "Acheter" }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  ONGLET — MIYUCLICKER (Mock GUI complet)
// ═══════════════════════════════════════════════════════════════════════

#[component]
fn TabMiyuClicker() -> Element {
    let _c = use_app_state().read().current_theme.palette();

    // Couleurs thème RPG Fantasy (parchemin / bois)
    let bg_dark = "#2a1a0a";       // fond principal bois sombre
    let _bg_mid = "#3a2a15";       // fond intermédiaire
    let _bg_light = "#4a3520";     // fond clair
    let gold = "#e8d5a0";          // texte or
    let parchment = "#FFF1D2";     // texte parchemin
    let brown_text = "#5a3a1a";    // texte marron foncé
    let border = "#6D4B27";        // bordure marron

    // SVG paramétriques
    let header_panel = ui_assets::svg_data_url(&ui_assets::panel_brown_dark(900, 90));
    let notif_panel = ui_assets::svg_data_url(&ui_assets::panel_grey(700, 36));
    let prod_panel = ui_assets::svg_data_url(&ui_assets::panel_brown(300, 340));
    let xp_panel = ui_assets::svg_data_url(&ui_assets::panel_brown_dark(900, 32));

    // Barres
    let build_track = ui_assets::svg_data_url(&ui_assets::bar_track(300, 14));
    let build_fill_35 = ui_assets::svg_data_url(&ui_assets::bar_green(105, 14));
    let build_fill_0 = ui_assets::svg_data_url(&ui_assets::bar_green(12, 14));
    let xp_track = ui_assets::svg_data_url(&ui_assets::bar_track(600, 12));
    let xp_fill = ui_assets::svg_data_url(&ui_assets::bar_blue(420, 12));

    // Boutons SVG Adventure
    let btn_brown = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN);
    let btn_grey = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_GREY);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0; background: {bg_dark}; border-radius: 8px; overflow: hidden; border: 2px solid {border};",

            // ═══════════════════════════════════════════════
            //  HEADER — Panneau marron sombre SVG
            // ═══════════════════════════════════════════════
            div {
                style: "position: relative; min-height: 90px;",
                img { src: "{header_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                div {
                    style: "position: relative; z-index: 1; padding: 8px 16px; display: flex; flex-direction: column; gap: 4px;",

                    // Ligne 1 : Pseudo + Ressources + Config
                    div {
                        style: "display: flex; align-items: center; gap: 4px; flex-wrap: wrap;",
                        div {
                            style: "padding: 3px 10px; background: rgba(0,0,0,0.3); border-radius: 4px; border: 1px solid {border}; margin-right: 6px;",
                            span { style: "font-size: 12px; font-weight: 700; color: {gold}; letter-spacing: 1px;", "⚜ Aldric" }
                        }
                        ClickerResource { icon: "🌾", name: "Food", current: 842, max: 1200 }
                        ClickerResource { icon: "🪵", name: "Bois", current: 356, max: 800 }
                        ClickerResource { icon: "🪨", name: "Pierre", current: 210, max: 600 }
                        ClickerResource { icon: "⛏️", name: "Métal", current: 78, max: 400 }
                        ClickerResource { icon: "🔧", name: "Outils", current: 45, max: 200 }
                        ClickerResource { icon: "⚔️", name: "Armes", current: 12, max: 100 }
                        div { style: "flex: 1;" }
                        div {
                            style: "width: 28px; height: 28px; border-radius: 50%; background: rgba(0,0,0,0.3); border: 1px solid {border}; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 14px;",
                            "⚙️"
                        }
                    }

                    // Ligne 2 : Population + Navigation
                    div {
                        style: "display: flex; align-items: center; gap: 4px; flex-wrap: wrap;",
                        ClickerPopBadge { label: "Pop", value: "47/60", color: "#FFF1D2" }
                        ClickerPopBadge { label: "Ouvriers", value: "28", color: "#ffbe5e" }
                        ClickerPopBadge { label: "Bâtisseurs", value: "12", color: "#68b8ff" }
                        ClickerPopBadge { label: "Soldats", value: "7", color: "#ff7366" }
                        ClickerPopBadge { label: "Bonheur", value: "72%", color: "#81d058" }
                        div { style: "flex: 1;" }
                        // Boutons navigation (SVG Adventure)
                        div {
                            style: "display: flex; gap: 6px;",
                            div {
                                style: "position: relative; width: 110px; height: 28px; cursor: pointer;",
                                img { src: "{btn_grey}", style: "width: 100%; height: 100%;" }
                                span { style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: 600; color: {brown_text};", "Carte du monde" }
                            }
                            div {
                                style: "position: relative; width: 100px; height: 28px; cursor: pointer;",
                                img { src: "{btn_brown}", style: "width: 100%; height: 100%;" }
                                span { style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: 600; color: {brown_text};", "Mon domaine" }
                            }
                        }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  VUE DOMAINE — Terrain fantasy + PNJ + Château
            // ═══════════════════════════════════════════════
            div {
                style: "position: relative; height: 220px; background: linear-gradient(180deg, #5a7a9a 0%, #7a9a6a 25%, #4a8c3a 40%, #3a6a2a 70%, #2a5a1a 100%); overflow: hidden; border-top: 2px solid {border}; border-bottom: 2px solid {border};",

                // Herbe
                div { style: "position: absolute; bottom: 0; left: 0; right: 0; height: 60%; background-image: radial-gradient(circle, #2a6a1a 1px, transparent 1px); background-size: 20px 16px; opacity: 0.12;" }

                // Château (style pierre fantaisie)
                div {
                    style: "position: absolute; bottom: 28%; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; align-items: center;",
                    // Toits coniques
                    div {
                        style: "display: flex; gap: 48px; margin-bottom: -2px;",
                        div { style: "width: 0; height: 0; border-left: 14px solid transparent; border-right: 14px solid transparent; border-bottom: 20px solid #8B4513;" }
                        div { style: "width: 0; height: 0; border-left: 14px solid transparent; border-right: 14px solid transparent; border-bottom: 20px solid #8B4513;" }
                    }
                    // Tours
                    div {
                        style: "display: flex; gap: 44px;",
                        div { style: "width: 18px; height: 36px; background: #b8a080; border: 2px solid #8a7060;" }
                        div { style: "width: 18px; height: 36px; background: #b8a080; border: 2px solid #8a7060;" }
                    }
                    // Corps principal
                    div { style: "width: 70px; height: 50px; background: #c8b090; border: 2px solid #8a7060; margin-top: -38px; display: flex; align-items: flex-end; justify-content: center;" }
                    // Toit principal
                    div { style: "width: 0; height: 0; border-left: 45px solid transparent; border-right: 45px solid transparent; border-bottom: 25px solid #A0522D; margin-top: -52px; margin-bottom: 0;" }
                }

                // ── PNJ Ouvriers (jaune) ──
                div { style: "position: absolute; bottom: 18%; left: 8%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 22%; left: 14%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 15%; left: 20%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 20%; left: 5%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 16%; right: 10%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 20%; right: 15%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                // ── PNJ Soldats (rouge) ──
                div { style: "position: absolute; bottom: 16%; left: 42%; width: 8px; height: 14px; background: #ff7366; border-radius: 3px 3px 1px 1px; border: 1px solid #c83737; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 14%; left: 45%; width: 8px; height: 14px; background: #ff7366; border-radius: 3px 3px 1px 1px; border: 1px solid #c83737; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 17%; left: 48%; width: 8px; height: 14px; background: #ff7366; border-radius: 3px 3px 1px 1px; border: 1px solid #c83737; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 15%; left: 52%; width: 8px; height: 14px; background: #ff7366; border-radius: 3px 3px 1px 1px; border: 1px solid #c83737; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 13%; left: 55%; width: 8px; height: 14px; background: #ff7366; border-radius: 3px 3px 1px 1px; border: 1px solid #c83737; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                // ── PNJ Bâtisseurs (bleu) ──
                div { style: "position: absolute; bottom: 22%; left: 63%; width: 8px; height: 14px; background: #68b8ff; border-radius: 3px 3px 1px 1px; border: 1px solid #3b6891; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 25%; left: 66%; width: 8px; height: 14px; background: #68b8ff; border-radius: 3px 3px 1px 1px; border: 1px solid #3b6891; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }
                div { style: "position: absolute; bottom: 20%; left: 69%; width: 8px; height: 14px; background: #68b8ff; border-radius: 3px 3px 1px 1px; border: 1px solid #3b6891; box-shadow: 0 1px 3px rgba(0,0,0,0.4);" }

                // Notification (coin droit) dans un mini SVG panel
                div {
                    style: "position: absolute; top: 10px; right: 10px; position: relative; display: inline-block;",
                    div {
                        style: "position: absolute; top: 10px; right: 10px; padding: 8px 16px; background: rgba(42,26,10,0.85); border-radius: 6px; border: 1px solid {border};",
                        div { style: "font-size: 10px; color: #81d058; font-weight: 600;", "Félicitation" }
                        div { style: "font-size: 12px; color: {gold};", "Tu es monté de niveau !" }
                    }
                }

                // Légende PNJ
                div {
                    style: "position: absolute; top: 8px; left: 8px; display: flex; gap: 10px; padding: 4px 8px; background: rgba(42,26,10,0.8); border-radius: 4px; border: 1px solid {border};",
                    div { style: "display: flex; align-items: center; gap: 4px;",
                        div { style: "width: 8px; height: 8px; background: #ffbe5e; border-radius: 2px;" }
                        span { style: "font-size: 9px; color: {parchment};", "Ouvriers" }
                    }
                    div { style: "display: flex; align-items: center; gap: 4px;",
                        div { style: "width: 8px; height: 8px; background: #ff7366; border-radius: 2px;" }
                        span { style: "font-size: 9px; color: {parchment};", "Soldats" }
                    }
                    div { style: "display: flex; align-items: center; gap: 4px;",
                        div { style: "width: 8px; height: 8px; background: #68b8ff; border-radius: 2px;" }
                        span { style: "font-size: 9px; color: {parchment};", "Bâtisseurs" }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  BARRE DE NOTIFICATION (panneau gris SVG)
            // ═══════════════════════════════════════════════
            div {
                style: "display: flex; align-items: center; justify-content: center; padding: 6px 16px; background: {bg_dark};",
                div {
                    style: "position: relative; flex: 1; max-width: 700px; height: 36px;",
                    img { src: "{notif_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                    div {
                        style: "position: relative; z-index: 1; display: flex; align-items: center; justify-content: center; height: 36px;",
                        span { style: "font-size: 11px; color: {brown_text}; font-weight: 600;", "📜 " }
                        span { style: "font-size: 11px; color: {brown_text};", "Construction des Maisons terminée ! Population max +10" }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  PANNEAU BAS — Production + Bâtiments
            // ═══════════════════════════════════════════════
            div {
                style: "display: flex; gap: 12px; padding: 12px; flex-wrap: wrap; background: {bg_dark};",

                // ── Colonne gauche : Production (panneau marron SVG) ──
                div {
                    style: "display: flex; flex-direction: column; gap: 0; min-width: 280px; flex: 1;",

                    div {
                        style: "position: relative; min-height: 340px;",
                        img { src: "{prod_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                        div {
                            style: "position: relative; z-index: 1; padding: 18px;",

                            div {
                                style: "font-size: 13px; font-weight: 700; color: {brown_text}; letter-spacing: 1px; text-align: center; margin-bottom: 10px;",
                                "⚒ La production"
                            }
                            div { style: "width: 120px; height: 1px; background: linear-gradient(90deg, transparent, #a3703a, transparent); margin: 0 auto 10px;" }

                            div {
                                style: "display: flex; flex-direction: column; gap: 4px;",
                                ClickerProdRow { name: "Ferme", rate: "12 Food/sec", count: 8 }
                                ClickerProdRow { name: "Scierie", rate: "6 Bois/sec", count: 5 }
                                ClickerProdRow { name: "Carrière", rate: "4 Pierre/sec", count: 3 }
                                ClickerProdRow { name: "Mine", rate: "2 Métal/sec", count: 2 }
                                ClickerProdRow { name: "Atelier", rate: "1 Outils/sec", count: 1 }
                                ClickerProdRow { name: "Forge", rate: "0.5 Armes/sec", count: 1 }
                            }
                        }
                    }

                    // Boutons recrutement (SVG Adventure buttons)
                    div {
                        style: "display: flex; flex-direction: column; gap: 6px; margin-top: 12px; align-items: center;",
                        ClickerRecruitBtn { label: "+ 1 Ouvrier", color: "#ffbe5e" }
                        ClickerRecruitBtn { label: "+ 1 Bâtisseur", color: "#68b8ff" }
                        ClickerRecruitBtn { label: "+ 1 Soldat", color: "#ff7366" }
                    }
                }

                // ── Colonne droite : Bâtiments (chaque dans un panneau marron SVG) ──
                div {
                    style: "display: flex; flex-direction: column; gap: 10px; flex: 1.5; min-width: 380px;",

                    ClickerBuildCard {
                        name: "Maisons",
                        desc: "Augmente la quantité de population maximale",
                        level: 3, progress_pct: 35, builders: 4,
                        cost_b: "20", cost_p: "15", cost_m: "0",
                        border_color: "#e8a030",
                        track_url: build_track.clone(), fill_url: build_fill_35.clone(),
                    }
                    ClickerBuildCard {
                        name: "Casernes",
                        desc: "Augmente la quantité maximale de soldats",
                        level: 1, progress_pct: 0, builders: 0,
                        cost_b: "30", cost_p: "25", cost_m: "10",
                        border_color: "#ff7366",
                        track_url: build_track.clone(), fill_url: build_fill_0.clone(),
                    }
                    ClickerBuildCard {
                        name: "Guilde des Maçons",
                        desc: "Augmente la quantité maximale de bâtisseurs",
                        level: 2, progress_pct: 0, builders: 0,
                        cost_b: "25", cost_p: "20", cost_m: "5",
                        border_color: "#a855f7",
                        track_url: build_track.clone(), fill_url: build_fill_0.clone(),
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  BARRE XP (panneau marron sombre SVG)
            // ═══════════════════════════════════════════════
            div {
                style: "position: relative; height: 32px;",
                img { src: "{xp_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                div {
                    style: "position: relative; z-index: 1; display: flex; align-items: center; gap: 8px; padding: 0 16px; height: 32px;",
                    span { style: "font-size: 10px; color: {gold}; font-weight: 700;", "⭐ Nv.5" }
                    div {
                        style: "position: relative; flex: 1; height: 12px;",
                        img { src: "{xp_track}", style: "position: absolute; inset: 0; width: 100%; height: 12px;" }
                        img { src: "{xp_fill}", style: "position: absolute; top: 0; left: 0; width: 70%; height: 12px;" }
                    }
                    span { style: "font-size: 10px; color: #a3703a;", "4200 / 6000 XP" }
                }
            }
        }
    }
}

// ── Sous-composants MiyuClicker ─────────────────────────────────────

/// Badge de ressource dans le header.
#[derive(Props, Clone, PartialEq)]
struct ClickerResourceProps {
    icon: &'static str,
    name: &'static str,
    current: u32,
    max: u32,
}

#[component]
fn ClickerResource(props: ClickerResourceProps) -> Element {
    let pct = (props.current as f32 / props.max as f32 * 100.0) as u32;
    let color = if pct > 80 { "#81d058" } else if pct > 40 { "#e8d5a0" } else { "#ff7366" };
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px; padding: 3px 8px; background: rgba(0,0,0,0.25); border-radius: 4px; border: 1px solid #6D4B27;",
            span { style: "font-size: 12px;", "{props.icon}" }
            span { style: "font-size: 11px; color: {color}; font-weight: 600;", "{props.current}" }
            span { style: "font-size: 9px; color: #a3703a;", "/{props.max}" }
        }
    }
}

/// Badge de population dans le header.
#[derive(Props, Clone, PartialEq)]
struct ClickerPopBadgeProps {
    label: &'static str,
    value: &'static str,
    color: &'static str,
}

#[component]
fn ClickerPopBadge(props: ClickerPopBadgeProps) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px; padding: 2px 8px; background: rgba(0,0,0,0.2); border-radius: 4px; border: 1px solid #6D4B27;",
            span { style: "font-size: 10px; color: #a3703a;", "{props.label}" }
            span { style: "font-size: 11px; color: {props.color}; font-weight: 600;", "{props.value}" }
        }
    }
}

/// Ligne de production (Ferme, Scierie, etc.).
#[derive(Props, Clone, PartialEq)]
struct ClickerProdRowProps {
    name: &'static str,
    rate: &'static str,
    count: u32,
}

#[component]
fn ClickerProdRow(props: ClickerProdRowProps) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px; padding: 5px 8px; background: rgba(0,0,0,0.15); border-radius: 4px; border: 1px solid rgba(109,75,39,0.4);",
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                span { style: "font-size: 11px; color: #5a3a1a; font-weight: 600;", "{props.name}" }
                span { style: "font-size: 9px; color: #a3703a;", "{props.rate}" }
            }
            // Bouton +
            div {
                style: "width: 20px; height: 20px; background: #4a7a3a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 12px; font-weight: 700; color: #d0f0a0; border: 1px solid #5a8a4a;",
                "+"
            }
            span { style: "font-size: 11px; color: #5a3a1a; font-weight: 700; min-width: 20px; text-align: center;", "{props.count}" }
            // Bouton -
            div {
                style: "width: 20px; height: 20px; background: #7a3a2a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 12px; font-weight: 700; color: #ffb0a0; border: 1px solid #8a4a3a;",
                "−"
            }
        }
    }
}

/// Bouton de recrutement (+1 Ouvrier, etc.).
#[derive(Props, Clone, PartialEq)]
struct ClickerRecruitBtnProps {
    label: &'static str,
    color: &'static str,
}

#[component]
fn ClickerRecruitBtn(props: ClickerRecruitBtnProps) -> Element {
    let btn_svg = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN);
    rsx! {
        div {
            style: "position: relative; width: 170px; height: 32px; cursor: pointer;",
            img { src: "{btn_svg}", style: "width: 100%; height: 100%;" }
            div {
                style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; gap: 6px;",
                div { style: "width: 8px; height: 8px; border-radius: 50%; background: {props.color}; border: 1px solid rgba(0,0,0,0.3);" }
                span { style: "font-size: 11px; font-weight: 600; color: #5a3a1a;", "{props.label}" }
            }
        }
    }
}

/// Carte de bâtiment (Maisons, Casernes, Guilde).
#[derive(Props, Clone, PartialEq)]
struct ClickerBuildCardProps {
    name: String,
    desc: String,
    level: u32,
    progress_pct: u32,
    builders: u32,
    cost_b: &'static str,
    cost_p: &'static str,
    cost_m: &'static str,
    border_color: String,
    track_url: String,
    fill_url: String,
}

#[component]
fn ClickerBuildCard(props: ClickerBuildCardProps) -> Element {
    let progress_label = format!("{}%", props.progress_pct);
    let fill_width = format!("{}%", props.progress_pct.max(4));
    let card_bg = ui_assets::svg_data_url(&ui_assets::panel_brown(500, 120));
    let build_btn = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN);
    rsx! {
        div {
            style: "position: relative; min-height: 120px;",
            img { src: "{card_bg}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
            // Accent bordure colorée sur la gauche
            div { style: "position: absolute; left: 0; top: 8px; bottom: 8px; width: 4px; border-radius: 0 3px 3px 0; background: {props.border_color};" }
            div {
                style: "position: relative; z-index: 1; padding: 14px 14px 14px 18px; display: flex; flex-direction: column; gap: 6px;",

                // Header : nom + level
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 13px; font-weight: 700; color: #5a3a1a;", "🏗 {props.name}" }
                    div {
                        style: "padding: 2px 8px; background: rgba(0,0,0,0.15); border-radius: 4px; border: 1px solid #6D4B27;",
                        span { style: "font-size: 10px; font-weight: 700; color: {props.border_color};", "LVL {props.level}" }
                    }
                }

                // Description
                span { style: "font-size: 10px; color: #a3703a;", "{props.desc}" }

                // Barre de progression
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    div {
                        style: "position: relative; flex: 1; height: 14px;",
                        img { src: "{props.track_url}", style: "position: absolute; inset: 0; width: 100%; height: 14px;" }
                        img { src: "{props.fill_url}", style: "position: absolute; top: 0; left: 0; width: {fill_width}; height: 14px;" }
                    }
                    span { style: "font-size: 10px; color: #a3703a; min-width: 28px; text-align: right;", "{progress_label}" }
                }

                // Ligne action : bâtisseurs + coût + bouton construire
                div {
                    style: "display: flex; align-items: center; gap: 6px; flex-wrap: wrap;",
                    span { style: "font-size: 10px; color: #68b8ff;", "Bâtisseurs" }
                    div {
                        style: "width: 18px; height: 18px; background: #4a7a3a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 10px; font-weight: 700; color: #d0f0a0;",
                        "+"
                    }
                    span { style: "font-size: 11px; color: #5a3a1a; font-weight: 700; min-width: 16px; text-align: center;", "{props.builders}" }
                    div {
                        style: "width: 18px; height: 18px; background: #7a3a2a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 10px; font-weight: 700; color: #ffb0a0;",
                        "−"
                    }

                    // Coûts
                    div {
                        style: "display: flex; gap: 6px; margin-left: auto;",
                        span { style: "font-size: 9px; color: #a3703a;", "🪵{props.cost_b}" }
                        span { style: "font-size: 9px; color: #a3703a;", "🪨{props.cost_p}" }
                        span { style: "font-size: 9px; color: #a3703a;", "⛏️{props.cost_m}" }
                    }

                    // Bouton construire (Adventure SVG)
                    div {
                        style: "position: relative; width: 100px; height: 26px; cursor: pointer; margin-left: 6px;",
                        img { src: "{build_btn}", style: "width: 100%; height: 100%;" }
                        span { style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: 600; color: #5a3a1a;", "Construire" }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  SOUS-COMPOSANTS DES MAQUETTES
// ═══════════════════════════════════════════════════════════════════════

/// Barre de stat pour la fiche RPG (Mock 1).
#[derive(Props, Clone, PartialEq)]
struct MockStatBarProps {
    label: &'static str,
    value: String,
    track_url: String,
    fill_url: String,
    fill_w: u32,
    total_w: u32,
    color: &'static str,
}

#[component]
fn MockStatBar(props: MockStatBarProps) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px;",
            span { style: "width: 30px; font-size: 11px; font-weight: 700; color: {props.color}; text-align: right;", "{props.label}" }
            div {
                style: "position: relative; width: {props.total_w}px; height: 18px; flex-shrink: 0;",
                img { src: "{props.track_url}", style: "position: absolute; inset: 0; width: {props.total_w}px; height: 18px;" }
                img { src: "{props.fill_url}", style: "position: absolute; top: 0; left: 0; width: {props.fill_w}px; height: 18px;" }
            }
            span { style: "font-size: 10px; color: #8a8aaa; min-width: 70px;", "{props.value}" }
        }
    }
}

/// Attribut pour la fiche RPG (Mock 1).
#[derive(Props, Clone, PartialEq)]
struct MockAttrProps {
    name: &'static str,
    value: &'static str,
}

#[component]
fn MockAttr(props: MockAttrProps) -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: space-between; padding: 4px 8px; border-radius: 4px; background: #1a1e2e;",
            span { style: "font-size: 12px; color: #8a8aaa;", "{props.name}" }
            span { style: "font-size: 12px; font-weight: 700; color: #e8d5a0;", "{props.value}" }
        }
    }
}

/// Slot d'inventaire (Mock 1).
#[derive(Props, Clone, PartialEq)]
struct MockInvSlotProps {
    icon: &'static str,
    count: &'static str,
}

#[component]
fn MockInvSlot(props: MockInvSlotProps) -> Element {
    rsx! {
        div {
            style: "aspect-ratio: 1; background: #222840; border-radius: 6px; border: 1px solid #3a4a6a; display: flex; align-items: center; justify-content: center; position: relative; cursor: pointer;",
            span { style: "font-size: 22px;", "{props.icon}" }
            span {
                style: "position: absolute; bottom: 2px; right: 4px; font-size: 10px; font-weight: 700; color: #8a8aaa;",
                "{props.count}"
            }
        }
    }
}

/// Ligne d'équipement (Mock 1).
#[derive(Props, Clone, PartialEq)]
struct MockEquipRowProps {
    slot: &'static str,
    item: &'static str,
    icon: &'static str,
}

#[component]
fn MockEquipRow(props: MockEquipRowProps) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; padding: 8px 12px; background: #222840; border-radius: 6px; border: 1px solid #2a3050;",
            span { style: "font-size: 20px;", "{props.icon}" }
            div {
                style: "flex: 1;",
                div { style: "font-size: 12px; color: #e8d5a0; font-weight: 500;", "{props.item}" }
                div { style: "font-size: 10px; color: #5a5a7a;", "{props.slot}" }
            }
        }
    }
}

/// Barre HUD overlay (Mock 3).
#[derive(Props, Clone, PartialEq)]
struct HudBarProps {
    label: &'static str,
    fill_pct: u32,
    color: String,
    w: u32,
    h: u32,
}

#[component]
fn HudBar(props: HudBarProps) -> Element {
    let fill_w = ((props.w as f32) * (props.fill_pct as f32) / 100.0) as u32;
    let fill_w = fill_w.max(12);
    let track_url = ui_assets::svg_data_url(&ui_assets::bar_track(props.w, props.h));
    let fill_svg = match props.color.as_str() {
        "blue" => ui_assets::bar_blue(fill_w, props.h),
        "green" => ui_assets::bar_green(fill_w, props.h),
        "orange" => ui_assets::bar_orange(fill_w, props.h),
        _ => ui_assets::bar_red(fill_w, props.h),
    };
    let fill_url = ui_assets::svg_data_url(&fill_svg);
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px;",
            span { style: "font-size: 10px; font-weight: 700; color: #ccc; width: 22px; text-shadow: 0 1px 3px rgba(0,0,0,0.8);", "{props.label}" }
            div {
                style: "position: relative; width: {props.w}px; height: {props.h}px;",
                img { src: "{track_url}", style: "position: absolute; inset: 0; width: {props.w}px; height: {props.h}px;" }
                img { src: "{fill_url}", style: "position: absolute; top: 0; left: 0; width: {fill_w}px; height: {props.h}px;" }
            }
            span { style: "font-size: 10px; color: #aaa; text-shadow: 0 1px 3px rgba(0,0,0,0.8);", "{props.fill_pct}%" }
        }
    }
}

/// Bouton d'action HUD (Mock 3).
#[derive(Props, Clone, PartialEq)]
struct HudActionProps {
    hotkey: &'static str,
    icon: &'static str,
    active: bool,
}

#[component]
fn HudAction(props: HudActionProps) -> Element {
    let border = if props.active { "2px solid #1a9fff" } else { "1px solid #3a5a4a" };
    let bg = if props.active { "rgba(26,159,255,0.2)" } else { "rgba(20,40,30,0.8)" };
    rsx! {
        div {
            style: "width: 48px; height: 48px; border-radius: 8px; background: {bg}; border: {border}; display: flex; flex-direction: column; align-items: center; justify-content: center; cursor: pointer; position: relative;",
            span { style: "font-size: 20px;", "{props.icon}" }
            span { style: "position: absolute; top: 2px; right: 4px; font-size: 9px; font-weight: 700; color: #8a8aaa;", "{props.hotkey}" }
        }
    }
}

/// Slot personnage menu principal (Mock 4).
#[derive(Props, Clone, PartialEq)]
struct MenuCharSlotProps {
    name: &'static str,
    class_name: &'static str,
    icon: &'static str,
    level: u32,
    selected: bool,
}

#[component]
fn MenuCharSlot(props: MenuCharSlotProps) -> Element {
    let border = if props.selected { "2px solid #e8d5a0" } else { "1px solid #2a3050" };
    let bg = if props.selected { "#1a2040" } else { "#0e1220" };
    let opacity = if props.selected { "1" } else { "0.6" };
    rsx! {
        div {
            style: "width: 110px; padding: 16px 12px; background: {bg}; border: {border}; border-radius: 10px; text-align: center; cursor: pointer; opacity: {opacity}; display: flex; flex-direction: column; align-items: center; gap: 6px;",
            span { style: "font-size: 32px;", "{props.icon}" }
            span { style: "font-size: 13px; font-weight: 600; color: #e8d5a0;", "{props.name}" }
            span { style: "font-size: 10px; color: #6a6a8a;", "{props.class_name} Nv.{props.level}" }
        }
    }
}

/// Bouton du menu principal (Mock 4).
#[derive(Props, Clone, PartialEq)]
struct MenuButtonProps {
    label: &'static str,
    primary: bool,
}

#[component]
fn MenuButton(props: MenuButtonProps) -> Element {
    let bg = if props.primary { "linear-gradient(180deg, #3a5a7c 0%, #2a4a68 100%)" } else { "transparent" };
    let border = if props.primary { "1px solid #4a7aac" } else { "1px solid #2a3050" };
    let color = if props.primary { "#e8d5a0" } else { "#6a6a8a" };
    let font_size = if props.primary { "15px" } else { "13px" };
    let padding = if props.primary { "12px 48px" } else { "8px 36px" };
    rsx! {
        div {
            style: "padding: {padding}; background: {bg}; border: {border}; border-radius: 6px; cursor: pointer; text-align: center; min-width: 200px;",
            span { style: "font-size: {font_size}; font-weight: 600; color: {color}; letter-spacing: 2px;", "{props.label}" }
        }
    }
}

/// Article de boutique (Mock 5).
#[derive(Props, Clone, PartialEq)]
struct ShopItemProps {
    icon: &'static str,
    name: &'static str,
    price: &'static str,
    rarity: &'static str,
}

#[component]
fn ShopItem(props: ShopItemProps) -> Element {
    let border_color = match props.rarity {
        "uncommon" => "#68b8ff",
        "rare" => "#a855f7",
        "epic" => "#ffbe5e",
        _ => "#8a7a5a",
    };
    rsx! {
        div {
            style: "padding: 10px; background: rgba(90,58,26,0.12); border-radius: 8px; border: 1px solid {border_color}40; display: flex; flex-direction: column; align-items: center; gap: 6px; cursor: pointer;",
            span { style: "font-size: 28px;", "{props.icon}" }
            span { style: "font-size: 11px; font-weight: 500; color: #5a3a1a; text-align: center;", "{props.name}" }
            div {
                style: "display: flex; align-items: center; gap: 4px;",
                span { style: "font-size: 12px;", "💰" }
                span { style: "font-size: 12px; font-weight: 700; color: #5a3a1a;", "{props.price}" }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  COMPOSANTS RÉUTILISABLES (ORIGINAUX)
// ═══════════════════════════════════════════════════════════════════════

/// Titre de section dans l'onglet curseurs.
#[derive(Props, Clone, PartialEq)]
struct CursorSectionProps {
    title: &'static str,
}

#[component]
fn CursorSection(props: CursorSectionProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        h3 {
            style: "font-size: 14px; color: {c.accent_blue}; border-bottom: 1px solid {c.border}; padding-bottom: 6px; margin: 0;",
            "{props.title}"
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
//  COMPOSANTS RÉUTILISABLES
// ═══════════════════════════════════════════════════════════════════════

/// Affiche un panneau SVG paramétrique avec son titre.
#[derive(Props, Clone, PartialEq)]
struct SvgPanelProps {
    title: String,
    svg: String,
    w: u32,
    h: u32,
}

#[component]
fn SvgPanel(props: SvgPanelProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let data_url = ui_assets::svg_data_url(&props.svg);
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",
            span { style: "font-size: 12px; color: {c.text_secondary};", "{props.title}" }
            img {
                src: "{data_url}",
                style: "width: {props.w}px; height: {props.h}px;",
            }
        }
    }
}

/// Conteneur démo : un panneau SVG avec du texte à l'intérieur.
#[derive(Props, Clone, PartialEq)]
struct ContainerDemoProps {
    panel_type: String,
    width: u32,
    height: u32,
    title: String,
    body: String,
}

#[component]
fn ContainerDemo(props: ContainerDemoProps) -> Element {
    let svg = match props.panel_type.as_str() {
        "grey" => ui_assets::panel_grey(props.width, props.height),
        _ => ui_assets::panel_brown(props.width, props.height),
    };
    let data_url = ui_assets::svg_data_url(&svg);
    let text_color = match props.panel_type.as_str() {
        "grey" => "#1b2838",
        _ => "#5a3a1a",
    };
    rsx! {
        div {
            style: "position: relative; width: {props.width}px; height: {props.height}px;",
            img {
                src: "{data_url}",
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%;",
            }
            div {
                style: "position: relative; padding: 20px; z-index: 1; color: {text_color};",
                h3 { style: "margin: 0 0 8px 0; font-size: 14px; font-weight: 600;", "{props.title}" }
                p { style: "font-size: 12px; line-height: 1.5; margin: 0;", "{props.body}" }
            }
        }
    }
}

/// Affiche un SVG inline (à taille fixe avec scale optionnel).
#[derive(Props, Clone, PartialEq)]
struct SvgInlineProps {
    svg: &'static str,
    scale: f32,
    label: &'static str,
}

#[component]
fn SvgInline(props: SvgInlineProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let data_url = ui_assets::svg_data_url(props.svg);
    // Extraire la taille du SVG depuis l'attribut width/height
    let scale_transform = format!("transform: scale({});", props.scale);
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 6px;",
            img {
                src: "{data_url}",
                style: "{scale_transform} transform-origin: top left;",
            }
            span { style: "font-size: 11px; color: {c.text_muted};", "{props.label}" }
        }
    }
}

/// Barre de progression avec track + fill.
#[derive(Props, Clone, PartialEq)]
struct BarDemoProps {
    label: String,
    w: u32,
    h: u32,
    fill_pct: u32,
    color: String,
}

#[component]
fn BarDemo(props: BarDemoProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let fill_w = ((props.w as f32) * (props.fill_pct as f32) / 100.0) as u32;
    let fill_w = fill_w.max(12);

    let track_svg = ui_assets::bar_track(props.w, props.h);
    let fill_svg = match props.color.as_str() {
        "blue" => ui_assets::bar_blue(fill_w, props.h),
        "green" => ui_assets::bar_green(fill_w, props.h),
        "orange" => ui_assets::bar_orange(fill_w, props.h),
        _ => ui_assets::bar_red(fill_w, props.h),
    };

    let track_url = ui_assets::svg_data_url(&track_svg);
    let fill_url = ui_assets::svg_data_url(&fill_svg);

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px;",
            span { style: "color: {c.text_secondary}; font-size: 12px; width: 70px;", "{props.label}" }
            div {
                style: "position: relative; width: {props.w}px; height: {props.h}px;",
                img {
                    src: "{track_url}",
                    style: "position: absolute; top: 0; left: 0; width: {props.w}px; height: {props.h}px;",
                }
                img {
                    src: "{fill_url}",
                    style: "position: absolute; top: 0; left: 0; width: {fill_w}px; height: {props.h}px;",
                }
            }
            span { style: "color: {c.text_muted}; font-size: 11px; width: 36px; text-align: right;", "{props.fill_pct}%" }
        }
    }
}

/// Aperçu d'un curseur avec zone interactive.
#[derive(Props, Clone, PartialEq)]
struct CursorPreviewProps {
    name: &'static str,
    src: &'static str,
}

#[component]
fn CursorPreview(props: CursorPreviewProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; border: 1px solid {c.border}; text-align: center; cursor: url(\"{props.src}\") 0 0, auto; min-height: 100px; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 8px;",
            img {
                src: "{props.src}",
                style: "width: 32px; height: 32px; image-rendering: pixelated;",
            }
            span { style: "font-size: 12px; color: {c.text_secondary};", "{props.name}" }
        }
    }
}

/// Tuile Fantasy UI Borders (48×48, affichée en 2×).
#[derive(Props, Clone, PartialEq)]
struct FantasyTileProps {
    name: &'static str,
    src: &'static str,
}

#[component]
fn FantasyTile(props: FantasyTileProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: 6px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px; border: 1px solid {c.border};",
            img {
                src: "{props.src}",
                style: "width: 96px; height: 96px; image-rendering: pixelated;",
            }
            span { style: "font-size: 11px; color: {c.text_muted};", "{props.name}" }
        }
    }
}

/// Divider Fantasy UI Borders (affiché en largeur étendue).
#[derive(Props, Clone, PartialEq)]
struct FantasyDividerProps {
    name: &'static str,
    src: &'static str,
}

#[component]
fn FantasyDivider(props: FantasyDividerProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px;",
            span { style: "font-size: 11px; color: {c.text_muted}; width: 90px; flex-shrink: 0;", "{props.name}" }
            img {
                src: "{props.src}",
                style: "height: 16px; width: 100%; image-rendering: pixelated; object-fit: contain;",
            }
        }
    }
}
