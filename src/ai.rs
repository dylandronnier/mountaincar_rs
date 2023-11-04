use candle_core::{Module, Tensor};
use candle_nn::VarMap;

use crate::mdp::{Agent, Mdp};
use crate::mountaincar::{self, Ground, MountainAction};

struct Nn {
    ln1: candle_nn::Linear,
    ln2: candle_nn::Linear,
    ln3: candle_nn::Linear,
    ln4: candle_nn::Linear,
}

impl Nn {
    fn new(vs: candle_nn::VarBuilder) -> candle_core::error::Result<Self> {
        let layer1 = candle_nn::linear(2, 4, vs.pp("ln1"))?;
        let layer2 = candle_nn::linear(4, 8, vs.pp("ln2"))?;
        let layer3 = candle_nn::linear(8, 4, vs.pp("ln3"))?;
        let layer4 = candle_nn::linear(4, 2, vs.pp("ln4"))?;
        Ok(Self {
            ln1: layer1,
            ln2: layer2,
            ln3: layer3,
            ln4: layer4,
        })
    }

    fn forward(&self, xs: &candle_core::Tensor) -> candle_core::error::Result<Tensor> {
        let xs = self.ln1.forward(xs)?;
        let xs = xs.relu()?;
        let xs = self.ln2.forward(&xs)?;
        let xs = xs.relu()?;
        let xs = self.ln3.forward(&xs)?;
        let xs = xs.relu()?;
        let xs = self.ln4.forward(&xs)?;
        candle_nn::ops::softmax(&xs, 1)
    }
}

impl<T: Ground> Agent<mountaincar::MountainCar<T>> for Nn {
    fn policy(
        &self,
        s: &mountaincar::MountainCar<T>,
    ) -> <mountaincar::MountainCar<T> as crate::mdp::Mdp>::Action {
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
    net: &mut Nn,
    vars: &mut VarMap,
    mdp: &mut mountaincar::MountainCar<T>,
    alpha: f64,
    nb_steps: usize,
) -> candle_core::error::Result<()> {
    let mut eps = 0.02;
    let mut score = 0.0;
    let mut reward: f64;
    let v = 0.0;
    for i in 0..nb_steps {
        mdp.reset();
        while !mdp.is_finished() {
            let proba = net
                .forward(
                    &candle_core::Tensor::from_slice(
                        &[mdp.pos, mdp.speed],
                        (2,),
                        &candle_core::Device::Cpu,
                    )
                    .unwrap(),
                )
                .unwrap();

            reward = mdp.step(net.policy(mdp), 0.1).unwrap_or(0.0) as f64;
            score += reward;
            for var in vars.all_vars().iter() {
                if let Some(grad) = proba.backward().unwrap().get(var) {
                    var.set(&var.sub(&(grad * (alpha * (reward - v)))?)?);
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
