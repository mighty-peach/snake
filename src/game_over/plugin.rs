use bevy::prelude::*;

use crate::{
    food::types::Food,
    snake::plugin::spawn_snake,
    snake::types::{SnakeSegment, SnakeSegments},
};

use super::{GameOverBehavior, GameOverEvent};

impl Plugin for GameOverBehavior {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOverEvent>();
    }
}

pub fn game_over_listener(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    foods: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    segments_res: ResMut<SnakeSegments>,
) {
    if game_over_reader.iter().next().is_some() {
        println!("Game Over!");
        for e in foods.iter().chain(segments.iter()) {
            commands.entity(e).despawn()
        }
        spawn_snake(commands, segments_res);
    }
}
