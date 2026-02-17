//! Allumina — Action RPG 2D. Point d'entrée Bevy 2D.
//!
//! Résolution jeu : 1280×720. Assets Tiny (100×100) affichés en 2× (200×200).
//! Joueur : sprite Lancer (tiny_rpg_character), 8 directions, gauche/droite en flip.

mod player;
mod animation;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use animation::AnimationState;
use player::{player_movement, Player};

/// Résolution logique du jeu (fixe).
pub const GAME_WIDTH: f32 = 1280.0;
pub const GAME_HEIGHT: f32 = 720.0;

/// Scale d'affichage pour les assets "tiny" (100×100) → doublé en taille.
pub const TINY_SPRITE_SCALE: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Allumina".into(),
                        resolution: WindowResolution::new(GAME_WIDTH, GAME_HEIGHT),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(GameConstants {
            tiny_scale: TINY_SPRITE_SCALE,
        })
        .add_systems(Startup, (setup_camera, setup_player))
        .add_systems(
            Update,
            (
                player_movement,
                animation::animate_player,
            ),
        )
        .run();
}

/// Constantes de jeu (scale tiny, etc.).
#[derive(Resource)]
pub struct GameConstants {
    pub tiny_scale: f32,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    game: Res<GameConstants>,
) {
    player::spawn_lancer(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        game.tiny_scale,
    );
}
