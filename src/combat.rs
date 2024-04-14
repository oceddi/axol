use bevy::prelude::*;
use rand::Rng;

use crate::{axol::Axol, events::{AxolBiteEvent, AxolDeath, PlayerDeathEvent, SwordHitEvent, SwordMissEvent, SwordSwingEvent}, player::Player, score::Score, sprite::{AnimState, MoveDir}};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, (check_player_attack, handle_player_damage, check_axol_attack, handle_axol_damage));
  }
}

#[derive(Default, Component)]
pub struct Health(pub i8, pub u8);


#[derive(Default, Component, Deref, DerefMut)]
pub struct AttackCooldown(pub Timer);


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

pub fn handle_player_damage (
  mut event: EventReader<AxolBiteEvent>,
  mut player: Query<(&mut AnimState, &mut Health), With<Player>>,
  mut player_death: EventWriter<PlayerDeathEvent>
) {
  for hit in event.read() {
    let (mut target_state, mut target_health) = player.get_single_mut().expect("No Player");

    if target_health.0 > 0 {
      target_health.0 -= hit.amount as i8;
      if target_health.0 <= 0 {
        *target_state = AnimState::Dead;
        player_death.send_default();
        println!("YOU DIED!!");
      } else {
        *target_state = AnimState::Idle;
      }
    }
  }
}

pub fn check_axol_attack(
  time: Res<Time>,
  mut axol: Query<(&Transform, &mut AnimState, &MoveDir, &Health, &mut AttackCooldown), With<Axol>>,
  player: Query<(&Transform, &Health), With<Player>>,
  mut bite: EventWriter<AxolBiteEvent>,
) {
  let (player_transform, player_health) = player.get_single().expect("Player despawned");

  for  (axol_transform, mut anim_state, axol_dir, axol_health, mut cooldown_timer) in axol.iter_mut() {
    let distance = axol_transform.translation.truncate().distance(player_transform.translation.truncate());

    cooldown_timer.tick(time.delta());

    if distance < 70. && axol_health.0 > 0 && player_health.0 > 0 && cooldown_timer.just_finished() {
      // Make sure axol is facing the player...
      let axol_facing = match axol_dir {
          MoveDir::Up => player_transform.translation.y > axol_transform.translation.y,
          MoveDir::Left => player_transform.translation.x < axol_transform.translation.x,
          MoveDir::Down => player_transform.translation.y < axol_transform.translation.y,
          MoveDir::Right => player_transform.translation.x > axol_transform.translation.x,
      };

      if axol_facing {
        let mut rng = rand::thread_rng();
        let damage = rng.gen_range(1..=6);

        bite.send(AxolBiteEvent{ amount: damage });

        if axol_health.0 < axol_health.1 as i8{
          *anim_state = AnimState::AttackInjured;
        } else {
          *anim_state = AnimState::Attack;
        }
      } else {
        // switch directions  
      }
    }
  }
  
}


pub fn handle_axol_damage (
  mut score: ResMut<Score>,
  mut event: EventReader<SwordHitEvent>,
  mut axol_list: Query<(&mut AnimState, &mut Health), With<Axol>>,
  mut axol_death: EventWriter<AxolDeath>
) {
  for hit in event.read() {
    let (mut target_state, mut target_health) = axol_list.get_mut(hit.target).expect("No target for attack");

    if target_health.0 > 0 {
      target_health.0 -= hit.amount as i8;
      if target_health.0 <= 0 {
        *target_state = AnimState::Dead;
        axol_death.send_default();
        **score += 100;
      }
    }
  }
}