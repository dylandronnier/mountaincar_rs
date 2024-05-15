use bevy::prelude::*;
use uilib::{default_plugin, ButtonColors, MenuPlugin, SplashPlugin};

mod game_render;

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
            //
            // Defaults plugin
            //
            default_plugin,
            //
            // Splash screen configuration
            //
            SplashPlugin {
                duration: 5.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                path_logo: None,
            },
            //
            // Menu configuration
            //
            MenuPlugin {
                title: "Ring Pong",
                button_colors: ButtonColors {
                    normal: Color::rgb(0.60, 0.50, 0.65),
                    howered: Color::rgb(0.75, 0.60, 0.85),
                    howered_pressed: Color::rgb(0.25, 0.65, 0.25),
                    pressed: Color::rgb(0.35, 0.75, 0.35),
                },
            },
            //
            // Main game rendering
            //
            game_render::mountain_car_plugin,
        ))
        .run()
}
