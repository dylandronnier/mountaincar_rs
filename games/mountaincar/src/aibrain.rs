use std::{collections::HashMap, convert::TryFrom, path::PathBuf};

use candle_core::{Device, Tensor};

use bevy::{math::cubic_splines::CubicCurve, prelude::*};
use rl::Agent;

use crate::mountaincar::MountainCar;

#[derive(Resource)]
pub struct NeuralNet {
    pub nn: Box<dyn Agent<MountainCar<CubicCurve<Vec2>>> + Send + Sync>,
}

pub trait EncodedAgent:
    Agent<MountainCar<CubicCurve<Vec2>>>
    + for<'a> TryFrom<&'a mut HashMap<String, Tensor>>
    + Send
    + Sync
    + 'static
{
    fn load_tensor(file: PathBuf) -> Result<NeuralNet, ()> {
        // Select the device. Try GPU and pick CPU if not found.
        let device = match Device::cuda_if_available(0) {
            Ok(d) => {
                // info!("Loading AI brain on the GPU.");
                d
            }
            Err(_) => {
                // info!("Failed to find a GPU. Loading AI brain on the CPU.");
                Device::Cpu
            }
        };

        let Ok(mut h) = candle_core::safetensors::load(file, &device) else {
            return Err(());
        };
        let Ok(brain) = Self::try_from(&mut h) else {
            return Err(());
        };
        Ok(NeuralNet {
            nn: Box::new(brain),
        })
    }
}
