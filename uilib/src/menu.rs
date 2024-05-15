use crate::{despawn_screen, GameMode, GameState};
use bevy::asset::embedded_asset;
use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin {
    pub title: &'static str,
    pub button_colors: ButtonColors,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        bevy::asset::embedded_asset!(app, "buttons/play.png");
        bevy::asset::embedded_asset!(app, "buttons/exit.png");
        bevy::asset::embedded_asset!(app, "buttons/deep-learning.png");

        app.insert_resource(self.button_colors)
            .insert_resource(MenuTitle(self.title))
            // At start, the menu is not enabled. This will be changed in `menu_setup` when
            // entering the `GameState::Menu` state.
            // Current screen in the menu is handled by an independent state from `GameState`
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            // Systems to handle the main menu screen
            //.add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), despawn_screen::<MenuScreen>);
    }
}
#[derive(Component)]
struct MenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Aiplay,
    Quit,
}

#[derive(Resource, Clone, Copy)]
pub struct ButtonColors {
    pub normal: Color,
    pub howered: Color,
    pub howered_pressed: Color,
    pub pressed: Color,
}

#[derive(Resource)]
pub struct MenuTitle(&'static str);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

type Modified = (Changed<Interaction>, With<Button>);

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        Modified,
    >,
    button_colors: Res<ButtonColors>,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
                button_colors.pressed.into()
            }
            (Interaction::Hovered, Some(_)) => button_colors.howered_pressed.into(),
            (Interaction::Hovered, None) => button_colors.howered.into(),
            (Interaction::None, None) => button_colors.normal.into(),
        }
    }
}

fn menu_setup(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    menu_title: Res<MenuTitle>,
    asset_server: Res<AssetServer>,
) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::BLACK,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            menu_title.0,
                            TextStyle {
                                font_size: 80.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: button_colors.normal.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("embedded://uilib/buttons/play.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Play", button_text_style.clone()));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: button_colors.normal.into(),
                                ..default()
                            },
                            MenuButtonAction::Aiplay,
                        ))
                        .with_children(|parent| {
                            let icon =
                                asset_server.load("embedded://uilib/buttons/deep-learning.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "AI play",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: button_colors.normal.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("embedded://uilib/buttons/exit.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), Modified>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_mode: ResMut<NextState<GameMode>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Playing);
                    game_mode.set(GameMode::Human);
                }
                MenuButtonAction::Aiplay => {
                    game_state.set(GameState::Playing);
                    game_mode.set(GameMode::AI);
                }
            }
        }
    }
}
