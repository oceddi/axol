use bevy::prelude::*;
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, assets::{LdtkProject, LevelMetadataAccessor}, GridCoords, LdtkIntCell, LevelEvent};
use std::collections::HashSet;

use crate::sprite::AtlasHandles;

pub const GRID_SIZE: i32 = 32;

pub struct NoWalkPlugin;

impl Plugin for NoWalkPlugin {
  fn build(&self, app: &mut App) {
      app.register_ldtk_int_cell::<NoWalkBundle>(2)
         .init_resource::<LevelNoWalk>()
         .add_systems(Startup,  (cache_nowalk_locations).run_if(resource_exists::<AtlasHandles>()))
         .add_systems(Update, cache_nowalk_locations);
  }
}


#[derive(Default, Component)]
struct NoWalk;

#[derive(Default, Bundle, LdtkIntCell)]
struct NoWalkBundle {
  nowalk:NoWalk
}

#[derive(Default, Resource)]
pub struct LevelNoWalk {
    nowalk_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelNoWalk {
    pub fn in_nowalk(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.nowalk_locations.contains(grid_coords)
    }
}

fn cache_nowalk_locations(
    mut level_nowalk: ResMut<LevelNoWalk>,
    mut level_events: EventReader<LevelEvent>,
    nowalk: Query<&GridCoords, With<NoWalk>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single())
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let nowalk_locations = nowalk.iter().copied().collect();

            let new_level_nowalk = LevelNoWalk {
                nowalk_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_nowalk = new_level_nowalk;
        }
    }
}