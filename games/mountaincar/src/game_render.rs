use crate::aibrain::*;
use crate::mountaincar::{MountainAction, MountainCar};
use crate::wrapper_bezier::Wrapper;
use crate::{HEIGHT, WIDTH};
use bevy::reflect::List;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{
    math::cubic_splines::CubicCurve, prelude::*, render::mesh::PrimitiveTopology,
    sprite::MaterialMesh2dBundle,
};
use rl::MarkovDecisionProcess;

use std::ops::{Add, Div};
use uilib::{despawn_screen, AIResource, GameMode, GameState};

const PADDING: f32 = 13.0;

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
                move_car_human
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(GameMode::Human)),
                move_car_ia
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(GameMode::AI))
                    .run_if(resource_exists::<AIResource<MountainCar<CubicCurve<Vec2>>>>),
                (
                    timer_text_update_system,
                    state_text_update_system,
                    end_of_game,
                )
                    .run_if(in_state(GameState::Playing)),
            ),
        )
        .add_systems(
            OnExit(GameState::Playing),
            (
                despawn_screen::<StateText>,
                despawn_screen::<TimeText>,
                despawn_screen::<Car>,
                despawn_screen::<Decor>,
                remove_brain.run_if(in_state(GameMode::AI)),
            ),
        );
}

#[derive(Debug, Clone)]
pub struct TriangleStrip {
    pub points: Vec<Vec3>,
}

pub fn remove_brain(mut commands: Commands, mut game_mode: ResMut<NextState<GameMode>>) {
    commands.remove_resource::<AIResource<MountainCar<CubicCurve<Vec2>>>>();
    game_mode.set(GameMode::Human)
}

pub trait CubicTransform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform;
}

impl From<TriangleStrip> for Mesh {
    fn from(line: TriangleStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let v = Vec2::new(WIDTH, HEIGHT);
        let mut line_points2d: Vec<Vec2> = Vec::new();
        for p in line.points.iter() {
            line_points2d.push(
                <Vec3 as FromReflect>::from_reflect(p)
                    .unwrap()
                    .xy()
                    .add(v.div(2.0))
                    .div(v),
            );
        }
        Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, line_points2d)
    }
}

impl CubicTransform for Transform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform {
        let p = c.position(pos);
        let dp = c.velocity(pos);
        let lambda = dp.distance(Vec2::ZERO);
        Transform::from_xyz(
            p.x - PADDING * dp.y / lambda.powf(0.9),
            p.y + PADDING * dp.x / lambda.powf(0.9),
            z_coordinate,
        )
        .with_rotation(Quat::from_rotation_z(f32::atan(dp.y / dp.x)))
    }
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
    let p = wrap.m.ground.position(1.80);
    let mut triangle_strip = Vec::new();

    for point in wrap.m.ground.iter_positions(40) {
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
            transform: Transform::from_cubic_curve(&wrap.m.ground, 0., 2.0),
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

fn move_car_human(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Car>>,
    mut wrap: ResMut<Wrapper>,
    time_step: Res<Time<Fixed>>,
) {
    for mut t in &mut query {
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
        *t = Transform::from_cubic_curve(&wrap.m.ground, wrap.m.pos, 2.0);
    }
}

fn move_car_ia(
    mut query: Query<&mut Transform, With<Car>>,
    mut wrap: ResMut<Wrapper>,
    time_step: Res<Time<Fixed>>,
    brain: Res<AIResource<MountainCar<CubicCurve<Vec2>>>>,
) {
    for mut t in &mut query {
        let action = brain.nn.policy(&wrap.m).unwrap_or_else(|_| {
            error!("AI brain could not compute the action to take!");
            MountainAction::DoNothing
        });
        wrap.m
            .step(action, time_step.timestep().as_secs_f32())
            .unwrap_or(0.0);
        *t = Transform::from_cubic_curve(&wrap.m.ground, wrap.m.pos, 2.0);
    }
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
