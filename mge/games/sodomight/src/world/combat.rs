// @id: Sodomight-World-Combat @do: combat-systems @role: back-end @layer: 4 @human: miyuk
//! Combat logic: player attacks, monster attacks, damage application,
//! combat logging, and monster death handling.

use super::SodomightWorld;
use super::types::{
    BASE_XP_PER_KILL, MonsterRecord, PLAYER_MELEE_RANGE, PlayerRecord, WorldError,
};
use mge_arpg_combat::{
    AttackerStats, CombatEvent, CombatProcessor, DamageType, DefenderStats,
};
use mge_arpg_loot::LootGenerator;
use mge_ecs::EntityId;

impl SodomightWorld {
    /// A monster attacks the player.
    pub(super) fn monster_attack_player(&mut self, monster_id: EntityId) {
        let monster = match self.ecs.get_component::<MonsterRecord>(monster_id) {
            Ok(r) => r.clone(),
            Err(_) => return,
        };

        let monster_level = i32::from(monster.level.get());
        let attacker = AttackerStats {
            min_damage: 1 + (monster_level / 2),
            max_damage: 3 + monster_level,
            attack_rating: monster_level * 5,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: monster.level.get(),
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };

        let defender = DefenderStats {
            defense_rating: self.player_stats.derived.defense_rating,
            fire_res: self.player_stats.derived.fire_res,
            cold_res: self.player_stats.derived.cold_res,
            light_res: self.player_stats.derived.light_res,
            poison_res: self.player_stats.derived.poison_res,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: u8::try_from(self.player_stats.level.level).unwrap_or(1),
            absorb: 0,
            current_hp: self.player_stats.current_life,
        };

        let result = CombatProcessor::process_attack(
            monster_id.index,
            self.player_id.index,
            &attacker,
            &defender,
            self.player_stats.current_life,
            &mut self.rng,
        );

        if result.total_damage > 0 {
            self.player_stats.take_damage(result.total_damage);

            // Sync player ECS component.
            let new_hp = self.player_stats.current_life;
            let max_hp = self.player_stats.derived.max_life;
            let _ = self
                .ecs
                .modify_component::<PlayerRecord>(self.player_id, |pr| {
                    pr.health.current = new_hp.max(0) as u32;
                    pr.health.max = max_hp.max(0) as u32;
                });
        }

        // Log events.
        for event in &result.events {
            match event {
                CombatEvent::Hit { damage, .. } => {
                    let msg = format!(
                        "{} hits you for {} damage{}",
                        monster.name,
                        damage.final_amount,
                        if damage.is_critical { " (CRITICAL)" } else { "" }
                    );
                    self.combat_log.push(msg);
                }
                CombatEvent::Miss { .. } => {
                    let msg = format!("{} misses you", monster.name);
                    self.combat_log.push(msg);
                }
                CombatEvent::Death { .. } => {
                    self.combat_log
                        .push("You have been slain!".to_string());
                }
                CombatEvent::Immune { .. } => {}
            }
        }
    }

    /// Apply raw damage to any entity (monster or player).
    pub(super) fn apply_damage_to_entity(&mut self, entity_id: EntityId, damage: i32) {
        if entity_id == self.player_id {
            self.player_stats.take_damage(damage);
            let new_hp = self.player_stats.current_life;
            let max_hp = self.player_stats.derived.max_life;
            let _ = self
                .ecs
                .modify_component::<PlayerRecord>(self.player_id, |pr| {
                    pr.health.current = new_hp.max(0) as u32;
                    pr.health.max = max_hp.max(0) as u32;
                });
        } else {
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(entity_id, |mr| {
                    mr.health.take_damage(damage.max(0) as u32);
                });
        }
    }

    // -------------------------------------------------------------------
    // Player combat
    // -------------------------------------------------------------------

