use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SIZE: f32 = 64.;
pub const ENEMY_SPEED: f32 = 200.;
pub const ENEMY_SPAWN_DURATION: f32 = 5.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), (spawn_enemies,))
            .add_systems(OnExit(AppState::Game), (despawn_enemies,))
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_DURATION, TimerMode::Repeating),
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.;
    let x_min = 0. + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0. + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for _ in 0..NUMBER_OF_ENEMIES {
        let position_x = x_min + random::<f32>() * (x_max - x_min);
        let position_y = y_min + random::<f32>() * (y_max - y_min);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(position_x, position_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.)
                    .normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    enemy_query.for_each(|enemy| {
        commands.entity(enemy).despawn();
    })
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.;
    let x_min = 0. + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0. + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    let mut direction_changed = false;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min {
            enemy.direction.x = enemy.direction.x.abs();
            direction_changed = true;
        } else if translation.x > x_max {
            enemy.direction.x = -enemy.direction.x.abs();
            direction_changed = true;
        }
        if translation.y < y_min {
            enemy.direction.y = enemy.direction.y.abs();
            direction_changed = true;
        } else if translation.y > y_max {
            enemy.direction.y = -enemy.direction.y.abs();
            direction_changed = true;
        }
    }
    if direction_changed {
        commands.spawn(AudioBundle {
            source: asset_server.load("audio/pluck_002.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..Default::default()
            },
        });
    }
}

pub fn confine_enemy_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
) {
    if let Ok(mut transform) = enemy_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.;

        let x_min = 0. + half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = 0. + half_enemy_size;
        let y_max = window.height() - half_enemy_size;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let half_enemy_size = ENEMY_SIZE / 2.;
        let x_min = 0. + half_enemy_size;
        let x_max = window.width() - half_enemy_size;
        let y_min = 0. + half_enemy_size;
        let y_max = window.height() - half_enemy_size;

        let position_x = x_min + random::<f32>() * (x_max - x_min);
        let position_y = y_min + random::<f32>() * (y_max - y_min);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(position_x, position_y, 0.),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.)
                    .normalize(),
            },
        ));
    }
}
