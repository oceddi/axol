use bevy::prelude::*;
use rand::prelude::*;

use crate::{events::{AxolBiteEvent, AxolDeath, PlayerDeathEvent, StartGameEvent, SwordHitEvent, SwordMissEvent}, game::{GameState, InGameSet}};
pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<AudioHandles>()
       .add_systems(Update, 
          (
          play_cave_theme_1_sound,
          play_sword_hit_sound,
          play_sword_miss_sound,
          play_axol_bite_sound,
          play_axol_died_sound,
          play_player_died_sound
        ).in_set(InGameSet::PlayAudio));
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
  pub axol_bite : Handle<AudioSource>,
  pub axol_death : Handle<AudioSource>,
  pub player_death: Handle<AudioSource>,
  pub cave_theme_1: Handle<AudioSource>
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
      axol_bite : assets.load("audio/sfx/07_landing_on_grass_1.wav"),
      axol_death : assets.load("audio/sfx/24_orc_death_spin.wav"),
      player_death: assets.load("audio/sfx/14_human_death_spin.wav"),
      cave_theme_1: assets.load("audio/music/cave_theme_1.wav")
    }
  }
}

#[derive(Component)]
pub struct SwordAudio;

#[derive(Component)]
pub struct BiteAudio;

#[derive(Component)]
pub struct DeathAudio;

#[derive(Component)]
pub struct Music;

pub fn play_cave_theme_1_sound(
  mut commands: Commands,
  handle: Res<AudioHandles>,
  state: Res<State<GameState>>,
  mut event: EventReader<StartGameEvent>,
  mut query_music: Query<(Entity, &AudioSink), With<Music>>
) {
  if event.read().next().is_some() {

    if let Ok((entity, music)) = query_music.get_single_mut() {
      music.stop();
      commands.entity(entity).despawn();
    }

    println!("PLAY MUSIC {:?}", state);
    commands.spawn((
        Music,
        AudioBundle {
            source: handle.cave_theme_1.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
    ));
  }
}

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

pub fn play_axol_bite_sound(
  mut commands: Commands,
  handle: Res<AudioHandles>,
  mut event: EventReader<AxolBiteEvent>,
  exists: Query<Entity, With<BiteAudio>>
) {
  // Only play 1 bite audio at a time.
  if exists.iter().next().is_some() {
    return;
  }

  if event.read().next().is_some() {
    commands.spawn((
        BiteAudio,
        AudioBundle {
            source: handle.axol_bite.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
    ));
  }
}

pub fn play_axol_died_sound(
  mut commands: Commands,
  handle: Res<AudioHandles>,
  mut event: EventReader<AxolDeath>,
  exists: Query<Entity, With<DeathAudio>>
) {
  // Only play 1 death audio at a time.
  if exists.iter().next().is_some() {
    return;
  }

  if event.read().next().is_some() {
    commands.spawn((
        DeathAudio,
        AudioBundle {
            source: handle.axol_death.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
    ));
  }
}

pub fn play_player_died_sound(
  mut commands: Commands,
  handle: Res<AudioHandles>,
  mut event: EventReader<PlayerDeathEvent>,
  exists: Query<Entity, With<DeathAudio>>
) {
  // Only play 1 death audio at a time.
  if exists.iter().next().is_some() {
    return;
  }

  if event.read().next().is_some() {
    commands.spawn((
        DeathAudio,
        AudioBundle {
            source: handle.player_death.clone(),
            settings: PlaybackSettings::DESPAWN,
        },
    ));
  }
}