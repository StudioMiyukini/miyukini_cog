//! @id     games_allumina_main
//! @do     definir_dt
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « main ».

//! Allumina entry point.
//!
//! * Default: open a window and play (isometric wgpu renderer + winit loop).
//! * `--headless`: run the original scripted hunt and print the loop to stdout.
//! * `--class <soldat|clerc|mage|archer>`: pick a starting class.
//! * `--server [--port N]`: run the authoritative TCP server (needs the
//!   `netcode` feature). Defaults to `0.0.0.0:7777`.
//! * `--connect [<host>] [--port N]`: run the multiplayer client (needs
//!   `netcode`). Host defaults to `allumina.miyukini-home.org`.
//! * `--ws-bridge [--port N] [--web-port N]`: run the browser bridge — it serves
//!   the `web/` client *and* the WebSocket on one `--web-port` (default 7000, the
//!   Cloudflare tunnel's local target) and relays to the TCP server on `--port`
//!   (default 7777). Needs `webclient`; auto-started by `--server` in that build.

use allumina::build_app;
use allumina::build_app_in;
use allumina::components::player::ClassType;
use allumina::components::world::ZoneId;
use allumina::ecs::{Entity, World};
use allumina::plugins::player::PlayerCommand;
use allumina::plugins::tilemap::render_ascii;
use allumina::ui::equipment_ui::render_equipment;
use allumina::ui::hud::render_hud;
use allumina::ui::inventory_ui::render_inventory;
use allumina::ui::skill_tree_ui::render_skill_tree;
use allumina::ui::tooltip::item_tooltip;
use allumina::plugins::inventory::equip_from_inventory;

const DT: f32 = 1.0 / 30.0;
const MAX_STEPS: u32 = 6000;

fn parse_class(args: &[String]) -> ClassType {
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if a == "--class" {
            if let Some(v) = it.next() {
                return match v.to_lowercase().as_str() {
                    "clerc" | "cleric" | "priest" => ClassType::Clerc,
                    "mage" | "magedeguerre" | "warmage" | "sorcerer" => ClassType::MageDeGuerre,
                    "archer" | "ranger" | "hunter" => ClassType::Archer,
                    _ => ClassType::Soldat,
                };
            }
        }
    }
    ClassType::Soldat
}

fn arg_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).map(String::as_str)
}

fn parse_port(args: &[String]) -> u16 {
    arg_value(args, "--port").and_then(|v| v.parse().ok()).unwrap_or(0)
}

/// Port the browser bridge serves HTTP + WebSocket on (`--web-port N`). Defaults
/// to 7000 — the local target of the `allumina.miyukini-home.org` Cloudflare
/// tunnel. (Literal so the parser stays usable without the `webclient` feature.)
fn parse_web_port(args: &[String]) -> u16 {
    arg_value(args, "--web-port").and_then(|v| v.parse().ok()).unwrap_or(7000)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let class_overridden = args.iter().any(|a| a == "--class");
    let class = parse_class(&args);

    if args.iter().any(|a| a == "--server") {
        return launch_server(parse_port(&args), parse_web_port(&args));
    }
    // Standalone WebSocket bridge (assumes a TCP server is already running).
    if args.iter().any(|a| a == "--ws-bridge") {
        return launch_ws_bridge(parse_port(&args), parse_web_port(&args));
    }
    if args.iter().any(|a| a == "--connect") {
        // Accept `--connect` alone (use default host) or `--connect <host>`.
        let host = arg_value(&args, "--connect")
            .filter(|s| !s.starts_with("--"))
            .map(str::to_string);
        return launch_client(host, parse_port(&args), class);
    }
    if args.iter().any(|a| a == "--headless") {
        // Headless mode bypasses the title/class-select flow and runs the
        // scripted hunt directly (used by tests + CI).
        let chosen = if class_overridden { class } else { ClassType::Soldat };
        run_headless(chosen);
        return Ok(());
    }

    let title = if class_overridden {
        format!("Allumina — {}", class.display_name())
    } else {
        "Allumina".to_string()
    };

    // When `--class` is passed, skip the title/class-select screens and go
    // straight into a new game. Otherwise let the title screen drive the flow.
    if class_overridden {
        let app = build_app(class, 0xA11CE);
        allumina::render::run_game(Some(app), title, None)
    } else {
        allumina::render::run_game(None, title, None)
    }
}

// ── Networking launchers (feature-gated) ─────────────────────────────────────

#[cfg(feature = "netcode")]
fn launch_server(port: u16, web_port: u16) -> anyhow::Result<()> {
    // With the browser bridge compiled in, start it automatically alongside the
    // TCP server (one command spins up native + web play).
    #[cfg(feature = "webclient")]
    {
        let resolved = if port == 0 { allumina::network::DEFAULT_PORT } else { port };
        std::thread::spawn(move || allumina::network::ws_bridge::run(web_port, resolved));
    }
    #[cfg(not(feature = "webclient"))]
    let _ = web_port;
    allumina::network::server::run_server(port)
}

#[cfg(not(feature = "netcode"))]
fn launch_server(_port: u16, _web_port: u16) -> anyhow::Result<()> {
    eprintln!("This build has no networking. Rebuild with: cargo run -p allumina --features netcode -- --server");
    Ok(())
}

#[cfg(feature = "webclient")]
fn launch_ws_bridge(port: u16, web_port: u16) -> anyhow::Result<()> {
    let resolved = if port == 0 { allumina::network::DEFAULT_PORT } else { port };
    allumina::network::ws_bridge::run(web_port, resolved)
}

