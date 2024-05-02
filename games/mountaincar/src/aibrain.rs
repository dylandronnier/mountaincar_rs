use std::convert::From;

use bevy::{math::cubic_splines::CubicCurve, prelude::*};
use rl::Agent;

use crate::mountaincar::MountainCar;

#[derive(Resource)]
pub struct NeuralNet {
    pub nn: Box<dyn Agent<MountainCar<CubicCurve<Vec2>>> + Send + Sync>,
}

impl<T: Agent<MountainCar<CubicCurve<Vec2>>> + Send + Sync + 'static> From<T> for NeuralNet {
    fn from(value: T) -> Self {
        NeuralNet {
            nn: Box::new(value),
        }
    }
}
