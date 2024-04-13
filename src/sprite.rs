use bevy::prelude::*;
use std::collections::HashMap;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Startup, setup_atlases);
  }
}

#[derive(Component, Clone, Eq, PartialEq, Copy, Debug, Default, Hash)]
pub enum AnimState {
  #[default]
  Idle,
  Walk
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
  pub handles : [Handle<TextureAtlas>; 2]
}

pub fn setup_atlases(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut atlases: ResMut<Assets<TextureAtlas>>,
) {
  let mut handle_vector : Vec<Handle<TextureAtlas>> = Vec::new();

  // PLAYER IDLE
  let texture_player: Handle<Image> = asset_server.load("sprites/player/Warrior_idle.png");
  let atlas_player = TextureAtlas::from_grid(
    texture_player,
    Vec2::new(80.0, 80.0),
    8,
    4,
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

  commands.insert_resource(AtlasHandles { handles: handle_vector.try_into().expect("Expected vector length of 2") });
}