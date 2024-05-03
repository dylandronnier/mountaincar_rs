use crate::mlp::MultiLayerPerceptron;
use crate::mountaincar::MountainCar;
use crate::tabular::Tabular;
use crate::wrapper_bezier::Wrapper;
use crate::WIDTH;
use bevy::{
    math::{cubic_splines::CubicCurve, vec2},
    prelude::*,
};
use rfd::FileDialog;
use rl::FileLoader;
use uilib::AIResource;

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
        m: MountainCar::new(bezier),
    });
    commands.insert_resource(<Time<Fixed>>::from_seconds(1.0 / 50.0));
    commands.insert_resource(GameTimer(Timer::from_seconds(30.0, TimerMode::Once)));
}

pub fn load_brain(mut commands: Commands, b: Res<BrainType>) {
    // Picking the file storing the brain
    let Some(file) = FileDialog::new()
        .add_filter("Safetensor file", &["safetensors"])
        .pick_file()
    else {
        error!("No file picked");
        return;
    };

    let nn: AIResource<MountainCar<CubicCurve<Vec2>>> =
        match *b {
            BrainType::Tab => AIResource {
                nn: Box::from(
                    <Tabular as FileLoader<MountainCar<CubicCurve<Vec2>>>>::from_file(file)
                        .unwrap(),
                ),
            },
            BrainType::Mlp => {
                AIResource {
                    nn:
                        Box::from(
                            <MultiLayerPerceptron<2, 3> as FileLoader<
                                MountainCar<CubicCurve<Vec2>>,
                            >>::from_file(file)
                            .unwrap(),
                        ),
                }
            }
        };

    commands.insert_resource(nn);
}
