// #![feature(adt_const_params)]
// #![feature(generic_const_exprs)]
// #![feature(variant_count)]

use bevy::{
    math::{cubic_splines::CubicCurve, vec2},
    prelude::*,
    render::mesh::PrimitiveTopology,
    sprite::MaterialMesh2dBundle,
};

// mod ai;
mod game_over;
mod game_render;
mod mdp;
// mod mlp;
mod mountaincar;

const HEIGHT: f32 = 1080.0;
const WIDTH: f32 = 1620.0;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct Env(mountaincar::MountainCar<CubicCurve<Vec2>>);

#[derive(Debug, Clone)]
pub struct TriangleStrip {
    pub points: Vec<Vec3>,
}

impl From<TriangleStrip> for Mesh {
    fn from(line: TriangleStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mountain Car".into(),
                resolution: (WIDTH, HEIGHT).into(),
                // mode: WindowMode::BorderlessFullscreen,
                // decorations: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((game_over::GameOverPlugin, game_render::GamePlugin))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn background image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("drawing3.png"),
        transform: Transform::from_xyz(0.0, 0.0, -2.0),
        ..default()
    });

    let control_points = [[
        vec2(-WIDTH.div_euclid(2.0), -80.0), // 0.0
        vec2(450.0, -1200.0),                // -1120.0
        vec2(630.0, 260.0),                  // 340.0
        vec2(WIDTH.div_euclid(2.0), -60.0),  // -30.0
    ]];

    let bezier = CubicBezier::new(control_points).to_curve();

    let mut triangle_strip = Vec::new();

    for point in bezier.iter_positions(100) {
        triangle_strip.push(Vec3::new(point.x, -HEIGHT.div_euclid(2.0), 2.0));
        triangle_strip.push(Vec3::new(point.x, point.y, 2.0));
    }

    // Spawn the flag
    let p = bezier.position(0.92);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("flag.png"),
        transform: Transform::from_xyz(p.x, p.y + 36.0, 2.0),
        ..default()
    });

    // Spawn the ground
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(TriangleStrip {
                points: triangle_strip,
            }))
            .into(),
        material: materials.add(Color::rgb(0.3, 0.2, 0.1).into()),
        ..Default::default()
    });

    // Spawn the associated bezier
    commands.spawn(Env(mountaincar::MountainCar::new(bezier)));
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
