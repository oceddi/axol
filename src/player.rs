use bevy::prelude::*;
use std::collections::HashMap;
use crate::{events::{RunEvent, SwordHitEvent, WalkEvent}, sprite::{AnimFrame, AnimState, AnimationDirection, AnimationIndices, AnimationTimer, AtlasHandles, MoveDir}};

#[derive(Default, Component, PartialEq)]
pub struct Moving(pub bool);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(PostStartup, setup_player)
      .add_systems(Update, (handle_input, animate_player));
  }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
  player: Player,
  amin_state: AnimState,
  moving: Moving,
  move_dir: MoveDir,
  sprite_sheet: SpriteSheetBundle,
  animation_indices: AnimationIndices,
  anim_timer: AnimationTimer,
  anim_frame: AnimFrame
}

pub fn setup_player(
  mut commands: Commands,
  atlas_handles: Res<AtlasHandles>
) {
  let sprite_player = TextureAtlasSprite {
    index: 0,
    ..default()
  };

  commands.spawn(
    PlayerBundle {
      player: Player,
      amin_state: AnimState::Idle,
      moving: Moving(false),
      move_dir: MoveDir::Right,
      sprite_sheet: SpriteSheetBundle {
        sprite: sprite_player,
        texture_atlas: atlas_handles.handles[0].clone(),
        transform: Transform {
          translation: Vec3 { x: 300., y: 200., z: 10.},
          ..default()
        },
        ..default()
      },
      animation_indices: setup_player_animations(),
      anim_timer: AnimationTimer(Timer::from_seconds(0.20, TimerMode::Repeating)),
      anim_frame: AnimFrame(0)
    }
  );
}

pub fn handle_input(
  key: Res<Input<KeyCode>>,
  mouse: Res<Input<MouseButton>>,
  mut walk_event: EventWriter<WalkEvent>,
  mut run_event: EventWriter<RunEvent>,
  mut sword_event: EventWriter<SwordHitEvent>,
  mut player: Query<(&mut Moving, &mut MoveDir, &mut AnimState), With<Player>>
) {
  let (mut moving, mut move_dir, mut anim_state) = player.get_single_mut().expect("player not spawned");
  let mut dir_facing = *move_dir;
  let mut is_moving = false;
  let shift = key.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

  if key.pressed(KeyCode::Space) || mouse.pressed(MouseButton::Left) {
    is_moving = false;
    *anim_state = AnimState::Attack;
    sword_event.send_default();
  } else {
    if key.pressed(KeyCode::Up) {
      dir_facing = MoveDir::Up;
      is_moving = true;
    } else if key.pressed(KeyCode::Down) {
      dir_facing = MoveDir::Down;
      is_moving = true;
    } else if key.pressed(KeyCode::Left) {
      dir_facing = MoveDir::Left;
      is_moving = true;
    } else if key.pressed(KeyCode::Right) {
      dir_facing = MoveDir::Right;
      is_moving = true;
    }
    if is_moving {
      if shift {
        *anim_state = AnimState::Run;
        run_event.send(RunEvent{ direction: dir_facing });
      } else {
        *anim_state = AnimState::Walk;
        walk_event.send(WalkEvent{ direction: dir_facing });
      }
    } else {
      *anim_state = AnimState::Idle;
    }
  }

  *moving = Moving(is_moving);
  *move_dir = dir_facing;
}

pub fn setup_player_animations() -> AnimationIndices {
  let mut animation_indices = AnimationIndices {
    sheet_index: HashMap::new(),
    animations: HashMap::new(),
    timer_duration: HashMap::new()
  };

  // IDLE
  animation_indices.timer_duration.insert(AnimState::Idle, 0.2);
  animation_indices.sheet_index.insert(AnimState::Idle, 0);
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Up), AnimationDirection {
    frames : vec![24, 25, 26, 27, 28, 29, 30, 31],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Down), AnimationDirection {
    frames : vec![16, 17, 18, 19, 20, 21, 22, 23],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Left), AnimationDirection {
    frames : vec![8, 9, 10, 11, 12, 13, 14, 15],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1, 2, 3, 4, 5, 6, 7],
    flip_x: false,
    flip_y: false,
    looping: true
  });

  // WALK
  animation_indices.timer_duration.insert(AnimState::Walk, 0.1);
  animation_indices.sheet_index.insert(AnimState::Walk, 1);
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Up), AnimationDirection {
    frames : vec![36, 37, 38, 39, 40, 41, 42, 43],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Down), AnimationDirection {
    frames : vec![24, 25, 26, 27, 28, 29, 30, 31],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Left), AnimationDirection {
    frames : vec![12, 13, 14, 15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Walk, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1, 2, 3, 4, 5, 6, 7],
    flip_x: false,
    flip_y: false,
    looping: true
  });

  // Run
  animation_indices.timer_duration.insert(AnimState::Run, 0.1);
  animation_indices.sheet_index.insert(AnimState::Run, 1);
  animation_indices.animations.insert((AnimState::Run, MoveDir::Up), AnimationDirection {
    frames : vec![44, 45, 46, 47],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Run, MoveDir::Down), AnimationDirection {
    frames : vec![32, 33, 34, 35],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Run, MoveDir::Left), AnimationDirection {
    frames : vec![20, 21, 22, 23],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Run, MoveDir::Right), AnimationDirection {
    frames : vec![8, 9, 10, 11],
    flip_x: false,
    flip_y: false,
    looping: true
  });

  // ATTACK
  animation_indices.timer_duration.insert(AnimState::Attack, 0.08);
  animation_indices.sheet_index.insert(AnimState::Attack, 2);
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Up), AnimationDirection {
    frames : vec![30, 31, 32, 33, 34, 35, 36, 37, 38, 39],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Down), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Left), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: true
  });

  animation_indices
}

pub fn animate_player(
  time: Res<Time>,
  atlas_handles: Res<AtlasHandles>,
  mut query: Query<(&AnimState, &MoveDir, &mut AnimFrame, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>), With<Player>>
) {
  let (anim_state, move_dir, mut frame, indices, mut timer, mut sprite, mut texture_atlas) = query.get_single_mut().expect("player not spawned");

  timer.tick(time.delta());
  if timer.just_finished() {
    let info = &indices.animations[&(*anim_state, *move_dir)];
    let next_frame_index = 
    if info.looping {
      (frame.0 + 1) % info.frames.len()
    } else if (frame.0 + 1) < info.frames.len() - 1 {
      frame.0 + 1
    } else {
      info.frames.len() - 1
    };
    
    sprite.index = info.frames[next_frame_index];

    frame.0 = next_frame_index;

    sprite.flip_x = info.flip_x;
    sprite.flip_y = info.flip_y;

    if *texture_atlas != atlas_handles.handles[indices.sheet_index[anim_state]] {
      *texture_atlas = atlas_handles.handles[indices.sheet_index[anim_state]].clone();
    }

    *timer = AnimationTimer(Timer::from_seconds(indices.timer_duration[anim_state], TimerMode::Repeating));
  }
}