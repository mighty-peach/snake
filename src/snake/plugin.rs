use bevy::{core::FixedTimestep, prelude::*};

use super::{
    types::{LastTailPosition, SnakeHead, SnakeSegment, SnakeSegments},
    SnakeBehavior,
};
use crate::{
    common::types::{Direction, Position, Size},
    config::{ARENA_HEIGHT, ARENA_WIDTH, SNAKE_HEAD_COLOR, SNAKE_SEGMENT_COLOR, SNAKE_SPEED},
    food::{types::Food, GrowthEvent},
    game_over::{plugin::game_over_listener, GameOverEvent},
};

impl Plugin for SnakeBehavior {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_snake)
            .add_system(snake_movement_input.before(snake_movement))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(SNAKE_SPEED))
                    .with_system(snake_movement)
                    .with_system(snake_eating.after(snake_movement))
                    .with_system(snake_growth.after(snake_eating)),
            )
            .add_system(game_over_listener.after(snake_movement));
    }
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 5, y: 5 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 5, y: 4 }),
    ]);
}

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        // check collisions with walls
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x > ARENA_WIDTH as i32
            || head_pos.y > ARENA_HEIGHT as i32
        {
            game_over_writer.send(GameOverEvent);
        }

        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
    }
}

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (food_entity, food_position) in food_positions.iter() {
            if food_position == head_pos {
                commands.entity(food_entity).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}
