pub mod types {
    use bevy::prelude::*;

    #[derive(Component, Clone, Copy, PartialEq, Eq)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Component)]
    pub struct Size {
        pub width: f32,
        pub height: f32,
    }
    impl Size {
        pub fn square(x: f32) -> Self {
            Self {
                width: x,
                height: x,
            }
        }
    }

    #[derive(PartialEq, Copy, Clone)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
    }
    impl Direction {
        pub fn opposite(self) -> Self {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
            }
        }
    }
}
