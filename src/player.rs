use bevy::prelude::*;
use std::collections::HashMap;
use crate::{combat::Health, events::{RunEvent, StartGameEvent, SwordSwingEvent, WalkEvent}, game::{GameState, InGameSet}, sprite::{AnimFrame, AnimState, AnimationDirection, AnimationIndices, AnimationTimer, AtlasHandles, MoveDir}};

#[derive(Default, Component, PartialEq)]
pub struct Moving(pub bool);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(OnExit(GameState::Splash), setup_player)
         .add_systems(Update,
           (handle_input).in_set(InGameSet::PlayerMovement)
          );
  }
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle)]
pub struct PlayerBundle {
  player: Player,
  health: Health,
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
  atlas_handles: Res<AtlasHandles>,
) {
  let sprite_player = TextureAtlasSprite {
    index: 0,
    ..default()
  };

  commands.spawn(
    PlayerBundle {
      player: Player,
      health: Health(20, 20),
      amin_state: AnimState::Idle,
      moving: Moving(false),
      move_dir: MoveDir::Right,
      sprite_sheet: SpriteSheetBundle {
        sprite: sprite_player,
        texture_atlas: atlas_handles.handles[0].clone(),
        transform: Transform {
          translation: Vec3 { x: 500., y: 500., z: 10.},
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
  mut sword_event: EventWriter<SwordSwingEvent>,
  mut player: Query<(Entity, &mut Moving, &mut MoveDir, &mut AnimState, &mut AnimFrame, &Health), With<Player>>
) {
  let (entity, mut moving, mut move_dir, mut anim_state, mut anim_frame, health) = player.get_single_mut().expect("player not spawned");
  let mut dir_facing = *move_dir;
  let mut is_moving = false;
  let shift = key.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

  if *anim_state == AnimState::Dead {
    return;
  }

  if key.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
    is_moving = false;
    *anim_state = AnimState::Attack;
    anim_frame.0 = 0;
    sword_event.send_default();
  } else if *anim_state != AnimState::Attack {
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
        walk_event.send(WalkEvent{ direction: dir_facing, entity: entity });
      }
    } else if *anim_state != AnimState::Attack {
      if health.0 < health.1 as i8 {
        *anim_state = AnimState::IdleInjured;
      } else {
        *anim_state = AnimState::Idle;
      }
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
  animation_indices.timer_duration.insert(AnimState::Idle, 0.15);
  animation_indices.sheet_index.insert(AnimState::Idle, 0);
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Up), AnimationDirection {
    frames : vec![30, 31, 32, 33, 34, 35, 36, 37, 38, 39],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Down), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Left), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::Idle, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: true
  });

  // IDLE Injured
  animation_indices.timer_duration.insert(AnimState::IdleInjured, 0.15);
  animation_indices.sheet_index.insert(AnimState::IdleInjured, 0);
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Up), AnimationDirection {
    frames : vec![70, 71, 72, 73, 74, 75, 76, 77, 78, 79],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Down), AnimationDirection {
    frames : vec![60, 61, 62, 63, 64, 65, 66, 67, 68, 69],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Left), AnimationDirection {
    frames : vec![50, 51, 52, 53, 54, 55, 56, 57, 58, 59],
    flip_x: false,
    flip_y: false,
    looping: true
  });
  animation_indices.animations.insert((AnimState::IdleInjured, MoveDir::Right), AnimationDirection {
    frames : vec![40, 41, 42, 43, 44, 45, 46, 47, 48, 49],
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
    looping: false
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Down), AnimationDirection {
    frames : vec![20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Left), AnimationDirection {
    frames : vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Attack, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    flip_x: false,
    flip_y: false,
    looping: false
  });

  // DYING
  animation_indices.timer_duration.insert(AnimState::Dead, 0.2);
  animation_indices.sheet_index.insert(AnimState::Dead, 3);
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Up), AnimationDirection {
    frames : vec![0, 1, 2, 3],
    flip_x: true,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Down), AnimationDirection {
    frames : vec![0, 1, 2, 3],
    flip_x: false,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Left), AnimationDirection {
    frames : vec![0, 1, 2, 3],
    flip_x: true,
    flip_y: false,
    looping: false
  });
  animation_indices.animations.insert((AnimState::Dead, MoveDir::Right), AnimationDirection {
    frames : vec![0, 1, 2, 3],
    flip_x: false,
    flip_y: false,
    looping: false
  });

  animation_indices
}

