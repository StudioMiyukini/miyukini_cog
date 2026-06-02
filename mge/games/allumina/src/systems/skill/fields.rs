//! Timed ground-effect "fields" spawned by skills: flame walls, blizzards,
//! meteor barrages, lightning shields, and arrow-rain zones. Each advances its
//! own timer(s), strikes the enemies in its footprint, and despawns when spent.
//! Split out of the parent `skill` module to keep that file navigable; the
//! shared targeting/damage helpers live there and are reached via `super`.

use super::*;

/// Whether `p` lies inside the oriented rectangle centred at `center` with long
/// axis `u` (a unit vector), half-length `half_len` and half-width `half_wid`.
fn in_rect(p: Position2D, center: Position2D, u: (f32, f32), half_len: f32, half_wid: f32) -> bool {
    let (dx, dy) = (p.x - center.x, p.y - center.y);
    let along = dx * u.0 + dy * u.1;
    let perp = -dx * u.1 + dy * u.0;
    along.abs() <= half_len && perp.abs() <= half_wid
}

/// Advance lightning shields: every `interval` seconds zap nearby enemies, and
/// expire the buff when its window closes.
pub(super) fn tick_lightning_shields(world: &mut World, dt: f32) {
    for owner in world.lightning_shield.keys().copied().collect::<Vec<_>>() {
        let Some(ls) = world.lightning_shield.get_mut(&owner) else { continue };
        ls.remaining -= dt;
        ls.timer -= dt;
        let expired = ls.remaining <= 0.0;
        let fire = ls.timer <= 0.0;
        let (dmg, radius) = (ls.dmg, ls.radius);
        if fire {
            ls.timer += ls.interval;
        }
        if fire {
            if let Some(center) = world.positions.get(&owner).copied() {
                let foes = enemies_within(world, center, radius);
                for e in &foes {
                    resolve_spell(world, owner, *e, dmg, Element::Lightning);
                }
                if !foes.is_empty() {
                    world.render_events.push(RenderEvent::Skill {
                        caster: owner,
                        from: center,
                        to: center,
                        fx: SkillFx::Lightning,
                    });
                }
            }
        }
        if expired {
            world.lightning_shield.remove(&owner);
        }
    }
}

/// Advance flame walls: burn the enemies inside the rectangle each tick, then
/// despawn the zone when its lifetime ends.
pub(super) fn tick_flame_walls(world: &mut World, dt: f32) {
    for zone in world.flame_walls.keys().copied().collect::<Vec<_>>() {
        let Some(w) = world.flame_walls.get_mut(&zone) else { continue };
        w.remaining -= dt;
        w.tick_timer -= dt;
        let expired = w.remaining <= 0.0;
        let fire = w.tick_timer <= 0.0;
        if fire {
            w.tick_timer += w.tick_interval;
        }
        let (owner, center, dir, half_len, half_wid, dmg) =
            (w.owner, w.center, w.dir, w.half_len, w.half_wid, w.dmg_per_tick);
        if fire {
            let foes = crate::systems::targeting::living_where(world, |p| {
                in_rect(p, center, dir, half_len, half_wid)
            });
            for f in &foes {
                resolve_spell(world, owner, *f, dmg, Element::Fire);
            }
        }
        if expired {
            world.despawn(zone);
        }
    }
}

/// Advance blizzards: continuous frost damage plus a slow that deepens over the
/// field's lifetime, then despawn when it ends.
pub(super) fn tick_blizzards(world: &mut World, dt: f32) {
    for zone in world.blizzards.keys().copied().collect::<Vec<_>>() {
        let Some(b) = world.blizzards.get_mut(&zone) else { continue };
        b.remaining -= dt;
        b.elapsed += dt;
        let expired = b.remaining <= 0.0;
        let (owner, center, radius, dps, elapsed) =
            (b.owner, b.center, b.radius, b.dmg_per_sec, b.elapsed);
        let slow = (0.10 + 0.05 * elapsed).min(0.90);
        let foes = enemies_within(world, center, radius);
        for f in &foes {
            resolve_spell(world, owner, *f, dps * dt, Element::Frost);
            world.slows.insert(*f, Slow { factor: 1.0 - slow, remaining: 0.5 });
        }
        if expired {
            world.despawn(zone);
        }
    }
}

/// Advance meteor barrages: drop one impact at a random point in the circle each
/// interval, then despawn once every impact has landed.
pub(super) fn tick_meteor_fields(world: &mut World, dt: f32) {
    for zone in world.meteor_fields.keys().copied().collect::<Vec<_>>() {
        let Some(m) = world.meteor_fields.get_mut(&zone) else { continue };
        m.impact_timer -= dt;
        let mut fire = false;
        if m.impact_timer <= 0.0 && m.impacts_left > 0 {
            m.impact_timer += m.impact_interval;
            m.impacts_left -= 1;
            fire = true;
        }
        let done = m.impacts_left == 0 && m.impact_timer <= 0.0;
        let (owner, center, radius, dmg) = (m.owner, m.center, m.radius, m.dmg_per_impact);
        if fire {
            // Uniform random point in the disc (sqrt keeps it area-uniform).
            let ang = world.rng.next_f32() * std::f32::consts::TAU;
            let rr = radius * world.rng.next_f32().sqrt();
            let ip = Position2D::new(center.x + rr * ang.cos(), center.y + rr * ang.sin());
            let foes = enemies_within(world, ip, 1.5);
            for f in &foes {
                resolve_spell(world, owner, *f, dmg, Element::Fire);
                // Each meteor impact briefly knocks struck mobs to the ground.
                apply_knockdown(world, *f, 0.5);
            }
            world.render_events.push(RenderEvent::Skill {
                caster: owner,
                from: ip,
                to: ip,
                fx: SkillFx::Meteor,
            });
        }
        if done {
            world.despawn(zone);
        }
    }
}

/// Resolve arrow-rain zones: at every interval, rain arrows on every foe inside
/// the disc; despawn the zone when its window closes.
pub(super) fn tick_arrow_rains(world: &mut World, dt: f32) {
    for zone in world.arrow_rains.keys().copied().collect::<Vec<_>>() {
        let Some(r) = world.arrow_rains.get_mut(&zone) else { continue };
        r.remaining -= dt;
        r.tick_timer -= dt;
        let expired = r.remaining <= 0.0;
        let fire = r.tick_timer <= 0.0;
        if fire {
            r.tick_timer += r.tick_interval;
        }
        let (owner, center, radius, dmg) = (r.owner, r.center, r.radius, r.dmg_per_tick);
        if fire {
            let foes = enemies_within(world, center, radius);
            for f in &foes {
                apply_flat_damage(world, owner, *f, dmg);
            }
            if !foes.is_empty() {
                world.render_events.push(RenderEvent::Skill {
                    caster: owner,
                    from: center,
                    to: center,
                    fx: SkillFx::ArrowRain,
                });
            }
        }
        if expired {
            world.despawn(zone);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::in_rect;
    use crate::components::Position2D;

    #[test]
    fn in_rect_respects_orientation() {
        let c = Position2D::new(0.0, 0.0);
        // Long axis +x: a point 2 tiles ahead is inside, one 2 tiles to the side is not.
        assert!(in_rect(Position2D::new(2.0, 0.0), c, (1.0, 0.0), 3.0, 1.0));
        assert!(!in_rect(Position2D::new(2.0, 2.0), c, (1.0, 0.0), 3.0, 1.0));
        // Rotate the wall to face +y and the side point now falls within it.
        assert!(in_rect(Position2D::new(0.0, 2.0), c, (0.0, 1.0), 3.0, 1.0));
    }
}
