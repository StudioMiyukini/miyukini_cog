// ---------------------------------------------------------------------------
// Gameplay constants (shared between server and client)
// ---------------------------------------------------------------------------

// Map — 1 tile = 1 meter, chunks = 100×100m
pub const ZONE_COUNT: usize = 25;
pub const MAP_W: usize = 100;
pub const MAP_H: usize = 100;

// Global world map — 5×5 chunks stitched into one continuous terrain
pub const WORLD_CHUNKS_X: usize = 5;
pub const WORLD_CHUNKS_Y: usize = 5;
pub const WORLD_W: usize = MAP_W * WORLD_CHUNKS_X; // 500
pub const WORLD_H: usize = MAP_H * WORLD_CHUNKS_Y; // 500
/// Offset to convert chunk coords (-2..2) to positive global indices.
pub const CHUNK_OFFSET: i32 = 2;

// Movement (unchanged — same walk speed in m/s, world is just bigger)
pub const PLAYER_SPEED: f32 = 0.06;
pub const PLAYER_RUN_SPEED: f32 = 0.10;
pub const CLICK_MOVE_SPEED: f32 = 0.05;
pub const CLICK_RUN_SPEED: f32 = 0.085;

// Combat — slower early game, more deliberate
pub const ATTACK_RANGE: f32 = 0.9;
pub const ATTACK_COOLDOWN_BASE: u32 = 55;  // Player attacks slower (was 45)
pub const ATTACK_DAMAGE: f32 = 3.5;        // Slightly lower starting melee
pub const PLAYER_MAX_HP: f32 = 65.0;       // Reduced from 80 — less tanky
pub const PLAYER_MAX_MP: f32 = 22.0;       // Moderate mana pool
pub const HP_REGEN_BASE: f32 = 0.012;      // Slower regen (~0.7 HP/sec)
pub const MP_REGEN_BASE: f32 = 0.010;      // Moderate mana regen
pub const XP_PER_KILL: f32 = 25.0;
pub const XP_PER_LEVEL: f32 = 500.0;     // D2 lv2 = 500 XP (~20 Fallen kills)

// Enemies — deliberate pace, packs are dangerous
pub const ENEMY_SPEED: f32 = 0.022;
pub const ENEMY_CHASE_RANGE: f32 = 5.0;
pub const ENEMY_ATTACK_RANGE: f32 = 0.65;
pub const ENEMY_ATTACK_CD: u32 = 120;    // 2.0s between attacks
pub const ENEMY_DAMAGE: f32 = 4.0;       // Back to D2-like base damage
pub const BOSS_MAX_HP: f32 = 250.0;      // D2 Andariel ~1024, scaled for early game

// Spells — reduced early damage, higher mana cost for deliberate pacing
pub const FIREBALL_SPEED: f32 = 0.10;
pub const FIREBALL_DAMAGE: f32 = 7.0;    // ~2 fireballs to kill a Fallen
pub const FIREBALL_COST: f32 = 5.0;      // Costs more mana — can't spam
pub const FIREBALL_RANGE: f32 = 8.0;
pub const FROST_BOLT_SPEED: f32 = 0.09;
pub const FROST_BOLT_DAMAGE: f32 = 4.0;  // Lower damage, but slows
pub const FROST_BOLT_COST: f32 = 4.0;    // More expensive
pub const FROST_BOLT_RANGE: f32 = 10.0;
pub const CHAIN_LIGHTNING_DAMAGE: f32 = 12.0; // Reduced AoE damage
pub const CHAIN_LIGHTNING_COST: f32 = 14.0;   // Very expensive — 64% of mana pool
pub const CHAIN_LIGHTNING_BOUNCES: u32 = 4;
pub const CHAIN_LIGHTNING_RANGE: f32 = 6.0;

// Enemy ranged
pub const ENEMY_RANGED_RANGE: f32 = 6.0;
pub const ENEMY_PROJ_SPEED: f32 = 0.10;

// Mercenary — D2 Act 1 Rogue hireling
pub const MERC_SPEED: f32 = 0.055;
pub const MERC_ATTACK_RANGE: f32 = 1.5;
pub const MERC_ATTACK_CD: u32 = 40;
pub const MERC_MAX_HP: f32 = 85.0;       // D2 Act 1 merc ~80-100 HP
pub const MERC_DAMAGE: f32 = 4.0;        // D2 Act 1 merc ~3-5 dmg
pub const MERC_HIRE_COST: u32 = 80;

// NPCs
pub const NPC_INTERACT_RANGE: f32 = 2.5;
pub const GAMBLE_COST: u32 = 30;
pub const UPGRADE_COST: u32 = 60;
pub const SCROLL_COST: u32 = 20;

