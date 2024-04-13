use bevy::prelude::*;
use std::collections::HashMap;
use crate::{player::Moving, sprite::{AnimFrame, AnimState, AnimationDirection, AnimationIndices, AnimationTimer, AtlasHandles, MoveDir}};

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
  anim_state: AnimState,
  moving: Moving,
  move_dir: MoveDir,
  sprite_sheet: SpriteSheetBundle,
  animation_indices: AnimationIndices,
  anim_timer: AnimationTimer,
  anim_frame: AnimFrame
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
      anim_state: AnimState::Idle,
      moving: Moving(false),
      move_dir: MoveDir::Left,
      sprite_sheet: SpriteSheetBundle {
        sprite: sprite_axol,
        texture_atlas: atlas_handles.handles[3].clone(),
        transform: Transform {
          translation: Vec3{ x: 305., y: 220., z: 10. },
          ..default()
        },
        ..default()
      },
      animation_indices: setup_axol_animations(),
      anim_timer: AnimationTimer(Timer::from_seconds(0.8, TimerMode::Repeating)),
      anim_frame: AnimFrame(0)
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
  animation_indices.sheet_index.insert(AnimState::Idle, 3);
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

  animation_indices
}