#[cfg(not(feature = "webclient"))]
fn launch_ws_bridge(_port: u16, _web_port: u16) -> anyhow::Result<()> {
    eprintln!("This build has no web bridge. Rebuild with: cargo run -p allumina --features webclient -- --ws-bridge");
    Ok(())
}

#[cfg(feature = "netcode")]
fn launch_client(host: Option<String>, port: u16, class: ClassType) -> anyhow::Result<()> {
    // `host`/`port` are honoured by the lobby connect path (which reads
    // DEFAULT_SERVER/DEFAULT_PORT). For now we always dial the default
    // endpoint; supporting a custom host through the GUI lobby is a follow-up.
    let _ = host;
    let _ = port;
    let title = format!("Allumina — {} (multi)", class.display_name());
    allumina::render::renderer::run_game_connect(title, class)
}

#[cfg(not(feature = "netcode"))]
fn launch_client(_host: Option<String>, _port: u16, _class: ClassType) -> anyhow::Result<()> {
    eprintln!("This build has no networking. Rebuild with: cargo run -p allumina --features netcode -- --connect [host]");
    Ok(())
}

// ── Headless scripted run (original MVP driver) ──────────────────────────────

fn nearest_mob(world: &World, player: Entity) -> Option<Entity> {
    let ppos = *world.positions.get(&player)?;
    world
        .mob_entities()
        .into_iter()
        .filter(|e| world.health.get(e).is_some_and(|h| !h.is_empty()))
        .filter_map(|e| world.positions.get(&e).map(|p| (e, ppos.distance_sq(*p))))
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

fn decide_command(world: &mut World) {
    let Some(player) = world.player_entity else { return };
    let Some(&ppos) = world.positions.get(&player) else { return };

    if let Some(item_e) = world
        .ground_item_entities()
        .into_iter()
        .find(|e| world.positions.get(e).is_some_and(|p| ppos.distance(*p) <= 1.8))
    {
        world.input.command = Some(PlayerCommand::Pickup(item_e));
        return;
    }

    let current = world.combat.get(&player).and_then(|c| c.target);
    let target_alive = current.is_some_and(|t| {
        world.health.get(&t).is_some_and(|h| !h.is_empty()) && world.positions.contains_key(&t)
    });

    if let (true, Some(target)) = (target_alive, current) {
        let tpos = *world.positions.get(&target).unwrap();
        let dist = ppos.distance(tpos);
        let range = world.combat.get(&player).map_or(1.8, |c| c.range);

        let sel = world.skills.get(&player).map_or(0, |s| s.selected);
        if let Some(skill) = world.skills.get(&player).and_then(|s| s.selected_skill().cloned()) {
            let cd_ready = world
                .skills
                .get(&player)
                .and_then(|s| s.cooldowns.get(sel))
                .is_some_and(|c| c.is_ready());
            let mana_ok = world.mana.get(&player).is_some_and(|m| m.current >= skill.mana_cost);
            if cd_ready && mana_ok && dist <= skill.range {
                world.input.command = Some(PlayerCommand::UseSkill { idx: sel, target });
                return;
            }
        }
        if dist > range && !world.move_targets.contains_key(&player) {
            world.input.command = Some(PlayerCommand::Attack(target));
        }
        return;
    }

    if let Some(mob) = nearest_mob(world, player) {
        world.input.command = Some(PlayerCommand::Attack(mob));
    }
}

fn flush_log(world: &mut World) {
    let tick = world.tick;
    for line in world.drain_log() {
        println!("[t{tick:>4}] {line}");
    }
}

fn run_headless(class: ClassType) {
    // Start in the first hunting zone so the scripted hunt has hostiles to clear.
    let mut app = build_app_in(class, 0xA11CE, ZoneId::GreenPlains);

    println!("=== ALLUMINA — headless MVP ({}) ===\n", class.display_name());
    flush_log(&mut app.world);
    println!("\nMap:\n{}\n", render_ascii(&app.world));

    let mut step = 0;
    while step < MAX_STEPS {
        decide_command(&mut app.world);
        app.tick(DT);
        flush_log(&mut app.world);
        app.world.render_events.clear();
        step += 1;
        if app.world.mob_entities().is_empty() {
            println!("\n*** All hostiles cleared in {step} ticks. ***");
            break;
        }
    }
    if !app.world.mob_entities().is_empty() {
        println!("\n*** Step cap reached; {} mobs remain. ***", app.world.mob_entities().len());
    }

    let player = app.world.player_entity.expect("player exists");
    println!("\n----- HUD -----\n{}", render_hud(&app.world));
    println!("\n----- MAP -----\n{}", render_ascii(&app.world));

    if let Some(inv) = app.world.inventory.get(&player) {
        println!("\n----- INVENTORY -----");
        println!("{}", render_inventory(inv));
    }

    let first_equippable = app
        .world
        .inventory
        .get(&player)
        .and_then(|inv| inv.slots.iter().position(|s| s.as_ref().is_some_and(|i| i.slot().is_some())));

    if let Some(idx) = first_equippable {
        if let Some(item) = app.world.inventory.get(&player).and_then(|inv| inv.get(idx).cloned()) {
            println!("\n----- ITEM TOOLTIP -----\n{}", item_tooltip(&item));
        }
        equip_from_inventory(&mut app.world, player, idx);
        flush_log(&mut app.world);
    }

    if let Some(eq) = app.world.equipment.get(&player) {
        println!("\n----- EQUIPMENT -----\n{}", render_equipment(eq));
    }
    if let Some(tree) = app.world.skill_trees.get(&player) {
        println!("\n----- SKILLS -----\n{}", render_skill_tree(tree));
    }
    println!("\n----- HUD (after equip) -----\n{}", render_hud(&app.world));
}
