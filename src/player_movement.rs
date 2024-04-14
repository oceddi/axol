use bevy::prelude::*;
use bevy_ecs_ldtk::utils::{translation_to_grid_coords, translation_to_ldtk_pixel_coords};

use crate::{events::{RunEvent, WalkEvent}, nowalk::{LevelNoWalk, GRID_SIZE}, player::Player, sprite::MoveDir};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, (handle_player_walk, handle_player_run));
  }
}

fn handle_player_walk(
  mut event: EventReader<WalkEvent>,
  level_nowalk: Res<LevelNoWalk>,
  mut player: Query<&mut Transform, With<Player>>
) {
  if let Some(event) = event.read().next() {
    let direction = event.direction;

    let mut transform = player.get_single_mut().expect("Player despawned");
    let mut change = Vec3::new(0.,0.,0.);

    match direction {
      MoveDir::Up => {
        change.y += 0.3;
      },
      MoveDir::Down => {
        change.y -= 0.3;
      },
      MoveDir::Left => {
        change.x -= 0.3;
      },
      MoveDir::Right => {
        change.x += 0.3;
      }
    }
    
    // Check for no walk
    let destination = transform.translation + change;

    let dest_gridcoords = translation_to_grid_coords(destination.xy(), IVec2::splat(GRID_SIZE));

    if !level_nowalk.in_nowalk(&dest_gridcoords) {
      transform.translation = destination;
    }
  }
}

fn handle_player_run(
  mut event: EventReader<RunEvent>,
  level_nowalk: Res<LevelNoWalk>,
  mut player: Query<&mut Transform, With<Player>>
) {
  if let Some(event) = event.read().next() {
    let direction = event.direction;

    let mut transform = player.get_single_mut().expect("Player despawned");
    let mut change = Vec3::new(0.,0.,0.);

    match direction {
      MoveDir::Up => {
        change.y += 0.5;
      },
      MoveDir::Down => {
        change.y -= 0.5;
      },
      MoveDir::Left => {
        change.x -= 0.5;
      },
      MoveDir::Right => {
        change.x += 0.5;
      }
    }

    // Check for no walk
    let destination = transform.translation + change;

    let dest_gridcoords = translation_to_grid_coords(destination.xy(), IVec2::splat(GRID_SIZE));

    if !level_nowalk.in_nowalk(&dest_gridcoords) {
      transform.translation = destination;
    }


  }
}