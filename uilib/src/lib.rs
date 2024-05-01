use bevy::prelude::*;
use rl::MarkovDecisionProcess;
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

pub trait Render<T>
where
    T: MarkovDecisionProcess,
{
    type Plugin: Plugin;
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
