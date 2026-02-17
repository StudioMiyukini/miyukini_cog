//! États et logique d'animation du personnage (Lancer).

use bevy::prelude::*;

use crate::player::{LancerHandles, PlayerAnimation, PlayerDirection, WALK_FRAME_DURATION};

/// État d'animation du personnage.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walk,
    Attack,
    Hurt,
    Death,
}

/// Indice de direction (0..8) : N, NE, E, SE, S, SW, W, NW.
/// Les sprites ont 8 colonnes (une par direction).
pub type DirectionIndex = u8;

/// Nombre de directions.
pub const DIRECTION_COUNT: u8 = 8;

/// Pour les directions "gauche" (W, SW, NW), on affiche la direction miroir (E, SE, NE) avec flip_x.
pub fn direction_to_atlas_and_flip(direction_index: DirectionIndex) -> (u32, bool) {
    match direction_index {
        0 => (0, false),   // N
        1 => (1, false),   // NE
        2 => (2, false),   // E
        3 => (3, false),   // SE
        4 => (4, false),   // S
        5 => (1, true),    // SW -> NE flip
        6 => (2, true),    // W  -> E  flip
        7 => (3, true),    // NW -> SE flip
        _ => (0, false),
    }
}

/// Met à jour le sprite du joueur selon l'état d'animation et la direction (image, index atlas, flip_x).
pub fn animate_player(
    time: Res<Time>,
    handles: Option<Res<LancerHandles>>,
    mut query: Query<(&PlayerDirection, &mut PlayerAnimation, &mut Sprite), With<crate::player::Player>>,
) {
    let Some(handles) = handles else { return };

    for (direction, mut anim, mut sprite) in query.iter_mut() {
        anim.timer += time.delta_secs();

        // Cycle frame pour Walk
        if anim.state == AnimationState::Walk && anim.timer >= WALK_FRAME_DURATION {
            anim.timer = 0.0;
            anim.frame = if anim.frame == 0 { 1 } else { 0 };
        }

        let (atlas_index, flip_x) = direction_to_atlas_and_flip(direction.index);

        let image = match anim.state {
            AnimationState::Idle => handles.idle.clone(),
            AnimationState::Walk => {
                if anim.frame == 0 {
                    handles.walk1.clone()
                } else {
                    handles.walk2.clone()
                }
            }
            AnimationState::Attack => {
                match anim.frame % 3 {
                    0 => handles.attack1.clone(),
                    1 => handles.attack2.clone(),
                    _ => handles.attack3.clone(),
                }
            }
            AnimationState::Hurt => handles.hurt.clone(),
            AnimationState::Death => handles.death.clone(),
        };

        sprite.image = image;
        sprite.flip_x = flip_x;
        if let Some(ref mut atlas) = sprite.texture_atlas {
            atlas.layout = handles.layout.clone();
            atlas.index = atlas_index as usize;
        }
    }
}
