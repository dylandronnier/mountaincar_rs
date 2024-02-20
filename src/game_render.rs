use crate::mdp::MarkovDecisionProcess;
use crate::mountaincar::{MountainAction, MountainCar};
use crate::wrapper_bezier::Wrapper;
use crate::{despawn_screen, GameState};
use crate::{HEIGHT, WIDTH};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{
    math::cubic_splines::CubicCurve, math::vec2, prelude::*, render::mesh::PrimitiveTopology,
    sprite::MaterialMesh2dBundle,
};

pub struct GamePlugin;

const PADDING: f32 = 22.0;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let control_points = [[
            vec2(-WIDTH.div_euclid(2.0), -80.0), // 0.0
            vec2(450.0, -1200.0),                // -1120.0
            vec2(630.0, 260.0),                  // 340.0
            vec2(WIDTH.div_euclid(2.0), -60.0),  // -30.0
        ]];

        let bezier = CubicBezier::new(control_points).to_curve();

        app.insert_resource(Wrapper {
            m: MountainCar::new(bezier),
        })
        .insert_resource(<Time<Fixed>>::from_seconds(1.0 / 50.0))
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

#[derive(Debug, Clone)]
pub struct TriangleStrip {
    pub points: Vec<Vec3>,
}

pub trait CubicTransform {
    fn from_cubic_curve(c: &CubicCurve<Vec2>, pos: f32, z_coordinate: f32) -> Transform;
}

impl From<TriangleStrip> for Mesh {
    fn from(line: TriangleStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::RENDER_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    wrap: Res<Wrapper>,
    mut timer: ResMut<GameTimer>,
) {
    // Spawn 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn background image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("drawing3.png"),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    });

    let mut triangle_strip = Vec::new();

    for point in wrap.m.ground.iter_positions(100) {
        triangle_strip.push(Vec3::new(point.x, -HEIGHT.div_euclid(2.0), 2.0));
        triangle_strip.push(Vec3::new(point.x, point.y, 2.0));
    }

    // Spawn the flag
    let p = wrap.m.ground.position(0.92);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("flag.png"),
        transform: Transform::from_xyz(p.x, p.y + 36.0, 2.0),
        ..default()
    });

    let custom_texture_handle: Handle<Image> = asset_server.load("texture/stone2.png");
    // Spawn the ground
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(TriangleStrip {
                points: triangle_strip,
            }))
            .into(),
        // transform: Transform::from_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(custom_texture_handle)),
        ..Default::default()
    });

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

    // Spawn the car
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("car.png"),
            transform: Transform::from_cubic_curve(&wrap.m.ground, 0., 2.0),
            ..default()
        },
        Car,
    ));

    // commands.insert_resource(Env(MountainCar::new(&phy.0)));

    timer.0.reset();
}

fn move_car(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Car>>,
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

    let mut t = query.single_mut();
    wrap.m
        .step(action, time_step.timestep().as_secs_f32())
        .unwrap_or(0.0);
    *t = Transform::from_cubic_curve(&wrap.m.ground, wrap.m.pos, 2.0);
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
        game_state.set(GameState::GameOver);
    }
}
