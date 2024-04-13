use audio::GameAudioPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use events::EventPlugin;
use player::{Player, PlayerPlugin};
use player_movement::PlayerMovementPlugin;
use sprite::SpritePlugin;

// AXOL
mod audio;
mod events;
mod player;
mod player_movement;
mod sprite;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()), LdtkPlugin))
        .add_plugins((EventPlugin, PlayerPlugin, PlayerMovementPlugin, SpritePlugin, GameAudioPlugin))
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_follow_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
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