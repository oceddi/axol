use bevy::prelude::*;
use std::collections::HashMap;
use crate::{combat::{AttackCooldown, Health}, player::Moving, sprite::{AnimFrame, AnimState, AnimationDirection, AnimationIndices, AnimationTimer, AtlasHandles, MoveDir}};

pub struct AxolPlugin;

impl Plugin for AxolPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(PostStartup, setup_axol);
  }
}

#[derive(Default, Component)]
pub struct Axol;

#[derive(Default, Bundle)]
pub struct AxolBundle {
  axol: Axol,
  health: Health,
  anim_state: AnimState,
  moving: Moving,
  move_dir: MoveDir,
  sprite_sheet: SpriteSheetBundle,
  animation_indices: AnimationIndices,
  anim_timer: AnimationTimer,
  anim_frame: AnimFrame,
  cooldown: AttackCooldown
}


pub fn setup_axol(
  mut commands: Commands,
  atlas_handles: Res<AtlasHandles>
) {
  let sprite_axol = TextureAtlasSprite {
    index: 0,
    ..default()
  };

  commands.spawn(
    AxolBundle {
      axol: Axol,
      health: Health(20, 20),
      anim_state: AnimState::Idle,
      moving: Moving(false),
      move_dir: MoveDir::Left,
      sprite_sheet: SpriteSheetBundle {
        sprite: sprite_axol,
        texture_atlas: atlas_handles.handles[4].clone(),
        transform: Transform {
          translation: Vec3{ x: 605., y: 620., z: 10. },
          ..default()
        },
        ..default()
      },
      animation_indices: setup_axol_animations(),
      anim_timer: AnimationTimer(Timer::from_seconds(0.8, TimerMode::Repeating)),
      anim_frame: AnimFrame(0),
      cooldown: AttackCooldown(Timer::from_seconds(1.5, TimerMode::Repeating))
    },
  );
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