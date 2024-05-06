use bevy::prelude::*;
use rl::{Agent, MarkovDecisionProcess};
mod menu;
mod splash;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameMode {
    AI,
    Human,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Playing,
    Menu,
}

pub use menu::MenuPlugin;
pub use splash::{IconPath, SplashPlugin};

pub fn default_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .insert_state(GameMode::Human)
        .add_systems(Startup, setup);
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    // Spawn 2D Camera
    commands.spawn(Camera2dBundle::default());
}

// Agent as a resource
#[derive(Resource)]
pub struct AIResource<T: MarkovDecisionProcess> {
    pub nn: Box<dyn Agent<T> + Send + Sync>,
}
