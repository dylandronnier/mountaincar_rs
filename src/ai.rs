use candle_core::{Module, Tensor};
use candle_nn::VarMap;

mod mdp;
mod mlp;
mod mountaincar;

use crate::mdp::{Agent, Mdp};
use crate::mlp::MultiLayerPerceptron;
use crate::mountaincar::{Ground, MountainAction};

impl<T: Ground> Agent<2, mountaincar::MountainCar<T>> for MultiLayerPerceptron<2, 3> {
    fn policy(
        &self,
        s: &mountaincar::MountainCar<T>,
    ) -> <mountaincar::MountainCar<T> as crate::mdp::Mdp<2>>::Action {
        let i = self
            .forward(
                &candle_core::Tensor::from_slice(
                    &[s.pos, s.speed],
                    (2,),
                    &candle_core::Device::Cpu,
                )
                .unwrap(),
            )
            .unwrap()
            .argmax(0)
            .unwrap()
            .to_scalar::<u32>()
            .unwrap() as isize;
        match i {
            0 => MountainAction::Left,
            1 => MountainAction::Right,
            2 => MountainAction::DoNothing,
            _ => MountainAction::Left,
        }
    }
}

fn sarsa<T: mountaincar::Ground>(
    net: &mut MultiLayerPerceptron<2, 3>,
    vars: &mut VarMap,
    mdp: &mut mountaincar::MountainCar<T>,
    alpha: f64,
    nb_steps: usize,
) -> candle_core::error::Result<()> {
    let mut eps = 0.02;
    let mut reward: f64;
    let q_previous = 0.0;
    let q_current = 0.0;
    for _ in 0..nb_steps {
        mdp.reset();
        while !mdp.is_finished() {
            let proba = net.forward(&candle_core::Tensor::from_slice(
                &[mdp.pos, mdp.speed],
                (2,),
                &candle_core::Device::Cpu,
            )?)?;

            reward = mdp.step(net.policy(mdp), 0.1).unwrap_or(0.0) as f64;
            for var in vars.all_vars().iter() {
                if let Some(grad) = proba.backward().unwrap().get(var) {
                    var.set(&var.sub(&(grad * (alpha * (q_current + reward - q_previous)))?)?);
                }
            }
        }
    }
    Ok(())
}

pub fn main() {
    let device: candle_core::Device =
        candle_core::Device::cuda_if_available(0).unwrap_or(candle_core::Device::Cpu);
}