    /// The player attacks a target entity.
    ///
    /// Returns combat log messages for this attack. If the monster dies,
    /// loot is generated and XP is awarded.
    pub fn player_attack(&mut self, target_id: EntityId) -> Result<Vec<String>, WorldError> {
        let monster = self
            .ecs
            .get_component::<MonsterRecord>(target_id)
            .map_err(|_| WorldError::EntityNotFound)?
            .clone();

        if !monster.health.is_alive() {
            return Err(WorldError::EntityNotFound);
        }

        // Enforce melee range limit.
        let (px, py) = self.player_position();
        let dx = px - monster.position.x();
        let dy = py - monster.position.y();
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > PLAYER_MELEE_RANGE {
            return Err(WorldError::TooFar {
                distance: dist,
                max: PLAYER_MELEE_RANGE,
            });
        }

        let attacker = AttackerStats {
            min_damage: self.player_stats.derived.min_damage,
            max_damage: self.player_stats.derived.max_damage,
            attack_rating: self.player_stats.derived.attack_rating,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: u8::try_from(self.player_stats.level.level).unwrap_or(1),
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };

        let defender = DefenderStats {
            defense_rating: 0,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: monster.level.get(),
            absorb: 0,
            current_hp: i32::try_from(monster.health.current).unwrap_or(i32::MAX),
        };

        let result = CombatProcessor::process_attack(
            self.player_id.index,
            target_id.index,
            &attacker,
            &defender,
            i32::try_from(monster.health.current).unwrap_or(i32::MAX),
            &mut self.rng,
        );

        let mut messages = Vec::new();

        // Apply damage to monster.
        if result.total_damage > 0 {
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(target_id, |mr| {
                    mr.health.take_damage(result.total_damage as u32);
                });
        }

        // Log combat events and detect death.
        let monster_died = Self::log_player_attack_events(
            &result.events,
            &monster.name,
            &mut messages,
            &mut self.combat_log,
        );

        // Handle monster death.
        if monster_died {
            self.handle_monster_death(target_id, &monster, &mut messages);
        }

        Ok(messages)
    }

    /// Log combat events from a player attack. Returns `true` if the monster died.
    fn log_player_attack_events(
        events: &[CombatEvent],
        monster_name: &str,
        messages: &mut Vec<String>,
        combat_log: &mut Vec<String>,
    ) -> bool {
        let mut died = false;
        for event in events {
            match event {
                CombatEvent::Hit { damage, .. } => {
                    let crit = if damage.is_critical { " (CRITICAL)" } else { "" };
                    let msg = format!(
                        "You hit {monster_name} for {} damage{crit}",
                        damage.final_amount
                    );
                    messages.push(msg.clone());
                    combat_log.push(msg);
                }
                CombatEvent::Miss { .. } => {
                    let msg = format!("You miss {monster_name}");
                    messages.push(msg.clone());
                    combat_log.push(msg);
                }
                CombatEvent::Death { .. } => {
                    died = true;
                }
                CombatEvent::Immune { dtype, .. } => {
                    let msg = format!("{monster_name} is immune to {dtype:?}");
                    messages.push(msg.clone());
                    combat_log.push(msg);
                }
            }
        }
        died
    }

    /// Handle all side-effects of a monster dying: XP, loot, quest tracking, AI cleanup.
    pub(super) fn handle_monster_death(
        &mut self,
        target_id: EntityId,
        monster: &MonsterRecord,
        messages: &mut Vec<String>,
    ) {
        let name = &monster.name;
        let death_msg = format!("{name} has been slain!");
        messages.push(death_msg.clone());
        self.combat_log.push(death_msg);

        // Award XP -- prefer per-monster override from MonsterDef, fall back
        // to the legacy formula (BASE_XP_PER_KILL * level).
        let xp_reward = self
            .monster_xp_rewards
            .remove(&target_id)
            .map_or_else(
                || {
                    let monster_level = u64::from(monster.level.get());
                    BASE_XP_PER_KILL * monster_level
                },
                |xp| u64::try_from(xp.max(0)).unwrap_or(0),
            );
        let mut xp_messages = self.player_gain_xp(xp_reward);
        messages.append(&mut xp_messages);

        // Generate loot -- prefer per-monster TC from MonsterDef, fall back
        // to "tc_default".
        let tc_id = self
            .monster_tc_ids
            .remove(&target_id)
            .unwrap_or_else(|| "tc_default".to_string());
        let mlvl = u32::from(monster.level.get());
        let drops = LootGenerator::generate(
            &tc_id,
            &self.tc_registry,
            mlvl,
            0,
            &mut self.rng,
        );

        // Always drop some gold.
        let gold = LootGenerator::gold_drop(mlvl, mlvl * 10, &mut self.rng);

        let mut all_drops = drops;
        all_drops.push(gold);

        let drop_count = all_drops.len();
        let drop_msg = format!("{name} dropped {drop_count} items");
        messages.push(drop_msg.clone());
        self.combat_log.push(drop_msg);

        self.pending_loot
            .push((monster.position.x(), monster.position.y(), all_drops));

        // Notify quest journal.
        self.player_quest_journal.register_kill(name, 1);

        // Remove AI agent.
        self.ai_agents.remove(&target_id);
        self.status_effects.remove(&target_id);
    }
}
