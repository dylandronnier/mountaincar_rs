use std::f32::consts::PI;

use crate::ringpong::{RingPong, RingPongAction, RADIUS, THETA};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rl::MarkovDecisionProcess;
use uilib::{despawn_screen, AIResource, GameMode, GameState};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_DIAMETER: f32 = 30.;

const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub fn mountain_car_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Playing),
        (
            setup_resources,
            setup_decor.after(setup_resources),
            setup_text.after(setup_resources),
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            update_mdp
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(GameMode::Human)),
            update_mdp_ai
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(GameMode::AI))
                .run_if(resource_exists::<AIResource<RingPong>>),
            (
                move_paddle,
                move_ball,
                timer_text_update_system,
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
            despawn_screen::<Paddle>,
            despawn_screen::<Ball>,
            remove_brain.run_if(in_state(GameMode::AI)),
        ),
    );
}

pub fn remove_brain(mut commands: Commands, mut game_mode: ResMut<NextState<GameMode>>) {
    commands.remove_resource::<AIResource<RingPong>>();
    game_mode.set(GameMode::Human)
}

// A unit struct to help identify the timer UI component, since there may be many Text components
#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct StateText;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Resource)]
pub struct Wrapper {
    pub m: RingPong,
}

#[derive(Resource)]
pub struct GameTimer(pub Timer);

pub fn setup_resources(mut commands: Commands) {
    commands.insert_resource(Wrapper { m: RingPong::new() });
    commands.insert_resource(<Time<Fixed>>::from_seconds(1.0 / 50.0));
    commands.insert_resource(GameTimer(Timer::from_seconds(30.0, TimerMode::Once)));
}

fn setup_decor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    wrap: Res<Wrapper>,
) {
    // Spawn the ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(BALL_COLOR),
            transform: Transform::from_translation(wrap.m.ball_pos.extend(2.0))
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            ..default()
        },
        Ball,
    ));

    // Spawn the paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: ((RADIUS + 10.0) * Vec2::from_angle(wrap.m.paddle_angle)).extend(3.0),
                rotation: Quat::from_rotation_z(wrap.m.paddle_angle + PI / 2.0),
                scale: Vec3::new(2.0 * RADIUS * f32::sin(THETA), 20.0, 1.0),
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
    ));
}

fn setup_text(mut commands: Commands, mut wrap: ResMut<Wrapper>, mut timer: ResMut<GameTimer>) {
    // Spawn score text
    wrap.m.reset();

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

    // Reset timer
    timer.0.reset();
}

fn update_mdp(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut wrap: ResMut<Wrapper>,
    time_step: Res<Time<Fixed>>,
) {
    let action = {
        if keyboard_input.pressed(KeyCode::KeyH) {
            RingPongAction::Left
        } else if keyboard_input.pressed(KeyCode::KeyL) {
            RingPongAction::Right
        } else {
            RingPongAction::DoNothing
        }
    };
    wrap.m
        .step(action, time_step.timestep().as_secs_f32())
        .unwrap_or(0.0);
}

fn move_ball(mut query_ball: Query<&mut Transform, With<Ball>>, wrap: Res<Wrapper>) {
    let mut t_ball = query_ball.single_mut();
    t_ball.translation = wrap.m.ball_pos.extend(2.0);
}

fn move_paddle(mut query_paddle: Query<&mut Transform, With<Paddle>>, wrap: Res<Wrapper>) {
    let mut t_paddle = query_paddle.single_mut();
    t_paddle.translation = ((RADIUS + 10.0) * Vec2::from_angle(wrap.m.paddle_angle)).extend(3.0);
    t_paddle.rotation = Quat::from_rotation_z(wrap.m.paddle_angle + PI / 2.0);
}

fn update_mdp_ai(
    mut wrap: ResMut<Wrapper>,
    time_step: Res<Time<Fixed>>,
    brain: Res<AIResource<RingPong>>,
) {
    let action = brain.nn.policy(&wrap.m).unwrap_or_else(|_| {
        error!("AI brain could not compute the action to take!");
        RingPongAction::DoNothing
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
