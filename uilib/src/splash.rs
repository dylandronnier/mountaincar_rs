use super::{despawn_screen, GameState};
use bevy::prelude::*;
use std::path::Path;

/// Path of the icon displayed on the splash screen as a resource to the game.²
#[derive(Resource)]
pub struct IconPath(pub &'static Path);

/// Struct implementing the Plugin trait. Differents configuration can be used.
#[derive(Default)]
pub struct SplashPlugin {
    /// Path of the logo displayed on the screen.
    pub path_logo: Option<&'static Path>,

    /// Color of the background.
    pub color: Color,

    /// Time that prints the splash screen.
    pub duration: f32,
}

impl Plugin for SplashPlugin {
    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
    fn build(&self, app: &mut App) {
        if let Some(pb) = self.path_logo {
            app.insert_resource(IconPath(pb));
        }
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app.insert_resource(SplashTimer(Timer::from_seconds(
            self.duration,
            TimerMode::Once,
        )))
        .insert_resource(ClearColor(self.color))
        // When entering the state, spawn everything needed for this screen
        .add_systems(
            OnEnter(GameState::Splash),
            logo_display.run_if(resource_added::<IconPath>),
        )
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn logo_display(mut commands: Commands, asset_server: Res<AssetServer>, ico: Res<IconPath>) {
    let icon = asset_server.load(ico.0);
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(400.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
