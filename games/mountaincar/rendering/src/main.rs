use std::path::Path;

use bevy::prelude::*;
use uilib::{default_plugin, ButtonColors, Customization, MenuPlugin, SplashPlugin};

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
                path_logo: Some(Path::new("branding/logo.jpg")),
            },
            // Menu configuration
            MenuPlugin {
                title: "Mountain Car",
                colors: Customization {
                    background: Color::DARK_GREEN,
                    buttons: ButtonColors {
                        normal: Color::rgb(0.60, 0.50, 0.65),
                        howered: Color::rgb(0.75, 0.60, 0.85),
                        howered_pressed: Color::rgb(0.25, 0.65, 0.25),
                        pressed: Color::rgb(0.35, 0.75, 0.35),
                    },
                    square: Color::TOMATO,
                },
            },
            // Main game rendering
            gamerender::mountain_car_plugin,
        ))
        .run()
}
