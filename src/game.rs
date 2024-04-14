use bevy::prelude::*;

use crate::{axol::Axol, combat::Health, events::{GameOverEvent, NewWaveEvent, PlayerDeathEvent, StartGameEvent}, player::Player, score::Score, spawner::{SpawnTimer, Spawner, SpawnerBundle, WaveCount}, sprite::{AnimFrame, AnimState}};


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
  PlayAudio,
  PlayerMovement,
  Animations,
  Camera,
  Combat,
  Restart,
  Score
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    InGame,
    Paused,
    NextWave,
    Restart,
    GameOver,
}

#[derive(Resource, Default)]
pub struct Game {
  wave_number: u8,
  lives_remaining: u8,
  high_score: usize
}


#[derive(Default, Resource, Deref, DerefMut)]
struct GameTimer(Timer);

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<Game>()
       .init_resource::<GameTimer>()
       .add_state::<GameState>()
       .add_systems(OnEnter(GameState::Splash), splash_setup)
       .add_systems(OnExit(GameState::Splash), (despawn_screen::<OnSplashScreen>, setup_new_game))
       .add_systems(Update, (handle_game_over, setup_spawner))
       .add_systems(Update, (wait_for_restart).in_set(InGameSet::Restart))
       .add_systems(Update, (check_for_wave_cleared).in_set(InGameSet::Score))
       .add_systems(Update, (game_state_input_events, handle_player_death))
       .configure_sets(
      Update,
       (
        InGameSet::Camera,
        InGameSet::Combat,
        InGameSet::PlayAudio,
        InGameSet::PlayerMovement,
        InGameSet::Score
       ).run_if(in_state(GameState::InGame))
    )
    .configure_sets(
      Update,
       (
        InGameSet::Restart,
       ).run_if(in_state(GameState::Restart))
    );
  }
}

fn setup_new_game (
  mut game: ResMut<Game>,
  mut score: ResMut<Score>,
  mut startgame_event: EventWriter<StartGameEvent>,
  mut newwave_event: EventWriter<NewWaveEvent>,
) {
  game.wave_number = 1;
  game.lives_remaining = 1;

  *score = Score(0);

  startgame_event.send_default();
  newwave_event.send_default();
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {

  if keyboard_input.just_pressed(KeyCode::S) && *state.get() == GameState::Splash {
    println!("GAME STATE INGAME");
    next_state.set(GameState::InGame);
  }

  if keyboard_input.just_pressed(KeyCode::Escape) {
    match state.get() {
        GameState::InGame => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::InGame),
        _ => (),
    }
  }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;


fn splash_setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>
) {
  let icon = asset_server.load("screens/splash.png");
  // Display the logo
  commands
      .spawn((
          NodeBundle {
              style: Style {
                  align_items: AlignItems::Center,
                  justify_content: JustifyContent::Center,
                  width: Val::Percent(100.0),
                  height: Val::Percent(100.0),
                  ..default()
              },
              ..default()
          },
          OnSplashScreen,
      ))
      .with_children(|parent| {
          parent.spawn(ImageBundle {
              style: Style {
                  // This will set the logo to be 200px wide, and auto adjust its height
                  width: Val::Px(1280.0),
                  ..default()
              },
              image: UiImage::new(icon),
              ..default()
          });
      });
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_player_death(
  mut commands: Commands,
  mut event: EventReader<PlayerDeathEvent>,
  mut game_over: EventWriter<GameOverEvent>,
  mut player: Query<(&mut AnimState, &mut AnimFrame), With<Player>>,
  mut game: ResMut<Game>,
  mut next_state: ResMut<NextState<GameState>>,
  spawner: Query<Entity, With<Spawner>>,
) {
  if event.read().next().is_some() {
    let (mut state, mut frame) = player.get_single_mut().expect("No Player");

    *state = AnimState::Dead;
    *frame = AnimFrame(0);

    if game.lives_remaining > 0 {
      game.lives_remaining -= 1;
      next_state.set(GameState::Restart);
      commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    } else {
      game_over.send_default();
      let spawn_entity = spawner.get_single().expect("Spawner despawned");
      commands.entity(spawn_entity).despawn_recursive();
    }
  }
}

fn wait_for_restart(
  mut commands: Commands,
  time: Res<Time>,
  mut timer: ResMut<GameTimer>,
  mut player: Query<(&mut AnimState, &mut AnimFrame, &mut Health), With<Player>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  if timer.tick(time.delta()).finished() {
    println!("RESTARTING");
    let (mut state, mut frame, mut health) = player.get_single_mut().expect("Player despawned");

    *state = AnimState::Idle;
    *frame = AnimFrame(0);
    health.0 = health.1 as i8;
    next_state.set(GameState::InGame);
  }
}

fn handle_game_over(
  mut commands: Commands,
  mut event: EventReader<GameOverEvent>,
  entities: Query<Entity, Or<(With<Axol>, With<Player>)>>,
  mut wave_events: EventReader<NewWaveEvent>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  if event.read().next().is_some() {
    println!("GAME OVER");
    for entity in entities.iter() {
      commands.entity(entity).despawn_recursive();
    }
    wave_events.clear();
    next_state.set(GameState::Splash);
  }
}

pub fn setup_spawner(
  mut commands: Commands,
  game: Res<Game>,
  mut event: EventReader<NewWaveEvent>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  if event.read().next().is_some() {
    let (wave_count, duration) = match game.wave_number {
      1..=2 => (2, 8.0),
      3..=4 => (4, 6.0),
      5..=10 => (6, 5.0),
      _ => (10, 4.0)
    };
    println!("{} {} {}", game.wave_number, wave_count, duration);
    commands.spawn(SpawnerBundle {
      spawner: Spawner,
      rate: SpawnTimer(Timer::from_seconds(duration, TimerMode::Repeating)),
      remaining: WaveCount(wave_count)
    });
    next_state.set(GameState::InGame);
    event.clear();
  }
}

pub fn check_for_wave_cleared(
  mut commands: Commands,
  mut game: ResMut<Game>,
  spawner: Query<(Entity, &WaveCount), With<Spawner>>,
  enemies: Query<&Health, With<Axol>>,
  mut event: EventWriter<NewWaveEvent>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  let mut total = 0;

  for (entity, count) in spawner.iter() {
    total += count.0;
  }

  for axol_health in enemies.iter() {
    if axol_health.0 > 0 {
      total += 1;
    }
  }

  if total == 0 {
    println!("WAVE CLEARED");
    game.wave_number += 1;

    for (entity, _) in spawner.iter() {
      commands.entity(entity).despawn_recursive();
    }
    event.send_default();
    next_state.set(GameState::NextWave);
  }
}