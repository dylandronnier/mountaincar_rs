use candle_core::{Module, Tensor};
use candle_nn::VarMap;

mod mdp;
mod mlp;
mod mountaincar;

use crate::mdp::{Agent, MarkovDecisionProcess};
use crate::mlp::MultiLayerPerceptron;
// use crate::mountaincar::{Ground, MountainAction};
// struct ActorCritic<const D: usize, const A: usize> {
//     pub agent: MultiLayerPerceptron<D, A>,
//     pub critic: MultiLayerPerceptron<D, 1>,
// }
trait Derivable {
    fn derivative(self) -> Tensor;
}

trait ActorCritic<const D: usize, T: MarkovDecisionProcess<D>> {
    type A: Agent<D, T> + Derivable;
}

// impl<const D: usize, T> Agent<D, T>
//     for MultiLayerPerceptron<D, { mem::variant_count::<<T as Mdp>::Action>() }>
// where
//     T: Mdp<D>,
// {
//     fn policy(&self, s: &T) -> <T as Mdp<D>>::Action {
//         let i = self
//             .forward(&candle_core::Tensor::from_slice(&s.feature(), D, &self.device).unwrap())
//             .unwrap()
//             .argmax(0)
//             .unwrap()
//             .to_scalar::<u32>()
//             .unwrap() as u8;
//         i as <T as Mdp<D>>::Action
//     }
// }

//fn sarsa<T: mountaincar::Ground>(
//    net: &mut MultiLayerPerceptron<2, 3>,
//    vars: &mut VarMap,
//    mdp: &mut mountaincar::MountainCar<T>,
//    alpha: f64,
//    nb_steps: usize,
//) -> candle_core::error::Result<()> {
//    let mut eps = 0.02;
//    let mut reward: f64;
//    let q_previous = 0.0;
//    let q_current = 0.0;
//    for _ in 0..nb_steps {
//        mdp.reset();
//        while !mdp.is_finished() {
//            let proba = net.forward(&candle_core::Tensor::from_slice(
//                &[mdp.pos, mdp.speed],
//                (2,),
//                &candle_core::Device::Cpu,
//            )?)?;
//
//            reward = mdp.step(net.policy(mdp), 0.1).unwrap_or(0.0) as f64;
//            for var in vars.all_vars().iter() {
//                if let Some(grad) = proba.backward().unwrap().get(var) {
//                    var.set(&var.sub(&(grad * (alpha * (q_current + reward - q_previous)))?)?);
//                }
//            }
//        }
//    }
//    Ok(())
//}

pub fn main() {
    let device: candle_core::Device =
        candle_core::Device::cuda_if_available(0).unwrap_or(candle_core::Device::Cpu);
}
