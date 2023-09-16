use bevy::prelude::*;

use crate::{events::GameOver, AppState};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), (insert_score,))
            .add_systems(OnExit(AppState::Game), (remove_score,))
            .add_systems(Update, (update_high_scores_on_game_over,));
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct HighScores {
    pub scores: Vec<u32>,
}

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>()
}

pub fn update_high_scores_on_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    game_over_event_reader
        .iter()
        .for_each(|event| high_scores.scores.push(event.score));
}
