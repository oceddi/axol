use bevy::prelude::*;
use rand::prelude::*;

use crate::events::{SwordHitEvent, SwordMissEvent};
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
  fn build(&self, app: &mut App) {
      app.init_resource::<AudioHandles>()
         .add_systems(Update, (play_sword_hit_sound, play_sword_miss_sound));
  }
}


#[derive(Resource)]
pub struct AudioHandles {
  pub sword_hit_1 : Handle<AudioSource>,
  pub sword_hit_2 : Handle<AudioSource>,
  pub sword_hit_3 : Handle<AudioSource>,
  pub sword_miss_1 : Handle<AudioSource>,
  pub sword_miss_2 : Handle<AudioSource>,
  pub sword_miss_3 : Handle<AudioSource>,
}

impl FromWorld for AudioHandles {
  fn from_world(world: &mut World) -> Self {
    let assets = world.resource::<AssetServer>();

    AudioHandles {
      sword_hit_1 : assets.load("audio/sfx/07_human_atk_sword_1.wav"),
      sword_hit_2 : assets.load("audio/sfx/07_human_atk_sword_2.wav"),
      sword_hit_3 : assets.load( "audio/sfx/07_human_atk_sword_3.wav"),
      sword_miss_1 : assets.load( "audio/sfx/27_sword_miss_1.wav"),
      sword_miss_2 : assets.load( "audio/sfx/27_sword_miss_2.wav"),
      sword_miss_3 : assets.load( "audio/sfx/27_sword_miss_3.wav"),
    }
  }
}

#[derive(Component)]
pub struct SwordAudio;

pub fn play_sword_hit_sound(
  mut commands: Commands,
  handle: Res<AudioHandles>,
  mut event: EventReader<SwordHitEvent>,
  exists: Query<Entity, With<SwordAudio>>
) {
  // Only play 1 sword audio at a time.
  if exists.iter().next().is_some() {
    return;
  }

  let mut rng = rand::thread_rng();
  let random_index = rng.gen_range(0..3);
  let selected_audio_handle = match random_index {
      0 => handle.sword_hit_1.clone(),
      1 => handle.sword_hit_2.clone(),
      2 => handle.sword_hit_3.clone(),
      _ => panic!("Invalid random index"),
  };

  if event.read().next().is_some() {
    commands.spawn((
        SwordAudio,
        AudioBundle {
            source: selected_audio_handle.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
    ));
  }
}

pub fn play_sword_miss_sound(
  mut commands: Commands,
  handle: Res<AudioHandles>,
  mut event: EventReader<SwordMissEvent>,
  exists: Query<Entity, With<SwordAudio>>
) {
  // Only play 1 sword audio at a time.
  if exists.iter().next().is_some() {
    return;
  }

  let mut rng = rand::thread_rng();
  let random_index = rng.gen_range(0..3);
  let selected_audio_handle = match random_index {
      0 => handle.sword_miss_1.clone(),
      1 => handle.sword_miss_2.clone(),
      2 => handle.sword_miss_3.clone(),
      _ => panic!("Invalid random index"),
  };

  if event.read().next().is_some() {
    commands.spawn((
        SwordAudio,
        AudioBundle {
            source: selected_audio_handle.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
    ));
  }
}