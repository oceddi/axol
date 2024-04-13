use bevy::prelude::*;

use crate::{axol::Axol, events::{SwordHitEvent, SwordMissEvent, SwordSwingEvent}, player::Player, sprite::MoveDir};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, check_player_attack);
  }
}

pub fn check_player_attack(
  mut event: EventReader<SwordSwingEvent>,
  axol: Query<&Transform, With<Axol>>,
  player: Query<(&Transform, &MoveDir), With<Player>>,
  mut sword_miss: EventWriter<SwordMissEvent>,
  mut sword_hit: EventWriter<SwordHitEvent>,
) {
  let (player_transform, player_dir) = player.get_single().expect("Player despawned");

  if event.read().next().is_some() {
    for axol_transform in axol.iter() {
      let distance = axol_transform.translation.truncate().distance(player_transform.translation.truncate());

      if distance < 70. {
        // Make sure player is facing the enemy...
        let player_facing = match player_dir {
            MoveDir::Up => axol_transform.translation.y > player_transform.translation.y,
            MoveDir::Left => axol_transform.translation.x < player_transform.translation.x,
            MoveDir::Down => axol_transform.translation.y < player_transform.translation.y,
            MoveDir::Right => axol_transform.translation.x > player_transform.translation.x,
        };

        if player_facing {
            sword_hit.send_default();
        } else {
            sword_miss.send_default();
        }
      } else {
        //println!("{}", distance);
        sword_miss.send_default();
      }
    }
  }
}