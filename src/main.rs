use audio::GameAudioPlugin;
use axol::AxolPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use combat::CombatPlugin;
use events::EventPlugin;
use nowalk::NoWalkPlugin;
use player::{Player, PlayerPlugin};
use player_movement::PlayerMovementPlugin;
use score::ScorePlugin;
use sprite::SpritePlugin;

// AXOL
mod audio;
mod axol;
mod combat;
mod events;
mod nowalk;
mod player;
mod player_movement;
mod score;
mod sprite;


fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), LdtkPlugin))
        .add_plugins((EventPlugin, PlayerPlugin, PlayerMovementPlugin, SpritePlugin, GameAudioPlugin, AxolPlugin, CombatPlugin, NoWalkPlugin, ScorePlugin))
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_follow_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.40;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("axol.ldtk"),
        ..Default::default()
    });
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>
) {
    let mut camera_transform = camera.get_single_mut().expect("Camera not spawned");
    let player_transform = player.get_single().expect("Player not spawned");

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
