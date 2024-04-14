use bevy::prelude::*;
use std::collections::HashMap;
use crate::{combat::{AttackCooldown, Health}, player::Moving, sprite::{AnimFrame, AnimState, AnimationDirection, AnimationIndices, AnimationTimer, MoveDir}};

#[derive(Default, Component)]
pub struct Axol;

#[derive(Default, Bundle)]
pub struct AxolBundle {
  pub axol: Axol,
  pub health: Health,
  pub anim_state: AnimState,
  pub moving: Moving,
  pub move_dir: MoveDir,
  pub sprite_sheet: SpriteSheetBundle,
  pub animation_indices: AnimationIndices,
  pub anim_timer: AnimationTimer,
  pub anim_frame: AnimFrame,
  pub cooldown: AttackCooldown
}

pub fn setup_axol_animations() -> AnimationIndices {
  let mut animation_indices = AnimationIndices {
    sheet_index: HashMap::new(),
    animations: HashMap::new(),
    timer_duration: HashMap::new()
  };

  // IDLE
  animation_indices.timer_duration.insert(AnimState::Idle, 0.8);
  animation_indices.sheet_index.insert(AnimState::Idle, 4);
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Up), AnimationDirection {
    frames : vec![0, 1],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Down), AnimationDirection {
    frames : vec![0, 1],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Left), AnimationDirection {
    frames : vec![0, 1],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1],
    flip_x: true,
    flip_y: false,
    looping: true
  });

  // IDLE Damage
  animation_indices.timer_duration.insert(AnimState::IdleInjured, 0.8);
  animation_indices.sheet_index.insert(AnimState::IdleInjured, 4);
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Up), AnimationDirection {
    frames : vec![2, 3],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Down), AnimationDirection {
    frames : vec![2, 3],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Left), AnimationDirection {
    frames : vec![2, 3],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Right), AnimationDirection {
    frames : vec![2, 3],
    flip_x: true,
    flip_y: false,
    looping: true
  });

  // WALK
  animation_indices.timer_duration.insert(AnimState::Walk, 0.1);
  animation_indices.sheet_index.insert(AnimState::Walk, 4);
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Up), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Down), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14],
    flip_x: true,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Left), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Right), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14],
    flip_x: true,
    flip_y: false,
    looping: true
  });

  // ATTACK
  animation_indices.timer_duration.insert(AnimState::Attack, 0.08);
  animation_indices.sheet_index.insert(AnimState::Attack, 4);
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Up), AnimationDirection {
    frames : vec![5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Down), AnimationDirection {
    frames : vec![5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Left), AnimationDirection {
    frames : vec![5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Right), AnimationDirection {
    frames : vec![5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: false
  });

  // AttackInjured
  animation_indices.timer_duration.insert(AnimState::AttackInjured, 0.08);
  animation_indices.sheet_index.insert(AnimState::AttackInjured, 4);
  animation_indices.animations.insert((AnimState::AttackInjured, MoveDir::Up), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::AttackInjured, MoveDir::Down), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::AttackInjured, MoveDir::Left), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::AttackInjured, MoveDir::Right), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24],
    flip_x: false,
    flip_y: false,
    looping: false
  });

  // Dead
  animation_indices.timer_duration.insert(AnimState::Dead, 0.4);
  animation_indices.sheet_index.insert(AnimState::Dead, 4);
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Up), AnimationDirection {
    frames : vec![15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Down), AnimationDirection {
    frames : vec![15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Left), AnimationDirection {
    frames : vec![15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Right), AnimationDirection {
    frames : vec![15, 16, 17, 18, 19],
    flip_x: true,
    flip_y: false,
    looping: false
  });

  animation_indices
}