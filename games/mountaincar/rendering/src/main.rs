use std::{path::PathBuf, str::FromStr};

use bevy::prelude::*;
use uilib::{default_plugin, MenuPlugin, SplashPlugin};

mod gamerender;
mod resources;
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
        .add_plugins((
            default_plugin,
            // Splash screen configuration
            SplashPlugin {
                duration: 5.0,
                color: Color::rgb(0.0, 0.0, 0.0),
                path_logo: PathBuf::from_str("branding/logo.jpg").ok(),
            },
            // Menu configuration
            MenuPlugin,
            // Main game rendering
            gamerender::mountain_car_plugin,
        ))
        .run()
}
