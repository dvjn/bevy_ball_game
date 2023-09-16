use bevy::audio::PlaybackMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::GameOver;
use crate::AppState;

use super::enemy::{Enemy, ENEMY_SIZE};
use super::score::Score;
use super::star::{Star, STAR_SIZE};
use super::SimulationState;

pub const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SIZE: f32 = 64.;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, ConfinementSystemSet.after(MovementSystemSet))
            .add_systems(OnEnter(AppState::Game), (spawn_player,))
            .add_systems(OnExit(AppState::Game), (despawn_player,))
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                    enemy_hit_player,
                    player_hit_star,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    player_query.for_each(|player| {
        commands.entity(player).despawn();
    })
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0., 1., 0.)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0., -1., 0.)
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.)
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.;

        let x_min = 0. + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0. + half_player_size;
        let y_max = window.height() - half_player_size;

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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let touch_distance = ENEMY_SIZE / 2. + PLAYER_SIZE / 2.;

    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        enemy_query.for_each(|enemy_transform| {
            let distance = enemy_transform
                .translation
                .distance(player_transform.translation);
            if distance <= touch_distance {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        })
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
) {
    let touch_distance = STAR_SIZE / 2. + PLAYER_SIZE / 2.;

    if let Ok(player_transform) = player_query.get_single() {
        star_query.for_each(|(star_entity, star_transform)| {
            let distance = star_transform
                .translation
                .distance(player_transform.translation);
            if distance <= touch_distance {
                score.value += 1;
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/laserLarge_000.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Despawn,
                        ..Default::default()
                    },
                });
                commands.entity(star_entity).despawn();
            }
        })
    }
}
