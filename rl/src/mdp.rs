#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use candle_core::{Device, Tensor};
use std::{collections::HashMap, convert::TryFrom, error::Error, fmt::Debug, path::PathBuf, u32};

/// A trait for implementing Markov Decision processes.
pub trait MarkovDecisionProcess {
    /// Action type.
    type Action: Debug + PartialEq;

    /// Reset the MDP to its initial state.
    fn reset(&mut self);

    /// Take one step forward for the Markov decision process. As the function simulates a
    /// continuous dynamics, the time step is alsso given as an argument of the function.
    fn step(&mut self, action: Self::Action, time_step: f32) -> Result<f32, Box<dyn Error>>;

    /// Indicate if the MDP has reached the terminal state.
    fn is_finished(&self) -> bool;

    /// Return the current set of the MDP as a feature tensor.
    fn feature(&self) -> Tensor;
}

/// Agent trait for implementing AI that plays a game.
pub trait Agent<T>
where
    T: MarkovDecisionProcess,
{
    /// The only function to define for implementing the trait. Take action given the state of a
    /// Markov decision state.
    fn policy(&self, s: &T) -> Result<T::Action, Box<dyn Error>>;

    /// Play the game until the end and return the total reward.
    fn play_game(&self, e: &mut T, time_step: Option<f32>) -> Result<f32, Box<dyn Error>> {
        let mut total_reward = 0.0;
        let time_step = time_step.unwrap_or(0.1);
        while !e.is_finished() {
            let a = self.policy(e)?;
            total_reward += e.step(a, time_step)?;
        }
        Ok(total_reward)
    }

    /// Monte-Carlo evaluation of the performance of the agent.
    fn evaluate(
        &self,
        e: &mut T,
        nb_games: Option<u32>,
        time_step: Option<f32>,
    ) -> Result<f32, Box<dyn Error>> {
        let n = nb_games.unwrap_or(1_000);
        let mut res = 0.0;
        for _ in 1..n {
            e.reset();
            res += self.play_game(e, time_step)?;
        }
        Ok(res)
    }
}

/// Sub-trait that implements agentss loading from safetensors file.
pub trait FileLoader<T: MarkovDecisionProcess>:
    Agent<T> + for<'a> TryFrom<&'a mut HashMap<String, Tensor>>
{
    /// Function that implements the loading of file.
    fn from_file(file: PathBuf) -> Result<Self, ()> {
        // Select the device. Try GPU and pick CPU if not found.
        let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);

        let Ok(mut h) = candle_core::safetensors::load(file, &device) else {
            return Err(());
        };
        let Ok(brain) = Self::try_from(&mut h) else {
            return Err(());
        };
        Ok(brain)
    }
}
