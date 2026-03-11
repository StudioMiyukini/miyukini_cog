use crate::constants::*;
use crate::entity::*;
use crate::player_state::PlayerState;
use crate::zone_state::ZoneState;

/// Outcome events emitted by a simulation tick.
/// The client (or server) can react to these for UI, sound, etc.
#[derive(Debug)]
pub enum SimEvent {
    DamageFloat {
        wx: f32,
        wy: f32,
        value: f32,
        is_heal: bool,
    },
    EnemyKilled {
        index: usize,
        xp: f32,
        pos: [f32; 2],
    },
    PlayerHit {
        damage: f32,
    },
    LevelUp {
        new_level: u32,
    },
    LogMessage {
        text: [u8; 40],
        len: usize,
        color: [f32; 4],
    },
}

/// Run one simulation tick on the zone + player.
/// Returns events that happened this tick.
///
/// NOTE: This is a scaffolding function. The full game logic will be migrated
/// incrementally from `SodomightApp` methods into here. For now, it handles
/// the core loops that benefit from spatial optimization.
pub fn sim_tick(zone: &mut ZoneState, player: &mut PlayerState, events: &mut Vec<SimEvent>) {
    if player.dead {
        update_death(player);
        return;
    }

    // Regen
    update_regen(player);

    // Poison
    update_poison(player, events);

    // Buffs countdown
    update_buffs(player);

    // Enemy AI
    update_enemies(zone, player);

    // Projectile movement
    update_projectiles(zone, player, events);

    // Lightning bolt visuals
    zone.lightning_bolts.retain_mut(|b| {
        b.life = b.life.saturating_sub(1);
        b.life > 0
    });

    // Drop lifetime
    zone.drops.retain_mut(|d| {
        d.life = d.life.saturating_sub(1);
        d.life > 0
    });

    // Float lifetime
    zone.floats.retain_mut(|f| {
        f.life = f.life.saturating_sub(1);
        f.dy -= 0.02;
        f.life > 0
    });

    // Enemy respawn
    update_enemy_respawn(zone);

    // Mercenary
    update_mercenary(zone, player, events);
}

fn update_death(player: &mut PlayerState) {
    if player.respawn_timer > 0 {
        player.respawn_timer -= 1;
        if player.respawn_timer == 0 {
            player.dead = false;
            player.hp = player.max_hp() * 0.5;
            player.mp = player.max_mp() * 0.5;
        }
    }
}

fn update_regen(player: &mut PlayerState) {
    let hr = player.hp_regen();
    let mr = player.mp_regen();
    let mhp = player.max_hp();
    let mmp = player.max_mp();
    player.hp = (player.hp + hr).min(mhp);
    player.mp = (player.mp + mr).min(mmp);
}

fn update_poison(player: &mut PlayerState, events: &mut Vec<SimEvent>) {
    if player.poison_timer > 0 {
        player.poison_timer -= 1;
        player.hp -= POISON_TICK_DMG;
        if player.hp <= 0.0 {
            player.hp = 0.0;
            player.dead = true;
            player.respawn_timer = RESPAWN_DELAY;
            // XP death penalty
            player.xp = (player.xp - player.xp * 0.10).max(0.0);
            events.push(SimEvent::PlayerHit {
                damage: POISON_TICK_DMG,
            });
        }
    }
}

fn update_buffs(player: &mut PlayerState) {
    player.buffs.retain_mut(|b| {
        b.remaining = b.remaining.saturating_sub(1);
        b.remaining > 0
    });
}

