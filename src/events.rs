use bevy::prelude::*;

use crate::sprite::MoveDir;

pub struct EventPlugin;

impl Plugin for EventPlugin {
  fn build(&self, app: &mut App) {
      app.add_event::<StartGameEvent>()
         .add_event::<WalkEvent>()
         .add_event::<RunEvent>()
         .add_event::<SwordSwingEvent>()
         .add_event::<SwordMissEvent>()
         .add_event::<SwordHitEvent>()
         .add_event::<AxolBiteEvent>()
         .add_event::<AxolDeath>()
         .add_event::<PlayerDeathEvent>()
         .add_event::<GameOverEvent>()
         .add_event::<NewWaveEvent>();
  }
}

#[derive(Event, Default)]
pub struct StartGameEvent;


#[derive(Event)]
pub struct WalkEvent {
  pub direction: MoveDir,
  pub entity:Entity,
}

#[derive(Event, Default)]
pub struct RunEvent {
  pub direction: MoveDir
}


#[derive(Event, Default)]
pub struct SwordSwingEvent;

#[derive(Event, Default)]
pub struct SwordMissEvent;

#[derive(Event, Debug)]
pub struct SwordHitEvent {
  pub target: Entity,
  pub amount: u8
}

#[derive(Event, Default)]
pub struct AxolBiteEvent {
  pub amount: u8
}


#[derive(Event, Default)]
pub struct AxolDeath;

#[derive(Event, Default)]
pub struct PlayerDeathEvent;

#[derive(Event, Default)]
pub struct GameOverEvent;

#[derive(Event, Default)]
pub struct NewWaveEvent;
