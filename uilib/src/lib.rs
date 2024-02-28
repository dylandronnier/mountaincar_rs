use bevy::prelude::*;
mod menu;
mod splash;

//use crate::despawn_screen;
//
//#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameMode {
    AI,
    Human,
}

// impl GameMode {
//     fn lol(self) -> GameState {
//         GameState::Playing(self)
//     }
// }
// impl States for fn(GameMode) -> GameState {}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Playing,
    Menu,
}

pub use menu::MenuPlugin;
pub use splash::{IconPath, SplashPlugin};
// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
// #[derive(Default)]
// pub struct UIPlugin {
//     pub icon: Option<Image>,
// }

//const BEVY_ICON_HANDLE: Handle<Image> = Handle::weak_from_u128(13828845428412094821);

// impl Plugin for UIPlugin {
//     fn build(&self, app: &mut App) {
//         bevy::asset::embedded_asset!(app, "buttons/right.png");
//         bevy::asset::embedded_asset!(app, "buttons/exitRight.png");
//
//         app.add_plugins((splash_plugin, menu_plugin));
//         // At start, the menu is not enabled. This will be changed in `menu_setup` when
//         // entering the `GameState::Menu` state.
//         // Current screen in the menu is handled by an independent state from `GameState`
//         // .add_systems(OnEnter(GameState::Splash), gameover_menu_setup)
//         // Common systems to all screens that handles buttons behavior
//         // .add_systems(
//         //     Update,
//         //     (menu_action, button_system).run_if(in_state(GameState::GameOver)),
//         // )
//         // .add_systems(OnExit(GameState::GameOver), despawn_screen::<MenuScreen>);
//     }
// }

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
