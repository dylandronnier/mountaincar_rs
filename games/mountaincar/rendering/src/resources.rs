use crate::wrapper_bezier::{RockyRoad, Wrapper};
use crate::WIDTH;
use bevy::{math::vec2, prelude::*};
use mountaincar_env::MountainCar;
use mountaincar_mods::mlp::MultiLayerPerceptron;
use mountaincar_mods::tabular::Tabular;
use rfd::FileDialog;
use rl::ai::FileLoader;
use uilib::{AIResource, GameMode, GameState};

// Resource timer
#[derive(Resource)]
pub struct GameTimer(pub Timer);

#[derive(Resource, Default)]
pub enum BrainType {
    #[default]
    Tab,
    Mlp,
}

pub fn setup_resources(mut commands: Commands) {
    let control_points = [
        [
            vec2(-WIDTH.div_euclid(2.0), -83.0), // 0.0
            vec2(-77.0, -645.0),                 // -1120.0
            vec2(311.0, -539.0),                 // 340.0
            vec2(515.0, -326.0),                 // -30.0
        ],
        [
            vec2(515.0, -326.0),                 // 0.0
            vec2(703.0, -130.0),                 // -1120.0
            vec2(714.0, -76.0),                  // 340.0
            vec2(WIDTH.div_euclid(2.0), -133.0), // -30.0
        ],
    ];

    let bezier = CubicBezier::new(control_points).to_curve();

    commands.insert_resource(Wrapper {
        m: MountainCar::new(RockyRoad(bezier)),
    });
    commands.insert_resource(<Time<Fixed>>::from_seconds(1.0 / 50.0));
    commands.insert_resource(GameTimer(Timer::from_seconds(30.0, TimerMode::Once)));
}

pub fn load_brain(
    mut commands: Commands,
    b: Res<BrainType>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_mode: ResMut<NextState<GameMode>>,
) {
    // Picking the file storing the brain
    let Some(file) = FileDialog::new()
        .add_filter("Safetensor file", &["safetensors"])
        .pick_file()
    else {
        info!("No file picked. Return to main menu.");
        game_state.set(GameState::Menu);
        game_mode.set(GameMode::Human);
        return;
    };

    let nn: AIResource<MountainCar<RockyRoad>> = match *b {
        BrainType::Tab => AIResource {
            nn: Box::from(
                <Tabular as FileLoader<MountainCar<RockyRoad>>>::from_file(file).unwrap(),
            ),
        },
        BrainType::Mlp => AIResource {
            nn: Box::from(
                <MultiLayerPerceptron<2, 3> as FileLoader<MountainCar<RockyRoad>>>::from_file(file)
                    .unwrap(),
            ),
        },
    };

    commands.insert_resource(nn);
}
