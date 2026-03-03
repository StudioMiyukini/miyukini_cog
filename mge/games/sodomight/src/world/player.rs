// @id: Sodomight-World-Player @do: player-systems @role: back-end @layer: 4 @human: miyuk
//! Player-centric systems: XP/levelling, skill usage, movement, respawn,
//! skill registration, and status effect helpers.

use super::SodomightWorld;
use super::types::{MonsterRecord, PlayerRecord, WorldError, xp_death_penalty};
use mge_arpg_combat::StatusEffect;
use mge_arpg_entity::{Level, Position};
use mge_arpg_skills::{SkillDef, SkillId};
use mge_ecs::EntityId;

impl SodomightWorld {
    // -------------------------------------------------------------------
    // Experience and levelling
    // -------------------------------------------------------------------

    /// Award experience points to the player.
    ///
    /// Automatically handles level-ups, stat point grants, and skill
    /// point grants. Returns messages for each level gained.
    pub fn player_gain_xp(&mut self, amount: u64) -> Vec<String> {
        let old_level = self.player_stats.level.level;

        self.player_stats.level.add_experience(amount, &self.exp_table);

        let new_level = self.player_stats.level.level;

        let mut messages = Vec::new();
        let xp_msg = format!("Gained {amount} XP");
        messages.push(xp_msg.clone());
        self.combat_log.push(xp_msg);

        if new_level > old_level {
            for lvl in (old_level + 1)..=new_level {
                let lvl_msg = format!("Level up! Now level {lvl}");
                messages.push(lvl_msg.clone());
                self.combat_log.push(lvl_msg);
            }

            // Recalculate derived stats on level-up.
            self.player_stats.recalculate();

            // Restore pools to full on level-up.
            self.player_stats.current_life = self.player_stats.derived.max_life;
            self.player_stats.current_mana = self.player_stats.derived.max_mana;

            // Sync player ECS component.
            let max_life = self.player_stats.derived.max_life;
            let _ = self
                .ecs
                .modify_component::<PlayerRecord>(self.player_id, |pr| {
                    pr.health.max = max_life.max(0) as u32;
                    pr.health.current = max_life.max(0) as u32;
                    pr.level = Level::new(new_level.min(255) as u8);
                });

            // Grant skill points to skill book.
            let skill_points_gained = new_level - old_level;
            self.player_skills.add_points(skill_points_gained);
        }

        messages
    }

    // -------------------------------------------------------------------
    // Skill usage
    // -------------------------------------------------------------------

    /// The player uses a skill on a target.
    ///
    /// Validates cooldown, mana cost, and skill availability before
    /// applying the effect.
    pub fn player_use_skill(
        &mut self,
        skill_id: &SkillId,
        target: Option<EntityId>,
    ) -> Result<Vec<String>, WorldError> {
        // Check skill exists and is learned.
        let skill_level = self.player_skills.level_of(skill_id);
        if skill_level == 0 {
            return Err(WorldError::SkillError(format!(
                "Skill '{skill_id}' not learned"
            )));
        }

        let def = self
            .skill_registry
            .get(skill_id)
            .ok_or_else(|| {
                WorldError::SkillError(format!("Skill '{skill_id}' not in registry"))
            })?
            .clone();

        // Check cooldown.
        if !self.cooldown_tracker.is_ready(skill_id) {
            return Err(WorldError::OnCooldown(skill_id.to_string()));
        }

        // Check mana cost.
        #[allow(clippy::cast_possible_truncation)]
        let mana_cost = def.mana_cost(skill_level) as i32;
        if self.player_stats.current_mana < mana_cost {
            return Err(WorldError::NotEnoughMana {
                need: mana_cost,
                have: self.player_stats.current_mana,
            });
        }

        // Deduct mana.
        self.player_stats.current_mana -= mana_cost;

        // Start cooldown.
        if def.cooldown_ms > 0 {
            self.cooldown_tracker
                .register(skill_id.clone(), def.cooldown_ms);
            self.cooldown_tracker.trigger(skill_id);
        }

        let mut messages = Vec::new();
        let use_msg = format!("You use {}", def.name);
        messages.push(use_msg.clone());
        self.combat_log.push(use_msg);

        // Apply skill effect to target (simplified: treat as bonus damage attack).
        if let Some(target_id) = target {
            let skill_lvl_i32 = i32::try_from(skill_level).unwrap_or(i32::MAX);
            let bonus_damage = skill_lvl_i32 * 5 + mana_cost;
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(target_id, |mr| {
                    mr.health.take_damage(bonus_damage.max(0) as u32);
                });
            let dmg_msg = format!(
                "{} deals {} damage to target",
                def.name, bonus_damage
            );
            messages.push(dmg_msg.clone());
            self.combat_log.push(dmg_msg);

            // Check if monster died -- delegate to the shared death handler.
            if let Ok(mr) = self.ecs.get_component::<MonsterRecord>(target_id) {
                if !mr.health.is_alive() {
                    let mr_clone = mr.clone();
                    self.handle_monster_death(target_id, &mr_clone, &mut messages);
                }
            }
        }

        Ok(messages)
    }

