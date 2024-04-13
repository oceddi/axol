use audio::GameAudioPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use events::EventPlugin;
use player::PlayerPlugin;
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

