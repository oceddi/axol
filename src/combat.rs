use bevy::prelude::*;
use rand::Rng;

use crate::{axol::Axol, events::{AxolDeath, SwordHitEvent, SwordMissEvent, SwordSwingEvent}, player::Player, sprite::{AnimState, MoveDir}};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, (check_player_attack, handle_axol_damage));
  }
}

#[derive(Default, Component)]
pub struct Health(pub i8);


pub fn check_player_attack(
  mut event: EventReader<SwordSwingEvent>,
  axol: Query<(Entity, &Transform), With<Axol>>,
  player: Query<(&Transform, &MoveDir), With<Player>>,
  mut sword_miss: EventWriter<SwordMissEvent>,
  mut sword_hit: EventWriter<SwordHitEvent>,
) {
  let (player_transform, player_dir) = player.get_single().expect("Player despawned");

  if event.read().next().is_some() {
    for (entity, axol_transform) in axol.iter() {
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
          let mut rng = rand::thread_rng();
          let damage = rng.gen_range(1..=6);

          sword_hit.send(SwordHitEvent{ target: entity, amount: damage });
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

pub fn handle_axol_damage (
  mut event: EventReader<SwordHitEvent>,
  mut axol_list: Query<(&mut AnimState, &mut Health), With<Axol>>,
  mut axol_death: EventWriter<AxolDeath>
) {
  for hit in event.read() {
    let (mut target_state, mut target_health) = axol_list.get_mut(hit.target).expect("No target for attack");

    println!("HIT EVENT {:?}", hit);

    if target_health.0 > 0 {
      target_health.0 -= hit.amount as i8;
      if target_health.0 < 0 {
        *target_state = AnimState::Dead;
        axol_death.send_default();
      } else {
        *target_state = AnimState::IdleDamage;
      }
    }
  }
}