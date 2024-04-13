use bevy::prelude::*;

use crate::sprite::MoveDir;

pub struct EventPlugin;

impl Plugin for EventPlugin {
  fn build(&self, app: &mut App) {
      app.add_event::<WalkEvent>()
         .add_event::<RunEvent>()
         .add_event::<SwordSwingEvent>()
         .add_event::<SwordMissEvent>()
         .add_event::<SwordHitEvent>()
         .add_event::<AxolDeath>();
  }
}

#[derive(Event, Default)]
pub struct WalkEvent {
  pub direction: MoveDir
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
pub struct AxolDeath;


