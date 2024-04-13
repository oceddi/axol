use bevy::prelude::*;

use crate::{events::{RunEvent, WalkEvent}, player::Player, sprite::MoveDir};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, (handle_player_walk, handle_player_run));
  }
}

fn handle_player_walk(
  mut event: EventReader<WalkEvent>,
  mut player: Query<&mut Transform, With<Player>>
) {
  if let Some(event) = event.read().next() {
    let direction = event.direction;

    let (mut transform) = player.get_single_mut().expect("Player despawned");

    match direction {
      MoveDir::Up => {
        transform.translation.y += 0.2;
      },
      MoveDir::Down => {
        transform.translation.y -= 0.2;
      },
      MoveDir::Left => {
        transform.translation.x -= 0.2;
      },
      MoveDir::Right => {
        transform.translation.x += 0.2;
      }
    }
  }
}

fn handle_player_run(
  mut event: EventReader<RunEvent>,
  mut player: Query<&mut Transform, With<Player>>
) {
  if let Some(event) = event.read().next() {
    let direction = event.direction;

    let (mut transform) = player.get_single_mut().expect("Player despawned");

    match direction {
      MoveDir::Up => {
        transform.translation.y += 0.5;
      },
      MoveDir::Down => {
        transform.translation.y -= 0.5;
      },
      MoveDir::Left => {
        transform.translation.x -= 0.5;
      },
      MoveDir::Right => {
        transform.translation.x += 0.5;
      }
    }
  }
}