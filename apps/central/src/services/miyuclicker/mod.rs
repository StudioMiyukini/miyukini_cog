//! Vue jouable MiyuClicker — Idle/Clicker RPG Fantasy intégré à Central.
//!
//! @id: central.services.miyuclicker
//! @role: ui
//! @layer: domain
//! @do: render_miyuclicker_game_view_with_interactive_ui
//! @human: Jeu complet : header ressources, vue domaine, production, bâtiments, barre XP.

use dioxus::prelude::*;
use crate::state::use_app_state;
use crate::services::ui_assets;
use miyuclicker::{idlesim, BuildingType, ClickTarget, GameState};

// ═══════════════════════════════════════════════════════════════════════════════
// Constantes thème RPG Fantasy
// ═══════════════════════════════════════════════════════════════════════════════

const BG_DARK: &str = "#2a1a0a";
const BORDER: &str = "#6D4B27";
const GOLD: &str = "#e8d5a0";
const PARCHMENT: &str = "#FFF1D2";
const BROWN_TEXT: &str = "#5a3a1a";
const BROWN_MID: &str = "#a3703a";

/// Vitesses de jeu disponibles.
const SPEEDS: [(f64, &str); 6] = [
    (idlesim::GAME_SPEED_PAUSE, "⏸"),
    (idlesim::GAME_SPEED_1, "▶"),
    (idlesim::GAME_SPEED_2, "▶▶"),
    (idlesim::GAME_SPEED_3, "▶▶▶"),
    (idlesim::GAME_SPEED_4, "⏩"),
    (idlesim::GAME_SPEED_5, "⏩⏩"),
];

// ═══════════════════════════════════════════════════════════════════════════════
// Vue principale
// ═══════════════════════════════════════════════════════════════════════════════

