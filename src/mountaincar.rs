use crate::mdp;
use candle_core::Tensor;
use rand::Rng;

pub trait Ground: Send + Sync {
    fn slope(&self, x: f32) -> f32;
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

#[derive(Debug, PartialEq)]
pub enum MountainAction {
    Left = -1,
    Right = 1,
    DoNothing = 0,
}

pub const MOTOR_POWER: f32 = 0.1;
pub const FRICTION: f32 = 0.4;
pub const GRAVITY: f32 = 0.1;

impl<T: Ground> MountainCar<T> {
    pub fn new(g: T) -> Self {
        MountainCar {
            pos: rand::thread_rng().gen_range(0.25..0.3),
            speed: 0.0,
            ground: g,
        }
    }
}

impl<T: Ground> mdp::MarkovDecisionProcess for MountainCar<T> {
    type Action = MountainAction;

    fn reset(&mut self) {
        self.pos = 0.0;
    }
    fn is_finished(&self) -> bool {
        self.pos > 0.9
    }
    fn step(
        &mut self,
        a: Self::Action,
        time_step: f32,
    ) -> Result<f32, mdp::NotAllowed<Self::Action>> {
        let slope = self.ground.slope(self.pos);
        self.speed +=
            time_step * (a as i8 as f32 * MOTOR_POWER - slope * GRAVITY - self.speed * FRICTION);
        self.pos += time_step * self.speed;
        Ok(-1.0)
    }

    fn feature(&self) -> Tensor {
        Tensor::try_from(vec![self.pos, self.speed]).unwrap()
    }
}
