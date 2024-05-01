use std::{collections::HashMap, convert::TryFrom};

use candle_core::{Device, Tensor};
use rfd::FileDialog;

use bevy::{math::cubic_splines::CubicCurve, prelude::*};
use rl::Agent;

use crate::mountaincar::MountainCar;

#[derive(Resource)]
pub struct NeuralNet {
    pub nn: Box<dyn Agent<MountainCar<CubicCurve<Vec2>>> + Send + Sync>,
}

pub fn load_tensor<A>(mut commands: Commands)
where
    A: Agent<MountainCar<CubicCurve<Vec2>>>
        + for<'a> TryFrom<&'a mut HashMap<String, Tensor>>
        + Send
        + Sync
        + 'static,
{
    // Select the device. Try GPU and pick CPU if not found.
    let device = match Device::cuda_if_available(0) {
        Ok(d) => {
            info!("Loading AI brain on the GPU.");
            d
        }
        Err(_) => {
            info!("Failed to find a GPU. Loading AI brain on the CPU.");
            Device::Cpu
        }
    };

    // Picking the file storing the brain
    let Some(file) = FileDialog::new()
        .add_filter("Safetensor file", &["safetensors"])
        .pick_file()
    else {
        error!("No file picked");
        return;
    };
    let Ok(mut h) = candle_core::safetensors::load(file, &device) else {
        error!("Invalid file");
        return;
    };
    let Ok(brain) = A::try_from(&mut h) else {
        error!("Invalid");
        return;
    };
    commands.insert_resource(NeuralNet {
        nn: Box::new(brain),
    });
}
