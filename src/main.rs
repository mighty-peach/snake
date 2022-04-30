mod common;
mod config;
mod food;
mod game_over;
mod game_setup;
mod snake;

use bevy::prelude::*;
use config::*;
use game_setup::GameSetup;

use common::types::{Position, Size};
use food::FoodBehavior;
use game_over::GameOverBehavior;
use snake::{
    types::{LastTailPosition, SnakeSegments},
    SnakeBehavior,
};

fn main() {
    App::new()
        // Setup game
        .add_plugin(GameSetup)
        .add_plugin(FoodBehavior)
        .add_plugin(SnakeBehavior)
        .add_plugin(GameOverBehavior)
        // Setup scaling
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling)
                .with_system(position_translation),
        )
        // Global Structures
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .run();
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / (ARENA_WIDTH as f32) * (window.width() as f32),
            sprite_size.height / (ARENA_HEIGHT as f32) * (window.height() as f32),
            1.0,
        )
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;

        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        )
    }
}
