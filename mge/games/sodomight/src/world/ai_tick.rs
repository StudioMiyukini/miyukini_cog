// @id: Sodomight-World-AiTick @do: ai-tick-systems @role: back-end @layer: 4 @human: miyuk
//! Game tick, AI evaluation, status effect processing, and ECS combat tick.

use super::SodomightWorld;
use super::types::{
    ATTACK_COOLDOWN_TICKS, DEFAULT_ATTACK_RANGE, DEFAULT_SIGHT_RANGE,
    LIFE_REGEN_AMOUNT, LIFE_REGEN_INTERVAL, MANA_REGEN_AMOUNT, MANA_REGEN_INTERVAL,
    MONSTER_MOVE_SPEED, MonsterRecord, TICK_DELTA_MS,
};
use mge_arpg_combat::StatusType;
use mge_arpg_entity::Position;
use mge_ecs::EntityId;

impl SodomightWorld {
    // -------------------------------------------------------------------
    // Game tick
    // -------------------------------------------------------------------

    /// Execute one game tick.
    ///
    /// This is the main simulation step, called at the configured tick rate.
    pub fn tick(&mut self) {
        self.game_tick += 1;

        // 1. Tick skill cooldowns.
        self.cooldown_tracker.tick_all(TICK_DELTA_MS);

        // 2. Process status effects.
        self.tick_status_effects();

        // 3. AI decisions and monster attacks.
        self.tick_ai();

        // 4. Life / mana regeneration (D2-style slow passive regen).
        if self.player_stats.is_alive() {
            if self.game_tick.is_multiple_of(LIFE_REGEN_INTERVAL) {
                self.player_stats.restore_life(LIFE_REGEN_AMOUNT);
            }
            if self.game_tick.is_multiple_of(MANA_REGEN_INTERVAL) {
                self.player_stats.restore_mana(MANA_REGEN_AMOUNT);
            }
        }
    }

    /// Process all active status effects (poison, burn, etc.).
    pub(super) fn tick_status_effects(&mut self) {
        let entity_ids: Vec<EntityId> = self.status_effects.keys().copied().collect();

        for entity_id in &entity_ids {
            let mut poison_damage = 0i32;

            if let Some(effects) = self.status_effects.get_mut(entity_id) {
                for effect in effects.iter_mut() {
                    if effect.kind == StatusType::Poison && effect.remaining_ms > 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        let dmg = effect.potency as i32;
                        poison_damage += dmg;
                    }
                    effect.remaining_ms = effect.remaining_ms.saturating_sub(TICK_DELTA_MS);
                }
                effects.retain(|e| e.remaining_ms > 0);
            }

            if poison_damage > 0 {
                self.apply_damage_to_entity(*entity_id, poison_damage);
            }
        }

