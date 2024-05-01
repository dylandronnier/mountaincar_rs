use std::{path::PathBuf, str::FromStr};

use bevy::prelude::*;
use uilib::{GameMode, GameState, IconPath, MenuPlugin, SplashPlugin};

mod aibrain;
mod game_render;
mod mdp;
mod mountaincar;
mod wrapper_bezier;

// Window size
const HEIGHT: f32 = 1080.0;
const WIDTH: f32 = 1620.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mountain Car".into(),
                resolution: (WIDTH, HEIGHT).into(),
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .init_state::<GameState>()
        .insert_state(GameMode::Human)
        .insert_resource(IconPath(PathBuf::from_str("branding/logo.jpg").unwrap()))
        .add_systems(Startup, setup)
        .add_plugins((SplashPlugin, MenuPlugin, game_render::GamePlugin))
        .run()
}

fn setup(mut commands: Commands) {
    // Spawn 2D Camera
    commands.spawn(Camera2dBundle::default());
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