    // -------------------------------------------------------------------
    // Movement
    // -------------------------------------------------------------------

    /// Move the player to a new position.
    pub fn set_player_position(&mut self, x: f32, y: f32) {
        let _ = self
            .ecs
            .modify_component::<PlayerRecord>(self.player_id, |pr| {
                pr.position = Position::new(x, y);
            });
    }

    // -------------------------------------------------------------------
    // Death and respawn
    // -------------------------------------------------------------------

    /// Respawn the player after death (D2-style).
    ///
    /// Steps performed:
    /// 1. Apply XP penalty according to [`self.difficulty`].
    /// 2. Restore HP and mana to their maximum values.
    /// 3. Teleport the player to the town spawn point `(0.0, 0.0)`.
    /// 4. Sync the ECS [`PlayerRecord`] to reflect the new position and HP.
    ///
    /// The XP penalty is calculated via [`xp_death_penalty`] and is guaranteed
    /// never to reduce the player below their current level's XP floor.
    pub fn respawn_player(&mut self) {
        // 1. Apply XP penalty.
        let new_xp = xp_death_penalty(
            self.difficulty,
            self.player_stats.level.experience,
            self.player_stats.level.level,
            &self.exp_table,
        );
        self.player_stats.level.experience = new_xp;

        // 2. Restore HP and mana to full.
        let max_life = self.player_stats.derived.max_life;
        let max_mana = self.player_stats.derived.max_mana;
        self.player_stats.current_life = max_life;
        self.player_stats.current_mana = max_mana;

        // 3 + 4. Move player to town and sync ECS component.
        let _ = self
            .ecs
            .modify_component::<PlayerRecord>(self.player_id, |pr| {
                pr.position = Position::new(0.0, 0.0);
                pr.health.current = max_life.max(0) as u32;
                pr.health.max = max_life.max(0) as u32;
            });

        self.combat_log
            .push("You have been resurrected in town.".to_string());
    }

    // -------------------------------------------------------------------
    // Registration helpers
    // -------------------------------------------------------------------

    /// Register a skill definition in the skill registry and its cooldown.
    pub fn register_skill(&mut self, def: SkillDef) {
        if def.cooldown_ms > 0 {
            self.cooldown_tracker
                .register(def.id.clone(), def.cooldown_ms);
        }
        self.skill_registry.register(def);
    }

    /// Add a status effect to an entity.
    pub fn add_status_effect(&mut self, entity_id: EntityId, effect: StatusEffect) {
        self.status_effects
            .entry(entity_id)
            .or_default()
            .push(effect);
    }

    /// Apply a status effect to an entity's [`StatusTracker`] (ECS-integrated path).
    ///
    /// This is the preferred mutation point for the `combat_tick` pipeline.
    /// For the legacy `Vec<StatusEffect>` path use [`add_status_effect`].
    pub fn apply_status_tracked(&mut self, entity_id: EntityId, effect: StatusEffect) {
        self.status_trackers
            .entry(entity_id)
            .or_default()
            .apply(effect);
    }
}
