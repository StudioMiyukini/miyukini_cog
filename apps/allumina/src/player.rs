//! Joueur : sprite Lancer (Tiny RPG Characters), 8 directions, flip gauche/droite.

use bevy::prelude::*;

use crate::animation::{direction_to_atlas_and_flip, AnimationState, DIRECTION_COUNT};
use crate::{GAME_HEIGHT, GAME_WIDTH};

/// Chemin des assets Lancer (sous assets/ au cwd). 100×100 par cellule, 8 colonnes.
const LANCER_ASSET_DIR: &str = "Tiny_RPG_Characters/Characters(100x100)/Lancer/Lancer with shadows";

/// Composant marqueur du joueur.
#[derive(Component)]
pub struct Player;

/// Direction de déplacement et indice pour les sprites (0..8).
#[derive(Component)]
pub struct PlayerDirection {
    pub index: DirectionIndex,
    /// Angle en radians pour le calcul (0 = droite, PI/2 = haut).
    pub angle_rad: f32,
}

/// Animation en cours et frame courante (pour Walk: 0 ou 1).
#[derive(Component)]
pub struct PlayerAnimation {
    pub state: AnimationState,
    pub frame: u32,
    pub timer: f32,
}

/// Vitesse de déplacement en unités par seconde.
const PLAYER_SPEED: f32 = 200.0;
/// Période de changement de frame pour la marche (secondes).
pub const WALK_FRAME_DURATION: f32 = 0.15;

/// Handles des textures Lancer par type d'animation (chargés au startup).
#[derive(Resource)]
pub struct LancerHandles {
    pub idle: Handle<Image>,
    pub walk1: Handle<Image>,
    pub walk2: Handle<Image>,
    pub attack1: Handle<Image>,
    pub attack2: Handle<Image>,
    pub attack3: Handle<Image>,
    pub hurt: Handle<Image>,
    pub death: Handle<Image>,
    /// Layout commun : grille 8×1, cellules 100×100.
    pub layout: Handle<TextureAtlasLayout>,
}

/// Spawn le personnage Lancer au centre, scale 2× pour assets tiny.
pub fn spawn_lancer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    tiny_scale: f32,
) {
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(100, 100),
        8,
        1,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    let idle_handle = asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Idle.png"));

    let handles = LancerHandles {
        idle: idle_handle.clone(),
        walk1: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Walk01.png")),
        walk2: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Walk02.png")),
        attack1: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Attack01.png")),
        attack2: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Attack02.png")),
        attack3: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Attack03.png")),
        hurt: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Hurt.png")),
        death: asset_server.load(format!("{LANCER_ASSET_DIR}/Lancer-Death.png")),
        layout: layout_handle.clone(),
    };

    commands.insert_resource(handles);

    let (atlas_index, flip_x) = direction_to_atlas_and_flip(4); // S par défaut

    commands.spawn((
        Player,
        PlayerDirection {
            index: 4,
            angle_rad: -std::f32::consts::FRAC_PI_2,
        },
        PlayerAnimation {
            state: AnimationState::Idle,
            frame: 0,
            timer: 0.0,
        },
        Sprite {
            image: idle_handle,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: atlas_index as usize,
            }),
            flip_x,
            flip_y: false,
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, 0.0, 1.0).with_scale(Vec3::splat(tiny_scale)),
    ));
}

/// Système de déplacement 8 directions (WASD + diagonales), mise à jour direction et état Walk/Idle.
pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut PlayerDirection, &mut PlayerAnimation)>,
) {
    let dt = time.delta_secs();
    let mut move_x = 0.0f32;
    let mut move_y = 0.0f32;

    if keyboard.pressed(KeyCode::KeyW) {
        move_y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        move_y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        move_x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        move_x += 1.0;
    }

    for (_player, mut transform, mut direction, mut anim) in query.iter_mut() {
        let moving = move_x != 0.0 || move_y != 0.0;
        anim.state = if moving { AnimationState::Walk } else { AnimationState::Idle };

        if moving {
            let len = (move_x * move_x + move_y * move_y).sqrt();
            let norm_x = move_x / len;
            let norm_y = move_y / len;
            transform.translation.x += norm_x * PLAYER_SPEED * dt;
            transform.translation.y += norm_y * PLAYER_SPEED * dt;
            transform.translation.x = transform.translation.x.clamp(-GAME_WIDTH / 2.0 + 50.0, GAME_WIDTH / 2.0 - 50.0);
            transform.translation.y = transform.translation.y.clamp(-GAME_HEIGHT / 2.0 + 50.0, GAME_HEIGHT / 2.0 - 50.0);

            // Angle : atan2(move_x, move_y) avec y positif = haut. Bevy 2D y+ = haut.
            // 0° = haut (N), 90° = droite (E). index 0=N, 1=NE, 2=E, 3=SE, 4=S, 5=SW, 6=W, 7=NW.
            let angle = move_y.atan2(move_x);
            direction.angle_rad = angle;
            direction.index = angle_to_direction_index(angle);
        }
    }
}

/// Convertit un angle (radians, 0 = droite, PI/2 = haut) en indice 0..8.
fn angle_to_direction_index(angle_rad: f32) -> DirectionIndex {
    // angle_rad: 0 = E, PI/2 = N, PI = W, -PI/2 = S.
    // On veut 0=N, 1=NE, 2=E, 3=SE, 4=S, 5=SW, 6=W, 7=NW.
    // N = PI/2 -> 0, E = 0 -> 2, S = -PI/2 -> 4, W = PI -> 6.
    let normalized = (angle_rad + std::f32::consts::TAU) % std::f32::consts::TAU;
    let slice = std::f32::consts::TAU / DIRECTION_COUNT as f32;
    let index = ((normalized + slice / 2.0) / slice) as u8 % DIRECTION_COUNT;
    index
}
