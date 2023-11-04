use crate::mdp::Mdp;
use crate::mountaincar;
use crate::{despawn_screen, Env, GameState};
use bevy::{math::cubic_splines::CubicCurve, prelude::*};

pub struct GamePlugin;

const PADDING: f32 = 22.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(1.0 / 50.0))
            .insert_resource(GameTimer(Timer::from_seconds(30.0, TimerMode::Once)))
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(
                FixedUpdate,
                (
                    move_car,
                    timer_text_update_system,
                    state_text_update_system,
                    end_of_game,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                (despawn_screen::<StateText>, despawn_screen::<TimeText>),
            );
    }
}

pub trait CubicTransform {
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

impl mountaincar::Ground for CubicCurve<Vec2> {
    fn slope(&self, x: f32) -> f32 {
        let v = self.velocity(x);
        v.y / v.x
    }
}

// A unit struct to help identify the timer UI component, since there may be many Text components
#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct StateText;

#[derive(Component)]
struct Car;

// Resource timer
#[derive(Resource)]
struct GameTimer(Timer);

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Env>,
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

    let e = query.single();

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
                e.0.pos.to_string(),
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
                e.0.speed.to_string(),
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

    // Spawn the car
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("car.png"),
            transform: Transform::from_cubic_curve(&e.0.ground, e.0.pos, 2.0),
            ..default()
        },
        Car,
    ));

    timer.0.reset();
}

fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut first_query: Query<&mut Env>,
    mut second_query: Query<&mut Transform, With<Car>>,
    time_step: Res<FixedTime>,
) {
    let action = {
        if keyboard_input.pressed(KeyCode::H) {
            mountaincar::MountainAction::Left
        } else if keyboard_input.pressed(KeyCode::L) {
            mountaincar::MountainAction::Right
        } else {
            mountaincar::MountainAction::DoNothing
        }
    };

    let mut e = first_query.single_mut();
    let mut t = second_query.single_mut();
    e.0.step(action, time_step.period.as_secs_f32());
    *t = Transform::from_cubic_curve(&e.0.ground, e.0.pos, 2.0);
}

fn timer_text_update_system(mut query: Query<&mut Text, With<TimeText>>, timer: Res<GameTimer>) {
    for mut text in &mut query {
        let t = timer.0.remaining_secs();
        text.sections[1].value = format!("{t:.1}")
    }
}
fn state_text_update_system(
    mut query: Query<&mut Text, With<StateText>>,
    second_query: Query<&Env>,
) {
    let e = second_query.single();
    for mut text in &mut query {
        let x = e.0.pos;
        text.sections[1].value = format!("{x:.3}");
        let v = e.0.speed;
        text.sections[3].value = format!("{v:.3}")
    }
}

// Tick the timer, and change state when finished
fn end_of_game(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Env>,
) {
    if timer.0.tick(time.delta()).just_finished() || query.single().0.is_finished() {
        game_state.set(GameState::GameOver);
    }
}