// Items & Potions
pub const POTION_HP_RESTORE: f32 = 30.0; // Restores ~46% HP at lv1
pub const POTION_MP_RESTORE: f32 = 18.0; // Restores ~82% MP at lv1

// Misc
pub const ARRIVE_THRESHOLD: f32 = 0.15;
pub const RESPAWN_DELAY: u32 = 750;      // ×5 respawn timer (~12.5s at 60fps)
pub const POISON_TICK_DMG: f32 = 0.04;   // ~2.4 DPS (was 18 DPS — way too high)
pub const SHRINE_DURATION: u32 = 600;

// Frost effects
pub const FREEZE_DURATION: u32 = 60;
pub const SLOW_DURATION: u32 = 180;
pub const FROST_SLOW_FACTOR: f32 = 0.4;

// Teleport
pub const TELEPORT_COST: f32 = 24.0;     // D2 lv1 Teleport mana cost
pub const TELEPORT_RANGE: f32 = 8.0;

// Knockback
pub const KNOCKBACK_FORCE: f32 = 1.2;
pub const KNOCKBACK_CHANCE: f32 = 0.25;

// Skill system
pub const SKILL_COUNT: usize = 5;
pub const EQUIP_SLOT_COUNT: usize = 6;

// Summons
pub const SUMMON_MAX: usize = 8;
pub const SKELETON_HP: f32 = 30.0;
pub const SKELETON_DAMAGE: f32 = 3.0;
pub const SKELETON_SPEED: f32 = 0.04;
pub const SKELETON_ATTACK_RANGE: f32 = 0.8;
pub const SKELETON_ATTACK_CD: u32 = 55;
pub const GOLEM_HP: f32 = 120.0;
pub const GOLEM_DAMAGE: f32 = 8.0;
pub const GOLEM_SPEED: f32 = 0.03;
pub const GOLEM_ATTACK_RANGE: f32 = 1.0;
pub const GOLEM_ATTACK_CD: u32 = 70;
pub const WOLF_HP: f32 = 45.0;
pub const WOLF_DAMAGE: f32 = 5.0;
pub const WOLF_SPEED: f32 = 0.07;
pub const WOLF_ATTACK_RANGE: f32 = 0.7;
pub const WOLF_ATTACK_CD: u32 = 35;

// Mercenary types
pub const MERC_ROGUE_RANGE: f32 = 6.0;      // Act 1 Rogue — ranged
pub const MERC_GUARD_RANGE: f32 = 1.2;       // Act 2 Guard — melee + aura
pub const MERC_WOLF_RIDER_RANGE: f32 = 1.5;  // Act 3 Wolf Rider — fast melee
pub const MERC_MAGE_RANGE: f32 = 7.0;        // Act 4 Iron Wolf — caster
pub const MERC_GUARD_HP: f32 = 130.0;
pub const MERC_GUARD_DAMAGE: f32 = 6.0;
pub const MERC_WOLF_RIDER_HP: f32 = 70.0;
pub const MERC_WOLF_RIDER_DAMAGE: f32 = 7.0;
pub const MERC_WOLF_RIDER_SPEED: f32 = 0.08;
pub const MERC_MAGE_HP: f32 = 55.0;
pub const MERC_MAGE_DAMAGE: f32 = 12.0;

// Auras (merc guard / keystone)
pub const AURA_RADIUS: f32 = 6.0;
pub const AURA_MIGHT_DMG_PCT: f32 = 0.20;      // +20% damage
pub const AURA_DEFIANCE_DEF_FLAT: f32 = 5.0;   // +5 flat defense
pub const AURA_MEDITATION_MP_REGEN: f32 = 0.02; // +0.02 mp/frame
pub const AURA_THORNS_REFLECT_PCT: f32 = 0.15;  // 15% damage reflected

// Combat extras
pub const CRIT_BASE_CHANCE: f32 = 0.05;     // 5% base crit
pub const CRIT_BASE_MULT: f32 = 2.0;        // 2× damage on crit
pub const COMBO_WINDOW: u32 = 90;            // frames to chain combo hits
pub const COMBO_MAX_STACK: u32 = 5;
pub const COMBO_DMG_PER_STACK: f32 = 0.08;  // +8% damage per combo stack

// MMO — party
pub const PARTY_MAX_SIZE: usize = 6;
pub const PARTY_XP_SHARE_RADIUS: f32 = 15.0;
pub const PARTY_XP_BONUS_PER_MEMBER: f32 = 0.10; // +10% XP per extra member

// World boss
pub const WORLD_BOSS_HP: f32 = 5000.0;
pub const WORLD_BOSS_DAMAGE: f32 = 35.0;
pub const WORLD_BOSS_SPAWN_INTERVAL: u32 = 18000; // 5 minutes at 60fps

// Combat log
pub const LOG_MAX: usize = 8;
pub const LOG_LIFE: u32 = 180;
