//! @id allumina.prototype.animation
//! @role system
//! @layer application
//! @domain allumina
//! @do sprite_animation_state_machine
//!
//! Système d'animation sprites — Phase 110.
//! Sélectionne le frame courant selon l'état du joueur : Idle / Walk / Dead.
//!
//! Atlas layout (doit correspondre à build_char_atlas dans renderer.rs) :
//!   0..9   → Knight Idle  (10 frames × 29→64×86 px)
//!   10..17 → Knight Walk  (8 frames × 64×64 px, knight_walk.png)
//!   18..27 → Knight Dead  (10 frames × 48→64×86 px)
//!   28..32 → Sprites mobs statiques (5 types)

use mge_ecs::Component;

use crate::components::{AlluminaInput, EntitySprite, PlayerMarker};
use crate::pathfinding::PathFollow;
use crate::stats::Dead;

// ── Constantes d'atlas ────────────────────────────────────────────────────────

/// Base frame animation Idle chevalier (10 frames, 29→64×86)
pub const KNIGHT_IDLE_BASE: u8 = 0;
pub const KNIGHT_IDLE_FRAMES: u8 = 10;

/// Base frame animation Walk chevalier (8 frames, 64×64, knight_walk.png)
pub const KNIGHT_WALK_BASE: u8 = 10;
pub const KNIGHT_WALK_FRAMES: u8 = 8;

/// Base frame animation Dead chevalier (10 frames, 48→64×86)
pub const KNIGHT_DEAD_BASE: u8 = 18;
pub const KNIGHT_DEAD_FRAMES: u8 = 10;

/// Base des sprites mobs statiques (5 types, indices 28–32)
pub const MOB_SPRITE_BASE: u8 = 28;

// ── Composant ─────────────────────────────────────────────────────────────────

/// État d'animation d'une entité sprites-chevalier.
pub struct AnimSprite {
    /// Frame courant dans l'animation active (0..frame_count)
    pub local_frame: u8,
    /// Base de l'animation actuellement jouée — permet de détecter les transitions
    pub current_base: u8,
    /// Accumulateur temps entre frames (secondes)
    pub timer: f32,
    /// Vitesse en frames/seconde
    pub fps: f32,
}

impl Component for AnimSprite {}

impl AnimSprite {
    pub fn new(fps: f32) -> Self {
        Self {
            local_frame: 0,
            current_base: KNIGHT_IDLE_BASE,
            timer: 0.0,
            fps,
        }
    }
}

// ── Système ───────────────────────────────────────────────────────────────────

/// Phase 110 — après Movement (Phase 100).
/// Détermine l'animation cible (Idle/Walk/Dead), réinitialise si changement,
/// avance le frame et met à jour EntitySprite.sprite_id.
pub fn animation_system(world: &mut mge_ecs::World, ctx: &mut mge_core::Context) {
    let dt = ctx.delta_secs();

    // Direction clavier globale (singleton AlluminaInput)
    let kbd_dir = world
        .iter1::<AlluminaInput>()
        .next()
        .map(|(_, inp)| inp.move_dir)
        .unwrap_or((0.0, 0.0));
    let kbd_moving = kbd_dir.0 * kbd_dir.0 + kbd_dir.1 * kbd_dir.1 > 0.01;

    // Collecter les entités animées (shared borrow → release)
    let ids: Vec<mge_ecs::EntityId> = world
        .iter2::<AnimSprite, EntitySprite>()
        .map(|(id, _, _)| id)
        .collect();

    for id in ids {
        let is_dead   = world.has_component::<Dead>(id);
        let is_player = world.has_component::<PlayerMarker>(id);

        // Chemin A* actif sur cette entité ?
        let path_active = world
            .get::<PathFollow>(id)
            .map(|pf| !pf.is_done())
            .unwrap_or(false);

        // Le joueur bouge si clavier OU A* actif
        let is_moving = path_active || (is_player && kbd_moving);

        // Choisir l'animation cible
        let (base, count, looping): (u8, u8, bool) = if is_dead {
            (KNIGHT_DEAD_BASE, KNIGHT_DEAD_FRAMES, false)
        } else if is_moving {
            (KNIGHT_WALK_BASE, KNIGHT_WALK_FRAMES, true)
        } else {
            (KNIGHT_IDLE_BASE, KNIGHT_IDLE_FRAMES, true)
        };

        // Avancer timer + frame (mutation AnimSprite, borrow release ensuite)
        let new_local: u8;
        if let Some(anim) = world.get_mut::<AnimSprite>(id) {
            // Transition vers nouvelle animation : réinitialiser
            if anim.current_base != base {
                anim.current_base = base;
                anim.local_frame = 0;
                anim.timer = 0.0;
            }
            // Avancer sauf si animation non-loop arrivée à la dernière frame
            let frozen = !looping && anim.local_frame >= count.saturating_sub(1);
            if !frozen {
                anim.timer += dt;
                let frame_dur = 1.0 / anim.fps.max(1.0);
                if anim.timer >= frame_dur {
                    anim.timer -= frame_dur;
                    let next = anim.local_frame + 1;
                    anim.local_frame = if next >= count {
                        if looping { 0 } else { count - 1 }
                    } else {
                        next
                    };
                }
            }
            new_local = anim.local_frame;
        } else {
            continue;
        } // borrow AnimSprite relâché

        // Mettre à jour EntitySprite.sprite_id (borrow séparé)
        if let Some(sprite) = world.get_mut::<EntitySprite>(id) {
            sprite.sprite_id = base + new_local;
        }
    }
}
