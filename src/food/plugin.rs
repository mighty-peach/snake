use crate::{
    common::types::{Position, Size},
    config::{ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR, SNAKE_SPEED},
};

use super::{types::Food, FoodBehavior, GrowthEvent};
use bevy::{core::FixedTimestep, prelude::*};
use rand::prelude::random;

impl Plugin for FoodBehavior {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(SNAKE_SPEED))
                .with_system(food_spawner),
        )
        .add_event::<GrowthEvent>();
    }
}

fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * (ARENA_WIDTH as f32)) as i32,
            y: (random::<f32>() * (ARENA_HEIGHT as f32)) as i32,
        })
        .insert(Size::square(0.8));
}
