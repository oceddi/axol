use bevy::prelude::*;

use crate::{axol::{setup_axol_animations, Axol, AxolBundle}, combat::{AttackCooldown, Health}, player::Moving, sprite::{AnimFrame, AnimState, AnimationTimer, AtlasHandles, MoveDir}};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
  fn build(&self, app: &mut App) {
      app.add_systems(Update, process_wait_to_spawn);
  }
}

#[derive(Default, Component)]
pub struct Spawner;

#[derive(Default, Component, Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);

#[derive(Default, Component, Deref, DerefMut)]
pub struct WaveCount(pub u8);


#[derive(Default, Bundle)]
pub struct SpawnerBundle {
  pub spawner: Spawner,
  pub rate: SpawnTimer,
  pub remaining: WaveCount
}

fn process_wait_to_spawn(
  mut commands: Commands,
  time: Res<Time>,
  atlas_handles: Res<AtlasHandles>,
  mut spawn_timers: Query<(&mut SpawnTimer, &mut WaveCount), With<Spawner>>
) {
  let sprite_axol = TextureAtlasSprite {
    index: 0,
    ..default()
  };

  for (mut timer, mut remaining) in spawn_timers.iter_mut() {
    if timer.tick(time.delta()).finished() && remaining.0 > 0 {
      commands.spawn(
        AxolBundle {
          axol: Axol,
          health: Health(20, 20),
          anim_state: AnimState::Idle,
          moving: Moving(false),
          move_dir: MoveDir::Left,
          sprite_sheet: SpriteSheetBundle {
            sprite: sprite_axol.clone(),
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

      remaining.0 -= 1;
    }
  }
}
