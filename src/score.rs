use bevy::prelude::*;

use crate::{combat::Health, game::InGameSet, player::Player};

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const HEALTH_TEXT_PADDING: Val = Val::Px(1000.0);
const TEXT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(Score(0))
        .add_systems(Startup, (setup_score, setup_health))
        .add_systems(Update, (update_scoreboard, update_health).in_set(InGameSet::Score));
  }
}

// This resource tracks the game's score
#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component)]
struct ScoreboardUi;

fn setup_score (
    mut commands: Commands
) {
  // Scoreboard
  commands.spawn((
      ScoreboardUi,
      TextBundle::from_sections([
          TextSection::new(
              "Score: ",
              TextStyle {
                  font_size: SCOREBOARD_FONT_SIZE,
                  color: TEXT_COLOR,
                  ..default()
              },
          ),
          TextSection::from_style(TextStyle {
              font_size: SCOREBOARD_FONT_SIZE,
              color: SCORE_COLOR,
              ..default()
          }),
      ])
      .with_style(Style {
          position_type: PositionType::Absolute,
          top: SCOREBOARD_TEXT_PADDING,
          left: SCOREBOARD_TEXT_PADDING,
          ..default()
      }),
  ));
}

fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.to_string();
}

#[derive(Component)]
struct HealthUi;

fn setup_health (
    mut commands: Commands
) {
  // Scoreboard
  commands.spawn((
      HealthUi,
      TextBundle::from_sections([
          TextSection::new(
              "Health: ",
              TextStyle {
                  font_size: SCOREBOARD_FONT_SIZE,
                  color: TEXT_COLOR,
                  ..default()
              },
          ),
          TextSection::from_style(TextStyle {
              font_size: SCOREBOARD_FONT_SIZE,
              color: SCORE_COLOR,
              ..default()
          }),
      ])
      .with_style(Style {
          position_type: PositionType::Absolute,
          top: SCOREBOARD_TEXT_PADDING,
          left: HEALTH_TEXT_PADDING,
          ..default()
      }),
  ));
}

fn update_health(
    player: Query<&Health, With<Player>>, 
    mut query: Query<&mut Text, With<HealthUi>>
) {
    let health = player.get_single().expect("Player despawned");
    let mut text = query.single_mut();
    text.sections[1].value = health.0.to_string();
}