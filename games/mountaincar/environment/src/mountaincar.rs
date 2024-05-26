use std::{convert::TryFrom, error::Error};

use candle_core::Tensor;
use rand::Rng;
use rl::mdp::MarkovDecisionProcess;

pub trait Ground: Send + Sync {
    // The slope of the curve at the given point
    fn slope(&self, x: f32) -> f32;

    // The
    fn derivivative(&self, x: f32) -> f32;
}

// "Mountain car" decision process

pub struct MountainCar<T>
where
    T: Ground,
{
    pub pos: f32,
    pub speed: f32,
    pub ground: T,
}

#[derive(Default, Debug, PartialEq)]
pub enum MountainAction {
    Left = -1,
    Right = 1,
    #[default]
    DoNothing = 0,
}

pub const MOTOR_POWER: f32 = 0.07;
pub const FRICTION: f32 = 0.2;
pub const GRAVITY: f32 = 0.15;

impl<T: Ground> MountainCar<T> {
    pub fn new(g: T) -> Self {
        MountainCar {
            pos: rand::thread_rng().gen_range(0.5..0.6),
            speed: 0.0,
            ground: g,
        }
    }
}

impl<T: Ground> MarkovDecisionProcess for MountainCar<T> {
    type Action = MountainAction;

    fn reset(&mut self) {
        self.pos = rand::thread_rng().gen_range(0.5..0.6);
    }
    fn is_finished(&self) -> bool {
        self.pos > 1.77
    }
    fn step(&mut self, action: Self::Action, time_step: f32) -> Result<f32, Box<dyn Error>> {
        let slope = self.ground.slope(self.pos);
        self.speed += time_step
            * (action as i8 as f32 * MOTOR_POWER - slope * GRAVITY - self.speed * FRICTION);
        self.pos += time_step * self.speed * self.ground.derivivative(self.pos);
        Ok(-1.0) // Reward -1 at each step
    }

    fn feature(&self) -> Tensor {
        Tensor::try_from(vec![self.pos, self.speed]).unwrap()
    }
}
