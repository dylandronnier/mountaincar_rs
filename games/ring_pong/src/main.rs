use bevy::prelude::*;
use uilib::{default_plugin, MenuPlugin, SplashPlugin};

mod game_render;
mod ringpong;

// Window size
const HEIGHT: f32 = 1080.0;
const WIDTH: f32 = 1620.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ring Pong".into(),
                resolution: (WIDTH, HEIGHT).into(),
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            default_plugin,
            // Splash screen configuration
            SplashPlugin {
                duration: 5.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                path_logo: None,
            },
            // Menu configuration
            MenuPlugin,
            // Main game rendering
            game_render::mountain_car_plugin,
        ))
        .run()
}