        self.status_effects.retain(|_, effects| !effects.is_empty());
    }

    // -------------------------------------------------------------------
    // AI tick
    // -------------------------------------------------------------------

    /// Run AI evaluation for all monster agents.
    pub(super) fn tick_ai(&mut self) {
        let player_pos = self.player_position();
        let current_tick = self.game_tick;

        let mut attacks: Vec<EntityId> = Vec::new();
        let mut moves: Vec<(EntityId, f32, f32)> = Vec::new();

        let agent_ids: Vec<EntityId> = self.ai_agents.keys().copied().collect();

        for monster_id in &agent_ids {
            if !self.ecs.is_alive(*monster_id) {
                continue;
            }

            let monster_record = match self.ecs.get_component::<MonsterRecord>(*monster_id) {
                Ok(r) => r.clone(),
                Err(_) => continue,
            };

            if !monster_record.health.is_alive() {
                continue;
            }

            let mx = monster_record.position.x();
            let my = monster_record.position.y();
            let dx = player_pos.0 - mx;
            let dy = player_pos.1 - my;
            let distance = (dx * dx + dy * dy).sqrt();

            let hp_ratio = monster_record.health.ratio();

            if let Some(agent) = self.ai_agents.get_mut(monster_id) {
                agent.update(Some(distance), hp_ratio);

                if agent.fsm.can_move()
                    && agent.fsm.is_hostile()
                    && distance > agent.aggro.attack_range
                    && distance > 0.01
                {
                    let inv = MONSTER_MOVE_SPEED / distance;
                    let new_x = mx + dx * inv;
                    let new_y = my + dy * inv;
                    moves.push((*monster_id, new_x, new_y));
                }

                if agent.fsm.can_attack() && agent.aggro.can_attack(distance) {
                    let last = self.attack_cooldowns.get(monster_id).copied().unwrap_or(0);
                    if current_tick.saturating_sub(last) >= ATTACK_COOLDOWN_TICKS {
                        attacks.push(*monster_id);
                    }
                }
            }
        }

        for (monster_id, new_x, new_y) in moves {
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(monster_id, |mr| {
                    mr.position = Position::new(new_x, new_y);
                });
        }

        for monster_id in &attacks {
            self.attack_cooldowns.insert(*monster_id, current_tick);
        }
        for monster_id in attacks {
            self.monster_attack_player(monster_id);
        }
    }

    // -------------------------------------------------------------------
    // ECS combat tick
    // -------------------------------------------------------------------

    /// Execute the combat sub-tick for one simulation step.
    pub fn combat_tick(&mut self, dt_ms: u32) {
        #[allow(clippy::cast_precision_loss)]
        let dt_secs: f32 = dt_ms as f32 / 1_000.0;

        let player_pos = self.player_position();

        // 1. AI FSM tick
        let agent_ids: Vec<EntityId> = self.ai_agents.keys().copied().collect();

        for monster_id in &agent_ids {
            if !self.ecs.is_alive(*monster_id) {
                continue;
            }

            let mr = match self.ecs.get_component::<MonsterRecord>(*monster_id) {
                Ok(r) => r.clone(),
                Err(_) => continue,
            };

            if !mr.health.is_alive() {
                continue;
            }

            let mx = mr.position.x();
            let my = mr.position.y();
            let dx = player_pos.0 - mx;
            let dy = player_pos.1 - my;
            let dist = (dx * dx + dy * dy).sqrt();

            let current_hp = i32::try_from(mr.health.current).unwrap_or(i32::MAX);
            let max_hp = i32::try_from(mr.health.max).unwrap_or(i32::MAX);

            let (sight, attack) = self
                .ai_agents
                .get(monster_id)
                .map_or((DEFAULT_SIGHT_RANGE, DEFAULT_ATTACK_RANGE), |a| {
                    (a.aggro.sight_range, a.aggro.attack_range)
                });

            if let Some(agent) = self.ai_agents.get_mut(monster_id) {
                agent.tick(current_hp, max_hp, dist, sight, attack, dt_secs);
            }
        }

        // 2. Status tick
        let tracker_ids: Vec<EntityId> = self.status_trackers.keys().copied().collect();

        for entity_id in &tracker_ids {
            if let Some(tracker) = self.status_trackers.get_mut(entity_id) {
                tracker.tick(dt_ms);
            }
        }

        // 3. Dead-entity cleanup
        let all_monster_ids: Vec<EntityId> = self.ai_agents.keys().copied().collect();
        let mut dead: Vec<EntityId> = Vec::new();

        for monster_id in &all_monster_ids {
            if !self.ecs.is_alive(*monster_id) {
                continue;
            }

            if let Ok(mr) = self.ecs.get_component::<MonsterRecord>(*monster_id) {
                if !mr.health.is_alive() {
                    dead.push(*monster_id);
                }
            }
        }

        for monster_id in dead {
            let mr = match self.ecs.get_component::<MonsterRecord>(monster_id) {
                Ok(r) => r.clone(),
                Err(_) => continue,
            };

            let mut death_msgs: Vec<String> = Vec::new();
            self.handle_monster_death(monster_id, &mr, &mut death_msgs);
            drop(death_msgs);

            let _ = self.ecs.despawn(monster_id);

            self.attack_cooldowns.remove(&monster_id);
            self.status_trackers.remove(&monster_id);
        }
    }
}