fn update_enemies(zone: &mut ZoneState, player: &PlayerState) {
    let pp = player.pos;
    // Pre-compute shaman alive flag to avoid borrow conflict in loop
    let shaman_alive = zone
        .enemies
        .iter()
        .any(|e| e.kind == EnemyKind::FallenShaman && e.alive);
    for enemy in &mut zone.enemies {
        if !enemy.alive {
            continue;
        }
        if enemy.attack_cd > 0 {
            enemy.attack_cd -= 1;
        }
        let d = dist(enemy.pos, pp);
        if d < ENEMY_CHASE_RANGE && d > ENEMY_ATTACK_RANGE {
            let dx = pp[0] - enemy.pos[0];
            let dy = pp[1] - enemy.pos[1];
            let len = (dx * dx + dy * dy).sqrt().max(0.001);
            // Fallen flee if shaman dead + low HP
            let speed = if enemy.kind == EnemyKind::Fallen
                && enemy.hp < enemy.max_hp * 0.5
                && !shaman_alive
            {
                -(ENEMY_SPEED * 1.5)
            } else {
                ENEMY_SPEED
            };
            enemy.pos[0] += dx / len * speed;
            enemy.pos[1] += dy / len * speed;
        } else if d > ENEMY_CHASE_RANGE {
            // Return to spawn
            let sx = enemy.spawn[0] - enemy.pos[0];
            let sy = enemy.spawn[1] - enemy.pos[1];
            let sl = (sx * sx + sy * sy).sqrt();
            if sl > 0.1 {
                enemy.pos[0] += sx / sl * ENEMY_SPEED * 0.5;
                enemy.pos[1] += sy / sl * ENEMY_SPEED * 0.5;
            }
        }
    }
}

fn update_projectiles(zone: &mut ZoneState, player: &mut PlayerState, events: &mut Vec<SimEvent>) {
    let mut hits: Vec<(usize, f32, [f32; 2])> = Vec::new();
    let mut player_dmg: f32 = 0.0;

    for proj in &mut zone.projectiles {
        let speed = if proj.is_frost {
            FROST_BOLT_SPEED
        } else if proj.friendly {
            FIREBALL_SPEED
        } else {
            ENEMY_PROJ_SPEED
        };
        proj.pos[0] += proj.dir[0] * speed;
        proj.pos[1] += proj.dir[1] * speed;
        proj.dist_left -= speed;

        if proj.friendly {
            for (i, enemy) in zone.enemies.iter().enumerate() {
                if !enemy.alive {
                    continue;
                }
                if dist(proj.pos, enemy.pos) < 0.8 {
                    hits.push((i, proj.damage, proj.pos));
                    proj.dist_left = 0.0;
                    break;
                }
            }
        } else if dist(proj.pos, player.pos) < 0.8 {
            player_dmg += proj.damage;
            proj.dist_left = 0.0;
        }
    }

    // Remove expired
    zone.projectiles.retain(|p| p.dist_left > 0.0);

    // Apply enemy hits
    for (i, dmg, pos) in hits {
        if let Some(enemy) = zone.enemies.get_mut(i) {
            if enemy.alive {
                let actual = dmg * player.spell_bonus();
                enemy.hp -= actual;
                events.push(SimEvent::DamageFloat {
                    wx: pos[0],
                    wy: pos[1],
                    value: actual,
                    is_heal: false,
                });
                if enemy.hp <= 0.0 {
                    enemy.alive = false;
                    enemy.fade_timer = 30;
                    enemy.respawn_timer = if enemy.is_boss() { 1800 } else { 600 };
                    let xp = enemy.xp_value();
                    events.push(SimEvent::EnemyKilled {
                        index: i,
                        xp,
                        pos: enemy.pos,
                    });
                }
            }
        }
    }

    // Apply player hit
    if player_dmg > 0.0 {
        let reduced = (player_dmg - player.damage_reduction()).max(1.0);
        player.hp -= reduced;
        events.push(SimEvent::PlayerHit { damage: reduced });
        if player.hp <= 0.0 {
            player.hp = 0.0;
            player.dead = true;
            player.respawn_timer = RESPAWN_DELAY;
            player.xp = (player.xp - player.xp * 0.10).max(0.0);
        }
    }
}

fn update_enemy_respawn(zone: &mut ZoneState) {
    for enemy in &mut zone.enemies {
        if enemy.alive {
            if enemy.fade_timer > 0 {
                enemy.fade_timer -= 1;
            }
            continue;
        }
        if enemy.fade_timer > 0 {
            enemy.fade_timer -= 1;
            continue;
        }
        if enemy.respawn_timer > 0 {
            enemy.respawn_timer -= 1;
            if enemy.respawn_timer == 0 {
                enemy.alive = true;
                enemy.hp = enemy.max_hp;
                enemy.pos = enemy.spawn;
                enemy.attack_cd = 0;
            }
        }
    }
}

fn update_mercenary(zone: &mut ZoneState, player: &PlayerState, events: &mut Vec<SimEvent>) {
    // Placeholder — mercenary logic will be migrated incrementally.
    // The full merc AI requires careful borrow management that is easier
    // to handle once the full update loop is migrated.
    let _ = (zone, player, events);
}
