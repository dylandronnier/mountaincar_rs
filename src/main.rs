use bevy::prelude::*;

// mod ai;
mod game_over;
mod game_render;
mod mdp;
// mod mlp;
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
                ..default()
            }),
            ..default()
        }))
        .init_state::<game_over::GameState>()
        .add_plugins((game_over::GameOverPlugin, game_render::GamePlugin))
        .run()
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
