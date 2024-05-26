use crate::resources::*;
use crate::wrapper_bezier::{CubicTransform, RockyRoad, TriangleStrip, Wrapper};
use crate::HEIGHT;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use mountaincar_env::{MountainAction, MountainCar};
use rl::mdp::MarkovDecisionProcess;
use uilib::{despawn_screen, remove_brain, AIResource, GameMode, GameState};

pub fn mountain_car_plugin(app: &mut App) {
    app.init_resource::<BrainType>()
        .add_systems(
            OnEnter(GameState::Playing),
            (
                setup_resources,
                setup_decor.after(setup_resources),
                setup_text.after(setup_resources),
            ),
        )
        .add_systems(OnEnter(GameMode::AI), load_brain)
        .add_systems(
            FixedUpdate,
            (
                play_human.run_if(in_state(GameMode::Human)),
                play_ai
                    .run_if(in_state(GameMode::AI))
                    .run_if(resource_exists::<AIResource<MountainCar<RockyRoad>>>),
                move_car,
                timer_text_update_system,
                state_text_update_system,
                end_of_game,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (
                despawn_screen::<StateText>,
                despawn_screen::<TimeText>,
                despawn_screen::<Car>,
                despawn_screen::<Decor>,
                remove_brain::<MountainCar<RockyRoad>>.run_if(in_state(GameMode::AI)),
            ),
        );
}

// A unit struct to help identify the timer UI component, since there may be many Text components
#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct StateText;

#[derive(Component)]
struct Car;

#[derive(Component)]
struct Decor;

fn setup_decor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    wrap: Res<Wrapper>,
) {
    // Spawn the ground
    let p = wrap.m.ground.0.position(1.80);
    let mut triangle_strip = Vec::new();

    for point in wrap.m.ground.0.iter_positions(40) {
        triangle_strip.push(Vec3::new(point.x, -HEIGHT.div_euclid(2.0), 2.0));
        triangle_strip.push(Vec3::new(point.x, point.y, 2.0));
    }

    // Spawn background image
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("drawing3.png"),
            transform: Transform::from_xyz(0.0, 0.0, -2.0),
            ..default()
        },
        Decor,
    ));

    // Spawn the flag
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("flag.png"),
            transform: Transform::from_xyz(p.x, p.y + 36.0, 2.0),
            ..default()
        },
        Decor,
    ));

    // Spawn the ground
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(TriangleStrip {
                    points: triangle_strip,
                }))
                .into(),
            material: materials.add(ColorMaterial {
                texture: Some(asset_server.load("texture/stone3.jpg")),
                ..Default::default()
            }),
            ..Default::default()
        },
        Decor,
    ));
}

fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut wrap: ResMut<Wrapper>,
    mut timer: ResMut<GameTimer>,
) {
    // Spawn score text
    // Spawn the car
    wrap.m.reset();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("car6.png"),
            transform: Transform::from_cubic_curve(&wrap.m.ground.0, 0., 2.0),
            ..default()
        },
        Car,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Remaining time:",
                TextStyle {
                    font_size: 40.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 40.0,
                color: Color::RED,
                ..Default::default()
            }),
        ])
        .with_style(Style {
            left: Val::Px(10.0),
            top: Val::Px(1000.0),
            ..Default::default()
        }),
        TimeText,
    ));

    // Spawn state text
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Position: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            ),
            TextSection::new(
                " ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            ),
            TextSection::new(
                ", Speed: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            ),
            TextSection::new(
                " ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::BLACK,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            left: Val::Px(10.0),
            top: Val::Px(30.0),
            ..Default::default()
        }),
        StateText,
    ));

    // Reset timer
    timer.0.reset();
}

fn play_human(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut wrap: ResMut<Wrapper>,
    time_step: Res<Time<Fixed>>,
) {
    let action = {
        if keyboard_input.pressed(KeyCode::KeyH) {
            MountainAction::Left
        } else if keyboard_input.pressed(KeyCode::KeyL) {
            MountainAction::Right
        } else {
            MountainAction::DoNothing
        }
    };
    wrap.m
        .step(action, time_step.timestep().as_secs_f32())
        .unwrap_or(0.0);
}

fn move_car(mut query: Query<&mut Transform, With<Car>>, wrap: Res<Wrapper>) {
    let mut t = query.single_mut();
    *t = Transform::from_cubic_curve(&wrap.m.ground.0, wrap.m.pos, 2.0);
}

fn play_ai(
    mut wrap: ResMut<Wrapper>,
    time_step: Res<Time<Fixed>>,
    brain: Res<AIResource<MountainCar<RockyRoad>>>,
) {
    let action = brain.nn.policy(&wrap.m).unwrap_or_else(|_| {
        error!("AI brain could not compute the action to take!");
        MountainAction::DoNothing
    });
    wrap.m
        .step(action, time_step.timestep().as_secs_f32())
        .unwrap_or(0.0);
}

fn timer_text_update_system(mut query: Query<&mut Text, With<TimeText>>, timer: Res<GameTimer>) {
    for mut text in &mut query {
        let t = timer.0.remaining_secs();
        text.sections[1].value = format!("{t:.1}")
    }
}
fn state_text_update_system(mut query: Query<&mut Text, With<StateText>>, wrap: Res<Wrapper>) {
    for mut text in &mut query {
        let x = wrap.m.pos;
        text.sections[1].value = format!("{x:.3}");
        let v = wrap.m.speed;
        text.sections[3].value = format!("{v:.3}")
    }
}

// Tick the timer, and change state when finished
fn end_of_game(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    wrap: Res<Wrapper>,
) {
    if timer.0.tick(time.delta()).just_finished() || wrap.m.is_finished() {
        game_state.set(GameState::Menu);
    }
}
