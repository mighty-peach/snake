use crate::common::types::{Direction, Position};
use bevy::prelude::{Component, Deref, DerefMut, Entity};

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default, Deref, DerefMut)]
pub struct SnakeSegments(pub(crate) Vec<Entity>);

#[derive(Default)]
pub struct LastTailPosition(pub(crate) Option<Position>);
