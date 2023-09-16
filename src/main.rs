pub mod events;
pub mod game;
pub mod main_menu;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use events::GameOver;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

use crate::game::SimulationState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (transition_to_game_state, transition_to_main_menu))
        .add_systems(Update, (handle_game_over,))
        .add_systems(Update, (exit_game,))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..Default::default()
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit)
    }
}

pub fn handle_game_over(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    game_over_event_reader.iter().for_each(|event| {
        println!("Game Over: {}", event.score);
        next_app_state.set(AppState::GameOver)
    })
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::Game {
            next_app_state.set(AppState::Game);
            println!("entered app state game")
        }
    }
}

pub fn transition_to_main_menu(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("entered app state main menu")
        }
    }
}
