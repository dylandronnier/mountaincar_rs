#![warn(missing_docs)]
//! Hello
//!
use bevy::prelude::*;
pub use menu::MenuPlugin;
use rl::{Agent, MarkovDecisionProcess};
pub use splash::{IconPath, SplashPlugin};

mod menu;
mod splash;

/// Enum class to determine who is playing the game: AI or human.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameMode {
    /// AI is playing.
    AI,
    /// Human is playing.
    Human,
}

/// The state in which the game is.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// The game is displaying the splash screen.
    #[default]
    Splash,

    /// The game is played.
    Playing,

    /// The game menu is being displayed.
    Menu,
}

/// Plugin that display the initiate the app for a basic 2D game.
pub fn default_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .insert_state(GameMode::Human)
        .add_systems(Startup, setup);
}

/// Recursively despawn entities in the game.
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    // Spawn 2D Camera
    commands.spawn(Camera2dBundle::default());
}

/// RL agent as a resource stored.
#[derive(Resource)]
pub struct AIResource<T: MarkovDecisionProcess> {
    /// Smart pointer to the actual agent.
    pub nn: Box<dyn Agent<T> + Send + Sync>,
}

/// Remove the neural net fro resources and switch to Human Game mode.
pub fn remove_brain<T: MarkovDecisionProcess + 'static>(
    mut commands: Commands,
    mut game_mode: ResMut<NextState<GameMode>>,
) {
    commands.remove_resource::<AIResource<T>>();
    game_mode.set(GameMode::Human)
}
