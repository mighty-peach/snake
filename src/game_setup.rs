use crate::config::{BACKGROUND_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct GameSetup;

impl Plugin for GameSetup {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_startup_system(setup_camera)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
