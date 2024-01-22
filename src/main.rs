use bevy::prelude::*;

// mod ai;
mod game_over;
mod game_render;
mod mdp;
// mod mlp;
mod mountaincar;
mod wrapper_bezier;

const HEIGHT: f32 = 1080.0;
const WIDTH: f32 = 1620.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mountain Car".into(),
                resolution: (WIDTH, HEIGHT).into(),
                // mode: WindowMode::BorderlessFullscreen,
                // decorations: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugins((game_over::GameOverPlugin, game_render::GamePlugin))
        .run()
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
