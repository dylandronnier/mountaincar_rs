use crate::{despawn_screen, Curve, GameState};
use bevy::{math::cubic_splines::CubicCurve, prelude::*};

use rand::Rng;

const MOTOR_POWER: f32 = 30.0;
const GRAVITY: f32 = 65.0;
const PADDING: f32 = 22.0;
const FRICTION: f32 = 0.1;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1.0 / 50.0))
            .insert_resource(GameTimer(Timer::from_seconds(30.0, TimerMode::Once)))
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(
                FixedUpdate,
                (move_car, text_update_system, end_of_game).run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                (despawn_screen::<Car>, despawn_screen::<TimeText>),
            );
    }
}

trait CubicTransform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform;
}

impl CubicTransform for Transform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform {
        let p = c.position(pos);
        let dp = c.velocity(pos).normalize();
        Transform::from_xyz(p.x - PADDING * dp.y, p.y + PADDING * dp.x, z_coordinate)
            .with_rotation(Quat::from_rotation_z(f32::atan(dp.y / dp.x)))
    }
}

// A unit struct to help identify the timer UI component, since there may be many Text components
#[derive(Component)]
struct TimeText;

// A unit
#[derive(Component, Default)]
struct Car {
    position: f32,
    speed: f32,
}

// Resource timer
#[derive(Resource)]
struct GameTimer(Timer);

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Curve>,
    mut timer: ResMut<GameTimer>,
) {
    // Spawn score text
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

    // Spawn the car
    let pos = rand::thread_rng().gen_range(0.25..0.3);
    let c = query.single();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("car.png"),
            transform: Transform::from_cubic_curve(&c.0, pos, 2.0),
            ..default()
        },
        Car {
            position: pos,
            speed: 0.0,
        },
    ));

    timer.0.reset();
}

fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Car)>,
    second_query: Query<&Curve>,
    time_step: Res<FixedTime>,
) {
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::H) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::L) {
        direction += 1.0;
    }

    let mut dp: Vec2;
    let c = second_query.single();

    for (mut transform, mut car) in &mut query {
        dp = c.0.velocity(car.position);
        car.speed += 2.0
            * time_step.period.as_secs_f32()
            * (direction * MOTOR_POWER - dp.normalize().y * GRAVITY - car.speed * FRICTION);
        car.position += 2.0 * car.speed * time_step.period.as_secs_f32() / dp.distance(Vec2::ZERO);
        *transform = Transform::from_cubic_curve(&c.0, car.position, 2.0);
    }
}

fn text_update_system(mut query: Query<&mut Text, With<TimeText>>, timer: Res<GameTimer>) {
    for mut text in &mut query {
        let t = timer.0.remaining_secs();
        text.sections[1].value = format!("{t:.1}")
    }
}

// Tick the timer, and change state when finished
fn end_of_game(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Car>,
) {
    if timer.0.tick(time.delta()).just_finished() || query.single().position > 0.92 {
        game_state.set(GameState::GameOver);
    }
}
