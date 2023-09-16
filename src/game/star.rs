use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::AppState;

use super::SimulationState;

pub const STAR_SIZE: f32 = 30.;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SPAWN_DURATION: f32 = 1.;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), (spawn_stars,))
            .add_systems(OnExit(AppState::Game), (despawn_stars,))
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_DURATION, TimerMode::Repeating),
        }
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_star_size = STAR_SIZE / 2.;
    let x_min = 0. + half_star_size;
    let x_max = window.width() - half_star_size;
    let y_min = 0. + half_star_size;
    let y_max = window.height() - half_star_size;

    for _ in 0..NUMBER_OF_STARS {
        let position_x = x_min + random::<f32>() * (x_max - x_min);
        let position_y = y_min + random::<f32>() * (y_max - y_min);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(position_x, position_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    star_query.for_each(|star| {
        commands.entity(star).despawn();
    })
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    star_spawn_timer: Res<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let half_star_size = STAR_SIZE / 2.;
        let x_min = 0. + half_star_size;
        let x_max = window.width() - half_star_size;
        let y_min = 0. + half_star_size;
        let y_max = window.height() - half_star_size;

        let position_x = x_min + random::<f32>() * (x_max - x_min);
        let position_y = y_min + random::<f32>() * (y_max - y_min);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(position_x, position_y, 0.),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}