/// Vue principale du jeu MiyuClicker jouable.
#[component]
pub fn MiyuClickerView() -> Element {
    let _c = use_app_state().read().current_theme.palette();

    // État de jeu — persiste grâce au pattern keep-alive (composant jamais démonté)
    let mut game = use_signal(|| GameState::new_game(1));
    let mut speed = use_signal(|| idlesim::GAME_SPEED_1);
    let mut notif = use_signal(|| "Bienvenue, Seigneur ! Assignez vos ouvriers pour commencer.".to_string());
    let mut last_notif_tick = use_signal(|| 0u64);
    let mut tick_started = use_signal(|| false);

    // Game tick — 100ms IRL.  Démarré une seule fois (persiste entre onglets).
    if !*tick_started.read() {
        tick_started.set(true);
        spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                let current_speed = *speed.read();
                let mut gs = game.write();

                let old_maisons = gs.maisons_lvl;
                let old_caserne = gs.caserne_lvl;
                let old_guilde = gs.guilde_lvl;
                let old_game_over = gs.game_over;

                idlesim::tick(&mut gs, 0.1, current_speed);

                let tick_id = (gs.temps_simule_s * 10.0) as u64;
                if tick_id != *last_notif_tick.peek() {
                    if gs.maisons_lvl > old_maisons {
                        drop(gs);
                        notif.set(format!("Maisons niveau {} ! Pop max +4", game.read().maisons_lvl));
                        last_notif_tick.set(tick_id);
                    } else if gs.caserne_lvl > old_caserne {
                        drop(gs);
                        notif.set(format!("Casernes niveau {} ! Soldats max +5", game.read().caserne_lvl));
                        last_notif_tick.set(tick_id);
                    } else if gs.guilde_lvl > old_guilde {
                        drop(gs);
                        notif.set(format!("Guilde niveau {} ! Batisseurs max +3", game.read().guilde_lvl));
                        last_notif_tick.set(tick_id);
                    } else if gs.game_over && !old_game_over {
                        drop(gs);
                        notif.set("GAME OVER - Votre peuple est mort de faim !".to_string());
                        last_notif_tick.set(tick_id);
                    }
                }
            }
        });
    }

    // SVG paramétriques
    let header_panel = ui_assets::svg_data_url(&ui_assets::panel_brown_dark(900, 90));
    let notif_panel = ui_assets::svg_data_url(&ui_assets::panel_grey(700, 36));
    let prod_panel = ui_assets::svg_data_url(&ui_assets::panel_brown(300, 420));
    let xp_panel = ui_assets::svg_data_url(&ui_assets::panel_brown_dark(900, 32));
    let btn_brown = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN);
    let _btn_grey = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_GREY);

    // Lecture de l'état pour le render
    let gs = game.read();
    let rates = idlesim::production_rates(&gs, *speed.read());
    let clock = GameState::format_clock(gs.temps_simule_s);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 0; background: {BG_DARK}; border-radius: 8px; overflow: hidden; border: 2px solid {BORDER}; height: 100%;",

            // ═══════════════════════════════════════════════
            //  HEADER — Panneau marron sombre SVG
            // ═══════════════════════════════════════════════
            div {
                style: "position: relative; min-height: 90px; flex-shrink: 0;",
                img { src: "{header_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                div {
                    style: "position: relative; z-index: 1; padding: 8px 16px; display: flex; flex-direction: column; gap: 4px;",

                    // Ligne 1 : Pseudo + Ressources + Config
                    div {
                        style: "display: flex; align-items: center; gap: 4px; flex-wrap: wrap;",
                        div {
                            style: "padding: 3px 10px; background: rgba(0,0,0,0.3); border-radius: 4px; border: 1px solid {BORDER}; margin-right: 6px;",
                            span { style: "font-size: 12px; font-weight: 700; color: {GOLD}; letter-spacing: 1px;",
                                "⚜ {gs.pseudo}"
                            }
                        }
                        McRes { icon: "🌾", val: gs.nourriture as i64, max: gs.cap_nourriture() as i64, rate: rates.nourriture }
                        McRes { icon: "🪵", val: gs.bois as i64, max: gs.cap_bois() as i64, rate: rates.bois }
                        McRes { icon: "🪨", val: gs.pierre as i64, max: gs.cap_pierre() as i64, rate: rates.pierre }
                        McRes { icon: "⛏️", val: gs.metal as i64, max: gs.cap_metal() as i64, rate: rates.metal }
                        McRes { icon: "🔧", val: gs.outils as i64, max: gs.cap_outils() as i64, rate: rates.outils }
                        McRes { icon: "⚔️", val: gs.armes as i64, max: gs.cap_armes() as i64, rate: rates.armes }
                        div { style: "flex: 1;" }
                        // Horloge
                        div {
                            style: "padding: 3px 8px; background: rgba(0,0,0,0.3); border-radius: 4px; border: 1px solid {BORDER};",
                            span { style: "font-size: 10px; color: {GOLD};", "🕐 {clock}" }
                        }
                    }

                    // Ligne 2 : Population + Vitesse + Navigation
                    div {
                        style: "display: flex; align-items: center; gap: 4px; flex-wrap: wrap;",
                        McBadge { label: "Pop", value: format!("{}/{}", gs.pop_totale(), gs.pop_max()), color: PARCHMENT }
                        McBadge { label: "Ouvriers", value: format!("{} ({}🔓)", gs.ouvriers, gs.ouvriers_libres()), color: "#ffbe5e" }
                        McBadge { label: "Bâtisseurs", value: format!("{}/{}", gs.batisseurs, gs.cap_batisseurs()), color: "#68b8ff" }
                        McBadge { label: "Soldats", value: format!("{}/{}", gs.soldats, gs.cap_soldats()), color: "#ff7366" }
                        McBadge { label: "Bonheur", value: format!("{}%", gs.bonheur_pourcent()), color: "#81d058" }

                        div { style: "flex: 1;" }

                        // Contrôle vitesse
                        div {
                            style: "display: flex; gap: 3px;",
                            for (spd, label) in SPEEDS {
                                {
                                    let is_active = (*speed.read() - spd).abs() < 0.1;
                                    let bg = if is_active { "rgba(232,213,160,0.3)" } else { "rgba(0,0,0,0.2)" };
                                    rsx! {
                                        div {
                                            style: "padding: 2px 6px; background: {bg}; border-radius: 3px; border: 1px solid {BORDER}; cursor: pointer; font-size: 10px; color: {GOLD};",
                                            onclick: move |_| speed.set(spd),
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  GAME OVER overlay
            // ═══════════════════════════════════════════════
            if gs.game_over {
                div {
                    style: "padding: 16px; background: #5a1010; text-align: center; border-bottom: 2px solid #ff3030;",
                    span { style: "font-size: 18px; font-weight: 700; color: #ff5050;", "💀 GAME OVER — Votre peuple est mort de faim !" }
                    div {
                        style: "margin-top: 8px;",
                        div {
                            style: "position: relative; width: 140px; height: 32px; cursor: pointer; display: inline-block;",
                            img { src: "{btn_brown}", style: "width: 100%; height: 100%;" }
                            span {
                                style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600; color: {BROWN_TEXT};",
                                onclick: move |_| {
                                    game.set(GameState::new_game(1));
                                    notif.set("Nouvelle partie ! Assignez vos ouvriers.".to_string());
                                },
                                "🔄 Nouvelle partie"
                            }
                        }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  VUE DOMAINE — Terrain + PNJ + Château + clics
            // ═══════════════════════════════════════════════
            div {
                style: "position: relative; height: 200px; background: linear-gradient(180deg, #5a7a9a 0%, #7a9a6a 25%, #4a8c3a 40%, #3a6a2a 70%, #2a5a1a 100%); overflow: hidden; border-top: 2px solid {BORDER}; border-bottom: 2px solid {BORDER}; flex-shrink: 0;",

                // Herbe
                div { style: "position: absolute; bottom: 0; left: 0; right: 0; height: 60%; background-image: radial-gradient(circle, #2a6a1a 1px, transparent 1px); background-size: 20px 16px; opacity: 0.12;" }

                // Château
                div {
                    style: "position: absolute; bottom: 28%; left: 50%; transform: translateX(-50%); display: flex; flex-direction: column; align-items: center;",
                    div {
                        style: "display: flex; gap: 48px; margin-bottom: -2px;",
                        div { style: "width: 0; height: 0; border-left: 14px solid transparent; border-right: 14px solid transparent; border-bottom: 20px solid #8B4513;" }
                        div { style: "width: 0; height: 0; border-left: 14px solid transparent; border-right: 14px solid transparent; border-bottom: 20px solid #8B4513;" }
                    }
                    div {
                        style: "display: flex; gap: 44px;",
                        div { style: "width: 18px; height: 36px; background: #b8a080; border: 2px solid #8a7060;" }
                        div { style: "width: 18px; height: 36px; background: #b8a080; border: 2px solid #8a7060;" }
                    }
                    div { style: "width: 70px; height: 50px; background: #c8b090; border: 2px solid #8a7060; margin-top: -38px;" }
                    div { style: "width: 0; height: 0; border-left: 45px solid transparent; border-right: 45px solid transparent; border-bottom: 25px solid #A0522D; margin-top: -52px;" }
                }

                // PNJ Ouvriers (jaune)
                for i in 0..(gs.allocation.total().min(12) as usize) {
                    {
                        let positions: [(f64, f64); 12] = [
                            (8.0, 18.0), (14.0, 22.0), (20.0, 15.0), (5.0, 20.0),
                            (25.0, 17.0), (11.0, 12.0), (85.0, 16.0), (80.0, 20.0),
                            (90.0, 14.0), (17.0, 25.0), (23.0, 10.0), (7.0, 14.0),
                        ];
                        let (left, bottom) = positions[i % 12];
                        rsx! {
                            div {
                                key: "ouvrier_{i}",
                                style: "position: absolute; bottom: {bottom}%; left: {left}%; width: 8px; height: 14px; background: #ffbe5e; border-radius: 3px 3px 1px 1px; border: 1px solid #d4a030; box-shadow: 0 1px 3px rgba(0,0,0,0.4);"
                            }
                        }
                    }
                }

                // PNJ Soldats (rouge)
                for i in 0..(gs.soldats.min(8) as usize) {
                    {
                        let positions: [(f64, f64); 8] = [
                            (42.0, 16.0), (45.0, 14.0), (48.0, 17.0), (52.0, 15.0),
                            (55.0, 13.0), (40.0, 18.0), (57.0, 16.0), (44.0, 12.0),
                        ];
                        let (left, bottom) = positions[i % 8];
                        rsx! {
                            div {
                                key: "soldat_{i}",
                                style: "position: absolute; bottom: {bottom}%; left: {left}%; width: 8px; height: 14px; background: #ff7366; border-radius: 3px 3px 1px 1px; border: 1px solid #c83737; box-shadow: 0 1px 3px rgba(0,0,0,0.4);"
                            }
                        }
                    }
                }

                // PNJ Bâtisseurs (bleu)
                for i in 0..(gs.batisseurs.min(6) as usize) {
                    {
                        let positions: [(f64, f64); 6] = [
                            (63.0, 22.0), (66.0, 25.0), (69.0, 20.0),
                            (72.0, 18.0), (75.0, 23.0), (60.0, 19.0),
                        ];
                        let (left, bottom) = positions[i % 6];
                        rsx! {
                            div {
                                key: "batisseur_{i}",
                                style: "position: absolute; bottom: {bottom}%; left: {left}%; width: 8px; height: 14px; background: #68b8ff; border-radius: 3px 3px 1px 1px; border: 1px solid #3b6891; box-shadow: 0 1px 3px rgba(0,0,0,0.4);"
                            }
                        }
                    }
                }

                // Zones de clic (6 bâtiments de production)
                McClickZone {
                    label: "🌾 Ferme",
                    left: "2%", bottom: "2%", width: "14%",
                    onclick: move |_| {
                        let mut gs = game.write();
                        idlesim::apply_click(&mut gs, ClickTarget::Ferme);
                    }
                }
                McClickZone {
                    label: "🪵 Scierie",
                    left: "17%", bottom: "2%", width: "14%",
                    onclick: move |_| {
                        let mut gs = game.write();
                        idlesim::apply_click(&mut gs, ClickTarget::Scierie);
                    }
                }
                McClickZone {
                    label: "🪨 Carrière",
                    left: "32%", bottom: "2%", width: "14%",
                    onclick: move |_| {
                        let mut gs = game.write();
                        idlesim::apply_click(&mut gs, ClickTarget::Carriere);
                    }
                }
                McClickZone {
                    label: "⛏️ Mine",
                    left: "54%", bottom: "2%", width: "14%",
                    onclick: move |_| {
                        let mut gs = game.write();
                        idlesim::apply_click(&mut gs, ClickTarget::Mine);
                    }
                }
                McClickZone {
                    label: "🔧 Atelier",
                    left: "69%", bottom: "2%", width: "14%",
                    onclick: move |_| {
                        let mut gs = game.write();
                        idlesim::apply_click(&mut gs, ClickTarget::Atelier);
                    }
                }
                McClickZone {
                    label: "⚔️ Forge",
                    left: "84%", bottom: "2%", width: "14%",
                    onclick: move |_| {
                        let mut gs = game.write();
                        idlesim::apply_click(&mut gs, ClickTarget::Forge);
                    }
                }

                // Légende PNJ
                div {
                    style: "position: absolute; top: 8px; left: 8px; display: flex; gap: 10px; padding: 4px 8px; background: rgba(42,26,10,0.8); border-radius: 4px; border: 1px solid {BORDER};",
                    McLegendDot { color: "#ffbe5e", label: "Ouvriers" }
                    McLegendDot { color: "#ff7366", label: "Soldats" }
                    McLegendDot { color: "#68b8ff", label: "Bâtisseurs" }
                }
            }

            // ═══════════════════════════════════════════════
            //  BARRE DE NOTIFICATION
            // ═══════════════════════════════════════════════
            div {
                style: "display: flex; align-items: center; justify-content: center; padding: 6px 16px; background: {BG_DARK}; flex-shrink: 0;",
                div {
                    style: "position: relative; flex: 1; max-width: 700px; height: 36px;",
                    img { src: "{notif_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                    div {
                        style: "position: relative; z-index: 1; display: flex; align-items: center; justify-content: center; height: 36px;",
                        span { style: "font-size: 11px; color: {BROWN_TEXT}; font-weight: 500;", "📜 {notif}" }
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  PANNEAU BAS — Production + Bâtiments (scrollable)
            // ═══════════════════════════════════════════════
            div {
                style: "display: flex; gap: 12px; padding: 12px; flex-wrap: wrap; background: {BG_DARK}; flex: 1; min-height: 0; overflow-y: auto;",

                // ── Colonne gauche : Production ──
                div {
                    style: "display: flex; flex-direction: column; gap: 0; min-width: 280px; flex: 1;",

                    div {
                        style: "position: relative; min-height: 420px;",
                        img { src: "{prod_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                        div {
                            style: "position: relative; z-index: 1; padding: 18px;",

                            div {
                                style: "font-size: 13px; font-weight: 700; color: {BROWN_TEXT}; letter-spacing: 1px; text-align: center; margin-bottom: 4px;",
                                "⚒ La production"
                            }
                            div { style: "width: 120px; height: 1px; background: linear-gradient(90deg, transparent, {BROWN_MID}, transparent); margin: 0 auto 8px;" }

                            // Lignes de production
                            McProdRow {
                                name: "Ferme", icon: "🌾",
                                count: gs.allocation.ferme,
                                rate: format!("{:.1}/j", gs.allocation.ferme as f64 * 2.0),
                                on_plus: move |_| {
                                    let mut gs = game.write();
                                    if gs.ouvriers_libres() > 0 { gs.allocation.ferme += 1; }
                                },
                                on_minus: move |_| {
                                    let mut gs = game.write();
                                    if gs.allocation.ferme > 0 { gs.allocation.ferme -= 1; }
                                },
                            }
                            McProdRow {
                                name: "Scierie", icon: "🪵",
                                count: gs.allocation.scierie,
                                rate: format!("{:.1}/j", gs.allocation.scierie as f64 * 1.0),
                                on_plus: move |_| {
                                    let mut gs = game.write();
                                    if gs.ouvriers_libres() > 0 { gs.allocation.scierie += 1; }
                                },
                                on_minus: move |_| {
                                    let mut gs = game.write();
                                    if gs.allocation.scierie > 0 { gs.allocation.scierie -= 1; }
                                },
                            }
                            McProdRow {
                                name: "Carrière", icon: "🪨",
                                count: gs.allocation.carriere,
                                rate: format!("{:.1}/j", gs.allocation.carriere as f64 * 1.0),
                                on_plus: move |_| {
                                    let mut gs = game.write();
                                    if gs.ouvriers_libres() > 0 { gs.allocation.carriere += 1; }
                                },
                                on_minus: move |_| {
                                    let mut gs = game.write();
                                    if gs.allocation.carriere > 0 { gs.allocation.carriere -= 1; }
                                },
                            }
                            McProdRow {
                                name: "Mine", icon: "⛏️",
                                count: gs.allocation.mine,
                                rate: format!("{:.1}/j", gs.allocation.mine as f64 * 0.5),
                                on_plus: move |_| {
                                    let mut gs = game.write();
                                    if gs.ouvriers_libres() > 0 { gs.allocation.mine += 1; }
                                },
                                on_minus: move |_| {
                                    let mut gs = game.write();
                                    if gs.allocation.mine > 0 { gs.allocation.mine -= 1; }
                                },
                            }
                            McProdRow {
                                name: "Atelier", icon: "🔧",
                                count: gs.allocation.atelier,
                                rate: format!("{:.1}/j", gs.allocation.atelier as f64 * 0.5),
                                on_plus: move |_| {
                                    let mut gs = game.write();
                                    if gs.ouvriers_libres() > 0 { gs.allocation.atelier += 1; }
                                },
                                on_minus: move |_| {
                                    let mut gs = game.write();
                                    if gs.allocation.atelier > 0 { gs.allocation.atelier -= 1; }
                                },
                            }
                            McProdRow {
                                name: "Forge", icon: "⚔️",
                                count: gs.allocation.forge,
                                rate: format!("{:.1}/j", gs.allocation.forge as f64 * 0.2),
                                on_plus: move |_| {
                                    let mut gs = game.write();
                                    if gs.ouvriers_libres() > 0 { gs.allocation.forge += 1; }
                                },
                                on_minus: move |_| {
                                    let mut gs = game.write();
                                    if gs.allocation.forge > 0 { gs.allocation.forge -= 1; }
                                },
                            }

                            // Séparateur
                            div { style: "width: 100%; height: 1px; background: linear-gradient(90deg, transparent, {BROWN_MID}, transparent); margin: 10px 0;" }

                            // Conversion Ouvrier → Bâtisseur / Soldat
                            div {
                                style: "display: flex; flex-direction: column; gap: 4px; align-items: center;",

                                McConvertBtn {
                                    label: "+ 1 Bâtisseur",
                                    cost: format!("🔧{:.0} outils", gs.cout_batisseur()),
                                    color: "#68b8ff",
                                    enabled: gs.can_convert_to_batisseur(),
                                    onclick: move |_| {
                                        let mut gs = game.write();
                                        idlesim::convert_ouvrier_to_batisseur(&mut gs);
                                    },
                                }
                                McConvertBtn {
                                    label: "+ 1 Soldat",
                                    cost: format!("⚔️{:.0} armes", gs.cout_soldat()),
                                    color: "#ff7366",
                                    enabled: gs.can_convert_to_soldat(),
                                    onclick: move |_| {
                                        let mut gs = game.write();
                                        idlesim::convert_ouvrier_to_soldat(&mut gs);
                                    },
                                }
                            }
                        }
                    }
                }

                // ── Colonne droite : Bâtiments ──
                div {
                    style: "display: flex; flex-direction: column; gap: 10px; flex: 1.5; min-width: 380px;",

                    McBuildCard {
                        game: game,
                        notif: notif,
                        building: BuildingType::Maison,
                        name: "🏠 Maisons",
                        desc: "Augmente la population maximale (+4/nv)",
                        accent: "#e8a030",
                    }
                    McBuildCard {
                        game: game,
                        notif: notif,
                        building: BuildingType::Caserne,
                        name: "🏰 Casernes",
                        desc: "Augmente le cap soldats (+5/nv)",
                        accent: "#ff7366",
                    }
                    McBuildCard {
                        game: game,
                        notif: notif,
                        building: BuildingType::GuildeDesMacons,
                        name: "🔨 Guilde des Maçons",
                        desc: "Augmente le cap bâtisseurs (+3/nv)",
                        accent: "#a855f7",
                    }
                }
            }

            // ═══════════════════════════════════════════════
            //  BARRE XP
            // ═══════════════════════════════════════════════
            div {
                style: "position: relative; height: 32px; flex-shrink: 0;",
                img { src: "{xp_panel}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
                div {
                    style: "position: relative; z-index: 1; display: flex; align-items: center; gap: 8px; padding: 0 16px; height: 32px;",

                    // "Niveau" = maisons_lvl + caserne_lvl + guilde_lvl
                    {
                        let total_lvl = gs.maisons_lvl + gs.caserne_lvl + gs.guilde_lvl;
                        rsx! {
                            span { style: "font-size: 10px; color: {GOLD}; font-weight: 700;", "⭐ Nv.{total_lvl}" }
                        }
                    }
                    // Barre XP basée sur le temps
                    {
                        let xp_current = (gs.temps_simule_s / 86400.0 * 100.0) as i64; // 100 XP/jour
                        let xp_next_lvl = ((gs.maisons_lvl + gs.caserne_lvl + gs.guilde_lvl) * 500) as i64;
                        let xp_pct = if xp_next_lvl > 0 { (xp_current % xp_next_lvl.max(1)) as f64 / xp_next_lvl as f64 * 100.0 } else { 0.0 };
                        let xp_pct_clamped = xp_pct.clamp(2.0, 100.0);
                        let track = ui_assets::svg_data_url(&ui_assets::bar_track(600, 12));
                        let fill_w = (600.0 * xp_pct_clamped / 100.0).max(12.0) as u32;
                        let fill = ui_assets::svg_data_url(&ui_assets::bar_blue(fill_w, 12));
                        rsx! {
                            div {
                                style: "position: relative; flex: 1; height: 12px;",
                                img { src: "{track}", style: "position: absolute; inset: 0; width: 100%; height: 12px;" }
                                img { src: "{fill}", style: "position: absolute; top: 0; left: 0; width: {xp_pct_clamped}%; height: 12px;" }
                            }
                            span { style: "font-size: 10px; color: {BROWN_MID};", "{xp_current} / {xp_next_lvl} XP" }
                        }
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Sous-composants
// ═══════════════════════════════════════════════════════════════════════════════

/// Ressource dans le header.
#[component]
fn McRes(icon: &'static str, val: i64, max: i64, rate: f64) -> Element {
    let pct = if max > 0 { val as f64 / max as f64 * 100.0 } else { 0.0 };
    let color = if pct > 80.0 { "#81d058" } else if pct > 40.0 { GOLD } else { "#ff7366" };
    let rate_str = if rate.abs() < 0.001 { String::new() } else { format!(" ({:+.1}/s)", rate) };
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 3px; padding: 2px 6px; background: rgba(0,0,0,0.25); border-radius: 3px; border: 1px solid {BORDER};",
            span { style: "font-size: 11px;", "{icon}" }
            span { style: "font-size: 10px; color: {color}; font-weight: 600;", "{val}" }
            span { style: "font-size: 8px; color: {BROWN_MID};", "/{max}{rate_str}" }
        }
    }
}

/// Badge population header.
#[component]
fn McBadge(label: &'static str, value: String, color: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 3px; padding: 2px 6px; background: rgba(0,0,0,0.2); border-radius: 3px; border: 1px solid {BORDER};",
            span { style: "font-size: 9px; color: {BROWN_MID};", "{label}" }
            span { style: "font-size: 10px; color: {color}; font-weight: 600;", "{value}" }
        }
    }
}

/// Point de légende PNJ.
#[component]
fn McLegendDot(color: &'static str, label: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 4px;",
            div { style: "width: 8px; height: 8px; background: {color}; border-radius: 2px;" }
            span { style: "font-size: 9px; color: {PARCHMENT};", "{label}" }
        }
    }
}

/// Zone de clic sur la carte (production manuelle).
#[component]
fn McClickZone(label: &'static str, left: &'static str, bottom: &'static str, width: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            style: "position: absolute; left: {left}; bottom: {bottom}; width: {width}; height: 24px; background: rgba(42,26,10,0.6); border: 1px solid {BORDER}; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: background 0.15s;",
            onclick: move |e| onclick.call(e),
            span { style: "font-size: 10px; color: {PARCHMENT}; font-weight: 500;", "{label}" }
        }
    }
}

/// Ligne de production (ouvriers alloués).
#[component]
fn McProdRow(
    name: &'static str,
    icon: &'static str,
    count: i64,
    rate: String,
    on_plus: EventHandler<MouseEvent>,
    on_minus: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px; padding: 4px 6px; background: rgba(0,0,0,0.12); border-radius: 4px; border: 1px solid rgba(109,75,39,0.3); margin-bottom: 3px;",
            span { style: "font-size: 12px; width: 20px; text-align: center;", "{icon}" }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                span { style: "font-size: 10px; color: {BROWN_TEXT}; font-weight: 600;", "{name}" }
                span { style: "font-size: 8px; color: {BROWN_MID};", "{rate}" }
            }
            div {
                style: "width: 18px; height: 18px; background: #4a7a3a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 11px; font-weight: 700; color: #d0f0a0; border: 1px solid #5a8a4a;",
                onclick: move |e| on_plus.call(e),
                "+"
            }
            span { style: "font-size: 11px; color: {BROWN_TEXT}; font-weight: 700; min-width: 20px; text-align: center;", "{count}" }
            div {
                style: "width: 18px; height: 18px; background: #7a3a2a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 11px; font-weight: 700; color: #ffb0a0; border: 1px solid #8a4a3a;",
                onclick: move |e| on_minus.call(e),
                "−"
            }
        }
    }
}

/// Bouton de conversion (Ouvrier → Bâtisseur/Soldat).
#[component]
fn McConvertBtn(
    label: &'static str,
    cost: String,
    color: &'static str,
    enabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let opacity = if enabled { "1.0" } else { "0.4" };
    let cursor = if enabled { "pointer" } else { "not-allowed" };
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding: 4px 12px; background: rgba(0,0,0,0.15); border: 1px solid {color}40; border-radius: 4px; cursor: {cursor}; opacity: {opacity}; width: 220px;",
            onclick: move |e| { if enabled { onclick.call(e); } },
            div { style: "width: 8px; height: 8px; border-radius: 50%; background: {color};" }
            span { style: "font-size: 10px; color: {PARCHMENT}; font-weight: 600; flex: 1;", "{label}" }
            span { style: "font-size: 9px; color: {BROWN_MID};", "{cost}" }
        }
    }
}

/// Carte de bâtiment avec gestion construction.
#[component]
fn McBuildCard(
    game: Signal<GameState>,
    notif: Signal<String>,
    building: BuildingType,
    name: &'static str,
    desc: &'static str,
    accent: &'static str,
) -> Element {
    let gs = game.read();
    let (lvl, paid, builders_assigned) = match building {
        BuildingType::Maison => (gs.maisons_lvl, gs.construction_maison_paid, gs.allocation_batisseurs.maison),
        BuildingType::Caserne => (gs.caserne_lvl, gs.construction_caserne_paid, gs.allocation_batisseurs.caserne),
        BuildingType::GuildeDesMacons => (gs.guilde_lvl, gs.construction_guilde_paid, gs.allocation_batisseurs.guilde_des_macons),
    };
    let progress = idlesim::construction_progress_pct(&gs, building);
    let cost = gs.cout_construction(building);
    let can_start = gs.can_start_construction(building);
    let can_add_builder = gs.batisseurs_libres() > 0 && paid;
    let card_bg = ui_assets::svg_data_url(&ui_assets::panel_brown(500, 120));
    let btn_svg = ui_assets::svg_data_url(ui_assets::svg_fixed::BUTTON_BROWN);
    let track = ui_assets::svg_data_url(&ui_assets::bar_track(300, 14));
    let fill_w = ((300.0 * progress / 100.0) as u32).max(12);
    let fill = ui_assets::svg_data_url(&ui_assets::bar_green(fill_w, 14));

    // Pré-calculer les chaînes pour éviter les problèmes de format dans rsx
    let progress_label = if paid { format!("{:.0}%", progress) } else { "—".to_string() };
    let add_bg = if can_add_builder { "#4a7a3a" } else { "#4a4a3a" };
    let cost_bois = format!("{:.0}", cost.bois);
    let cost_pierre = format!("{:.0}", cost.pierre);
    let cost_metal = format!("{:.0}", cost.metal);
    let build_opacity = if can_start { "1.0" } else { "0.4" };
    let build_cursor = if can_start { "pointer" } else { "not-allowed" };

    rsx! {
        div {
            style: "position: relative; min-height: 120px;",
            img { src: "{card_bg}", style: "position: absolute; inset: 0; width: 100%; height: 100%;" }
            div { style: "position: absolute; left: 0; top: 8px; bottom: 8px; width: 4px; border-radius: 0 3px 3px 0; background: {accent};" }
            div {
                style: "position: relative; z-index: 1; padding: 12px 12px 12px 16px; display: flex; flex-direction: column; gap: 5px;",

                // Header
                div {
                    style: "display: flex; align-items: center; justify-content: space-between;",
                    span { style: "font-size: 12px; font-weight: 700; color: {BROWN_TEXT};", "{name}" }
                    div {
                        style: "padding: 2px 8px; background: rgba(0,0,0,0.15); border-radius: 4px; border: 1px solid {BORDER};",
                        span { style: "font-size: 10px; font-weight: 700; color: {accent};", "LVL {lvl}" }
                    }
                }

                // Description
                span { style: "font-size: 9px; color: {BROWN_MID};", "{desc}" }

                // Barre de progression
                div {
                    style: "display: flex; align-items: center; gap: 6px;",
                    div {
                        style: "position: relative; flex: 1; height: 14px;",
                        img { src: "{track}", style: "position: absolute; inset: 0; width: 100%; height: 14px;" }
                        img { src: "{fill}", style: "position: absolute; top: 0; left: 0; width: {progress}%; height: 14px;" }
                    }
                    span { style: "font-size: 9px; color: {BROWN_MID}; min-width: 28px; text-align: right;", "{progress_label}" }
                }

                // Actions
                div {
                    style: "display: flex; align-items: center; gap: 5px; flex-wrap: wrap;",
                    span { style: "font-size: 9px; color: #68b8ff;", "Bâtisseurs" }
                    div {
                        style: "width: 16px; height: 16px; background: {add_bg}; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 10px; font-weight: 700; color: #d0f0a0;",
                        onclick: {
                            let building = building;
                            move |_| {
                                let mut gs = game.write();
                                if gs.batisseurs_libres() > 0 {
                                    match building {
                                        BuildingType::Maison => gs.allocation_batisseurs.maison += 1,
                                        BuildingType::Caserne => gs.allocation_batisseurs.caserne += 1,
                                        BuildingType::GuildeDesMacons => gs.allocation_batisseurs.guilde_des_macons += 1,
                                    }
                                }
                            }
                        },
                        "+"
                    }
                    span { style: "font-size: 10px; color: {BROWN_TEXT}; font-weight: 700; min-width: 14px; text-align: center;", "{builders_assigned}" }
                    div {
                        style: "width: 16px; height: 16px; background: #7a3a2a; border-radius: 3px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 10px; font-weight: 700; color: #ffb0a0;",
                        onclick: {
                            let building = building;
                            move |_| {
                                let mut gs = game.write();
                                match building {
                                    BuildingType::Maison => { if gs.allocation_batisseurs.maison > 0 { gs.allocation_batisseurs.maison -= 1; } },
                                    BuildingType::Caserne => { if gs.allocation_batisseurs.caserne > 0 { gs.allocation_batisseurs.caserne -= 1; } },
                                    BuildingType::GuildeDesMacons => { if gs.allocation_batisseurs.guilde_des_macons > 0 { gs.allocation_batisseurs.guilde_des_macons -= 1; } },
                                }
                            }
                        },
                        "−"
                    }

                    // Coûts
                    div {
                        style: "display: flex; gap: 4px; margin-left: auto;",
                        span { style: "font-size: 8px; color: {BROWN_MID};", "B:{cost_bois}" }
                        span { style: "font-size: 8px; color: {BROWN_MID};", "P:{cost_pierre}" }
                        span { style: "font-size: 8px; color: {BROWN_MID};", "M:{cost_metal}" }
                    }

                    // Bouton Construire
                    if !paid {
                        div {
                            style: "position: relative; width: 90px; height: 24px; cursor: {build_cursor}; opacity: {build_opacity}; margin-left: 4px;",
                            img { src: "{btn_svg}", style: "width: 100%; height: 100%;" }
                            span {
                                style: "position: absolute; inset: 0; display: flex; align-items: center; justify-content: center; font-size: 9px; font-weight: 600; color: {BROWN_TEXT};",
                                onclick: {
                                    let building = building;
                                    move |_| {
                                        let mut gs = game.write();
                                        if gs.can_start_construction(building) {
                                            idlesim::start_construction(&mut gs, building);
                                        }
                                    }
                                },
                                "Construire"
                            }
                        }
                    } else {
                        span { style: "font-size: 9px; color: #81d058; font-weight: 600; margin-left: 4px;", "En cours..." }
                    }
                }
            }
        }
    }
}
