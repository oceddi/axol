use bevy::prelude::*;
use std::collections::HashMap;

use crate::combat::Health;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Startup, setup_atlases)
         .add_systems(Update, animate_sprites);
  }
}

#[derive(Component, Clone, Eq, PartialEq, Copy, Debug, Default, Hash)]
pub enum AnimState {
  #[default]
  Idle,
  IdleInjured,
  Walk,
  Run,
  Attack,
  AttackInjured,
  Dead
}

#[derive(Debug, Default, Component, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MoveDir {
  Up,
  Left,
  Down,
  #[default]
  Right
}

#[derive(Component, Clone)]
pub struct AnimationDirection {
  pub frames: Vec<usize>,
  pub flip_x: bool,
  pub flip_y: bool,
  pub looping: bool,
}

#[derive(Default, Component, Clone)]
pub struct AnimationIndices {
  pub sheet_index : HashMap<AnimState, usize>,
  pub animations : HashMap<(AnimState, MoveDir), AnimationDirection>,
  pub timer_duration: HashMap<AnimState, f32>
}

#[derive(Default, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Default, Component, PartialEq, Eq)]
pub struct AnimFrame(pub usize);

#[derive(Resource, Clone)]
pub struct AtlasHandles {
  pub handles : [Handle<TextureAtlas>; 5]
}

pub fn setup_atlases(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlas>>,
) {
  let mut handle_vector : Vec<Handle<TextureAtlas>> = Vec::new();

  // PLAYER IDLE/IDLEINJURED
  let texture_player: Handle<Image> = asset_server.load("sprites/player/Warrior_idle.png");
  let atlas_player = TextureAtlas::from_grid(
    texture_player,
    Vec2::new(80.0, 80.0),
    10,
    8,
    None,
    None
  );
  let handle = atlases.add(atlas_player);
  handle_vector.push(handle);

  // PLAYER WALK/RUN
  let texture_player: Handle<Image> = asset_server.load("sprites/player/Warrior_walk_run.png");
  let atlas_player = TextureAtlas::from_grid(
    texture_player,
    Vec2::new(80.0, 80.0),
    12,
    4,
    None,
    None
  );
  let handle = atlases.add(atlas_player);
  handle_vector.push(handle);

  // PLAYER SWORD ATTACKS
  let texture_player: Handle<Image> = asset_server.load("sprites/player/Warrior_sword_attacks.png");
  let atlas_player = TextureAtlas::from_grid(
    texture_player,
    Vec2::new(80.0, 80.0),
    10,
    5,
    None,
    None
  );
  let handle = atlases.add(atlas_player);
  handle_vector.push(handle);

  // PLAYER DEATH
  let texture_player: Handle<Image> = asset_server.load("sprites/player/Warrior_death.png");
  let atlas_player = TextureAtlas::from_grid(
    texture_player,
    Vec2::new(80.0, 80.0),
    4,
    2,
    None,
    None
  );
  let handle = atlases.add(atlas_player);
  handle_vector.push(handle);


  // AXOL
  let texture_player: Handle<Image> = asset_server.load("sprites/axol/Alien_sheet.png");
  let atlas_player = TextureAtlas::from_grid(
    texture_player,
    Vec2::new(128.0, 64.0),
    5,
    5,
    None,
    None
  );
  let handle = atlases.add(atlas_player);
  handle_vector.push(handle);

  commands.insert_resource(AtlasHandles { handles: handle_vector.try_into().expect("Expected vector length of 5") });
}

pub fn animate_sprites(
  time: Res<Time>,
  atlas_handles: Res<AtlasHandles>,
  mut query: Query<(&mut AnimState, &MoveDir, &Health, &mut AnimFrame, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite, &mut Handle<TextureAtlas>)>
) {
  for (mut anim_state, move_dir, health, mut frame, indices, mut timer, mut sprite, mut texture_atlas) in &mut query {

    timer.tick(time.delta());
    if timer.just_finished() {
      let info = &indices.animations[&(*anim_state, *move_dir)];
      let next_frame_index = 
      if info.looping {
        (frame.0 + 1) % info.frames.len()
      } else if (frame.0 + 1) < info.frames.len() - 1 {
        frame.0 + 1
      } else if *anim_state != AnimState::Dead {
        // Not looping and at end of frames.  Go back to Idle If not Dead.
        if health.0 < health.1 as i8 {
          *anim_state = AnimState::IdleInjured;
        } else {
          *anim_state = AnimState::Idle;
        }
        0
      } else {
        // Dead... stay dead
        info.frames.len() - 1
      };
      
      sprite.index = info.frames[next_frame_index];

      frame.0 = next_frame_index;

      sprite.flip_x = info.flip_x;
      sprite.flip_y = info.flip_y;

      if *texture_atlas != atlas_handles.handles[indices.sheet_index[&anim_state]] {
        *texture_atlas = atlas_handles.handles[indices.sheet_index[&anim_state]].clone();
      }

      *timer = AnimationTimer(Timer::from_seconds(indices.timer_duration[&anim_state], TimerMode::Repeating));
    }
  }
}